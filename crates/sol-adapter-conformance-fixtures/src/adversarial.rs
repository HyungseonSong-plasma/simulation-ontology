use sol_adapter_conformance::{
    ConformanceCaseId, ConformanceCaseRecord, ConformanceCaseResult, ConformanceEvidence,
    ConformanceReport, ConformanceScope, ConformanceViolation, HarnessFailure, HarnessFailureKind,
    PublishedContract,
};
use sol_adapter_conformance_harness::{
    AdapterProtocolObservation, ExternalAdapterCommand, ExternalAdapterHarness, ExternalAdapterLaunch,
};
use sol_adapter_protocol::{
    ExecutePlanRequest, ExecutePlanResponse, FailureCategory, PreflightOutcome, ProtocolFailure,
    ProtocolOperation, SideEffectEvidence, ValidatePlanRequest, ValidatePlanResponse,
    DIAGNOSTIC_MISSING_CAPABILITY, DIAGNOSTIC_PRECONDITION_REJECTED, DIAGNOSTIC_TARGET_MISMATCH,
    DIAGNOSTIC_TRANSIENT_UNAVAILABLE, DIAGNOSTIC_UNSUPPORTED_ACTION,
};
use std::fmt::Display;
use std::fs;
use std::path::{Path, PathBuf};

const VALIDATE_REQUEST: &str = "validate-plan-accepted-request.json";
const EXECUTE_REQUEST: &str = "execute-plan-thermal-request.json";
const EXECUTE_CHANGED_REQUEST: &str = "execute-plan-changed-request.json";
const EXECUTE_INDEPENDENT_REQUEST: &str = "execute-plan-independent-request.json";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PublishedAdversarialScenario {
    CompatibilityMissing,
    TargetMismatch,
    MissingCapability,
    UnsupportedAction,
    PreflightPrerequisiteRejected,
    PreflightTransientUnavailable,
    ExecutionPartial,
    ExecutionUnsupported,
    ExecutionUnavailable,
    ExecutionAuthoritativeRejected,
    ExecutionAlternateOrder,
    ExecutionParallelIndependent,
    PriorAlreadyRealized,
    PriorPartialExecution,
    PriorUnresolvedPrerequisite,
    ValidateInvalidRequestFailure,
    ValidateOperationalFailure,
    ExecuteFailureBeforeSideEffect,
    ExecuteFailureAmbiguous,
    ExecuteResponseLoss,
    DependencyScheduleViolation,
    AggregateEffectMismatch,
    ProvenanceIdentityLeakage,
}

impl PublishedAdversarialScenario {
    pub const ALL: [Self; 23] = [
        Self::CompatibilityMissing,
        Self::TargetMismatch,
        Self::MissingCapability,
        Self::UnsupportedAction,
        Self::PreflightPrerequisiteRejected,
        Self::PreflightTransientUnavailable,
        Self::ExecutionPartial,
        Self::ExecutionUnsupported,
        Self::ExecutionUnavailable,
        Self::ExecutionAuthoritativeRejected,
        Self::ExecutionAlternateOrder,
        Self::ExecutionParallelIndependent,
        Self::PriorAlreadyRealized,
        Self::PriorPartialExecution,
        Self::PriorUnresolvedPrerequisite,
        Self::ValidateInvalidRequestFailure,
        Self::ValidateOperationalFailure,
        Self::ExecuteFailureBeforeSideEffect,
        Self::ExecuteFailureAmbiguous,
        Self::ExecuteResponseLoss,
        Self::DependencyScheduleViolation,
        Self::AggregateEffectMismatch,
        Self::ProvenanceIdentityLeakage,
    ];

