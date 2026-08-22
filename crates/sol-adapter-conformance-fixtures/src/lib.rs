#![forbid(unsafe_code)]

//! Execution of published SOL Adapter Protocol/Public Contract fixture material through
//! the external-process conformance harness.
//!
//! This crate does not define a second protocol representation or a stable CLI/report
//! serialization contract. Published JSON is decoded by the existing canonical DTO
//! parsers and observations are recorded with the Phase 0 conformance result model.

use sol_adapter_conformance::{
    ConformanceCaseId, ConformanceCaseRecord, ConformanceCaseResult, ConformanceEvidence,
    ConformanceReport, ConformanceScope, ConformanceViolation, HarnessFailure, HarnessFailureKind,
    PublishedContract,
};
use sol_adapter_conformance_harness::{
    AdapterProtocolObservation, ExternalAdapterCommand, ExternalAdapterHarness, ExternalAdapterLaunch,
};
use sol_adapter_protocol::{
    AdapterDescription, CompatibilityOutcome, ExecutePlanRequest, ExecutePlanResponse,
    ProtocolOperation, ValidatePlanRequest, ValidatePlanResponse, ADAPTER_PROTOCOL_VERSION,
};
use std::fmt::Display;
use std::fs;
use std::path::{Path, PathBuf};

const DESCRIPTION_FIXTURE: &str = "dual-compatible-description.json";
const VALIDATE_REQUEST_FIXTURE: &str = "validate-plan-accepted-request.json";
const VALIDATE_RESPONSE_FIXTURE: &str = "validate-plan-accepted-response.json";
const EXECUTE_REQUEST_FIXTURE: &str = "execute-plan-thermal-request.json";
const EXECUTE_RESPONSE_FIXTURE: &str = "execute-plan-exact-response.json";

#[derive(Debug, Clone)]
pub struct PublishedPositiveFixtureSuite {
    fixture_dir: PathBuf,
    harness: ExternalAdapterHarness,
}

impl PublishedPositiveFixtureSuite {
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

    pub fn fixture_dir(&self) -> &Path {
        &self.fixture_dir
    }

