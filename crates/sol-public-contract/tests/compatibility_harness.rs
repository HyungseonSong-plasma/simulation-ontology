use serde_json::Value;
use sol_public_contract::{EvaluationResultDto, SimulationDto, ValidationReport};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CompatibilityOutcome {
    Compatible,
    Incompatible,
    Unknown,
}

impl CompatibilityOutcome {
    fn as_str(self) -> &'static str {
        match self {
            Self::Compatible => "compatible",
            Self::Incompatible => "incompatible",
            Self::Unknown => "unknown",
        }
    }
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}

fn read_text(relative: &str) -> String {
    fs::read_to_string(repo_root().join(relative)).unwrap()
}

fn read_json(relative: &str) -> Value {
    serde_json::from_str(&read_text(relative)).unwrap()
}

fn required_str<'a>(value: &'a Value, key: &str) -> &'a str {
    value
        .get(key)
        .and_then(Value::as_str)
        .unwrap_or_else(|| panic!("compatibility case is missing string field {key}"))
}

fn clear_simulation_extensions(simulation: &mut SimulationDto) {
    simulation.extensions.clear();
    simulation.model.extensions.clear();

    for entity in simulation
        .model
        .physics
        .iter_mut()
        .chain(&mut simulation.model.mathematical)
        .chain(&mut simulation.model.constitutive)
        .chain(&mut simulation.model.spatial)
        .chain(&mut simulation.model.material)
        .chain(&mut simulation.model.conditions)
        .chain(&mut simulation.model.numerical)
        .chain(&mut simulation.model.observations)
    {
        entity.extensions.clear();
    }

    for scope in &mut simulation.model.scopes {
        scope.extensions.clear();
    }

    for task in &mut simulation.tasks {
        task.extensions.clear();
        for entity in task
            .analyses
            .iter_mut()
            .chain(&mut task.solver_configurations)
        {
            entity.extensions.clear();
        }
    }

    for relation in &mut simulation.relations {
        relation.extensions.clear();
    }
}

fn clear_validation_extensions(report: &mut ValidationReport) {
    report.extensions.clear();
    for diagnostic in &mut report.diagnostics {
        diagnostic.extensions.clear();
    }
}

fn assess_simulation(old_fixture: &str, candidate_fixture: &str) -> CompatibilityOutcome {
    let mut old = SimulationDto::from_json(&read_text(old_fixture))
        .expect("old Public Contract simulation fixture must remain conforming");
    let Ok(mut candidate) = SimulationDto::from_json(&read_text(candidate_fixture)) else {
        return CompatibilityOutcome::Incompatible;
    };

    clear_simulation_extensions(&mut old);
    clear_simulation_extensions(&mut candidate);

    if old == candidate {
        CompatibilityOutcome::Compatible
    } else {
        CompatibilityOutcome::Incompatible
    }
}

fn assess_validation_report(old_fixture: &str, candidate_fixture: &str) -> CompatibilityOutcome {
    let mut old = ValidationReport::from_json(&read_text(old_fixture))
        .expect("old ValidationReport fixture must remain conforming");
    let Ok(mut candidate) = ValidationReport::from_json(&read_text(candidate_fixture)) else {
        return CompatibilityOutcome::Incompatible;
    };

    clear_validation_extensions(&mut old);
    clear_validation_extensions(&mut candidate);

    if old == candidate {
        CompatibilityOutcome::Compatible
    } else {
        CompatibilityOutcome::Incompatible
    }
}

fn assess_evaluation(old_fixture: &str, candidate_fixture: &str) -> CompatibilityOutcome {
    let old = EvaluationResultDto::from_json(&read_text(old_fixture))
        .expect("old EvaluationResult fixture must remain conforming");
    let Ok(candidate) = EvaluationResultDto::from_json(&read_text(candidate_fixture)) else {
        return CompatibilityOutcome::Incompatible;
    };

    if old == candidate {
        CompatibilityOutcome::Compatible
    } else {
        CompatibilityOutcome::Incompatible
    }
}

fn assess_case(case: &Value) -> CompatibilityOutcome {
    let rule = required_str(case, "rule");
    let old_fixture = required_str(case, "old_fixture");

    match rule {
        "simulation" => assess_simulation(
            old_fixture,
            required_str(case, "candidate_fixture"),
        ),
        "validation_report" => assess_validation_report(
            old_fixture,
            required_str(case, "candidate_fixture"),
        ),
        "evaluation" => assess_evaluation(
            old_fixture,
            required_str(case, "candidate_fixture"),
        ),
        "missing_evidence" => CompatibilityOutcome::Unknown,
        other => panic!("unknown compatibility rule: {other}"),
    }
}

#[test]
fn public_contract_01_compatibility_manifest_matches_current_semantics() {
    let manifest = read_json("fixtures/compatibility/public-contract-0.1/manifest.json");
    assert_eq!(
        manifest
            .get("public_contract_version")
            .and_then(Value::as_str),
        Some("0.1")
    );

    let declared_outcomes: Vec<_> = manifest
        .get("outcomes")
        .and_then(Value::as_array)
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap())
        .collect();
    assert_eq!(
        declared_outcomes,
        vec!["compatible", "incompatible", "unknown"]
    );

    let cases = manifest.get("cases").and_then(Value::as_array).unwrap();
    assert!(!cases.is_empty());

    for case in cases {
        let id = required_str(case, "id");
        let expected = required_str(case, "expected_outcome");
        let actual = assess_case(case);
        assert_eq!(
            actual.as_str(),
            expected,
            "compatibility case {id} changed classification"
        );
    }
}

#[test]
fn breaking_change_template_contains_required_policy_fields() {
    let template = read_text("docs/templates/breaking-change.md");
    for required in [
        "BREAKING CHANGE",
        "Affected surface / contract",
        "Affected version",
        "Old behavior",
        "New behavior",
        "Affected consumers",
        "Migration path",
    ] {
        assert!(
            template.contains(required),
            "breaking-change template is missing required marker/field: {required}"
        );
    }
}
