# SOL v0.1 AR-01–AR-04 Architecture Reassessment v0.1

**Status:** Research / change-proposal evidence; no ADR modified  
**Date:** 2026-08-20  
**Branch:** `validation/sol-v0.1-independent-review`  
**Primary inputs:** independent validation report; ADR-0006, ADR-0007, ADR-0009, ADR-0010

## 1. Purpose and evidence boundary

This note independently re-evaluates the four architecture counterexamples AR-01 through AR-04 reported in [`sol-v0.1-independent-architecture-validation-report.md`](../validation/sol-v0.1-independent-architecture-validation-report.md).

The validation report is treated as evidence, not as an automatic decision. The accepted ADRs are also treated as design commitments to be tested rather than assumptions of correctness.

This reassessment uses official backend documentation to test whether the reported failure modes correspond to real backend semantics:

- MOOSE HeatConduction: <https://mooseframework.inl.gov/moose/source/kernels/HeatConduction.html>
- MOOSE HeatConductionTimeDerivative: <https://mooseframework.inl.gov/moose/source/kernels/HeatConductionTimeDerivative.html>
- MOOSE Action System: <https://mooseframework.inl.gov/moose/source/actions/Action.html>
- COMSOL material API: <https://doc.comsol.com/6.4/doc/com.comsol.help.comsol/comsol_api_general.47.40.html>
- COMSOL Physics API: <https://doc.comsol.com/6.4/doc/com.comsol.help.comsol/api/com/comsol/model/physics/Physics.html>
- COMSOL PhysicsFeature API: <https://doc.comsol.com/6.4/doc/com.comsol.help.comsol/api/com/comsol/model/physics/PhysicsFeature.html>
- COMSOL selections API: <https://doc.comsol.com/6.4/doc/com.comsol.help.comsol/comsol_api_general.47.56.html>
- COMSOL Plasma Reaction Group: <https://doc.comsol.com/6.4/doc/com.comsol.help.plasma/plasma_ug_heavy_species.08.08.html>
- COMSOL Plasma Chemistry File Format: <https://doc.comsol.com/6.4/doc/com.comsol.help.plasma/plasma_ug_data.05.12.html>
- Ansys Charge Plus: <https://ansys.synopsys.com/en-gb/products/electronics/ansys-charge-plus>
- Ansys Chemkin-CFD API Reference Guide: <https://ansyshelp.ansys.com/public/Views/Secured/corp/v251/en/pdf/Ansys_Chemkin-CFD_API_Reference_Guide.pdf>

No executable MOOSE/COMSOL/Ansys adapter or backend artifact was produced during this reassessment. Therefore this note can establish contract sufficiency/insufficiency and change requirements, but it does **not** convert the independent execution gate from FAIL to PASS.

## 2. Baseline ADR commitments

### ADR-0006

The SOL graph remains semantically authoritative. Backend inability must not erase semantic information, and lossy/unsupported mapping must be surfaced rather than silently weakening SOL.

### ADR-0007

Constraints compose conjunctively; six primitive families are defined. The v0.1 predicate vocabulary is Compare, Membership, Exists, and Boolean. Collection quantifiers were explicitly deferred.

### ADR-0009

Schema identity, model-instance identity, and backend-local identity are distinct. Backend compatibility scope is adapter-defined.

### ADR-0010

The architecture fixes:

```text
MappingRule = source + applicability + realization + bindings + capabilities

MappingClaim = obligation + source + realization + provenance
```

`MappingPlan` is the inspectable intermediate representation. Realization dependency metadata remains inside `realization`. Applicable MappingRules compose by default. The canonical conflict invariant is currently stated as:

```text
same source
+ same obligation
+ incompatible realization
=> MappingConflict
```

Terminal representability outcomes are `exact`, `transformed`, `lossy`, and `unsupported`.

---

# 3. AR-01 — MappingClaim conflict detection

## Validation report counterexample

The report gives two important forms.

1. Different SOL sources and obligations write incompatible values to the same effective backend property slot, for example material-derived thermal conductivity versus a user-defined physics property.
2. Different BC source objects resolve to overlapping backend selections and impose incompatible prescribed values.

The existing same-source/same-obligation key does not, by itself, require these claims to conflict.

## Does the counterexample actually hold?

**Yes.**

