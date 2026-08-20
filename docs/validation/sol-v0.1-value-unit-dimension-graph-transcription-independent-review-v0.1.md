# SOL v0.1 Value / Unit / PhysicalDimension Graph Transcription — Independent Review v0.1

**Status:** Independent Validation  
**Date:** 2026-08-20  
**Input:** `docs/research/sol-v0.1-value-unit-dimension-graph-transcription-proposal-v0.1.md`  
**Normative basis:** ADR-0002, ADR-0003, ADR-0004, ADR-0005, ADR-0007, ADR-0009, ADR-0020, ADR-0021, ADR-0024  

## Verdict

**Revise**

The proposal's principal graph-boundary decisions are consistent with the accepted architecture:

- `PhysicalDimension` as canonical typed `DimensionVector`, not mandatory Core Entity;
- `Unit` as external/metrology `UnitReference`, not a Core unit catalogue;
- `Value` as typed evaluated data, not a Core Entity by default;
- `requires_dimension`, `has_dimension`, `expressed_in`, and `evaluates_to` not blindly transcribed as Entity-to-Entity Core Relations;
- reification of `ValueDefinition` only when graph identity/relation participation is semantically required;
- backend runtime/license state excluded from metrology semantic validity.

No architecture rejection is warranted. Three contract-determinism gaps remain.

## VUD-01 — unresolved unit/metrology state is not mapped to the common validation state

The proposal says missing required unit context or unavailable metrology resolution yields "incomplete semantic evaluation", but does not normatively state whether this is `INDETERMINATE`, `FAIL`, or another state.

ADR-0024 already establishes the common design-stage slice `FAIL > INDETERMINATE > PASS`. Two validators could therefore publish different outcomes for the same unresolved UnitReference/context.

**Required remediation:** for semantic validation, unresolved required metrology/unit-context evidence SHALL yield `INDETERMINATE` with a stable diagnostic; definite dimensional or quantity-specific incompatibility SHALL yield `FAIL`. Backend runtime/license absence remains outside this semantic state.

**Classification:** Architecture/contract serialization gap.

## VUD-02 — inline semantic-dependency detection is under-specified

The proposal correctly requires a reified ValueDefinition when evaluation depends on another independently identifiable semantic Entity. However, for expression/function/tabular authoring it does not define whether symbols embedded in mechanism payloads are parsed by the semantic validator, pre-resolved by a compiler, or treated as opaque local variables.

Example:

```text
expression: "2*pi*f"
```

One validator may infer `f` as a SOL semantic dependency and reject inline form; another may treat it as a local expression argument and accept it.

**Required remediation:** normalized inline ValueDefinition SHALL contain no canonical semantic dependency references. Name/expression parsing and semantic-name resolution belong to the authoring/compiler phase. Only compiler-resolved semantic dependency references participate in the reification decision. Opaque/local mechanism variables do not become SOL Relations by string inspection.

**Classification:** Language/compiler boundary gap.

## VUD-03 — reified ValueDefinition identity/link consistency is not explicit

The proposal introduces a reified `ValueDefinition` Entity and `has_value_definition` / `depends_on` Relations, but does not explicitly require all owner links and dependency edges to resolve to the same canonical model-instance ValueDefinition identity.

Without this, an implementation could duplicate the same definition payload under multiple anonymous/generated nodes or attach `depends_on` edges to a different definition instance than the owner references.

**Required remediation:** every reified ValueDefinition SHALL have one canonical model-instance identity in the resolved model graph; `has_value_definition` and `depends_on` graph statements SHALL reference that same identity. Inline definitions have no graph identity and cannot be targets/sources of semantic Relations.

**Classification:** Model-instance identity / graph-transcription gap.

## Accepted boundaries retained

The following were not found defective and SHOULD NOT be reopened in the revision:

1. DimensionVector equality and explicit DimensionOne.
2. No mandatory Core PhysicalDimension Entity.
3. UnitReference external/metrology boundary and no hard QUDT dependency.
4. Missing Unit does not imply dimensionless/SI/backend default.
5. Affine absolute-vs-difference semantics belong to semantic quantity/use-site context, not UnitReference identity alone.
6. Value is typed data by default.
7. Shape and evaluation mechanism remain orthogonal.
8. `ReferenceDefinition` remains excluded.
9. UnitReference only on numeric Value in the focused v0.1 slice.
10. Backend installation/license/runtime state does not participate in this design-stage semantic contract.

## Next State

```text
Current State: Value/Unit/Dimension transcription proposal v0.1 validated
Role Invoked: Validation
Input Artifact: proposal v0.1 + accepted ADRs
Output Artifact: this report
Verdict: Revise
Next State: Research revision limited to VUD-01..03
```
