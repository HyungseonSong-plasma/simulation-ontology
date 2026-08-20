# SOL v0.1 ADR-0010 Supplement Remediation Proposal v0.2

**Status:** Research proposal revision; no accepted ADR modified  
**Date:** 2026-08-20  
**Branch:** `validation/sol-v0.1-independent-review`  
**Revises:** [`sol-v0.1-adr0010-supplement-remediation-proposal-v0.1.md`](sol-v0.1-adr0010-supplement-remediation-proposal-v0.1.md)  
**Review basis:** [`sol-v0.1-remediation-proposals-independent-review.md`](../validation/sol-v0.1-remediation-proposals-independent-review.md)  
**Architecture baseline:** [`sol-v0.1-ar01-ar04-architecture-reassessment-v0.1.md`](sol-v0.1-ar01-ar04-architecture-reassessment-v0.1.md)

## 1. Revision objective

This revision addresses the independent review's unresolved contract ambiguities without changing accepted ADRs and without adding a sixth `MappingRule` field or a fifth `MappingClaim` field.

The public shapes remain:

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

The remediation remains internal to `realization`, `MappingPlan`, and Profile/Adapter target metadata.

The revised contracts cover:

- canonicalization and alias-aware comparison of `RealizationEffect` resources;
- comparator provenance and deterministic missing-data treatment;
- executable `PlanAction` descriptors and effect coverage;
- evidence-backed ordering semantics;
- backend-state-dependent idempotency;
- strict `blocked` versus `indeterminate` semantics;
- item-to-plan representability aggregation;
- separation of evaluation completion from execution permission;
- separation of backend target requirements from observed runtime identity;
- formulation ownership and multi-product target semantics.

---

## 2. Common validation decision contract

The MappingPlan validator SHALL use the following decision meanings for contract checks. This is a validation/reporting convention, not a new SOL semantic type hierarchy.

```text
PASS
FAIL
BLOCKED
INDETERMINATE
```

### V-I1 — PASS

`PASS` means all inputs required by the checked invariant are present, the registered normative procedure completed, and the invariant is satisfied.

### V-I2 — FAIL

`FAIL` means either:

1. the invariant is determinately violated; or
2. the Profile/Adapter/MappingPlan object is malformed with respect to a required contract; or
3. a required comparator/executable implementation declared by the selected adapter is missing.

A missing implementation is not semantic indeterminacy.

### V-I3 — BLOCKED

`BLOCKED` means the invariant cannot yet be evaluated because a **declared external or resolvable prerequisite** is absent or temporarily unavailable, and obtaining that prerequisite may change the decision without changing semantic policy.

Examples: unresolved external backend object, unavailable runtime capability snapshot, target component not yet resolved, incomplete dynamic selection resolution.

### V-I4 — INDETERMINATE

`INDETERMINATE` means:

1. the contract is well formed;
2. all declared required inputs/evidence are present; and
3. the selected versioned comparator legitimately cannot decide the semantic/effect relation.

Missing comparison fields that the normalization contract requires are `FAIL`, not `INDETERMINATE`. Missing external evidence required to populate those fields is `BLOCKED`.

### Boundary examples

| Input condition | Unique result |
|---|---|
| Effect comparator has all required normalized fields and proves disjoint resources | PASS |
| Same canonical resource/slot/scope, incompatible payloads | FAIL |
| Selection resolution is declared external and the runtime selection snapshot is unavailable | BLOCKED |
| Selection snapshot is complete but the registered comparator returns `indeterminate` for overlap | INDETERMINATE |
| Adapter declares comparator `effect-v2` but implementation is not installed | FAIL |

---

# 3. AR-01 revision — `RealizationEffect` normalization and cross-claim collision

## 3.1 Architectural placement

`RealizationEffect` remains a **MappingPlan/runtime IR construct** derived from `MappingClaim.realization`. Backend tags, property names, API operations, and selection representations remain adapter-owned. No backend-native vocabulary enters SOL Core ontology semantics.

## 3.2 Minimum normalized effect contract

A selected adapter MUST normalize every externally observable realization mutation into one or more effects with this minimum information:

