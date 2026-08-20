# SOL v0.1 Independent Cross-Backend Architecture Validation

**Status:** Complete — architecture freeze not approved  
**Date:** 2026-08-20  
**Validation basis:** [Cross-Backend Reference Validation Plan v0.1](cross-backend-reference-validation-plan-v0.1.md)  
**Primary decision under test:** [ADR-0010 — Profile and Backend Mapping Contract](../decisions/0010-profile-backend-mapping-contract.md)  
**Source branch reviewed:** `init-ontology-v0.1`

## 1. Executive verdict

SOL v0.1 is **directionally sound but not ready for architecture freeze**.

The Profile/BackendAdapter separation, five top-level `MappingRule` concerns, explicit `MappingPlan`, stable SOL identity, and the distinction between semantic validity and backend representability are supported by the three backend families. No evidence requires copying a backend-native object model into Core.

However, the validation plan's pass criteria were not met:

1. no concrete Thermal reference-model instance exists;
2. no executable MOOSE, COMSOL, or Ansys Profile/Adapter exists;
3. no MappingClaims or MappingPlan can be generated and inspected;
4. no backend artifact can be created or compared;
5. conflict detection is under-specified for cross-obligation realization collisions;
6. MappingPlan dependency semantics do not yet define action/effect granularity;
7. representability outcomes lack a separate blocked/indeterminate evaluation state;
8. the target `Ansys` is too broad to identify a plasma formulation or adapter;
9. the plasma constraint model exposes an unresolved collection-quantifier counterexample.

The Thermal model therefore **passes only a paper representability check** and **fails the execution gate**. Under the validation plan, the full Plasma execution stress test was not run. A documentation-level plasma preflight was performed only to identify freeze-blocking counterexamples.

## 2. Method and independence

The review did not assume that ADR-0010 was correct. It:

- read the validation plan and ADRs 0006, 0007, 0009, and 0010;
- inspected the repository's Core YAML and backend directories;
- treated research examples as design claims, not executed evidence;
- compared projection claims with official MOOSE, COMSOL, and Ansys documentation;
- attempted to falsify the minimal contracts using scope, identity, composition, dependency, unit, and plasma-chemistry counterexamples;
- classified findings before recommending any remediation.

No architecture files were modified.

## 3. Evidence inventory

### Present

- architecture and language documents;
- accepted ADRs;
- Thermal mapping research sketch;
- mapping-rule schema stress-test narrative;
- validation plan;
- draft Core entity, relation, and constraint YAML;
- placeholder backend directories.

### Missing

- concrete SOL Thermal instance with geometry/domain, values, units, equation/formulation, selections, and identities;
- concrete SOL Plasma instance;
- serialized Profiles containing actual five-field MappingRules;
- generated four-field MappingClaims;
- MappingPlan schema and generator;
- conflict comparator;
- capability discovery;
- MOOSE adapter and target application/version;
- COMSOL adapter and module/version binding;
- Ansys product/solver-specific adapter and version binding;
- generated backend artifacts;
- backend executions and result comparisons;
- identity and constraint-preservation traces.

This absence is not treated as proof that the architecture is wrong. It is a **Validation-tooling defect** and, where mapping policy is absent, a **Profile/Adapter defect**. It prevents a PASS.

## 4. Thermal reference-model gate

### 4.1 Reference-model adequacy

The validation plan lists Material, ThermalConductivity, TemperatureField, BoundaryCondition, Study, Scope, and ValueDefinitions. That is not yet a concrete solvable model.

At minimum, an executable reference needs:

- a geometry/domain or imported mesh with stable scope identities;
- an explicit heat-equation/formulation contract;
- material assignment to a domain;
- conductivity value, physical dimension, and unit;
- boundary selections and prescribed values;
- enough boundary/source information for a well-posed problem;
- steady or transient study selection;
- for transient analysis, density, heat capacity, initial condition, and time domain;
- stable model-instance identities and expected projection traces.