COMSOL is direct evidence. Its material API states that property groups can expose the same output property and that later property groups determine the effective output; it also permits heat-transfer material properties to be switched from `From material` to `User defined`. Thus two semantically different mapping concerns can compete for one effective backend property without sharing SOL source or obligation identity.

COMSOL selections are independently addressable and reusable by other entities. Consequently two different SOL BCs can resolve to overlapping backend geometric selections. Source inequality does not prove effect independence.

MOOSE provides the same class of issue at the formulation level: transient thermal behavior requires both the conduction term and `HeatConductionTimeDerivative`; the latter consumes density and specific-heat material properties. A plan that treats mapping obligations independently without comparing their effective mutations can produce an internally inconsistent or incomplete formulation.

## Final defect classification

**Architecture defect — normative compatibility gap, not a top-level data-shape defect.**

The report is correct that the canonical conflict invariant is insufficient if interpreted as the complete conflict algorithm. However, ADR-0010 already says claims with different obligations may compose *when their realizations are compatible* and assigns realization compatibility checks to adapters. This gives a place for the missing rule, but not a sufficiently normative contract.

## Can existing ADRs already solve it?

**Partially.**

ADR-0010 contains the architectural hook (`realization compatibility`) but does not require a canonical realization effect/resource contract or an all-claims collision pass. Therefore an implementation can conform textually while still checking only the same-source/same-obligation key.

## Required normative clarification

Keep the four required MappingClaim fields. Strengthen ADR-0010 semantics so that conflict detection has two stages:

```text
Stage 1 — obligation-key conflict
same source + same obligation + incompatible realization
=> conflict

Stage 2 — realization-effect conflict
any composed claims whose resolved effects overlap incompatibly
=> conflict
```

`MappingClaim.realization` MUST be able to expose, directly or through adapter normalization, a comparable effect contract. Core does not need to understand MOOSE/COMSOL/Ansys object models. It only requires an adapter-normalized effect surface sufficient for compatibility testing.

Minimum normalized effect contract:

```text
Effect
├── operation       # create | write | attach | select | remove | adapter extension
├── resource        # canonical plan-local/backend-resource handle
├── slot/path?      # addressable property/subresource when applicable
├── scope?          # resolved selection/domain/boundary subset when applicable
└── value/effect fingerprint?  # enough for equality/compatibility comparison
```

The adapter MAY own resource and slot vocabulary, but it MUST provide deterministic `overlaps(effectA,effectB)` and `compatible(effectA,effectB)` behavior or return an indeterminate result.

No fifth MappingClaim field is required because the effect contract is part of `realization`.

## Minimal architecture change if clarification is considered insufficient

A follow-up mapping-plan ADR could normatively define `RealizationEffect` as an internal MappingPlan construct referenced from `MappingClaim.realization`. It still would not alter MappingClaim's four required top-level fields.

## Alternatives and trade-offs

### Alternative A — Add `write_set` as a fifth MappingClaim field

**Benefit:** conflict checker becomes explicit and simple.  
**Cost:** duplicates realization semantics and breaks the intentionally minimal four-field claim shape. Backend mappings with selection or aggregation effects still need richer data than a flat write set.

**Rejected unless implementation evidence proves realization cannot expose effects reliably.**

### Alternative B — Use obligation keys to encode every shared backend slot

**Benefit:** no new effect model.  
**Cost:** leaks backend object/property vocabulary into obligation identity and cannot safely handle dynamic selection overlap.

**Rejected.**

### Alternative C — Adapter-only implicit collision checking

**Benefit:** smallest Core.  
**Cost:** no inspectable MappingPlan semantics and inconsistent cross-adapter diagnostics.

**Insufficient for freeze.**

## Thermal example

```text
Claim A
source      = conductivity-1
obligation  = coefficient-binding
realization.effect = WRITE Solid.k := material.k on domain-1

Claim B
source      = material-assignment-1
obligation  = property-source
realization.effect = WRITE Solid.k := user_expression on domain-1
```

Sources and obligations differ. Normalized effect overlap detects a collision on `(Solid.k, domain-1)`.

## Plasma example

```text
Claim A
source      = wall-electron-bc
obligation  = electron-wall-condition
realization.effect = WRITE boundary-flux on selection {3,4,5}

Claim B
source      = absorbing-wall-model
obligation  = particle-wall-model
realization.effect = WRITE incompatible boundary-flux on selection {5,6}
```

