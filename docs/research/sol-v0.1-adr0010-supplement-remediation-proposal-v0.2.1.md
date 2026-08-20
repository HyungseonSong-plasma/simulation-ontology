# SOL v0.1 ADR-0010 Supplement Remediation Proposal v0.2.1

**Status:** Limited contract revision; no accepted ADR modified  
**Date:** 2026-08-20  
**Branch:** `validation/sol-v0.1-independent-review`  
**Supersedes for contract review:** `sol-v0.1-adr0010-supplement-remediation-proposal-v0.2.md`  
**Primary review input:** `docs/validation/sol-v0.1-remediation-proposals-v0.2-contract-re-review.md`

## 1. Revision scope

This is a **v0.2.x contract-only revision**. It preserves the v0.2 architecture direction and changes only the normative ambiguities identified by the independent contract re-review.

The following public shapes remain unchanged:

```text
MappingRule
├── source
├── applicability
├── realization
├── bindings
└── capabilities

MappingClaim
├── obligation
├── source
├── realization
└── provenance
```

No sixth `MappingRule` field and no fifth `MappingClaim` field are introduced.

The following v0.2 decisions remain unchanged unless explicitly tightened below:

- `RealizationEffect`, `EffectComparison`, and `PlanAction` are MappingPlan/Adapter IR, not Core semantic vocabulary;
- declaration order and numeric priority are not semantic resolution mechanisms;
- `pending / blocked / indeterminate / complete` remain evaluation lifecycle states;
- `exact / transformed / lossy / unsupported` remain terminal representability outcomes;
- BackendTarget requirement and resolved runtime identity remain distinct;
- evaluation revisions remain immutable;
- vendor/product vocabulary remains Profile/Adapter-owned.

---

# 2. Common deterministic decision contract

For all contract checks in this proposal the decision domain is:

```text
PASS
FAIL
BLOCKED
INDETERMINATE
```

Normative provenance boundary:

- `FAIL`: supplied contract/input is malformed, contradictory, ambiguously bound, or a required implementation/registry binding is absent or multiply matched when the contract requires exactly one.
- `BLOCKED`: the contract is valid, but required external/runtime evidence that may become available is absent or unresolved.
- `INDETERMINATE`: all required inputs and the uniquely selected procedure are present, but that procedure returns no semantic decision for the concrete complete inputs.
- `PASS`: the check is valid and deterministically satisfied.

A missing registered procedure is **not** `INDETERMINATE`; it is `FAIL`. An unresolved runtime snapshot is **not** `FAIL`; it is `BLOCKED`.

---

# 3. RealizationEffect candidate-pair completeness

## 3.1 Normative rule RE21-I1 — complete pair universe

After all action descriptors have been normalized for one immutable MappingPlan revision, the validator SHALL form the unordered pair universe:

```text
P = { {Ei, Ej} | i < j }
```

over all normalized runtime mutation effects participating in the plan, including effects classified as non-semantic bookkeeping when they can mutate runtime state.

A pair MAY be omitted from subsequent compatibility comparison **only** when the uniquely bound resource/alias comparison procedure returns a provenance-bearing conclusive result equivalent to:

```text
resource relation = separate
AND
effect scopes cannot overlap
```

under the same immutable `ResolvedBackendTarget` snapshot.

The following are insufficient grounds for pruning:

- different plan-local resource keys;
- different source identities;
- different obligation keys;
- different backend aliases without resolved alias evidence;
- declaration or serialization order.

If disjointness requires external/runtime evidence that is missing, the pair remains unresolved and contributes `BLOCKED`. If complete evidence is present but the uniquely selected comparator cannot decide disjointness/overlap, it contributes `INDETERMINATE`.

Therefore:

```text
pair omitted
=> recorded conclusive proof of disjointness

no proof of disjointness
=> pair is retained and evaluated
```

## 3.2 Positive boundary case

Effects `E1` and `E2` have different canonical plan keys. The bound alias comparator resolves them to two disjoint domain selections `{1,2}` and `{5,6}` with complete target evidence.

```text
comparison = separate
scope overlap = false
=> pair may be pruned
=> PASS for this pair
```

## 3.3 Negative boundary case

Effects `E1` and `E2` have different plan keys, but one key refers to an external named selection whose runtime membership snapshot is not available.

```text
resource-key inequality alone => no pruning
missing external selection snapshot => BLOCKED
```

A validator returning PASS by prefiltering the pair is non-conforming.

