#![forbid(unsafe_code)]

use std::collections::BTreeSet;

use sol_core_identity::{CanonicalId, IdentityResolver, ResolvedGraph};
use sol_core_model::Simulation;
use sol_core_plan::{MappingPlan, PlanAction};
use sol_mock_adapter::MockAdapter;
use sol_public_contract::{validate_simulation, MappingPlanDto, PlanActionDto, SimulationDto};
use sol_target_resolver::{
    resolve_target, AdapterDescriptor, BackendCapability, BackendTarget, ResolveTargetError,
};

/// Human-readable validation output for the original M0.1 CLI path.
///
/// This text is intentionally distinct from the declared Public Contract
/// machine-facing JSON output exposed by [`validate_public_contract_document`].
pub fn validate_document(input: &str) -> Result<String, String> {
    let simulation = Simulation::from_json(input).map_err(|error| error.to_string())?;
    let graph = IdentityResolver::resolve(&simulation).map_err(|error| error.to_string())?;

    Ok(format!(
        "VALIDATION PASS\nontology_version={}\nmodel={}\ncanonical_nodes={}\ncanonical_relations={}",
        simulation.ontology_version,
        simulation.model.id,
        graph.nodes().len(),
        graph.relations().len()
    ))
}

/// Machine-facing validation path for Public Contract 0.1.
///
/// The input must be a Public Contract 0.1 `SimulationDto`. The returned string
/// is canonical Public Contract `ValidationReport` JSON and therefore does not
/// depend on Rust `Debug`/`Display` formatting or internal resolver layout.
pub fn validate_public_contract_document(input: &str) -> Result<String, String> {
    let simulation = SimulationDto::from_json(input).map_err(|error| error.to_string())?;
    validate_simulation(&simulation)
        .to_canonical_json()
        .map_err(|error| error.to_string())
}

/// Human-readable planning output for the original M0.1 CLI path.
pub fn plan_document(input: &str, target: &str) -> Result<String, String> {
    let simulation = Simulation::from_json(input).map_err(|error| error.to_string())?;
    let graph = IdentityResolver::resolve(&simulation).map_err(|error| error.to_string())?;
    verify_thermal_reference(&graph)?;

    let plan =
        thermal_mapping_plan().map_err(|error| format!("invalid mapping plan: {error:?}"))?;
    let selected = resolve_thermal_target(&plan, target)?;
    let order = plan
        .topological_order()
        .map_err(|error| format!("invalid mapping plan: {error:?}"))?
        .into_iter()
        .map(|id| id.as_str().to_owned())
        .collect::<Vec<_>>()
        .join(" -> ");

    Ok(format!(
        "MAPPING PLAN PASS\ntarget={}\nadapter={}\nactions={}\norder={}",
        selected.target.as_str(),
        selected.id,
        plan.topological_order()
            .map_err(|error| format!("invalid mapping plan: {error:?}"))?
            .len(),
        order
    ))
}

/// Machine-facing planning path for Public Contract 0.1.
///
/// Target compatibility is checked through the current reference resolver, but
/// the returned payload is the canonical solver-independent `MappingPlanDto`.
/// Adapter identity is deliberately not serialized into the Public Contract
/// plan payload.
pub fn plan_public_contract_document(input: &str, target: &str) -> Result<String, String> {
    SimulationDto::from_json(input).map_err(|error| error.to_string())?;

    let simulation = Simulation::from_json(input).map_err(|error| error.to_string())?;
    let graph = IdentityResolver::resolve(&simulation).map_err(|error| error.to_string())?;
    verify_thermal_reference(&graph)?;

    let core_plan =
        thermal_mapping_plan().map_err(|error| format!("invalid mapping plan: {error:?}"))?;
    resolve_thermal_target(&core_plan, target)?;

    MappingPlanDto::new(vec![
        PlanActionDto::new("thermal.domain"),
        PlanActionDto::new("thermal.material").depends_on("thermal.domain"),
        PlanActionDto::new("thermal.solve").depends_on("thermal.material"),
    ])
    .map_err(|error| error.to_string())?
    .to_canonical_json()
    .map_err(|error| error.to_string())
}