```text
RealizationEffect
├── effect_id
├── resource
├── slot?
├── scope?
├── payload_fingerprint?
└── normalization_provenance
```

`operation` MAY exist in adapter IR, but Core architecture SHALL NOT prescribe a universal `create/write/attach/select/remove` operation enumeration. Operation vocabulary is adapter/MappingPlan-IR owned.

### `resource`

`resource` MUST include a **plan-canonical resource address** and resolution status:

```text
resource
├── canonical_plan_key
├── adapter_resource_kind
├── runtime_aliases[]?
└── resolution_provenance
```

The canonical plan key is stable within one immutable MappingPlan revision. It is not a SOL schema identity and not necessarily a backend runtime tag.

### RE-I1 — Canonicalization before comparison

Before two effects can be declared disjoint, each resource MUST be normalized by the same selected adapter contract/version against the same `ResolvedBackendTarget` snapshot.

Two syntactically different resource references SHALL be treated as equivalent when adapter normalization proves that they resolve to the same runtime object or the same canonical plan resource.

Resource inequality before canonicalization SHALL NOT establish disjointness.

### RE-I2 — Alias evidence

If two resource references may alias, normalization MUST emit one of:

```text
same
separate
overlapping
unresolved
```

with evidence and normalization provenance.

`unresolved` because runtime alias data is unavailable => `BLOCKED`.

A complete alias snapshot for which the registered comparator cannot decide => `INDETERMINATE`.

### RE-I3 — Comparison sufficiency

For every pair of effects selected as potentially interacting, the normalization contract MUST identify which of `slot`, `scope`, payload/effect identity, or adapter-specific normalized fields are required to decide overlap/compatibility.

If a required field is absent because the Profile/Adapter produced a malformed effect => `FAIL`.

If a required field depends on a declared external binding that has not resolved => `BLOCKED`.

If all required fields are present but the comparator cannot decide => `INDETERMINATE`.

## 3.3 Typed comparison result and provenance

Every effect comparison SHALL produce a record logically equivalent to:

```text
EffectComparison
├── overlap          = yes | no | indeterminate
├── compatibility    = compatible | conflict | indeterminate
├── ordering         = commutes | order_required | conflict | indeterminate
├── reason_code
├── evidence[]
└── comparator_provenance
    ├── comparator_id
    ├── comparator_version
    └── adapter_identity
```

This is comparison evidence, not a new `MappingClaim` field.

### RE-I4 — Comparator provenance

A PASS/FAIL/INDETERMINATE collision decision is conformant only if comparator identity/version and input normalization provenance are recorded. Two validators using different comparator versions MUST therefore produce distinguishable evidence rather than silently appearing equivalent.

### RE-I5 — Missing comparator

If the selected adapter contract requires a comparator but no implementation for the declared comparator identity/version exists, validation SHALL return `FAIL` with classification `Adapter/Validation-tooling contract defect`.

It SHALL NOT return `INDETERMINATE`.

## 3.4 Collision invariant

MappingPlan composition SHALL execute both checks:

```text
A. obligation-key conflict
same source + same obligation + incompatible realization
=> FAIL / MappingConflict

B. cross-claim effect collision
any effects that canonically overlap and are incompatible
=> FAIL / MappingConflict
```

Different source/obligation identities never exempt effects from stage B.

If overlap or compatibility is `INDETERMINATE`, the plan cannot complete evaluation.

## 3.5 Thermal boundary examples

### Positive — aliases are disjoint

```text
Effect A resource alias -> material-domain-1
Effect B resource alias -> material-domain-2
adapter alias comparator proves separate
```

Result: `PASS` for collision check.

### Negative — distinct names alias the same effective property

```text
A: resource alias `solid-k-from-material`
B: resource alias `solid-k-user`
canonicalization -> same Solid.k on domain-1
payloads incompatible
```

Result: `FAIL`.

### Blocked

Runtime Named Selection referenced by one effect has not yet been resolved to an entity set.

Result: `BLOCKED`.

### Indeterminate

Both selection snapshots are present, but an adapter-specific geometric equivalence comparator explicitly returns `indeterminate` for overlap.

Result: `INDETERMINATE`.

## 3.6 Plasma boundary examples

