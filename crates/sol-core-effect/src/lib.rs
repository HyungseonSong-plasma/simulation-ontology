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

#[cfg(test)]
mod tests {
    use super::{RealizationEffect, RealizationQuality};
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
}
