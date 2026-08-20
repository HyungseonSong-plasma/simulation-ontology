# SOL v0.1 Interface Serialization Proposal v0.2

**Status:** Focused Research revision  
**Date:** 2026-08-20  
**Revision input:** IFS-01 through IFS-03 from `docs/validation/sol-v0.1-interface-serialization-independent-review-v0.1.md`  
**Base:** `docs/research/sol-v0.1-interface-serialization-proposal-v0.1.md`

## 1. Scope preservation

The following v0.1 directions remain unchanged and are not reopened:

- `Interface` is a language-level capability contract, not an Entity Type;
- `Interface` is distinct from spatial `SpatialInterface`;
- direct Interface extension may be multiple and composes conjunctively;
- extension cycles are invalid;
- backend implementation inheritance is non-normative;
- Property/Relation requirements use canonical semantic references;
- implementation mappings are explicit rather than name-inferred;
- backend-local handles are invalid mapping targets;
- schema capability existence is distinct from model-instance value/edge cardinality;
- Action/Operation requirements and optional Interface members remain deferred;
- final PropertyDefinition/package storage syntax remains a later consolidation concern.

This revision changes only Constraint application context, overlapping implementation composition, and inherited Entity-type Interface guarantees.

## 2. InterfaceDefinition

The normalized InterfaceDefinition remains a schema resource with canonical identity:

```yaml
id: https://simulation-ontology.org/id/<interface-id>
extends:
  - https://simulation-ontology.org/id/<parent-interface-id>
property_requirements:
  - property: https://simulation-ontology.org/id/<property-id>
relation_requirements:
  - relation: https://simulation-ontology.org/id/<relation-id>
constraint_applications:
  - constraint: https://simulation-ontology.org/id/<constraint-definition-id>
    target:
      kind: property_requirement
      requirement: https://simulation-ontology.org/id/<property-id>
```

Requirement arrays and `constraint_applications` have order-independent collection semantics after canonical resolution.

## 3. IFS-01 — explicit Interface Constraint application target

A bare ConstraintDefinition reference is replaced by an explicit **ConstraintApplication** wrapper.

```text
ConstraintApplication = {
  constraint: canonical ConstraintDefinition identity,
  target: InterfaceConstraintTarget
}
```

Supported v0.1 target forms are exactly:

```yaml
# whole-Interface semantic invariant
target:
  kind: interface

# applies to one effective Property requirement
target:
  kind: property_requirement
  requirement: <canonical-property-id>

# applies to one effective Relation requirement
target:
  kind: relation_requirement
  requirement: <canonical-relation-id>
```

Rules:

1. `property_requirement` target SHALL resolve to exactly one effective Property requirement of the Interface.
2. `relation_requirement` target SHALL resolve to exactly one effective Relation requirement of the Interface.
3. `interface` target has no `requirement` field.
4. A target requirement not in the Interface effective contract is invalid:
   `INTERFACE_CONSTRAINT_TARGET_UNKNOWN`.
5. A Property/Relation target-kind mismatch is invalid:
   `INTERFACE_CONSTRAINT_TARGET_KIND_MISMATCH`.
6. The referenced ConstraintDefinition SHALL be context-complete apart from this explicit applicability target. Any additional operands/references required by its family (for example Compatibility left/right or Conditional EvaluationReferences) remain part of the accepted ConstraintDefinition/family contract and are not guessed by Interface serialization.
7. The target kind SHALL be semantically admissible for the referenced ConstraintDefinition/family. A family/definition that cannot apply to the supplied target produces:
   `INTERFACE_CONSTRAINT_TARGET_INCOMPATIBLE`.

This wrapper supplies applicability/provenance context; it does not create a new Constraint family or graph-query language.

### Example

```yaml
property_requirements:
  - property: thermal:temperature
  - property: thermal:emissivity
constraint_applications:
  - constraint: thermal:temperature-dimension-theta
    target:
      kind: property_requirement
      requirement: thermal:temperature
```

Two validators now identify the same constrained requirement.

## 4. Interface extension closure

Only direct `extends` edges are serialized.

For Interface `I`:

```text
EffectiveInterfaceContract(I)
  = local requirements/applications
    ∪ transitive contracts of direct parents
```

The extension graph SHALL be acyclic:

```text
cycle -> INTERFACE_EXTENSION_CYCLE
```

Canonical duplicate Property/Relation requirements collapse by `(requirement_kind, canonical_requirement_id)`.

Canonical duplicate ConstraintApplications collapse only when both the canonical ConstraintDefinition identity and normalized target are identical. Otherwise both applications remain conjunctive contributors.

