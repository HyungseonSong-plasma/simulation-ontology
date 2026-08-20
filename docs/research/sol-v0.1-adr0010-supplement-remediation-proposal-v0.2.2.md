# SOL v0.1 ADR-0010 Supplement Remediation Proposal v0.2.2

**Status:** Limited contract revision  
**Date:** 2026-08-20  
**Branch:** `validation/sol-v0.1-independent-review`  
**Base:** `sol-v0.1-adr0010-supplement-remediation-proposal-v0.2.1.md`  
**Validation input:** `docs/validation/sol-v0.1-remediation-proposals-v0.2.1-final-contract-re-review.md`

## 1. Revision scope

This revision changes only comparator-context resolution for effect pairs that span target components. All v0.2.1 rules not explicitly replaced below remain normative and unchanged. No Core concept, MappingRule field, or MappingClaim field is added.

## 2. Effect component ownership

### CC22-I1 — every normalized effect has one resource-owning component

Every normalized `RealizationEffect` participating in compatibility/collision validation SHALL resolve to exactly one `resource_component_id` identifying the target component that owns the mutated/read backend resource represented by that effect.

The value is derived from the validated `PlanAction` executable descriptor and component-keyed `ResolvedBackendTarget`; it is not a new `MappingClaim` field.

For an orchestration-owned runtime resource, `resource_component_id` is the orchestration component. For a transfer action that reads source component A and writes destination component B, the read-side and write-side effects resolve to A and B respectively even though the enclosing executable descriptor also names the orchestration component.

Boundary:

- descriptor structurally omits required component ownership => `FAIL: EFFECT_COMPONENT_SCOPE_MISSING`;
- component requirement is valid but required runtime component resolution is temporarily unavailable => `BLOCKED`;
- component reference resolves ambiguously => `FAIL: EFFECT_COMPONENT_SCOPE_AMBIGUOUS`.

No validator may infer component ownership from vendor name, resource spelling, declaration order, or the enclosing action's component when the descriptor explicitly spans components.

## 3. Canonical comparison context

### CC22-I2 — local versus cross-component context

For every retained unordered effect pair `{EL, ER}`, the validator SHALL derive exactly one comparison context before resolving any alias/overlap/effect comparator.

```text
if EL.resource_component_id == ER.resource_component_id:
    context = local(component_id)
else:
    context = cross_component(
        left_component_binding,
        right_component_binding,
        orchestration_component_binding
    )
```

A `component_binding` is the tuple:

```text
(component_id, adapter_contract_id, adapter_contract_version)
```

For a composite target, `orchestration_component_binding` is the uniquely resolved orchestration component/adapter required by BT21-I2. If the required orchestration runtime binding is unresolved, comparison is `BLOCKED`; if the composite contract omits or ambiguously defines the orchestration component, validation is `FAIL` before comparator resolution.

Effects belonging to the same PlanAction are not exempt from this rule. They remain in the v0.2.1 complete pair universe unless provenance-proven disjoint, and their compatibility is decided through the same canonical comparison-context procedure.

## 4. Comparator registry key replacement

### CC22-I3 — one registry key for local comparison

For local comparison, the v0.2.1 single-component registry key is retained in normalized form:

```text
(
  comparison_scope = local,
  component_binding,
  comparison_purpose,
  normalized_left_kind,
  normalized_right_kind
)
```

### CC22-I4 — one registry key for cross-component comparison

For a cross-component pair, comparator resolution SHALL use:

```text
(
  comparison_scope = cross_component,
  left_component_binding,
  right_component_binding,
  orchestration_component_binding,
  comparison_purpose,
  normalized_left_kind,
  normalized_right_kind
)
```

The selected versioned orchestration adapter contract SHALL contain the comparator binding for this exact cross-component key. Component-local adapter contracts MAY contribute normalization metadata/evidence but SHALL NOT independently choose the cross-component comparator.

For symmetric comparison purposes, `(left_component_binding, normalized_left_kind)` and `(right_component_binding, normalized_right_kind)` SHALL be canonicalized as a pair using the adapter-contract-defined canonical ordering before registry lookup. For directional purposes, source/target order SHALL be preserved and SHALL be part of the key.

