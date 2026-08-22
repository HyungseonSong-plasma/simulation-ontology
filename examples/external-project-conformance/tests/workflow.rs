use sol_external_conformance_consumer::{run_external_project_observation, PROVISIONAL_FORMAT};

const MANIFEST: &str = include_str!("../Cargo.toml");

#[test]
fn external_project_observation_is_deterministic_and_machine_readable() {
    let first = run_external_project_observation().expect("external conformance workflow runs");
    let second = run_external_project_observation().expect("external conformance workflow repeats");
    assert_eq!(first, second);

    assert_eq!(first["format"], PROVISIONAL_FORMAT);
    assert_eq!(
        first["stability"],
        "provisional_not_a_public_cli_or_report_contract"
    );
    assert_eq!(
        first["backend_validation"],
        "not_assessed_by_conformance"
    );

    let observations = first["observations"]
        .as_array()
        .expect("observations are machine-readable JSON");
    let determinations = observations
        .iter()
        .map(|value| {
            (
                value["label"].as_str().unwrap(),
                value["determination"].as_str().unwrap(),
                value["expected_determination"].as_str().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        determinations,
        vec![
            ("positive", "conformant", "conformant"),
            ("adversarial_valid_negative", "conformant", "conformant"),
            (
                "semantic_violation_detection",
                "non_conformant",
                "non_conformant"
            ),
            (
                "execute_response_loss",
                "not_established",
                "not_established"
            ),
        ]
    );

    let encoded = serde_json::to_string(&first).unwrap();
    let decoded: serde_json::Value = serde_json::from_str(&encoded).unwrap();
    assert_eq!(decoded, first);
}

#[test]
fn standalone_consumer_does_not_link_reference_adapter_or_solver_implementation() {
    let manifest = MANIFEST.to_ascii_lowercase();
    for forbidden in ["sol-mock-adapter", "sol-adapter-authoring-skeleton", "moose", "comsol", "ansys"] {
        assert!(
            !manifest.contains(forbidden),
            "standalone conformance consumer must not link {forbidden}"
        );
    }
}