Two wall effects use different backend selection tags. Canonicalization proves both contain boundary identity `wall-5`; their electron-flux payloads are incompatible.

Result: `FAIL` even though source and obligation differ.

If all selected boundaries resolve and are disjoint, result is `PASS`.

---

# 4. AR-02 revision — executable `PlanAction`, ordering, state-dependent idempotency

## 4.1 Minimum action contract

`MappingRule.realization` SHALL lower to action-level MappingPlan IR. Each executable action SHALL contain or normatively reference:

```text
PlanAction
├── action_id
├── requires[]
├── produces[]
├── effects[]
├── executable_descriptor
└── idempotency_contract
```

Ordering is represented by evidence-bearing graph edges derived as described below rather than by declaration order.

## 4.2 Executable descriptor

`executable_descriptor` is adapter-owned and MUST be sufficient for the selected adapter to identify the actual operation and normalized inputs to execute.

It MUST include adapter implementation identity/version or a stable operation reference resolvable through that adapter.

### PA-I1 — Executability

Once `ResolvedBackendTarget` is complete, every non-analysis-only `PlanAction` MUST have a resolvable executable descriptor.

Missing descriptor after target resolution => `FAIL`.

Descriptor resolution blocked only because the declared target runtime component has not resolved => `BLOCKED`.

### PA-I2 — Effect coverage

The adapter MUST provide a deterministic `describe_effects(executable_descriptor, normalized_inputs, target_snapshot)` result.

The declared `PlanAction.effects` MUST be equivalent to the externally observable mutation surface returned by that procedure, modulo explicitly declared non-semantic bookkeeping effects.

Mismatch => `FAIL`.

This prevents validation of one effect set followed by execution of a different mutation.

## 4.3 Ordering semantics

### PA-I3 — Dependency edges

`A -> B` SHALL be added when `B.requires` consumes a handle produced by `A`.

### PA-I4 — Noncommutativity is not direction

```text
noncommuting(A,B) != must_precede(A,B)
```

An overlap comparator may prove that order matters, but this does not justify a direction.

A directional edge may be added only when a versioned adapter/Profile rule supplies `must_precede(A,B)` evidence with provenance based on backend lifecycle or semantic intent.

If effects require ordering but no direction is justified, validation SHALL return `FAIL / mutation-order conflict` rather than choosing declaration order.

### PA-I5 — Declaration order

Source file order, rule order, claim order, or generated list order SHALL NOT supply a semantic ordering edge.

### PA-I6 — Cycles

After prerequisite and justified `must_precede` edges are added, the graph MUST be acyclic.

Cycle => `FAIL`.

An adapter MAY expose a compound executable action only if it provides one atomic descriptor whose externally observable effects, prerequisite set, failure semantics, and idempotency are validated as one unit. A compound action SHALL NOT be used merely to hide an unresolved cycle.

## 4.4 Producer invariants

### PA-I7 — Unresolved prerequisite

Every required handle MUST resolve to:

- an external binding in the resolved target snapshot; or
- exactly one accepted producer action; or
- multiple producer actions proven equivalent and deterministically coalesced.

External binding unavailable => `BLOCKED`.

No producer and no declared external source in an otherwise complete plan => `FAIL`.

### PA-I8 — Duplicate producer

Multiple actions producing one exclusive canonical handle without proof of equivalence => `FAIL`.

If equivalence depends on unresolved runtime identity => `BLOCKED`.

If all evidence is present but equivalence comparator returns indeterminate => `INDETERMINATE`.

## 4.5 State-dependent idempotency

A boolean `idempotent` flag is insufficient. The idempotency contract MUST state either:

```text
state_independent_idempotent
```

or:

```text
state_dependent
├── precondition/comparator identity
├── required backend-state evidence
└── equivalent-postcondition rule
```

### PA-I9 — Coalescing repeated actions

Two repeated actions MAY be coalesced only if all are true:

1. executable descriptors and normalized semantic inputs are equivalent;
2. target component/runtime identity is equivalent;
3. declared effects compare compatible/equivalent;
4. the idempotency precondition is satisfied for the observed backend-state snapshot.

