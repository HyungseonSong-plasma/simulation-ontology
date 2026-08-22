#![forbid(unsafe_code)]

//! Minimal solver-independent adapter logic for the authoring skeleton.
//!
//! The `mock` target and thermal action names intentionally match the published Protocol 0.1
//! fixture baseline. Real adapters replace this sample backend mapping while keeping the
//! published Adapter Protocol/Public Contract DTO semantics unchanged.

use sol_adapter_protocol::{
    ActionExecutionReport, ActionExecutionState, AdapterBootstrap, AdapterDescription,
    AdapterProtocolDiagnosticContext, CapabilityDeclaration, ExecutePlanRequest,
    ExecutePlanResponse, ExecutionOutcome, ExecutionProvenance, OpaqueExecutionReference,
    PreflightOutcome, ProtocolFailure, ProtocolOperation, SideEffectEvidence, TargetDeclaration,
    ValidatePlanRequest, ValidatePlanResponse, ADAPTER_PROTOCOL_VERSION,
    DIAGNOSTIC_EXECUTION_REJECTED, DIAGNOSTIC_MISSING_CAPABILITY, DIAGNOSTIC_TARGET_MISMATCH,
    DIAGNOSTIC_UNSUPPORTED_ACTION,
};
use sol_adapter_protocol::protocol_diagnostic;
use sol_public_contract::{
    Diagnostic, MappingSubjectDto, RealizationEffectDto, RealizationQualityDto,
    PUBLIC_CONTRACT_VERSION,
};
use std::collections::{BTreeMap, BTreeSet};

const SAMPLE_TARGET: &str = "mock";
const SAMPLE_CAPABILITIES: [&str; 3] = ["thermal.domain", "thermal.material", "thermal.solve"];

#[derive(Debug, Clone, Copy, Default)]
pub struct ReferenceAdapter;

impl ReferenceAdapter {
    pub fn describe_adapter(&self) -> Result<AdapterDescription, ProtocolFailure> {
        let description = AdapterDescription {
            bootstrap: AdapterBootstrap {
                adapter_id: "adapter.authoring_reference".to_owned(),
                adapter_version: env!("CARGO_PKG_VERSION").to_owned(),
                supported_adapter_protocol_versions: Some(vec![ADAPTER_PROTOCOL_VERSION.to_owned()]),
                supported_public_contract_versions: Some(vec![PUBLIC_CONTRACT_VERSION.to_owned()]),
                extensions: Default::default(),
            },
            targets: vec![TargetDeclaration {
                target: SAMPLE_TARGET.to_owned(),
                capabilities: vec![
                    CapabilityDeclaration {
                        capability: "thermal.solve".to_owned(),
                        revision: Some("mock-capabilities-1".to_owned()),
                        extensions: Default::default(),
                    },
                    CapabilityDeclaration {
                        capability: "thermal.material".to_owned(),
                        revision: None,
                        extensions: Default::default(),
                    },
                    CapabilityDeclaration {
                        capability: "thermal.domain".to_owned(),
                        revision: None,
                        extensions: Default::default(),
                    },
                ],
                extensions: Default::default(),
            }],
            extensions: Default::default(),
        };

        let canonical = description
            .to_canonical_json()
            .map_err(|error| operation_failure(ProtocolOperation::DescribeAdapter, error, SideEffectEvidence::None))?;
        AdapterDescription::from_json(&canonical)
            .map_err(|error| operation_failure(ProtocolOperation::DescribeAdapter, error, SideEffectEvidence::None))
    }

