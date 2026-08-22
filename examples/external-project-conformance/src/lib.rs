#![forbid(unsafe_code)]

use serde_json::{json, Value};
use sol_adapter_conformance::{
    ConformanceCaseResult, ConformanceDetermination, ConformanceReport, HarnessFailureKind,
};
use sol_adapter_conformance_fixtures::{
    PublishedAdversarialFixtureSuite, PublishedAdversarialScenario, PublishedPositiveFixtureSuite,
};
use sol_adapter_conformance_harness::ExternalAdapterCommand;
use std::env;
use std::path::PathBuf;

pub const PROVISIONAL_FORMAT: &str = "m0.6-provisional-ci-observation-v1";

pub fn run_external_project_observation() -> Result<Value, String> {
    let fixture_dir = required_path("SOL_CONFORMANCE_FIXTURES")?;
    let positive_adapter = required_path("SOL_REFERENCE_ADAPTER")?;
    let mock_adapter = required_path("SOL_MOCK_ADAPTER")?;
    let transport_probe = required_path("SOL_TRANSPORT_PROBE")?;
    let semantic_adversary = required_path("SOL_SEMANTIC_ADVERSARY")?;

    let positive = PublishedPositiveFixtureSuite::new(&fixture_dir)
        .run(ExternalAdapterCommand::new(&positive_adapter));

    let adversarial = PublishedAdversarialFixtureSuite::new(&fixture_dir);
    let valid_negative = adversarial.run_matrix(valid_negative_cases(&mock_adapter));
    let semantic_violations = adversarial.run_matrix([
        (
            PublishedAdversarialScenario::DependencyScheduleViolation,
            command_with_arg(&semantic_adversary, "dependency-schedule"),
        ),
        (
            PublishedAdversarialScenario::AggregateEffectMismatch,
            command_with_arg(&semantic_adversary, "aggregate-effect"),
        ),
        (
            PublishedAdversarialScenario::ProvenanceIdentityLeakage,
            command_with_arg(&semantic_adversary, "provenance-identity"),
        ),
    ]);
    let response_loss = ConformanceReport::new(vec![adversarial.run_case(
        PublishedAdversarialScenario::ExecuteResponseLoss,
        command_with_arg(&transport_probe, "execute-response-loss"),
    )])
    .map_err(|error| error.to_string())?;

    Ok(json!({
        "format": PROVISIONAL_FORMAT,
        "stability": "provisional_not_a_public_cli_or_report_contract",
        "backend_validation": "not_assessed_by_conformance",
        "observations": [
            report_value("positive", "conformant", &positive),
            report_value("adversarial_valid_negative", "conformant", &valid_negative),
            report_value("semantic_violation_detection", "non_conformant", &semantic_violations),
            report_value("execute_response_loss", "not_established", &response_loss),
        ]
    }))
}

fn valid_negative_cases(
    adapter: &PathBuf,
) -> Vec<(PublishedAdversarialScenario, ExternalAdapterCommand)> {
    use PublishedAdversarialScenario as Scenario;
    vec![
        (
            Scenario::CompatibilityMissing,
            profile(adapter, "failure-compatibility"),
        ),
        (Scenario::TargetMismatch, profile(adapter, "exact")),
        (Scenario::MissingCapability, profile(adapter, "exact")),
        (Scenario::UnsupportedAction, profile(adapter, "exact")),
        (
            Scenario::PreflightPrerequisiteRejected,
            profile(adapter, "preflight-prerequisite-rejected"),
        ),
        (
            Scenario::PreflightTransientUnavailable,
            profile(adapter, "preflight-transient-unavailable"),
        ),
        (
            Scenario::ExecutionPartial,
            profile(adapter, "execution-partial"),
        ),
        (
            Scenario::ExecutionUnsupported,
            profile(adapter, "execution-unsupported"),
        ),
        (
            Scenario::ExecutionUnavailable,
            profile(adapter, "execution-unavailable"),
        ),
        (
            Scenario::ExecutionAuthoritativeRejected,
            profile(adapter, "execution-authoritative-rejected"),
        ),
        (
            Scenario::ExecutionAlternateOrder,
            profile(adapter, "execution-alternate-order"),
        ),
        (
            Scenario::ExecutionParallelIndependent,
            profile(adapter, "execution-parallel-independent"),
        ),
        (
            Scenario::PriorAlreadyRealized,
            profile(adapter, "prior-already-realized"),
        ),
        (
            Scenario::PriorPartialExecution,
            profile(adapter, "prior-partial-execution"),
        ),
        (
            Scenario::PriorUnresolvedPrerequisite,
            profile(adapter, "prior-unresolved-prerequisite"),
        ),
        (
            Scenario::ValidateInvalidRequestFailure,
            profile(adapter, "failure-invalid-request"),
        ),
        (
            Scenario::ValidateOperationalFailure,
            profile(adapter, "failure-validate-operational"),
        ),
        (
            Scenario::ExecuteFailureBeforeSideEffect,
            profile(adapter, "failure-execute-before-side-effect"),
        ),
        (
            Scenario::ExecuteFailureAmbiguous,
            profile(adapter, "failure-execute-ambiguous"),
        ),
    ]
}

fn profile(program: &PathBuf, value: &str) -> ExternalAdapterCommand {
    command_with_arg(program, &format!("--profile={value}"))
}

fn command_with_arg(program: &PathBuf, argument: &str) -> ExternalAdapterCommand {
    ExternalAdapterCommand::new(program).arg(argument)
}

fn required_path(name: &str) -> Result<PathBuf, String> {
    env::var_os(name)
        .map(PathBuf::from)
        .ok_or_else(|| format!("required external-project environment variable is missing: {name}"))
}

fn report_value(label: &str, expected: &str, report: &ConformanceReport) -> Value {
    json!({
        "label": label,
        "expected_determination": expected,
        "determination": determination_name(report.determination()),
        "cases": report.cases().iter().map(case_value).collect::<Vec<_>>(),
    })
}

fn case_value(case: &sol_adapter_conformance::ConformanceCaseRecord) -> Value {
    match case.result() {
        ConformanceCaseResult::Conformant(evidence) => json!({
            "id": case.id().as_str(),
            "result": "conformant",
            "detail": evidence.as_str(),
        }),
        ConformanceCaseResult::NonConformant(violation) => json!({
            "id": case.id().as_str(),
            "result": "non_conformant",
            "detail": violation.as_str(),
        }),
        ConformanceCaseResult::HarnessFailure(failure) => json!({
            "id": case.id().as_str(),
            "result": "harness_failure",
            "failure_kind": harness_failure_name(failure.kind()),
            "detail": failure.detail(),
        }),
    }
}

const fn determination_name(value: ConformanceDetermination) -> &'static str {
    match value {
        ConformanceDetermination::Conformant => "conformant",
        ConformanceDetermination::NonConformant => "non_conformant",
        ConformanceDetermination::NotEstablished => "not_established",
    }
}

const fn harness_failure_name(value: HarnessFailureKind) -> &'static str {
    match value {
        HarnessFailureKind::FixtureLoad => "fixture_load",
        HarnessFailureKind::AdapterInvocation => "adapter_invocation",
        HarnessFailureKind::TransportExchange => "transport_exchange",
        HarnessFailureKind::ResultDecoding => "result_decoding",
        HarnessFailureKind::RunnerInvariant => "runner_invariant",
    }
}