Registry cardinality remains exactly one:

```text
0 matches  => FAIL: COMPARATOR_MISSING
1 match    => use it
>1 matches => FAIL: COMPARATOR_AMBIGUOUS
```

No validator may substitute the left component comparator, right component comparator, or orchestration comparator based on implementation preference.

## 5. Cross-component disjointness and candidate pruning

### CC22-I5 — different components are not automatically disjoint

Different `resource_component_id` values SHALL NOT by themselves prove resource/effect disjointness. A cross-component pair may be pruned from the v0.2.1 complete pair universe only when the uniquely selected cross-component resource/alias procedure returns provenance-bearing conclusive disjointness under the immutable target snapshot.

This preserves the v0.2.1 rule:

```text
no proof of disjointness
=> pair retained and evaluated
```

An orchestration contract MAY define component isolation as a proof rule for specific component/resource-kind combinations. Such a proof must be versioned and recorded; it is not a Core assumption.

## 6. Boundary cases

### 6.1 Positive — local pair

Two MOOSE thermal effects both bind to component `moose-thermal` and use the same adapter contract/build context.

```text
resource components equal
=> local(moose-thermal)
=> exactly one local comparator binding
=> deterministic comparison
```

### 6.2 Positive — cross-component transfer

Composite target:

```text
A = thermal-fluid participant
B = thermal-structural participant
O = orchestration/System-Coupling-like component
```

A transfer descriptor produces a source-read effect owned by A and a destination-write effect owned by B.

```text
A != B
=> cross_component(A-binding, B-binding, O-binding)
=> orchestration contract binds exactly one transfer/effect comparator
=> deterministic comparison
```

For a directional transfer comparator, A→B and B→A are distinct registry keys.

### 6.3 Negative — old v0.2.1 ambiguity

Effects belong to A and B, while local comparators exist on A, B, and O.

A validator chooses A's comparator and another chooses O's comparator.

```text
v0.2.2: non-conforming
required key = cross_component(A, B, O, purpose, kinds)
```

If that key has no exact binding:

```text
=> FAIL: COMPARATOR_MISSING
```

### 6.4 Negative — orchestration runtime unresolved

The composite Profile validly requires O, but its runtime adapter build has not yet been resolved.

```text
valid external prerequisite missing
=> BLOCKED
```

The validator SHALL NOT fall back to A or B.

## 7. Deterministic sequence delta

Replace comparator-context handling in v0.2.1 steps 3/11–13 with:

```text
3. Resolve component-local and orchestration adapter bindings required by the target
...
11. Enumerate complete unordered effect-pair universe
12. Resolve each effect's resource_component_id
13. Derive exactly one local/cross-component comparison context per retained pair
14. Resolve exactly one comparator for that context
15. Prune only with provenance-proven disjointness
16. Compare all retained pairs and validate ordering/collisions
17. Aggregate validation decision using FAIL > BLOCKED > INDETERMINATE > PASS
```

All later lifecycle/representability/execution-permission rules remain unchanged.

## 8. Finding closure

| Validation finding | v0.2.2 status |
|---|---|
| C-21-01 single `target_component_id` cannot uniquely key cross-component comparisons | **Resolved by proposal** |
| local comparator selection | deterministic local context |
| cross-component comparator selection | deterministic pair + orchestration context |
| source/target direction | preserved for directional purposes |
| different components treated as automatically disjoint | explicitly forbidden |
| missing orchestration runtime evidence | canonical BLOCKED |
| missing/ambiguous comparator binding | canonical FAIL |

No previously resolved v0.2.1 contract is intentionally reopened.

## 9. Public-shape decision

- `MappingRule`: five fields remain sufficient.
- `MappingClaim`: four fields remain sufficient.
- `resource_component_id` is MappingPlan/Adapter-IR evidence derived under `realization`/PlanAction lowering, not a new claim field.
- comparator context and registry keys remain MappingPlan/Adapter contracts, not Core ontology vocabulary.

## 10. Research verdict

**READY FOR FOCUSED FINAL CONTRACT RE-REVIEW.**

Only the delta in this document and its consistency with v0.2.1 should be re-reviewed. Architecture freeze remains blocked pending executable validation even if this contract delta passes.