---

# 4. Comparator registry uniqueness

## 4.1 Normative rule RE21-I2 — exact comparator binding

The selected versioned adapter contract SHALL bind every required comparison to **exactly one** comparator implementation/version by the canonical registry key:

```text
(
  adapter_contract_id,
  adapter_contract_version,
  target_component_id,
  comparison_purpose,
  normalized_left_kind,
  normalized_right_kind
)
```

For symmetric comparison purposes, left/right kinds SHALL first be canonicalized into the adapter-defined symmetric order.

The adapter contract SHALL identify the allowed comparator ID and version constraint for that key. Registry resolution SHALL produce exactly one installed implementation satisfying the binding.

```text
0 matching implementations  => FAIL: COMPARATOR_MISSING
1 matching implementation   => use it
>1 matching implementations => FAIL: COMPARATOR_AMBIGUOUS
```

No newest-version rule, registration order, declaration order, generic-vs-specific preference, or implementation-defined tie-break is permitted.

Every `EffectComparison`, effect-equivalence decision, idempotency decision, formulation compatibility decision, and alias/overlap decision SHALL record the exact selected comparator identity/version and normalized input fingerprints.

## 4.2 Positive boundary case

The adapter contract binds selection overlap on component `thermal` to `selection-overlap@2.1.0`. The registry contains that exact implementation and an unrelated `geometric-overlap@2.0.0` not bound to this registry key.

```text
unique bound implementation => PASS
```

## 4.3 Negative boundary case

The contract allows `selection-overlap >=2 <3`, and two installed implementations `2.1.0` and `2.2.0` both satisfy it without an exact deterministic resolver binding.

```text
multiple valid matches => FAIL: COMPARATOR_AMBIGUOUS
```

The validator SHALL NOT choose the highest version implicitly.

---

# 5. Executable descriptor and declared effect equivalence

## 5.1 Normative rule PA21-I1 — one executable surface

Every `PlanAction` SHALL carry or resolve to one adapter-owned executable descriptor. For the selected adapter contract/version and immutable target-state snapshot, the adapter SHALL expose one deterministic procedure:

```text
describe_effects(executable_descriptor, target_state_snapshot)
  -> normalized full effect set
```

The descriptor executed by the adapter MUST be the descriptor whose effects were validated. Substitution after validation invalidates the evaluated plan revision and requires replan/re-evaluation.

## 5.2 Normative rule PA21-I2 — semantic effect equivalence

The uniquely bound effect-equivalence comparator SHALL compare:

1. the effect set declared/retained by the MappingPlan for semantic validation; and
2. the normalized full effect set derived from the executable descriptor.

Every derived effect MUST satisfy exactly one of:

- it is semantically equivalent to one declared effect; or
- the selected versioned adapter contract classifies its exact normalized effect kind/path as **non-semantic bookkeeping**.

Every declared semantic effect MUST have an equivalent derived effect.

Otherwise:

```text
=> FAIL: EXECUTABLE_EFFECT_MISMATCH
```

## 5.3 Normative rule PA21-I3 — bookkeeping ownership and boundary

Only the selected versioned adapter contract may classify an effect as non-semantic bookkeeping. Profile rules, validators, and action authors SHALL NOT self-classify unmatched mutations as bookkeeping.

A bookkeeping classification is valid only if the adapter contract guarantees that the effect does not alter any of the following:

- a SOL-addressable semantic value or binding;
- SOL schema/model-instance identity correspondence;
- resolved semantic scope/selection membership;
- formulation or equation meaning;
- capability/representability evidence;
- any backend setting that can change the simulated semantic result represented by the action.

Bookkeeping effects remain available to runtime ordering/collision checks if they mutate shared runtime state; they are excluded only from semantic equivalence accounting.

## 5.4 Positive boundary case

A COMSOL-like action descriptor creates a feature and also increments an adapter-local generated-tag counter. The selected adapter contract classifies the counter mutation as bookkeeping, and `describe_effects` derives both the feature creation and the counter update.

```text
feature creation <-> declared semantic effect
counter update   -> contract-authorized bookkeeping
=> PASS
```

## 5.5 Negative boundary case

The descriptor also changes the boundary selection from `{3}` to `{3,4}`, but the declared effects omit the selection change and the adapter contract does not classify selection mutation as bookkeeping.

```text
unmatched semantic mutation => FAIL: EXECUTABLE_EFFECT_MISMATCH
```

