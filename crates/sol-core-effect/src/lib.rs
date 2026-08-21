#![forbid(unsafe_code)]

use sol_core_mapping::MappingSubjectRef;

/// Solver-independent realization quality. These values describe how a
/// proposed realization relates to semantic intent; they do not redefine it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RealizationQuality {
    Exact,
    Compatible,
    Degraded,
    Unsupported,
    Unknown,
}

/// Semantic effect produced or proposed by a realization path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RealizationEffect {
    pub subject: MappingSubjectRef,
    pub quality: RealizationQuality,
    pub detail: Option<String>,
}

impl RealizationEffect {
    pub fn new(subject: MappingSubjectRef, quality: RealizationQuality) -> Self {
        Self {
            subject,
            quality,
            detail: None,
        }
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }
}

/// Result of comparing semantic intent with a realization effect. Lifecycle
/// PASS/FAIL/BLOCKED/INDETERMINATE classification is intentionally deferred.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemanticComparison {
    Exact,
    Compatible,
    Degraded,
    Unsupported,
    Unknown,
    SubjectMismatch,
}

/// Compare intended semantic subject with the reported realization effect.
pub fn compare_semantics(
    intended: &MappingSubjectRef,
    effect: &RealizationEffect,
) -> SemanticComparison {
    if intended != &effect.subject {
        return SemanticComparison::SubjectMismatch;
    }

    match effect.quality {
        RealizationQuality::Exact => SemanticComparison::Exact,
        RealizationQuality::Compatible => SemanticComparison::Compatible,
        RealizationQuality::Degraded => SemanticComparison::Degraded,
        RealizationQuality::Unsupported => SemanticComparison::Unsupported,
        RealizationQuality::Unknown => SemanticComparison::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::{compare_semantics, RealizationEffect, RealizationQuality, SemanticComparison};
    use sol_core_mapping::MappingSubjectRef;

    #[test]
    fn thermal_exact_effect_preserves_semantic_subject() {
        let subject = MappingSubjectRef::Entity("thermal.energy_conservation".parse().unwrap());
        let effect = RealizationEffect::new(subject.clone(), RealizationQuality::Exact)
            .with_detail("semantic intent preserved exactly");

        assert_eq!(effect.subject, subject);
        assert_eq!(effect.quality, RealizationQuality::Exact);
        assert_eq!(
            effect.detail.as_deref(),
            Some("semantic intent preserved exactly")
        );
    }

    #[test]
    fn degraded_effect_is_distinct_from_exact_effect() {
        assert_ne!(RealizationQuality::Exact, RealizationQuality::Degraded);
    }

    #[test]
    fn comparator_reports_exact_for_matching_exact_effect() {
        let intended = MappingSubjectRef::Entity("thermal.energy_conservation".parse().unwrap());
        let effect = RealizationEffect::new(intended.clone(), RealizationQuality::Exact);

        assert_eq!(
            compare_semantics(&intended, &effect),
            SemanticComparison::Exact
        );
    }

    #[test]
    fn comparator_reports_degraded_without_redefining_intent() {
        let intended = MappingSubjectRef::Entity("thermal.energy_conservation".parse().unwrap());
        let effect = RealizationEffect::new(intended.clone(), RealizationQuality::Degraded)
            .with_detail("backend realizes a reduced-order approximation");

        assert_eq!(
            compare_semantics(&intended, &effect),
            SemanticComparison::Degraded
        );
        assert_eq!(effect.subject, intended);
    }

    #[test]
    fn comparator_rejects_effect_for_different_semantic_subject() {
        let intended = MappingSubjectRef::Entity("thermal.energy_conservation".parse().unwrap());
        let effect = RealizationEffect::new(
            MappingSubjectRef::Entity("thermal.temperature_field".parse().unwrap()),
            RealizationQuality::Exact,
        );

        assert_eq!(
            compare_semantics(&intended, &effect),
            SemanticComparison::SubjectMismatch
        );
    }
}
