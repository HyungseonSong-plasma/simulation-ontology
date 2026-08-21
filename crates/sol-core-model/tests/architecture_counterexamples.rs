use std::{fs, path::Path};

use sol_core_model::violates_backend_boundary;

const PUBLIC_CONTRACT_INTERNAL_MARKERS: [&str; 4] = [
    "sol-core-",
    "sol-target-resolver",
    "sol-mock-adapter",
    "sol-cli",
];

fn dependency_lines(manifest: &str) -> impl Iterator<Item = &str> {
    manifest
        .split_once("[dependencies]")
        .map(|(_, dependencies)| dependencies)
        .unwrap_or_default()
        .lines()
        .take_while(|line| !line.trim_start().starts_with('['))
}

fn violates_public_contract_boundary(manifest_line: &str) -> bool {
    let normalized = manifest_line.trim().to_ascii_lowercase();

    if normalized.is_empty() || normalized.starts_with('#') {
        return false;
    }

    PUBLIC_CONTRACT_INTERNAL_MARKERS
        .iter()
        .any(|marker| normalized.contains(marker))
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

#[test]
fn public_contract_internal_dependency_counterexample_is_rejected() {
    let fixture = include_str!(
        "../../../fixtures/counterexamples/public-contract-internal-dependency.toml"
    );

    assert!(dependency_lines(fixture).any(violates_public_contract_boundary));
}

#[test]
fn public_contract_dependency_fixture_is_accepted() {
    let fixture = include_str!("../../../fixtures/golden/public-contract-dependencies.toml");

    assert!(!dependency_lines(fixture).any(violates_public_contract_boundary));
}

#[test]
fn public_contract_manifest_avoids_internal_crates_when_present() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let manifest_path = workspace_root.join("crates/sol-public-contract/Cargo.toml");

    if !manifest_path.exists() {
        return;
    }

    let manifest = fs::read_to_string(&manifest_path).expect("read sol-public-contract manifest");
    let violations: Vec<_> = dependency_lines(&manifest)
        .filter(|line| violates_public_contract_boundary(line))
        .collect();

    assert!(
        violations.is_empty(),
        "public contract must not depend directly on internal Rust implementation crates: {violations:?}"
    );
}
