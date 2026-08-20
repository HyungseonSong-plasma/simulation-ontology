# SOL v0.1 Model-Component Relations and `applied_to` — Independent Review v0.1

**Role:** Validation  
**Date:** 2026-08-20  
**Branch:** `validation/sol-v0.1-independent-review`

## Inputs

Validation used the remediation proposal plus accepted normative contracts only:

- `docs/research/sol-v0.1-model-component-relations-and-applied-to-remediation-v0.1.md`
- ADR-0007 Constraint architecture
- ADR-0008 inheritance/interface composition
- ADR-0014 explicit taxonomy / no implicit tree inheritance
- ADR-0015 Simulation/Model/Task relations
- current Core entity/relation registries

Research conclusions were not treated as authoritative.

## Evaluation contract

Test whether independent validators can decide the same model-instance edge validity without relying on implicit tree inheritance, declaration order, backend object trees, or unstated subtype behavior.

Verdict domain: `Accept / Revise / Reject / Blocked`.

## Findings

### MC-01 — Type matching for compatibility tables is underspecified

**Classification:** Architecture / language-contract defect in the proposal.  
**Verdict:** Revise.

The proposal defines exact allowed source→target names for `includes_component` and an explicit type union for `applied_to`, but does not state whether a domain-extension subtype is accepted through canonical subtype closure.

Counterexample:

```text
TemperatureField is_a Field
math-1 : MathematicalModel
T : TemperatureField
math-1 includes_component T
```

Validator A may accept because `TemperatureField <: Field`; Validator B may reject because `TemperatureField` is not literally listed. The same ambiguity exists for subtype instances of `BoundaryCondition`, `Source`, `Load`, `Equation`, and `Scope` under `applied_to`.

Required correction: define source and target compatibility by canonical semantic type identity plus the accepted subtype closure. An instance endpoint conforms when at least one consistent resolved type is equal to or a subtype of an allowed source/target type. Type inconsistency remains a prior validation failure. Declaration order or backend inheritance must not affect matching.

### MC-02 — `range: Entity` mixes language metatype and Core ontology type semantics

**Classification:** Language/schema-contract defect in the proposal.  
**Verdict:** Revise.

`Entity` is a SOL language construct/metatype, not a declared Core Entity Type in `ontology/core/entities.yaml`. The proposal uses:

```text
includes_component
  range: Entity
```

while existing Core relations use concrete semantic Entity Types. Independent implementations can therefore interpret this either as a wildcard over all ontology entities or as an unresolved range identifier.

Required correction: make the allowed-pair matrix normative and sufficient for endpoint validity, and treat any broad domain/range summary as derived documentation rather than an additional semantic authority; alternatively enumerate the canonical union explicitly. Do not introduce a new `Entity` Core supertype solely to support this relation.

### MC-03 — `applied_to` direct-target semantics and minimum cardinality are supported

**Classification:** Accepted contract evidence.  
**Verdict:** Resolved/Accept.

The explicit leaf-domain approach removes the old reliance on `ConditionModel` as an implicit superclass. The `1..*` source cardinality is semantically coherent for `BoundaryCondition`, `InitialCondition`, `Source`, and `Load`: each instance must constrain or force at least one field/equation/scope target. Backend-native selection IDs remain out of Core.

MOOSE boundary-condition contracts bind directly to a variable and one or more boundaries; COMSOL boundary/source features likewise carry explicit domain/boundary selection. These support direct condition/forcing-to-target semantics without requiring aggregate `ConditionModel -> applied_to`.

### MC-04 — `includes_component` non-owning/non-transitive semantics are acceptable

**Classification:** Accepted contract evidence.  
**Verdict:** Resolved/Accept.

The generic relation is sufficiently distinct from semantic relations such as `represented_by`, `closed_by`, `defined_on`, `discretized_by`, and `solved_by`. Non-owning, order-independent, direct-only membership avoids copying backend tree ownership into Core.

The proposal correctly keeps component requiredness orthogonal to structural membership.

## Final verdict

| Item | Verdict |
|---|---|
| Generic `includes_component` concept | Accept with revision |
| Non-owning / non-transitive / unordered semantics | Accept |
| Allowed-pair matrix | Revise — subtype matching must be normative |
| `range: Entity` | Revise — metatype/Core-type ambiguity |
| `applied_to` explicit leaf-domain repair | Accept with subtype clarification |
| `applied_to` 1..* | Accept |
| New superclass or one-use Interface required | No |
| Architecture freeze reopened broadly | No |

**Overall: Revise.**

## Required next state

Research SHALL revise only MC-01 and MC-02. Preserve the accepted `includes_component` semantics and the `applied_to` leaf-domain/1..* decision. After revision, perform a focused final contract review before ADR drafting.
