# SOL v0.1 Value Constraint Schema Proposal v0.1

**Role:** Research  
**Date:** 2026-08-20  
**Branch:** `validation/sol-v0.1-independent-review`

## 1. Objective

Define the minimum machine-readable `Value` Constraint family required by ADR-0007 without conflating:

- a semantic quantity with its evaluated `Value` (ADR-0002);
- a `ValueConstraint` with `ValueDefinition` evaluation mechanisms;
- Entity taxonomic `Type` constraints with scalar literal kinds;
- numeric value comparison with unresolved Unit/metrology conversion;
- ordinary Value restrictions with Conditional activation.

The focused v0.1 slice covers scalar numeric intervals and finite scalar allowed sets.

## 2. Accepted semantic basis

ADR-0002 establishes:

```text
SemanticQuantity != Value
ValueDefinition describes how a Value is obtained
value shape != evaluation mechanism
```

ADR-0007 establishes `Value` as one of the six Constraint primitive families, conjunctive composition, normalization before intersection where possible, and explicit conflict detection.

ADR-0020 now supplies a canonical machine-comparable Dimension boundary, while Unit registry/conversion representation remains separate work.

## 3. Cross-backend evidence

### MOOSE

MOOSE `InputParameters` supports range-checked parameters and enum-style allowed options. This provides backend evidence for scalar range and finite-set restrictions without making MOOSE parameter syntax part of SOL Core.

Reference:
https://mooseframework.inl.gov/releases/moose/2024-11-11/source/utils/InputParameters.html

### COMSOL

COMSOL Physics Builder supports integer/real value checks with unbounded, open, and closed min/max bounds. It also exposes Allowed Values lists. Dynamic activation of allowed values exists separately and is evidence for SOL Conditional constraints rather than for ordinary Value payload syntax.

References:
https://doc.comsol.com/6.4/doc/com.comsol.help.comsol/COMSOL_PhysicsBuilderManual.pdf
https://doc.comsol.com/6.4/doc/com.comsol.help.comsol/physics_builder_manual_tools.43.109.html
https://doc.comsol.com/6.4/doc/com.comsol.help.comsol/physics_builder_manual_tools.43.110.html

### Ansys

Ansys parameter definitions distinguish continuous lower/upper limits from discrete value lists, again supporting interval/set normalization rather than backend-specific option metadata.

Reference:
https://ansyshelp.ansys.com/public/Views/Secured/hpcplat/v140/en/rep_ug/input_output_parameters.html

## 4. v0.1 scalar-kind boundary

The focused Value Constraint operates on evaluated **scalar literals** only.

Supported literal kinds:

```text
number
string
boolean
```

`scalar_kind` describes the literal-data comparison axis. It is NOT an Entity Type and SHALL NOT participate in ADR-0019 taxonomic Type intersection.

Vector/tensor elementwise value constraints, structured objects, complex values, regex/string-pattern constraints, symbolic expressions, uncertainty, and arbitrary predicates are deferred.

## 5. Normalized forms

SOL v0.1 uses two normalized Value Constraint forms.

### 5.1 Numeric interval

```yaml
type: value
form: numeric_interval
lower?:
  value: <JSON number>
  inclusive: <boolean>
upper?:
  value: <JSON number>
  inclusive: <boolean>
```

Rules:

- at least one bound MAY be absent;
- absent lower means negative-unbounded;
- absent upper means positive-unbounded;
- if both are absent, the constraint is a semantically redundant numeric no-op but structurally valid;
- `inclusive` is required whenever that bound exists;
- no Unit, dimension, backend handle, expression, or predicate is embedded in the interval payload.

JSON itself excludes NaN and infinities from valid interoperable number syntax; unboundedness is represented structurally by an absent bound.

### 5.2 Finite scalar allowed set

```yaml
type: value
form: allowed_set
scalar_kind: number | string | boolean
values: [<scalar values>]
```

Rules:

- all values SHALL belong to the declared scalar kind;
- values are semantically an unordered mathematical set, not an ordered sequence;
- duplicates collapse under scalar equality during normalization;
- an empty authored/normalized set denotes an unsatisfiable Value constraint and SHALL produce a Schema Conflict rather than silently becoming a no-op;
- arrays/objects/null are not supported scalar values in this v0.1 slice.

## 6. Scalar equality

Normalized allowed-set equality uses:

### Boolean

Exact Boolean equality.

### String

Exact Unicode string value equality. Locale-aware or case-folded equality is not part of Core v0.1.

### Number

Mathematical JSON-number equality for finite JSON numbers:

```text
1 == 1.0
-0 == 0
```

A validator SHALL NOT use host-language type identity to distinguish mathematically equal JSON numeric values.

Normalized set serialization order is non-semantic. Validators compare the normalized mathematical set, not array order.

A future canonical byte serialization MAY define deterministic element ordering independently of this semantic rule.

## 7. Numeric interval normalization

For lower bounds, the stronger bound is the numerically larger value; if values are equal, an open bound is stronger than a closed bound.

For upper bounds, the stronger bound is the numerically smaller value; if values are equal, an open bound is stronger than a closed bound.

An interval is empty when:

```text
lower.value > upper.value
```

or when:

```text
lower.value == upper.value
AND
(lower.inclusive == false OR upper.inclusive == false)
```

A single-point interval is valid only when both equal bounds are inclusive.

## 8. Value intersection algebra

