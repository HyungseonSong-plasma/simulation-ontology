use std::path::Path;
use std::process::Command;

fn thermal_fixture() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/thermal/thermal-reference.json")
}

#[test]
fn validate_thermal_cli_output_matches_golden_contract() {
    let output = Command::new(env!("CARGO_BIN_EXE_sol-cli"))
        .arg("validate")
        .arg(thermal_fixture())
        .output()
        .unwrap();

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "VALIDATION PASS\nontology_version=0.1\nmodel=model.thermal_reference\ncanonical_nodes=15\ncanonical_relations=9\n"
    );
}

#[test]
fn plan_thermal_mock_cli_output_matches_golden_contract() {
    let output = Command::new(env!("CARGO_BIN_EXE_sol-cli"))
        .arg("plan")
        .arg(thermal_fixture())
        .arg("--target")
        .arg("mock")
        .output()
        .unwrap();

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "MAPPING PLAN PASS\ntarget=mock\nadapter=mock.thermal\nactions=3\norder=thermal.domain -> thermal.material -> thermal.solve\n"
    );
}
