use sol_adapter_protocol::{
    diagnostic_context, AdapterProtocolDiagnosticContext, PreflightError, PreflightOutcome,
    ValidatePlanRequest, ValidatePlanResponse, DIAGNOSTIC_CONTEXT_EXTENSION,
    DIAGNOSTIC_MISSING_CAPABILITY, DIAGNOSTIC_PRECONDITION_REJECTED, DIAGNOSTIC_TARGET_MISMATCH,
    DIAGNOSTIC_TRANSIENT_UNAVAILABLE, DIAGNOSTIC_UNSUPPORTED_ACTION,
};

const ACCEPTED_REQUEST: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/validate-plan-accepted-request.json");
const ACCEPTED_RESPONSE: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/validate-plan-accepted-response.json");
const TARGET_MISMATCH: &str = include_str!(
    "../../../fixtures/counterexamples/adapter-protocol-target-mismatch-response.json"
);
const MISSING_CAPABILITY: &str = include_str!(
    "../../../fixtures/counterexamples/adapter-protocol-missing-capability-response.json"
);
const UNSUPPORTED_ACTION: &str = include_str!(
    "../../../fixtures/counterexamples/adapter-protocol-unsupported-action-response.json"
);
const PRECONDITION_REJECTED: &str = include_str!(
    "../../../fixtures/counterexamples/adapter-protocol-precondition-rejected-response.json"
);
const TRANSIENT_UNAVAILABLE: &str = include_str!(
    "../../../fixtures/counterexamples/adapter-protocol-transient-unavailable-response.json"
);
const INCONSISTENT_ACCEPTED: &str = include_str!(
    "../../../fixtures/counterexamples/adapter-protocol-inconsistent-accepted-response.json"
);
const TOKEN_AUTHORITY: &str = include_str!(
    "../../../fixtures/counterexamples/adapter-protocol-preflight-token-authority.json"
);
const BACKEND_NATIVE: &str = include_str!(
    "../../../fixtures/counterexamples/adapter-protocol-preflight-backend-native-leakage.json"
);
const CHANGED_PLAN: &str = include_str!(
    "../../../fixtures/counterexamples/adapter-protocol-validate-plan-changed-plan-request.json"
);

#[test]
fn validate_plan_request_reuses_public_contract_target_and_plan() {
    let request = ValidatePlanRequest::from_json(ACCEPTED_REQUEST).unwrap();

    assert_eq!(request.adapter_protocol_version, "0.1");
    assert_eq!(request.target.public_contract_version, "0.1");
    assert_eq!(request.plan.public_contract_version, "0.1");
    assert_eq!(request.target.target, "mock");
    assert_eq!(
        request.plan.topological_order().unwrap(),
        vec!["thermal.domain", "thermal.material", "thermal.solve"]
    );

    let canonical = request.to_canonical_json().unwrap();
    assert!(!canonical.contains("jsonrpc"));
    assert!(!canonical.contains("validation_token"));
    assert!(!canonical.contains("plan_hash"));
}

#[test]
fn accepted_preflight_is_advisory_and_contains_no_authority_token() {
    let response = ValidatePlanResponse::from_json(ACCEPTED_RESPONSE).unwrap();

    assert_eq!(response.preflight, PreflightOutcome::Accepted);
    assert!(response.target_compatible);
    assert!(response.capabilities_satisfied);
    assert!(response.diagnostics.is_empty());

    let canonical = response.to_canonical_json().unwrap();
    assert!(!canonical.contains("validation_token"));
    assert!(!canonical.contains("acceptance_id"));
    assert!(!canonical.contains("lease_id"));
}

#[test]
fn target_capability_and_preflight_outcomes_remain_distinct() {
    let target = ValidatePlanResponse::from_json(TARGET_MISMATCH).unwrap();
    assert!(!target.target_compatible);
    assert_eq!(target.preflight, PreflightOutcome::Rejected);
    assert!(target
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == DIAGNOSTIC_TARGET_MISMATCH));

    let capability = ValidatePlanResponse::from_json(MISSING_CAPABILITY).unwrap();
    assert!(capability.target_compatible);
    assert!(!capability.capabilities_satisfied);
    assert_eq!(capability.preflight, PreflightOutcome::Rejected);
    assert!(capability
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == DIAGNOSTIC_MISSING_CAPABILITY));
}