The overlap at boundary 5 triggers compatibility evaluation even though the SOL sources and obligations differ.

## SOL v0.1 freeze impact

**Blocker until clarified and implemented in MappingPlan validation.**

The top-level `MappingRule` five-field and `MappingClaim` four-field contracts survive.

---

# 4. AR-02 — MappingPlan dependency DAG

## Validation report counterexample

The report observes that treating an entire backend realization as one DAG node can create a false cycle:

```text
create physics
attach species to physics
create reaction requiring species
configure physics from reaction set
```

If `create physics` and `configure physics` are fused, a logical cycle can appear even though execution is phase-separable. Conversely, if configuration effects are omitted, the graph may look acyclic while execution remains order-sensitive.

## Does the counterexample actually hold?

**Yes.**

MOOSE's Action system explicitly defines task dependencies and allows applications to add task prerequisites. This is strong evidence that setup ordering is an action-level concern rather than merely an object-level dependency concern.

COMSOL's API likewise separates entity creation from subsequent configuration (`create`, `set`, `selection`, feature creation). A single object identity therefore does not correspond to one indivisible execution action.

## Final defect classification

**Architecture defect — MappingPlan execution semantics are under-specified.**

ADR-0010 correctly places dependency information inside `realization` and requires a DAG, but `create / attach / configure` is illustrative rather than a normative action-node contract. Without node granularity, cycle and prerequisite checks are not deterministic.

## Can existing ADRs already solve it?

**Partially.**

ADR-0010 already permits `create / attach / configure`, `produces`, and `requires / depends_on`, so no sixth MappingRule field is required. What is missing is a normative internal MappingPlan node model.

## Required normative clarification

Use **action/effect nodes**, not one node per SOL construct or one node per backend object.

Minimum `PlanAction` contract:

```text
PlanAction
├── id
├── requires[]      # handles/resources that must exist before action
├── produces[]      # handles/resources made available by action
├── effects[]       # normalized effects, shared with AR-01
└── idempotency     # deterministic replay/duplicate semantics
```

`create`, `attach`, and `configure` are generic effect/action kinds, not fixed lifecycle phases that every backend must implement.

The DAG is derived from `requires -> producer(produces)` relationships plus any explicit adapter-required ordering between noncommutative effects.

Normative diagnostics:

1. **Unresolved prerequisite:** a required handle has neither a prior/external binding nor exactly one acceptable producer.
2. **Duplicate producer:** multiple actions produce the same exclusive handle unless the adapter proves them equivalent/idempotent.
3. **Mutation collision:** unordered actions have overlapping noncommutative effects.
4. **False cycle prevention:** separate create/configure actions for the same resource may occur at different graph positions.
5. **Idempotency:** replay semantics MUST be declared as at least `idempotent` or `non-idempotent`; equivalent idempotent actions MAY be coalesced.

## Minimal architecture change if clarification is considered insufficient

Define `PlanAction` and normalized `Effect` as explicit MappingPlan internal constructs in a follow-up ADR. They remain under `MappingRule.realization`; no MappingRule top-level field changes.

## Alternatives and trade-offs

### Alternative A — Fixed phased realization (`create -> attach -> configure`)

**Benefit:** simple scheduler and diagnostics.  
**Cost:** assumes a universal backend lifecycle. Some backends require interleaved creation/configuration or domain-specific phases.

**Useful as a profile convention, too rigid for Core.**

### Alternative B — Arbitrary action/effect nodes

**Benefit:** fits MOOSE task ordering, COMSOL API sequences, Ansys API workflows, N:1 aggregation, and incremental setup.  
**Cost:** slightly richer plan representation and validation.

**Recommended.**

### Alternative C — Adapter executes realization imperatively without plan action nodes

**Benefit:** least schema work.  
**Cost:** loses dry-run explainability and deterministic preflight, contradicting the purpose of MappingPlan.

**Rejected for v0.1 freeze.**

## Thermal example

MOOSE transient heat mapping should plan at least:

```text
A1 produce variable T
A2 require T -> add HeatConduction(T)
A3 require T,density,specific_heat -> add HeatConductionTimeDerivative(T)
A4 require T -> add BCs
A5 require formulation -> configure transient Executioner
```

