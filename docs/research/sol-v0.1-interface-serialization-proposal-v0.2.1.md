# SOL v0.1 Interface Serialization Proposal v0.2.1

**Status:** Final focused Research revision  
**Date:** 2026-08-20  
**Base:** v0.2  
**Revision input:** IFS-04 from `docs/validation/sol-v0.1-interface-serialization-v0.2-focused-review.md`

## 1. Scope

All v0.2 contracts remain unchanged. IFS-01 through IFS-03 stay closed.

This revision adds only the deterministic target-admissibility contract required when a reified ConstraintDefinition is used by an Interface `ConstraintApplication`.

## 2. ConstraintDefinition Interface-application metadata

A reified ConstraintDefinition that is eligible for use in an Interface `ConstraintApplication` SHALL expose:

```yaml
interface_application_target_kinds:
  - property_requirement
```

The value is a nonempty mathematical set drawn only from:

```text
interface
property_requirement
relation_requirement
```

This is metadata on the reusable ConstraintDefinition contract. It is not part of the normalized Cardinality/Type/Value/Dimension/Compatibility/Conditional semantic payload and does not affect primitive-family intersection identity.

Examples:

```yaml
# reusable Dimension restriction intended for a Property requirement
interface_application_target_kinds:
  - property_requirement
```

```yaml
# reusable invariant that may be attached to a Relation requirement or whole Interface
interface_application_target_kinds:
  - relation_requirement
  - interface
```

Array order is non-semantic and duplicate target kinds collapse under set semantics.

## 3. Resolution rule

During Interface validation:

1. resolve the `ConstraintApplication.constraint` reference to exactly one active reified ConstraintDefinition;
2. read that definition's `interface_application_target_kinds` contract;
3. normalize the `ConstraintApplication.target.kind`;
4. test membership in the allowed target-kind set.

Outcomes:

```text
ConstraintDefinition unresolved/ambiguous
    -> existing canonical-definition resolution failure

interface_application_target_kinds missing
    -> INTERFACE_CONSTRAINT_TARGET_CONTRACT_MISSING

empty or containing a value outside the closed three-kind vocabulary
    -> INTERFACE_CONSTRAINT_TARGET_CONTRACT_INVALID

target.kind not in allowed set
    -> INTERFACE_CONSTRAINT_TARGET_INCOMPATIBLE

target.kind in allowed set
    -> continue ordinary Interface target-resolution and Constraint-family validation
```

No default target kind is inferred from Constraint family name, declaration location, payload shape, backend metadata, or implementation order.

## 4. Relation to Constraint families

The six accepted Constraint families remain the only primitive semantic families.

`interface_application_target_kinds` answers only:

> At which Interface contract context(s) may this reusable ConstraintDefinition be applied?

It does not answer whether the semantic constraint itself is satisfied. After target admissibility and requirement mapping resolve, the referenced family contract performs its normal semantic validation.

A package that publishes a ConstraintDefinition is responsible for declaring a target-kind set consistent with that definition's semantic contract. A malformed or internally contradictory reusable definition is a package/schema validation defect, not a backend representability result.

## 5. Serialization boundary

The exact final package layout of `ConstraintDefinition` remains deferred to canonical package integration, as already allowed by ADR-0007/0018.

However, for any ConstraintDefinition referenced from the Interface schema slice, the **semantic existence and lookup rule** for `interface_application_target_kinds` is normative in v0.1. A temporary design-stage registry/helper may represent it as a field on the resolved ConstraintDefinition fixture.

## 6. Focused boundary cases

1. Property-targeted application + allowed `{property_requirement}` -> admissible;
2. Relation-targeted application + allowed `{property_requirement}` -> `INTERFACE_CONSTRAINT_TARGET_INCOMPATIBLE`;
3. whole-Interface application + allowed `{interface, relation_requirement}` -> admissible;
4. missing target-kind contract -> `INTERFACE_CONSTRAINT_TARGET_CONTRACT_MISSING`;
5. empty target-kind set -> `INTERFACE_CONSTRAINT_TARGET_CONTRACT_INVALID`;
6. unknown target-kind token -> `INTERFACE_CONSTRAINT_TARGET_CONTRACT_INVALID`;
7. duplicate/list-order permutations -> same normalized allowed set;
8. backend/runtime metadata does not alter admissibility.

## 7. Finding closure

| Finding | Status |
|---|---|
| IFS-01 | Resolved in v0.2 |
| IFS-02 | Resolved in v0.2 |
| IFS-03 | Resolved in v0.2 |
| IFS-04 | Resolved in v0.2.1 |

**Contract-level unresolved proposed:** none.  
**Regression proposed:** none.

## 8. Research verdict

**Ready for final focused contract Validation.**