    pub fn validate_plan(
        &self,
        request: &ValidatePlanRequest,
    ) -> Result<ValidatePlanResponse, ProtocolFailure> {
        let canonical = request
            .to_canonical_json()
            .map_err(|error| ProtocolFailure::invalid_request(error.to_string()).expect("non-blank request error"))?;
        let request = ValidatePlanRequest::from_json(&canonical)
            .map_err(|error| ProtocolFailure::invalid_request(error.to_string()).expect("non-blank request error"))?;

        if request.target.target != SAMPLE_TARGET {
            return negative_preflight(
                false,
                false,
                vec![
                    protocol_diagnostic(
                        DIAGNOSTIC_TARGET_MISMATCH,
                        "sample adapter does not address the requested target",
                        AdapterProtocolDiagnosticContext {
                            target: Some(request.target.target.clone()),
                            ..Default::default()
                        },
                    ),
                    protocol_diagnostic(
                        DIAGNOSTIC_MISSING_CAPABILITY,
                        "capabilities cannot be satisfied for an unmatched target",
                        AdapterProtocolDiagnosticContext::default(),
                    ),
                ],
            );
        }

        let missing = request
            .target
            .required_capabilities
            .iter()
            .filter(|capability| !SAMPLE_CAPABILITIES.contains(&capability.as_str()))
            .cloned()
            .collect::<Vec<_>>();
        if !missing.is_empty() {
            let diagnostics = missing
                .into_iter()
                .map(|capability| {
                    protocol_diagnostic(
                        DIAGNOSTIC_MISSING_CAPABILITY,
                        "required capability is not declared by the sample adapter",
                        AdapterProtocolDiagnosticContext {
                            capability: Some(capability),
                            ..Default::default()
                        },
                    )
                })
                .collect();
            return negative_preflight(true, false, diagnostics);
        }

        let unsupported = request
            .plan
            .actions
            .iter()
            .filter(|action| !SAMPLE_CAPABILITIES.contains(&action.id.as_str()))
            .map(|action| action.id.clone())
            .collect::<Vec<_>>();
        if !unsupported.is_empty() {
            let diagnostics = unsupported
                .into_iter()
                .map(|action_id| {
                    protocol_diagnostic(
                        DIAGNOSTIC_UNSUPPORTED_ACTION,
                        "plan action is not supported by the sample adapter",
                        AdapterProtocolDiagnosticContext {
                            plan_action_id: Some(action_id),
                            ..Default::default()
                        },
                    )
                })
                .collect::<Vec<_>>();
            return negative_preflight(true, true, diagnostics);
        }

        Ok(ValidatePlanResponse::accepted())
    }

    pub fn execute_plan(
        &self,
        request: &ExecutePlanRequest,
    ) -> Result<ExecutePlanResponse, ProtocolFailure> {
        let canonical = request
            .to_canonical_json()
            .map_err(|error| ProtocolFailure::invalid_request(error.to_string()).expect("non-blank request error"))?;
        let request = ExecutePlanRequest::from_json(&canonical)
            .map_err(|error| ProtocolFailure::invalid_request(error.to_string()).expect("non-blank request error"))?;

        if request.target.target != SAMPLE_TARGET
            || request
                .target
                .required_capabilities
                .iter()
                .any(|capability| !SAMPLE_CAPABILITIES.contains(&capability.as_str()))
            || request
                .plan
                .actions
                .iter()
                .any(|action| !SAMPLE_CAPABILITIES.contains(&action.id.as_str()))
        {
            return checked_execute(
                rejected_execution(&request),
                &request,
                SideEffectEvidence::None,
            );
        }

        let execution_batches = dependency_safe_serial_schedule(&request);
        let action_reports = request
            .plan
            .actions
            .iter()
            .map(|action| ActionExecutionReport {
                action_id: action.id.clone(),
                state: ActionExecutionState::Completed,
                effects: effects_for_action(&action.id),
                diagnostics: Vec::new(),
                provenance: None,
                extensions: Default::default(),
            })
            .collect::<Vec<_>>();
        let effects = action_reports
            .iter()
            .flat_map(|report| report.effects.iter().cloned())
            .collect();

        checked_execute(
            ExecutePlanResponse {
                adapter_protocol_version: ADAPTER_PROTOCOL_VERSION.to_owned(),
                execution: ExecutionOutcome::Completed,
                execution_batches,
                action_reports,
                effects,
                diagnostics: Vec::new(),
                provenance: Some(ExecutionProvenance {
                    producer: "adapter.mock_thermal".to_owned(),
                    opaque_references: vec![OpaqueExecutionReference {
                        namespace: "mock.job".to_owned(),
                        reference: "job-42".to_owned(),
                        extensions: Default::default(),
                    }],
                    extensions: Default::default(),
                }),
                extensions: Default::default(),
            },
            &request,
            SideEffectEvidence::MayHaveOccurred,
        )
    }
}

