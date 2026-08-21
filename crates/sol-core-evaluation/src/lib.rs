#![forbid(unsafe_code)]

use sol_core_effect::SemanticComparison;

/// Lifecycle outcome for evaluating a semantic realization.
///
/// `Blocked` means evaluation cannot proceed because the requested realization
/// is unavailable. `Indeterminate` means available evidence is insufficient to
/// decide pass/fail. Neither state is equivalent to `Fail`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvaluationStatus {
    Pass,
    Fail,
    Blocked,
    Indeterminate,
}

/// Classify a semantic comparison into the M0.1 evaluation lifecycle.
///
/// Exact and compatible realizations satisfy semantic intent. A degraded or
/// mismatched realization is a completed comparison that does not satisfy the
/// required semantics. Unsupported realization is blocked because no valid
/// realization path is available. Unknown remains indeterminate.
pub fn classify(comparison: SemanticComparison) -> EvaluationStatus {
    match comparison {
        SemanticComparison::Exact | SemanticComparison::Compatible => EvaluationStatus::Pass,
        SemanticComparison::Degraded | SemanticComparison::SubjectMismatch => {
            EvaluationStatus::Fail
        }
        SemanticComparison::Unsupported => EvaluationStatus::Blocked,
        SemanticComparison::Unknown => EvaluationStatus::Indeterminate,
    }
}

#[cfg(test)]
mod tests {
    use super::{classify, EvaluationStatus};
    use sol_core_effect::SemanticComparison;

    #[test]
    fn exact_thermal_realization_passes() {
        assert_eq!(
            classify(SemanticComparison::Exact),
            EvaluationStatus::Pass
        );
    }

    #[test]
    fn degraded_thermal_realization_fails() {
        assert_eq!(
            classify(SemanticComparison::Degraded),
            EvaluationStatus::Fail
        );
    }

    #[test]
    fn unsupported_realization_is_blocked_not_failed() {
        let status = classify(SemanticComparison::Unsupported);
        assert_eq!(status, EvaluationStatus::Blocked);
        assert_ne!(status, EvaluationStatus::Fail);
    }

    #[test]
    fn unknown_realization_is_indeterminate_not_failed() {
        let status = classify(SemanticComparison::Unknown);
        assert_eq!(status, EvaluationStatus::Indeterminate);
        assert_ne!(status, EvaluationStatus::Fail);
    }

    #[test]
    fn subject_mismatch_is_a_completed_failure() {
        assert_eq!(
            classify(SemanticComparison::SubjectMismatch),
            EvaluationStatus::Fail
        );
    }
}