Missing required backend-state snapshot => `BLOCKED`.

Complete snapshot but idempotency comparator returns indeterminate => `INDETERMINATE`.

Precondition false or non-idempotent duplicate where one exclusive mutation is expected => `FAIL`.

## 4.6 Thermal boundary examples

### PASS

MOOSE transient plan has separate actions for `T`, `HeatConduction`, `HeatConductionTimeDerivative`, BCs, and transient execution; density and specific heat have valid producers; justified prerequisite edges are acyclic; action descriptors match declared effects.

### FAIL — direction invented from noncommutativity

Two actions write the same property source with incompatible semantic intent. Comparator returns `order_required`, but no semantic/backend lifecycle rule proves which write must win.

Result: `FAIL`, not arbitrary A→B or B→A.

### BLOCKED — state-dependent idempotency

Repeated creation targets an external object whose existence snapshot cannot currently be read.

Result: `BLOCKED`.

### INDETERMINATE

Snapshot is available but adapter's declared idempotency comparator cannot decide whether an existing externally created object is equivalent to the requested object.

Result: `INDETERMINATE`.

## 4.7 Plasma boundary example

COMSOL-like plasma physics can be created first, species/reaction actions can consume its handle, and a later configuration action can consume the completed reaction-set handle. Create and configure are separate executable actions, eliminating false object-level cycles.

---

# 5. AR-04 revision — deterministic evaluation lifecycle and plan aggregation

## 5.1 Separate axes

The architecture SHALL keep three distinct concepts:

```text
MappingPlanEvaluationState
  pending | blocked | indeterminate | complete

RepresentabilityOutcome
  exact | transformed | lossy | unsupported

ExecutionPermission
  permitted | prohibited
```

`ExecutionPermission` is operational policy derived after evaluation. It is not a representability class.

## 5.2 `blocked` versus `indeterminate`

### EL-I1 — Blocked

A plan/item is `blocked` only when a **declared required external/resolvable input** is absent or temporarily unavailable.

Examples:

- target component or actual release not resolved;
- runtime capability discovery transport failed/timed out without an authoritative absence result;
- external backend binding unresolved;
- dynamic selection snapshot unavailable.

Malformed Profile/Adapter contracts are `FAIL`, not `blocked`.

Authoritative evidence that a required capability is absent is not `blocked`; evaluation can complete and that required obligation becomes `unsupported`.

### EL-I2 — Indeterminate

A plan/item is `indeterminate` only when every declared required input is present and the registered comparator/evaluator legitimately returns no decision.

Missing comparator implementation => `FAIL`.

### EL-I3 — Pending

`pending` is the initial state of a new evaluation revision. A finished synchronous validation run SHALL NOT leave a final plan in `pending`.

## 5.3 Immutable evaluation revision

For one immutable MappingPlan evaluation revision, only these transitions are allowed:

```text
pending -> blocked
pending -> indeterminate
pending -> complete
```

`blocked` or `indeterminate` resolution creates a **new evaluation revision** referencing the prior revision; the historical revision is not mutated back to pending.

Runtime drift after `complete` invalidates the evaluated target snapshot and requires a new plan/evaluation revision.

## 5.4 Per-obligation representability

Representability SHALL first be evaluated per selected semantic mapping obligation, using obligation criticality declared by Profile policy:

```text
required
optional-nonsemantic
```

`optional-nonsemantic` is restricted to information whose omission does not weaken SOL semantic truth under ADR-0006. Semantic model obligations SHALL be `required`.

A required obligation reaches one of the four terminal outcomes only when its evaluation is complete.

## 5.5 Item-to-plan aggregation

After all required items complete, plan summary is deterministic:

```text
if any required item == unsupported -> plan outcome = unsupported
else if any required item == lossy   -> plan outcome = lossy
else if any required item == transformed -> plan outcome = transformed
else -> plan outcome = exact
```

Optional-nonsemantic items are reported separately and do not worsen semantic representability.

If any required item is `BLOCKED`, plan state = `blocked`.

Else if none are blocked and any required item is `INDETERMINATE`, plan state = `indeterminate`.

Else plan state = `complete` and the aggregation rule above SHALL produce exactly one plan outcome.

