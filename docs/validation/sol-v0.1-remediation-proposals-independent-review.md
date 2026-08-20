# Independent Review of SOL v0.1 Remediation Proposals

**Status:** Complete — Research Lab revision required before ADR drafting  
**Date:** 2026-08-20  
**Review branch:** `validation/sol-v0.1-independent-review`

## 1. Documents reviewed

- [ADR-0010 supplement remediation proposal](../research/sol-v0.1-adr0010-supplement-remediation-proposal-v0.1.md)
- [Qualified Relation Cardinality proposal](../research/sol-v0.1-qualified-relation-cardinality-constraint-proposal-v0.1.md)
- [Original independent architecture validation](sol-v0.1-independent-architecture-validation-report.md)
- [ADR-0010](../decisions/0010-profile-backend-mapping-contract.md)
- [ADR-0007](../decisions/0007-constraint-architecture-and-composition.md)

The original AR-01–04 classifications and the proposed remediations were treated as hypotheses, not accepted conclusions.

## 2. Executive decision

| Proposal item | Contract-level decision | Architecture closure | Executable evidence |
|---|---|---|---|
| `RealizationEffect` | **Revise** | Not closed | **requires executable validation** |
| `PlanAction` | **Revise** | Not closed | **requires executable validation** |
| Evaluation lifecycle | **Revise** | Not closed | **requires executable validation** |
| Executable `BackendTarget` | **Revise** | Not closed | **requires executable validation** |
| Qualified Relation Cardinality | **Revise** | Not closed | **requires executable validation** |
| Preserve `MappingRule` five fields | **Accept conditionally** | Shape supported; internals unresolved | **requires executable validation** |
| Preserve `MappingClaim` four fields | **Accept conditionally** | Shape supported; normalization/source cardinality unresolved | **requires executable validation** |

None of the concepts should be rejected. Each addresses a real failure mode without inherently copying MOOSE, COMSOL, or Ansys object models into SOL Core. However, every proposal leaves at least one normative ambiguity capable of producing different valid validator implementations. Therefore the current texts are not ready to become ADRs unchanged.

**Final process verdict: Research Lab rework is required before ADR drafting.**

## 3. Review method

Each item was tested against:

1. the original counterexample;
2. a second-order counterexample introduced by the remediation itself;
3. Core/Profile/Adapter ownership;
4. deterministic validation requirements;
5. thermal and plasma backend structures documented by MOOSE, COMSOL, and Ansys;
6. whether the public five-field/four-field contracts still carry sufficient meaning;
7. whether paper reasoning can be separated from executable evidence.

Official backend evidence confirms that action ordering, resource tags, feature selections, and scoping are real adapter concerns:

- MOOSE syntax generates Actions, Actions perform tasks, and common uses include setup and object creation: [MOOSE Action System](https://mooseframework.inl.gov/source/actions/Action.html).
- MOOSE resolves Action dependency sets in dependency order: [MOOSE ActionWarehouse](https://mooseframework.inl.gov/docs/doxygen/moose/classActionWarehouse.html).
- COMSOL creates physics features using backend-local tags and feature types: [COMSOL PhysicsFeature API](https://doc.comsol.com/5.3a/doc/com.comsol.help.comsol/api/com/comsol/model/physics/PhysicsFeature.html).
- COMSOL plasma surface reactions carry explicit boundary selections: [COMSOL Argon/Oxygen CCP model](https://doc.comsol.com/6.4/doc/com.comsol.help.models.plasma.ccp_argon_oxygen/ccp_argon_oxygen.html).
- Ansys Mechanical loads can be attached to Named Selections: [Ansys convection scripting example](https://ansyshelp.ansys.com/public/views/secured/corp/v251/en/act_script/act_script_examples_convection.html).

These facts support adapter-owned normalization and execution metadata, not new backend-specific Core ontology terms.

---

## 4. `RealizationEffect`

### Decision: **Revise**

### 4.1 Does it solve AR-01?

**Partially yes.**

The proposed second-stage invariant—

```text
any claims with overlapping non-compatible effects
=> MappingConflict
```

—correctly removes the false assumption that only equal source/obligation keys can conflict. It covers:

- different conductivity claims writing the same COMSOL effective property;
- different wall models applying incompatible flux semantics over overlapping selections;
- different Ansys loads resolving to the same scoped backend resource.

This is the correct architectural direction.

### 4.2 New ambiguities

#### RE-01 — Resource alias ambiguity

Two effects can refer to different plan-local resources that resolve to the same runtime object:

```text
resource A = generated selection "heated_faces"
resource B = external named selection "NS_HEATED"
```

If both resolve to the same entity set, resource inequality cannot prove non-overlap. The proposal delegates `overlaps` to the adapter, which is appropriate, but does not require canonical resource-resolution evidence or an alias-equivalence result.

#### RE-02 — Optional address fields cannot guarantee preflight

`slot`, `scope`, and `payload_fingerprint` are optional. An adapter may therefore produce effects too weak to compare while still appearing schema-valid. Returning `indeterminate` is safe, but deterministic validation also needs a rule specifying when missing comparison data is:

- a legitimate `indeterminate`;
- a malformed effect and Validation-tooling/Profile defect;
- an unresolved external binding and `blocked`.

#### RE-03 — `semantics` is circular

The phrase “adapter-owned compatibility metadata sufficient to determine compatibility” does not define a testable contract. Two adapters can normalize the same semantic situation differently and both claim conformance.

#### RE-04 — Operation vocabulary ownership

A Core-required list such as `create/write/attach/select/remove` would risk turning current backend mutation styles into universal semantics. These operations belong to MappingPlan IR or an adapter protocol, not the SOL semantic ontology. The proposal says the list “MAY” be generic, but the normative boundary must be explicit.

### 4.3 Required revision

Retain `RealizationEffect`, but require:

- a versioned adapter normalization contract;
- a canonical plan-local resource address plus resolution provenance;
- explicit alias/overlap evidence;
- a typed comparator result containing decision, reason, comparator identity/version, and evidence;
- a conformance rule that missing required comparison information is not silently accepted;
- a clear statement that effect operations and slots are MappingPlan/Adapter IR, not Core semantic vocabulary.

A useful result shape is:

```text
EffectComparison
├── overlap: true | false | indeterminate
├── compatibility: compatible | conflict | indeterminate
├── commutativity: commutes | ordered | conflict | indeterminate
├── evidence
└── comparator_provenance
```

This does not need to become a `MappingClaim` field.

### 4.4 Core leakage assessment

**No necessary Core leakage**, provided `RealizationEffect` is explicitly a mapping-runtime IR contract. Backend tags, properties, selections, and operations must stay inside adapter-normalized payloads.

### 4.5 Evidence status

**requires executable validation**

Minimum executable tests:

- same resource/slot, equal payload → coalescible;
- same resource/slot, incompatible payload → conflict;
- distinct resource aliases resolving to overlapping selection → conflict;
- unresolved dynamic selection → indeterminate;
- absent required comparator data → deterministic malformed/blocked result.

---

## 5. `PlanAction`

### Decision: **Revise**

### 5.1 Does it solve AR-02?

**Partially yes.**

Decomposing an object-level realization into action-level nodes can remove false cycles and expose missing prerequisites. This matches real backend behavior: MOOSE uses dependency-resolved Actions, COMSOL creates tagged features before later configuration/selection, and Ansys scripting creates analysis/load objects before assigning values and locations.

The proposed invariants for unresolved prerequisites, duplicate producers, cycles, mutation collisions, and hidden prerequisites are necessary.

### 5.2 New ambiguities

#### PA-01 — No executable action descriptor

The minimum contract contains:

```text
id, requires, produces, effects, idempotency
```

It describes dependencies and effects but not the adapter operation/payload that realizes the action. A plan could validate structurally yet be non-executable. If executable content remains elsewhere in `MappingClaim.realization`, the correspondence between validated effects and executed operation is not fixed.

#### PA-02 — Noncommutativity does not determine direction

When two effects do not commute, the proposal allows an adapter to add `A -> B`. But `not commutative` proves only that order matters; it does not prove which order preserves SOL semantics.

Example:

```text
A = set property source to material
B = set property source to user expression
```

Neither A→B nor B→A is justified merely by backend behavior. One must be a conflict unless semantic intent supplies an ordering/override relation.

#### PA-03 — Idempotency is context-sensitive

“Same fingerprint + idempotent” is insufficient when execution depends on:

- current backend state;
- target release;
- external object existence;
- transaction boundary;
- generated backend-local identity.

An action can be idempotent in an empty model but not after partial execution.

#### PA-04 — Hidden compound transactions

COMSOL feature creation/configuration or an Ansys analysis insertion may be atomic only at an adapter/API level. Replacing a cycle with an adapter-defined compound action is safe only if its externally observable effects and failure behavior are validated as one unit. Otherwise the compound action hides the original dependency problem.

### 5.3 Required revision

Add or normatively associate:

- an adapter-owned executable descriptor/reference;
- a proof that normalized effects cover the executable operation;
- `must_precede` evidence with provenance, distinct from noncommutativity;
- explicit conflict when direction cannot be semantically justified;
- idempotency scope/precondition and backend-state fingerprint;
- atomicity/failure semantics for compound actions;
- deterministic action identity and stable source-identity trace.

The required distinction is:

```text
noncommuting(A,B)
  !=
must_precede(A,B)

must_precede(A,B)
  requires semantic or backend lifecycle evidence
```

### 5.4 Core leakage assessment

**No necessary Core leakage.** `PlanAction` should be a MappingPlan runtime construct. MOOSE Action types, COMSOL feature tags, and Ansys automation methods must remain adapter data.

### 5.5 Evidence status

**requires executable validation**

At minimum, implement a dry-run MOOSE thermal plan with separate Variable, conduction kernel, time-derivative kernel, BC, and Executioner actions, then verify:

- missing density/specific heat blocks transient completion;
- declaration order does not change the plan;
- justified order edges are stable;
- repeated planning is deterministic;
- executed operations match declared effects.

---

## 6. MappingPlan evaluation lifecycle

### Decision: **Revise**

### 6.1 Does it solve AR-04?

**Yes at the conceptual boundary.**

Separating:

```text
pending | blocked | indeterminate | complete
```

from:

```text
exact | transformed | lossy | unsupported
```

prevents missing evidence from being mislabeled as a backend limitation. The invariant forbidding terminal representability before `complete` is sound.

### 6.2 New ambiguities

#### EL-01 — `blocked` and `indeterminate` boundary

The proposal lists capability discovery failure as `blocked`, while an adapter comparator returning no decision is `indeterminate`. It does not define:

- timeout versus authoritative capability absence;
- missing comparator implementation versus genuine undecidability;
- malformed Profile versus unresolved runtime state.

These cases need different defect classifications and retry behavior.

#### EL-02 — Outcome aggregation

A complete MappingPlan may contain:

- exact identity mapping;
- transformed field realization;
- lossy provenance serialization;
- unsupported optional or required obligation.

The proposal requires “exactly one terminal outcome” but does not define whether outcomes exist per claim/effect/action/obligation, how plan-level summary is derived, or whether an unsupported optional obligation prevents execution.

#### EL-03 — `unsupported` execution policy

ADR-0010 says unsupported means faithful realization cannot proceed under current policy. A plan containing an unsupported item should not necessarily be `complete` in the same operational sense as an executable transformed plan unless “complete evaluation” is clearly separated from “executable plan.”

#### EL-04 — State transition determinism

Permitted transitions and invalid regressions are unspecified. For example:

```text
blocked -> pending -> complete
complete -> blocked after runtime drift
```

requires either a new plan revision or invalidation, not mutation of historical evidence.

### 6.3 Required revision

Define:

- validation/tooling error versus blocked input versus semantic indeterminacy;
- per-obligation representability results;
- deterministic plan-summary aggregation;
- required versus optional obligation policy;
- `evaluation_complete` separately from `execution_permitted`;
- immutable evaluated-plan revision and invalidation/replan rules;
- explicit state-transition table.

The four terminal representability values should remain unchanged.

### 6.4 Core leakage assessment

None. This is MappingPlan lifecycle, not Core ontology.

### 6.5 Evidence status

**requires executable validation**

Test target-resolution failure, missing rule, capability absence, comparator indeterminacy, lossy optional metadata, and unsupported required physics as separate fixtures.

---

## 7. Executable `BackendTarget`

### Decision: **Revise**

### 7.1 Does it solve the generic-backend counterexample?

**Mostly yes.**

Requiring an adapter, product/application, formulation, release, and modules/capabilities prevents “Ansys” from silently resolving to Mechanical, Chemkin, or Charge Plus and prevents generic MOOSE from standing in for an unidentified application.

This is supported by official evidence:

- MOOSE applications register their own Actions, tasks, syntax, and objects.
- COMSOL Plasma behavior depends on the Plasma Module and particular physics features/selections.
- Ansys products expose materially different model formulations and automation surfaces.

### 7.2 New ambiguities

#### BT-01 — Declared target versus resolved runtime target

`release_compatibility` is a requirement/range, not the actual release discovered at runtime. The proposal combines desired target policy and observed executable environment.

#### BT-02 — Formulation duplication

`formulation` may duplicate or contradict SOL's MathematicalModel/Analysis semantics:

```text
SOL Analysis = Transient
BackendTarget.formulation = Steady-State Thermal
```

If formulation is backend realization policy, it must be named and validated as a binding to SOL semantics, not a second source of semantic truth.

#### BT-03 — Capability duplication

Target-level `required_modules_or_capabilities` overlaps MappingRule `capabilities`. The proposal does not define whether the target list is:

- an environment envelope;
- the union of selected rule requirements;
- a user-declared minimum;
- independently authored policy.

Different interpretations can produce different blocked/unsupported decisions.

#### BT-04 — Coupled/multi-product target

A plasma workflow may need COMSOL Plasma plus AC/DC, or multiple Ansys products. Singular `product_or_application` and `adapter_identity` may not cover a composite target without ad hoc strings.

### 7.3 Required revision

Separate:

```text
BackendTargetRequirement
  adapter contract
  product/application requirements
  allowed formulation bindings
  release range
  module/capability requirements

ResolvedBackendTarget
  concrete adapter build
  concrete product/application instances
  actual releases
  installed/licensed modules
  discovered capabilities
  evidence/provenance
```

Also require:

- formulation compatibility with SOL Analysis/MathematicalModel;
- a deterministic rule deriving selected-rule capability requirements;
- support for a target component set or explicitly defer coupled targets;
- no vendor/product terms in Core semantic taxonomy.

### 7.4 Core leakage assessment

The proposal correctly keeps vendor vocabulary in Profile/Adapter metadata. The `formulation` field needs revision to prevent it from becoming competing semantic truth.

### 7.5 Evidence status

**requires executable validation**

Demonstrate at least:

- MOOSE Heat Transfer application/framework target;
- COMSOL Heat Transfer target with required module;
- Ansys Mechanical Steady-State Thermal target;
- generic “Ansys” rejected as incomplete;
- fluid plasma target not allowed to bind silently to a PIC formulation.

---

## 8. Qualified Relation Cardinality

### Decision: **Revise**

### 8.1 Does it solve AR-03?

**Yes for the exact existential typed-subset counterexample.**

```text
count(products targets whose type <: NegativeIonSpecies) >= 1
```

is precise, backend-independent, and does not require a general query language. It reuses Cardinality and Type semantics rather than introducing plasma-specific Core terms. The construct is not derived from COMSOL reaction tables or another backend object model.

### 8.2 New ambiguities

#### QRC-01 — Open-world versus closed-world counting

A cardinality validator must know whether the relation target set is complete. Under an open-world graph, observing zero negative-ion products does not prove that none exist. The proposal assumes closed model-instance validation but does not state it.

#### QRC-02 — Multiple typing and subtype closure

A target may have multiple asserted/inferred types. The validator needs a canonical subtype closure, ontology-version context, and inconsistency policy.

#### QRC-03 — Set versus multiset semantics

The same target identity can be reached by duplicate serialized edges. Counting edges and counting unique semantic target identities produce different results.

#### QRC-04 — Incorrectly stated monotonic refinement

For target sets (A subseteq B):

```text
count(A) <= count(B)
```

Therefore:

- `min n` on A implies `min n` on B;
- `max n` on B implies `max n` on A;
- the opposite directions do not follow.

A “more specific subtype qualifier” is not uniformly a monotonic refinement for arbitrary min/max intervals. QRC-I5 is too broad.

#### QRC-05 — Distinct qualifier satisfiability

Two constraints with different qualifiers may overlap through subtype or multiple typing. Simply evaluating them conjunctively is deterministic on a closed concrete model, but schema-level satisfiability and normalization require disjointness/subsumption knowledge. The proposal does not define when the validator must reason about this versus preserve separate obligations.

#### QRC-06 — Universal claim workaround is incomplete

The statement that “all products are T” can often use endpoint narrowing is true only when the relation contract itself should exclude every non-T target. Equality between total and qualified counts is not available in the proposed v0.1 language. The proposal should explicitly leave universal qualified constraints unsupported rather than imply coverage.

### 8.3 Required revision

Specify:

- qualified cardinality is evaluated under a closed-world model-instance relation snapshot;
- unique stable target identities are counted, unless a separately reified relation-instance model defines multiplicity;
- type matching uses a versioned canonical subtype closure;
- unknown/inconsistent target typing produces a defined validation result;
- formal subtype/min/max implication rules;
- equivalent qualifier normalization;
- distinct qualifiers remain separate unless a proven subset/disjointness rule applies;
- schema-level satisfiability scope;
- universal arbitrary qualified rules remain deferred.

### 8.4 Core expansion assessment

This is a **justified narrow Core/schema extension** because it expresses a backend-independent graph invariant reused across domains. It does not add a seventh constraint family or backend-native semantics.

However, the proposal should name it as an optional qualifier on Relation Cardinality, not imply that all Cardinality constraints acquire arbitrary filtering.

### 8.5 Evidence status

**requires executable validation**

Required fixtures:

- empty product set;
- one matching subtype;
- no matching subtype;
- multiple matching subtypes;
- duplicate serialized edge to the same target identity;
- multiply typed target;
- unknown target type;
- parent/subtype min and max combinations;
- Conditional activation;
- provenance-preserving failure output.

---

## 9. Five-field `MappingRule` assessment

### Decision: **Accept conditionally**

No reviewed counterexample requires a sixth top-level field. Actions, effects, dependencies, and executable descriptors can be contained in or derived from `realization`; capability requirements remain in `capabilities`; target environment belongs to Profile metadata.

This remains justified only if:

- `realization` receives a normative internal schema/protocol;
- action/effect normalization is deterministic and versioned;
- source/applicability support N-source and selection-pattern semantics;
- BackendTarget remains Profile metadata rather than being duplicated into every rule.

**requires executable validation**

A parser/generator must demonstrate that real Thermal and Plasma rules retain exactly the five top-level fields without hidden side channels.

## 10. Four-field `MappingClaim` assessment

### Decision: **Accept conditionally**

A fifth required field is not yet justified. Effects, actions, execution order, capabilities, diagnostics, and representability belong to realization/plan/reporting layers.

Conditions:

- `source` must normatively allow a stable ordered or unordered identity set for N:1/N:N mappings;
- `realization` must retain an executable descriptor and normalize to effects/actions;
- `provenance` must identify rule/profile versions and normalization provider;
- the plan must detect conflicts across all claims, not only equal claim keys.

**requires executable validation**

Generate claims for COMSOL reaction grouping (N SOL reactions → one group) and verify that every contributing SOL identity survives without adding a fifth field.

---

## 11. Deterministic validation verdict

The proposals make deterministic validation possible in principle, but not yet mandatory in a unique way.

| Concern | Current proposal | Remaining determinism gap |
|---|---|---|
| Effect collision | Ternary adapter comparator | Comparator evidence, aliases, malformed effect rule |
| Action DAG | Producers/requirements/effects | Executable binding, order direction proof, stateful idempotency |
| Lifecycle | Four plan states | Aggregation, transitions, error taxonomy, execution permission |
| Backend target | Minimum metadata | Requirement/resolution split, coupled targets, formulation compatibility |
| Qualified cardinality | Typed subset count | Closed world, unique identity count, subtype/min-max algebra |

No architecture item may be marked fully remediated until these rules are normatively fixed and executable fixtures agree.

## 12. Final disposition

### May ADR drafting proceed now?

**NO.**

The proposals are sufficiently promising to revise rather than redesign, but ADR drafting now would freeze ambiguous semantics into normative language.

### Required next step

**Research Lab rework is required.** It should produce revised proposals addressing only the gaps listed in this report. It should preserve the current minimal direction unless a new counterexample disproves it:

- keep five `MappingRule` fields;
- keep four `MappingClaim` fields;
- keep four terminal representability outcomes;
- keep effect/action constructs outside Core semantics;
- keep BackendTarget in Profile/Adapter architecture;
- keep Qualified Relation Cardinality narrow and backend-independent.

After proposal revision, Validation Lab should perform a short contract re-review. Even if that passes, every item remains **requires executable validation** until schemas, validators, MappingPlan generation, and at least one backend artifact path exist.

### Architecture freeze impact

SOL v0.1 architecture freeze remains **blocked**.
