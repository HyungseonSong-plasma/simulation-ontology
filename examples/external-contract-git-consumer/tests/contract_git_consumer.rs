//! Package boundary exercise: this separate Cargo project imports immutable
//! upstream Git crates. It is not a declared stable release certification.
use sol_adapter_protocol::{
    AdapterBootstrap, ADAPTER_PROTOCOL_VERSION_0_1, ADAPTER_PROTOCOL_VERSION_0_2,
};
use sol_adapter_transport::{DESCRIBE_ADAPTER_METHOD, EXECUTE_PLAN_METHOD, VALIDATE_PLAN_METHOD};
use sol_public_contract::{
    RealizationSpecDtoV02, SimulationRunRecordDtoV03, PUBLIC_CONTRACT_VERSION_0_1,
    PUBLIC_CONTRACT_VERSION_0_2, PUBLIC_CONTRACT_VERSION_0_3,
};

#[test]
fn independently_resolved_git_contract_exports_are_available() {
    assert_eq!(PUBLIC_CONTRACT_VERSION_0_1, "0.1");
    assert_eq!(PUBLIC_CONTRACT_VERSION_0_2, "0.2");
    assert_eq!(PUBLIC_CONTRACT_VERSION_0_3, "0.3");
    assert_eq!(ADAPTER_PROTOCOL_VERSION_0_1, "0.1");
    assert_eq!(ADAPTER_PROTOCOL_VERSION_0_2, "0.2");
    assert_eq!(DESCRIBE_ADAPTER_METHOD, "describe_adapter");
    assert_eq!(VALIDATE_PLAN_METHOD, "validate_plan");
    assert_eq!(EXECUTE_PLAN_METHOD, "execute_plan");
}

#[test]
fn accepted_thermal_v02_realization_is_readable_using_git_packages() {
    let fixture = std::fs::read_to_string(
        std::env::var("SOL_THERMAL_SPEC_FIXTURE").expect("required fixture path"),
    )
    .unwrap();
    let spec = RealizationSpecDtoV02::from_json(&fixture).unwrap();
    let json = spec.to_canonical_json().unwrap();
    assert!(json.contains("unit.kelvin"));
    assert!(json.contains("unit.watt_per_meter_kelvin"));
    assert!(json.contains("scope.main_domain"));
    assert!(json.contains("observation.maximum_temperature"));
}

#[test]
fn accepted_scalar_v03_is_readable_without_dataset_promotion() {
    let fixture = std::fs::read_to_string(
        std::env::var("SOL_THERMAL_SCALAR_FIXTURE").expect("required fixture path"),
    )
    .unwrap();
    let record = SimulationRunRecordDtoV03::from_json(&fixture).unwrap();
    let json = record.to_canonical_json().unwrap();
    assert!(json.contains("observation.maximum_temperature"));
    assert!(json.contains("unit.kelvin"));
    assert!(json.contains("run.thermal_reference_001"));
}

#[test]
fn consumer_capability_declaration_does_not_automatically_upgrade_to_protocol_03() {
    let mut bootstrap = AdapterBootstrap {
        adapter_id: "sol.adapter.package_consumer".into(),
        adapter_version: "0.1.0".into(),
        supported_adapter_protocol_versions: Some(vec![
            ADAPTER_PROTOCOL_VERSION_0_1.into(),
            ADAPTER_PROTOCOL_VERSION_0_2.into(),
        ]),
        supported_public_contract_versions: Some(vec![
            PUBLIC_CONTRACT_VERSION_0_1.into(),
            PUBLIC_CONTRACT_VERSION_0_2.into(),
        ]),
        extensions: Default::default(),
    };
    bootstrap.normalize().unwrap();
    assert!(!bootstrap
        .supported_adapter_protocol_versions
        .as_ref()
        .unwrap()
        .iter()
        .any(|version| version == "0.3"));
    assert!(!bootstrap
        .supported_public_contract_versions
        .as_ref()
        .unwrap()
        .iter()
        .any(|version| version == "0.3"));
}