    pub const fn case_id(self) -> &'static str {
        match self {
            Self::CompatibilityMissing => "compatibility.missing",
            Self::TargetMismatch => "preflight.target-mismatch",
            Self::MissingCapability => "preflight.missing-capability",
            Self::UnsupportedAction => "preflight.unsupported-action",
            Self::PreflightPrerequisiteRejected => "preflight.prerequisite-rejected",
            Self::PreflightTransientUnavailable => "preflight.transient-unavailable",
            Self::ExecutionPartial => "execution.partial",
            Self::ExecutionUnsupported => "execution.unsupported",
            Self::ExecutionUnavailable => "execution.unavailable",
            Self::ExecutionAuthoritativeRejected => "execution.authoritative-rejected",
            Self::ExecutionAlternateOrder => "scheduling.alternate-order",
            Self::ExecutionParallelIndependent => "scheduling.parallel-independent",
            Self::PriorAlreadyRealized => "execution.prior-already-realized",
            Self::PriorPartialExecution => "execution.prior-partial",
            Self::PriorUnresolvedPrerequisite => "execution.prior-unresolved-prerequisite",
            Self::ValidateInvalidRequestFailure => "failure.validate-invalid-request",
            Self::ValidateOperationalFailure => "failure.validate-operational",
            Self::ExecuteFailureBeforeSideEffect => "failure.execute-before-side-effect",
            Self::ExecuteFailureAmbiguous => "failure.execute-ambiguous",
            Self::ExecuteResponseLoss => "transport.execute-response-loss",
            Self::DependencyScheduleViolation => "counterexample.dependency-schedule",
            Self::AggregateEffectMismatch => "counterexample.aggregate-effect",
            Self::ProvenanceIdentityLeakage => "counterexample.provenance-identity",
        }
    }
}

/// Runs versioned adversarial Protocol/Public Contract cases through externally supplied
/// adapter commands. The command is deliberately supplied by the caller: profile/env/config
/// spelling is adapter-specific test setup and is not a SOL semantic or stable CLI contract.
#[derive(Debug, Clone)]
pub struct PublishedAdversarialFixtureSuite {
    fixture_dir: PathBuf,
    harness: ExternalAdapterHarness,
}

impl PublishedAdversarialFixtureSuite {
    pub fn new(fixture_dir: impl Into<PathBuf>) -> Self {
        Self {
            fixture_dir: fixture_dir.into(),
            harness: ExternalAdapterHarness::default(),
        }
    }

    pub fn with_harness(mut self, harness: ExternalAdapterHarness) -> Self {
        self.harness = harness;
        self
    }

    pub fn run_case(
        &self,
        scenario: PublishedAdversarialScenario,
        command: ExternalAdapterCommand,
    ) -> ConformanceCaseRecord {
        match self.run_case_inner(scenario, command) {
            Ok(record) => record,
            Err(failure) => harness_case(
                scenario.case_id(),
                scope_for(scenario),
                failure,
            ),
        }
    }

    pub fn run_matrix(
        &self,
        cases: impl IntoIterator<Item = (PublishedAdversarialScenario, ExternalAdapterCommand)>,
    ) -> ConformanceReport {
        let records = cases
            .into_iter()
            .map(|(scenario, command)| self.run_case(scenario, command))
            .collect();
        ConformanceReport::new(records)
            .expect("adversarial matrix requires each scenario at most once")
    }

