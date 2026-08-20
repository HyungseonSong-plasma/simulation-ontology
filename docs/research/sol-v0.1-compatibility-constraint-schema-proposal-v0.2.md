# SOL v0.1 Compatibility Constraint Schema Proposal v0.2

**Status:** Focused Research revision  
**Date:** 2026-08-20  
**Supersedes for review:** v0.1 proposal only  
**Revision input:** `docs/validation/sol-v0.1-compatibility-constraint-independent-review-v0.1.md`

## 1. Scope

This revision preserves the accepted v0.1 direction:

- one generic Compatibility primitive;
- canonical criterion identity;
- binary v0.1 obligations;
- `expect = compatible | incompatible`;
- conjunctive obligation accumulation;
- semantic compatibility separated from backend/Profile representability.

Only COMP-01 through COMP-04 are changed.

## 2. Payloads

### Authoring payload

```yaml
type: compatibility
criterion: thermal:transfer_quantity_compatible
left: source_temperature
right: target_temperature
expect: compatible
```

Authoring references remain lightweight names/references resolved by the compiler. This ADR slice does not define a general path language.

### Normalized payload

```yaml
type: compatibility
criterion: https://simulation-ontology.org/id/<criterion-id>
left:
  identity_space: model_instance
  id: model-A:source-temperature
right:
  identity_space: model_instance
  id: model-A:target-temperature
expect: compatible
```

A normalized operand reference is:

```text
ResolvedSemanticReference = {
  identity_space: schema | model_instance,
  id: stable resolved identifier
}
```

`backend` is not an allowed identity space in a Core Compatibility payload.

For `identity_space = schema`, `id` is the canonical semantic identity from ADR-0009.
For `identity_space = model_instance`, `id` is the stable SOL model-instance identity available in the resolved model. v0.1 does not require a universal URI grammar for that identity.

This resolves COMP-01.

## 3. Criterion contract

The normalized `criterion` SHALL resolve to exactly one active reified `ConstraintDefinition` whose family is Compatibility.

The criterion definition SHALL provide:

```text
family = compatibility
arity = 2
operand_order = ordered | symmetric
evaluator_contract = deterministic semantic evaluator contract
```

The criterion definition belongs to the loaded SOL/domain package environment. Backend adapters and backend release matchers cannot satisfy this criterion-definition role.

## 4. Obligation identity

### Ordered criterion

```text
ObligationKey = (criterion_id, left_ref, right_ref)
```

`(A,B)` and `(B,A)` are distinct.

### Symmetric criterion

```text
ObligationKey = (criterion_id, unordered_multiset{left_ref, right_ref})
```

The semantic key is an unordered **two-member multiset**, not a lexically sorted tuple.

Therefore:

```text
(A,B) == (B,A)
(A,A) remains a two-operand self-pair
```

JSON/YAML serialization order of `left` and `right` is non-semantic for a symmetric criterion. A compiler MAY choose a stable presentation order, but obligation equality SHALL NOT depend on such ordering.

This resolves COMP-02 without introducing a canonical lexical ordering rule for model-instance identifiers.

## 5. Composition

After criterion and operand resolution:

```text
same ObligationKey + same expect
    -> one obligation

same ObligationKey + opposite expect
    -> empty compatibility intersection

unlike ObligationKeys
    -> retain all obligations conjunctively
```

Different criterion identities are never merged by label or implementation similarity.

## 6. Evaluation-context boundary

Compatibility evaluation has two stages.

### Stage A — resolve semantic evaluation context

Before binary evaluation, all semantic prerequisites declared by the criterion contract must be resolvable.

Examples may include a semantic registry or metrology resolver when a specific criterion requires it.

If a required semantic prerequisite cannot be resolved:

```text
state: INDETERMINATE
code: COMPATIBILITY_EVALUATION_CONTEXT_UNRESOLVED
```

No `compatible` or `incompatible` result is produced.

`INDETERMINATE` is an evaluation-completeness state, not a Constraint Conflict and not backend representability `Blocked`.

### Stage B — binary semantic evaluation

Only after Stage A succeeds:

```text
evaluate(criterion, left, right)
    -> compatible | incompatible
```

Satisfaction:

```text
expect=compatible   + compatible   -> PASS
expect=compatible   + incompatible -> FAIL
expect=incompatible + incompatible -> PASS
expect=incompatible + compatible   -> FAIL
```

Unavailable backend installation, license, runtime, module, release, or adapter capability does not participate in Stage A and cannot produce `COMPATIBILITY_EVALUATION_CONTEXT_UNRESOLVED`.

This resolves COMP-03.

## 7. Constraint-conflict classification

For a completed binary compatibility evaluation:

- an impossible static conjunction such as the same ObligationKey requiring both `compatible` and `incompatible` is a Schema Conflict when contributed statically;
- the same contradiction produced only by simultaneously active Conditional consequents is a Configuration Conflict;
- backend non-representability remains mapping evidence, not a Core Compatibility conflict.

`INDETERMINATE` due to unresolved semantic evaluation context is neither Schema Conflict nor Configuration Conflict.

## 8. Separation from Cardinality/Type/Dimension/Value

The architecture rule remains:

> Define a Compatibility criterion only when the semantic relation is not adequately represented by a single existing primitive family.

However, **generic automatic reducibility detection is not a v0.1 validator requirement**.

A package/ontology review MAY flag a suspicious criterion as a design lint, and a criterion-specific contract MAY define mechanically checkable redundancy rules, but a generic runtime validator SHALL NOT be required to prove that a Compatibility criterion is irreducible.

This resolves COMP-04.

## 9. Reference examples

### Cross-transfer semantic quantity compatibility

Ansys System Coupling declares participant variables by quantity type and creates transfers between same/compatible quantity types. A SOL domain package may define a semantic criterion for transfer-quantity compatibility when the semantic relation is stronger than physical-dimension equality alone.

### Unit/quantity semantic admissibility

After Dimension validation, a future criterion may determine whether a resolved UnitReference is semantically admissible for an expected SemanticQuantity. If that criterion requires a metrology semantic service and the service is unavailable, the result is `INDETERMINATE`, not semantic incompatibility.

These are semantic examples only; backend native compatibility systems are not imported into Core.

## 10. Minimal schema slice after acceptance

Authoring schema fields:

```text
type, criterion, left, right, expect
```

Normalized schema fields:

```text
type
criterion
left.identity_space
left.id
right.identity_space
right.id
expect
```

Structural schema SHALL enforce:

- `identity_space` is `schema | model_instance`;
- no extra backend-local identity field;
- `expect` is `compatible | incompatible`;
- exact field closure for the focused slice.

Semantic helper/tests SHALL cover criterion resolution, ordered/symmetric obligation identity, composition, evaluation-context `INDETERMINATE`, and semantic/representability separation.

## 11. Boundary cases

Required deterministic cases:

1. normalized schema operand with `identity_space=backend` -> structural failure;
2. schema and model-instance operands using the same text ID remain distinct typed references;
3. symmetric `(A,B)` and `(B,A)` -> same ObligationKey without lexical sorting;
4. symmetric `(A,A)` remains valid two-operand self-pair;
5. ordered `(A,B)` and `(B,A)` -> distinct keys;
6. same key + same expectation -> deduplicate;
7. same key + opposite expectation -> empty intersection;
8. different criteria over same operands -> retain both;
9. unresolved semantic criterion prerequisite -> `INDETERMINATE / COMPATIBILITY_EVALUATION_CONTEXT_UNRESOLVED`;
10. backend runtime/license unavailable -> does not produce semantic `INDETERMINATE` or incompatibility;
11. generic reducibility lint absence/presence -> does not affect conformance verdict.

## 12. Finding closure matrix

| Finding | Status | Resolution |
|---|---|---|
| COMP-01 identity-space ambiguity | Resolved | typed `ResolvedSemanticReference` |
| COMP-02 symmetric comparison-key ambiguity | Resolved | unordered two-member multiset semantic key |
| COMP-03 unresolved evaluator boundary | Resolved | explicit evaluation-context `INDETERMINATE` state before binary evaluation |
| COMP-04 generic reducibility detector | Resolved | design lint only, removed from mandatory validator contract |

**Unresolved:** none proposed at contract level.  
**Regression:** none proposed.

## 13. Research verdict

**Ready for focused final contract re-review.**
