#![forbid(unsafe_code)]

use sol_core_model::{EntityKind, OntologyEntity, RelationKind, Simulation, SpatialScope};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Namespace(String);

impl Namespace {
    pub fn parse(value: &str) -> Result<Self, CanonicalIdError> {
        if value.is_empty() || !value.split('.').all(valid_segment) {
            return Err(CanonicalIdError::InvalidNamespace(value.to_owned()));
        }
        Ok(Self(value.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CanonicalId {
    namespace: Namespace,
    local_name: String,
}

impl CanonicalId {
    pub fn new(
        namespace: Namespace,
        local_name: impl Into<String>,
    ) -> Result<Self, CanonicalIdError> {
        let local_name = local_name.into();
        if !valid_segment(&local_name) {
            return Err(CanonicalIdError::InvalidLocalName(local_name));
        }
        Ok(Self {
            namespace,
            local_name,
        })
    }

    pub fn namespace(&self) -> &Namespace {
        &self.namespace
    }

    pub fn local_name(&self) -> &str {
        &self.local_name
    }
}

impl FromStr for CanonicalId {
    type Err = CanonicalIdError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (namespace, local_name) = value
            .rsplit_once('.')
            .ok_or_else(|| CanonicalIdError::MissingNamespace(value.to_owned()))?;
        Self::new(Namespace::parse(namespace)?, local_name)
    }
}

impl Display for CanonicalId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}.{}", self.namespace.as_str(), self.local_name)
    }
}

fn valid_segment(segment: &str) -> bool {
    !segment.is_empty()
        && segment
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || matches!(character, '_' | '-'))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CanonicalIdError {
    MissingNamespace(String),
    InvalidNamespace(String),
    InvalidLocalName(String),
}

impl Display for CanonicalIdError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingNamespace(value) => {
                write!(formatter, "canonical id lacks namespace: {value}")
            }
            Self::InvalidNamespace(value) => {
                write!(formatter, "invalid canonical namespace: {value}")
            }
            Self::InvalidLocalName(value) => {
                write!(formatter, "invalid canonical local name: {value}")
            }
        }
    }
}

