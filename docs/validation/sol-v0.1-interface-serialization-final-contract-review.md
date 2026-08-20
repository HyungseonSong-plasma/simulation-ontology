# SOL v0.1 Interface Serialization Final Contract Review

**Role:** Validation  
**Date:** 2026-08-20  
**Inputs:** v0.2 + v0.2.1 focused Research revisions  
**Normative basis:** ADR-0007, ADR-0008, ADR-0009, ADR-0014 and accepted Constraint-family contracts  
**Verdict:** Accept

## Finding closure

### IFS-01 — Constraint target/context

**Resolved.** `ConstraintApplication` explicitly binds a reusable ConstraintDefinition to `interface`, `property_requirement`, or `relation_requirement`, with canonical requirement identity for requirement-scoped targets.

### IFS-02 — overlapping implementations

**Resolved.** Direct `(entity_type, interface)` declarations are unique. Every overlapping effective requirement is keyed by requirement kind + canonical identity and all concrete mapping contributors must converge to one canonical concrete identity. Divergence fails deterministically with `INTERFACE_EFFECTIVE_MAPPING_CONFLICT`.

### IFS-03 — Entity subtype inherited guarantees

**Resolved.** Effective Interface guarantees include declarations from the Entity `is_a` ancestor closure and Interface extension closure. Ancestor mappings and Interface ConstraintApplications remain effective for subtypes; a local overlap cannot weaken or reinterpret an inherited mapping.

### IFS-04 — ConstraintDefinition target admissibility

**Resolved.** A reified ConstraintDefinition used by Interface application exposes a nonempty finite `interface_application_target_kinds` set over exactly:

```text
interface
property_requirement
relation_requirement
```

Missing/invalid target contracts and incompatible applications have deterministic diagnostics. No family-name inference, backend metadata, or declaration-order heuristic is permitted.

## Regression review

No new contract ambiguity was found in:

- Interface versus SpatialInterface identity;
- Interface being a schema resource rather than an Entity Type;
- multiple Interface extension and cycle rejection;
- canonical requirement set semantics;
- schema capability versus model-instance cardinality;
- explicit semantic implementation mappings;
- redundant parent/child Interface implementation;
- declaration-order independence;
- inherited Interface constraints;
- six-family Constraint reuse without a seventh Interface-specific family;
- backend/runtime exclusion.

The v0.1 exact-canonical mapping-convergence rule is intentionally conservative until a later accepted Property/Relation refinement contract can prove safe remapping between distinct concrete identities.

## Verdict

**Accept.**

The Interface serialization contract is decision-ready for a focused ADR and design-stage schema/helper slice.

**Next State:** Decision -> ADR -> Interface definition/implementation schemas + focused semantic helper/tests -> independent implementation readback.
