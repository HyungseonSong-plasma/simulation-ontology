use sol_core_model::{EntityKind, RelationKind, Simulation};

const THERMAL_FIXTURE: &str = include_str!("../../../fixtures/thermal/thermal-reference.json");

#[test]
fn thermal_reference_model_round_trips_through_json() {
    let simulation = Simulation::from_json(THERMAL_FIXTURE).expect("thermal fixture must deserialize");

    assert_eq!(simulation.ontology_version, "0.1");
    assert_eq!(simulation.model.id, "model.thermal_reference");
    assert_eq!(simulation.model.physics.len(), 1);
    assert_eq!(simulation.model.mathematical.len(), 2);
    assert_eq!(simulation.model.constitutive.len(), 1);
    assert_eq!(simulation.model.spatial.len(), 2);
    assert_eq!(simulation.model.material.len(), 2);
    assert_eq!(simulation.model.conditions.len(), 1);
    assert_eq!(simulation.model.numerical.len(), 1);
    assert_eq!(simulation.model.observations.len(), 1);
    assert_eq!(simulation.tasks.len(), 1);
    assert_eq!(simulation.relations.len(), 9);

    assert_eq!(simulation.model.physics[0].kind, EntityKind::PhysicsModel);
    assert!(simulation
        .relations
        .iter()
        .any(|relation| relation.kind == RelationKind::RepresentedBy));
    assert!(simulation
        .relations
        .iter()
        .any(|relation| relation.kind == RelationKind::SolvedBy));

    let serialized = simulation
        .to_json_pretty()
        .expect("thermal model must serialize");
    let reparsed = Simulation::from_json(&serialized).expect("serialized model must deserialize");

    assert_eq!(simulation, reparsed);
}