## 5.6 Evaluation completion versus execution permission

### EL-I4 — Completion

`complete` means evaluation terminated deterministically. It does **not** mean the plan is executable.

### EL-I5 — Permission

Execution permission is derived as follows unless stricter Profile policy applies:

```text
state != complete -> prohibited
outcome == exact -> permitted
outcome == transformed -> permitted
outcome == unsupported -> prohibited
outcome == lossy -> permitted only if explicit Profile/execution policy allows the identified losses
```

Therefore a plan may be:

```text
state = complete
outcome = unsupported
execution = prohibited
```

without semantic contradiction.

## 5.7 Boundary examples

| Input | Evaluation state | Outcome | Execution |
|---|---|---|---|
| All thermal obligations exact/transformed | complete | transformed | permitted |
| Required module authoritatively absent | complete | unsupported | prohibited |
| Runtime capability service timeout, no authoritative snapshot | blocked | none | prohibited |
| All evidence present, effect comparator genuinely undecidable | indeterminate | none | prohibited |
| Required MappingRule absent from a Profile that claims support and provides no explicit unsupported policy | validation `FAIL` | none | prohibited |
| Lossy unit/provenance projection explicitly accepted by policy | complete | lossy | permitted |
| Same lossy projection with policy forbidding loss | complete | lossy | prohibited |

---

# 6. Executable `BackendTarget` revision

## 6.1 Requirement/runtime separation

A Profile SHALL declare a backend target **requirement**, while adapter discovery SHALL produce an observed runtime target.

```text
BackendTargetRequirement
├── adapter_contract
├── components[]
├── release_constraints
├── formulation_bindings
└── capability_floor[]

ResolvedBackendTarget
├── adapter_build_identity
├── runtime_components[]
├── actual_releases
├── installed/licensed modules
├── discovered_capabilities
└── resolution_provenance
```

These are Profile/Adapter architecture contracts, not SOL Core semantic taxonomy.

## 6.2 Component and multi-product semantics

A `component` identifies one concrete product/application runtime requirement. A one-product target has one component. A coupled target MAY have multiple components.

### BT-I1 — No vendor umbrella target

A vendor/framework umbrella such as `Ansys` or generic `MOOSE` SHALL NOT satisfy executable target completeness unless it resolves through the selected adapter contract to concrete required components.

### BT-I2 — Multi-product coordination

For `components.count > 1`, the selected adapter contract MUST explicitly declare orchestration/cross-component capability.

Every `PlanAction.executable_descriptor` MUST bind to exactly one runtime component or to the declared orchestration adapter.

Semantic data transfer between components MUST be represented by explicit PlanActions/effects; implicit cross-product state transfer is prohibited.

If one required component is unresolved => `BLOCKED`.

If all components resolve but the orchestration adapter lacks a required transfer capability => evaluation completes `unsupported` for that required obligation.

## 6.3 Formulation ownership

SOL `Analysis` / `MathematicalModel` semantics remain authoritative. Backend target metadata SHALL NOT define an independent competing formulation truth.

`formulation_bindings` are mappings:

```text
SOL formulation identity -> backend formulation identifier
```

owned by Profile/Adapter mapping policy.

### BT-I3 — Formulation compatibility

For every SOL formulation used by selected rules, exactly one applicable backend formulation binding MUST be selected for each relevant target component.

A Profile that binds `Transient` SOL semantics to a backend `Steady-State Thermal` formulation without an explicit semantics-preserving transformation proof => `FAIL / Profile semantic conflict`.

If the Profile is well formed but the resolved product cannot provide any compatible backend formulation => `complete + unsupported`.

If runtime target identity required to choose among declared compatible formulations is unresolved => `BLOCKED`.

A fluid drift-diffusion plasma model SHALL NOT silently bind to a PIC formulation solely because both are plasma-capable.

## 6.4 Capability requirement derivation

Target-level capability requirements and MappingRule capabilities SHALL not compete.

Define:

```text
effective_required_capabilities
=
BackendTargetRequirement.capability_floor
UNION
capabilities of all selected MappingRules
UNION
capabilities required by selected formulation bindings/orchestration
```