#[test]
fn adapter_rejection_and_transient_unavailability_are_not_lifecycle_states() {
    let unsupported = ValidatePlanResponse::from_json(UNSUPPORTED_ACTION).unwrap();
    assert_eq!(unsupported.preflight, PreflightOutcome::Rejected);
    assert!(unsupported
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == DIAGNOSTIC_UNSUPPORTED_ACTION));

    let rejected = ValidatePlanResponse::from_json(PRECONDITION_REJECTED).unwrap();
    assert_eq!(rejected.preflight, PreflightOutcome::Rejected);
    assert!(rejected
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == DIAGNOSTIC_PRECONDITION_REJECTED));

    let unavailable = ValidatePlanResponse::from_json(TRANSIENT_UNAVAILABLE).unwrap();
    assert_eq!(unavailable.preflight, PreflightOutcome::Unavailable);
    assert!(unavailable
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == DIAGNOSTIC_TRANSIENT_UNAVAILABLE));

    for fixture in [
        UNSUPPORTED_ACTION,
        PRECONDITION_REJECTED,
        TRANSIENT_UNAVAILABLE,
    ] {
        assert!(!fixture.contains("PASS"));
        assert!(!fixture.contains("FAIL"));
        assert!(!fixture.contains("BLOCKED"));
        assert!(!fixture.contains("INDETERMINATE"));
    }
}

#[test]
fn inconsistent_accepted_response_is_rejected() {
    assert!(matches!(
        ValidatePlanResponse::from_json(INCONSISTENT_ACCEPTED),
        Err(PreflightError::InconsistentResponse(_))
    ));
}

#[test]
fn durable_validation_tokens_and_backend_native_semantics_are_rejected() {
    assert!(matches!(
        ValidatePlanRequest::from_json(TOKEN_AUTHORITY),
        Err(PreflightError::ForbiddenField(field)) if field == "validation_token"
    ));
    assert!(matches!(
        ValidatePlanRequest::from_json(BACKEND_NATIVE),
        Err(PreflightError::ForbiddenField(field)) if field == "backend_native_id"
    ));
}

#[test]
fn changed_plan_after_preflight_has_different_canonical_identity() {
    let original = ValidatePlanRequest::from_json(ACCEPTED_REQUEST).unwrap();
    let changed = ValidatePlanRequest::from_json(CHANGED_PLAN).unwrap();

    assert_ne!(
        original.canonical_plan_identity().unwrap(),
        changed.canonical_plan_identity().unwrap()
    );
    assert_ne!(original.plan, changed.plan);
}

#[test]
fn diagnostic_context_round_trips_without_becoming_canonical_subject_identity() {
    let response = ValidatePlanResponse::from_json(UNSUPPORTED_ACTION).unwrap();
    let diagnostic = response
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code == DIAGNOSTIC_UNSUPPORTED_ACTION)
        .unwrap();

    assert!(diagnostic.subject.is_none());
    assert!(diagnostic
        .extensions
        .contains_key(DIAGNOSTIC_CONTEXT_EXTENSION));
    assert_eq!(
        diagnostic_context(diagnostic).unwrap(),
        Some(AdapterProtocolDiagnosticContext {
            plan_action_id: Some("thermal.radiation".to_owned()),
            ..AdapterProtocolDiagnosticContext::default()
        })
    );
}

#[test]
fn canonical_plan_order_does_not_create_a_physical_schedule_field() {
    let request = ValidatePlanRequest::from_json(ACCEPTED_REQUEST).unwrap();
    let canonical = request.to_canonical_json().unwrap();

    assert!(!canonical.contains("execution_order"));
    assert!(!canonical.contains("physical_schedule"));
    assert_eq!(
        request.plan.topological_order().unwrap(),
        vec!["thermal.domain", "thermal.material", "thermal.solve"]
    );
}