    fn run_case_inner(
        &self,
        scenario: PublishedAdversarialScenario,
        command: ExternalAdapterCommand,
    ) -> Result<ConformanceCaseRecord, HarnessFailure> {
        match scenario {
            PublishedAdversarialScenario::CompatibilityMissing => {
                self.run_bootstrap_failure(scenario, command, "protocol-failure-compatibility-missing.json")
            }
            PublishedAdversarialScenario::TargetMismatch
            | PublishedAdversarialScenario::MissingCapability
            | PublishedAdversarialScenario::UnsupportedAction
            | PublishedAdversarialScenario::PreflightPrerequisiteRejected
            | PublishedAdversarialScenario::PreflightTransientUnavailable => {
                self.run_preflight(scenario, command)
            }
            PublishedAdversarialScenario::ValidateInvalidRequestFailure => self.run_validate_failure(
                scenario,
                command,
                "protocol-failure-invalid-request.json",
            ),
            PublishedAdversarialScenario::ValidateOperationalFailure => self.run_validate_failure(
                scenario,
                command,
                "protocol-failure-validate-operational.json",
            ),
            PublishedAdversarialScenario::ExecuteFailureBeforeSideEffect => self.run_execute_failure(
                scenario,
                command,
                "protocol-failure-execute-before-side-effect.json",
            ),
            PublishedAdversarialScenario::ExecuteFailureAmbiguous => self.run_execute_failure(
                scenario,
                command,
                "protocol-failure-execute-ambiguous.json",
            ),
            PublishedAdversarialScenario::ExecutionPartial => self.run_execute_fixture(
                scenario,
                command,
                EXECUTE_REQUEST,
                "execute-plan-partial-response.json",
            ),
            PublishedAdversarialScenario::ExecutionUnsupported => self.run_execute_fixture(
                scenario,
                command,
                EXECUTE_REQUEST,
                "execute-plan-unsupported-response.json",
            ),
            PublishedAdversarialScenario::ExecutionUnavailable => self.run_execute_fixture(
                scenario,
                command,
                EXECUTE_REQUEST,
                "execute-plan-unavailable-response.json",
            ),
            PublishedAdversarialScenario::ExecutionAuthoritativeRejected => self.run_execute_fixture(
                scenario,
                command,
                EXECUTE_CHANGED_REQUEST,
                "execute-plan-authoritative-rejection-response.json",
            ),
            PublishedAdversarialScenario::ExecutionAlternateOrder => self.run_execute_fixture(
                scenario,
                command,
                EXECUTE_INDEPENDENT_REQUEST,
                "execute-plan-alternate-order-response.json",
            ),
            PublishedAdversarialScenario::ExecutionParallelIndependent => self.run_execute_fixture(
                scenario,
                command,
                EXECUTE_INDEPENDENT_REQUEST,
                "execute-plan-parallel-response.json",
            ),
            PublishedAdversarialScenario::PriorAlreadyRealized => self.run_execute_fixture(
                scenario,
                command,
                EXECUTE_REQUEST,
                "execute-plan-already-realized-response.json",
            ),
            PublishedAdversarialScenario::PriorPartialExecution => self.run_execute_fixture(
                scenario,
                command,
                EXECUTE_REQUEST,
                "execute-plan-partial-prior-rejection-response.json",
            ),
            PublishedAdversarialScenario::PriorUnresolvedPrerequisite => self.run_execute_fixture(
                scenario,
                command,
                EXECUTE_REQUEST,
                "execute-plan-unresolved-prerequisite-response.json",
            ),
            PublishedAdversarialScenario::ExecuteResponseLoss
            | PublishedAdversarialScenario::DependencyScheduleViolation
            | PublishedAdversarialScenario::AggregateEffectMismatch
            | PublishedAdversarialScenario::ProvenanceIdentityLeakage => {
                self.run_execute_boundary_probe(scenario, command)
            }
        }
    }

    fn run_bootstrap_failure(
        &self,
        scenario: PublishedAdversarialScenario,
        command: ExternalAdapterCommand,
        expected_fixture: &str,
    ) -> Result<ConformanceCaseRecord, HarnessFailure> {
        let expected = self.load_fixture(expected_fixture, ProtocolFailure::from_json)?;
        match self.harness.launch(command) {
            Ok(ExternalAdapterLaunch::ProtocolFailure(observed)) if observed == expected => Ok(
                conformant_case(
                    scenario.case_id(),
                    scope_for(scenario),
                    "bootstrap returned the published compatibility ProtocolFailure",
                ),
            ),
            Ok(ExternalAdapterLaunch::ProtocolFailure(_)) => Ok(nonconformant_case(
                scenario.case_id(),
                scope_for(scenario),
                "bootstrap ProtocolFailure differed from the published adversarial expectation",
            )),
            Ok(ExternalAdapterLaunch::Ready(session)) => {
                let _ = session.shutdown();
                Ok(nonconformant_case(
                    scenario.case_id(),
                    scope_for(scenario),
                    "bootstrap unexpectedly established a ready session",
                ))
            }
            Err(error) => Err(error.into_report_failure()),
        }
    }

