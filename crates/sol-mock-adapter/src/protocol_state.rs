//! Protocol-facing failure and current-state reference behavior for MockAdapter.
//!
//! This module exercises the published Adapter Protocol 0.1 failure algebra and
//! repeated-execution preconditions without adding transport policy or semantic lifecycle state.

use sol_adapter_protocol::{
    protocol_diagnostic, ActionExecutionReport, ActionExecutionState,
    AdapterProtocolDiagnosticContext, ExecutePlanRequest, ExecutePlanResponse, ExecutionOutcome,
    PlanOperation, PlanOperationIdempotency, ProtocolFailure, ProtocolOperation,
    SideEffectEvidence, ValidatePlanRequest, ValidatePlanResponse, ADAPTER_PROTOCOL_VERSION,
    DIAGNOSTIC_PRECONDITION_REJECTED, PRECONDITION_ALREADY_REALIZED,
    PRECONDITION_PARTIAL_PRIOR_EXECUTION, PRECONDITION_UNRESOLVED_PREREQUISITE,
};

use crate::MockAdapter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MockProtocolFailureState {
    #[default]
    None,
    CompatibilityNotEstablished,
    InvalidRequest,
    ValidateOperational,
    ExecuteOperationalBeforeSideEffect,
    ExecuteOperationalAmbiguous,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MockPriorExecutionState {
    #[default]
    Fresh,
    AlreadyRealized,
    PartialPriorExecution,
    UnresolvedPrerequisite,
}

impl MockAdapter {
    pub fn with_protocol_failure_state(mut self, state: MockProtocolFailureState) -> Self {
        self.protocol_failure_state = state;
        self
    }

    pub fn with_prior_execution_state(mut self, state: MockPriorExecutionState) -> Self {
        self.protocol_prior_execution_state = state;
        self
    }

    pub fn describe_adapter_operation(
        &self,
    ) -> Result<sol_adapter_protocol::AdapterDescription, ProtocolFailure> {
        if self.protocol_failure_state == MockProtocolFailureState::CompatibilityNotEstablished {
            return Err(compatibility_failure());
        }

        self.describe_adapter().map_err(|error| {
            operational_failure(
                format!("adapter could not produce an adapter description: {error}"),
                SideEffectEvidence::None,
                ProtocolOperation::DescribeAdapter,
            )
        })
    }

    pub fn validate_plan_operation(
        &self,
        request: &ValidatePlanRequest,
    ) -> Result<ValidatePlanResponse, ProtocolFailure> {
        if let Some(failure) = self.validate_failure() {
            return Err(failure);
        }

        self.validate_plan(request)
            .map_err(|error| invalid_request_failure(error.to_string()))
    }

    pub fn execute_plan_operation(
        &self,
        request: &ExecutePlanRequest,
    ) -> Result<ExecutePlanResponse, ProtocolFailure> {
        if let Some(failure) = self.execute_failure() {
            return Err(failure);
        }

        if self.protocol_prior_execution_state != MockPriorExecutionState::Fresh {
            return Ok(prior_execution_rejection(
                request,
                self.protocol_prior_execution_state,
            ));
        }

        self.execute_plan(request)
            .map_err(|error| invalid_request_failure(error.to_string()))
    }

    fn validate_failure(&self) -> Option<ProtocolFailure> {
        match self.protocol_failure_state {
            MockProtocolFailureState::None
            | MockProtocolFailureState::ExecuteOperationalBeforeSideEffect
            | MockProtocolFailureState::ExecuteOperationalAmbiguous => None,
            MockProtocolFailureState::CompatibilityNotEstablished => Some(compatibility_failure()),
            MockProtocolFailureState::InvalidRequest => Some(invalid_request_failure(
                "request cannot be accepted as a Protocol 0.1 operation",
            )),
            MockProtocolFailureState::ValidateOperational => Some(operational_failure(
                "adapter could not produce a validate_plan response",
                SideEffectEvidence::None,
                ProtocolOperation::ValidatePlan,
            )),
        }
    }

    fn execute_failure(&self) -> Option<ProtocolFailure> {
        match self.protocol_failure_state {
            MockProtocolFailureState::None | MockProtocolFailureState::ValidateOperational => None,
            MockProtocolFailureState::CompatibilityNotEstablished => Some(compatibility_failure()),
            MockProtocolFailureState::InvalidRequest => Some(invalid_request_failure(
                "request cannot be accepted as a Protocol 0.1 operation",
            )),
            MockProtocolFailureState::ExecuteOperationalBeforeSideEffect => {
                Some(operational_failure(
                    "execute_plan failed before backend execution began",
                    SideEffectEvidence::None,
                    ProtocolOperation::ExecutePlan,
                ))
            }
            MockProtocolFailureState::ExecuteOperationalAmbiguous => Some(operational_failure(
                "execute_plan did not produce a valid response after execution may have begun",
                SideEffectEvidence::MayHaveOccurred,
                ProtocolOperation::ExecutePlan,
            )),
        }
    }
}

pub const fn mock_plan_operation_idempotency(operation: PlanOperation) -> PlanOperationIdempotency {
    sol_adapter_protocol::plan_operation_idempotency(operation)
}

fn compatibility_failure() -> ProtocolFailure {
    ProtocolFailure::compatibility_not_established("required compatibility evidence is missing")
        .expect("published compatibility failure is valid")
}

fn invalid_request_failure(detail: impl Into<String>) -> ProtocolFailure {
    ProtocolFailure::invalid_request(detail).expect("published invalid-request failure is valid")
}

fn operational_failure(
    detail: impl Into<String>,
    side_effects: SideEffectEvidence,
    operation: ProtocolOperation,
) -> ProtocolFailure {
    let failure = ProtocolFailure::operational(detail, side_effects)
        .expect("published operational failure is valid");
    failure
        .validate_for(operation)
        .expect("mock operational side-effect evidence matches the operation");
    failure
}

fn prior_execution_rejection(
    request: &ExecutePlanRequest,
    state: MockPriorExecutionState,
) -> ExecutePlanResponse {
    let (precondition, detail) = match state {
        MockPriorExecutionState::Fresh => unreachable!("fresh state does not reject execution"),
        MockPriorExecutionState::AlreadyRealized => (
            PRECONDITION_ALREADY_REALIZED,
            "current state shows the plan is already realized",
        ),
        MockPriorExecutionState::PartialPriorExecution => (
            PRECONDITION_PARTIAL_PRIOR_EXECUTION,
            "current state shows partial prior execution",
        ),
        MockPriorExecutionState::UnresolvedPrerequisite => (
            PRECONDITION_UNRESOLVED_PREREQUISITE,
            "current execution prerequisite is unresolved",
        ),
    };

    let diagnostic = protocol_diagnostic(
        DIAGNOSTIC_PRECONDITION_REJECTED,
        detail,
        AdapterProtocolDiagnosticContext {
            precondition: Some(precondition.to_owned()),
            ..Default::default()
        },
    )
    .expect("published precondition diagnostic context is valid");

    let mut response = ExecutePlanResponse {
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
        diagnostics: vec![diagnostic],
        provenance: None,
        extensions: Default::default(),
    };

    response
        .validate_against(request)
        .expect("mock prior-execution rejection must satisfy published execution invariants");
    response
}
