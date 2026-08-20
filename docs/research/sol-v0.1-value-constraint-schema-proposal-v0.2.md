# SOL v0.1 Value Constraint Schema Proposal v0.2

**Role:** Research  
**Date:** 2026-08-20  
**Revision scope:** VAL-V1, VAL-V2, VAL-V3 only

## 1. Preserved contract

This revision does not reopen:

- scalar-only v0.1 Value Constraint scope;
- two normalized forms: numeric interval and finite allowed set;
- scalar kinds `number | string | boolean`;
- open/closed/unbounded numeric interval semantics;
- interval/set cross-form intersection;
- Value versus ValueDefinition separation;
- Value versus Entity taxonomic Type separation;
- Conditional as a separate meta-constraint;
- Unit/metrology data excluded from the Value payload;
- vector/tensor/complex/regex/arbitrary-expression deferrals.

## 2. Canonical exact decimal numeric scalar

To remove host-language number ambiguity, every normalized numeric Value operand SHALL use an exact decimal canonical scalar rather than a native JSON number.

Canonical normalized form:

```yaml
coefficient: "<canonical signed base-10 integer>"
exponent10: <signed integer>
```

Semantic value:

```text
coefficient * 10^exponent10
```

### 2.1 Canonical coefficient rules

For nonzero values:

- optional `-` sign only;
- first digit is `1..9`;
- remaining digits are `0..9`;
- no leading zeros;
- no trailing zeros.

Zero is uniquely:

```yaml
coefficient: "0"
exponent10: 0
```

Negative zero normalizes to the same zero representation.

### 2.2 Canonicalization examples

All of these:

```text
1
1.0
1e0
10e-1
```

normalize to:

```yaml
coefficient: "1"
exponent10: 0
```

`1000` normalizes to:

```yaml
coefficient: "1"
exponent10: 3
```

`0.00100` normalizes to:

```yaml
coefficient: "1"
exponent10: -3
```

`-12.30` normalizes to:

```yaml
coefficient: "-123"
exponent10: -1
```

### 2.3 Authoring-reader requirement

Human authoring MAY continue to use ordinary JSON/YAML numeric literals.

However, a conforming SOL authoring reader/normalizer SHALL preserve the exact decimal numeric token/value sufficiently to construct the canonical decimal scalar **before** host-language binary floating-point rounding can collapse distinct authored values.

Therefore a compiler that first materializes every number as binary64 and then attempts semantic normalization is non-conforming when that loses authored numeric distinctions.

Example values that MUST remain distinct:

```text
9007199254740992
9007199254740993
```

This requirement is a compiler/authoring-reader contract, not a backend runtime requirement.

## 3. Exact numeric equality and ordering

Normalized numeric equality is exact equality of the canonical decimal value.

Because canonical representation is unique:

```text
numeric_equal(A,B)
IFF
A.coefficient == B.coefficient
AND
A.exponent10 == B.exponent10
```

Ordering compares the exact decimal values represented by:

```text
coefficient * 10^exponent10
```

using exact integer/decimal arithmetic. Validators SHALL NOT use binary floating approximation as the semantic authority.

This rule governs:

- interval bound comparison;
- allowed-set numeric duplicate elimination;
- numeric allowed-set equality;
- interval/set filtering;
- cross-form intersection.

## 4. Revised normalized forms

### 4.1 Numeric interval

Authoring remains ergonomic and may use JSON/YAML numbers:

```yaml
type: value
form: numeric_interval
lower?:
  value: <authored numeric literal>
  inclusive: <boolean>
upper?:
  value: <authored numeric literal>
  inclusive: <boolean>
```

Normalized form uses canonical decimal scalars:

```yaml
type: value
form: numeric_interval
lower?:
  value:
    coefficient: "1"
    exponent10: -3
  inclusive: true
upper?:
  value:
    coefficient: "2"
    exponent10: 0
  inclusive: false
```

Both bounds absent remain a valid numeric no-op.

### 4.2 Finite allowed set

Authoring:

```yaml
type: value
form: allowed_set
scalar_kind: number | string | boolean
values: [...]
```

Normalized numeric set members use canonical decimal scalars. String and Boolean members remain their native scalar values.

Example:

```text
{1, 1.0, 2}
```

normalizes semantically to:

```yaml
scalar_kind: number
values:
  - {coefficient: "1", exponent10: 0}
  - {coefficient: "2", exponent10: 0}
```

Set order remains non-semantic.

## 5. Non-finite numeric inputs

NaN, positive/negative infinity, and implementation-specific non-finite numeric extensions are not valid Value numeric operands in SOL v0.1.

A YAML/host parser that can construct such values SHALL reject them before normalized Value payload emission:

```text
FAIL: VALUE_NUMBER_NONFINITE
```

Unbounded interval sides are represented only by absent bound objects, never by infinity numeric values.

## 6. Empty Value result versus conflict classification

Normalization/intersection SHALL first produce one of:

```text
ValueResult
├── satisfiable(normalized Value payload)
└── empty
```