fn resolve_thermal_target<'a>(
    plan: &MappingPlan,
    target: &str,
) -> Result<AdapterDescriptor, String> {
    let adapter = MockAdapter::thermal();
    let descriptor =
        AdapterDescriptor::from_adapter("mock.thermal", BackendTarget::mock(), &adapter);
    let descriptors = [descriptor];
    let required = thermal_requirements();
    let target = BackendTarget::new(target);
    resolve_target(&target, plan, &required, &descriptors)
        .cloned()
        .map_err(resolve_target_error_to_string)
}

fn verify_thermal_reference(graph: &ResolvedGraph) -> Result<(), String> {
    for raw_id in [
        "thermal.energy_conservation",
        "scope.main_domain",
        "scope.hot_wall",
        "material.copper",
        "solver.default",
    ] {
        let id: CanonicalId = raw_id.parse().map_err(|error| format!("{error}"))?;
        if graph.resolve(&id).is_none() {
            return Err(format!(
                "thermal reference missing required semantic entity: {id}"
            ));
        }
    }
    Ok(())
}

fn thermal_mapping_plan() -> Result<MappingPlan, sol_core_plan::PlanError> {
    MappingPlan::from_actions([
        PlanAction::new("thermal.domain"),
        PlanAction::new("thermal.material").depends_on("thermal.domain"),
        PlanAction::new("thermal.solve").depends_on("thermal.material"),
    ])
}

fn thermal_requirements() -> BTreeSet<BackendCapability> {
    [
        BackendCapability::new("thermal.domain"),
        BackendCapability::new("thermal.material"),
        BackendCapability::new("thermal.solve"),
    ]
    .into_iter()
    .collect()
}

fn resolve_target_error_to_string(error: ResolveTargetError) -> String {
    match error {
        ResolveTargetError::InvalidPlan => "invalid mapping plan".to_owned(),
        ResolveTargetError::NoCompatibleAdapter => "no compatible adapter".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        plan_document, plan_public_contract_document, validate_document,
        validate_public_contract_document,
    };

    const THERMAL: &str = include_str!("../../../fixtures/thermal/thermal-reference.json");
    const PUBLIC_THERMAL: &str =
        include_str!("../../../fixtures/public-contract/0.1/thermal-simulation.json");
    const PUBLIC_VALIDATION: &str =
        include_str!("../../../fixtures/public-contract/0.1/thermal-validation-report.json");
    const PUBLIC_PLAN: &str =
        include_str!("../../../fixtures/public-contract/0.1/thermal-mapping-plan.json");

    #[test]
    fn thermal_validation_output_is_human_readable() {
        assert_eq!(
            validate_document(THERMAL).unwrap(),
            "VALIDATION PASS\nontology_version=0.1\nmodel=model.thermal_reference\ncanonical_nodes=17\ncanonical_relations=9"
        );
    }

    #[test]
    fn thermal_mock_mapping_plan_output_is_human_readable() {
        assert_eq!(
            plan_document(THERMAL, "mock").unwrap(),
            "MAPPING PLAN PASS\ntarget=mock\nadapter=mock.thermal\nactions=3\norder=thermal.domain -> thermal.material -> thermal.solve"
        );
    }

    #[test]
    fn public_validation_output_matches_canonical_contract() {
        let expected = sol_public_contract::CanonicalDocument::parse(PUBLIC_VALIDATION)
            .unwrap()
            .to_canonical_json()
            .unwrap();
        assert_eq!(
            validate_public_contract_document(PUBLIC_THERMAL).unwrap(),
            expected
        );
    }

    #[test]
    fn public_plan_output_matches_canonical_contract() {
        let expected = sol_public_contract::CanonicalDocument::parse(PUBLIC_PLAN)
            .unwrap()
            .to_canonical_json()
            .unwrap();
        assert_eq!(
            plan_public_contract_document(PUBLIC_THERMAL, "mock").unwrap(),
            expected
        );
    }

    #[test]
    fn non_mock_target_is_rejected_in_m01() {
        assert_eq!(
            plan_document(THERMAL, "comsol").unwrap_err(),
            "no compatible adapter"
        );
        assert_eq!(
            plan_public_contract_document(PUBLIC_THERMAL, "comsol").unwrap_err(),
            "no compatible adapter"
        );
    }
}
