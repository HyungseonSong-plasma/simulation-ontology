#![forbid(unsafe_code)]

use std::collections::BTreeSet;

use sol_core_effect::{RealizationEffect, RealizationQuality, SemanticComparison};
use sol_core_evaluation::{classify, EvaluationStatus};
use sol_core_mapping::MappingSubjectRef;
use sol_core_plan::MappingPlan;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AdapterCapability(String);

impl AdapterCapability {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlanAcceptance {
    Accepted,
    Rejected { reason: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FailureInjection {
    None,
    RejectPlan,
    Degraded,
    Unsupported,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulatedRealization {
    pub effect: RealizationEffect,
    pub status: EvaluationStatus,
}

pub trait Adapter {
    fn capabilities(&self) -> &BTreeSet<AdapterCapability>;

    fn assess_plan(
        &self,
        plan: &MappingPlan,
        required: &BTreeSet<AdapterCapability>,
    ) -> PlanAcceptance;

    fn realize(&self, subject: MappingSubjectRef) -> SimulatedRealization;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MockAdapter {
    capabilities: BTreeSet<AdapterCapability>,
    failure: FailureInjection,
}

impl MockAdapter {
    pub fn thermal() -> Self {
        Self {
            capabilities: [
                AdapterCapability::new("thermal.domain"),
                AdapterCapability::new("thermal.material"),
                AdapterCapability::new("thermal.solve"),
            ]
            .into_iter()
            .collect(),
            failure: FailureInjection::None,
        }
    }

    pub fn with_failure(mut self, failure: FailureInjection) -> Self {
        self.failure = failure;
        self
    }
}

impl Adapter for MockAdapter {
    fn capabilities(&self) -> &BTreeSet<AdapterCapability> {
        &self.capabilities
    }

    fn assess_plan(
        &self,
        plan: &MappingPlan,
        required: &BTreeSet<AdapterCapability>,
    ) -> PlanAcceptance {
        if self.failure == FailureInjection::RejectPlan {
            return PlanAcceptance::Rejected {
                reason: "mock failure injection requested plan rejection".to_owned(),
            };
        }

        if let Err(error) = plan.topological_order() {
            return PlanAcceptance::Rejected {
                reason: format!("invalid mapping plan: {error:?}"),
            };
        }

        let missing: Vec<_> = required.difference(&self.capabilities).cloned().collect();
        if !missing.is_empty() {
            return PlanAcceptance::Rejected {
                reason: format!("missing adapter capabilities: {missing:?}"),
            };
        }

        PlanAcceptance::Accepted
    }

    fn realize(&self, subject: MappingSubjectRef) -> SimulatedRealization {
        let (quality, comparison, detail) = match self.failure {
            FailureInjection::None | FailureInjection::RejectPlan => (
                RealizationQuality::Exact,
                SemanticComparison::Exact,
                "mock realization preserves semantic intent",
            ),
            FailureInjection::Degraded => (
                RealizationQuality::Degraded,
                SemanticComparison::Degraded,
                "mock realization injected a degraded outcome",
            ),
            FailureInjection::Unsupported => (
                RealizationQuality::Unsupported,
                SemanticComparison::Unsupported,
                "mock adapter reports unsupported realization",
            ),
            FailureInjection::Unknown => (
                RealizationQuality::Unknown,
                SemanticComparison::Unknown,
                "mock adapter reports insufficient evidence",
            ),
        };

        SimulatedRealization {
            effect: RealizationEffect::new(subject, quality).with_detail(detail),
            status: classify(comparison),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use sol_core_evaluation::EvaluationStatus;
    use sol_core_mapping::MappingSubjectRef;
    use sol_core_plan::{MappingPlan, PlanAction};

    use super::{Adapter, AdapterCapability, FailureInjection, MockAdapter, PlanAcceptance};

    fn thermal_plan() -> MappingPlan {
        MappingPlan::from_actions([
            PlanAction::new("thermal.domain"),
            PlanAction::new("thermal.material").depends_on("thermal.domain"),
            PlanAction::new("thermal.solve").depends_on("thermal.material"),
        ])
        .unwrap()
    }

    fn thermal_requirements() -> BTreeSet<AdapterCapability> {
        [
            AdapterCapability::new("thermal.domain"),
            AdapterCapability::new("thermal.material"),
            AdapterCapability::new("thermal.solve"),
        ]
        .into_iter()
        .collect()
    }

    #[test]
    fn mock_adapter_declares_thermal_capabilities() {
        let adapter = MockAdapter::thermal();
        assert!(adapter
            .capabilities()
            .contains(&AdapterCapability::new("thermal.solve")));
    }

    #[test]
    fn mock_adapter_accepts_supported_thermal_plan() {
        let adapter = MockAdapter::thermal();
        assert_eq!(
            adapter.assess_plan(&thermal_plan(), &thermal_requirements()),
            PlanAcceptance::Accepted
        );
    }

    #[test]
    fn mock_adapter_rejects_missing_capability() {
        let adapter = MockAdapter::thermal();
        let required = [AdapterCapability::new("thermal.radiation")]
            .into_iter()
            .collect();
        assert!(matches!(
            adapter.assess_plan(&thermal_plan(), &required),
            PlanAcceptance::Rejected { .. }
        ));
    }

    #[test]
    fn mock_adapter_simulates_exact_realization() {
        let adapter = MockAdapter::thermal();
        let result = adapter.realize(MappingSubjectRef::Entity(
            "thermal.energy_conservation".parse().unwrap(),
        ));
        assert_eq!(result.status, EvaluationStatus::Pass);
    }

    #[test]
    fn failure_injection_can_reject_plan() {
        let adapter = MockAdapter::thermal().with_failure(FailureInjection::RejectPlan);
        assert!(matches!(
            adapter.assess_plan(&thermal_plan(), &thermal_requirements()),
            PlanAcceptance::Rejected { .. }
        ));
    }

    #[test]
    fn unsupported_realization_is_blocked() {
        let adapter = MockAdapter::thermal().with_failure(FailureInjection::Unsupported);
        let result = adapter.realize(MappingSubjectRef::Entity(
            "thermal.energy_conservation".parse().unwrap(),
        ));
        assert_eq!(result.status, EvaluationStatus::Blocked);
    }

    #[test]
    fn unknown_realization_is_indeterminate() {
        let adapter = MockAdapter::thermal().with_failure(FailureInjection::Unknown);
        let result = adapter.realize(MappingSubjectRef::Entity(
            "thermal.energy_conservation".parse().unwrap(),
        ));
        assert_eq!(result.status, EvaluationStatus::Indeterminate);
    }
}