    pub fn run(&self, command: ExternalAdapterCommand) -> ConformanceReport {
        let fixtures = match PositiveFixtures::load(&self.fixture_dir) {
            Ok(fixtures) => fixtures,
            Err(failure) => {
                return report(vec![harness_case(
                    "published-fixtures.load",
                    ConformanceScope::Fixture(PublishedContract::AdapterProtocol),
                    failure,
                )]);
            }
        };

        let mut cases = vec![conformant_case(
            "adapter-protocol.fixture-decode",
            ConformanceScope::Fixture(PublishedContract::AdapterProtocol),
            "published Adapter Protocol 0.1 positive fixtures decoded through canonical DTO parsers",
        )];

        if let Err(failure) = fixtures.validate_public_contract_payloads() {
            cases.push(harness_case(
                "public-contract.reused-payloads",
                ConformanceScope::Schema(PublishedContract::PublicContract),
                failure,
            ));
            return report(cases);
        }
        cases.push(conformant_case(
            "public-contract.reused-payloads",
            ConformanceScope::Schema(PublishedContract::PublicContract),
            "published validate/execute requests preserve canonical Public Contract target and MappingPlan payloads",
        ));

        let mut session = match self.harness.launch(command) {
            Ok(ExternalAdapterLaunch::Ready(session)) => session,
            Ok(ExternalAdapterLaunch::ProtocolFailure(_)) => {
                cases.push(nonconformant_case(
                    "describe-adapter.positive",
                    ConformanceScope::OperationSemantics(ProtocolOperation::DescribeAdapter),
                    "positive bootstrap expected AdapterDescription but received ProtocolFailure",
                ));
                return report(cases);
            }
            Err(error) => {
                cases.push(harness_case(
                    "describe-adapter.positive",
                    ConformanceScope::OperationSemantics(ProtocolOperation::DescribeAdapter),
                    error.into_report_failure(),
                ));
                return report(cases);
            }
        };

        cases.push(description_case(session.description(), &fixtures.description));
        cases.push(compatibility_case(
            "compatibility.adapter-protocol",
            PublishedContract::AdapterProtocol,
            session.compatibility().adapter_protocol.outcome,
            session
                .compatibility()
                .adapter_protocol
                .selected_version
                .as_deref(),
            ADAPTER_PROTOCOL_VERSION,
        ));
        cases.push(compatibility_case(
            "compatibility.public-contract",
            PublishedContract::PublicContract,
            session.compatibility().public_contract.outcome,
            session
                .compatibility()
                .public_contract
                .selected_version
                .as_deref(),
            fixtures.validate_request.target.public_contract_version.as_str(),
        ));

        cases.push(match session.validate_plan(&fixtures.validate_request) {
            Ok(AdapterProtocolObservation::Success(observed)) if observed == fixtures.validate_response => {
                conformant_case(
                    "validate-plan.positive",
                    ConformanceScope::OperationSemantics(ProtocolOperation::ValidatePlan),
                    "validate_plan observation matched the published accepted-response fixture",
                )
            }
            Ok(AdapterProtocolObservation::Success(_)) => nonconformant_case(
                "validate-plan.positive",
                ConformanceScope::OperationSemantics(ProtocolOperation::ValidatePlan),
                "validate_plan success differed from the published accepted-response fixture",
            ),
            Ok(AdapterProtocolObservation::ProtocolFailure(_)) => nonconformant_case(
                "validate-plan.positive",
                ConformanceScope::OperationSemantics(ProtocolOperation::ValidatePlan),
                "positive validate_plan fixture returned ProtocolFailure",
            ),
            Err(error) => harness_case(
                "validate-plan.positive",
                ConformanceScope::OperationSemantics(ProtocolOperation::ValidatePlan),
                error.into_report_failure(),
            ),
        });

        cases.push(match session.execute_plan(&fixtures.execute_request) {
            Ok(AdapterProtocolObservation::Success(mut observed)) => {
                match observed.validate_against(&fixtures.execute_request) {
                    Ok(()) if observed == fixtures.execute_response => conformant_case(
                        "execute-plan.positive",
                        ConformanceScope::OperationSemantics(ProtocolOperation::ExecutePlan),
                        "execute_plan observation matched the published exact-response fixture and request-relative invariants",
                    ),
                    Ok(()) => nonconformant_case(
                        "execute-plan.positive",
                        ConformanceScope::OperationSemantics(ProtocolOperation::ExecutePlan),
                        "execute_plan success differed from the published exact-response fixture",
                    ),
                    Err(_) => nonconformant_case(
                        "execute-plan.positive",
                        ConformanceScope::OperationSemantics(ProtocolOperation::ExecutePlan),
                        "execute_plan response violated published request-relative invariants",
                    ),
                }
            }
            Ok(AdapterProtocolObservation::ProtocolFailure(_)) => nonconformant_case(
                "execute-plan.positive",
                ConformanceScope::OperationSemantics(ProtocolOperation::ExecutePlan),
                "positive execute_plan fixture returned ProtocolFailure",
            ),
            Err(error) => harness_case(
                "execute-plan.positive",
                ConformanceScope::OperationSemantics(ProtocolOperation::ExecutePlan),
                error.into_report_failure(),
            ),
        });

        if let Err(error) = session.shutdown() {
            cases.push(harness_case(
                "external-adapter.shutdown",
                ConformanceScope::Fixture(PublishedContract::AdapterProtocol),
                error.into_report_failure(),
            ));
        }

        report(cases)
    }
}

#[derive(Debug)]
struct PositiveFixtures {
    description: AdapterDescription,
    validate_request: ValidatePlanRequest,
    validate_response: ValidatePlanResponse,
    execute_request: ExecutePlanRequest,
    execute_response: ExecutePlanResponse,
}