This union is derived deterministically after rule selection.

### BT-I4 — Capability outcomes

- capability discovery snapshot unavailable => `BLOCKED`;
- required capability authoritatively absent => required obligation `unsupported`;
- capability present => continue evaluation;
- Profile names an unknown capability outside the selected adapter contract => `FAIL`.

## 6.5 Release identity

`release_constraints` are requirements. `actual_releases` belong only to `ResolvedBackendTarget`.

A plan SHALL compare actual runtime releases against declared constraints using the versioned adapter matcher required by ADR-0009.

Matcher implementation missing => `FAIL`.

Runtime release not yet discoverable => `BLOCKED`.

Runtime release known and outside allowed range => `complete + unsupported` unless Profile policy explicitly defines another valid target component.

## 6.6 Boundary examples

### PASS

Requirement: Ansys Mechanical Steady-State Thermal, allowed 2026 R1-compatible range. Resolved component and adapter build match, required thermal capabilities are present, and SOL steady analysis binds to steady-state thermal.

Result: target-resolution check `PASS`.

### FAIL

SOL model declares transient analysis; Profile explicitly maps it to Steady-State Thermal without transformation proof.

Result: `FAIL` Profile semantic conflict.

### BLOCKED

Requirement is well formed, but actual MOOSE application build/runtime identity is unavailable.

Result: `BLOCKED`.

### INDETERMINATE

All target/runtime data are present, but a declared versioned formulation-equivalence comparator returns `indeterminate` for a custom coupled formulation.

Result: `INDETERMINATE`.

### Multi-product positive

A coupled workflow declares two concrete components and an orchestration adapter; all cross-component transfer actions are explicit and capabilities resolve.

Result: `PASS` target-resolution check.

### Multi-product negative

Two products are declared but no orchestration capability/adapter exists while MappingPlan contains cross-component data dependencies.

Result: `complete + unsupported` if target is well formed and the capability is authoritatively absent; `FAIL` if the Profile falsely claims orchestration semantics without defining the required binding.

---

# 7. Deterministic remediation validation algorithm

For a given immutable SOL model, Profile, adapter contract/version, and target snapshot:

```text
1. Validate BackendTargetRequirement syntax/semantics.
2. Resolve ResolvedBackendTarget or return BLOCKED.
3. Verify adapter/matcher/comparator implementations; missing required implementation => FAIL.
4. Evaluate MappingRule source/applicability.
5. Generate four-field MappingClaims.
6. Lower claim.realization to PlanActions + RealizationEffects + executable descriptors.
7. Validate effect/action normalization completeness.
8. Canonicalize resource identities and aliases.
9. Resolve external prerequisites/producers.
10. Validate duplicate producers.
11. Derive prerequisite edges.
12. Compare overlapping effects.
13. Add only evidence-backed must_precede edges.
14. Validate idempotency under required backend-state snapshot.
15. Check DAG acyclicity.
16. Evaluate each required obligation representability.
17. Aggregate MappingPlanEvaluationState.
18. If complete, aggregate exact/transformed/lossy/unsupported.
19. Derive execution permission independently.
```

The same normalized inputs, adapter/comparator versions, and target snapshot MUST produce the same decision/evidence set. Declaration order or numeric priority SHALL NOT affect the result.

---

# 8. Re-check of public contracts

## MappingRule five-field shape

**Still sufficient.**

- executable/action/effect data remain inside or are derived from `realization`;
- environment requirements remain in Profile `BackendTargetRequirement`;
- mapping capability requirements remain in `capabilities`;
- no reviewed counterexample requires a sixth top-level field.

## MappingClaim four-field shape

**Still sufficient.**

- effects and executable actions remain inside/derived from `realization`;
- comparator evidence belongs to MappingPlan validation evidence;
- target/runtime identity belongs to Profile/Adapter target resolution;
- no reviewed counterexample requires a fifth required claim field.

This conclusion remains conditional on executable tests proving that no hidden side-channel metadata is needed.

---

# 9. Core/schema change versus normative clarification

