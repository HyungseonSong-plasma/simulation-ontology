use sol_core_identity::{IdentityResolver, ResolveError, ResolvedNodeKind};
use sol_core_model::{EntityKind, Simulation};

const THERMAL: &str = include_str!("../../../fixtures/thermal/thermal-reference.json");

#[test]
fn thermal_scopes_resolve_to_spatial_members() {
    let simulation = Simulation::from_json(THERMAL).unwrap();
    let graph = IdentityResolver::resolve(&simulation).unwrap();
    let scope_id = "scope.hot_wall".parse().unwrap();
    let scope = graph.scopes().get(&scope_id).unwrap();

    assert_eq!(scope.members.len(), 1);
    assert_eq!(scope.members[0].to_string(), "boundary.hot_wall");
    assert_eq!(
        graph.resolve(&scope_id).unwrap().kind,
        ResolvedNodeKind::SpatialScope
    );
}

#[test]
fn empty_scope_is_rejected() {
    let mut simulation = Simulation::from_json(THERMAL).unwrap();
    simulation.model.scopes[0].members.clear();

    assert!(matches!(
        IdentityResolver::resolve(&simulation),
        Err(ResolveError::EmptyScope(_))
    ));
}

#[test]
fn duplicate_scope_member_is_rejected() {
    let mut simulation = Simulation::from_json(THERMAL).unwrap();
    simulation.model.scopes[0]
        .members
        .push("domain.main".to_owned());

    assert!(matches!(
        IdentityResolver::resolve(&simulation),
        Err(ResolveError::DuplicateScopeMember { .. })
    ));
}

#[test]
fn unresolved_scope_member_is_rejected() {
    let mut simulation = Simulation::from_json(THERMAL).unwrap();
    simulation.model.scopes[0].members = vec!["domain.missing".to_owned()];

    assert!(matches!(
        IdentityResolver::resolve(&simulation),
        Err(ResolveError::UnresolvedScopeMember { .. })
    ));
}

#[test]
fn non_spatial_scope_member_is_rejected() {
    let mut simulation = Simulation::from_json(THERMAL).unwrap();
    simulation.model.scopes[0].members = vec!["material.copper".to_owned()];

    assert!(matches!(
        IdentityResolver::resolve(&simulation),
        Err(ResolveError::NonSpatialScopeMember {
            actual: ResolvedNodeKind::Entity(EntityKind::MaterialModel),
            ..
        })
    ));
}
