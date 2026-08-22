use super::PublishedAdversarialScenario;
use sol_adapter_conformance::{
    ConformanceCaseId, ConformanceCaseRecord, ConformanceCaseResult, ConformanceEvidence,
    ConformanceScope, ConformanceViolation, HarnessFailure, HarnessFailureKind, PublishedContract,
};
use sol_adapter_conformance_harness::{AdapterProtocolObservation, ExternalAdapterSession};
use sol_adapter_protocol::{ProtocolFailure, ProtocolOperation};
use std::fmt::Display;
use std::path::Path;

pub(super) enum CaseLaunch {
    Ready(ExternalAdapterSession),
    ProtocolFailure,
}

pub(super) fn unexpected_bootstrap_failure_case(
    scenario: PublishedAdversarialScenario,
) -> ConformanceCaseRecord {
    nonconformant_case(
        scenario.case_id(),
        scope_for(scenario),
        "scenario expected a ready session but bootstrap returned logical ProtocolFailure",
    )
}

pub(super) fn protocol_failure_case<T>(
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

pub(super) fn scope_for(scenario: PublishedAdversarialScenario) -> ConformanceScope {
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

pub(super) fn fixture_failure(subject: impl AsRef<Path>, error: impl Display) -> HarnessFailure {
    HarnessFailure::new(
        HarnessFailureKind::FixtureLoad,
        format!("{}: {error}", subject.as_ref().display()),
    )
    .expect("fixture failure detail is non-blank")
}

pub(super) fn conformant_case(
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

pub(super) fn nonconformant_case(
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

pub(super) fn harness_case(
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
