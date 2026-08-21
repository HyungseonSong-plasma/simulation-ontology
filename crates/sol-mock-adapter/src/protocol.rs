//! Protocol-facing MockAdapter reference behavior for the published SOL adapter contract.
//!
//! M0.4 phases add deterministic in-process reference operations here while preserving
//! the pre-existing Rust-only helper API in `lib.rs` as a separate compatibility layer.
//!
//! This module is intentionally transport-independent and solver-independent.

use std::collections::{BTreeMap, BTreeSet};

use sol_adapter_protocol::{
    protocol_diagnostic, ActionExecutionReport, ActionExecutionState, AdapterBootstrap,
    AdapterDescription, AdapterProtocolDiagnosticContext, CapabilityDeclaration,
    CompatibilitySupport, ExecutePlanRequest, ExecutePlanResponse, ExecutionError,
    ExecutionOutcome, ExecutionProvenance, OpaqueExecutionReference, PreflightError,
    PreflightOutcome, ProtocolError, TargetDeclaration, ValidatePlanRequest, ValidatePlanResponse,
    ADAPTER_PROTOCOL_VERSION, DIAGNOSTIC_DEPENDENCY_SKIPPED, DIAGNOSTIC_EXECUTION_REJECTED,
    DIAGNOSTIC_MISSING_CAPABILITY, DIAGNOSTIC_PRECONDITION_REJECTED, DIAGNOSTIC_TARGET_MISMATCH,
    DIAGNOSTIC_TRANSIENT_UNAVAILABLE, DIAGNOSTIC_UNSUPPORTED_ACTION,
};
use sol_public_contract::{
    Diagnostic, MappingPlanDto, MappingSubjectDto, RealizationEffectDto, RealizationQualityDto,
};

use crate::{Adapter, MockAdapter};

pub const MOCK_ADAPTER_ID: &str = "adapter.mock";
pub const MOCK_TARGET: &str = "mock";
pub const MOCK_CAPABILITY_REVISION: &str = "mock-capabilities-1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MockPreflightState {
    #[default]
    Ready,
    PrerequisiteRejected,
    TransientUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MockExecutionState {
    #[default]
    Exact,
    Degraded,
    Unsupported,
    Partial,
    Unavailable,
    AuthoritativeRejected,
    AlternateOrder,
    ParallelIndependent,
}

impl MockAdapter {
    pub fn with_preflight_state(mut self, state: MockPreflightState) -> Self {
        self.protocol_preflight_state = state;
        self
    }

    pub fn with_execution_state(mut self, state: MockExecutionState) -> Self {
        self.protocol_execution_state = state;
        self
    }

    pub fn describe_adapter(&self) -> Result<AdapterDescription, ProtocolError> {
        let support = CompatibilitySupport::current();
        let capabilities = self
            .capabilities()
            .iter()
            .map(|capability| CapabilityDeclaration {
                capability: capability.as_str().to_owned(),
                revision: (capability.as_str() == "thermal.solve")
                    .then(|| MOCK_CAPABILITY_REVISION.to_owned()),
                extensions: Default::default(),
            })
            .collect();

        let description = AdapterDescription {
            bootstrap: AdapterBootstrap {
                adapter_id: MOCK_ADAPTER_ID.to_owned(),
                adapter_version: env!("CARGO_PKG_VERSION").to_owned(),
                supported_adapter_protocol_versions: support.adapter_protocol_versions,
                supported_public_contract_versions: support.public_contract_versions,
                extensions: Default::default(),
            },
            targets: vec![TargetDeclaration {
                target: MOCK_TARGET.to_owned(),
                capabilities,
                extensions: Default::default(),
            }],
            extensions: Default::default(),
        };

        let canonical = description.to_canonical_json()?;
        let normalized = AdapterDescription::from_json(&canonical)?;
        debug_assert_eq!(
            normalized.bootstrap.supported_adapter_protocol_versions,
            Some(vec![ADAPTER_PROTOCOL_VERSION.to_owned()])
        );
        Ok(normalized)
    }

