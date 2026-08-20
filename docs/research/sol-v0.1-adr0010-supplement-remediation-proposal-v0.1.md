# SOL v0.1 ADR-0010 Supplement Remediation Proposal v0.1

**Status:** Research proposal; no accepted ADR modified  
**Date:** 2026-08-20  
**Branch:** `validation/sol-v0.1-independent-review`  
**Baseline:** [`sol-v0.1-ar01-ar04-architecture-reassessment-v0.1.md`](sol-v0.1-ar01-ar04-architecture-reassessment-v0.1.md)

## 1. Purpose

This proposal addresses AR-01, AR-02, AR-04, and the executable BackendTarget gap while preserving the accepted top-level shapes of ADR-0010 wherever the counterexamples permit it.

The proposal does **not** modify ADR-0010. It defines the minimal normative supplement that should be reviewed before any ADR change is made.

The following remain unchanged:

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

No sixth MappingRule field and no fifth MappingClaim field are required by the reviewed counterexamples.

---

# 2. Canonical remediation model

```text
SOL Model
   ↓
Profile + BackendTarget
   ↓
MappingRules
   ↓
MappingClaims
   ↓ normalize realization
RealizationEffects + PlanActions
   ↓
MappingPlan validation
   ├── prerequisite resolution
   ├── producer uniqueness
   ├── cycle detection
   ├── cross-claim effect collision
   ├── mutation ordering
   ├── idempotency checks
   └── capability/target resolution
   ↓
MappingPlanEvaluationState
   ↓ complete only
RepresentabilityOutcome
   ↓
BackendAdapter execution
```

The key addition is not a new mapping layer but a normative internal contract for `MappingClaim.realization` and `MappingPlan`.

---

# 3. AR-01 remediation — RealizationEffect

## 3.1 Problem

ADR-0010 currently guarantees conflict detection for:

```text
same source
+ same obligation
+ incompatible realization
=> MappingConflict
```

This does not cover two claims with different SOL sources and/or obligations that mutate the same backend resource, property, selection, formulation slot, or overlapping scope incompatibly.

## 3.2 Proposed internal construct

`MappingClaim.realization` MUST be normalizable by the selected adapter into one or more `RealizationEffect` records.

Minimum contract:

```text
RealizationEffect
├── operation
├── resource
├── slot?
├── scope?
├── payload_fingerprint?
└── semantics
```

### Field meaning

- `operation`: adapter-normalized mutation class. Core need not prescribe backend object vocabulary. A minimal generic set MAY include `create`, `write`, `attach`, `select`, `remove`, and adapter extensions.
- `resource`: a stable MappingPlan-local resource identity or external backend-resource handle.
- `slot`: optional addressable subresource/property/path within `resource`.
- `scope`: optional resolved set/domain/boundary/selection extent affected by the effect.
- `payload_fingerprint`: optional normalized value/effect identity used to test duplicate/equivalent writes.
- `semantics`: adapter-owned compatibility metadata sufficient to determine commutativity, exclusivity, or mergeability.

These are **not** added to `MappingClaim`; they live inside or are derived from `realization`.

## 3.3 Required adapter operations

For every pair of normalized effects whose resources may overlap, the adapter MUST provide deterministic evaluation of:

```text
overlaps(effectA, effectB)
compatible(effectA, effectB)
commutes(effectA, effectB)
```

Each evaluation SHALL return one of:

```text
true
false
indeterminate
```

`indeterminate` SHALL not be silently treated as compatible.

## 3.4 Normative collision invariant

MappingPlan conflict checking SHALL perform both stages:

```text
Stage 1 — obligation-key conflict
same source + same obligation + incompatible realization
=> MappingConflict

Stage 2 — effect collision
any claims with overlapping non-compatible effects
=> MappingConflict
```

Therefore different `source` or different `obligation` does not exempt claims from compatibility checking.

## 3.5 Thermal walkthrough

COMSOL-like example:

```text
Claim A
source      = conductivity-1
obligation  = coefficient-binding
realization = use material thermal conductivity

effect A
operation = write
resource  = solid-physics-1
slot      = thermal-conductivity-source
scope     = domain-1
payload   = material:k

Claim B
source      = material-assignment-1
obligation  = property-source
realization = set user expression

effect B
operation = write
resource  = solid-physics-1
slot      = thermal-conductivity-source
scope     = domain-1
payload   = expression:k_user
```

The claim keys differ, but effects overlap at the same resource/slot/scope and are incompatible. The plan deterministically rejects them.

## 3.6 Plasma walkthrough

