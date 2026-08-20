# ADR-0020: Dimension Constraint and Canonical DimensionVector

**Status:** Accepted  
**Date:** 2026-08-20  
**Supplements:** ADR-0004, ADR-0005, ADR-0007, ADR-0018

## Context

SOL v0.1 requires physical-dimensional compatibility to remain a backend-independent semantic contract, but the machine-readable `Dimension` Constraint payload and canonical DimensionVector serialization were not fixed.

Independent review accepted a minimal representation based on the seven SI base dimensions, sparse human authoring, explicit normalized vectors, and strict separation from Unit/metrology registries.

## Decision

### 1. Canonical v0.1 dimension basis

SOL v0.1 compares dimensions using seven canonical base-dimension axes:

```text
time
length
mass
electric_current
thermodynamic_temperature
amount_of_substance
luminous_intensity
```

These correspond to the BIPM SI base dimensions T, L, M, I, Theta, N, and J.

The long ASCII names are SOL serialization identifiers. JSON/YAML object order is non-semantic.

### 2. v0.1 exponent domain

Each DimensionVector exponent is a signed integer in v0.1.

Fractional/rational exponent support is deferred. A future extension SHALL use an exact normalization contract rather than floating-point equality.

### 3. Authoring DimensionVector

Human authoring MAY omit zero-valued axes:

```yaml
vector:
  mass: 1
  length: 1
  time: -3
  thermodynamic_temperature: -1
```

Every omitted recognized axis normalizes to exponent `0`.

Unknown axis names are invalid.

### 4. Explicit DimensionOne

DimensionOne is represented by an explicit Dimension payload with an empty sparse vector:

```yaml
type: dimension
vector: {}
```

This normalizes to all seven exponents equal to zero.

Therefore:

```text
missing vector field != DimensionOne
explicit vector: {}   == DimensionOne
```

Missing Unit metadata SHALL NOT be used to infer DimensionOne.

### 5. Normalized DimensionVector

A normalized Dimension payload contains every axis explicitly:

```yaml
type: dimension
vector:
  time: 0
  length: 1
  mass: 1
  electric_current: 0
  thermodynamic_temperature: -1
  amount_of_substance: 0
  luminous_intensity: 0
```

Equality is defined over the normalized axis/exponent mapping, not textual key order.

### 6. Dimension Constraint payload

Authoring and normalized payloads use:

```text
type: dimension
vector: DimensionVector
```

The containing declaration and ADR-0018 evidence identify the semantic subject/use-site. This ADR does not introduce a universal graph path.

### 7. PhysicalDimension graph representation remains orthogonal

The normalized DimensionVector is the machine-comparable validation payload.

This ADR does not decide whether graph-level `PhysicalDimension` is ultimately represented as an Entity, inline typed structure, or another canonical authoring construct.

Any graph-level PhysicalDimension identity used by a compiler SHALL resolve to exactly one canonical DimensionVector before Dimension Constraint composition.

### 8. Dimension composition

Active semantic Dimension constraints on the same semantic subject/use-site compose by normalized equality:

```text
D1 == D2 -> D1
D1 != D2 -> empty Dimension intersection
```

Schema versus Configuration conflict classification follows ADR-0007 activation context. Declaration order and last-write-wins do not participate.

### 9. Unit/metrology boundary

A Dimension Constraint SHALL NOT contain:

- Unit identifiers;
- conversion factors;
- display symbols;
- offsets;
- external metrology-registry handles.

Unit compatibility and conversion may consume the required DimensionVector later, but the Dimension contract itself remains registry-independent.

### 10. Backend representability separation

A backend/Profile inability to serialize Units or dimension metadata SHALL NOT weaken the upstream semantic Dimension contract.

Backend unit handling remains mapping/representability work under ADR-0006/0011.

## Schema implementation slice

The immediate slice SHALL include:

- `constraint-dimension-authoring-v0.1.schema.json`;
- `constraint-dimension-normalized-v0.1.schema.json`;
- a minimal semantic helper/test for sparse normalization, explicit DimensionOne, equality/intersection, and backend-metadata independence.

No backend runtime or external metrology registry is required.

## Consequences

### Positive

- dimensions are machine-comparable without backend or Unit registry dependency;
- dimension-one is explicit;
- sparse authoring and normalized equality are deterministic;
- Unit and semantic quantity identity remain orthogonal;
- Value/Unit consolidation can build on a stable dimensional boundary.

### Costs / scope limits

- validators must normalize sparse vectors before comparison;
- v0.1 intentionally does not support fractional dimensional exponents;
- graph-level PhysicalDimension serialization remains a later representation decision.

## Validation evidence

- `docs/research/sol-v0.1-dimension-constraint-and-dimensionvector-schema-proposal-v0.1.md`
- `docs/validation/sol-v0.1-dimension-constraint-and-dimensionvector-independent-review-v0.1.md`

## Decision summary

SOL v0.1 adopts a seven-axis, integer-exponent canonical DimensionVector, sparse authoring with omitted axes equal to zero, explicit `{}` DimensionOne authoring, full normalized vectors, equality-based Dimension composition, and strict separation from Unit/metrology and backend representability semantics.
