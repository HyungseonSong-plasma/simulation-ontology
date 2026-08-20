# SOL v0.1 Model-Component Relations and `applied_to` — Focused Final Review v0.2

**Role:** Validation  
**Date:** 2026-08-20

## Scope

Re-review only MC-01 and MC-02 from the prior independent review against:

- `docs/research/sol-v0.1-model-component-relations-and-applied-to-remediation-v0.2.md`
- ADR-0007, ADR-0008, ADR-0014, ADR-0015.

Previously accepted relation direction, non-owning/non-transitive semantics, and `applied_to` requiredness were not reopened.

## MC-01 — subtype-aware endpoint matching

**Verdict: Resolved.**

The revised contract now defines endpoint matching by canonical semantic identity plus canonical subtype closure:

```text
matches(x,T) iff some consistent resolved type X of x satisfies X = T or X <: T
```

This closes the prior `TemperatureField <: Field` counterexample for both source and target endpoints. Multiple consistent typing has a deterministic existential rule; inconsistent typing fails before relation matching. Backend inheritance and declaration order are explicitly excluded.

Independent validators therefore receive the same endpoint-matching rule.

## MC-02 — `Entity` metatype / Core type ambiguity

**Verdict: Resolved.**

`range: Entity` is no longer normative. `includes_component` validity is determined only by the canonical allowed-pair matrix plus subtype-aware endpoint matching. Derived domain/range summaries cannot broaden the allowed pairs, and no Core supertype named `Entity` is introduced.

The counterexample:

```text
MaterialModel includes_component BoundaryCondition
```

is deterministically invalid even if a tooling summary happens to contain `BoundaryCondition` elsewhere.

## Regression check

No regression found in the previously accepted boundaries:

- `includes_component`: direct, non-transitive, non-owning, unordered, generic `0..*`;
- structural membership remains distinct from `represented_by`, `closed_by`, `parameterized_by`, `defined_on`, `discretized_by`, and `solved_by`;
- `applied_to` source family remains `BoundaryCondition | InitialCondition | Source | Load`;
- target family remains `Field | Equation | Scope`;
- each conforming condition/forcing instance requires `1..*` `applied_to` targets;
- no artificial `Condition` superclass or one-use Interface is required;
- backend installation/runtime evidence is not required for this design-stage contract.

## Final verdict

| Item | Verdict |
|---|---|
| MC-01 subtype matching | Resolved |
| MC-02 metatype ambiguity | Resolved |
| Regression | None |
| Architecture defect remaining in this scope | None |
| ADR drafting readiness | Yes |

**Overall: Accept.**

## Next state

Operating Desk may promote the accepted contract to a focused ADR and transcribe the relation/constraint semantics into the machine-readable Core. A readback Validation should confirm that serialization does not create a second authority or reintroduce implicit inheritance.
