# ADR-0022: Compatibility Constraint and Semantic Criterion Contract

**Status:** Accepted  
**Date:** 2026-08-20  
**Supplements:** ADR-0007, ADR-0008, ADR-0009, ADR-0018, ADR-0019, ADR-0020, ADR-0021

## Context

ADR-0007 requires one generic `Compatibility` primitive whose obligations compose conjunctively, while backend/Profile representability remains a separate evaluation axis. After Cardinality/QRC, Type, Dimension, and Value schema slices were consolidated, Compatibility still needed a deterministic machine-level contract for identifying what compatibility relation is being asserted, which operands it applies to, how symmetric obligations are compared, and how incomplete semantic evaluation is reported.

Research proposals v0.1 through v0.2.2 and independent Validation closed six contract ambiguities before this ADR was accepted.

## Decision

### 1. One generic Compatibility primitive

SOL v0.1 retains one generic Compatibility family.

```text
Constraint
  └── Compatibility
```

SOL SHALL NOT introduce backend-, unit-, quantity-, or domain-specific Compatibility subclasses in Core v0.1.

### 2. Canonical criterion identity

Each normalized Compatibility obligation SHALL reference exactly one canonical semantic compatibility criterion.

A criterion is represented through the reified `ConstraintDefinition` mechanism already permitted by ADR-0007 because compatibility semantics require independent identity, documentation, versioned package ownership, provenance, and deterministic evaluator semantics.

The resolved criterion definition SHALL provide at least:

```text
family = compatibility
arity = 2
operand_order = ordered | symmetric
evaluator_contract = deterministic semantic evaluator contract
```

Backend adapters, backend release matchers, MappingPlan logic, and vendor capability registries SHALL NOT satisfy this Core semantic criterion role.

### 3. Focused authoring payload

The v0.1 authoring slice uses:

```yaml
type: compatibility
criterion: thermal:transfer_quantity_compatible
left: source_temperature
right: target_temperature
expect: compatible
```

Required fields:

- `type = compatibility`
- `criterion`
- `left`
- `right`
- `expect = compatible | incompatible`

Authoring references are resolved before semantic composition. This ADR does not define a general graph-path language.

### 4. Typed normalized operand references

Normalized operands preserve ADR-0009 identity space explicitly:

```text
ResolvedSemanticReference = {
  identity_space: schema | model_instance,
  id: stable resolved identifier
}
```

For `schema`, the identifier is canonical semantic identity. For `model_instance`, the identifier is stable SOL model-instance identity available in the resolved model.

Backend-local identity is forbidden from the Core Compatibility payload.

### 5. Normalized payload

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

The normalized payload contains no backend identifier, backend release, adapter matcher, vendor capability, or MappingPlan state.

### 6. Obligation identity

For an ordered criterion:

```text
ObligationKey = (criterion_id, left_ref, right_ref)
```

For a symmetric criterion:

```text
ObligationKey = (criterion_id, unordered_multiset{left_ref, right_ref})
```

The symmetric key is an unordered two-member multiset, not a lexically sorted tuple. Thus `(A,B)` equals `(B,A)` while `(A,A)` remains a two-operand self-pair.

Serialization order is non-semantic for symmetric criteria.

### 7. Conjunctive composition

After criterion/operand resolution:

```text
same ObligationKey + same expect
    -> one obligation

same ObligationKey + opposite expect
    -> empty Compatibility intersection

unlike ObligationKeys
    -> retain all obligations conjunctively
```

Declaration order SHALL NOT override an obligation.

### 8. Semantic evaluation context

Compatibility evaluation first resolves semantic prerequisites required by the criterion contract.

If a required semantic prerequisite cannot be resolved:

```text
state: INDETERMINATE
code: COMPATIBILITY_EVALUATION_CONTEXT_UNRESOLVED
```

No binary compatibility result is produced.

`INDETERMINATE` is an evaluation-completeness state, not Schema Conflict, Configuration Conflict, or backend representability `Blocked`.

Backend installation, license, runtime, module, release, or adapter capability SHALL NOT create this semantic `INDETERMINATE` state.

