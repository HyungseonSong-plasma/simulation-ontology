use std::path::Path;
use std::process::Command;

use sol_public_contract::CanonicalDocument;

fn thermal_fixture() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/thermal/thermal-reference.json")
}

fn public_thermal_fixture() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/public-contract/0.1/thermal-simulation.json")
}

fn canonical_fixture(relative: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    let input = std::fs::read_to_string(path).unwrap();
    CanonicalDocument::parse(&input)
        .unwrap()
        .to_canonical_json()
        .unwrap()
}

#[test]
fn validate_thermal_cli_human_output_remains_distinct() {
    let output = Command::new(env!("CARGO_BIN_EXE_sol-cli"))
        .arg("validate")
        .arg(thermal_fixture())
        .output()
        .unwrap();

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "VALIDATION PASS\nontology_version=0.1\nmodel=model.thermal_reference\ncanonical_nodes=17\ncanonical_relations=9\n"
    );
}

#[test]
fn plan_thermal_mock_cli_human_output_remains_distinct() {
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

#[test]
fn validate_json_cli_matches_public_validation_report_golden() {
    let output = Command::new(env!("CARGO_BIN_EXE_sol-cli"))
        .arg("validate")
        .arg(public_thermal_fixture())
        .arg("--json")
        .output()
        .unwrap();

    assert!(output.status.success());
    let expected = format!(
        "{}\n",
        canonical_fixture("../../fixtures/public-contract/0.1/thermal-validation-report.json")
    );
    assert_eq!(String::from_utf8(output.stdout).unwrap(), expected);
}

#[test]
fn plan_json_cli_matches_public_mapping_plan_golden() {
    let output = Command::new(env!("CARGO_BIN_EXE_sol-cli"))
        .arg("plan")
        .arg(public_thermal_fixture())
        .arg("--target")
        .arg("mock")
        .arg("--json")
        .output()
        .unwrap();

    assert!(output.status.success());
    let expected = format!(
        "{}\n",
        canonical_fixture("../../fixtures/public-contract/0.1/thermal-mapping-plan.json")
    );
    assert_eq!(String::from_utf8(output.stdout).unwrap(), expected);
}
