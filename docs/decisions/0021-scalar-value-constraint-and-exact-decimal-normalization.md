# ADR-0021: Scalar Value Constraint and Exact Decimal Normalization

**Status:** Accepted  
**Date:** 2026-08-20  
**Supplements:** ADR-0002, ADR-0007, ADR-0018, ADR-0020

## Context

SOL v0.1 accepts a `Value` Constraint family but lacked a deterministic machine-readable scalar restriction contract. MOOSE, COMSOL, and Ansys all provide recurring range and discrete allowed-value restrictions, while their native parameter and unit syntaxes differ.

Independent review accepted a focused scalar contract after requiring host-language-independent numeric canonicalization, separation of Value emptiness from conflict classification, and an explicit numeric comparison-space precondition.

## Decision

### 1. Focused scalar scope

The v0.1 Value Constraint slice applies to evaluated scalar data only.

Supported scalar kinds are:

```text
number
string
boolean
```

This scalar-data kind is not an Entity taxonomic Type. Vector/tensor/complex values, regex/pattern constraints, arbitrary symbolic expressions, uncertainty, and general structured-value constraints are deferred.

### 2. Two Value forms

SOL v0.1 supports:

```text
ValueConstraint
├── numeric_interval
└── allowed_set
```

Conditional activation is not embedded in either form; ADR-0007 Conditional constraints wrap/activate ordinary Value constraints.

### 3. Exact normalized decimal scalar

Every normalized numeric operand SHALL use:

```yaml
coefficient: "<canonical signed base-10 integer>"
exponent10: <signed integer>
```

with semantic value:

```text
coefficient * 10^exponent10
```

For nonzero coefficients:

- optional minus sign only;
- no leading zeros;
- no trailing zeros.

Zero is uniquely:

```yaml
coefficient: "0"
exponent10: 0
```

Negative zero normalizes to the same representation.

Equivalent authoring forms such as `1`, `1.0`, `1e0`, and `10e-1` normalize identically.

A conforming authoring reader SHALL preserve numeric source semantics sufficiently to construct this exact decimal representation before host-language floating-point rounding can collapse distinct authored values.

### 4. Numeric equality and ordering

Normalized numeric equality is exact canonical decimal equality. Ordering compares the exact decimal values represented by `coefficient * 10^exponent10` and SHALL NOT use binary floating-point identity as semantic authority.

This rule governs interval comparison, numeric allowed-set deduplication, and interval/set filtering.

### 5. Non-finite values

NaN and positive/negative infinity are invalid numeric Value operands in v0.1.

Unbounded interval sides are represented by an absent bound, never by an infinite number.

### 6. Numeric interval

Authoring form:

```yaml
type: value
form: numeric_interval
lower?:
  value: <numeric authoring literal>
  inclusive: <boolean>
upper?:
  value: <numeric authoring literal>
  inclusive: <boolean>
```

Normalized bound values use canonical decimal scalar objects.

Both bounds may be absent, producing a redundant but valid numeric no-op.

An interval is empty when lower exceeds upper or when equal endpoints are open on either side. Equal endpoints with both bounds inclusive form a valid singleton interval.

### 7. Finite allowed set

Authoring form:

```yaml
type: value
form: allowed_set
scalar_kind: number | string | boolean
values: [...]
```

All members SHALL conform to the declared scalar kind. Arrays, objects, and null are excluded from this focused slice.

Set order is non-semantic. Duplicates collapse under scalar equality. Numeric set members normalize to canonical decimal scalars; strings and booleans retain exact scalar values.

### 8. Value satisfiability and conflict classification

Normalization/intersection first returns:

```text
ValueResult
├── satisfiable(normalized payload)
└── empty
```

`empty` is not itself a conflict class. ADR-0007 contributor/activation context determines Schema versus Configuration Conflict.

No empty set/interval survives as a valid effective normalized Value payload.

### 9. Intersection algebra

- interval ∩ interval -> stronger lower/upper bounds using exact numeric ordering;
- allowed_set ∩ allowed_set -> set intersection when scalar kinds match, otherwise empty;
- numeric interval ∩ numeric allowed_set -> filter set by exact interval membership;
- numeric interval ∩ string/boolean allowed_set -> empty.

Declaration order and source priority do not affect the result.

### 10. Value versus ValueDefinition

A Value Constraint restricts the evaluated scalar datum. It does not describe how that datum is obtained. Literal/expression/function/table/external-data mechanisms remain ValueDefinition concerns under ADR-0002.

### 11. Numeric comparison-space precondition

Every numeric Value normalization/intersection uses one comparison-space context state:

```text
not_required
resolved
unresolved
```

The state belongs to ADR-0018 compiler/evidence context, not the semantic Value payload.

If state is `unresolved`, no normalized numeric Value payload may be emitted:

```text
FAIL: VALUE_COMPARISON_SPACE_UNRESOLVED
```

If PhysicalDimensions are incompatible, ADR-0020 Dimension validation fails before Value comparison-space normalization. That is distinct from unresolved comparison space and from Value emptiness.

### 12. Unit/metrology boundary

Unit identifiers, conversion factors, offsets, symbols, and backend handles SHALL NOT be embedded in this Value payload.

Where Unit conversion is needed, operands must be resolved into one comparison representation before Value normalization. This ADR does not define Unit conversion or Unit authoring syntax.

## Schema implementation slice

The immediate design-stage implementation SHALL include:

- `constraint-value-authoring-v0.1.schema.json`;
- `constraint-value-normalized-v0.1.schema.json`;
- minimal exact-decimal normalization and Value intersection helpers/tests.

Tests SHALL cover exact decimal canonicalization, large adjacent integers, signed zero, non-finite rejection, interval openness, allowed-set normalization, cross-form intersection, empty-result classification boundary, and unresolved comparison-space behavior.

No backend runtime or Unit registry is required.

## Consequences

### Positive

- common MOOSE/COMSOL/Ansys range/list restrictions have one backend-independent semantic form;
- numeric equality is independent of Python/JavaScript binary numeric identity;
- ValueDefinition, Type, Dimension, Unit, and Conditional semantics remain orthogonal;
- comparison-space uncertainty cannot silently become a false numeric conflict.

### Costs / deferred scope

- authoring readers must preserve exact numeric meaning before normalization;
- normalized numeric operands are more explicit than native JSON numbers;
- unit-bearing authoring, vector/tensor constraints, regex/pattern rules, complex values, and uncertainty remain later work.

## Validation evidence

- `docs/research/sol-v0.1-value-constraint-schema-proposal-v0.1.md`
- `docs/validation/sol-v0.1-value-constraint-schema-independent-review-v0.1.md`
- `docs/research/sol-v0.1-value-constraint-schema-proposal-v0.2.md`
- `docs/validation/sol-v0.1-value-constraint-schema-focused-final-review-v0.2.md`

## Decision summary

SOL v0.1 uses exact-decimal normalized scalar numbers, numeric intervals, and finite scalar allowed sets for the focused Value Constraint family; Value emptiness is separated from conflict classification, numeric comparison requires a resolved/not-required comparison space, and Unit/ValueDefinition/Conditional semantics remain outside the payload.
