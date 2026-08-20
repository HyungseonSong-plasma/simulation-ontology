# SOL v0.1 ADR-0016 Transcription Readback Validation

**Role:** Validation  
**Date:** 2026-08-20

## Inputs

- `docs/decisions/0016-model-component-membership-and-condition-target-semantics.md`
- `ontology/core/relations.yaml`
- `ontology/core/constraints.yaml`
- `docs/architecture.md`
- `docs/ontology-language.md`

The Research drafting process and prior conclusions were excluded from the validation basis.

## Checks

### V16-01 — `includes_component` authority

**PASS.**

The machine-readable relation uses one `allowed_pairs` matrix and does not define a normative `range: Entity`. Notes explicitly state that allowed pairs are authoritative and derived summaries cannot broaden them.

### V16-02 — direct/non-owning/non-transitive semantics

**PASS.**

ADR, relation registry, constraints, and architecture all preserve:

```text
source cardinality 0..*
non-owning
unordered
transitive = false
```

No document introduces transitive inference or deletion/lifecycle ownership.

### V16-03 — subtype-aware endpoint matching

**PASS.**

Both `includes_component` and `applied_to` use canonical equal-or-subtype matching, and the constraint registry preserves the rule that type inconsistency fails before relation matching.

No backend inheritance or declaration-order rule is introduced.

### V16-04 — `applied_to` repair

**PASS.**

The machine-readable domain is exactly:

```text
BoundaryCondition
InitialCondition
Source
Load
```

with target family:

```text
Field
Equation
Scope
```

and source cardinality `1..*`. `ConditionModel` is explicitly excluded as a direct source.

### V16-05 — no implicit taxonomy regression

**PASS.**

Human-readable architecture now names model-tree edges as `includes_component`; it does not convert those edges into `is_a`. ADR-0014 remains intact.

### V16-06 — serialization authority boundary

**PASS with consolidation note.**

Fields such as `allowed_pairs` and `endpoint_matching` are current machine-readable transcription forms, while the files still mark final structural-schema enforcement/field spelling as consolidation work. This does not create a second semantic authority because ADR-0016 remains normative.

## Verdict

| Dimension | Verdict |
|---|---|
| ADR-0016 contract preservation | PASS |
| Machine-readable transcription | PASS |
| Human-readable transcription | PASS |
| Architecture regression | None |
| Adapter/backend implementation dependency | None |

**Overall: Accept.**

## Next state

ADR-0016 cycle is complete. Operating Desk may proceed to the next design-stage consolidation question. The next dependency-ordered task is the remaining Core relation cardinality/requiredness matrix, beginning with `represented_by`, `closed_by`, `parameterized_by`, `defined_on`, `discretized_by`, `solved_by`, and `observed_by`, while preserving already accepted ADR-0015/0016 cardinalities.