The official MOOSE transient example contains both `HeatConduction` and `HeatConductionTimeDerivative` alongside a Transient Executioner; execution-mode selection alone is not the formulation.

## Plasma example

COMSOL-style reaction grouping can plan:

```text
P1 create plasma physics context -> handle plas
P2 require plas -> create species -> species handles
P3 require species -> create reaction-group container -> rg
P4 require rg,species -> add individual reaction rows
P5 require completed reaction set -> configure dependent physics/reaction settings
```

The same backend resource may be created early and configured later without a cycle.

## SOL v0.1 freeze impact

**Blocker until action/effect granularity and diagnostics are normative and executable in the MappingPlan validator.**

The five-field MappingRule shape survives.

---

# 5. AR-03 — Collection constraint semantics

## Validation report counterexample

The plasma preflight requires semantics equivalent to:

> an attachment reaction's products contain at least one negative-ion species.

A path such as `products[*]` combined with ordinary Type/Compatibility semantics is ambiguous. It could mean all products, any product, or merely that the collection allows that type.

## Does the counterexample actually hold?

**Yes.**

ADR-0007 explicitly defers collection quantifiers. Its existing primitives can constrain total relation cardinality and endpoint type, but no rule states how to constrain the cardinality of a typed subset of relation targets.

The backend domain also justifies the need. COMSOL plasma chemistry represents reactions with explicit reactants/products and supports large reaction groups, including chemistries with negatively charged species such as `O2-` and `SF6`-family negative ions. A reaction-level existential product condition is therefore a natural domain invariant rather than a hypothetical query-language feature.

## Final defect classification

**Architecture defect — constraint expressiveness gap in ADR-0007.**

This is the strongest of the four findings because the current ADR deliberately deferred the required semantics.

## Can existing ADRs already solve it?

**No, not unambiguously.**

A domain ontology can invent a derived relation such as `has_negative_ion_product`, but then it must define how that relation is derived from `products`. Without Core-level semantics, this merely moves the quantifier into an implicit adapter/domain rule.

## Option comparison

### Option A — General `any / all / none` predicate quantifiers

Example:

```text
any(products, type_is NegativeIonSpecies)
```

**Advantages**
- expressive and familiar;
- useful beyond cardinality/type constraints.

**Costs**
- turns the minimal predicate vocabulary toward a general collection query language;
- raises nesting, empty-collection, path, and three-valued/unknown evaluation questions;
- broadens Core much more than the observed counterexample requires.

### Option B — Qualified / filtered relation cardinality

Example:

```yaml
type: cardinality
relation: products
qualified_type: NegativeIonSpecies
min: 1
```

Normative meaning:

```text
count({ target in products | target is NegativeIonSpecies }) >= 1
```

**Advantages**
- reuses existing Cardinality and Type semantics;
- domain-neutral;
- exactly expresses the counterexample;
- supports deterministic intersection/refinement using the existing type hierarchy;
- does not introduce a general-purpose collection predicate language.

**Costs**
- Cardinality's internal schema becomes slightly richer;
- intersections between qualified cardinalities require qualifier-subtype reasoning.

### Option C — Domain-specific relation redesign

Example:

```text
AttachmentReaction --negative_ion_product--> NegativeIonSpecies
cardinality min = 1
```

**Advantages**
- no Core constraint change.

**Costs**
- duplicates/derives information already represented by `products`;
- scales poorly to `positive_ion_product`, `electron_product`, excited-state subsets, etc.;
- risks inconsistent duplicated relations.

## Recommended minimal change

Adopt **qualified relation cardinality** as a refinement of the existing Cardinality primitive.

Minimal extension:

```text
CardinalityConstraint
├── relation
├── min/max
└── qualifier?       # initially a semantic endpoint type/interface requirement
```

For v0.1, the qualifier SHOULD remain narrower than the full Predicate language. A type/interface semantic qualifier is sufficient for the validated counterexample.

This is semantically equivalent to the existential statement required here without adding `any/all/none` as general Predicate primitives.

Because ADR-0007 explicitly deferred collection quantifiers, this should be adopted through a **follow-up ADR or explicit ADR-0007 amendment proposal**, not silently treated as already specified.

## Thermal example

A thermal domain can state, if needed:

```text
BoundaryCondition.targets
min qualified cardinality 1 of Boundary
```