---

# 6. Ordering semantics, idempotency, exclusivity, and atomicity

The v0.2 rule that noncommutativity alone does not create direction remains unchanged. `must_precede` edges require versioned provenance-bearing evidence.

## 6.1 Normative rule PA21-I4 — state-independent idempotency requires adapter guarantee

`state_independent_idempotent` SHALL be accepted only when the selected adapter contract contains a versioned guarantee for the executable descriptor kind and normalized parameter class on the resolved target component/release.

A bare author/profile declaration without that adapter guarantee is malformed:

```text
=> FAIL: IDEMPOTENCY_GUARANTEE_MISSING
```

## 6.2 Normative rule PA21-I5 — state-dependent idempotency

For state-dependent idempotency, the adapter contract SHALL bind exactly one idempotency procedure/version. The procedure receives:

- canonical executable descriptor fingerprint;
- normalized effect fingerprint set;
- target component identity/release;
- immutable pre-state fingerprint;
- declared postcondition.

It SHALL return `idempotent`, `not-idempotent`, or `indeterminate`.

- missing pre-state evidence => `BLOCKED`;
- missing/ambiguous procedure => `FAIL`;
- complete evidence + comparator `indeterminate` => `INDETERMINATE`.

Actions MAY be coalesced only when descriptor equivalence, effect equivalence, component binding, applicable idempotency guarantee, and pre-state context all agree.

## 6.3 Normative rule PA21-I6 — exclusivity source

A resource/producer is treated as exclusive only if the selected adapter contract/version declares exclusivity for the normalized resource kind/slot under the resolved target component, or if the executable descriptor contract explicitly declares exclusive production for its canonical produced handle.

No validator heuristic such as “one object normally has one creator” is permitted.

When exclusivity is required but no canonical exclusivity declaration exists, duplicate producers SHALL NOT be silently coalesced. If both claim the same canonical produced handle, validation fails:

```text
=> FAIL: DUPLICATE_PRODUCER_UNRESOLVED
```

## 6.4 Normative rule PA21-I7 — atomic compound action

A compound executable descriptor may be treated as atomic only when the selected adapter contract guarantees **semantic all-or-none visibility**:

```text
success => all declared semantic effects are committed
failure => none of the declared semantic effects are externally visible
```

Residual non-semantic bookkeeping is permitted only when already classified under PA21-I3 and when it cannot become a prerequisite or semantic input to another action in the same plan revision.

If the adapter cannot guarantee semantic all-or-none visibility, the action SHALL be decomposed. If required decomposition is unavailable:

```text
=> FAIL: ATOMICITY_CONTRACT_INSUFFICIENT
```

## 6.5 Positive boundary case

A state-independent `create-if-absent` descriptor is covered by an adapter guarantee stating that executing it twice against the same component/release yields the same resource and semantic effects. Two actions have identical normalized descriptors/effects.

```text
adapter guarantee present
same canonical produced handle
exclusive production declared
=> deterministic coalescing allowed => PASS
```

## 6.6 Negative boundary case

Two identical-looking create descriptors target an API that allocates a fresh backend tag on every call. No state-independent guarantee exists.

```text
author says idempotent, adapter does not guarantee it
=> FAIL: IDEMPOTENCY_GUARANTEE_MISSING
```

---

# 7. Plan-level PASS/FAIL/BLOCKED/INDETERMINATE aggregation

## 7.1 Normative rule EL21-I1 — canonical precedence

All independently evaluable validation checks for one immutable MappingPlan revision SHALL contribute their decisions. The canonical plan validation decision is the maximum under this strict precedence:

```text
FAIL > BLOCKED > INDETERMINATE > PASS
```

Therefore:

- any `FAIL` => plan validation decision `FAIL`;
- else any `BLOCKED` => `BLOCKED`;
- else any `INDETERMINATE` => `INDETERMINATE`;
- else `PASS`.

Fail-fast execution MAY be used internally for performance only if the published canonical plan decision remains `FAIL` and diagnostics already established before termination are preserved. An implementation SHALL NOT publish `BLOCKED` or `INDETERMINATE` when a known validation `FAIL` exists in the same immutable revision.

Representability aggregation occurs only after plan validation is `PASS` and evaluation reaches `complete`.

## 7.2 Positive boundary case

One obligation is complete/PASS and one requires an external selection snapshot.

```text
{PASS, BLOCKED} => plan decision BLOCKED
```

