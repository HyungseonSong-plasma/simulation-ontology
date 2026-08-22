use super::common::{
    conformant_case, harness_case, nonconformant_case, protocol_failure_case, scope_for,
    unexpected_bootstrap_failure_case, CaseLaunch,
};
use super::{PublishedAdversarialFixtureSuite, PublishedAdversarialScenario};
use sol_adapter_conformance::{ConformanceCaseRecord, HarnessFailure};
use sol_adapter_conformance_harness::{AdapterProtocolObservation, ExternalAdapterCommand};
use sol_adapter_protocol::{
    ExecutePlanRequest, ExecutePlanResponse, ProtocolFailure, ProtocolOperation,
};

const EXECUTE_REQUEST: &str = "execute-plan-thermal-request.json";

impl PublishedAdversarialFixtureSuite {
    pub(super) fn run_execute_failure(
        &self,
        scenario: PublishedAdversarialScenario,
        command: ExternalAdapterCommand,
        expected_fixture: &str,
    ) -> Result<ConformanceCaseRecord, HarnessFailure> {
        let request = self.load_fixture(EXECUTE_REQUEST, ExecutePlanRequest::from_json)?;
        let expected = self.load_fixture(expected_fixture, ProtocolFailure::from_json)?;
        let mut session = match self.launch_case(command)? {
            CaseLaunch::Ready(session) => session,
            CaseLaunch::ProtocolFailure => {
                return Ok(unexpected_bootstrap_failure_case(scenario));
            }
        };
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

    pub(super) fn run_execute_fixture(
        &self,
        scenario: PublishedAdversarialScenario,
        command: ExternalAdapterCommand,
        request_fixture: &str,
        response_fixture: &str,
    ) -> Result<ConformanceCaseRecord, HarnessFailure> {
        let request = self.load_fixture(request_fixture, ExecutePlanRequest::from_json)?;
        let mut expected = self.load_fixture(response_fixture, ExecutePlanResponse::from_json)?;
        expected.validate_against(&request).map_err(|error| {
            super::common::fixture_failure(self.fixture_dir.join(response_fixture), error)
        })?;

        let mut session = match self.launch_case(command)? {
            CaseLaunch::Ready(session) => session,
            CaseLaunch::ProtocolFailure => {
                return Ok(unexpected_bootstrap_failure_case(scenario));
            }
        };
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

    pub(super) fn run_execute_boundary_probe(
        &self,
        scenario: PublishedAdversarialScenario,
        command: ExternalAdapterCommand,
    ) -> Result<ConformanceCaseRecord, HarnessFailure> {
        let request = self.load_fixture(EXECUTE_REQUEST, ExecutePlanRequest::from_json)?;
        let mut session = match self.launch_case(command)? {
            CaseLaunch::Ready(session) => session,
            CaseLaunch::ProtocolFailure => {
                return Ok(unexpected_bootstrap_failure_case(scenario));
            }
        };
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
}
