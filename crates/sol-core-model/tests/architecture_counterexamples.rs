use serde_json::Value;
use std::{fs, path::Path};

use sol_core_model::violates_backend_boundary;

const PUBLIC_CONTRACT_INTERNAL_MARKERS: [&str; 4] = [
    "sol-core-",
    "sol-target-resolver",
    "sol-mock-adapter",
    "sol-cli",
];

const PROTOCOL_OPERATION_CANDIDATES: [&str; 3] =
    ["describe_adapter", "validate_plan", "execute_plan"];

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

fn adapter_protocol_boundary_violations(fixture: &str) -> Vec<&'static str> {
    let value: Value = serde_json::from_str(fixture).expect("valid protocol-boundary fixture JSON");
    let mut violations = Vec::new();

    let operations: Vec<_> = value["public_operations"]
        .as_array()
        .expect("public_operations array")
        .iter()
        .map(|operation| operation.as_str().expect("operation string"))
        .collect();
    if operations != PROTOCOL_OPERATION_CANDIDATES {
        violations.push("operation_surface");
    }

    for (field, violation) in [
        (
            "duplicates_public_contract_payload",
            "duplicate_public_contract_payload",
        ),
        (
            "backend_native_semantic_payload",
            "backend_native_semantic_payload",
        ),
        (
            "opaque_provenance_affects_semantic_identity",
            "opaque_provenance_semantic_identity",
        ),
        (
            "requires_canonical_total_execution_order",
            "canonical_order_overconstraint",
        ),
        (
            "one_method_per_responsibility",
            "one_method_per_responsibility",
        ),
    ] {
        if value[field].as_bool().expect("boolean boundary field") {
            violations.push(violation);
        }
    }

    violations
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
    let fixture =
        include_str!("../../../fixtures/counterexamples/public-contract-internal-dependency.toml");

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

#[test]
fn adapter_protocol_boundary_golden_fixture_is_accepted() {
    let fixture = include_str!("../../../fixtures/golden/adapter-protocol-boundary.json");

    assert!(adapter_protocol_boundary_violations(fixture).is_empty());
}

#[test]
fn duplicate_public_contract_payload_counterexample_is_rejected() {
    let fixture = include_str!(
        "../../../fixtures/counterexamples/adapter-protocol-duplicate-public-payload.json"
    );

    assert!(adapter_protocol_boundary_violations(fixture)
        .contains(&"duplicate_public_contract_payload"));
}

#[test]
fn backend_native_semantic_payload_counterexample_is_rejected() {
    let fixture = include_str!(
        "../../../fixtures/counterexamples/adapter-protocol-backend-native-semantic-leakage.json"
    );

    assert!(adapter_protocol_boundary_violations(fixture).contains(&"backend_native_semantic_payload"));
}

#[test]
fn opaque_provenance_semantic_identity_counterexample_is_rejected() {
    let fixture = include_str!(
        "../../../fixtures/counterexamples/adapter-protocol-opaque-provenance-semantic-identity.json"
    );

    assert!(adapter_protocol_boundary_violations(fixture)
        .contains(&"opaque_provenance_semantic_identity"));
}

#[test]
fn canonical_order_overconstraint_counterexample_is_rejected() {
    let fixture = include_str!(
        "../../../fixtures/counterexamples/adapter-protocol-canonical-order-overconstraint.json"
    );

    assert!(
        adapter_protocol_boundary_violations(fixture).contains(&"canonical_order_overconstraint")
    );
}

#[test]
fn one_method_per_responsibility_counterexample_is_rejected() {
    let fixture = include_str!(
        "../../../fixtures/counterexamples/adapter-protocol-one-method-per-responsibility.json"
    );

    let violations = adapter_protocol_boundary_violations(fixture);
    assert!(violations.contains(&"operation_surface"));
    assert!(violations.contains(&"one_method_per_responsibility"));
}