More useful multi-material examples can constrain an assignment relation to include at least one target satisfying a particular semantic subtype without inventing a new relation.

## Plasma example

```yaml
constraint:
  type: cardinality
  relation: products
  qualifier:
    type: NegativeIonSpecies
  min: 1
```

For an attachment reaction such as a chemistry in which one product is a negatively charged species, the constraint is unambiguous even if other products are neutral.

## SOL v0.1 freeze impact

**Architecture blocker.**

Unlike AR-01/02/04, this cannot be solved solely by clarifying ADR-0010. A constraint architecture decision is required.

---

# 6. AR-04 — Representability evaluation lifecycle

## Validation report counterexample

The report identifies cases where no terminal backend representability conclusion is justified:

- backend product/application is not yet selected;
- backend release is unresolved;
- capability discovery fails;
- a required profile/rule is missing or unresolved;
- claim/effect compatibility cannot yet be decided;
- a MappingPlan prerequisite is unresolved.

Calling these cases `unsupported` incorrectly attributes incomplete evaluation to a backend semantic limitation.

## Does the counterexample actually hold?

**Yes.**

ADR-0010 states that the terminal classification depends on Profile intent, concrete SOL model, Adapter capabilities, and backend release/runtime conditions. Therefore the architecture itself already assumes an evaluation process that can fail to obtain one or more of those inputs.

MOOSE is a framework whose available object vocabulary depends on the selected application/modules. Ansys is a vendor/product family: Mechanical, Chemkin, and Charge Plus expose materially different formulations. Charge Plus itself documents multiple solver/capability families including 3D particle transport, charging, chemistry, and plasma-dynamics functionality. A generic unresolved target cannot truthfully be labelled `unsupported`.

## Final defect classification

**Architecture defect — lifecycle/state clarification required; representability taxonomy itself is not defective.**

## Can existing ADRs already solve it?

**Partially.**

ADR-0010 correctly defines only semantic terminal outcomes, but lacks a distinct MappingPlan evaluation state. The fix does not require a fifth representability class.

## Required normative clarification

Retain:

```text
RepresentabilityOutcome
= exact | transformed | lossy | unsupported
```

Add a separate evaluation lifecycle:

```text
MappingPlanEvaluationState
= pending | blocked | indeterminate | complete
```

Semantics:

- `pending`: evaluation has not completed yet;
- `blocked`: required external/prerequisite evidence is unavailable or unresolved (target, release, module, capability discovery, prerequisite producer, required profile dependency);
- `indeterminate`: required inputs are present but the compatibility/effect comparator cannot decide deterministically;
- `complete`: a terminal representability outcome is available.

Invariant:

```text
state == complete
=> representability outcome MUST be one of exact/transformed/lossy/unsupported

state != complete
=> no terminal representability outcome may be asserted as authoritative
```

A missing rule should be classified carefully:

- if the Profile declares that the semantic construct is unsupported for a fully resolved target, terminal `unsupported` is valid;
- if the rule set is incomplete/unknown or the target is unresolved, evaluation is `blocked`, with Profile/Adapter provenance.

## Alternatives and trade-offs

### Alternative A — Add `blocked` as fifth representability outcome

**Benefit:** simplest enum.  
**Cost:** mixes epistemic/process failure with semantic backend capability and makes `unsupported` versus `blocked` unstable as evidence arrives.

**Rejected.**

### Alternative B — Use only `indeterminate`

**Benefit:** smaller state model.  
**Cost:** conflates missing prerequisites with a genuine undecidable comparison, reducing diagnostics and remediation quality.

**Acceptable minimum, but weaker than separate blocked/indeterminate.**

### Alternative C — Separate evaluation state and terminal outcome

**Benefit:** clean semantic boundary, good provenance, supports asynchronous/external capability discovery without changing ontology meaning.

**Recommended.**

## Thermal example

```text
Target: Ansys Mechanical 2026 R1
Adapter loaded: yes
Required thermal capability discovered: yes
Plan effects compatible: yes
=> state = complete
=> outcome = transformed
```

If the Mechanical adapter cannot inspect the installation/modules:

```text
state = blocked
reason = capability-discovery-failed
```

not `unsupported`.

## Plasma example

```text
Target: "Ansys" only
product/formulation unresolved
=> state = blocked
=> no representability outcome
```