    fn run_preflight(
        &self,
        scenario: PublishedAdversarialScenario,
        command: ExternalAdapterCommand,
    ) -> Result<ConformanceCaseRecord, HarnessFailure> {
        let mut request = self.load_fixture(VALIDATE_REQUEST, ValidatePlanRequest::from_json)?;
        let mut session = self.ready_session(command)?;

        match scenario {
            PublishedAdversarialScenario::TargetMismatch => {
                request.target.target = unmatched_target(session.description());
            }
            PublishedAdversarialScenario::MissingCapability => {
                request.target.required_capabilities = vec![missing_capability(session.description())];
            }
            PublishedAdversarialScenario::UnsupportedAction => {
                let last = request
                    .plan
                    .actions
                    .last_mut()
                    .expect("published validate fixture contains actions");
                last.id = "sol.conformance.unsupported-action".to_owned();
            }
            PublishedAdversarialScenario::PreflightPrerequisiteRejected
            | PublishedAdversarialScenario::PreflightTransientUnavailable => {}
            _ => unreachable!("non-preflight scenario reached preflight runner"),
        }

        let observation = session
            .validate_plan(&request)
            .map_err(|error| error.into_report_failure())?;
        let _ = session.shutdown();

        let AdapterProtocolObservation::Success(response) = observation else {
            return Ok(nonconformant_case(
                scenario.case_id(),
                scope_for(scenario),
                "adversarial preflight expected a valid negative response, not ProtocolFailure",
            ));
        };

        let conforms = match scenario {
            PublishedAdversarialScenario::TargetMismatch => {
                !response.target_compatible
                    && response.preflight == PreflightOutcome::Rejected
                    && has_diagnostic(&response, DIAGNOSTIC_TARGET_MISMATCH)
            }
            PublishedAdversarialScenario::MissingCapability => {
                !response.capabilities_satisfied
                    && response.preflight == PreflightOutcome::Rejected
                    && has_diagnostic(&response, DIAGNOSTIC_MISSING_CAPABILITY)
            }
            PublishedAdversarialScenario::UnsupportedAction => {
                response.preflight == PreflightOutcome::Rejected
                    && has_diagnostic(&response, DIAGNOSTIC_UNSUPPORTED_ACTION)
            }
            PublishedAdversarialScenario::PreflightPrerequisiteRejected => {
                response.preflight == PreflightOutcome::Rejected
                    && has_diagnostic(&response, DIAGNOSTIC_PRECONDITION_REJECTED)
            }
            PublishedAdversarialScenario::PreflightTransientUnavailable => {
                response.preflight == PreflightOutcome::Unavailable
                    && has_diagnostic(&response, DIAGNOSTIC_TRANSIENT_UNAVAILABLE)
            }
            _ => false,
        };

        Ok(if conforms {
            conformant_case(
                scenario.case_id(),
                scope_for(scenario),
                "external adapter produced the required published valid-negative preflight semantics",
            )
        } else {
            nonconformant_case(
                scenario.case_id(),
                scope_for(scenario),
                "external adapter preflight contradicted the published adversarial expectation",
            )
        })
    }

    fn run_validate_failure(
        &self,
        scenario: PublishedAdversarialScenario,
        command: ExternalAdapterCommand,
        expected_fixture: &str,
    ) -> Result<ConformanceCaseRecord, HarnessFailure> {
        let request = self.load_fixture(VALIDATE_REQUEST, ValidatePlanRequest::from_json)?;
        let expected = self.load_fixture(expected_fixture, ProtocolFailure::from_json)?;
        let mut session = self.ready_session(command)?;
        let observed = session
            .validate_plan(&request)
            .map_err(|error| error.into_report_failure())?;
        let _ = session.shutdown();
        Ok(protocol_failure_case(
            scenario,
            observed,
            expected,
            ProtocolOperation::ValidatePlan,
        ))
    }