```text
Claim A
source      = electron-wall-bc
obligation  = electron-boundary-flux

effect A
resource = wall-flux-condition
slot     = electron-flux
scope    = {wall-3, wall-4, wall-5}

Claim B
source      = absorbing-wall-model
obligation  = particle-wall-model

effect B
resource = wall-flux-condition
slot     = electron-flux
scope    = {wall-5, wall-6}
```

If payload semantics are incompatible, overlap on `wall-5` produces a MappingConflict even though claim source and obligation differ.

## 3.7 Classification of change

**Normative clarification / MappingPlan internal schema addition.**  
No top-level MappingRule or MappingClaim schema change is required.

---

# 4. AR-02 remediation — Action/effect MappingPlan semantics

## 4.1 Problem

A DAG over entire backend objects or entire realizations cannot distinguish creation from later configuration. It can therefore create false cycles or hide real order dependencies.

## 4.2 Proposed internal construct

`MappingRule.realization` SHALL lower into one or more `PlanAction` nodes.

Minimum contract:

```text
PlanAction
├── id
├── requires[]
├── produces[]
├── effects[]
└── idempotency
```

### Field meaning

- `id`: plan-local stable identity.
- `requires[]`: resource/handle prerequisites that must already exist or be produced by predecessor actions.
- `produces[]`: resource/handle identities made available after successful action completion.
- `effects[]`: `RealizationEffect` records induced by the action.
- `idempotency`: at minimum `idempotent` or `non-idempotent`; adapters MAY refine this vocabulary.

`create`, `attach`, and `configure` remain possible operation kinds, but they are not universal fixed phases.

## 4.3 Dependency derivation

A directed edge `A -> B` SHALL exist when:

1. `B.requires` contains a handle produced by `A`; or
2. the adapter states that noncommuting effects require A before B.

The DAG is therefore action-level, not backend-object-level.

## 4.4 Deterministic validation invariants

### MP-I1 — Unresolved prerequisite

Every required handle MUST resolve to exactly one of:

- an external/pre-existing backend binding declared in the target environment; or
- one valid producer action in the MappingPlan; or
- multiple producers proven equivalent and coalescible by the adapter.

Otherwise the plan is `blocked`.

### MP-I2 — Duplicate producer

Two actions MUST NOT independently produce the same exclusive resource handle unless the adapter proves them equivalent/idempotent and the planner coalesces them deterministically.

Otherwise the plan has a MappingConflict.

### MP-I3 — Mutation collision

Unordered actions with overlapping noncommutative effects MUST produce a MappingConflict or an explicit deterministic ordering edge supplied by adapter semantics. Declaration order is not allowed as an implicit tie-breaker.

### MP-I4 — Cycle

The final dependency graph MUST be acyclic after action decomposition. A cycle is a MappingPlan conflict unless an adapter-defined compound action replaces the cyclic subgraph with one atomic realization.

### MP-I5 — Idempotency

Equivalent repeated actions MAY be coalesced only when both are declared/proven idempotent with the same normalized effect fingerprint. Non-idempotent duplicates MUST remain distinct and ordered or be rejected.

### MP-I6 — No hidden prerequisites

Adapter execution MUST NOT introduce a prerequisite absent from the validated MappingPlan. Runtime discovery of a new prerequisite invalidates the plan and returns execution to `blocked`/replan rather than silently mutating execution order.

## 4.5 Thermal walkthrough — MOOSE transient heat

A correct plan can be decomposed as:

```text
T1 produce variable T
T2 require T -> add HeatConduction kernel
T3 require T,density,specific_heat -> add HeatConductionTimeDerivative
T4 require T,boundary-scope -> add temperature BC
T5 require completed transient formulation -> configure transient execution context
```

This prevents the defect where `TransientStudy` maps only to a transient executioner while omitting the time-derivative equation term.

If density or specific heat has no producer/binding, T3 is an unresolved prerequisite and the plan is `blocked`, not `unsupported`.

## 4.6 Plasma walkthrough — COMSOL-style reaction grouping

```text
P1 create plasma physics context -> plas
P2 require plas -> create species -> species handles
P3 require plas -> create reaction group -> rg
P4 require rg,species -> add reaction rows
P5 require completed reaction rows -> configure grouped reaction settings
```

Creation and later configuration of the same physics resource are separate actions, avoiding a false cycle.

## 4.7 Classification of change

**Normative clarification + MappingPlan internal schema addition.**  
No sixth MappingRule field is required because actions are generated from `realization`.

---

# 5. AR-04 remediation — MappingPlan evaluation lifecycle

## 5.1 Problem