## 7.3 Negative boundary case

One action descriptor is malformed (`FAIL`) while another obligation lacks a runtime snapshot (`BLOCKED`).

```text
{FAIL, BLOCKED} => plan decision FAIL
```

A validator returning `BLOCKED` is non-conforming.

---

# 8. Evaluation completion, representability, and execution permission

The v0.2 separation remains:

```text
validation decision
!= evaluation lifecycle state
!= terminal representability outcome
!= execution permission
```

## 8.1 Normative rule EL21-I2 — representability precondition

A terminal representability outcome may be produced only when:

```text
plan validation decision == PASS
AND
evaluation state == complete
```

Otherwise no terminal representability outcome is asserted for that revision.

## 8.2 Normative rule EL21-I3 — deterministic loss policy composition

Every identified semantic loss SHALL have a canonical loss key normalized from:

```text
(source model-instance identity,
 obligation key,
 canonical lost-semantic-aspect identity/path)
```

The key SHALL be independent of declaration and serialization order.

Each applicable Profile execution policy evaluates each canonical loss key to `allow` or `deny`. Policies compose conjunctively:

```text
effective permission for a loss
= allow only if every applicable policy returns allow
```

- any `deny` => denied;
- no applicable explicit allowance => denied by default;
- policy declaration order has no effect.

Thus “stricter Profile policy” means additional conjunctive restriction, not ordered override.

Execution permission is then:

- `exact` or `transformed`: permitted if no independent execution blocker exists;
- `lossy`: permitted only if every canonical loss key is effectively allowed;
- `unsupported`: not permitted.

## 8.3 Positive boundary case

A lossy mapping has one canonical loss `L1`. Two applicable policies both allow `L1`.

```text
allow AND allow => execution permitted
```

## 8.4 Negative boundary case

The same mapping has policy A allowing `L1` and policy B denying `L1`.

```text
allow AND deny => denied
```

Reversing policy declaration order does not change the result.

---

# 9. Component-keyed BackendTarget requirement and resolution

## 9.1 Normative rule BT21-I1 — component identity

Every target component in a single- or multi-product target SHALL have a Profile-local stable `component_id`. All release requirements, resolved releases, capability requirements/observations, formulation bindings, adapter contracts, and PlanAction bindings SHALL be keyed to exactly one `component_id`.

For a single-component target, the same rule applies with exactly one component.

For a composite target, unscoped top-level release, capability, or formulation records are invalid:

```text
=> FAIL: TARGET_COMPONENT_SCOPE_MISSING
```

## 9.2 Normative rule BT21-I2 — component adapter contracts and orchestration

Each component SHALL name exactly one required adapter contract identity/version constraint. `ResolvedBackendTarget` SHALL provide exactly one concrete adapter build satisfying that requirement for the same component.

A composite target SHALL additionally identify exactly one orchestration component by `component_id`. Its adapter contract is the orchestration adapter contract for the composite plan. No implicit relation to another component adapter is assumed.

Every PlanAction SHALL bind to exactly one component. A cross-component transfer action SHALL identify its source component, destination component, and orchestration component in its executable descriptor; all three references must resolve.

## 9.3 Normative rule BT21-I3 — release matching per component

For every component `C`:

```text
ResolvedBackendTarget[C].actual_release
MUST satisfy
BackendTargetRequirement[C].release_compatibility
```

- unresolved/missing runtime release observation => `BLOCKED`;
- resolved release outside required range => target mismatch `FAIL` before backend execution;
- malformed/ambiguous release matcher => `FAIL`.

No release from one component may satisfy another component's requirement.

## 9.4 Normative rule BT21-I4 — capability canonicalization and scope

Capabilities are adapter-owned canonical identities under a versioned component adapter contract. Aliases SHALL normalize to one canonical capability identity before union/intersection.

For each component `C`, effective required capabilities are exactly the normalized union of:

1. target-floor requirements scoped to `C`;
2. capabilities from selected MappingRules whose realized actions bind to `C`;
3. capabilities required by formulation bindings scoped to `C`;
4. orchestration requirements only when `C` is the orchestration component.

Requirements for the same canonical capability identity SHALL intersect their version/range constraints. An empty requirement intersection is:

```text
=> FAIL: CAPABILITY_REQUIREMENT_CONFLICT
```

Runtime capability observations are also keyed to `C`.

- required capability observation missing/unresolved => `BLOCKED`;
- authoritative resolved absence => representability item `unsupported` after successful validation/completion;
- ambiguous alias/canonicalization => `FAIL`.