After resolving target to Charge Plus with a particle formulation, a fluid drift-diffusion SOL model may still be semantically non-equivalent; only then can the Profile/Adapter evaluate whether mapping is `lossy` or `unsupported`.

## SOL v0.1 freeze impact

**Blocker until the lifecycle state is normatively separated and implemented.**

No MappingRule or MappingClaim field count changes are required.

---

# 7. Additional issue — Profile backend target identity

## Validation report concern

A Profile target of only `MOOSE` or `Ansys` is too broad. Plasma realization depends on a specific MOOSE application/module/object vocabulary, while Ansys products expose materially different mathematical formulations.

## Does the concern hold?

**Yes.**

Official evidence supports it:

- MOOSE runtime capability is application/module/object dependent rather than a single closed solver vocabulary.
- Ansys product families are distinct. Charge Plus advertises particle transport, charging, chemistry, and plasma-dynamics capabilities; Chemkin exposes reaction-mechanism and surface-kinetics APIs; Mechanical provides thermal/structural analysis contexts.

A vendor/framework umbrella does not identify a unique realization semantics.

## Final defect classification

**Primary classification: Profile defect.**

**Secondary architecture status: ADR-0010 normative clarification required, not a new top-level field requirement.**

ADR-0010 already permits Profile identity/version, `target backend adapter binding`, backend release compatibility, MappingRules, and capabilities. The missing information can fit this structure, but the target binding is currently described as `MAY` and its minimum resolved identity is not normative.

## Recommended target contract

A Profile that is executable SHOULD/MUST resolve a backend target descriptor containing at least:

```text
BackendTarget
├── adapter_identity           # required
├── product_or_application     # required when adapter supports multiple targets
├── formulation               # required when product exposes materially different mathematical models
├── release_compatibility      # required when mapping depends on release
└── required_capabilities/modules  # adapter-defined
```

The vocabulary for product/application, formulation, modules, and compatibility scopes remains adapter-defined under ADR-0009. Core does not define Ansys or MOOSE product taxonomies.

This descriptor can live under the existing Profile `target backend adapter binding` plus release/capability metadata; no sixth MappingRule field is required.

## Thermal example

```text
adapter: ansys-mechanical-adapter
product/application: Mechanical
formulation: Steady-State Thermal
release: 2026 R1
capabilities: thermal, engineering-data, geometry-scoping
```

## Plasma example

MOOSE:

```text
adapter: moose-input-adapter
application: specific Zapdos/CRANE/custom app identity
release/versioner: adapter-defined
capabilities: registered plasma objects actually required by rules
```

Ansys:

```text
adapter: charge-plus-adapter
product: Charge Plus
formulation: explicit Profile-selected particle/plasma formulation
release: 2026 R1 (or compatible range)
capabilities/modules: adapter-defined
```

A different Profile may target Chemkin for reaction-network semantics, but it must not be treated as automatically equivalent to a spatial drift-diffusion/PDE plasma model.

## Freeze impact

**Profile/reference-model blocker for executable validation.**

A small ADR-0010 clarification is advisable so future Profiles cannot claim executable status with only a vendor umbrella name.

---

# 8. Consolidated decision table

| Finding | Counterexample valid? | Final classification | Existing ADR sufficient? | Minimum remedy | Top-level shape impact |
|---|---|---|---|---|---|
| AR-01 | Yes | Architecture defect (normative compatibility gap) | Partially | realization effect/resource contract + all-claim collision pass | none |
| AR-02 | Yes | Architecture defect (MappingPlan execution semantics gap) | Partially | action/effect PlanAction nodes with requires/produces/effects/idempotency | none |
| AR-03 | Yes | Architecture defect (constraint expressiveness gap) | No | qualified relation cardinality; follow-up constraint decision | no new primitive required |
| AR-04 | Yes | Architecture defect (evaluation lifecycle gap) | Partially | separate plan evaluation state from terminal representability | none |
| Backend target identity | Yes | Profile defect + ADR-0010 clarification | Mostly | resolved adapter/product/application/formulation/release/capability target descriptor | fits Profile metadata |

The independent report also identifies missing executable reference models, Profiles, Adapters, generated plans, artifacts, and execution evidence. Those remain **Reference-model / Profile / Adapter / Validation-tooling defects**, not evidence that SOL Core is structurally wrong.

