# SOL v0.1 InlineValueDefinition Payload Boundary Proposal v0.1

**Status:** Focused Research proposal  
**Date:** 2026-08-20  
**Input finding:** VDI-01  
**Scope:** machine-readable normalized InlineValueDefinition payload boundary only

## 1. Problem

ADR-0026 defines the focused mechanism tags:

```text
literal | expression | function | tabular
```

but it intentionally does not define a universal portable expression language, function language, interpolation algorithm, table source model, or evaluator runtime.

Cross-backend evidence demonstrates that these payloads differ materially:

- MOOSE ParsedFunction uses expression text plus optional bound symbols and built-in space/time arguments;
- MOOSE PiecewiseLinear may use x/y arrays, pair data, or external files;
- COMSOL Analytic separates expression and formal arguments;
- COMSOL Interpolation may use local tables, files, result tables, or another function.

Therefore SOL v0.1 should not invent a common executable DSL merely to fill a JSON Schema.

## 2. Decision candidate — Core-native literal, format-owned nonliteral payload

### 2.1 Literal

Literal evaluation is fully representable by Core `Value` semantics:

```yaml
mechanism: literal
value: <normalized Value>
```

No external payload format is required.

### 2.2 Expression / Function / Tabular

For nonliteral inline mechanisms, Core normalizes only a deterministic envelope:

```yaml
mechanism: expression | function | tabular
format: <canonical schema/capability identity>
payload: <JSON value governed by that format>
```

`format` is a stable schema/capability identity resolved under SOL identity/package rules. It identifies the semantic contract that defines the payload syntax and evaluation meaning.

Examples of future format providers might include a SOL expression extension, a domain function contract, or a tabular-data format. Backend-native formats MAY appear only in import/intermediate layers unless explicitly exposed through an accepted extension contract; a backend-local object handle is not a valid Core format identity.

## 3. Core authority boundary

Core structural validation owns only:

1. exact mechanism tag;
2. literal versus nonliteral branch;
3. required `format` for nonliteral mechanisms;
4. presence of one `payload` JSON value for nonliteral mechanisms;
5. absence of graph identity/dependency fields in InlineValueDefinition itself.

The selected `format` provider owns:

- payload field names;
- argument/local-symbol semantics;
- expression/function syntax;
- table layout/interpolation semantics;
- mechanism-specific structural and semantic validation;
- evaluation capability.

Core SHALL NOT infer these semantics from payload shape.

## 4. Dependency rule preservation

The ADR-0026 compiler boundary remains authoritative.

A normalized InlineValueDefinition SHALL contain no canonical SOL semantic dependency references as Core graph fields.

Core validators SHALL NOT scan `payload` text/object members to infer `depends_on` edges.

If authoring/compiler resolution identifies a canonical SOL semantic dependency, normalization SHALL choose the reified ValueDefinition graph form before emitting normalized Core data.

A format provider MAY expose local argument/binding syntax internally, but any binding resolved to a SOL semantic Entity is promoted by the compiler into the reified graph representation rather than hidden inside the normalized inline payload.

## 5. Format resolution state

Structural Core validity and mechanism-format semantic evaluability are separate axes.

- missing/invalid `format` field -> structural `FAIL`;
- valid canonical format identity but unavailable provider during a validation that requires mechanism interpretation -> semantic `INDETERMINATE`;
- available provider reports payload invalid -> `FAIL`;
- available provider validates payload -> mechanism-specific `PASS` for that slice.

This uses the ADR-0024 common state slice where applicable and does not make package/plugin installation itself a backend runtime concern.

## 6. Why not a permissive Core payload schema

`payload` is intentionally format-owned, not silently permissive Core semantics. The semantic meaning is determined by the explicit `format` identity.

Thus:

```text
opaque payload without format     -> invalid
payload + resolved format contract -> deterministic ownership boundary
```

This is analogous to SOL's external metrology boundary: Core preserves the reference/contract boundary without owning the entire external vocabulary or evaluator.

## 7. Normalized schema candidate

```text
InlineValueDefinition
  = LiteralDefinition
  | FormattedNonliteralDefinition

LiteralDefinition = {
  mechanism: "literal",
  value: Value
}

FormattedNonliteralDefinition = {
  mechanism: "expression" | "function" | "tabular",
  format: canonical schema/capability identity,
  payload: JSON value
}
```

No `id`, `semantic_dependencies`, or `depends_on` fields exist in either branch.

## 8. Boundary cases

1. literal + Value -> valid Core-native definition;
2. literal + format/payload -> structural failure;
3. expression + format + payload -> structurally valid;
4. expression without format -> structural failure;
5. function/tabular with backend-local handle as `format` -> canonical resolution failure;
6. valid format but provider unavailable when interpretation is required -> INDETERMINATE;
7. format provider rejects payload -> FAIL;
8. payload contains string token matching an Entity name but compiler did not resolve a semantic binding -> Core does not invent dependency;
9. compiler resolves one payload binding to a SOL Entity -> normalized inline form forbidden; reified ValueDefinition required;
10. inline form containing `id` or `depends_on` -> structural failure.

## 9. Scope deferred

- standard SOL expression DSL;
- standard SOL function DSL;
- canonical interpolation/tabular semantics;
- provider discovery mechanism beyond normal package/schema resolution;
- execution/runtime API for mechanism evaluators.

## 10. Research verdict

**Ready for focused independent Validation.**
