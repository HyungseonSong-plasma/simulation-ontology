# SOL v0.1 remediation proposals v0.2 — contract-only independent re-review

**Status:** Independent Validation Lab re-review  
**Date:** 2026-08-20  
**Branch:** `validation/sol-v0.1-independent-review`  
**Scope:** Contract completeness and deterministic implementability only  
**Compared proposals:**
- [ADR-0010 supplement remediation proposal v0.2](../research/sol-v0.1-adr0010-supplement-remediation-proposal-v0.2.md)
- [Qualified Relation Cardinality proposal v0.2](../research/sol-v0.1-qualified-relation-cardinality-constraint-proposal-v0.2.md)

**Prior review:** [SOL v0.1 remediation proposals independent review](sol-v0.1-remediation-proposals-independent-review.md)

## 1. Review rule

This re-review does not assume that the earlier AR-01–04 findings or either Research Lab solution is correct. A prior `Revise` reason is `Resolved` only when the v0.2 normative text forces independent conforming validators, given the same immutable model, Profile, adapter contract/version, target snapshot, and registered procedures, to reach the same decision.

Ratings mean:

| Rating | Meaning |
|---|---|
| `Resolved` | The earlier ambiguity is removed at contract level. |
| `Partially resolved` | The intended boundary is improved, but at least one conforming implementation choice can still change the result. |
| `Unresolved` | The prior ambiguity remains materially intact. |
| `Regression` | v0.2 introduces a worse or contradictory rule. |

Absence of machine-readable schemas or executable fixtures is not a contract failure in this report. It is recorded separately as execution-validation readiness.

## 2. Executive verdict

| Proposal | Contract completeness | Execution-validation readiness | Final disposition |
|---|---|---|---|
| ADR-0010 supplement v0.2 | Substantially improved, but four decision-affecting ambiguities remain | Not ready; no schemas/registry/generator/resolver/backend fixture evidence | **revision required** |
| Qualified Relation Cardinality v0.2 | Core count and subtype algebra are deterministic; two input/result-boundary ambiguities remain | Not ready; no schema/validator/fixture evidence | **revision required** |

No regression was found. Neither proposal unnecessarily imports backend-native semantics into SOL Core. The QRC extension remains a narrow, backend-independent Core/schema addition; the ADR-0010 additions remain MappingPlan/Profile/Adapter contracts.

Because both proposals still require contract revision, **ADR drafting is not yet ready**. This is a contract gate only; even a subsequent contract PASS would not authorize SOL v0.1 architecture freeze without executable validation.

## 3. ADR-0010 supplement v0.2 re-review

### 3.1 Prior Revise reasons

