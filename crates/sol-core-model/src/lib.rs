#![forbid(unsafe_code)]

/// Version of the semantic-core bootstrap contract.
pub const CORE_BOOTSTRAP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Names that are reserved for backend-specific realization layers and must not
/// become dependencies of the solver-independent semantic Core.
pub const BACKEND_BOUNDARY_MARKERS: [&str; 4] = ["moose", "comsol", "ansys", "adapter-"];

/// Returns true when a manifest line appears to introduce a backend-specific
/// dependency into the semantic Core.
pub fn violates_backend_boundary(manifest_line: &str) -> bool {
    let normalized = manifest_line.trim().to_ascii_lowercase();

    if normalized.is_empty() || normalized.starts_with('#') {
        return false;
    }

    BACKEND_BOUNDARY_MARKERS
        .iter()
        .any(|marker| normalized.contains(marker))
}

#[cfg(test)]
mod tests {
    use super::violates_backend_boundary;

    #[test]
    fn backend_dependency_marker_is_detected() {
        assert!(violates_backend_boundary(
            "moose-adapter = { path = \"../moose\" }"
        ));
    }

    #[test]
    fn ordinary_core_dependency_is_allowed() {
        assert!(!violates_backend_boundary("serde = \"1\""));
    }
}
