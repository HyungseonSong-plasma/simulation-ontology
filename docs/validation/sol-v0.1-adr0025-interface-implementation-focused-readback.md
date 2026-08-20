# SOL v0.1 ADR-0025 Interface Implementation Focused Readback

**Role:** Validation  
**Date:** 2026-08-20  
**Scope:** ADR-0025 InterfaceDefinition/InterfaceImplementation schemas and semantic helper/tests  
**Verdict:** Revise

## Inputs

- `docs/decisions/0025-interface-serialization-and-inherited-capability-conformance.md`
- `schema/interface-definition-v0.1.schema.json`
- `schema/interface-implementation-v0.1.schema.json`
- `tests/interface_semantics.py`
- `tests/test_interface_schema.py`

## Accepted implementation areas

The readback matches ADR-0025 for:

- InterfaceDefinition being separate from Entity Type;
- closed ConstraintApplication target shapes;
- direct Interface extension and cycle detection;
- canonical requirement set collapse;
- explicit direct implementation mappings;
- unique direct `(entity_type, interface)` keys;
- mapping completeness/ambiguity/unknown checks;
- exact canonical overlapping-mapping convergence;
- Entity ancestor declarations contributing Interface guarantees;
- declaration-order independence;
- backend metadata exclusion.

## Findings

### IFIV-01 — requirement/concrete provider-definition resolution is incomplete

**Classification:** Validation-tooling defect

ADR-0025 validation order begins by resolving canonical Property/Relation references. The helper currently categorizes Interface requirements from their list location but does not verify that the canonical requirement ID resolves to an actual provider definition. For concrete mapping targets, `definition_kinds.get(concrete) != expected_kind` also collapses “unresolved concrete definition” into `INTERFACE_REQUIREMENT_KIND_MISMATCH`.

Counterexamples:

1. `property_requirements: [{property: "prop:missing"}]` can enter the effective contract even when `prop:missing` is absent from the resolved definition registry.
2. `prop:req -> prop:missing` reports kind mismatch instead of a canonical-reference resolution failure.

Required remediation:

- validate both requirement and concrete canonical IDs against the resolved definition registry before kind comparison;
- missing requirement/concrete definitions must produce explicit reference-resolution diagnostics rather than kind mismatch;
- wrong resolved kind remains `INTERFACE_REQUIREMENT_KIND_MISMATCH`.

Final PropertyDefinition serialization remains deferred; the focused helper may use the existing design-stage `definition_kinds` registry fixture as the resolution boundary.

### IFIV-02 — Entity ancestor closure accepts an unresolved parent as a root

**Classification:** Validation-tooling defect

`_entity_ancestors` currently follows `entity_parent.get(current)`. If `entity:Child -> entity:MissingParent` and the missing parent is absent from the registry/map, `.get()` returns `None` and the helper treats the missing Entity Type as a valid root.

ADR-0025 requires the resolved Entity taxonomy before Interface conformance.

Required remediation:

- every Entity ID visited in the ancestor closure must resolve in the supplied taxonomy registry;
- missing Entity Type/parent -> explicit Entity reference-resolution failure;
- cycle detection remains unchanged.

## Regression note

No contract change is required. No backend runtime/install/license evidence is relevant.

## Verdict

**Revise — IFIV-01 and IFIV-02 only.**

Return only these tooling findings to Research/implementation. After focused remediation, re-read the helper/tests for resolution, mapping convergence, inheritance, and order-independence regression.