    fn run_execute_failure(
        &self,
        scenario: PublishedAdversarialScenario,
        command: ExternalAdapterCommand,
        expected_fixture: &str,
    ) -> Result<ConformanceCaseRecord, HarnessFailure> {
        let request = self.load_fixture(EXECUTE_REQUEST, ExecutePlanRequest::from_json)?;
        let expected = self.load_fixture(expected_fixture, ProtocolFailure::from_json)?;
        let mut session = self.ready_session(command)?;
        let observed = session
            .execute_plan(&request)
            .map_err(|error| error.into_report_failure())?;
        let _ = session.shutdown();
        Ok(protocol_failure_case(
            scenario,
            observed,
            expected,
            ProtocolOperation::ExecutePlan,
        ))
    }

    fn run_execute_fixture(
        &self,
        scenario: PublishedAdversarialScenario,
        command: ExternalAdapterCommand,
        request_fixture: &str,
        response_fixture: &str,
    ) -> Result<ConformanceCaseRecord, HarnessFailure> {
        let request = self.load_fixture(request_fixture, ExecutePlanRequest::from_json)?;
        let mut expected = self.load_fixture(response_fixture, ExecutePlanResponse::from_json)?;
        expected
            .validate_against(&request)
            .map_err(|error| fixture_failure(self.fixture_dir.join(response_fixture), error))?;

        let mut session = self.ready_session(command)?;
        let observed = session
            .execute_plan(&request)
            .map_err(|error| error.into_report_failure())?;
        let _ = session.shutdown();
        let AdapterProtocolObservation::Success(mut response) = observed else {
            return Ok(nonconformant_case(
                scenario.case_id(),
                scope_for(scenario),
                "published adversarial execution expected a valid Protocol response, not ProtocolFailure",
            ));
        };

        if response.validate_against(&request).is_ok() && response == expected {
            Ok(conformant_case(
                scenario.case_id(),
                scope_for(scenario),
                "external adapter matched the published adversarial execution fixture",
            ))
        } else {
            Ok(nonconformant_case(
                scenario.case_id(),
                scope_for(scenario),
                "external adapter execution contradicted the published adversarial fixture",
            ))
        }
    }

    fn run_execute_boundary_probe(
        &self,
        scenario: PublishedAdversarialScenario,
        command: ExternalAdapterCommand,
    ) -> Result<ConformanceCaseRecord, HarnessFailure> {
        let request = self.load_fixture(EXECUTE_REQUEST, ExecutePlanRequest::from_json)?;
        let mut session = self.ready_session(command)?;
        let observation = session.execute_plan(&request);
        let _ = session.shutdown();

        match scenario {
            PublishedAdversarialScenario::ExecuteResponseLoss => match observation {
                Err(error) => Ok(harness_case(
                    scenario.case_id(),
                    scope_for(scenario),
                    error.into_report_failure(),
                )),
                Ok(_) => Ok(nonconformant_case(
                    scenario.case_id(),
                    scope_for(scenario),
                    "response-loss probe unexpectedly produced a Protocol observation",
                )),
            },
            PublishedAdversarialScenario::DependencyScheduleViolation
            | PublishedAdversarialScenario::AggregateEffectMismatch
            | PublishedAdversarialScenario::ProvenanceIdentityLeakage => match observation {
                Ok(AdapterProtocolObservation::Success(mut response)) => {
                    if response.validate_against(&request).is_err() {
                        Ok(nonconformant_case(
                            scenario.case_id(),
                            scope_for(scenario),
                            "parsable execute response violated published request-relative semantics",
                        ))
                    } else {
                        Ok(conformant_case(
                            scenario.case_id(),
                            scope_for(scenario),
                            "execute response satisfied published request-relative semantics",
                        ))
                    }
                }
                Ok(AdapterProtocolObservation::ProtocolFailure(_)) => Ok(nonconformant_case(
                    scenario.case_id(),
                    scope_for(scenario),
                    "semantic-response counterexample unexpectedly returned ProtocolFailure",
                )),
                Err(error) => Ok(harness_case(
                    scenario.case_id(),
                    scope_for(scenario),
                    error.into_report_failure(),
                )),
            },
            _ => unreachable!("non-boundary scenario reached boundary probe runner"),
        }
    }

