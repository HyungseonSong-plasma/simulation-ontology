#![forbid(unsafe_code)]

use std::collections::BTreeSet;

use sol_core_identity::{CanonicalId, IdentityResolver, ResolvedGraph};
use sol_core_model::Simulation;
use sol_core_plan::{MappingPlan, PlanAction};
use sol_mock_adapter::MockAdapter;
use sol_target_resolver::{
    resolve_target, AdapterDescriptor, BackendCapability, BackendTarget, ResolveTargetError,
};

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

pub fn plan_document(input: &str, target: &str) -> Result<String, String> {
    let simulation = Simulation::from_json(input).map_err(|error| error.to_string())?;
    let graph = IdentityResolver::resolve(&simulation).map_err(|error| error.to_string())?;
    verify_thermal_reference(&graph)?;

    let plan = thermal_mapping_plan().map_err(|error| format!("invalid mapping plan: {error:?}"))?;
    let adapter = MockAdapter::thermal();
    let descriptor = AdapterDescriptor::from_adapter("mock.thermal", BackendTarget::mock(), &adapter);
    let descriptors = [descriptor];
    let required = thermal_requirements();
    let target = BackendTarget::new(target);
    let selected = resolve_target(&target, &plan, &required, &descriptors)
        .map_err(resolve_target_error_to_string)?;
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

fn verify_thermal_reference(graph: &ResolvedGraph) -> Result<(), String> {
    for raw_id in [
        "thermal.energy_conservation",
        "domain.main",
        "material.copper",
        "solver.default",
    ] {
        let id: CanonicalId = raw_id.parse().map_err(|error| format!("{error}"))?;
        if graph.resolve(&id).is_none() {
            return Err(format!("thermal reference missing required semantic entity: {id}"));
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
    use super::{plan_document, validate_document};

    const THERMAL: &str = include_str!("../../../fixtures/thermal/thermal-reference.json");

    #[test]
    fn thermal_validation_output_is_canonical() {
        assert_eq!(
            validate_document(THERMAL).unwrap(),
            "VALIDATION PASS\nontology_version=0.1\nmodel=model.thermal_reference\ncanonical_nodes=15\ncanonical_relations=9"
        );
    }

    #[test]
    fn thermal_mock_mapping_plan_output_is_golden() {
        assert_eq!(
            plan_document(THERMAL, "mock").unwrap(),
            "MAPPING PLAN PASS\ntarget=mock\nadapter=mock.thermal\nactions=3\norder=thermal.domain -> thermal.material -> thermal.solve"
        );
    }

    #[test]
    fn non_mock_target_is_rejected_in_m01() {
        assert_eq!(
            plan_document(THERMAL, "comsol").unwrap_err(),
            "no compatible adapter"
        );
    }
}