impl Error for CanonicalIdError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolvedNodeKind {
    SimulationModel,
    SimulationTask,
    SpatialScope,
    Entity(EntityKind),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedNode {
    pub id: CanonicalId,
    pub kind: ResolvedNodeKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedSpatialScope {
    pub id: CanonicalId,
    pub members: Vec<CanonicalId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedRelation {
    pub kind: RelationKind,
    pub source: CanonicalId,
    pub target: CanonicalId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedGraph {
    nodes: BTreeMap<CanonicalId, ResolvedNode>,
    scopes: BTreeMap<CanonicalId, ResolvedSpatialScope>,
    relations: Vec<ResolvedRelation>,
}

impl ResolvedGraph {
    pub fn nodes(&self) -> &BTreeMap<CanonicalId, ResolvedNode> {
        &self.nodes
    }

    pub fn scopes(&self) -> &BTreeMap<CanonicalId, ResolvedSpatialScope> {
        &self.scopes
    }

    pub fn relations(&self) -> &[ResolvedRelation] {
        &self.relations
    }

    pub fn resolve(&self, id: &CanonicalId) -> Option<&ResolvedNode> {
        self.nodes.get(id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolveError {
    InvalidCanonicalId {
        raw_id: String,
        reason: CanonicalIdError,
    },
    DuplicateCanonicalId(CanonicalId),
    EmptyScope(CanonicalId),
    DuplicateScopeMember {
        scope: CanonicalId,
        member: CanonicalId,
    },
    UnresolvedScopeMember {
        scope: CanonicalId,
        member: CanonicalId,
    },
    NonSpatialScopeMember {
        scope: CanonicalId,
        member: CanonicalId,
        actual: ResolvedNodeKind,
    },
    UnresolvedEndpoint {
        relation: RelationKind,
        endpoint: &'static str,
        id: CanonicalId,
    },
}

impl Display for ResolveError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCanonicalId { raw_id, reason } => {
                write!(formatter, "invalid canonical id {raw_id:?}: {reason}")
            }
            Self::DuplicateCanonicalId(id) => write!(formatter, "duplicate canonical id: {id}"),
            Self::EmptyScope(id) => write!(formatter, "spatial scope has no members: {id}"),
            Self::DuplicateScopeMember { scope, member } => {
                write!(formatter, "duplicate spatial scope member {member} in {scope}")
            }
            Self::UnresolvedScopeMember { scope, member } => {
                write!(formatter, "unresolved spatial scope member {member} in {scope}")
            }
            Self::NonSpatialScopeMember {
                scope,
                member,
                actual,
            } => write!(
                formatter,
                "non-spatial scope member {member} in {scope}: {actual:?}"
            ),
            Self::UnresolvedEndpoint {
                relation,
                endpoint,
                id,
            } => write!(
                formatter,
                "unresolved {endpoint} endpoint {id} for relation {relation:?}"
            ),
        }
    }
}

impl Error for ResolveError {}

pub struct IdentityResolver;

impl IdentityResolver {
    pub fn resolve(simulation: &Simulation) -> Result<ResolvedGraph, ResolveError> {
        let mut nodes = BTreeMap::new();

        insert_raw_node(
            &mut nodes,
            &simulation.model.id,
            ResolvedNodeKind::SimulationModel,
        )?;

        for entity in model_entities(simulation) {
            insert_entity(&mut nodes, entity)?;
        }

        let mut scopes = BTreeMap::new();
        for scope in &simulation.model.scopes {
            let resolved = resolve_scope(&mut nodes, scope)?;
            scopes.insert(resolved.id.clone(), resolved);
        }

        for task in &simulation.tasks {
            insert_raw_node(&mut nodes, &task.id, ResolvedNodeKind::SimulationTask)?;
            for entity in task.analyses.iter().chain(&task.solver_configurations) {
                insert_entity(&mut nodes, entity)?;
            }
        }

        let mut relations = Vec::with_capacity(simulation.relations.len());
        for relation in &simulation.relations {
            let source = parse_id(&relation.source)?;
            let target = parse_id(&relation.target)?;

            if !nodes.contains_key(&source) {
                return Err(ResolveError::UnresolvedEndpoint {
                    relation: relation.kind,
                    endpoint: "source",
                    id: source,
                });
            }
            if !nodes.contains_key(&target) {
                return Err(ResolveError::UnresolvedEndpoint {
                    relation: relation.kind,
                    endpoint: "target",
                    id: target,
                });
            }

            relations.push(ResolvedRelation {
                kind: relation.kind,
                source,
                target,
            });
        }

        Ok(ResolvedGraph {
            nodes,
            scopes,
            relations,
        })
    }
}

fn resolve_scope(
    nodes: &mut BTreeMap<CanonicalId, ResolvedNode>,
    scope: &SpatialScope,
) -> Result<ResolvedSpatialScope, ResolveError> {
    let scope_id = parse_id(&scope.id)?;
    if scope.members.is_empty() {
        return Err(ResolveError::EmptyScope(scope_id));
    }
    if nodes.contains_key(&scope_id) {
        return Err(ResolveError::DuplicateCanonicalId(scope_id));
    }

    let mut seen = BTreeSet::new();
    let mut members = Vec::with_capacity(scope.members.len());
    for raw_member in &scope.members {
        let member = parse_id(raw_member)?;
        if !seen.insert(member.clone()) {
            return Err(ResolveError::DuplicateScopeMember {
                scope: scope_id,
                member,
            });
        }
        let node = nodes
            .get(&member)
            .ok_or_else(|| ResolveError::UnresolvedScopeMember {
                scope: scope_id.clone(),
                member: member.clone(),
            })?;
        if node.kind != ResolvedNodeKind::Entity(EntityKind::SpatialModel) {
            return Err(ResolveError::NonSpatialScopeMember {
                scope: scope_id,
                member,
                actual: node.kind,
            });
        }
        members.push(member);
    }

    nodes.insert(
        scope_id.clone(),
        ResolvedNode {
            id: scope_id.clone(),
            kind: ResolvedNodeKind::SpatialScope,
        },
    );

    Ok(ResolvedSpatialScope {
        id: scope_id,
        members,
    })
}

fn model_entities(simulation: &Simulation) -> impl Iterator<Item = &OntologyEntity> {
    simulation
        .model
        .physics
        .iter()
        .chain(&simulation.model.mathematical)
        .chain(&simulation.model.constitutive)
        .chain(&simulation.model.spatial)
        .chain(&simulation.model.material)
        .chain(&simulation.model.conditions)
        .chain(&simulation.model.numerical)
        .chain(&simulation.model.observations)
}

fn insert_entity(
    nodes: &mut BTreeMap<CanonicalId, ResolvedNode>,
    entity: &OntologyEntity,
) -> Result<(), ResolveError> {
    insert_raw_node(nodes, &entity.id, ResolvedNodeKind::Entity(entity.kind))
}

fn insert_raw_node(
    nodes: &mut BTreeMap<CanonicalId, ResolvedNode>,
    raw_id: &str,
    kind: ResolvedNodeKind,
) -> Result<(), ResolveError> {
    let id = parse_id(raw_id)?;
    if nodes.contains_key(&id) {
        return Err(ResolveError::DuplicateCanonicalId(id));
    }
    nodes.insert(id.clone(), ResolvedNode { id, kind });
    Ok(())
}

fn parse_id(raw_id: &str) -> Result<CanonicalId, ResolveError> {
    raw_id
        .parse()
        .map_err(|reason| ResolveError::InvalidCanonicalId {
            raw_id: raw_id.to_owned(),
            reason,
        })
}

#[cfg(test)]
mod tests {
    use super::{CanonicalId, Namespace};

    #[test]
    fn canonical_id_preserves_namespace_and_local_name() {
        let id: CanonicalId = "thermal.energy_conservation".parse().unwrap();
        assert_eq!(id.namespace().as_str(), "thermal");
        assert_eq!(id.local_name(), "energy_conservation");
        assert_eq!(id.to_string(), "thermal.energy_conservation");
    }

    #[test]
    fn nested_namespace_is_supported_without_backend_identity_semantics() {
        let namespace = Namespace::parse("ontology.thermal").unwrap();
        let id = CanonicalId::new(namespace, "temperature").unwrap();
        assert_eq!(id.to_string(), "ontology.thermal.temperature");
    }

    #[test]
    fn namespace_is_required() {
        assert!("temperature".parse::<CanonicalId>().is_err());
    }
}