    fn ready_session(
        &self,
        command: ExternalAdapterCommand,
    ) -> Result<sol_adapter_conformance_harness::ExternalAdapterSession, HarnessFailure> {
        match self.harness.launch(command) {
            Ok(ExternalAdapterLaunch::Ready(session)) => Ok(session),
            Ok(ExternalAdapterLaunch::ProtocolFailure(failure)) => Err(HarnessFailure::new(
                HarnessFailureKind::RunnerInvariant,
                format!("scenario requires a ready session but bootstrap returned ProtocolFailure: {}", failure.detail),
            )
            .expect("bootstrap failure detail is non-blank")),
            Err(error) => Err(error.into_report_failure()),
        }
    }

    fn load_fixture<T, E>(
        &self,
        name: &str,
        parse: impl FnOnce(&str) -> Result<T, E>,
    ) -> Result<T, HarnessFailure>
    where
        E: Display,
    {
        let path = self.fixture_dir.join(name);
        let input = fs::read_to_string(&path).map_err(|error| fixture_failure(&path, error))?;
        parse(&input).map_err(|error| fixture_failure(&path, error))
    }
}

fn protocol_failure_case<T>(
    scenario: PublishedAdversarialScenario,
    observed: AdapterProtocolObservation<T>,
    expected: ProtocolFailure,
    operation: ProtocolOperation,
) -> ConformanceCaseRecord {
    match observed {
        AdapterProtocolObservation::ProtocolFailure(failure)
            if failure == expected && failure.validate_for(operation).is_ok() =>
        {
            conformant_case(
                scenario.case_id(),
                scope_for(scenario),
                "external adapter returned the published expected ProtocolFailure with valid side-effect evidence",
            )
        }
        AdapterProtocolObservation::ProtocolFailure(_) => nonconformant_case(
            scenario.case_id(),
            scope_for(scenario),
            "ProtocolFailure differed from the published adversarial expectation",
        ),
        AdapterProtocolObservation::Success(_) => nonconformant_case(
            scenario.case_id(),
            scope_for(scenario),
            "adversarial case expected ProtocolFailure but received a success response",
        ),
    }
}

fn has_diagnostic(response: &ValidatePlanResponse, code: &str) -> bool {
    response.diagnostics.iter().any(|diagnostic| diagnostic.code == code)
}

fn unmatched_target(description: &sol_adapter_protocol::AdapterDescription) -> String {
    let declared = description
        .targets
        .iter()
        .map(|target| target.target.as_str())
        .collect::<Vec<_>>();
    let mut candidate = "sol.conformance.unmatched-target".to_owned();
    while declared.contains(&candidate.as_str()) {
        candidate.push_str(".missing");
    }
    candidate
}

fn missing_capability(description: &sol_adapter_protocol::AdapterDescription) -> String {
    let declared = description
        .targets
        .iter()
        .flat_map(|target| target.capabilities.iter())
        .map(|capability| capability.capability.as_str())
        .collect::<Vec<_>>();
    let mut candidate = "sol.conformance.missing-capability".to_owned();
    while declared.contains(&candidate.as_str()) {
        candidate.push_str(".missing");
    }
    candidate
}