**Finding TH-RM-01 — Reference-model defect (fatal to execution validation):** the repository contains a semantic sketch, not an executable reference instance.

### 4.2 MOOSE paper projection

A minimal steady conduction projection is plausible:

```text
TemperatureField
  -> Variable(T)
  -> HeatConduction kernel(variable=T)

ThermalConductivity
  -> HeatConductionMaterial / named material property

PrescribedTemperatureBC
  -> DirichletBC or FunctionDirichletBC
  -> variable + boundary bindings

SteadyStudy
  -> steady execution context
```

Official MOOSE documentation confirms that `HeatConduction` implements the diffusion term using a thermal-conductivity material property and a temperature variable. `HeatConductionMaterial` supplies thermal conductivity and specific heat. For a transient formulation, `HeatConductionTimeDerivative` is a separate kernel using density and specific heat.

Evidence:

- [MOOSE HeatConduction](https://mooseframework.inl.gov/source/kernels/HeatConduction.html)
- [MOOSE HeatConductionMaterial](https://mooseframework.inl.gov/source/materials/HeatConductionMaterial.html)
- [MOOSE HeatConductionTimeDerivative](https://mooseframework.inl.gov/source/kernels/HeatConductionTimeDerivative.html)
- [MOOSE FunctionDirichletBC](https://mooseframework.inl.gov/source/bcs/FunctionDirichletBC.html)

**Assessment:** the conceptual mapping is `transformed`, not `exact`, because field, equation term, material property, BC, and study are separate backend objects/contexts.

**Counterexample TH-MOOSE-01:** mapping `TransientStudy -> Transient Executioner` without adding the time-derivative equation term produces a runnable-looking but semantically incomplete transient model. Different obligations can affect the same effective formulation without being the same source/obligation pair.

**Classification:** Profile defect if the rule is missing; Adapter defect if the rule is present but realization omits it; Architecture defect if the plan cannot express or detect the coupled obligation.

**Backend verdict:** **FAIL** — backend representability supported, but no Profile, MappingPlan, adapter realization, artifact, or execution evidence exists.

### 4.3 COMSOL paper projection

A minimal projection is plausible:

```text
TemperatureField + heat formulation
  -> Heat Transfer in Solids physics and dependent temperature variable

Material + ThermalConductivity
  -> Material/Solid property context and domain selection

PrescribedTemperatureBC
  -> Temperature feature + boundary selection + value/expression

Study
  -> Stationary or Time Dependent study step
```

COMSOL's official materials guidance identifies thermal conductivity, density, and heat capacity as Solid-node inputs and allows them to come from material data; temperature dependence can use the temperature dependent variable. The official capability chart lists Heat Transfer in Solids and thermal boundary conditions.

Evidence:

- [COMSOL Heat Transfer specification chart](https://www.comsol.com/products/specifications/heat-transfer/)
- [COMSOL material properties for heat transfer](https://www.comsol.com/blogs/using-the-material-libraries-in-comsol-multiphysics)

**Assessment:** `transformed`; the temperature field is physics-context-owned and selections are separate realization objects.

**Counterexample TH-COMSOL-01:** a conductivity definition and a material assignment can be individually valid yet conflict through selection overlap or a physics property configured as “From material” versus user-defined. These can be different MappingClaim obligations but compete for the same effective property source.

**Classification:** Architecture defect for the missing generic cross-obligation effect-collision contract; Profile defect for an unmodelled ownership rule.

**Backend verdict:** **FAIL** — representability supported, execution evidence absent.

### 4.4 Ansys Mechanical paper projection

A minimal thermal projection is plausible only after narrowing `Ansys` to **Ansys Mechanical Steady-State Thermal** or **Transient Thermal**:

```text
Material + ThermalConductivity
  -> Engineering Data + body material assignment

TemperatureField
  -> thermal analysis temperature DOF/context

PrescribedTemperatureBC
  -> Thermal Condition + geometry scoping + magnitude

Study
  -> Steady-State Thermal or Transient Thermal analysis
```

Official Ansys Mechanical documentation requires thermal conductivity for steady-state thermal analysis and allows constant or temperature-dependent, isotropic or orthotropic values. A known temperature is inserted as a Thermal Condition with a magnitude and geometric application semantics.

Evidence:

- [Ansys Steady-State Thermal Analysis](https://ansyshelp.ansys.com/public/Views/Secured/corp/v252/en/wb_sim/ds_static_thermal_analysis_type.html)
- [Ansys Thermal Condition](https://ansyshelp.ansys.com/public/Views/Secured/corp/v261/en/wb_sim/ds_therm_cond.html)

**Assessment:** `transformed`.

**Counterexample TH-ANSYS-01:** for a surface body, a Thermal Condition applies to both top and bottom faces by default unless Shell Face is refined. A SOL scope meaning “one physical face” can therefore be silently broadened by a backend default.

**Classification:** Profile defect if scoping semantics omit shell-face intent; Adapter defect if the intent is present but not applied; Backend limitation only if the requested distinction cannot be represented.

**Backend verdict:** **FAIL** — backend representability supported, but the plan targets generic “Ansys,” and no Mechanical-specific Profile/Adapter or execution evidence exists.

### 4.5 Thermal gate result

| Check | MOOSE | COMSOL | Ansys Mechanical |
|---|---:|---:|---:|
| Official backend construct exists | PASS | PASS | PASS |
| Same SOL meaning appears projectable | PASS (paper) | PASS (paper) | PASS (paper) |
| Concrete five-field rules exist | FAIL | FAIL | FAIL |
| Four-field claims generated | FAIL | FAIL | FAIL |
| MappingPlan DAG generated | FAIL | FAIL | FAIL |
| Identity trace inspected | FAIL | FAIL | FAIL |
| Constraint trace inspected | FAIL | FAIL | FAIL |
| Backend artifact inspected/executed | FAIL | FAIL | FAIL |
| **Overall thermal backend verdict** | **FAIL** | **FAIL** | **FAIL** |

The Thermal gate did not pass. The validation plan therefore does not authorize claiming that the Plasma execution stress test passed.

## 5. Plasma preflight only

### 5.1 COMSOL

COMSOL 6.4 official documentation supports a coherent fluid/plasma target:

- Drift Diffusion computes electron density and mean electron energy;
- plasma chemistry contains electron-impact, heavy-species, and surface reactions;
- surface-reaction groups are separated by boundary selection;
- reaction groups have backend-specific grouping and selection semantics.

Evidence:

- [COMSOL Plasma Modeling](https://doc.comsol.com/6.4/doc/com.comsol.help.plasma/plasma_introduction.02.02.html)
- [COMSOL Plasma Module User's Guide](https://doc.comsol.com/6.4/doc/com.comsol.help.plasma/PlasmaModuleUsersGuide.pdf)

A SOL plasma model could therefore project to COMSOL, mostly as `transformed`. However, grouping multiple SOL reactions into a Reaction Group creates an N:1 realization with aggregation identity, selection, and lifecycle semantics not yet demonstrated by the MappingPlan contract.

**Preflight verdict:** representable in principle; **not validated**.

### 5.2 MOOSE

“MOOSE” is a framework, not one fixed plasma application contract. A plasma projection requires a selected application/module and available registered objects, for example a specific Zapdos/CRANE/custom application version. The current Profile does not identify one, and the backend directory is a placeholder.

**Finding PL-MOOSE-01 — Profile defect:** no plasma application identity, object vocabulary, capability contract, or release scope.

**Preflight verdict:** **indeterminate**, not legitimately `unsupported`. The backend may support the model through an application, but the target and capabilities were not resolved.

### 5.3 Ansys

“Ansys” is not a single plasma backend. Official products expose materially different formulations:

- Ansys Chemkin documents electrons/species, plasma reaction mechanisms, and surface kinetics;
- Ansys Charge Plus exposes particle-in-cell/plasma-dynamics capabilities and electron-energy-distribution analysis;
- Ansys Mechanical is the thermal target but not the same plasma formulation.

Evidence:

- [Ansys Chemkin API Manual](https://ansyshelp.ansys.com/public/Views/Secured/corp/v261/en/pdf/Ansys_Chemkin_Application_Programming_Interface_Manual.pdf)
- [Ansys Charge Plus](https://ansys.synopsys.com/products/electronics/ansys-charge-plus)

A fluid drift-diffusion `ElectronDensityField` is not automatically semantically equivalent to a PIC realization, and a Chemkin reaction mechanism does not by itself realize a spatial electron-density field and wall-boundary PDE model.

**Finding PL-ANSYS-01 — Reference-model defect:** “Ansys” does not identify the intended product and mathematical formulation.

**Finding PL-ANSYS-02 — Profile defect:** no product-specific mapping/capability contract distinguishes Mechanical, Chemkin, Charge Plus, or a coupled workflow.

**Preflight verdict:** **indeterminate**. Choosing a product without an explicit formulation contract would risk a semantic substitution.

## 6. Contract sufficiency assessment

### 6.1 MappingRule five-field structure

```text
source
applicability
realization
bindings
capabilities
```

**Verdict: CONDITIONALLY SUFFICIENT as top-level partitioning; not sufficiently specified for freeze.**

All tested information can be placed under one of the five fields. No sixth top-level field is required by the reviewed examples. The defect lies inside the contracts:

- `source` needs explicit single/set/pattern and scope semantics;
- `realization` needs addressable action/effect nodes, produced handles, mutation targets, and lifecycle phase;
- `bindings` needs unit conversion, selection/scoping, ownership, and value-source semantics;
- `capabilities` needs versioned capability evidence and discovery provenance;
- `applicability` needs defined collection/quantifier semantics when paths select multiple objects.

Retaining five fields is reasonable, but their internal normative schemas must be frozen before architecture freeze.

### 6.2 MappingClaim four-field structure

```text
obligation
source
realization
provenance
```

**Verdict: MINIMAL DATA SHAPE MAY BE SUFFICIENT; conflict semantics are not.**

A realization can carry backend target/effect information, so a fifth required field is not yet proven necessary. But ADR-0010's canonical invariant is incomplete:

```text
same source + same obligation + incompatible realization => conflict
```

#### Counterexample MC-01 — Cross-obligation write collision

```text
Claim A:
  source = ThermalConductivity
  obligation = coefficient-binding
  realization = set Solid.k = material.k

Claim B:
  source = MaterialAssignment
  obligation = material-source
  realization = set Solid.k = user_expression
```

Sources and obligations differ, but both claim incompatible ownership of the same effective backend slot. The canonical invariant does not require a conflict.

#### Counterexample MC-02 — Scope overlap collision

Two BC claims can have different SOL source identities but overlapping resolved backend selections and incompatible prescribed values. Equality of `source` does not detect the conflict.

**Finding AR-01 — Architecture defect:** conflict detection needs realization compatibility/effect-collision checks across all composed claims, not only the same-source/same-obligation key. The four-field shape can survive if `realization` has a canonical effect/write-set contract and the MappingPlan is required to compare it.

### 6.3 Rule composition

**Verdict: FAIL for freeze.**

Additive composition and no declaration-order priority are good principles. Missing normative items are:

- overlapping source-pattern resolution;
- cross-obligation compatibility;
- backend resource ownership;
- duplicate/idempotent realization detection;
- scope-overlap evaluation;
- aggregation rules for N:1 contexts such as COMSOL Reaction Groups;
- deterministic diagnostics when compatibility cannot be decided.

### 6.4 MappingPlan dependency DAG

**Verdict: concept accepted; contract incomplete.**

A DAG is appropriate for creation dependencies, but a graph is only correct if its nodes and edges are well defined.

#### Counterexample MP-01 — Object-level DAG hides phase conflict

```text
create physics
attach species to physics
create reaction requiring species
configure physics from reaction set
```

If one node represents the entire “physics realization,” dependencies can appear cyclic even though its create and configure phases are acyclic. Conversely, if configuration effects are omitted, the plan can appear acyclic while execution is order-sensitive.

**Finding AR-02 — Architecture defect:** MappingPlan must define action/effect granularity or an equivalent phased realization model. A bare dependency DAG is insufficient.

The plan also needs unresolved prerequisites, duplicate producer, absent producer, and backend-mutation collision diagnostics.

### 6.5 Identity preservation

**Verdict: principle PASS; evidence FAIL.**

ADR-0009's separation of schema identity, model-instance identity, and backend-local identity is correct. However, no generated trace demonstrates:

- every produced backend artifact links to one or more stable SOL identities;
- N:1 aggregation preserves all contributing identities;
- regeneration reuses or deterministically replaces backend-local identities;
- backend tags/names never become semantic references.

**Finding VT-IDENT-01 — Validation-tooling defect:** no identity-trace or round-trip comparator exists.

### 6.6 Constraint preservation

**Verdict: architecture principle plausible; executable preservation not demonstrated.**

Conjunctive composition and monotonic refinement in ADR-0007 are appropriate. The current machine-readable `ontology/core/constraints.yaml` still lists identifiers, units/dimensions, cardinality, required relations, and acyclicity as pending. There is no implication/refinement checker.

#### Counterexample CP-01 — Missing collection quantifier

The plasma study states “attachment products must contain a negative ion.” A compatibility check over `products[*]` is ambiguous:

- every product must be a negative ion;
- at least one product must be a negative ion;
- the collection type merely permits negative ions.

The v0.1 predicate vocabulary explicitly defers `all/any/none`. This semantic requirement cannot be validated unambiguously with Compare, Membership, Exists, and Boolean alone.

**Finding AR-03 — Architecture defect:** either collection quantifier semantics or a domain relation/constraint shape with equivalent existential meaning is required before the proposed plasma reference can be validated.

**Finding VT-CONSTRAINT-01 — Validation-tooling defect:** no canonical normalizer/intersection/refinement implementation exists.

### 6.7 Representability outcomes

```text
exact | transformed | lossy | unsupported
```

**Verdict: sufficient as terminal semantic representability outcomes; insufficient as complete MappingPlan evaluation states.**

They correctly distinguish lossless structural transformation from semantic loss. But a preflight can fail to reach a terminal outcome because:

- backend product/version is unresolved;
- capability discovery failed;
- required plugin/licence is not inspectable;
- mapping rules are missing;
- claim compatibility is undecidable;
- a MappingPlan prerequisite is unresolved.

Calling those cases `unsupported` incorrectly attributes missing evidence to a backend limitation.

**Finding AR-04 — Architecture defect/clarification required:** keep the four terminal outcomes, but define a separate non-terminal evaluation state such as `blocked` or `indeterminate`, with failure provenance. It must not be presented as a fifth semantic representability class.

### 6.8 Diagnostic provenance

**Verdict: schema intent PASS; evidence FAIL.**

MappingClaim provenance is helpful but insufficient by itself. A diagnostic must distinguish rule/profile/adapter/backend/reference/tooling origins and retain capability/version evidence. No report schema or implementation exists.

## 7. Findings by required classification

| ID | Classification | Finding | Freeze impact |
|---|---|---|---|
| TH-RM-01 | Reference-model defect | Thermal sketch is not a concrete solvable instance | Blocker |
| PL-ANSYS-01 | Reference-model defect | Generic Ansys target does not select product/formulation | Blocker |
| PF-01 | Profile defect | No executable five-field profiles for any backend | Blocker |
| PL-MOOSE-01 | Profile defect | No MOOSE plasma application/capability binding | Blocker |
| PL-ANSYS-02 | Profile defect | No Ansys product-specific plasma mapping | Blocker |
| AD-01 | Adapter defect | No adapters/capability discovery/artifact realization | Blocker |
| BL-01 | Backend limitation | No confirmed universal limitation in Thermal; Plasma limitations cannot be assigned before product/profile resolution | None yet |
| VT-01 | Validation-tooling defect | No MappingClaim/MappingPlan generator or conflict checker | Blocker |
| VT-IDENT-01 | Validation-tooling defect | No identity preservation trace/comparator | Blocker |
| VT-CONSTRAINT-01 | Validation-tooling defect | No constraint normalization/intersection/refinement checker | Blocker |
| AR-01 | Architecture defect | Same-source/same-obligation invariant misses cross-obligation and overlapping-scope realization collisions | Blocker |
| AR-02 | Architecture defect | DAG lacks normative action/effect or phase granularity | Blocker |
| AR-03 | Architecture defect | Plasma existential collection constraint is not unambiguously expressible | Blocker |
| AR-04 | Architecture defect | Terminal representability vocabulary lacks a separate blocked/indeterminate evaluation state | Blocker |

## 8. Required contract changes before freeze

This validation does not directly modify architecture. The following items should return to Research Lab for focused ADR review:

1. **Claim compatibility contract:** preserve four required fields if possible, but define canonical realization effects/resources and cross-claim collision checking.
2. **MappingPlan execution model:** define action/effect or phased nodes, producers/requirements, ownership, idempotency, and collision diagnostics.
3. **Evaluation state model:** retain four terminal representability outcomes and add a separate blocked/indeterminate plan state.
4. **Collection semantics:** add minimal existential/universal collection semantics or an equivalent typed constraint construct justified by the plasma counterexample.
5. **Backend target identity:** bind Profiles to a product/application, formulation, adapter, release range, and required modules—not a vendor umbrella name.

Items 1–4 are architecture questions. Item 5 is primarily a Profile/reference-model requirement unless a backend-target identity slot is absent from the Profile contract.

## 9. Final backend verdicts

| Backend target | Thermal | Plasma | Overall |
|---|---|---|---|
| MOOSE framework + unspecified app | FAIL — paper-projectable, no execution | NOT RUN; preflight indeterminate | **FAIL** |
| COMSOL Heat Transfer / Plasma Module | FAIL — paper-projectable, no execution | NOT RUN; preflight projectable | **FAIL** |
| Ansys Mechanical / unspecified plasma product | FAIL — paper-projectable, no execution | NOT RUN; target ambiguous | **FAIL** |

These are validation verdicts for the current repository evidence, not claims that the products lack the required thermal capabilities.

## 10. Freeze decision

### Architecture change required?

**YES.** Focused ADR clarification/revision is required for AR-01 through AR-04. The five-field MappingRule and four-field MappingClaim may remain structurally unchanged, but the composition, effect compatibility, MappingPlan, collection semantics, and evaluation lifecycle contracts require normative additions.

### SOL v0.1 architecture freeze possible?

**NO — FREEZE REJECTED.**

Freeze may be reconsidered when:

1. the architecture counterexamples have accepted dispositions;
2. a concrete Thermal reference instance and three product-specific Profiles exist;
3. dry-run MappingClaims and MappingPlans are generated;
4. conflict, DAG, identity, constraint, and representability evidence is recorded;
5. three backend artifacts are inspected or executed;
6. only after Thermal passes, a concrete Plasma model is run against clearly identified backend products/formulations.

## 11. Evidence limitations

- No installed MOOSE, COMSOL, or Ansys runtime was available through the repository evidence.
- Proprietary backend execution was not claimed.
- Official documentation establishes capability and native structure, not successful SOL adapter implementation.
- Research documents were treated as hypotheses and design provenance, not independent validation evidence.