`exact`, `transformed`, `lossy`, and `unsupported` are terminal semantic representability outcomes. They are insufficient to describe a plan that cannot yet be evaluated because target identity, release, modules, capabilities, rules, prerequisites, or compatibility results are unresolved.

## 5.2 Proposed lifecycle

Introduce a separate `MappingPlanEvaluationState`:

```text
pending
blocked
indeterminate
complete
```

This state is **not** a fifth representability class.

### `pending`

Evaluation has not yet completed. Required rule/target/capability checks may still be outstanding.

### `blocked`

A concrete required input is missing or unresolved, for example:

- BackendTarget incomplete;
- backend release not selected/resolved;
- required module/capability unavailable or discovery failed;
- missing MappingRule for a required obligation;
- unresolved PlanAction prerequisite;
- missing external backend binding.

The missing item MUST be reported with provenance.

### `indeterminate`

Required evidence is present, but the evaluator cannot decide semantic compatibility or effect compatibility with the available comparator/adapter semantics.

Examples:

- effect overlap cannot be resolved because backend selection equality is undecidable;
- capability exists but semantic equivalence cannot be established;
- adapter compatibility function returns `indeterminate`.

### `complete`

All required evaluation checks have terminated deterministically. Only then SHALL one terminal representability outcome be emitted:

```text
exact
transformed
lossy
unsupported
```

## 5.3 Normative invariant

```text
MappingPlanEvaluationState != representability outcome
```

and:

```text
state == complete
=> exactly one terminal representability outcome exists

state != complete
=> no terminal representability outcome SHALL be asserted
```

This prevents unknown target/capability state from being mislabeled as backend `unsupported`.

## 5.4 Thermal walkthrough

If the profile targets `Ansys` but no product/application is selected:

```text
state = blocked
reason = BackendTarget.product/application unresolved
```

No `unsupported` conclusion is valid yet.

If the target is `Ansys Mechanical / Steady-State Thermal`, all required thermal capabilities resolve, and the mapping structurally changes but preserves semantics:

```text
state = complete
outcome = transformed
```

## 5.5 Plasma walkthrough

A plasma mapping targeting generic MOOSE with no selected application/object vocabulary is:

```text
state = blocked
reason = target application/capability contract unresolved
```

A selected plasma application with all required mappings but a comparator that cannot decide whether two dynamic selections overlap is:

```text
state = indeterminate
```

## 5.6 Classification of change

**Normative clarification + MappingPlan state model addition.**  
The representability enum does not change.

---

# 6. Executable BackendTarget minimum contract

## 6.1 Problem

Vendor/framework labels such as `Ansys` or `MOOSE` are insufficient for an executable mapping target because multiple products/applications and formulations may exist under one umbrella.

## 6.2 Proposed minimum contract

A Profile targeting execution SHALL resolve a `BackendTarget` containing at least:

```text
BackendTarget
├── adapter_identity
├── product_or_application
├── formulation
├── release_compatibility
└── required_modules_or_capabilities[]
```

### Meaning

- `adapter_identity`: exact executable adapter implementation/provider identity.
- `product_or_application`: concrete product/application contract, e.g. Ansys Mechanical, COMSOL Multiphysics + Plasma Module, MOOSE + selected application.
- `formulation`: selected mathematical/solver formulation relevant to semantic equivalence, e.g. steady-state thermal, transient thermal, fluid drift-diffusion plasma, PIC plasma.
- `release_compatibility`: adapter-defined release matching under ADR-0009.
- `required_modules_or_capabilities[]`: adapter-owned module/capability requirements.

This contract SHALL remain Profile/Adapter metadata; backend product vocabulary is not promoted into SOL Core ontology taxonomy.

## 6.3 Normative target invariant

A MappingPlan SHALL NOT reach `complete` evaluation unless BackendTarget is sufficiently resolved for all MappingRules and capabilities used by the plan.

Incomplete target identity produces `blocked`, not `unsupported`.

## 6.4 Examples

### Thermal Ansys

```text
adapter_identity       = ansys-mechanical-adapter
product_or_application = Ansys Mechanical
formulation            = Steady-State Thermal
release_compatibility  = adapter-defined 2026 R1 range
required capabilities  = thermal-conductivity, temperature-condition, geometry-scoping
```

### Plasma MOOSE

```text
adapter_identity       = moose-plasma-adapter
product_or_application = MOOSE + selected Zapdos/CRANE/custom application
formulation            = selected fluid/plasma formulation
release_compatibility  = MOOSE/application-specific compatibility
required capabilities  = registered species/transport/reaction/BC objects
```

### Plasma Ansys

