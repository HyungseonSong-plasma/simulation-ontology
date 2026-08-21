use sol_public_contract::{
    BackendTargetDto, EvaluationResultDto, EvaluationStatusDto, MappingClaimsDto, MappingPlanDto,
    MappingSubjectDto, RealizationDtoError, SemanticComparisonDto,
};

const THERMAL_CLAIMS: &str =
    include_str!("../../../fixtures/public-contract/0.1/thermal-mapping-claims.json");
const THERMAL_PLAN: &str =
    include_str!("../../../fixtures/public-contract/0.1/thermal-mapping-plan.json");
const THERMAL_TARGET: &str =
    include_str!("../../../fixtures/public-contract/0.1/thermal-backend-target.json");
const THERMAL_EVALUATION: &str =
    include_str!("../../../fixtures/public-contract/0.1/thermal-evaluation.json");
const PLAN_CYCLE: &str =
    include_str!("../../../fixtures/counterexamples/public-contract-plan-cycle.json");
const UNSUPPORTED: &str =
    include_str!("../../../fixtures/counterexamples/public-contract-evaluation-unsupported.json");
const UNKNOWN: &str =
    include_str!("../../../fixtures/counterexamples/public-contract-evaluation-unknown.json");
const SUBJECT_MISMATCH: &str = include_str!(
    "../../../fixtures/counterexamples/public-contract-evaluation-subject-mismatch.json"
);
const MISMATCH_FALSE_PASS: &str = include_str!(
    "../../../fixtures/counterexamples/public-contract-evaluation-mismatch-false-pass.json"
);
const BACKEND_NATIVE_LEAKAGE: &str =
    include_str!("../../../fixtures/counterexamples/public-contract-backend-native-leakage.json");

#[test]
fn thermal_mapping_claims_round_trip_canonically() {
    let claims = MappingClaimsDto::from_json(THERMAL_CLAIMS).unwrap();
    assert_eq!(claims.claims.len(), 1);
    assert_eq!(claims.claims[0].rule_id, "thermal.energy-equation");

    let canonical = claims.to_canonical_json().unwrap();
    let reparsed = MappingClaimsDto::from_json(&canonical).unwrap();
    assert_eq!(claims, reparsed);
}

#[test]
fn mapping_claims_are_sorted_deterministically() {
    let input = r#"{
        "public_contract_version":"0.1",
        "claims":[
            {
                "rule_id":"z.rule",
                "subject":{"subject_kind":"entity","id":"thermal.temperature_field"},
                "evidence":[]
            },
            {
                "rule_id":"a.rule",
                "subject":{"subject_kind":"entity","id":"thermal.energy_conservation"},
                "evidence":[]
            }
        ]
    }"#;

    let claims = MappingClaimsDto::from_json(input).unwrap();
    assert_eq!(claims.claims[0].rule_id, "a.rule");
    assert_eq!(claims.claims[1].rule_id, "z.rule");
}

#[test]
fn thermal_plan_preserves_dag_and_canonical_order() {
    let plan = MappingPlanDto::from_json(THERMAL_PLAN).unwrap();
    assert_eq!(
        plan.topological_order().unwrap(),
        vec!["thermal.domain", "thermal.material", "thermal.solve"]
    );
    assert_eq!(plan.actions[0].id, "thermal.domain");
    assert_eq!(plan.actions[1].dependencies, vec!["thermal.domain"]);

    let canonical = plan.to_canonical_json().unwrap();
    let reparsed = MappingPlanDto::from_json(&canonical).unwrap();
    assert_eq!(plan, reparsed);
}

#[test]
fn cyclic_public_plan_is_rejected() {
    assert!(matches!(
        MappingPlanDto::from_json(PLAN_CYCLE),
        Err(RealizationDtoError::CycleDetected(_))
    ));
}

#[test]
fn backend_target_requirements_are_sorted_and_deduplicated() {
    let target = BackendTargetDto::from_json(THERMAL_TARGET).unwrap();
    assert_eq!(target.target, "mock");
    assert_eq!(
        target.required_capabilities,
        vec!["thermal.domain", "thermal.material", "thermal.solve"]
    );

    let unsorted = r#"{
        "public_contract_version":"0.1",
        "target":"mock",
        "required_capabilities":["thermal.solve","thermal.domain","thermal.solve"]
    }"#;
    let normalized = BackendTargetDto::from_json(unsorted).unwrap();
    assert_eq!(
        normalized.required_capabilities,
        vec!["thermal.domain", "thermal.solve"]
    );
}

