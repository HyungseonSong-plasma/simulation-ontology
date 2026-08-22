mod common;
mod execution;
mod preflight;
mod scenario;

pub use scenario::PublishedAdversarialScenario;

use common::{harness_case, scope_for, CaseLaunch};
use sol_adapter_conformance::{ConformanceCaseRecord, ConformanceReport, HarnessFailure};
use sol_adapter_conformance_harness::{
    ExternalAdapterCommand, ExternalAdapterHarness, ExternalAdapterLaunch,
};
use std::fmt::Display;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct PublishedAdversarialFixtureSuite {
    pub(super) fixture_dir: PathBuf,
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
            Err(failure) => harness_case(scenario.case_id(), scope_for(scenario), failure),
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
            PublishedAdversarialScenario::CompatibilityMissing => self.run_bootstrap_failure(
                scenario,
                command,
                "protocol-failure-compatibility-missing.json",
            ),
            PublishedAdversarialScenario::TargetMismatch
            | PublishedAdversarialScenario::MissingCapability
            | PublishedAdversarialScenario::UnsupportedAction
            | PublishedAdversarialScenario::PreflightPrerequisiteRejected
            | PublishedAdversarialScenario::PreflightTransientUnavailable => {
                self.run_preflight(scenario, command)
            }
            PublishedAdversarialScenario::ValidateInvalidRequestFailure => self
                .run_validate_failure(scenario, command, "protocol-failure-invalid-request.json"),
            PublishedAdversarialScenario::ValidateOperationalFailure => self.run_validate_failure(
                scenario,
                command,
                "protocol-failure-validate-operational.json",
            ),
            PublishedAdversarialScenario::ExecuteFailureBeforeSideEffect => self
                .run_execute_failure(
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
                "execute-plan-thermal-request.json",
                "execute-plan-partial-response.json",
            ),
            PublishedAdversarialScenario::ExecutionUnsupported => self.run_execute_fixture(
                scenario,
                command,
                "execute-plan-thermal-request.json",
                "execute-plan-unsupported-response.json",
            ),
            PublishedAdversarialScenario::ExecutionUnavailable => self.run_execute_fixture(
                scenario,
                command,
                "execute-plan-thermal-request.json",
                "execute-plan-unavailable-response.json",
            ),
            PublishedAdversarialScenario::ExecutionAuthoritativeRejected => self
                .run_execute_fixture(
                    scenario,
                    command,
                    "execute-plan-changed-request.json",
                    "execute-plan-authoritative-rejection-response.json",
                ),
            PublishedAdversarialScenario::ExecutionAlternateOrder => self.run_execute_fixture(
                scenario,
                command,
                "execute-plan-independent-request.json",
                "execute-plan-alternate-order-response.json",
            ),
            PublishedAdversarialScenario::ExecutionParallelIndependent => self.run_execute_fixture(
                scenario,
                command,
                "execute-plan-independent-request.json",
                "execute-plan-parallel-response.json",
            ),
            PublishedAdversarialScenario::PriorAlreadyRealized => self.run_execute_fixture(
                scenario,
                command,
                "execute-plan-thermal-request.json",
                "execute-plan-already-realized-response.json",
            ),
            PublishedAdversarialScenario::PriorPartialExecution => self.run_execute_fixture(
                scenario,
                command,
                "execute-plan-thermal-request.json",
                "execute-plan-partial-prior-rejection-response.json",
            ),
            PublishedAdversarialScenario::PriorUnresolvedPrerequisite => self.run_execute_fixture(
                scenario,
                command,
                "execute-plan-thermal-request.json",
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

    pub(super) fn launch_case(
        &self,
        command: ExternalAdapterCommand,
    ) -> Result<CaseLaunch, HarnessFailure> {
        match self.harness.launch(command) {
            Ok(ExternalAdapterLaunch::Ready(session)) => Ok(CaseLaunch::Ready(session)),
            Ok(ExternalAdapterLaunch::ProtocolFailure(_)) => Ok(CaseLaunch::ProtocolFailure),
            Err(error) => Err(error.into_report_failure()),
        }
    }

    pub(super) fn load_fixture<T, E>(
        &self,
        name: &str,
        parse: impl FnOnce(&str) -> Result<T, E>,
    ) -> Result<T, HarnessFailure>
    where
        E: Display,
    {
        let path = self.fixture_dir.join(name);
        let input =
            fs::read_to_string(&path).map_err(|error| common::fixture_failure(&path, error))?;
        parse(&input).map_err(|error| common::fixture_failure(&path, error))
    }
}