fn scope_for(scenario: PublishedAdversarialScenario) -> ConformanceScope {
    match scenario {
        PublishedAdversarialScenario::CompatibilityMissing => {
            ConformanceScope::Compatibility(PublishedContract::AdapterProtocol)
        }
        PublishedAdversarialScenario::TargetMismatch
        | PublishedAdversarialScenario::MissingCapability
        | PublishedAdversarialScenario::UnsupportedAction
        | PublishedAdversarialScenario::PreflightPrerequisiteRejected
        | PublishedAdversarialScenario::PreflightTransientUnavailable
        | PublishedAdversarialScenario::ValidateInvalidRequestFailure
        | PublishedAdversarialScenario::ValidateOperationalFailure => {
            ConformanceScope::OperationSemantics(ProtocolOperation::ValidatePlan)
        }
        PublishedAdversarialScenario::ExecutionAlternateOrder
        | PublishedAdversarialScenario::ExecutionParallelIndependent
        | PublishedAdversarialScenario::DependencyScheduleViolation => ConformanceScope::Scheduling,
        PublishedAdversarialScenario::ExecuteFailureBeforeSideEffect
        | PublishedAdversarialScenario::ExecuteFailureAmbiguous
        | PublishedAdversarialScenario::ExecuteResponseLoss => ConformanceScope::FailureAndReplay,
        PublishedAdversarialScenario::ProvenanceIdentityLeakage => ConformanceScope::Provenance,
        PublishedAdversarialScenario::ExecutionPartial
        | PublishedAdversarialScenario::ExecutionUnsupported
        | PublishedAdversarialScenario::ExecutionUnavailable
        | PublishedAdversarialScenario::ExecutionAuthoritativeRejected
        | PublishedAdversarialScenario::PriorAlreadyRealized
        | PublishedAdversarialScenario::PriorPartialExecution
        | PublishedAdversarialScenario::PriorUnresolvedPrerequisite
        | PublishedAdversarialScenario::AggregateEffectMismatch => {
            ConformanceScope::OperationSemantics(ProtocolOperation::ExecutePlan)
        }
    }
}

fn fixture_failure(subject: impl AsRef<Path>, error: impl Display) -> HarnessFailure {
    HarnessFailure::new(
        HarnessFailureKind::FixtureLoad,
        format!("{}: {error}", subject.as_ref().display()),
    )
    .expect("fixture failure detail is non-blank")
}

fn conformant_case(
    id: &'static str,
    scope: ConformanceScope,
    evidence: &'static str,
) -> ConformanceCaseRecord {
    ConformanceCaseRecord::new(
        ConformanceCaseId::new(id).expect("fixed case id is valid"),
        scope,
        ConformanceCaseResult::Conformant(
            ConformanceEvidence::new(evidence).expect("fixed evidence is non-blank"),
        ),
    )
}

fn nonconformant_case(
    id: &'static str,
    scope: ConformanceScope,
    violation: &'static str,
) -> ConformanceCaseRecord {
    ConformanceCaseRecord::new(
        ConformanceCaseId::new(id).expect("fixed case id is valid"),
        scope,
        ConformanceCaseResult::NonConformant(
            ConformanceViolation::new(violation).expect("fixed violation is non-blank"),
        ),
    )
}

fn harness_case(
    id: &'static str,
    scope: ConformanceScope,
    failure: HarnessFailure,
) -> ConformanceCaseRecord {
    ConformanceCaseRecord::new(
        ConformanceCaseId::new(id).expect("fixed case id is valid"),
        scope,
        ConformanceCaseResult::HarnessFailure(failure),
    )
}

#[allow(dead_code)]
fn _failure_boundary_type_checks(failure: &ProtocolFailure) -> bool {
    matches!(failure.category, FailureCategory::Compatibility | FailureCategory::InvalidRequest | FailureCategory::Operational)
        && matches!(failure.side_effects, SideEffectEvidence::None | SideEffectEvidence::MayHaveOccurred)
}