| Prior issue | v0.2 mechanism | Rating | Independent finding |
|---|---|---|---|
| RE-01 resource canonicalization and alias ambiguity | plan-canonical key; same adapter/target snapshot; `same/separate/overlapping/unresolved` evidence | **Partially resolved** | Alias outcomes and provenance are defined, but the contract does not define a complete, deterministic candidate-pair enumeration. “May alias” and “selected as potentially interacting” permit one validator to compare a pair that another prunes. Resource inequality cannot prove disjointness, but no required conservative fallback says that every unproved-disjoint pair must be compared or retained as unresolved. |
| RE-02 optional comparison fields and missing-data boundary | comparison-sufficiency declaration; malformed→FAIL, external missing→BLOCKED, complete-but-undecidable→INDETERMINATE | **Resolved** | The origin of missing information, rather than mere absence, uniquely selects the result. |
| RE-03 circular semantics/comparator evidence | typed `EffectComparison`; comparator and normalization provenance | **Partially resolved** | Provenance makes differing comparator versions visible but does not itself prevent them. The adapter contract must normatively bind each comparison kind/resource kind to exactly one comparator ID/version (or define deterministic selection precedence). Otherwise two installed registered comparators can both be conforming and disagree. |
| RE-04 operation vocabulary ownership | operation vocabulary explicitly adapter/MappingPlan-IR owned | **Resolved** | No backend mutation vocabulary enters Core. |
| PA-01 executable action descriptor | mandatory descriptor; stable adapter operation reference; deterministic `describe_effects`; effect-coverage check | **Partially resolved** | Executability is now required. However, “equivalent to the externally observable mutation surface” and “explicitly declared non-semantic bookkeeping effects” lack a required equivalence procedure/registry binding and a rule for who may classify bookkeeping as non-semantic. The checked and executed surfaces can therefore differ by validator policy. |
| PA-02 ordering direction inferred from noncommutativity | only versioned, provenance-bearing `must_precede` may add direction; otherwise FAIL | **Resolved** | Declaration order and noncommutativity cannot invent direction; cycles deterministically fail. |
| PA-03 state-dependent idempotency | state-independent/state-dependent alternatives; evidence/precondition/postcondition; four coalescing predicates | **Partially resolved** | The required inputs are identified, but comparator version selection is not explicit and `state_independent_idempotent` is accepted as a declaration without a normative proof/adapter guarantee. “Where one exclusive mutation is expected” also lacks a canonical exclusivity source. These choices can change coalescing from PASS to FAIL. |
| PA-04 compound transaction hiding | one atomic descriptor with effects, prerequisites, failure semantics, and idempotency validated as a unit | **Partially resolved** | The guard is correct, but “atomic” has no required observable failure contract (rollback, no externally visible partial effects, or compensating transition). Validators can accept different failure semantics as atomic. |
| EL-01 blocked/indeterminate/tooling-error boundary | common PASS/FAIL/BLOCKED/INDETERMINATE contract | **Resolved** | Missing external prerequisite, malformed contract, missing implementation, and genuine undecidability are separated. |
| EL-02 lifecycle aggregation | per-required-obligation outcomes; BLOCKED precedence over INDETERMINATE; semantic-loss aggregation | **Partially resolved** | Representability aggregation is deterministic after valid item evaluations. The proposal does not define plan-level precedence/reporting when a validation `FAIL` coexists with a blocked or indeterminate obligation. A fail-fast validator and an accumulate-all validator can publish different final plan decisions. |
| EL-03 evaluation completion vs execution permission | independent state, outcome, and permission axes | **Partially resolved** | The default table is deterministic. “Unless stricter Profile policy applies” does not define policy composition/precedence or the canonical representation of allowed identified losses, so identical plans with semantically equivalent but differently ordered policy rules can differ. |
| EL-04 transition determinism | immutable evaluation revisions; only pending→terminal transitions | **Resolved** | Re-evaluation and runtime drift create new revisions rather than mutate historical decisions. |
| BT-01 requirement/runtime split | distinct `BackendTargetRequirement` and `ResolvedBackendTarget` | **Resolved** | Requirement ranges and observed runtime identities are no longer conflated. |
| BT-02 formulation duplication | SOL formulation remains authoritative; one compatible binding per relevant component | **Partially resolved** | Ownership is correct, but “relevant target component” is not normatively derived. In a composite target, validators may attach the same SOL formulation to different component subsets. |
| BT-03 capability duplication | deterministic union of target floor, selected rules, formulation, and orchestration requirements | **Partially resolved** | The sources are fixed, but capability canonical identity/versioning and per-component versus orchestration scope are absent. String-equal, alias, or version-constrained capabilities can produce different unions and outcomes. |
| BT-04 coupled/multi-product semantics | components set; orchestration capability; every action binds to one component or orchestrator; explicit transfer actions | **Partially resolved** | Singular top-level release/formulation/capability collections are not normatively keyed to components, and the orchestration adapter's relation to the selected adapter contract is underspecified. The same composite requirement can resolve differently. |

### 3.2 Direct counterexamples that remain

#### C-ADR-01 — Candidate-pair pruning

Effects A and B have different canonical plan keys and unresolved runtime aliases. Validator V1 conservatively includes the pair and returns BLOCKED. Validator V2's “potentially interacting” prefilter excludes different keys and returns PASS. Neither rule explicitly violates RE-I1–I3. The contract must require a sound candidate-set rule: a pair may be omitted only with recorded proof of disjointness; otherwise it is compared or yields BLOCKED/INDETERMINATE according to available evidence.

#### C-ADR-02 — Comparator registry ambiguity

An adapter package installs `selection-overlap-v2` and `geometric-overlap-v2`, both applicable to the same normalized selection resource. One returns `overlapping`; the other `indeterminate`. Provenance distinguishes results but no normative selection mapping chooses one. A conforming adapter contract must bind a comparison purpose and normalized resource kind to exactly one comparator implementation/version, rejecting zero or multiple unprioritized matches.

#### C-ADR-03 — Validation FAIL mixed with BLOCKED

One action has a malformed required descriptor (FAIL), while another awaits an external selection snapshot (BLOCKED). The lifecycle aggregation defines BLOCKED/INDETERMINATE precedence only among item evaluation states, not precedence over validation failure. The contract must define whether the plan result is a separate validation failure with blocked diagnostics, or another single canonical result.

