use sol_core_identity::{CanonicalId, IdentityResolver};
use sol_core_model::{EntityKind, RelationKind, Simulation};
use sol_core_qrc::{evaluate, Constraint, ConstraintViolation};

fn load_fixture(path: &str) -> Simulation {
    Simulation::from_json(path).expect("fixture must deserialize")
}

#[test]
fn thermal_positive_fixture_passes_qrc_constraints() {
    let simulation = load_fixture(include_str!(
        "../../../fixtures/thermal/thermal-reference.json"
    ));
    let graph = IdentityResolver::resolve(&simulation).expect("fixture must resolve");

    let constraints = [
        Constraint::EntityType {
            entity: "thermal.transport".parse().unwrap(),
            expected: EntityKind::PhysicsModel,
        },
        Constraint::Relation {
            source: "thermal.transport".parse().unwrap(),
            kind: RelationKind::RepresentedBy,
            target: "thermal.energy_conservation".parse().unwrap(),
        },
        Constraint::RequiredRelation {
            source: "thermal.transport".parse().unwrap(),
            kind: RelationKind::RepresentedBy,
            target_kind: EntityKind::MathematicalModel,
        },
        Constraint::RelationCardinality {
            source: "thermal.transport".parse().unwrap(),
            kind: RelationKind::RepresentedBy,
            target_kind: EntityKind::MathematicalModel,
            min: 1,
            max: Some(1),
        },
        Constraint::ReferenceIntegrity,
    ];

    for constraint in &constraints {
        assert_eq!(evaluate(&graph, constraint), Ok(()));
    }
}

#[test]
fn thermal_invalid_relation_fixture_fails_qrc_relation_constraint() {
    let simulation = load_fixture(include_str!(
        "../../../fixtures/counterexamples/thermal-invalid-relation.json"
    ));
    let graph = IdentityResolver::resolve(&simulation).expect("fixture endpoints must resolve");
    let source: CanonicalId = "thermal.transport".parse().unwrap();
    let target: CanonicalId = "thermal.energy_conservation".parse().unwrap();

    let constraint = Constraint::Relation {
        source: source.clone(),
        kind: RelationKind::RepresentedBy,
        target: target.clone(),
    };

    assert_eq!(
        evaluate(&graph, &constraint),
        Err(ConstraintViolation::MissingRelation {
            source,
            kind: RelationKind::RepresentedBy,
            target,
        })
    );
}
