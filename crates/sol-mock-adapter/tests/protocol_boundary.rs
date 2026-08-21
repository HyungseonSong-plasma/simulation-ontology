use std::fs;
use std::path::PathBuf;

fn crate_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn protocol_facing_source_is_separate_from_legacy_helper_surface() {
    let root = crate_root();
    let protocol_paths = [
        root.join("src/protocol.rs"),
        root.join("src/protocol_state.rs"),
    ];
    for path in &protocol_paths {
        assert!(
            path.exists(),
            "M0.4 protocol-facing source boundary must exist"
        );
    }

    let source = protocol_paths
        .iter()
        .map(|path| fs::read_to_string(path).unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    for forbidden in [
        "EvaluationStatus",
        "sol_core_evaluation",
        "PASS",
        "FAIL",
        "BLOCKED",
        "INDETERMINATE",
        "validation_token",
        "acceptance_id",
        "lease_id",
        "idempotency_key",
        "retryable",
        "jsonrpc",
        "request_id",
        "subprocess",
        "reqwest",
        "tokio",
    ] {
        assert!(
            !source.contains(forbidden),
            "protocol-facing MockAdapter source leaked forbidden marker: {forbidden}"
        );
    }
}

#[test]
fn mock_adapter_crate_remains_solver_and_transport_dependency_free() {
    let cargo = fs::read_to_string(crate_root().join("Cargo.toml")).unwrap();

    for forbidden in [
        "moose", "comsol", "ansys", "jsonrpc", "reqwest", "tokio", "tonic", "zmq",
    ] {
        assert!(
            !cargo.to_ascii_lowercase().contains(forbidden),
            "sol-mock-adapter Cargo.toml leaked backend/transport dependency marker: {forbidden}"
        );
    }
}

#[test]
fn phase_zero_boundary_document_classifies_legacy_api_as_non_normative() {
    let doc =
        crate_root().join("../../docs/implementation/m0.4-mock-adapter-conformance-boundary.md");
    let text = fs::read_to_string(doc).unwrap();

    for required in [
        "`Adapter` trait",
        "legacy/internal Rust helper",
        "Protocol-facing authority",
        "Lifecycle boundary",
        "Transport boundary",
        "Additive compatibility strategy",
    ] {
        assert!(
            text.contains(required),
            "missing Phase 0 boundary evidence: {required}"
        );
    }
}
