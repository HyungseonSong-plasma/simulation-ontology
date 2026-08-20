# SOL v0.1 Compatibility Constraint Schema Proposal v0.1

**Status:** Research proposal  
**Date:** 2026-08-20  
**Scope:** SOL v0.1 design-stage Constraint consolidation  
**Depends on:** ADR-0007, ADR-0008, ADR-0009, ADR-0018, ADR-0019, ADR-0020, ADR-0021

## 1. Problem

ADR-0007 requires one generic `Compatibility` primitive whose obligations compose conjunctively, while explicitly forbidding backend representability from becoming Core semantic compatibility. Cardinality, Type, Dimension, and Value now have focused normalized contracts, so the remaining question is what irreducible semantic work `Compatibility` performs and how two independent validators identify the same compatibility obligation.

A free-form `kind: string` is insufficient because two validators may attach different semantics to the same text, and backend/profile capability checks could leak into Core. Conversely, splitting Compatibility into quantity/unit/backend subclasses would contradict ADR-0007.

## 2. Research conclusion

Keep a single generic Compatibility primitive, but require every normalized obligation to reference a **canonical compatibility criterion** with durable semantic identity.

No new Core Entity family is introduced. A compatibility criterion is represented by the already-permitted reified `ConstraintDefinition` mechanism from ADR-0007 because independent identity, reuse, provenance, documentation, versioning, and evaluator semantics are precisely what justify reification here.

Conceptually:

```text
Compatibility Constraint instance
        |
        | criterion
        v
Compatibility ConstraintDefinition
        |
        | defines semantic evaluation contract
        v
resolved operand pair -> compatible / incompatible
```

## 3. Minimal v0.1 payload

### Authoring

```yaml
type: compatibility
criterion: thermal:transfer_quantity_compatible
left: source_temperature
right: target_temperature
expect: compatible
```

`criterion`, `left`, and `right` are authoring references. Resolution is performed before semantic composition. This proposal does not standardize a general graph-path language.

### Normalized

```yaml
type: compatibility
criterion: https://simulation-ontology.org/id/<criterion-id>
left: <resolved-semantic-reference>
right: <resolved-semantic-reference>
expect: compatible
```

Required fields:

- `type = compatibility`
- `criterion`
- `left`
- `right`
- `expect = compatible | incompatible`

The normalized payload contains no backend identifier, backend release, adapter matcher, vendor capability, or MappingPlan state.

## 4. Criterion contract

The canonical `criterion` SHALL resolve to exactly one active reified `ConstraintDefinition` whose family is Compatibility.

For v0.1 the definition SHALL provide enough normative information for deterministic dispatch:

```text
family = compatibility
arity = 2
operand_order = ordered | symmetric
evaluator_contract = semantic evaluator identity/contract
```

The evaluator is a semantic validator contract supplied by the owning SOL/domain package. It is not a backend adapter dispatch hook.

`operand_order` means:

- `ordered`: `(A,B)` and `(B,A)` are different obligations unless the criterion itself defines otherwise;
- `symmetric`: normalization canonicalizes the pair using stable resolved-reference comparison keys so `(A,B)` and `(B,A)` are the same obligation.

Missing criterion, multiple active criterion definitions, wrong family, wrong arity, or missing deterministic evaluator contract are normalization/contract failures rather than a compatibility result.

## 5. Why criterion identity is necessary

Examples called “compatibility” can have different semantics even when they involve the same operands:

```text
same physical dimension
same semantic transfer quantity
interface-role refinement compatibility
semantic coupling compatibility
```

They must not be silently treated as the same relation. Canonical criterion identity answers **which compatibility relation is being required**.

This follows ADR-0009 stable semantic identity: display labels, package versions, paths, and backend-local names are not semantic identity.

## 6. Composition algebra

After criterion and operand references are resolved and symmetric pairs are canonicalized, define obligation identity as:

```text
ObligationKey = (criterion_id, canonical_operand_pair)
```

Composition is conjunctive set accumulation:

```text
same key + same expect
    -> deduplicate / one obligation

same key + opposite expect
    -> empty compatibility intersection

unlike keys
    -> preserve both obligations conjunctively
```

No declaration-order override is allowed.

Two different criterion IDs SHALL NOT be merged merely because their labels or evaluator implementations appear similar.

## 7. Evaluation semantics

For each satisfiable normalized obligation:

```text
evaluate(criterion, left, right) -> compatible | incompatible
```

Constraint satisfaction is:

```text
expect=compatible   and result=compatible   -> PASS
expect=compatible   and result=incompatible -> FAIL
expect=incompatible and result=incompatible -> PASS
expect=incompatible and result=compatible   -> FAIL
```

Unavailable backend runtime, backend license, unsupported backend release, or adapter capability SHALL NOT change this semantic result. Those belong to representability/mapping validation under ADR-0006/0011.