### 9. Binary evaluation

Only after semantic evaluation context is complete:

```text
evaluate(criterion, left, right) -> compatible | incompatible
```

Satisfaction:

```text
expect=compatible   + compatible   -> PASS
expect=compatible   + incompatible -> FAIL
expect=incompatible + incompatible -> PASS
expect=incompatible + compatible   -> FAIL
```

### 10. Family aggregation

The Compatibility family is conjunctive. Its identity and precedence are:

```text
aggregate({}) = PASS

FAIL > INDETERMINATE > PASS
```

Normatively:

```text
if no active obligations:
    PASS
else if any active obligation == FAIL:
    FAIL
else if any active obligation == INDETERMINATE:
    INDETERMINATE
else:
    PASS
```

Normalization/composition conflicts are reported before family-level evaluation aggregation.

### 11. Conflict classification

A static contradiction for the same ObligationKey, such as simultaneously requiring `compatible` and `incompatible`, is a Schema Conflict when contributed statically.

The same contradiction created only by simultaneously active Conditional consequents is a Configuration Conflict.

Backend non-representability remains mapping/representability evidence rather than a Core Compatibility conflict.

### 12. Separation from other primitive families

Compatibility SHOULD be defined only when the semantic relation is not adequately represented by a single existing primitive family.

However generic automatic proof of reducibility to Cardinality/Type/Dimension/Value is **not** a v0.1 validator requirement. Such checking may be a package/design lint or criterion-specific rule.

### 13. Evidence boundary

Reference-system evidence supports this separation:

- MOOSE `MooseUnits` has explicit dimensional-conformance semantics, supporting Dimension as a separate primitive concern;
- COMSOL checks expected dimensions and unit consistency in expression/operator contexts;
- Ansys System Coupling uses same/compatible quantity types for participant data transfers, providing a real cross-entity semantic compatibility case richer than literal unit equality.

These native mechanisms are evidence only and are not copied into SOL Core.

## Schema implementation slice

The design-stage slice SHALL include:

- `schema/constraint-compatibility-authoring-v0.1.schema.json`;
- `schema/constraint-compatibility-normalized-v0.1.schema.json`;
- a minimal semantic helper/test covering typed operand references, ordered/symmetric obligation identity, duplicate/opposite composition, semantic `INDETERMINATE`, family aggregation, and semantic/representability separation.

No backend runtime, backend license, production Adapter, or external execution environment is required.

## Consequences

### Positive

- Compatibility remains generic without becoming semantically free-form;
- criterion meaning has durable identity;
- schema and model-instance operands cannot be conflated;
- symmetric obligations are deterministic without inventing a global lexical ordering;
- unresolved semantic prerequisites do not become false incompatibility;
- backend limitations remain outside Core semantic validity.

### Costs / limits

- criteria that need durable semantics must be reified as `ConstraintDefinition` resources;
- v0.1 supports binary compatibility only;
- production evaluator plugin APIs remain deferred;
- generic reducibility checking is a design concern rather than a runtime requirement.

## Validation evidence

- `docs/research/sol-v0.1-compatibility-constraint-schema-proposal-v0.1.md`
- `docs/validation/sol-v0.1-compatibility-constraint-independent-review-v0.1.md`
- `docs/research/sol-v0.1-compatibility-constraint-schema-proposal-v0.2.md`
- `docs/validation/sol-v0.1-compatibility-constraint-v0.2-focused-re-review.md`
- `docs/research/sol-v0.1-compatibility-constraint-schema-proposal-v0.2.1.md`
- `docs/validation/sol-v0.1-compatibility-constraint-v0.2.1-final-re-review.md`
- `docs/research/sol-v0.1-compatibility-constraint-schema-proposal-v0.2.2.md`
- `docs/validation/sol-v0.1-compatibility-constraint-final-contract-review.md`

## Decision summary

SOL v0.1 Compatibility is a binary, criterion-identified semantic obligation over typed resolved references, with deterministic ordered/symmetric identity, conjunctive composition, explicit semantic `INDETERMINATE`, empty-set `PASS`, and strict separation from backend/Profile representability.