impl PositiveFixtures {
    fn load(root: &Path) -> Result<Self, HarnessFailure> {
        let description = load_fixture(root, DESCRIPTION_FIXTURE, AdapterDescription::from_json)?;
        let validate_request =
            load_fixture(root, VALIDATE_REQUEST_FIXTURE, ValidatePlanRequest::from_json)?;
        let validate_response =
            load_fixture(root, VALIDATE_RESPONSE_FIXTURE, ValidatePlanResponse::from_json)?;
        let execute_request =
            load_fixture(root, EXECUTE_REQUEST_FIXTURE, ExecutePlanRequest::from_json)?;
        let mut execute_response =
            load_fixture(root, EXECUTE_RESPONSE_FIXTURE, ExecutePlanResponse::from_json)?;
        execute_response
            .validate_against(&execute_request)
            .map_err(|error| fixture_failure(root.join(EXECUTE_RESPONSE_FIXTURE), error))?;

        Ok(Self {
            description,
            validate_request,
            validate_response,
            execute_request,
            execute_response,
        })
    }

    fn validate_public_contract_payloads(&self) -> Result<(), HarnessFailure> {
        for (label, result) in [
            ("validate request target", self.validate_request.target.to_canonical_json()),
            ("validate request plan", self.validate_request.plan.to_canonical_json()),
            ("execute request target", self.execute_request.target.to_canonical_json()),
            ("execute request plan", self.execute_request.plan.to_canonical_json()),
        ] {
            result.map_err(|error| fixture_failure(label, error))?;
        }
        Ok(())
    }
}

fn description_case(
    observed: &AdapterDescription,
    published: &AdapterDescription,
) -> ConformanceCaseRecord {
    let compatible_axes = observed.bootstrap.supported_adapter_protocol_versions
        == published.bootstrap.supported_adapter_protocol_versions
        && observed.bootstrap.supported_public_contract_versions
            == published.bootstrap.supported_public_contract_versions;
    let same_targets = observed.targets == published.targets;

    if compatible_axes && same_targets {
        conformant_case(
            "describe-adapter.positive",
            ConformanceScope::OperationSemantics(ProtocolOperation::DescribeAdapter),
            "description matched published compatibility and target/capability fixture semantics; adapter implementation version was intentionally ignored",
        )
    } else {
        nonconformant_case(
            "describe-adapter.positive",
            ConformanceScope::OperationSemantics(ProtocolOperation::DescribeAdapter),
            "description differed from published compatibility or target/capability fixture semantics",
        )
    }
}

fn load_fixture<T, E>(
    root: &Path,
    name: &str,
    parse: impl FnOnce(&str) -> Result<T, E>,
) -> Result<T, HarnessFailure>
where
    E: Display,
{
    let path = root.join(name);
    let input = fs::read_to_string(&path).map_err(|error| fixture_failure(&path, error))?;
    parse(&input).map_err(|error| fixture_failure(&path, error))
}

fn fixture_failure(subject: impl AsRef<Path>, error: impl Display) -> HarnessFailure {
    HarnessFailure::new(
        HarnessFailureKind::FixtureLoad,
        format!("{}: {error}", subject.as_ref().display()),
    )
    .expect("fixture failure detail is non-blank")
}

fn compatibility_case(
    id: &'static str,
    contract: PublishedContract,
    outcome: CompatibilityOutcome,
    selected_version: Option<&str>,
    expected_version: &str,
) -> ConformanceCaseRecord {
    if outcome == CompatibilityOutcome::Compatible && selected_version == Some(expected_version) {
        conformant_case(
            id,
            ConformanceScope::Compatibility(contract),
            "external adapter established compatibility with the published baseline",
        )
    } else {
        nonconformant_case(
            id,
            ConformanceScope::Compatibility(contract),
            "external adapter did not establish compatibility with the published baseline",
        )
    }
}

fn conformant_case(
    id: &'static str,
    scope: ConformanceScope,
    evidence: &'static str,
) -> ConformanceCaseRecord {
    ConformanceCaseRecord::new(
        fixed_id(id),
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
        fixed_id(id),
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
    ConformanceCaseRecord::new(fixed_id(id), scope, ConformanceCaseResult::HarnessFailure(failure))
}

fn fixed_id(value: &'static str) -> ConformanceCaseId {
    ConformanceCaseId::new(value).expect("fixed conformance case id is valid")
}

fn report(cases: Vec<ConformanceCaseRecord>) -> ConformanceReport {
    ConformanceReport::new(cases).expect("fixed positive-suite case identifiers are unique")
}