---

# 9. Architecture change proposal and impact analysis

## Proposal P1 — Clarify ADR-0010 claim compatibility

Require normalized realization effects and cross-claim collision checking across **all** composed claims, not only matching `(source, obligation)` keys.

**Impact:** MappingClaim remains four fields; MappingRule remains five fields. Adapters gain an effect normalization/comparator responsibility; MappingPlan gains inspectable collision diagnostics.

## Proposal P2 — Clarify ADR-0010 MappingPlan action semantics

Define action/effect granularity with plan-local handles, prerequisites, producers, effects, and idempotency. Do not impose fixed backend-native object phases.

**Impact:** internal MappingPlan contract becomes richer; no MappingRule top-level change.

## Proposal P3 — Follow-up constraint decision for qualified cardinality

Extend Cardinality with a semantic target qualifier sufficient to express `min 1 NegativeIonSpecies product`.

**Impact:** small Core constraint extension and validator/intersection work. No new constraint primitive or general collection query language is required.

## Proposal P4 — Clarify ADR-0010 evaluation lifecycle

Separate `MappingPlanEvaluationState` from terminal representability.

**Impact:** MappingPlan/report schema only; the semantic outcome taxonomy remains four classes.

## Proposal P5 — Clarify executable backend target resolution

Require executable Profiles to resolve a concrete adapter target; product/application/formulation/release/module details remain adapter-defined and use existing Profile target/release/capability metadata.

**Impact:** Profile validation becomes stricter but Core remains backend-neutral.

---

# 10. Final requested verdicts

## 10.1 AR-01–AR-04 final classifications

```text
AR-01 -> Architecture defect (normative realization-compatibility gap)
AR-02 -> Architecture defect (MappingPlan execution/action granularity gap)
AR-03 -> Architecture defect (Constraint/Cardinality expressiveness gap)
AR-04 -> Architecture defect (MappingPlan evaluation lifecycle gap)
```

These classifications do **not** convert missing executable Profiles/Adapters/reference models/tooling into Architecture defects; those remain separately classified as in the independent report.

## 10.2 Can MappingRule remain five fields?

**YES.**

No reviewed counterexample requires a sixth top-level field. AR-01/02 information belongs inside `realization`; AR-04 belongs to MappingPlan evaluation; target identity belongs to Profile metadata; AR-03 belongs to Constraint architecture.

## 10.3 Can MappingClaim remain four fields?

**YES.**

The required write/effect information can remain inside `realization`. A fifth field is not justified by the evidence if effect normalization is normative.

## 10.4 Is ADR-0010 clarification alone sufficient?

**NO.**

ADR-0010 clarification can address AR-01, AR-02, AR-04, and backend target-resolution requirements. AR-03 affects ADR-0007's deliberately deferred collection semantics and therefore requires a separate constraint decision.

## 10.5 Is a follow-up ADR required?

**YES. At least one follow-up ADR is required.**

The minimum unavoidable follow-up is a constraint ADR for qualified relation cardinality (or an equivalent collection-semantic mechanism). A separate MappingPlan supplement ADR is also preferable to overloading ADR-0010 with operational details, but AR-01/02/04 can technically be handled as an explicit ADR-0010 clarification proposal if project governance prefers fewer ADRs.

Recommended governance split:

```text
ADR-0010 clarification/supplement
  -> realization effects
  -> PlanAction semantics
  -> evaluation lifecycle
  -> executable BackendTarget resolution

Follow-up Constraint ADR
  -> qualified relation cardinality
```

## 10.6 Can SOL v0.1 architecture freeze be revalidated after resolution?

**YES, but not immediately approved.**

After the normative gaps above are resolved, Validation Lab must rerun the execution gate with:

1. a concrete solvable Thermal SOL model;
2. serialized Profiles and actual five-field MappingRules;
3. generated four-field MappingClaims and PlanActions/effects;
4. MappingPlan conflict/dependency/evaluation validation;
5. resolved product/application/formulation targets for all three backend families;
6. generated backend artifacts and, where tooling/licensing is available, backend execution evidence;
7. identity and constraint-preservation traces;
8. a Plasma model exercising qualified product cardinality and N:1 reaction-group mapping.

Only that rerun can approve the architecture freeze under the existing validation plan.