If a semantic criterion itself depends on a separately resolved semantic service (for example a metrology resolver), inability to resolve that semantic service SHALL be reported as an evaluation-resolution state/diagnostic rather than guessed as compatible or incompatible. This proposal does not classify external execution availability as semantic incompatibility.

## 8. Separation from other Constraint families

Compatibility SHALL NOT duplicate checks already reducible to one primitive family:

- endpoint subtype narrowing -> Type;
- physical-dimension equality -> Dimension;
- scalar interval/set restrictions -> Value;
- relation multiplicity -> Cardinality.

Compatibility is retained for a cross-entity/cross-contract semantic relation whose satisfaction cannot be represented as one of those primitives alone.

For example, an Ansys-style transfer may require source/target variables to have the same or compatible **semantic quantity type**, while dimensional equality alone can be insufficient. This is a Compatibility criterion, not a Dimension constraint.

Similarly, quantity-specific unit admissibility may eventually use a semantic compatibility criterion between a UnitReference and expected SemanticQuantity after dimension validation, without moving the external unit registry into Core.

## 9. Interface interaction

ADR-0008 permits explicit mappings from Interface requirements to concrete Properties/Relations. Compatibility may be used only when the mapping requires an irreducible semantic compatibility judgment after ordinary Type/Cardinality/etc. constraints have been applied.

It SHALL NOT be used as a generic escape hatch to bypass Interface requirements.

## 10. Semantic versus representability axis

The same separation established for Type constraints remains mandatory:

```text
semantic compatibility result
    !=
backend/profile representability result
```

A Profile may declare that a backend cannot realize a semantically compatible contract. That produces unsupported/lossy mapping evidence, not a failing Core Compatibility constraint.

Backend release matching from ADR-0009 is adapter/Profile-owned and SHALL NOT be encoded as a Core Compatibility criterion.

## 11. Minimal structural schema slice

If this proposal is accepted, add:

- `schema/constraint-compatibility-authoring-v0.1.schema.json`
- `schema/constraint-compatibility-normalized-v0.1.schema.json`

Authoring structural shape:

```json
{
  "type": "compatibility",
  "criterion": "<authoring reference>",
  "left": "<authoring reference>",
  "right": "<authoring reference>",
  "expect": "compatible | incompatible"
}
```

Normalized structural shape is the same five fields after canonical resolution. JSON Schema validates field shape only; criterion resolution, pair canonicalization, evaluator dispatch, and obligation composition remain semantic/compiler checks.

## 12. Required boundary cases

An independent implementation should be forced to the following outcomes:

1. unresolved criterion -> normalization failure;
2. criterion resolves to non-Compatibility ConstraintDefinition -> failure;
3. multiple active definitions for same canonical criterion -> failure;
4. symmetric criterion `(A,B)` and `(B,A)` -> same ObligationKey;
5. ordered criterion `(A,B)` and `(B,A)` -> distinct ObligationKeys;
6. same key `compatible + compatible` -> one obligation;
7. same key `compatible + incompatible` -> empty intersection/conflict;
8. different criterion IDs over same operands -> both obligations retained;
9. backend-only criterion/matcher presented as Core semantic criterion -> rejected by semantic-axis contract;
10. Type/Dimension/Value/Cardinality-reducible rule disguised as Compatibility -> validation/design diagnostic, not a new Compatibility semantic;
11. unavailable backend runtime/license -> no change to semantic compatibility verdict;
12. criterion evaluator unavailable because required semantic service is unresolved -> explicit unresolved evaluation state, never guessed PASS/FAIL.

## 13. Reference evidence

Official backend evidence supports the separation between generic semantic compatibility and primitive checks:

- MOOSE `MooseUnits` canonicalizes seven-base-unit dimensions and tests dimensional conformance, showing Dimension compatibility has a precise dedicated algebra rather than requiring a generic Compatibility kind.
- COMSOL expression/operator documentation explicitly checks expected argument dimensions and performs unit conversion, likewise supporting a distinct Dimension/unit-comparison boundary.
- Ansys System Coupling declares participant transfer variables by quantity type and creates transfers between same/compatible quantity types, showing a real cross-entity semantic compatibility relation that is richer than literal unit equality.

These backend concepts are evidence for the semantic boundary only; their native implementation contracts are not copied into SOL Core.

## 14. Non-goals / deferred

This proposal does not define:

- a universal backend capability matcher;
- backend release compatibility;
- arbitrary N-ary compatibility (v0.1 arity is 2);
- fuzzy/tolerance compatibility;
- logical OR among compatibility criteria;
- a general graph selector/path syntax;
- a standardized global catalogue of compatibility criterion kinds;
- production evaluator plugin APIs.

## 15. Research verdict

**Ready for independent contract Validation.**

The proposal preserves ADR-0007's single generic Compatibility primitive while removing free-form semantic ambiguity through canonical criterion identity and deterministic binary-obligation composition. It does not reopen backend representability, Type, Dimension, Value, or Cardinality semantics.