Contributor provenance from all extension paths SHALL be retained for diagnostics even when semantic duplicates collapse.

## 5. InterfaceImplementation direct declaration

The direct serialization remains:

```yaml
entity_type: <canonical-entity-type-id>
interface: <canonical-interface-id>
property_mappings:
  - requirement: <canonical-required-property-id>
    concrete: <canonical-concrete-property-id>
relation_mappings:
  - requirement: <canonical-required-relation-id>
    concrete: <canonical-concrete-relation-id>
```

A direct declaration is keyed by:

```text
DirectImplementationKey = (entity_type_id, interface_id)
```

For one package-resolution environment there SHALL be at most one direct declaration for a DirectImplementationKey.

```text
>1 -> INTERFACE_IMPLEMENTATION_DUPLICATE_DIRECT
```

This uniqueness rule is independent of declaration order and file location.

## 6. Direct implementation completeness

For every direct InterfaceImplementation declaration, every effective Property/Relation requirement of that Interface closure SHALL have exactly one mapping in that declaration.

```text
0 -> INTERFACE_REQUIREMENT_MAPPING_MISSING
>1 -> INTERFACE_REQUIREMENT_MAPPING_AMBIGUOUS
non-effective requirement -> INTERFACE_REQUIREMENT_MAPPING_UNKNOWN
wrong Property/Relation kind -> INTERFACE_REQUIREMENT_KIND_MISMATCH
```

Even identity mappings remain explicit.

## 7. IFS-02 — overlapping effective Interface mappings

An Entity Type may directly implement multiple Interfaces whose effective contracts overlap, and Interface extension may cause the same requirement to arrive by multiple paths.

After every direct declaration is individually complete, build the Entity Type's effective mapping contributors by canonical requirement key:

```text
EffectiveRequirementKey
  = (requirement_kind, canonical_requirement_id)
```

For each key, collect every concrete canonical mapping target contributed by:

- all direct InterfaceImplementation declarations on the Entity Type;
- Interface extension closure of those declarations;
- inherited Entity-type Interface guarantees defined in Section 9.

v0.1 requires **mapping convergence**:

```text
all concrete canonical IDs for one EffectiveRequirementKey are identical
    -> one effective mapping; preserve contributor provenance

more than one distinct concrete canonical ID
    -> FAIL: INTERFACE_EFFECTIVE_MAPPING_CONFLICT
```

This applies equally to:

- sibling Interfaces sharing one canonical requirement;
- child + redundantly direct parent Interface declarations;
- Entity-taxonomy inherited implementation plus a local overlapping implementation.

No declaration priority, import priority, or most-recent declaration rule exists.

### Why exact canonical convergence in v0.1

A future Property/Relation refinement contract may justify proving that two different concrete definitions are compatible refinements of the same requirement. That proof system is not yet part of the frozen v0.1 language. Therefore v0.1 SHALL NOT guess equivalence or refinement between distinct concrete canonical identities.

This is deliberately stricter than a future version, but deterministic and monotonic.

## 8. Redundant ancestor Interface implementation

If `B extends A`, an Entity Type implementing `B` already has `A` in its effective Interface set.

A separate direct declaration for `A` is not required. If one is present, it is permitted only because it may provide redundant provenance; its overlapping requirement mappings MUST converge under Section 7.

Thus redundant declarations are semantically harmless when identical and fail deterministically when they disagree.

## 9. IFS-03 — Entity `is_a` inheritance of Interface guarantees

Entity taxonomic specialization SHALL preserve Interface guarantees from its Entity ancestors.

Let `Ancestors(E)` be the resolved Entity `is_a` closure including `E` itself. Entity taxonomy validation (including cycle/inconsistency checks) occurs before Interface conformance.

Define:

```text
DirectInterfaces(T)
  = Interfaces named by direct InterfaceImplementation declarations for Entity Type T

EffectiveInterfaces(E)
  = Interface-extension closure of
    union over T in Ancestors(E): DirectInterfaces(T)
```

Therefore an Entity subtype cannot silently lose an Interface guarantee held by an ancestor Entity Type.

### Mapping inheritance

Direct InterfaceImplementation declarations are stored only on the Entity Type that declares them. For conformance of subtype `E`, ancestor declarations contribute their concrete mappings unchanged into E's effective mapping contributor set.

If E adds a local declaration whose Interface closure overlaps an inherited requirement, Section 7 convergence applies.

In v0.1:

```text
inherited requirement R -> concrete X
local overlapping mapping R -> concrete X
    -> valid / deduplicate semantically

inherited requirement R -> concrete X
local overlapping mapping R -> concrete Y, X != Y
    -> INTERFACE_EFFECTIVE_MAPPING_CONFLICT
```