All active Value constraints on the same semantic scalar use-site compose conjunctively.

### 8.1 interval ∩ interval

Take the stronger lower bound and stronger upper bound. Empty result -> Value conflict.

### 8.2 allowed_set ∩ allowed_set

If `scalar_kind` differs, intersection is empty.

If kinds match, intersect normalized scalar sets. Empty result -> Value conflict.

### 8.3 numeric_interval ∩ numeric allowed_set

Filter the numeric set to members satisfying the interval.

Empty result -> Value conflict.

### 8.4 numeric_interval ∩ string/boolean allowed_set

Empty Value-axis intersection.

This cross-form rule is necessary so equivalent restrictions do not depend on which backend supplied interval versus discrete-list metadata.

## 9. Conflict classification

As with the other Constraint families:

- intrinsically unsatisfiable declarations -> Schema Conflict;
- incompatibility caused only by jointly active Conditional consequents -> Configuration Conflict.

Declaration order, source priority, and last-write-wins are forbidden.

## 10. Value / ValueDefinition boundary

A Value Constraint restricts the **evaluated scalar datum**.

It does not specify whether the datum was produced by:

- literal definition;
- expression;
- function;
- interpolation/table;
- external data;
- another future ValueDefinition mechanism.

Therefore ValueDefinition subtype taxonomy remains outside this Constraint payload.

## 11. Unit and Dimension boundary

Numeric Value comparison is valid only inside one resolved semantic comparison space.

For physical quantities:

1. dimensional compatibility SHALL be established using the ADR-0020 Dimension contract;
2. if Unit conversion is required, the authoring/compiler layer must resolve the operands to one canonical comparison representation before Value intersection;
3. raw numeric magnitudes from unresolved or incompatible Unit contexts SHALL NOT be compared merely because they are JSON numbers.

The focused Value payload therefore contains no Unit identifier or conversion factor.

This proposal does NOT yet define unit-bearing Value authoring syntax. That is intentionally deferred to the accepted Unit/metrology representation work.

Dimension-one numeric values are not inferred from missing Unit metadata; ADR-0004/0020 remain authoritative.

## 12. Conditional boundary

Backend evidence includes conditionally activated allowed values, but ordinary Value constraints SHALL NOT contain activation predicates.

Use:

```text
Conditional(P -> ValueConstraint)
```

rather than embedding `if` syntax inside numeric intervals or allowed sets.

This preserves ADR-0007's orthogonal Conditional meta-constraint architecture.

## 13. Authoring/normalized schema boundary

For this first Value slice, authoring and normalized structural forms MAY share the same field shape because there are no semantic identifiers inside the payload.

Normalization still performs:

- numeric equality canonicalization for set semantics;
- duplicate elimination;
- interval emptiness detection;
- allowed-set homogeneous-kind checking;
- cross-form canonical intersection.

ADR-0018 evidence/context remains outside the semantic payload.

## 14. Proposed schema files after acceptance

### `constraint-value-authoring-v0.1.schema.json`

A discriminated union of:

- `numeric_interval`;
- `allowed_set`.

Structural rules include:

- only finite JSON numeric syntax for interval bound values;
- bound object requires `value` and `inclusive`;
- allowed-set `scalar_kind` required;
- allowed values restricted to scalar JSON number/string/boolean;
- no extra properties.

JSON Schema may use `if/then` solely to enforce structure based on `scalar_kind`; this is not SOL Conditional semantic logic.

### `constraint-value-normalized-v0.1.schema.json`

Same semantic forms, with normalizer-level invariants tested outside structural schema:

- duplicates removed semantically;
- no empty interval survives normalization;
- no empty allowed set survives as a valid effective constraint;
- array order is non-semantic.

## 15. Counterexamples

### VC-01 — numeric endpoint openness

```text
x > 0
x <= 1
```

Expected normalized interval: `(0,1]`.

### VC-02 — equal open endpoint

```text
x >= 1
x < 1
```

Expected: empty Value intersection.

### VC-03 — numeric set equality

```text
{1, 1.0, 2}
```

Expected normalized mathematical set: `{1,2}`.

### VC-04 — heterogeneous set

```yaml
scalar_kind: number
values: [1, "1"]
```

Expected: structural/semantic failure; validator SHALL NOT coerce string to number.

### VC-05 — set/interval cross intersection

```text
allowed = {0, 0.5, 2}
interval = (0,1]
```

Expected effective set: `{0.5}`.

### VC-06 — boolean/numeric distinction

```text
true != 1
false != 0
```

Expected: separate scalar kinds; no host-language coercion.

### VC-07 — unit-context leak

One source supplies `1 m`, another `100 cm` but Unit normalization has not been resolved.

Expected: do not compare raw magnitudes `1` and `100` as a Value conflict. Unit/metrology normalization must establish the common comparison representation first.

### VC-08 — Conditional allowed value

Backend says value `advanced` is allowed only when mode predicate P is true.

Expected SOL normalization:

```text
P -> allowed_set{advanced}
```

not a predicate embedded inside the Value payload.

## 16. Research verdict

**Ready for independent Validation.**

The proposal keeps Value constraints focused on evaluated scalar restrictions, supports the interval/discrete-set patterns demonstrated by MOOSE/COMSOL/Ansys, defines cross-form intersection deterministically, and preserves the already accepted separation from ValueDefinition, Type, Dimension, Unit/metrology, and Conditional semantics.
