use super::common::{
    conformant_case, nonconformant_case, protocol_failure_case, scope_for,
    unexpected_bootstrap_failure_case, CaseLaunch,
};
use super::{PublishedAdversarialFixtureSuite, PublishedAdversarialScenario};
use sol_adapter_conformance::{ConformanceCaseRecord, HarnessFailure};
use sol_adapter_conformance_harness::{
    AdapterProtocolObservation, ExternalAdapterCommand, ExternalAdapterLaunch,
};
use sol_adapter_protocol::{
    PreflightOutcome, ProtocolFailure, ProtocolOperation, ValidatePlanRequest,
    ValidatePlanResponse, DIAGNOSTIC_MISSING_CAPABILITY, DIAGNOSTIC_PRECONDITION_REJECTED,
    DIAGNOSTIC_TARGET_MISMATCH, DIAGNOSTIC_TRANSIENT_UNAVAILABLE, DIAGNOSTIC_UNSUPPORTED_ACTION,
};

const VALIDATE_REQUEST: &str = "validate-plan-accepted-request.json";

impl PublishedAdversarialFixtureSuite {
    pub(super) fn run_bootstrap_failure(
        &self,
        scenario: PublishedAdversarialScenario,
        command: ExternalAdapterCommand,
        expected_fixture: &str,
    ) -> Result<ConformanceCaseRecord, HarnessFailure> {
        let expected = self.load_fixture(expected_fixture, ProtocolFailure::from_json)?;
        match self.harness.launch(command) {
            Ok(ExternalAdapterLaunch::ProtocolFailure(observed)) if observed == expected => {
                Ok(conformant_case(
                    scenario.case_id(),
                    scope_for(scenario),
                    "bootstrap returned the published compatibility ProtocolFailure",
                ))
            }
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

    pub(super) fn run_preflight(
        &self,
        scenario: PublishedAdversarialScenario,
        command: ExternalAdapterCommand,
    ) -> Result<ConformanceCaseRecord, HarnessFailure> {
        let mut request = self.load_fixture(VALIDATE_REQUEST, ValidatePlanRequest::from_json)?;
        let mut session = match self.launch_case(command)? {
            CaseLaunch::Ready(session) => session,
            CaseLaunch::ProtocolFailure => {
                return Ok(unexpected_bootstrap_failure_case(scenario));
            }
        };

        match scenario {
            PublishedAdversarialScenario::TargetMismatch => {
                request.target.target = unmatched_target(session.description());
            }
            PublishedAdversarialScenario::MissingCapability => {
                request.target.required_capabilities =
                    vec![missing_capability(session.description())];
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

    pub(super) fn run_validate_failure(
        &self,
        scenario: PublishedAdversarialScenario,
        command: ExternalAdapterCommand,
        expected_fixture: &str,
    ) -> Result<ConformanceCaseRecord, HarnessFailure> {
        let request = self.load_fixture(VALIDATE_REQUEST, ValidatePlanRequest::from_json)?;
        let expected = self.load_fixture(expected_fixture, ProtocolFailure::from_json)?;
        let mut session = match self.launch_case(command)? {
            CaseLaunch::Ready(session) => session,
            CaseLaunch::ProtocolFailure => {
                return Ok(unexpected_bootstrap_failure_case(scenario));
            }
        };
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
}

fn has_diagnostic(response: &ValidatePlanResponse, code: &str) -> bool {
    response
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == code)
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
