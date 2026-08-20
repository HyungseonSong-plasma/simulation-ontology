# SOL v0.1 Interface Serialization v0.2 Focused Review

**Role:** Validation  
**Date:** 2026-08-20  
**Input:** `docs/research/sol-v0.1-interface-serialization-proposal-v0.2.md`  
**Scope:** IFS-01 through IFS-03 closure plus regression/counterexample review  
**Verdict:** Revise

## Closure of prior findings

### IFS-01 — Constraint target/context

**Substantively resolved.** `ConstraintApplication` now identifies `interface`, `property_requirement`, or `relation_requirement`, and Property/Relation target forms bind an explicit canonical requirement identity. This closes the original two-Property/one-Dimension ambiguity.

A remaining implementation-contract issue is split out below as IFS-04.

### IFS-02 — overlapping InterfaceImplementation composition

**Resolved.** The direct declaration key `(entity_type_id, interface_id)` is unique. Effective overlapping mappings are keyed by `(requirement_kind, canonical_requirement_id)` and must converge to one concrete canonical identity. Distinct targets fail with `INTERFACE_EFFECTIVE_MAPPING_CONFLICT`. No declaration/import priority is used.

The v0.1 restriction against proving refinement between different concrete canonical IDs is conservative but deterministic and does not weaken an accepted guarantee.

### IFS-03 — Entity subtype inherited Interface guarantee

**Resolved.** Effective Interfaces include direct declarations from the Entity `is_a` ancestor closure plus Interface extension closure. Ancestor mappings and Interface constraints remain effective for subtypes. Local overlap is accepted only when mapping convergence holds, so specialization cannot silently remove or reinterpret an inherited guarantee.

## New finding

### IFS-04 — ConstraintDefinition target-admissibility contract is not machine-deterministic

**Classification:** Contract serialization completeness defect  
**Architecture impact:** none; Interface/Constraint taxonomy is unchanged

v0.2 requires:

```text
constraint target kind incompatible with referenced ConstraintDefinition/family
-> INTERFACE_CONSTRAINT_TARGET_INCOMPATIBLE
```

but it does not define the normative contract by which an independent validator determines whether a reified ConstraintDefinition accepts:

```text
interface
property_requirement
relation_requirement
```

The phrase “family/definition that cannot apply to the supplied target” is insufficient because accepted Constraint-family payloads do not all carry one common target-kind field. Two validators could therefore disagree on whether a given reified definition is admissible at a Property, Relation, or whole-Interface target.

Required focused remediation:

- every reified ConstraintDefinition used by an Interface ConstraintApplication must expose a deterministic Interface-application target contract;
- the contract should be a finite set over exactly `interface | property_requirement | relation_requirement`;
- the Interface validator must reject a target kind not in that set with `INTERFACE_CONSTRAINT_TARGET_INCOMPATIBLE`;
- this metadata is application/context metadata around the existing six-family semantic payload and must not create a seventh family or alter family intersection algebra;
- exact storage location may remain part of later canonical ConstraintDefinition/package serialization, but the semantic field/lookup rule itself must be normative now.

## Regression review

No new ambiguity was found in:

- Interface versus SpatialInterface identity;
- multiple Interface extension and cycle rejection;
- canonical requirement identity and set semantics;
- schema capability versus model-instance cardinality;
- explicit semantic implementation mappings;
- direct declaration uniqueness;
- overlapping-mapping convergence;
- redundant parent/child Interface declarations;
- Entity taxonomic inheritance of Interface guarantees;
- inherited ConstraintApplication preservation;
- backend/runtime exclusion.

## Verdict

**Revise — IFS-04 only.**

Return only IFS-04 to Research. IFS-01 through IFS-03 and the previously accepted directions should remain closed.
