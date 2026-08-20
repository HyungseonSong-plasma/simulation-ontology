# ADR-0027 — InlineValueDefinition Format-Provider Boundary

**Status:** Accepted  
**Date:** 2026-08-20  
**Scope:** SOL v0.1 normalized InlineValueDefinition representation

## Context

ADR-0026 accepted `literal | expression | function | tabular` as the focused ValueDefinition mechanism vocabulary but did not define one universal backend-neutral expression/function/tabular payload DSL. MOOSE, COMSOL, and Ansys expose materially different function/table payload structures.

Focused Research and independent Validation established that SOL Core should own the normalized envelope and dependency/reification boundary, while mechanism-specific nonliteral payload syntax belongs to an explicitly identified format provider.

## Decision

### 1. Literal is Core-native

```yaml
mechanism: literal
value: <normalized Value>
```

The literal branch requires no external format provider.

### 2. Nonliteral normalized envelope

Expression, function, and tabular inline definitions use:

```yaml
mechanism: expression | function | tabular
format: <canonical schema/capability identity>
payload: <format-owned normalized JSON value>
```

An unqualified opaque payload without `format` is invalid.

### 3. Format-provider authority

A conforming nonliteral format provider owns payload syntax, local argument semantics, mechanism-specific normalization, validation, and evaluation meaning.

It SHALL expose a deterministic normalization boundary conceptually equivalent to:

```text
normalize_inline_payload(authored_payload, semantic_resolution_context)
  -> normalized_local_payload
     + resolved_semantic_dependencies[]
```

The canonical dependency set is order-independent and deduplicated by resolved semantic identity.

### 4. Reification selection

If `resolved_semantic_dependencies` is nonempty, normalized inline form SHALL NOT be emitted. ADR-0026 reified `ValueDefinition` is required and dependencies are preserved as semantic Relations.

If the dependency set is empty, inline form remains eligible subject to all other ADR-0026 reification triggers.

Core validators SHALL NOT scan raw payload text/object members to invent dependencies.

### 5. Provider resolution states

- missing/invalid format identity -> `FAIL`;
- valid format identity but required provider unavailable -> `INDETERMINATE`;
- definite ambiguous/invalid SOL semantic reference -> `FAIL`;
- malformed provider normalization output -> `FAIL`;
- validated provider output -> continue normal validation.

No raw-string fallback is permitted.

### 6. Immutable normalization evidence

For fixed authored payload, package environment, provider version/identity, and semantic resolution context, normalization SHALL be deterministic. A provider semantic revision that changes dependency extraction or payload normalization creates new normalization evidence and SHALL NOT silently mutate an immutable validated model snapshot.

## Consequences

SOL v0.1 avoids inventing a premature universal expression/function/tabular DSL while keeping inline/reified ValueDefinition normalization deterministic and portable across extensions.

## Deferred

- standard SOL expression DSL;
- standard SOL function DSL;
- canonical interpolation/tabular semantics;
- evaluator execution API;
- provider discovery beyond normal package/schema resolution.