## 9.5 Normative rule BT21-I5 — formulation ownership and relevant component derivation

SOL `MathModel`/`Study` semantics remain authoritative. A backend formulation record is only a realization binding.

A SOL formulation obligation is relevant to a target component **iff** at least one selected MappingRule realization for that obligation lowers to a PlanAction bound to that component. The relevant component set is therefore derived from the canonical selected-rule/action graph, not from vendor heuristics.

For each pair `(SOL formulation obligation, relevant component_id)`, exactly one backend formulation binding SHALL resolve through the selected adapter contract.

- zero binding => `BLOCKED` only when required external binding evidence is unresolved; otherwise `FAIL: FORMULATION_BINDING_MISSING`;
- multiple competing bindings => `FAIL: FORMULATION_BINDING_AMBIGUOUS`;
- one binding semantically incompatible according to the uniquely bound formulation comparator => representability `unsupported` after evaluation completes;
- comparator present with complete evidence but undecidable => `INDETERMINATE`.

A SOL formulation may bind to multiple components only when the canonical action graph makes all of those components relevant. No component is added merely because it belongs to the composite target.

## 9.6 Positive boundary case

Composite target:

```text
component mechanical:
  release = 2026 R1
  capabilities = {thermal}

component orchestrator:
  release = 1.4
  capabilities = {data-transfer}
```

A thermal action binds to `mechanical`; a transfer action binds through the orchestration descriptor. Thermal capability lookup is performed only on `mechanical`, transfer capability only on `orchestrator` (plus explicitly required source/destination transfer compatibility if declared by their adapter contracts).

```text
all component-keyed requirements satisfied => PASS
```

## 9.7 Negative boundary case

Capability `thermal` appears in an unkeyed composite capability list and can be associated either with `mechanical` or `orchestrator`.

```text
unscoped composite capability => FAIL: TARGET_COMPONENT_SCOPE_MISSING
```

No validator may guess the component.

---

# 10. Deterministic validation sequence

The v0.2 sequence is retained with the following tightened ordering constraints:

```text
1. Validate Profile/adapter contract structure and component scoping
2. Resolve BackendTargetRequirement -> ResolvedBackendTarget per component
3. Resolve exact comparator/normalizer/idempotency registry bindings
4. Evaluate MappingRule source/applicability
5. Generate four-field MappingClaims
6. Resolve executable descriptors and derive full normalized effect sets
7. Verify declared semantic effects against derived effects; classify only contract-authorized bookkeeping
8. Resolve producers/prerequisites/exclusivity/idempotency evidence
9. Build action dependency graph using requires/producers and provenance-bearing must_precede rules
10. Reject cycles
11. Enumerate complete unordered effect-pair universe
12. Prune only provenance-proven disjoint pairs
13. Compare all retained pairs and validate ordering/collisions
14. Aggregate validation decision using FAIL > BLOCKED > INDETERMINATE > PASS
15. Only on PASS, finish item representability evaluation and lifecycle completion
16. Aggregate terminal representability
17. Evaluate execution permission using conjunctive canonical-loss policy
```

Declaration order, registry iteration order, component ordering, and serialization ordering SHALL NOT affect any step.

---

# 11. MappingRule and MappingClaim shape re-confirmation

## MappingRule

**Five-field shape remains sufficient at contract level.**

- action/effect/executable descriptor data remain within or derive from `realization`;
- rule capability requirements remain in `capabilities`;
- target component and runtime identity remain Profile/BackendTarget data.

No remaining counterexample requires a sixth field.

## MappingClaim

**Four-field shape remains sufficient at contract level.**

- effects/executable actions derive from `realization`;
- comparator and evaluation evidence belong to MappingPlan evidence;
- capabilities remain rule/target concerns;
- target component identity is resolved by the realization/action binding and BackendTarget, not added as a fifth claim field.

This remains subject to executable validation proving no hidden required side channel is needed.

---

# 12. Contract re-review finding matrix

