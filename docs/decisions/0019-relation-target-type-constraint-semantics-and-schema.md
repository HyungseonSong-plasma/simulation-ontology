# ADR-0019: Relation-Target Type Constraint Semantics and Schema

**Status:** Accepted  
**Date:** 2026-08-20  
**Supplements:** ADR-0007, ADR-0008, ADR-0016, ADR-0018

## Context

SOL v0.1 accepts a `Type` Constraint family, but the machine-readable payload had not been fixed. Existing relation and constraint studies repeatedly require context-specific narrowing such as:

```text
BoundaryCondition target -> Field
TemperatureBoundaryCondition target -> TemperatureField
```

without overriding the RelationDefinition, introducing multiple Entity inheritance, or creating a universal graph-path language.

Independent Validation accepted a focused relation-target Type payload after clarifying backend/Profile representability separation and deterministic narrowing for union/allowed-pair relation contracts.

## Decision

### 1. Minimal v0.1 Type payload

For the current v0.1 evidenced use-case, a Type Constraint narrows the allowed target Entity Type of one semantic Relation at a contributing context/use-site.

Authoring payload:

```yaml
type: type
relation: <resolvable relation-id>
target_type: <resolvable Entity Type id>
```

Normalized semantic payload:

```yaml
type: type
relation: <canonical relation-id>
target_type: <canonical Entity Type id>
```

The containing Entity/Interface/local declaration plus normalized evidence identifies the constrained source/use-site. This ADR does not introduce a universal `subject` or graph-path field.

### 2. Taxonomic Entity Type axis only

`target_type` denotes an Entity taxonomic Type, not an Interface identifier.

Entity taxonomic type and Interface capability composition remain orthogonal under ADR-0008.

A Type Constraint SHALL NOT infer synthetic multiple inheritance from implemented Interfaces.

### 3. Base relation compatibility

A Type Constraint narrows but never broadens the underlying relation target contract.

For a finite canonical target family:

```text
R = {R1, R2, ... Rn}
```

constraint target type `T` is a valid narrowing iff:

```text
exists Ri in R:
  T = Ri OR T <: Ri
```

Canonical semantic identities and the canonical Entity subtype closure are used.

For a simple RelationDefinition range, the family contains that range type.

For ADR-0016 `includes_component`, the validator first resolves the authoritative allowed pair(s) applicable to the contributing source context, derives the allowed target family, and then applies the same narrowing rule. A Type Constraint cannot create a new allowed pair.

### 4. Deterministic Type intersection

For active semantic Type Constraints on the same semantic use-site/relation:

```text
A = B   -> A
A <: B  -> A
B <: A  -> B
otherwise -> empty Type-axis intersection
```

An empty intersection is classified according to activation context:

- intrinsically incompatible semantic declarations -> Schema Conflict;
- only jointly active Conditional constraints cause incompatibility -> Configuration Conflict.

Declaration order, profile priority, or last-write-wins SHALL NOT resolve the conflict.

### 5. Semantic and backend/Profile evaluation axes are separate

Upstream semantic Type constraints may be contributed by:

```text
inherited Entity semantic contracts
Interface semantic requirements
local/domain semantic constraints
active semantic Conditional consequents
```

Backend/Profile representability/applicability restrictions MAY reuse the same normalized payload shape for tooling, but they SHALL NOT be inserted into the upstream semantic Type intersection.

A semantically valid target that violates only a selected backend/Profile restriction remains semantically valid and receives a representability/applicability diagnostic under the mapping contract.

The evaluation axis is NormalizedConstraintEvidence/context metadata, not part of the three-field Type payload.

### 6. QRC remains orthogonal

QRC `qualifier.target_type` selects the subset counted by Cardinality. It does not imply that every target of the relation must match that type.

A Type Constraint and QRC may coexist independently.

Example:

```text
Type Constraint: products targets <= Species
QRC: products contains at least 1 NegativeIonSpecies
NegativeIonSpecies <: Species
```

### 7. Authoring/normalization boundary

ADR-0018 applies:

```text
AuthoringConstraint
   -> identity normalization
   -> NormalizedConstraintPayload
   + contributor/context evidence
```

Authoring and normalized Type payloads share structural fields, but normalized identifiers are canonical/resolved. JSON Schema SHALL NOT claim to prove canonical identity, Entity-Type-versus-Interface identity, subtype closure, base-relation compatibility, or Type intersection.

### 8. Schema implementation slice

The immediate slice SHALL include:

- `constraint-type-authoring-v0.1.schema.json`;
- `constraint-type-normalized-v0.1.schema.json`;
- minimal semantic helper/tests for canonical identity boundary, subtype intersection, finite target-family compatibility, allowed-pair non-expansion, Interface rejection at semantic-validation level, and Profile evaluation-axis separation.

No backend runtime is required.

## Consequences

### Positive

- subtype/use-site target narrowing has a minimal machine-readable form;
- Entity taxonomy and Interface capabilities remain orthogonal;
- Profile/backend restrictions cannot silently invalidate upstream semantic truth;
- union/allowed-pair relation targets narrow deterministically;
- no generic path/query language is invented prematurely.

### Costs

- source/use-site context must be retained by the containing normalized evidence layer;
- semantic validation still requires canonical identity and subtype closure;
- other Type use-cases, if later demonstrated, may require focused extension rather than overloading this payload.

## Validation evidence

- `docs/research/sol-v0.1-type-constraint-schema-proposal-v0.1.md`
- `docs/validation/sol-v0.1-type-constraint-schema-independent-review-v0.1.md`
- `docs/research/sol-v0.1-type-constraint-schema-proposal-v0.2.md`
- `docs/validation/sol-v0.1-type-constraint-schema-focused-final-review-v0.2.md`

## Decision summary

SOL v0.1 uses `type + relation + target_type` as the focused relation-target Type Constraint payload, canonicalizes identities during normalization, composes Type restrictions by deterministic subtype intersection, keeps Interface and QRC semantics orthogonal, and separates backend/Profile representability restrictions from upstream semantic Type validity.