`Ansys Mechanical`, `Ansys Chemkin`, and `Ansys Charge Plus` SHALL be separate product/application targets. A fluid drift-diffusion plasma ontology MUST NOT silently resolve to a PIC product solely because both are called plasma-capable.

## 6.5 Classification of change

**Primarily normative Profile contract clarification.**  
ADR-0010 already permits target adapter binding, release compatibility, and required capabilities. The supplement makes executable target completeness explicit.

---

# 7. Deterministic MappingPlan validation algorithm

The following order is proposed as normative:

```text
1. Resolve BackendTarget
2. Discover/validate adapter capabilities
3. Evaluate MappingRule source/applicability
4. Generate MappingClaims
5. Normalize claim realizations into PlanActions + RealizationEffects
6. Resolve producers and external prerequisites
7. Detect duplicate producers
8. Build action dependency graph
9. Check cycles
10. Compare all potentially overlapping effects
11. Add deterministic ordering edges for noncommutative-but-compatible effects where adapter semantics permit
12. Re-check acyclicity
13. Evaluate idempotency/coalescing
14. Determine MappingPlanEvaluationState
15. If complete, assign exact/transformed/lossy/unsupported
```

At no stage may declaration order or numeric priority act as an implicit semantic resolution mechanism.

---

# 8. Core/schema changes vs normative clarification

| Item | Category | Proposed change |
|---|---|---|
| MappingRule 5 fields | No change | Preserve exactly |
| MappingClaim 4 fields | No change | Preserve exactly |
| RealizationEffect | MappingPlan internal schema | Add explicit normalized internal construct |
| PlanAction | MappingPlan internal schema | Add explicit action-node construct |
| Effect collision invariant | Normative clarification | Expand conflict algorithm across all claims |
| pending/blocked/indeterminate/complete | MappingPlan schema/state | Add evaluation lifecycle |
| exact/transformed/lossy/unsupported | No change | Keep terminal semantic outcomes |
| BackendTarget minimum contract | Profile normative clarification | Require executable target resolution |
| backend object taxonomy | No Core change | Remains adapter-owned |

---

# 9. Validator/runtime contract required before Validation Lab rerun

Validation Lab SHALL not treat the supplement as validated until at least the following exist:

1. serialized schema/types for `RealizationEffect`, `PlanAction`, `MappingPlanEvaluationState`, and executable `BackendTarget`;
2. MappingClaim generator preserving the four required fields;
3. MappingRule generator/parser preserving the five top-level fields;
4. adapter normalization hook from realization payloads to effects/actions;
5. producer/prerequisite resolver;
6. duplicate-producer checker;
7. effect overlap/compatibility comparator with `true/false/indeterminate` result;
8. cycle checker after action decomposition;
9. idempotency/coalescing checker;
10. evaluation lifecycle implementation that forbids terminal representability before `complete`;
11. Thermal reference instance capable of generating inspectable MappingPlan output;
12. at least one executable backend adapter path for artifact generation.

Paper walkthrough alone is not a PASS.

---

# 10. ADR-0010 supplement scope decision

Recommended supplement scope:

```text
ADR-0010 Supplement
├── RealizationEffect normalization contract
├── cross-claim collision invariant
├── PlanAction dependency semantics
├── producer/prerequisite/cycle/idempotency invariants
├── MappingPlan evaluation lifecycle
└── executable BackendTarget minimum contract
```

This supplement does **not** need to redefine Profile/BackendAdapter separation, MappingRule cardinality, MappingClaim field count, or the four terminal representability outcomes.

---

# 11. Canonical architecture impact

If accepted, the canonical SOL v0.1 mapping architecture changes from:

```text
MappingClaims
  -> simple obligation conflict
  -> realization DAG
  -> representability result
```

to:

```text
MappingClaims
  -> normalized PlanActions + RealizationEffects
  -> obligation conflicts + cross-claim effect collisions
  -> action-level deterministic DAG validation
  -> explicit evaluation lifecycle
  -> terminal representability only when complete
```

The public top-level MappingRule/MappingClaim contracts remain stable.

---

# 12. Gate for next validation stage

Validation Lab may re-enter cross-backend architecture validation only after:

1. this supplement is accepted or normatively equivalent contracts are approved;
2. AR-03 has a separately approved Constraint remediation contract;
3. the MappingPlan validator/runtime contracts listed above are implemented;
4. executable BackendTargets are selected for MOOSE, COMSOL, and Ansys reference projections;
5. a concrete Thermal reference model exists and generates MappingClaims, PlanActions, effects, plan state, and backend artifacts;
6. execution evidence is collected rather than inferred solely from documentation.

Only after the Thermal gate passes should Plasma execution validation be used for architecture-freeze approval.
