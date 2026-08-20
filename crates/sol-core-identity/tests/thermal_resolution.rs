use sol_core_identity::{CanonicalId, IdentityResolver, ResolveError, ResolvedNodeKind};
use sol_core_model::{EntityKind, Simulation};

const THERMAL_FIXTURE: &str = include_str!("../../../fixtures/thermal/thermal-reference.json");
const UNRESOLVED_FIXTURE: &str =
    include_str!("../../../fixtures/counterexamples/thermal-unresolved-reference.json");

#[test]
fn thermal_graph_resolves_deterministically() {
    let simulation = Simulation::from_json(THERMAL_FIXTURE).expect("thermal fixture must deserialize");

    let first = IdentityResolver::resolve(&simulation).expect("thermal graph must resolve");
    let second = IdentityResolver::resolve(&simulation).expect("thermal graph must resolve again");

    assert_eq!(first, second);
    assert_eq!(first.nodes().len(), 15);
    assert_eq!(first.relations().len(), 9);

    let model_id: CanonicalId = "model.thermal_reference".parse().unwrap();
    assert_eq!(
        first.resolve(&model_id).map(|node| node.kind),
        Some(ResolvedNodeKind::SimulationModel)
    );

    let temperature_id: CanonicalId = "thermal.temperature_field".parse().unwrap();
    assert_eq!(
        first.resolve(&temperature_id).map(|node| node.kind),
        Some(ResolvedNodeKind::Entity(EntityKind::MathematicalModel))
    );
}

#[test]
fn unresolved_relation_endpoint_is_rejected() {
    let simulation =
        Simulation::from_json(UNRESOLVED_FIXTURE).expect("counterexample fixture must deserialize");

    let error = IdentityResolver::resolve(&simulation).expect_err("missing endpoint must fail");

    assert!(matches!(
        error,
        ResolveError::UnresolvedEndpoint {
            endpoint: "target",
            ..
        }
    ));
}

#[test]
fn duplicate_canonical_id_is_rejected() {
    let mut simulation = Simulation::from_json(THERMAL_FIXTURE).expect("thermal fixture must deserialize");
    simulation.tasks[0].analyses[0].id = simulation.model.physics[0].id.clone();

    let error = IdentityResolver::resolve(&simulation).expect_err("duplicate id must fail");
    assert!(matches!(error, ResolveError::DuplicateCanonicalId(_)));
}
