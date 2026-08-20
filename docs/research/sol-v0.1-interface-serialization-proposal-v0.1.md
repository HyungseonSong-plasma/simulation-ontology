# SOL v0.1 Interface Serialization Proposal v0.1

**Status:** Research proposal  
**Date:** 2026-08-20  
**Scope:** Language/schema consolidation of accepted ADR-0008/0014 Interface semantics  
**Depends on:** ADR-0007, ADR-0008, ADR-0009, ADR-0014, ADR-0018 through ADR-0024

## 1. Problem

SOL v0.1 already accepts `Interface` as a language-level reusable capability contract, distinct from Entity taxonomy and from the spatial `SpatialInterface` Entity. The remaining question is the minimum deterministic machine representation for:

- Interface identity;
- Interface extension;
- Property requirements;
- Relation requirements;
- attached/referenced Constraints;
- Entity Type implementation declarations;
- explicit implementation mappings.

The representation must not copy MOOSE/COMSOL/Ansys implementation-language hierarchies and must remain compatible with later canonical package integration.

## 2. External reference evidence

### Palantir

Current Palantir Ontology documentation treats an Interface as a separate ontology type describing an object shape/capability. Interfaces may contain properties and link-type constraints, may extend other interfaces, and may be implemented by multiple object types. Required interface properties/links are fulfilled by mappings to concrete object properties/links during implementation.

This is strong evidence for explicit contract-member mapping rather than name-based structural guessing.

### MOOSE

MOOSE exposes reusable capability interfaces such as `Coupleable` and `BlockRestrictable`. These capabilities contribute parameters/behavior to many unrelated MOOSE objects. They are backend implementation evidence for orthogonal capabilities, not candidates for direct SOL Entity inheritance.

### COMSOL

COMSOL `PhysicsFeature` is itself a Java interface extending multiple capability interfaces such as `ParameterEntity` and selection/equation-view capabilities. This again supports capability composition while reinforcing ADR-0008's rule that backend Java inheritance is non-normative to SOL taxonomy.

### Ansys

Ansys System Coupling participant variables expose reusable semantic capability contracts such as quantity type, tensor type, transfer direction, and location. Variables are admitted to transfers by compatible semantic capability rather than by a single universal backend class hierarchy. This supports contract-oriented validation while not implying an Ansys-specific Interface taxonomy in SOL Core.

## 3. InterfaceDefinition is a schema resource, not an Entity

`Interface` remains a language-level schema/capability resource.

A normalized InterfaceDefinition has stable canonical identity under ADR-0009:

```yaml
id: https://simulation-ontology.org/id/<interface-id>
extends:
  - https://simulation-ontology.org/id/<parent-interface-id>
property_requirements:
  - property: https://simulation-ontology.org/id/<property-id>
relation_requirements:
  - relation: https://simulation-ontology.org/id/<relation-id>
constraint_requirements:
  - https://simulation-ontology.org/id/<constraint-definition-id>
```

`extends`, requirement arrays, and constraint references are empty by default.

This proposal does not make Interface an Entity Type and does not place Interface in `ontology/core/entities.yaml`.

## 4. Requirement identity

A v0.1 Interface requirement is identified by the canonical semantic identity of the required Property or Relation.

```text
Property requirement key = canonical Property identity
Relation requirement key = canonical Relation identity
```

A second interface-local requirement identity layer is not introduced in v0.1.

If two semantically distinct roles are required, they require distinct semantic Property/Relation identities rather than two differently named copies of the same canonical requirement.

Requirement arrays therefore have set semantics after canonical reference resolution.

## 5. Requirement requiredness

Every Property or Relation listed in an InterfaceDefinition is a required **schema capability** for an implementing Entity Type.

This does not imply that every model instance must contain a concrete value or relation edge. Model-instance requiredness/cardinality remains Constraint semantics.

Therefore:

```text
schema capability exists
    !=
model instance has at least one value/edge
```

An explicit `optional: true|false` Interface-requirement flag is not introduced in v0.1. Optional capability members are deferred until evidence requires them.

Palantir supports optional interface members, but SOL does not copy that product feature merely because it exists.

## 6. Interface Constraints

ADR-0008 permits Interfaces to declare or reference Constraints. For the v0.1 normalized serialization, InterfaceDefinition stores **canonical references to reified ConstraintDefinitions**:

```yaml
constraint_requirements:
  - https://simulation-ontology.org/id/<constraint-definition-id>
```

This focused choice avoids inventing a second inline constraint-targeting syntax before canonical package integration.

Reification is justified here under ADR-0007 because an Interface constraint is inherited/reused through a named capability contract and requires stable provenance during Interface extension and implementation validation.

The referenced ConstraintDefinition remains one of the accepted six Constraint families; Interface does not define a seventh family or a parallel constraint algebra.

Inline context-local constraints outside Interface serialization remain valid under ADR-0007.

## 7. Interface extension

Only **direct** `extends` edges are serialized.

The effective Interface contract is the transitive closure of direct extensions plus the Interface's local requirements/constraints.

The extension graph SHALL be acyclic.

```text
cycle -> FAIL: INTERFACE_EXTENSION_CYCLE
```

Multiple extension is permitted. Effective inherited requirements and constraint references compose conjunctively under ADR-0007.

Duplicate inherited requirement references collapse by canonical identity. Declaration order does not affect the effective contract.