| Item | Ownership | Revision type |
|---|---|---|
| MappingRule five fields | ADR-0010 public contract | no change |
| MappingClaim four fields | ADR-0010 public contract | no change |
| Resource canonicalization/alias rules | MappingPlan/Adapter IR | normative clarification + runtime schema |
| `EffectComparison` provenance | MappingPlan evidence | normative clarification |
| executable descriptor | MappingPlan/Adapter IR | normative clarification + runtime schema |
| evidence-backed ordering | MappingPlan validator | normative clarification |
| state-dependent idempotency | MappingPlan/Adapter IR | normative clarification + runtime schema |
| lifecycle aggregation | MappingPlan | normative clarification + state schema |
| execution permission | MappingPlan/runtime policy | normative clarification |
| target requirement/runtime separation | Profile/Adapter | Profile contract clarification + runtime schema |
| formulation bindings | Profile/Adapter mapping policy | normative clarification |
| multi-product components/orchestration | Profile/Adapter | Profile contract extension; no Core semantic type |

No backend-native object model is added to Core.

---

# 10. Validation review issue matrix

| Independent-review issue | Revision response | Status |
|---|---|---|
| RE-01 resource alias ambiguity | canonical plan resource key + alias resolution provenance + same/separate/overlapping/unresolved contract | **resolved at contract level** |
| RE-02 optional address fields / missing comparison data | normalization sufficiency rule distinguishes malformed=FAIL, external missing=BLOCKED, comparator undecidable=INDETERMINATE | **resolved at contract level** |
| RE-03 circular `semantics` field | replaced by versioned comparator procedures/results with explicit provenance/evidence | **resolved at contract level** |
| RE-04 operation vocabulary ownership | explicitly adapter/MappingPlan-IR owned, not Core semantic vocabulary | **resolved** |
| PA-01 no executable descriptor | mandatory resolvable executable descriptor + effect-coverage invariant | **resolved at contract level** |
| PA-02 noncommutativity does not determine direction | `must_precede` requires independent evidence; otherwise conflict | **resolved** |
| PA-03 context-sensitive idempotency | state-independent/state-dependent contract + snapshot/precondition rules | **resolved at contract level** |
| PA-04 compound transactions | compound action allowed only with atomic descriptor/effect/failure/idempotency proof | **resolved at contract level** |
| EL-01 blocked vs indeterminate ambiguity | strict external-missing vs evidence-complete-undecidable boundary; missing implementation=FAIL | **resolved** |
| EL-02 outcome aggregation | required-obligation results + deterministic worst-semantic-loss aggregation | **resolved at contract level** |
| EL-03 unsupported execution policy | completion and execution permission separated | **resolved** |
| EL-04 transition determinism | immutable evaluation revision; pending has one-way terminal transition | **resolved** |
| BT-01 requirement vs resolved runtime | separate requirement and resolved target contracts | **resolved** |
| BT-02 formulation duplication | formulation expressed only as binding from SOL semantic formulation to backend formulation | **resolved** |
| BT-03 capability duplication | effective requirement = target floor ∪ selected-rule capabilities ∪ formulation/orchestration capabilities | **resolved** |
| BT-04 multi-product target | component set + explicit orchestration/cross-component action semantics | **resolved at contract level** |
| Executable evidence for all above | requires schemas, validator, adapter fixtures, and backend artifact path | **unresolved — executable validation required** |

---

# 11. Contract re-review gate

This proposal is ready for a **short contract re-review** if Validation Lab limits that review to determinism and architecture ownership, because every prior textual ambiguity has an explicit normative result rule.

It is **not** evidence for architecture freeze. After contract re-review, executable validation still requires at minimum:

1. machine-readable `RealizationEffect` and `PlanAction` schemas;
2. comparator registry with version/provenance records;
3. target requirement/resolution implementation;
4. MappingPlan generator and validator implementing the exact decision ordering above;
5. deterministic fixtures for PASS/FAIL/BLOCKED/INDETERMINATE boundaries;
6. one concrete Thermal MappingPlan and generated backend artifact path;
7. one Plasma plan exercising selection alias/collision and target formulation binding.

**Revision verdict:** ADR-0010 supplement proposal may enter short contract re-review. SOL v0.1 architecture freeze remains blocked pending executable validation and the separate AR-03 remediation.
