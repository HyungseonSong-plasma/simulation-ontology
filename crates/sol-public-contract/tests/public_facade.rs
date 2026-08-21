use serde_json::Value;
use sol_public_contract::{
    BackendTargetDto, EvaluationResultDto, MappingClaimsDto, MappingPlanDto,
    RealizationEffectDocumentDto, SimulationDto, ValidationReport,
};
use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}

fn read(relative: &str) -> String {
    fs::read_to_string(repo_root().join(relative)).unwrap()
}

#[test]
fn sdk_like_consumer_can_use_public_contract_from_one_crate_root() {
    SimulationDto::from_json(&read(
        "fixtures/public-contract/0.1/thermal-simulation.json",
    ))
    .unwrap();
    ValidationReport::from_json(&read(
        "fixtures/public-contract/0.1/thermal-validation-report.json",
    ))
    .unwrap();
    MappingClaimsDto::from_json(&read(
        "fixtures/public-contract/0.1/thermal-mapping-claims.json",
    ))
    .unwrap();
    MappingPlanDto::from_json(&read(
        "fixtures/public-contract/0.1/thermal-mapping-plan.json",
    ))
    .unwrap();
    BackendTargetDto::from_json(&read(
        "fixtures/public-contract/0.1/thermal-backend-target.json",
    ))
    .unwrap();
    RealizationEffectDocumentDto::from_json(&read(
        "fixtures/public-contract/0.1/thermal-realization-effect.json",
    ))
    .unwrap();
    EvaluationResultDto::from_json(&read(
        "fixtures/public-contract/0.1/thermal-evaluation.json",
    ))
    .unwrap();
}

#[test]
fn public_contract_manifest_does_not_require_internal_core_or_adapter_crates() {
    let manifest = read("crates/sol-public-contract/Cargo.toml");
    for forbidden in [
        "sol-core-",
        "sol-mock-adapter",
        "sol-target-resolver",
        "sol-cli",
    ] {
        assert!(
            !manifest.contains(forbidden),
            "public facade leaked internal/runtime dependency marker: {forbidden}"
        );
    }
}

#[test]
fn public_contract_schemas_do_not_define_adapter_protocol_operations() {
    let schema_dir = repo_root().join("schemas/public-contract/0.1");
    for entry in fs::read_dir(schema_dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|value| value.to_str()) != Some("json") {
            continue;
        }
        let value: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        let serialized = serde_json::to_string(&value).unwrap();
        for forbidden in [
            "describe_adapter",
            "validate_plan",
            "execute_plan",
            "jsonrpc",
            "stdio",
            "transport_method",
        ] {
            assert!(
                !serialized.contains(forbidden),
                "Public Contract schema {} leaked Adapter Protocol/transport marker {forbidden}",
                path.display()
            );
        }
    }
}