fn negative_preflight(
    target_compatible: bool,
    capabilities_satisfied: bool,
    diagnostics: Vec<Result<Diagnostic, sol_adapter_protocol::PreflightError>>,
) -> Result<ValidatePlanResponse, ProtocolFailure> {
    let diagnostics = diagnostics
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| operation_failure(ProtocolOperation::ValidatePlan, error, SideEffectEvidence::None))?;
    ValidatePlanResponse::new(
        target_compatible,
        capabilities_satisfied,
        PreflightOutcome::Rejected,
        diagnostics,
    )
    .map_err(|error| operation_failure(ProtocolOperation::ValidatePlan, error, SideEffectEvidence::None))
}

fn rejected_execution(request: &ExecutePlanRequest) -> ExecutePlanResponse {
    ExecutePlanResponse {
        adapter_protocol_version: ADAPTER_PROTOCOL_VERSION.to_owned(),
        execution: ExecutionOutcome::Rejected,
        execution_batches: Vec::new(),
        action_reports: request
            .plan
            .actions
            .iter()
            .map(|action| ActionExecutionReport {
                action_id: action.id.clone(),
                state: ActionExecutionState::NotStarted,
                effects: Vec::new(),
                diagnostics: Vec::new(),
                provenance: None,
                extensions: Default::default(),
            })
            .collect(),
        effects: Vec::new(),
        diagnostics: vec![Diagnostic::error(
            DIAGNOSTIC_EXECUTION_REJECTED,
            None,
            "authoritative execute-time request check rejected the sample request",
        )],
        provenance: None,
        extensions: Default::default(),
    }
}

fn effects_for_action(action_id: &str) -> Vec<RealizationEffectDto> {
    let subject = match action_id {
        "thermal.domain" => "domain.main",
        "thermal.material" => "material.copper",
        "thermal.solve" => "thermal.temperature_field",
        _ => return Vec::new(),
    };
    vec![RealizationEffectDto::new(
        MappingSubjectDto::entity(subject),
        RealizationQualityDto::Exact,
    )]
}

fn dependency_safe_serial_schedule(request: &ExecutePlanRequest) -> Vec<Vec<String>> {
    let actions = request
        .plan
        .actions
        .iter()
        .map(|action| (action.id.clone(), action.dependencies.clone()))
        .collect::<BTreeMap<_, _>>();
    let mut remaining = actions.keys().cloned().collect::<BTreeSet<_>>();
    let mut completed = BTreeSet::new();
    let mut batches = Vec::new();

    while !remaining.is_empty() {
        let next = remaining
            .iter()
            .find(|id| {
                actions
                    .get(*id)
                    .expect("remaining action exists")
                    .iter()
                    .all(|dependency| completed.contains(dependency))
            })
            .expect("Public Contract validation rejects cyclic plans")
            .clone();
        remaining.remove(&next);
        completed.insert(next.clone());
        batches.push(vec![next]);
    }
    batches
}

fn checked_execute(
    mut response: ExecutePlanResponse,
    request: &ExecutePlanRequest,
    side_effects: SideEffectEvidence,
) -> Result<ExecutePlanResponse, ProtocolFailure> {
    response
        .validate_against(request)
        .map_err(|error| operation_failure(ProtocolOperation::ExecutePlan, error, side_effects))?;
    Ok(response)
}

fn operation_failure(
    operation: ProtocolOperation,
    error: impl ToString,
    side_effects: SideEffectEvidence,
) -> ProtocolFailure {
    let failure = ProtocolFailure::operational(error.to_string(), side_effects)
        .expect("adapter error detail is non-blank");
    failure
        .validate_for(operation)
        .expect("side-effect evidence is valid for operation");
    failure
}