#### C-ADR-04 — Composite target field association

A two-component Ansys workflow declares two actual releases and a capability list without normative component keys. Validator V1 associates capability X with Mechanical; V2 associates it with the orchestration component. One finds the required action supported, the other unsupported. Component-scoped requirements and resolved observations must be explicitly keyed.

### 3.3 Minimal contract revision required before ADR drafting

1. Define sound effect comparison candidate generation: omission requires provenance-backed proof of disjointness; otherwise compare.
2. Define comparator registry cardinality and deterministic resolution: exact comparison purpose, normalized kinds, adapter contract, version constraints, and tie/error behavior.
3. Bind effect equivalence, bookkeeping classification, idempotency comparison, exclusivity, and atomic failure semantics to versioned adapter contracts.
4. Add one plan-level validation-result aggregation rule covering validation FAIL together with BLOCKED/INDETERMINATE diagnostics.
5. Define deterministic Profile execution-policy composition and canonical loss identifiers.
6. Make release, formulation, capability, and runtime observation records component-keyed; define the orchestration adapter/component identity and “relevant component” derivation.

### 3.4 Architecture ownership and public shapes

The proposal does not require a Core expansion. `RealizationEffect`, `EffectComparison`, `PlanAction`, lifecycle records, and resolved target data remain runtime/Profile/Adapter structures. Vendor terms stay adapter-owned.

**MappingRule five-field verdict: maintainable at contract level.** No remaining counterexample requires a sixth top-level field. The unresolved data belong under `realization`, `capabilities`, or Profile target metadata.

**MappingClaim four-field verdict: maintainable at contract level.** No remaining counterexample requires a fifth field. Comparator/evaluation evidence is plan evidence; executable actions/effects derive from `realization`; target identity is external to the claim. This remains **requires executable validation** because hidden side-channel dependence has not yet been disproved.

## 4. Qualified Relation Cardinality v0.2 re-review

### 4.1 Prior Revise reasons

| Prior issue | v0.2 mechanism | Rating | Independent finding |
|---|---|---|---|
| QRC-01 open vs closed world | complete immutable relation snapshot required; incomplete→BLOCKED; schema reasoning separate | **Partially resolved** | Closed-world counting is explicit. In open-world/schema-only mode, however, the required result is merely “not evaluable” and “route to schema-level reasoning,” which is not one of the shared decision values and has no canonical report state. Independent validators can emit BLOCKED, unsupported operation, or no result. |
| QRC-02 multiple typing and subtype closure | resolved ontology/package version set; canonical closure; existential match across consistent types; inconsistency precheck | **Resolved** | Complete closure has no legitimate INDETERMINATE result; missing closure is BLOCKED and inability to compute it is tooling FAIL. A multiply typed identity counts once. |
| QRC-03 set vs multiset | unique canonical target identities after ADR-0009 alias resolution | **Resolved** | Duplicate serialized edges do not alter the count; semantic multiplicity requires reified relation instances. |
| QRC-04 subtype/min/max algebra | subset theorem and correct one-way min/max implications; broad refinement claim removed | **Resolved** | The proposal states the valid directions and explicitly rejects invalid reverse and exact implications. |
| QRC-05 distinct qualifier satisfiability | equivalent qualifiers intersect; distinct qualifiers stay separate; required subtype contradiction; general solver excluded | **Resolved** | The bounded reasoning scope is explicit and deterministic. |
| QRC-06 universal workaround | arbitrary `all` explicitly deferred | **Resolved** | No unsupported universal semantics are implied. |

### 4.2 New directly related ambiguity

#### C-QRC-01 — Mixed `exact`, `min`, and `max` authoring

The shape permits `min?`, `max?`, and `exact?` simultaneously. QRC-I8 says `exact=n` normalizes to `min=n,max=n`, but does not state whether explicit `min/max` are intersected with that interval or make the object malformed.

For the same input:

```yaml
min: 1
max: 3
exact: 2
```

one validator can normalize to `[2,2]`; another can reject mutually exclusive forms. For:

```yaml
min: 3
exact: 2
```

one can produce an empty-interval constraint conflict while another produces a syntax FAIL. The result classification and provenance differ.

### 4.3 Minimal contract revision required before ADR drafting

