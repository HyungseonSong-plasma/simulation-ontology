# SOL v0.1 Type Constraint Schema Proposal v0.2

**Role:** Research  
**Date:** 2026-08-20  
**Revision scope:** TC-V3 and TC-V4 only

## 1. Preserved contract

This revision preserves:

- minimal payload `type + relation + target_type`;
- container/evidence-layer ownership of source/use-site context;
- authoring identifiers normalized to canonical relation/type identities;
- deterministic single-inheritance subtype intersection;
- Interface identity separate from Entity taxonomic Type;
- QRC qualifier separate from universal target Type restriction.

## 2. Semantic Type contributors and representability restrictions are separate axes

The semantic effective Type constraint is formed only from semantic contributors such as:

```text
inherited Entity semantic constraints
Interface semantic requirements
local/domain semantic constraints
active semantic Conditional consequents
```

These compose conjunctively and define upstream SOL semantic validity.

A Backend/Profile restriction MAY reuse the same normalized `type + relation + target_type` payload shape for implementation convenience, but its normalized evidence SHALL be classified on the **representability/applicability axis**, not inserted into the upstream semantic Type intersection.

Conceptually:

```text
SemanticEffectiveType
  = inherited
  ∩ Interface
  ∩ local/domain
  ∩ active semantic conditionals

Backend/ProfileRepresentabilityType
  = representability/applicability restrictions
```

A valid semantic target that violates only a backend/profile Type restriction remains semantically valid and receives the appropriate backend representability/applicability diagnostic rather than a Core/domain Schema Conflict.

### Example

```text
Semantic contract:
  relation target <= Field

Model instance:
  target = ElectricField <: Field

Selected backend Profile restriction:
  representable target <= TemperatureField
```

Expected:

```text
Semantic validation: PASS
Backend/Profile representability: unsupported/inapplicable according to mapping contract
```

not:

```text
Semantic Schema Conflict
```

The evaluation axis is supplied by the normalized evidence/contributor context, not added to the three-field semantic Type payload.

## 3. Canonical target-family narrowing

For a relation/use-site whose accepted target contract is a finite canonical family:

```text
R = {R1, R2, ... Rn}
```

a Type Constraint target type `T` is a valid narrowing iff:

```text
exists Ri in R:
  T = Ri OR T <: Ri
```

The validator uses canonical semantic identity and canonical taxonomic subtype closure.

`T` SHALL NOT widen the family or add a new target branch.

### ADR-0016 `applied_to`

Base target family:

```text
{Field, Equation, Scope}
```

Therefore:

```text
TemperatureField <: Field
```

is a valid `target_type`, while an unrelated `Material` is invalid.

## 4. Allowed-pair relation narrowing

For a relation such as ADR-0016 `includes_component`, the base target family depends on the contributing source/use-site context.

Validation proceeds deterministically:

1. resolve the contributing source context canonical Entity Type;
2. collect the authoritative ADR-0016 allowed pair(s) whose source type is equal to or a supertype accepted by the source-context subtype rule;
3. form the canonical allowed target family from those matching allowed pairs;
4. require the Type Constraint `target_type T` to satisfy:

```text
exists allowed target Ri:
  T = Ri OR T <: Ri
```

A Type Constraint does not create or mutate `includes_component` allowed pairs.

If no authoritative allowed source pair applies, the use-site is invalid before Type narrowing.

## 5. Type intersection is unchanged

For active semantic Type Constraints on the same use-site/relation:

```text
A = B -> A
A <: B -> A
B <: A -> B
otherwise -> empty Type intersection
```

Conflict classification remains based on activation/contributor context:

- intrinsically incompatible semantic declarations -> Schema Conflict;
- only jointly active conditionals create incompatibility -> Configuration Conflict;
- backend/profile-only incompatibility -> representability/applicability diagnostic, not upstream semantic conflict.

## 6. Authoring and normalized payload

Authoring:

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

The payload remains free of evaluation-axis/provenance fields. Those are NormalizedConstraintEvidence context under ADR-0018.

JSON Schema checks structure only. Canonical identity, Entity-Type-versus-Interface identity, relation target-family compatibility, and subtype intersection remain semantic validation.

## 7. Counterexample closure

### TC-V3-A — Profile restriction invalidates upstream semantics

Expected: prohibited evaluator design. Profile/backend Type restriction is representability/applicability evidence, not upstream semantic contributor.

### TC-V3-B — identical payload on different axes

Two evidence records may carry identical Type payloads while one is semantic and one is representability-only. Payload equality does not collapse their evaluation axes or provenance.

### TC-V4-A — union-family narrowing

```text
base targets = {Field, Equation, Scope}
T = TemperatureField <: Field
```

Expected: valid narrowing.

### TC-V4-B — unrelated target

```text
base targets = {Field, Equation, Scope}
T = Material
```

Expected: Schema Conflict / invalid Type Constraint declaration.

### TC-V4-C — includes_component new pair attempt

A source context `MaterialModel` with Type Constraint `target_type = BoundaryCondition` does not create permission for:

```text
MaterialModel includes_component BoundaryCondition
```

Expected: invalid because no ADR-0016 allowed pair supports it.

## 8. Research verdict

TC-V3 and TC-V4 are resolved without changing the three-field Type Constraint payload.

**Ready for focused final Validation.**