    pub fn validate_plan(
        &self,
        request: &ValidatePlanRequest,
    ) -> Result<ValidatePlanResponse, PreflightError> {
        let canonical_request = request.to_canonical_json()?;
        let request = ValidatePlanRequest::from_json(&canonical_request)?;
        let target_compatible = request.target.target == MOCK_TARGET;
        let declared_capabilities = self.declared_capability_names();

        let missing_capabilities = request
            .target
            .required_capabilities
            .iter()
            .filter(|capability| {
                !target_compatible || !declared_capabilities.contains(capability.as_str())
            })
            .cloned()
            .collect::<Vec<_>>();
        let capabilities_satisfied = target_compatible && missing_capabilities.is_empty();

        let mut diagnostics = Vec::new();
        if !target_compatible {
            diagnostics.push(protocol_diagnostic(
                DIAGNOSTIC_TARGET_MISMATCH,
                "requested target is not addressed by this adapter",
                AdapterProtocolDiagnosticContext {
                    target: Some(request.target.target.clone()),
                    ..Default::default()
                },
            )?);
        }

        for capability in &missing_capabilities {
            diagnostics.push(protocol_diagnostic(
                DIAGNOSTIC_MISSING_CAPABILITY,
                if target_compatible {
                    "required capability is not declared"
                } else {
                    "capabilities cannot be satisfied for the unmatched target"
                },
                AdapterProtocolDiagnosticContext {
                    capability: Some(capability.clone()),
                    ..Default::default()
                },
            )?);
        }

        if !target_compatible && missing_capabilities.is_empty() {
            diagnostics.push(protocol_diagnostic(
                DIAGNOSTIC_MISSING_CAPABILITY,
                "capabilities cannot be satisfied for the unmatched target",
                AdapterProtocolDiagnosticContext::default(),
            )?);
        }

        for action in &request.plan.actions {
            if !supports_mock_action(&action.id) {
                diagnostics.push(protocol_diagnostic(
                    DIAGNOSTIC_UNSUPPORTED_ACTION,
                    "plan action is not supported by the adapter",
                    AdapterProtocolDiagnosticContext {
                        plan_action_id: Some(action.id.clone()),
                        ..Default::default()
                    },
                )?);
            }
        }

        if !target_compatible || !capabilities_satisfied || !diagnostics.is_empty() {
            return ValidatePlanResponse::new(
                target_compatible,
                capabilities_satisfied,
                PreflightOutcome::Rejected,
                diagnostics,
            );
        }

        match self.protocol_preflight_state {
            MockPreflightState::Ready => {
                ValidatePlanResponse::new(true, true, PreflightOutcome::Accepted, Vec::new())
            }
            MockPreflightState::PrerequisiteRejected => ValidatePlanResponse::new(
                true,
                true,
                PreflightOutcome::Rejected,
                vec![protocol_diagnostic(
                    DIAGNOSTIC_PRECONDITION_REJECTED,
                    "adapter-specific prerequisite is not satisfied",
                    AdapterProtocolDiagnosticContext {
                        precondition: Some("mesh.loaded".to_owned()),
                        ..Default::default()
                    },
                )?],
            ),
            MockPreflightState::TransientUnavailable => ValidatePlanResponse::new(
                true,
                true,
                PreflightOutcome::Unavailable,
                vec![protocol_diagnostic(
                    DIAGNOSTIC_TRANSIENT_UNAVAILABLE,
                    "backend is temporarily unavailable",
                    AdapterProtocolDiagnosticContext {
                        precondition: Some("backend.available".to_owned()),
                        ..Default::default()
                    },
                )?],
            ),
        }
    }

