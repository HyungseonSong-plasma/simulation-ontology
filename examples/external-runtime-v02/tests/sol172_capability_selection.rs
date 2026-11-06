//! SOL #172: live runtime selection rejection, without inventing plasma support.
use serde_json::{json, Value};
use sol_external_runtime_v02_consumer::{
    adapter_from_env, fixture_from_env, run_consumer_v02,
};
use std::fs;

#[test]
fn unadvertised_plasma_capability_fails_closed_before_validation_or_execution() {
    let adapter = adapter_from_env("SOL_RUNTIME_V02_ADAPTER").unwrap();
    let fixture = fixture_from_env().unwrap();
    let mut plan_request: Value =
        serde_json::from_str(&fs::read_to_string(fixture).unwrap()).unwrap();
    // The published external-runtime-v02 fixture adapter advertises mock thermal
    // capabilities, not plasma. The candidate capability is deliberately
    // unsupported; it must not be routed to the adapter as a HIT edit command.
    plan_request["target"]["target"] = json!("mock");
    let result = run_consumer_v02(
        &adapter,
        &json!({
            "target": "mock",
            "required_capabilities": [
                "thermal.domain",
                "thermal.material",
                "thermal.solve",
                "plasma.electron_energy_diffusion"
            ],
            "plan_request": plan_request
        }),
    );
    assert_eq!(result.unwrap_err(), "NO_COMPATIBLE_CANDIDATE");
}
