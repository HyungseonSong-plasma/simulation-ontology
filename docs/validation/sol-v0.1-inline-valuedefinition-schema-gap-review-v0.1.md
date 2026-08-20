# SOL v0.1 InlineValueDefinition Schema Gap Review v0.1

**Status:** Focused implementation-boundary Validation  
**Date:** 2026-08-20  
**Trigger:** ADR-0026 machine-readable transcription

## Verdict

**Revise before full InlineValueDefinition schema implementation.**

`DimensionVector`, `UnitReference`, and `Value` are sufficiently specified for direct structural schema transcription. `InlineValueDefinition` is not yet fully specified at payload-field level for `expression`, `function`, and `tabular` mechanisms.

## VDI-01 — mechanism tag is defined but nonliteral payload contract is not

ADR-0026 accepts the mechanism vocabulary:

```text
literal | expression | function | tabular
```

but only the literal form has an obvious closed Core payload (`Value`). Existing backend evidence shows that analytic/function/tabular representations have materially different argument, expression, source, interpolation, and evaluator metadata. Inventing a common set of payload fields during implementation would create new semantics without Research/Validation.

A permissive unqualified `payload: {}` would also be unsafe if consumers interpret it as Core semantic truth.

### Required remediation

Define one of the following explicitly before implementation:

1. a closed Core payload contract for each mechanism; or
2. a closed Core envelope that treats nonliteral mechanism payload as explicitly external/mechanism-format-owned data, identified by a stable format/schema reference, with Core validators forbidden from inferring semantic dependencies or evaluation meaning from the opaque payload.

The second option better preserves the current v0.1 scope unless reference evidence justifies a general expression/function/tabular DSL.

## Evidence

- MOOSE ParsedFunction distinguishes expression text, local coordinates/time, and separately bound symbols; PiecewiseLinear supports inline pairs or external files.
- COMSOL Analytic functions distinguish expression and arguments, while Interpolation supports local tables, files, result tables, or functions.
- These differences demonstrate that a backend-neutral mechanism name does not by itself define a complete portable payload syntax.

## Classification

**Language/schema serialization gap**, not architecture rejection.

## Next State

```text
Current State: ADR-0026 accepted; partial schema transcription ready
Role Invoked: Validation
Finding: VDI-01
Verdict: Revise focused InlineValueDefinition payload boundary only
Next State: Research focused payload-envelope decision
```