| Re-review finding | v0.2.1 status | Revision disposition |
|---|---|---|
| RE-01 resource canonicalization / alias candidate completeness | **Resolved** | all unordered effect pairs retained unless provenance-proven disjoint |
| RE-02 missing comparison data boundary | **Resolved** | v0.2 rule unchanged; FAIL/BLOCKED/INDETERMINATE origin boundary preserved |
| RE-03 comparator evidence / selection ambiguity | **Resolved** | exact registry key; zero/multiple match is FAIL |
| RE-04 operation vocabulary ownership | **Resolved** | v0.2 rule unchanged; operations remain MappingPlan/Adapter IR |
| PA-01 executable descriptor/effect coverage | **Resolved** | deterministic `describe_effects`; unique equivalence comparator; adapter-only bookkeeping classification |
| PA-02 noncommutativity vs ordering direction | **Resolved** | v0.2 rule unchanged |
| PA-03 state-dependent idempotency / exclusivity | **Resolved** | adapter guarantee/procedure required; exclusivity source canonicalized |
| PA-04 compound transaction atomicity | **Resolved** | semantic all-or-none failure contract required |
| EL-01 blocked/indeterminate/tooling boundary | **Resolved** | v0.2 rule unchanged |
| EL-02 FAIL with BLOCKED/INDETERMINATE aggregation | **Resolved** | canonical precedence `FAIL > BLOCKED > INDETERMINATE > PASS` |
| EL-03 completion vs execution permission / stricter policy | **Resolved** | canonical loss keys; conjunctive allow/deny composition; default deny |
| EL-04 transition determinism | **Resolved** | v0.2 immutable-revision rule unchanged |
| BT-01 requirement/runtime identity split | **Resolved** | v0.2 rule unchanged |
| BT-02 relevant formulation component derivation | **Resolved** | derived from selected rule -> PlanAction component bindings |
| BT-03 capability identity/scope | **Resolved** | canonical capability identity + per-component union/intersection |
| BT-04 multi-product field association/orchestration | **Resolved** | all records component-keyed; exact orchestration component/adapter |
| C-ADR-01 candidate-pair pruning counterexample | **Resolved** | different keys cannot justify pruning |
| C-ADR-02 comparator registry ambiguity | **Resolved** | exactly one bound implementation required |
| C-ADR-03 FAIL mixed with BLOCKED | **Resolved** | FAIL has canonical precedence |
| C-ADR-04 composite target association | **Resolved** | unscoped composite target fields are validation FAIL |
| MappingRule five-field preservation | **Resolved at contract level** | maintained unchanged |
| MappingClaim four-field preservation | **Resolved at contract level** | maintained unchanged; executable validation still required |

**Regression:** none intentionally introduced. Any implementation that reintroduces order-based selection, implicit target-component association, or unbound comparator choice is non-conforming.

---

# 13. Boundary decision summary

For identical immutable model/Profile/adapter-contract/target snapshots and registry contents:

| Boundary | Canonical result |
|---|---|
| malformed descriptor + unresolved runtime snapshot | `FAIL` |
| valid plan + missing external selection snapshot | `BLOCKED` |
| all evidence present + uniquely selected comparator cannot decide | `INDETERMINATE` |
| all checks decide successfully | `PASS` |
| unproved-disjoint effect pair | must be retained; never auto-PASS |
| zero/multiple comparator matches | `FAIL` |
| composite capability without component key | `FAIL` |
| actual required capability authoritatively absent | validation may PASS; terminal item representability becomes `unsupported` after completion |

---

# 14. Final contract re-review gate

**Contract-only status proposed by Research Lab: READY FOR FINAL CONTRACT RE-REVIEW.**

The remaining issues are no longer intended to be architecture/contract design questions. They are execution-validation requirements.

## Execution-validation-only items remaining

1. machine-readable schemas for five-field `MappingRule`, four-field `MappingClaim`, `RealizationEffect`, `EffectComparison`, `PlanAction`, lifecycle/evaluation records, and component-keyed BackendTarget records;
2. comparator/normalizer/idempotency registry implementation enforcing exactly-one resolution;
3. MappingPlan generator proving complete effect-pair enumeration and order-independent canonical plans;
4. executable descriptor `describe_effects` and effect-equivalence implementation;
5. target resolver proving component-keyed release/capability/formulation/orchestration behavior;
6. MOOSE Thermal executable fixture exercising prerequisite, action ordering, effect coverage, and idempotency;
7. COMSOL or Ansys Plasma fixture exercising alias overlap, N:1 realization, multi-component/formulation/capability behavior;
8. repeated-run tests showing serialization/declaration/component ordering does not alter decisions or canonical plan evidence.

A final contract re-review PASS would authorize ADR drafting consideration, **not** architecture freeze. Architecture freeze still requires the executable validation evidence above.