    pub fn execute_plan(
        &self,
        request: &ExecutePlanRequest,
    ) -> Result<ExecutePlanResponse, ExecutionError> {
        let canonical_request = request.to_canonical_json()?;
        let request = ExecutePlanRequest::from_json(&canonical_request)?;

        if request.target.target != MOCK_TARGET {
            return checked_response(
                rejected_response(
                    &request.plan,
                    Diagnostic::error(
                        DIAGNOSTIC_EXECUTION_REJECTED,
                        None,
                        "authoritative execute-time target check rejected the request",
                    ),
                ),
                &request,
            );
        }

        let declared_capabilities = self.declared_capability_names();
        if request
            .target
            .required_capabilities
            .iter()
            .any(|capability| !declared_capabilities.contains(capability.as_str()))
        {
            return checked_response(
                rejected_response(
                    &request.plan,
                    Diagnostic::error(
                        DIAGNOSTIC_EXECUTION_REJECTED,
                        None,
                        "authoritative execute-time capability check rejected the request",
                    ),
                ),
                &request,
            );
        }

        if self.protocol_execution_state == MockExecutionState::AuthoritativeRejected {
            return checked_response(
                rejected_response(
                    &request.plan,
                    Diagnostic::error(
                        DIAGNOSTIC_PRECONDITION_REJECTED,
                        None,
                        "execute-time plan/current-state re-check rejected the request",
                    ),
                ),
                &request,
            );
        }

        if self.protocol_execution_state == MockExecutionState::Unavailable
            || self.protocol_preflight_state == MockPreflightState::TransientUnavailable
        {
            return checked_response(unavailable_response(&request.plan), &request);
        }

        if self.protocol_preflight_state == MockPreflightState::PrerequisiteRejected {
            return checked_response(
                rejected_response(
                    &request.plan,
                    Diagnostic::error(
                        DIAGNOSTIC_PRECONDITION_REJECTED,
                        None,
                        "execute-time prerequisite check rejected the request",
                    ),
                ),
                &request,
            );
        }

        if request
            .plan
            .actions
            .iter()
            .any(|action| !supports_mock_action(&action.id))
        {
            return checked_response(
                rejected_response(
                    &request.plan,
                    Diagnostic::error(
                        DIAGNOSTIC_EXECUTION_REJECTED,
                        None,
                        "authoritative execute-time action support check rejected the request",
                    ),
                ),
                &request,
            );
        }

        let response = match self.protocol_execution_state {
            MockExecutionState::Exact => completed_response(
                &request.plan,
                ScheduleStyle::Serial,
                EffectStyle::Exact,
                true,
            ),
            MockExecutionState::Degraded => completed_response(
                &request.plan,
                ScheduleStyle::Serial,
                EffectStyle::Degraded,
                false,
            ),
            MockExecutionState::Unsupported => completed_response(
                &request.plan,
                ScheduleStyle::Serial,
                EffectStyle::Unsupported,
                false,
            ),
            MockExecutionState::Partial => partial_response(&request.plan),
            MockExecutionState::AlternateOrder => completed_response(
                &request.plan,
                ScheduleStyle::AlternateIndependent,
                EffectStyle::None,
                false,
            ),
            MockExecutionState::ParallelIndependent => completed_response(
                &request.plan,
                ScheduleStyle::ParallelIndependent,
                EffectStyle::None,
                false,
            ),
            MockExecutionState::Unavailable | MockExecutionState::AuthoritativeRejected => {
                unreachable!("pre-execution states are handled before realization")
            }
        };

        checked_response(response, &request)
    }

