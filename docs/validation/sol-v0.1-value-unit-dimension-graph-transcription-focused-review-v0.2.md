# SOL v0.1 Value / Unit / PhysicalDimension Graph Transcription — Focused Review v0.2

**Status:** Focused independent Validation  
**Date:** 2026-08-20  
**Input:** `docs/research/sol-v0.1-value-unit-dimension-graph-transcription-proposal-v0.2.md`  
**Focus:** closure of VUD-01 through VUD-03 and regression check against accepted boundaries

## Verdict

**Accept**

## Finding closure

### VUD-01 — CLOSED

The revision maps unresolved semantic metrology evidence to `INDETERMINATE`, definite contradiction to `FAIL`, and resolved valid evidence to `PASS`, with `FAIL > INDETERMINATE > PASS` aggregation under ADR-0024. Backend runtime/license/release state remains outside this semantic verdict.

### VUD-02 — CLOSED

The inline-versus-reified decision now depends only on compiler/name-resolution output. Raw expression/function/tabular text is not scanned heuristically for SOL semantic dependencies. Canonically resolved semantic references require graph-preserving reification; local opaque mechanism variables do not.

### VUD-03 — CLOSED

A reified ValueDefinition is one resolved model-graph Entity instance. All `has_value_definition` and `depends_on` statements referring to that definition use the same resolved model-instance identity. Inline definitions have no graph identity and cannot participate as Relation endpoints.

## Regression review

No regression was found in the accepted boundaries:

- DimensionVector remains the Core machine-comparable physical-dimension representation.
- UnitReference remains external/metrology-resolvable rather than a Core unit catalogue.
- Value remains typed evaluated data by default.
- missing Unit does not imply dimensionless, SI, backend default, or semantic validity.
- affine semantics remain quantity/use-site context plus metrology-service concerns.
- evaluation mechanism remains orthogonal to reification.
- `ReferenceDefinition` remains excluded.
- backend runtime/license state remains outside this design-stage semantic contract.

## Decision readiness

The Value / Unit / PhysicalDimension graph-transcription contract is ready for ADR promotion and focused machine-readable transcription.

## State transition

```text
Current State: Value/Unit/Dimension proposal v0.2
Role Invoked: Validation
Input Artifact: proposal v0.2 + prior findings
Output Artifact: this focused review
Verdict: Accept
Next State: Decision -> ADR-0026 -> focused schema/machine transcription
```
