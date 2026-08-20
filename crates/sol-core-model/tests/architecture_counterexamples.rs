use sol_core_model::violates_backend_boundary;

fn dependency_lines(manifest: &str) -> impl Iterator<Item = &str> {
    manifest
        .split_once("[dependencies]")
        .map(|(_, dependencies)| dependencies)
        .unwrap_or_default()
        .lines()
        .take_while(|line| !line.trim_start().starts_with('['))
}

#[test]
fn semantic_core_manifest_contains_no_backend_native_dependency() {
    let manifest = include_str!("../Cargo.toml");

    let violations: Vec<_> = dependency_lines(manifest)
        .filter(|line| violates_backend_boundary(line))
        .collect();

    assert!(
        violations.is_empty(),
        "semantic Core must not depend on backend-native crates: {violations:?}"
    );
}

#[test]
fn negative_counterexample_is_rejected() {
    let fixture = include_str!("../../../fixtures/counterexamples/backend-native-dependency.toml");

    assert!(dependency_lines(fixture).any(violates_backend_boundary));
}

#[test]
fn golden_core_dependency_fixture_is_accepted() {
    let fixture = include_str!("../../../fixtures/golden/core-dependencies.toml");

    assert!(!dependency_lines(fixture).any(violates_backend_boundary));
}