1. Assign one canonical outcome/report state to open-world or schema-only invocation without a closed instance snapshot. Prefer an explicit mode error/non-applicability result distinct from model-instance PASS/FAIL, or define it as malformed invocation FAIL; do not leave free text.
2. Define authoring exclusivity or intersection rules for `exact` combined with `min/max`, including the canonical defect/result code.
3. State whether unknown qualifier canonical identity is malformed FAIL or unresolved package/type evidence BLOCKED; use the same provenance-based boundary already adopted elsewhere.

### 4.4 Core scope

The optional `qualifier.target_type` is a justified narrow extension to existing Relation Cardinality. It uses SOL canonical identity and subtype semantics and introduces neither backend vocabulary nor a general query/predicate language. The six constraint families remain unchanged.

## 5. Contract completeness vs execution-validation readiness

### 5.1 Contract completeness gate

The current gate fails only on the normative ambiguities identified above. It does **not** fail because schemas or backend runs are absent.

A subsequent contract re-review may pass when:

- every counterexample in Sections 3.2 and 4.2 has exactly one normative result and defect code;
- the specified registry/component/policy resolution procedures reject ambiguous matches rather than select by iteration or declaration order;
- no fix adds backend-native vocabulary to Core or new top-level MappingRule/MappingClaim fields.

### 5.2 Minimum executable validation scope after contract PASS

| Implementation | Minimum scope | Pass criteria |
|---|---|---|
| Schemas | MappingRule 5-field and MappingClaim 4-field envelopes; RealizationEffect, EffectComparison, PlanAction, evaluation record, target requirement/resolution, QRC qualifier/interval | Valid fixtures accept; every malformed/missing/ambiguous case has one stable diagnostic code; no sixth/fifth public field or hidden required side channel. |
| Validator | Common PASS/FAIL/BLOCKED/INDETERMINATE boundaries; validation-error aggregation; QRC closed snapshot/count/type rules | Repeated and independent runs over canonicalized equivalent inputs yield identical decision and evidence set; declaration and serialization order do not change results. |
| MappingPlan generator | claim lowering, producer resolution, candidate effect pairs, evidence-backed DAG, idempotent coalescing | Same semantic input produces isomorphic canonical plans; unproved-disjoint effects are not pruned; no arbitrary ordering; cycles and duplicate producers deterministically fail. |
| Comparator registry | purpose/kind/adapter/version resolution; alias, overlap, payload/effect equivalence, idempotency and formulation comparators | Exactly one implementation resolves or validation FAILs; provenance records exact version and normalized inputs; missing external evidence versus missing implementation follows the contract boundary. |
| BackendTarget resolver | component-keyed requirement/runtime records, release matching, formulation binding, capability union, multi-component orchestration | Generic vendor/framework targets reject; unresolved observation BLOCKs; authoritative absence yields unsupported; ambiguous component or comparator match FAILs; equivalent component order gives the same result. |
| Thermal executable fixture | At minimum one fully generated MOOSE thermal path; dry-run adapter plus produced backend artifact; action/effect/DAG and target evidence | Exact/transformed outcome matches expected policy; missing transient prerequisites fail or become unsupported as specified; descriptor effects equal observed mutations; repeat planning/execution satisfies declared idempotency. |
| Plasma executable fixture | At minimum one COMSOL or Ansys plasma path exercising N:1 claims, selection aliases/collision, QRC negative-ion constraint, formulation and component binding | Stable source identities survive in four-field claims; alias collision is detected; duplicate edges count once; subtype/multiple typing cases match expected results; unsupported formulation/module and unresolved runtime evidence separate cleanly. |

Cross-backend validation should subsequently add the remaining Thermal MOOSE/COMSOL/Ansys projections and Plasma stress paths, but the table above is the minimum evidence needed to reopen the architecture-freeze decision.

## 6. Final disposition

| Question | Decision |
|---|---|
| ADR-0010 supplement v0.2 | **revision required** |
| Qualified Relation Cardinality v0.2 | **revision required** |
| MappingRule 5-field retention | **Yes at contract level; requires executable validation** |
| MappingClaim 4-field retention | **Yes at contract level; requires executable validation** |
| ADR drafting now | **No** |
| Research Lab action | Focused v0.2.x normative repair limited to Sections 3.3 and 4.3 |
| SOL v0.1 architecture freeze | **Not eligible; contract gate remains open and executable validation has not begun** |

The correct next step is a small contract revision, not architecture redesign. After those rules are fixed, Validation Lab should perform one final short contract check and then execute the minimum scope in Section 5.2.