A subtype cannot remap an inherited Interface guarantee to a different concrete canonical identity until a later accepted Property/Relation refinement contract can prove such remapping preserves the inherited guarantee.

This is monotonic: specialization may add Interfaces/requirements/constraints but cannot remove or silently reinterpret inherited Interface guarantees.

## 10. Effective ConstraintApplications under Entity inheritance

ConstraintApplications belong to Interface contracts, not to implementation mapping declarations.

For Entity Type `E`, effective Interface ConstraintApplications are the conjunctive union of all Interface contracts in `EffectiveInterfaces(E)` after extension closure.

Duplicates collapse only by exact `(constraint_definition_id, normalized_target)` identity; contributor provenance is preserved.

Mappings do not retarget an Interface ConstraintApplication by name. A target expressed as a Property/Relation requirement is resolved through the Entity Type's one converged effective mapping for that requirement, then the referenced Constraint family validates the concrete definition/refinement.

## 11. Validation order

A deterministic v0.1 Interface conformance pass uses:

1. resolve canonical Interface, Entity, Property, Relation, and ConstraintDefinition references;
2. validate Entity `is_a` graph required for ancestor closure;
3. validate Interface extension graph and reject cycles;
4. compute effective Interface contracts and effective requirement sets;
5. validate uniqueness of direct `(entity_type, interface)` declarations;
6. validate each direct declaration is complete for its Interface closure;
7. collect direct + inherited mapping contributors for the Entity Type;
8. require exact canonical convergence for every EffectiveRequirementKey;
9. resolve ConstraintApplication targets through effective requirement keys and converged mappings;
10. validate mapped concrete Property/Relation definitions satisfy the Interface requirements and all effective ConstraintApplications conjunctively under accepted Constraint-family contracts.

No backend adapter/runtime state participates.

## 12. Required focused boundary cases

### IFS-01

1. two Property requirements + one Dimension ConstraintApplication explicitly targets one Property -> deterministic target;
2. target requirement absent from effective Interface -> `INTERFACE_CONSTRAINT_TARGET_UNKNOWN`;
3. Property target declared as relation target -> `INTERFACE_CONSTRAINT_TARGET_KIND_MISMATCH`;
4. whole-Interface target carrying `requirement` field -> structural failure;
5. ConstraintDefinition incompatible with target kind -> `INTERFACE_CONSTRAINT_TARGET_INCOMPATIBLE`.

### IFS-02

6. two direct declarations with same `(entity_type, interface)` -> `INTERFACE_IMPLEMENTATION_DUPLICATE_DIRECT`;
7. sibling Interfaces share requirement R and both map R -> X -> one effective mapping;
8. sibling Interfaces share R but map X vs Y -> `INTERFACE_EFFECTIVE_MAPPING_CONFLICT`;
9. `B extends A`, entity directly implements B and redundantly A with same mappings -> valid;
10. same case with conflicting mapping -> effective mapping conflict;
11. declaration/import order permutation -> same result.

### IFS-03

12. Parent Entity implements A; Child `is_a` Parent with no local declaration -> Child effective interfaces include A;
13. Child adds B while Parent supplies A -> both guarantees effective;
14. Child overlapping local mapping agrees with inherited mapping -> valid;
15. Child overlapping local mapping uses distinct concrete canonical ID -> effective mapping conflict;
16. inherited Interface constraint remains active for Child;
17. backend inheritance metadata does not affect Entity ancestor or Interface closure.

## 13. Schema slice after acceptance

If focused Validation accepts this contract, add:

- `schema/interface-definition-v0.1.schema.json`
- `schema/interface-implementation-v0.1.schema.json`

plus a semantic helper/test for:

- extension closure/cycle detection;
- ConstraintApplication target resolution;
- direct declaration uniqueness;
- effective mapping convergence;
- Entity-taxonomy inheritance of Interface guarantees;
- deterministic conformance under declaration-order permutation.

Final package location remains deferred to canonical package integration.

## 14. Finding closure matrix

| Finding | Proposed status | Resolution |
|---|---|---|
| IFS-01 constraint target/context | Resolved | explicit ConstraintApplication target: interface/property_requirement/relation_requirement |
| IFS-02 overlapping implementations | Resolved | unique direct key + exact canonical effective-mapping convergence |
| IFS-03 Entity subtype inherited guarantees | Resolved | effective Interfaces include Entity `is_a` ancestor declarations; inherited mappings/constraints preserved |

**Contract-level unresolved proposed:** none.  
**Regression proposed:** none.

## 15. Research verdict

**Ready for focused independent Validation.**
