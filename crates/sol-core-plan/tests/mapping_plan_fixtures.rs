use serde::Deserialize;
use sol_core_plan::{MappingPlan, PlanAction, PlanActionId, PlanError};

#[derive(Debug, Deserialize)]
struct PlanFixture {
    actions: Vec<ActionFixture>,
}

#[derive(Debug, Deserialize)]
struct ActionFixture {
    id: String,
    dependencies: Vec<String>,
}

fn load_plan(raw: &str) -> Result<MappingPlan, PlanError> {
    let fixture: PlanFixture = serde_json::from_str(raw).unwrap();
    let actions = fixture.actions.into_iter().map(|action| {
        action
            .dependencies
            .into_iter()
            .fold(PlanAction::new(action.id), |plan_action, dependency| {
                plan_action.depends_on(dependency)
            })
    });

    MappingPlan::from_actions(actions)
}

#[test]
fn valid_thermal_mapping_plan_fixture_orders_deterministically() {
    let plan = load_plan(include_str!(
        "../../../fixtures/golden/thermal-mapping-plan-valid.json"
    ))
    .unwrap();

    assert_eq!(
        plan.topological_order().unwrap(),
        vec![
            PlanActionId::new("thermal.domain"),
            PlanActionId::new("thermal.material"),
            PlanActionId::new("thermal.solve"),
        ]
    );
}

#[test]
fn cyclic_thermal_mapping_plan_fixture_is_rejected() {
    let error = load_plan(include_str!(
        "../../../fixtures/counterexamples/thermal-mapping-plan-cycle.json"
    ))
    .unwrap_err();

    assert!(matches!(error, PlanError::CycleDetected(_)));
}
