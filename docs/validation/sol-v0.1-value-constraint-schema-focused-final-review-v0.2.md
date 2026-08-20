# SOL v0.1 Value Constraint Schema Focused Final Review v0.2

**Role:** Validation  
**Date:** 2026-08-20

## Scope

Focused re-review of VAL-V1, VAL-V2, and VAL-V3 from the v0.1 independent review. Only the revised proposal and accepted ADR context were used as authority.

## Findings

### VAL-V1 — Numeric equality/canonicalization

**Resolved.**

Normalized numeric operands now use exact canonical decimal scalars:

```text
coefficient * 10^exponent10
```

with unique nonzero coefficient normalization and unique zero. This removes dependency on host-language `int`, binary64, or lexical-number identity.

The authoring-reader requirement explicitly forbids losing numeric distinctions before canonical normalization. Exact equality and ordering therefore have one semantic authority.

### VAL-V2 — Empty result / conflict classification

**Resolved.**

The revision separates:

```text
ValueResult = satisfiable(payload) | empty
```

from ADR-0007 conflict classification. Static/intrinsic empty results can be Schema Conflicts; emptiness caused only by active Conditional consequents is a Configuration Conflict. Inactive consequents contribute nothing.

No declaration-order behavior is introduced.

### VAL-V3 — Numeric comparison-space gate

**Resolved.**

Numeric normalization requires context state:

```text
not_required | resolved | unresolved
```

`unresolved` deterministically prevents normalized numeric payload emission with:

```text
VALUE_COMPARISON_SPACE_UNRESOLVED
```

This state is correctly separated from ADR-0020 Dimension incompatibility, Value emptiness, and backend representability.

## Regression review

The following accepted v0.1 decisions remain intact:

- Value Constraint applies to evaluated scalar data, not ValueDefinition mechanism.
- numeric interval and finite allowed set remain the only focused forms.
- open/closed/unbounded interval semantics remain explicit.
- scalar kinds remain number/string/boolean and are not Entity Types.
- interval/set cross-form intersection is defined.
- Conditional predicates do not enter the Value payload.
- Unit/metrology identifiers and conversion factors remain outside the payload.
- vector/tensor/complex/regex/arbitrary-expression constraints remain deferred.

## Remaining implementation boundary

Before implementation acceptance, schema/helper tests should cover:

1. exact decimal canonicalization across equivalent lexical numeric forms;
2. distinction of large adjacent integers beyond binary64 exact range;
3. signed zero normalization;
4. non-finite rejection;
5. open/closed interval emptiness;
6. numeric allowed-set deduplication;
7. heterogeneous set rejection/no coercion;
8. interval/set filtering;
9. semantic `empty` result separated from conflict classifier;
10. comparison-space unresolved diagnostic;
11. Dimension conflict treated before Value normalization by orchestration/precondition tests.

## Verdict

**Accept.**

The Value Constraint contract is ready for focused ADR drafting and a design-stage schema/semantic-helper implementation slice. No backend runtime or Unit registry installation is required.