#[test]
fn backend_native_objects_do_not_enter_canonical_target_payload() {
    assert!(matches!(
        BackendTargetDto::from_json(BACKEND_NATIVE_LEAKAGE),
        Err(RealizationDtoError::BackendNativeLeakage(_))
    ));
}

#[test]
fn thermal_exact_evaluation_passes() {
    let result = EvaluationResultDto::from_json(THERMAL_EVALUATION).unwrap();
    assert_eq!(result.comparison, SemanticComparisonDto::Exact);
    assert_eq!(result.status, EvaluationStatusDto::Pass);

    let canonical = result.to_canonical_json().unwrap();
    let reparsed = EvaluationResultDto::from_json(&canonical).unwrap();
    assert_eq!(result, reparsed);
}

#[test]
fn unsupported_is_blocked_not_failed() {
    let result = EvaluationResultDto::from_json(UNSUPPORTED).unwrap();
    assert_eq!(result.comparison, SemanticComparisonDto::Unsupported);
    assert_eq!(result.status, EvaluationStatusDto::Blocked);
    assert_ne!(result.status, EvaluationStatusDto::Fail);
}

#[test]
fn unknown_is_indeterminate_not_failed() {
    let result = EvaluationResultDto::from_json(UNKNOWN).unwrap();
    assert_eq!(result.comparison, SemanticComparisonDto::Unknown);
    assert_eq!(result.status, EvaluationStatusDto::Indeterminate);
    assert_ne!(result.status, EvaluationStatusDto::Fail);
}

#[test]
fn subject_mismatch_is_completed_failure() {
    let result = EvaluationResultDto::from_json(SUBJECT_MISMATCH).unwrap();
    assert_eq!(result.comparison, SemanticComparisonDto::SubjectMismatch);
    assert_eq!(result.status, EvaluationStatusDto::Fail);
}

#[test]
fn subject_mismatch_cannot_be_declared_exact_pass() {
    assert!(matches!(
        EvaluationResultDto::from_json(MISMATCH_FALSE_PASS),
        Err(RealizationDtoError::InconsistentEvaluation { .. })
    ));
}

#[test]
fn unknown_subject_extensions_do_not_change_semantic_identity() {
    let intended_json = r#"{
        "subject_kind":"entity",
        "id":"thermal.energy_conservation",
        "future_hint":"left"
    }"#;
    let effect_subject_json = r#"{
        "subject_kind":"entity",
        "id":"thermal.energy_conservation",
        "future_hint":"right"
    }"#;
    let intended: MappingSubjectDto = serde_json::from_str(intended_json).unwrap();
    let effect_subject: MappingSubjectDto = serde_json::from_str(effect_subject_json).unwrap();
    let input = serde_json::json!({
        "public_contract_version": "0.1",
        "intended_subject": intended,
        "effect": {
            "subject": effect_subject,
            "quality": "exact"
        },
        "comparison": "exact",
        "status": "pass"
    });

    let result = EvaluationResultDto::from_json(&input.to_string()).unwrap();
    assert_eq!(result.status, EvaluationStatusDto::Pass);
}

#[test]
fn opaque_backend_artifact_reference_may_remain_nonsemantic_provenance() {
    let input = r#"{
        "public_contract_version":"0.1",
        "claims":[{
            "rule_id":"thermal.energy-equation",
            "subject":{"subject_kind":"entity","id":"thermal.energy_conservation"},
            "evidence":[],
            "provenance":{
                "producer":"sol-adapter-example",
                "backend_artifact_ref":"backend:job/123"
            }
        }]
    }"#;

    let claims = MappingClaimsDto::from_json(input).unwrap();
    assert_eq!(
        claims.claims[0]
            .provenance
            .as_ref()
            .unwrap()
            .extensions
            .get("backend_artifact_ref")
            .and_then(|value| value.as_str()),
        Some("backend:job/123")
    );
}