## 8. InterfaceImplementation declaration

Entity Type conformance is represented by an explicit implementation declaration rather than a bare Interface name:

```yaml
entity_type: https://simulation-ontology.org/id/<entity-type-id>
interface: https://simulation-ontology.org/id/<interface-id>
property_mappings:
  - requirement: https://simulation-ontology.org/id/<required-property-id>
    concrete: https://simulation-ontology.org/id/<concrete-property-id>
relation_mappings:
  - requirement: https://simulation-ontology.org/id/<required-relation-id>
    concrete: https://simulation-ontology.org/id/<concrete-relation-id>
```

The declaration is type-level schema metadata, not a model-instance Entity and not a backend MappingRule.

## 9. Explicit mapping rule

Every effective Property/Relation requirement SHALL have exactly one implementation mapping in each direct implementation declaration.

Even when requirement and concrete definition use the same canonical identity, the normalized implementation declaration records the mapping explicitly:

```yaml
requirement: <id-X>
concrete: <id-X>
```

This prevents local-name matching, import-order inference, or implementation heuristics from becoming semantic authority.

Normalization failures:

```text
0 mappings for effective requirement
  -> INTERFACE_REQUIREMENT_MAPPING_MISSING

>1 mappings for effective requirement
  -> INTERFACE_REQUIREMENT_MAPPING_AMBIGUOUS

mapping for non-effective requirement
  -> INTERFACE_REQUIREMENT_MAPPING_UNKNOWN
```

Mappings are keyed by canonical requirement identity, not display/API/local names.

## 10. Mapping kind boundary

Property requirements map only to Property definitions. Relation requirements map only to Relation definitions.

```text
Property -> Relation mapping
Relation -> Property mapping
    -> FAIL: INTERFACE_REQUIREMENT_KIND_MISMATCH
```

Backend-local object names/handles are invalid concrete mapping targets. Backend realization remains Profile/BackendAdapter responsibility.

## 11. Semantic conformance after mapping

A structurally complete mapping is not automatically conforming.

After Interface extension closure and implementation mappings are resolved:

1. resolve required Property/Relation semantic definitions;
2. resolve concrete Property/Relation definitions;
3. validate kind compatibility;
4. apply requirement and Interface constraints conjunctively;
5. validate compatible concrete refinement under ADR-0007/0008/0019 and applicable family contracts.

A concrete relation/property MAY be more specialized but SHALL NOT weaken the Interface guarantee.

This proposal does not define a new generic Compatibility algorithm; accepted Type/Cardinality/Dimension/Value/Compatibility contracts are reused where applicable.

## 12. Direct versus effective `implements`

Only direct InterfaceImplementation declarations are serialized.

If Entity Type `E` directly implements child Interface `B` and `B extends A`, then `E` satisfies `A` through the effective Interface closure after all inherited requirements are mapped and validated.

A duplicate direct implementation of `A` is not required merely because `B extends A`.

Likewise, Entity `is_a` inheritance and Interface implementation remain separate axes. Whether an Entity subtype inherits an ancestor Entity Type's InterfaceImplementation declaration is not introduced as a new v0.1 rule by this proposal; existing Entity/Interface conformance must be explicit unless later canonical package validation establishes an accepted derived rule.

## 13. Proposed schema slice

After contract acceptance, add:

- `schema/interface-definition-v0.1.schema.json`
- `schema/interface-implementation-v0.1.schema.json`

The focused structural schemas use canonical reference strings and exact field closure.

No PropertyDefinition schema is invented here. Property and Relation requirements are canonical references whose provider definitions are resolved by the package/compiler environment. RelationDefinition already has partial machine representation; canonical PropertyDefinition serialization remains a later package/language consolidation dependency.

## 14. Required boundary cases

1. Interface canonical identity unresolved -> failure;
2. parent Interface unresolved -> failure;
3. extension cycle -> `INTERFACE_EXTENSION_CYCLE`;
4. diamond extension with same canonical requirement -> one effective requirement;
5. same textual local name but different canonical requirement IDs -> distinct requirements;
6. missing mapping -> `INTERFACE_REQUIREMENT_MAPPING_MISSING`;
7. duplicate mapping -> `INTERFACE_REQUIREMENT_MAPPING_AMBIGUOUS`;
8. mapping to requirement not in effective contract -> `INTERFACE_REQUIREMENT_MAPPING_UNKNOWN`;
9. Property requirement mapped to Relation -> kind mismatch;
10. Relation requirement mapped to Property -> kind mismatch;
11. identical canonical requirement/concrete ID still has explicit mapping;
12. backend-local handle used as concrete target -> rejected by identity/kind resolution;
13. inherited constraints compose conjunctively and preserve provenance;
14. declaration order does not affect effective contract;
15. child Interface implementation satisfies parent Interface through extension closure without duplicate parent declaration;
16. model-instance zero relation edges do not by themselves violate the existence of a relation capability unless Cardinality requires them.

## 15. Non-goals / deferred

- Interface Action/Operation requirements;
- optional Interface members;
- backend interface/class hierarchy mirroring;
- implicit name-based mapping;
- runtime duck typing;
- generic method signatures;
- production SDK generation;
- final PropertyDefinition serialization;
- final package storage layout for Interface definitions and implementation declarations;
- a universal global validation-state envelope.

## 16. Research verdict

**Ready for independent contract Validation.**