    fn declared_capability_names(&self) -> BTreeSet<&str> {
        self.capabilities()
            .iter()
            .map(|capability| capability.as_str())
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EffectStyle {
    Exact,
    Degraded,
    Unsupported,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ScheduleStyle {
    Serial,
    AlternateIndependent,
    ParallelIndependent,
}

fn supports_mock_action(action_id: &str) -> bool {
    matches!(
        action_id,
        "thermal.domain"
            | "thermal.material"
            | "thermal.solve"
            | "thermal.a"
            | "thermal.b"
            | "thermal.c"
            | "thermal.solve.changed"
    )
}

fn completed_response(
    plan: &MappingPlanDto,
    schedule_style: ScheduleStyle,
    effect_style: EffectStyle,
    include_provenance: bool,
) -> ExecutePlanResponse {
    let execution_batches = schedule(plan, schedule_style);
    let action_reports = plan
        .actions
        .iter()
        .map(|action| ActionExecutionReport {
            action_id: action.id.clone(),
            state: ActionExecutionState::Completed,
            effects: effects_for_action(&action.id, effect_style),
            diagnostics: Vec::new(),
            provenance: None,
            extensions: Default::default(),
        })
        .collect::<Vec<_>>();
    let effects = action_reports
        .iter()
        .flat_map(|report| report.effects.iter().cloned())
        .collect();

    ExecutePlanResponse {
        adapter_protocol_version: ADAPTER_PROTOCOL_VERSION.to_owned(),
        execution: ExecutionOutcome::Completed,
        execution_batches,
        action_reports,
        effects,
        diagnostics: Vec::new(),
        provenance: include_provenance.then(|| ExecutionProvenance {
            producer: "adapter.mock_thermal".to_owned(),
            opaque_references: vec![OpaqueExecutionReference {
                namespace: "mock.job".to_owned(),
                reference: "job-42".to_owned(),
                extensions: Default::default(),
            }],
            extensions: Default::default(),
        }),
        extensions: Default::default(),
    }
}

fn partial_response(plan: &MappingPlanDto) -> ExecutePlanResponse {
    let mut reports = Vec::new();
    let mut execution_batches = Vec::new();
    let mut blocked = BTreeSet::new();

    for action in &plan.actions {
        let dependency_blocked = action
            .dependencies
            .iter()
            .any(|dependency| blocked.contains(dependency));
        if dependency_blocked {
            blocked.insert(action.id.clone());
            reports.push(ActionExecutionReport {
                action_id: action.id.clone(),
                state: ActionExecutionState::SkippedDependency,
                effects: Vec::new(),
                diagnostics: vec![Diagnostic::error(
                    DIAGNOSTIC_DEPENDENCY_SKIPPED,
                    None,
                    format!(
                        "{} was skipped because a dependency did not complete",
                        action.id
                    ),
                )],
                provenance: None,
                extensions: Default::default(),
            });
            continue;
        }

        if action.id == "thermal.material" {
            blocked.insert(action.id.clone());
            execution_batches.push(vec![action.id.clone()]);
            reports.push(ActionExecutionReport {
                action_id: action.id.clone(),
                state: ActionExecutionState::Failed,
                effects: vec![RealizationEffectDto::new(
                    MappingSubjectDto::entity("material.copper"),
                    RealizationQualityDto::Degraded,
                )
                .with_detail("partial backend realization before failure")],
                diagnostics: vec![Diagnostic::error(
                    "adapter.execution_failed",
                    None,
                    "material realization failed after partial work",
                )],
                provenance: None,
                extensions: Default::default(),
            });
        } else {
            execution_batches.push(vec![action.id.clone()]);
            reports.push(ActionExecutionReport {
                action_id: action.id.clone(),
                state: ActionExecutionState::Completed,
                effects: effects_for_action(&action.id, EffectStyle::Exact),
                diagnostics: Vec::new(),
                provenance: None,
                extensions: Default::default(),
            });
        }
    }

    let effects = reports
        .iter()
        .flat_map(|report| report.effects.iter().cloned())
        .collect();
    ExecutePlanResponse {
        adapter_protocol_version: ADAPTER_PROTOCOL_VERSION.to_owned(),
        execution: ExecutionOutcome::Partial,
        execution_batches,
        action_reports: reports,
        effects,
        diagnostics: Vec::new(),
        provenance: None,
        extensions: Default::default(),
    }
}

fn unavailable_response(plan: &MappingPlanDto) -> ExecutePlanResponse {
    let action_reports = plan
        .actions
        .iter()
        .enumerate()
        .map(|(index, action)| ActionExecutionReport {
            action_id: action.id.clone(),
            state: if index == 0 {
                ActionExecutionState::Unavailable
            } else {
                ActionExecutionState::NotStarted
            },
            effects: Vec::new(),
            diagnostics: if index == 0 {
                vec![Diagnostic::error(
                    DIAGNOSTIC_TRANSIENT_UNAVAILABLE,
                    None,
                    "backend environment is temporarily unavailable",
                )]
            } else {
                Vec::new()
            },
            provenance: None,
            extensions: Default::default(),
        })
        .collect();

    ExecutePlanResponse {
        adapter_protocol_version: ADAPTER_PROTOCOL_VERSION.to_owned(),
        execution: ExecutionOutcome::Unavailable,
        execution_batches: Vec::new(),
        action_reports,
        effects: Vec::new(),
        diagnostics: vec![Diagnostic::error(
            DIAGNOSTIC_TRANSIENT_UNAVAILABLE,
            None,
            "authoritative execute-time availability check failed",
        )],
        provenance: None,
        extensions: Default::default(),
    }
}

fn rejected_response(plan: &MappingPlanDto, diagnostic: Diagnostic) -> ExecutePlanResponse {
    ExecutePlanResponse {
        adapter_protocol_version: ADAPTER_PROTOCOL_VERSION.to_owned(),
        execution: ExecutionOutcome::Rejected,
        execution_batches: Vec::new(),
        action_reports: plan
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
        diagnostics: vec![diagnostic],
        provenance: None,
        extensions: Default::default(),
    }
}

fn effects_for_action(action_id: &str, style: EffectStyle) -> Vec<RealizationEffectDto> {
    let subject = match action_id {
        "thermal.domain" => Some("domain.main"),
        "thermal.material" => Some("material.copper"),
        "thermal.solve" => Some("thermal.temperature_field"),
        _ => None,
    };
    let Some(subject) = subject else {
        return Vec::new();
    };

    match style {
        EffectStyle::Exact => vec![RealizationEffectDto::new(
            MappingSubjectDto::entity(subject),
            RealizationQualityDto::Exact,
        )],
        EffectStyle::Degraded if action_id == "thermal.solve" => vec![
            RealizationEffectDto::new(
                MappingSubjectDto::entity(subject),
                RealizationQualityDto::Degraded,
            )
            .with_detail("backend used an approximate realization"),
        ],
        EffectStyle::Unsupported if action_id == "thermal.solve" => vec![
            RealizationEffectDto::new(
                MappingSubjectDto::entity(subject),
                RealizationQualityDto::Unsupported,
            )
            .with_detail("requested semantic realization is not supported"),
        ],
        EffectStyle::Degraded | EffectStyle::Unsupported | EffectStyle::None => Vec::new(),
    }
}

fn schedule(plan: &MappingPlanDto, style: ScheduleStyle) -> Vec<Vec<String>> {
    if style == ScheduleStyle::AlternateIndependent
        && plan.actions.iter().map(|action| action.id.as_str()).collect::<BTreeSet<_>>()
            == BTreeSet::from(["thermal.a", "thermal.b", "thermal.c"])
    {
        return vec![
            vec!["thermal.b".to_owned()],
            vec!["thermal.a".to_owned()],
            vec!["thermal.c".to_owned()],
        ];
    }

    let actions = plan
        .actions
        .iter()
        .map(|action| (action.id.clone(), action.dependencies.clone()))
        .collect::<BTreeMap<_, _>>();
    let mut remaining = actions.keys().cloned().collect::<BTreeSet<_>>();
    let mut completed = BTreeSet::new();
    let mut batches = Vec::new();

    while !remaining.is_empty() {
        let ready = remaining
            .iter()
            .filter(|id| {
                actions
                    .get(*id)
                    .expect("remaining action exists")
                    .iter()
                    .all(|dependency| completed.contains(dependency))
            })
            .cloned()
            .collect::<Vec<_>>();
        debug_assert!(!ready.is_empty(), "MappingPlanDto validation rejects cycles");

        let batch = if style == ScheduleStyle::ParallelIndependent {
            ready
        } else {
            vec![ready[0].clone()]
        };
        for id in &batch {
            remaining.remove(id);
            completed.insert(id.clone());
        }
        batches.push(batch);
    }
    batches
}

fn checked_response(
    mut response: ExecutePlanResponse,
    request: &ExecutePlanRequest,
) -> Result<ExecutePlanResponse, ExecutionError> {
    response.validate_against(request)?;
    Ok(response)
}