`empty` is a semantic satisfiability result. It is NOT itself a conflict-class label.

Conflict classification uses ADR-0007 contributor/activation evidence:

```text
empty from always-active/intrinsic schema contributors
  -> Schema Conflict

empty only because a Conditional consequent is active for this configuration
  -> Configuration Conflict

inactive Conditional contributor
  -> contributes no Value constraint
```

Therefore an authored empty allowed set or empty interval inside a Conditional is not unconditionally classified as Schema Conflict.

No empty allowed-set or empty-interval payload survives as a valid normalized **effective** Value constraint; it is represented by the internal `empty` result for conflict classification.

## 7. Numeric comparison-space precondition

Every numeric Value normalization/intersection occurs in exactly one resolved semantic comparison space.

The normalization context SHALL classify numeric comparison-space state as one of:

```text
not_required
resolved
unresolved
```

This state belongs to ADR-0018 compiler/evidence context, not to the Value semantic payload.

### `not_required`

No Unit/metrology resolution is needed for the numeric use-site, for example a dimensionless/configuration scalar already defined in the local semantic space.

### `resolved`

Any required Unit/metrology conversion has already resolved all numeric operands to one common semantic comparison representation compatible with ADR-0020 Dimension.

### `unresolved`

A common comparison representation is required but unavailable.

In this state:

```text
NO normalized numeric Value payload may be emitted
FAIL: VALUE_COMPARISON_SPACE_UNRESOLVED
```

This diagnostic means the Value normalizer/compiler precondition is unmet. It is NOT:

- a Dimension conflict;
- a Value interval/set conflict;
- a backend representability failure.

This focused slice still does not define Unit conversion itself.

## 8. Dimension conflict precedence

If numeric operands are known to have incompatible PhysicalDimensions, ADR-0020 Dimension validation fails before Value comparison-space normalization.

A Value normalizer SHALL NOT attempt to convert or compare raw magnitudes across incompatible dimensions.

Thus:

```text
Dimension incompatibility
  != VALUE_COMPARISON_SPACE_UNRESOLVED
  != Value empty intersection
```

These evaluation dimensions remain orthogonal.

## 9. Value intersection algebra (preserved, now exact)

### interval ∩ interval

Use exact canonical decimal ordering to select the stronger lower/upper bounds. Equal-bound openness is handled as previously specified. Empty result is returned as `ValueResult.empty`.

### allowed_set ∩ allowed_set

- different scalar kinds -> `empty`;
- same kind -> mathematical set intersection;
- numeric equality uses canonical decimal scalar equality;
- string/boolean equality remains exact.

### numeric interval ∩ numeric allowed_set

Filter exact normalized numeric set members using exact interval membership.

### numeric interval ∩ string/boolean set

Return `empty`.

## 10. Authoring and normalized schema implications

### Authoring schema

`constraint-value-authoring-v0.1.schema.json` MAY use ordinary JSON numeric instances for ergonomic input. Structural schema cannot by itself prove that a YAML/host parser preserved exact source numeric semantics; the lossless normalization requirement is compiler-level.

### Normalized schema

`constraint-value-normalized-v0.1.schema.json` SHALL use canonical decimal scalar objects for every numeric operand.

Canonical numeric scalar structural shape:

```yaml
coefficient: string
exponent10: integer
```

Semantic canonicality checks include:

- coefficient grammar;
- no nonzero trailing zero;
- zero requires exponent10 = 0.

These may be split between JSON Schema and semantic helper as needed, but independent validators must reject non-canonical normalized numeric payloads.

## 11. Counterexample closure

### VAL-V1-A — binary64 collision

```text
9007199254740992
9007199254740993
```

Expected: distinct canonical decimal scalars. A reader that collapses them is non-conforming.

### VAL-V1-B — equivalent lexical forms

```text
1
1.0
1e0
10e-1
```

Expected: identical canonical scalar `{coefficient:"1", exponent10:0}`.

### VAL-V1-C — signed zero

```text
-0
0.0
```

Expected: unique zero canonical scalar.

### VAL-V2-A — conditional empty set

```text
P -> allowed_set {}
```

Expected:

- P false -> no active conflict;
- P true -> `empty` effective result classified as Configuration Conflict.

### VAL-V2-B — static empty set

Always-active local schema constraint is empty.

Expected: `empty` result classified as Schema Conflict.

### VAL-V3-A — unresolved units

Two numeric operands require Unit conversion, but common comparison space is unresolved.

Expected:

```text
VALUE_COMPARISON_SPACE_UNRESOLVED
```

No normalized numeric Value payload emitted.

### VAL-V3-B — incompatible dimensions

Operands have incompatible ADR-0020 DimensionVectors.

Expected: Dimension conflict before Value comparison. Do not classify as comparison-space unresolved or Value empty intersection.

## 12. Research verdict

VAL-V1, VAL-V2, and VAL-V3 are resolved without changing the accepted two-form Value Constraint architecture.

**Ready for focused final Validation.**
