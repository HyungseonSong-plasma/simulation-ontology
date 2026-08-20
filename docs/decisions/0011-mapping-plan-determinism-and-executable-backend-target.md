# ADR-0011: MappingPlan Determinism and Executable Backend Target

**Status:** Proposed  
**Date:** 2026-08-20  
**Supplements:** ADR-0010 Profile and Backend Mapping Contract

## Context

ADR-0010 established Profile/BackendAdapter separation, an explicit MappingPlan, five-field `MappingRule`, four-field `MappingClaim`, realization-local dependencies, additive rule composition, and terminal representability outcomes `exact / transformed / lossy / unsupported`.

Independent AR-01/02/04 validation found that the accepted architecture did not fully determine cross-claim effect collision checking, executable action dependency semantics, evaluation lifecycle, or executable backend-target identity. Remediation proposals v0.2.1 and v0.2.2 were independently contract-reviewed before this ADR was drafted.

This ADR adds deterministic MappingPlan/Profile/Adapter contracts. It does not add backend-native object vocabulary to SOL Core and does not change the public `MappingRule` or `MappingClaim` shapes.

## Decision

### 1. Public mapping shapes remain unchanged

SOL v0.1 SHALL retain:

```text
MappingRule
├── source
├── applicability
├── realization
├── bindings
└── capabilities
```

and:

```text
MappingClaim
├── obligation
├── source
├── realization
└── provenance
```

Action, effect, comparator, runtime-target, lifecycle, and execution evidence belong to MappingPlan/Profile/Adapter contracts rather than additional top-level claim/rule fields.

### 2. Common validation decision domain

MappingPlan contract checks SHALL use:

```text
PASS
FAIL
BLOCKED
INDETERMINATE
```

with these boundaries:

- `FAIL`: malformed, contradictory, ambiguously bound input, or a required implementation/registry binding is absent or multiply matched where exactly one is required;
- `BLOCKED`: a valid contract requires external/runtime evidence that is not currently resolved;
- `INDETERMINATE`: complete required evidence and one selected procedure exist, but that procedure cannot decide the concrete case;
- `PASS`: the check is deterministically satisfied.

Missing implementation is `FAIL`; missing resolvable runtime evidence is `BLOCKED`.

Plan-level validation SHALL aggregate with strict precedence:

```text
FAIL > BLOCKED > INDETERMINATE > PASS
```

### 3. RealizationEffect and complete collision candidate set

Executable `realization` SHALL lower into normalized `RealizationEffect` evidence sufficient to identify the operation/effect kind, resource, semantic slot/path where applicable, scope where applicable, value/effect fingerprint where applicable, provenance, and resource-owning target component.

For one immutable MappingPlan revision, the validator SHALL form the unordered pair universe over all normalized runtime-mutating effects. Different source identities, obligations, resource spellings, aliases, declaration order, serialization order, or component identities SHALL NOT alone justify pruning.

A pair MAY be removed from compatibility/collision evaluation only when one uniquely selected, provenance-bearing comparison procedure proves the resources/effect scopes disjoint under the same immutable target snapshot.

No proof of disjointness means the pair remains subject to evaluation. Missing required external evidence yields `BLOCKED`; complete evidence with an undecidable selected procedure yields `INDETERMINATE`.

### 4. Effect component ownership

Every normalized effect SHALL resolve to exactly one `resource_component_id` identifying the target component owning the backend resource represented by the effect.

For cross-component transfer actions, source-side and destination-side effects retain their respective resource component identities even when the enclosing action is executed by an orchestration component.

- structurally missing ownership => `FAIL`;
- unresolved valid runtime component evidence => `BLOCKED`;
- ambiguous ownership => `FAIL`.

### 5. Comparator binding is unique and context-dependent

Every required alias, overlap, effect-equivalence, idempotency, formulation, or other MappingPlan comparison SHALL resolve to exactly one versioned comparator through the selected adapter contracts.

Comparator resolution SHALL distinguish:

```text
local(component_binding)
```

from:

```text
cross_component(
    left_component_binding,
    right_component_binding,
    orchestration_component_binding
)
```

where a component binding includes stable component identity and adapter contract identity/version.

Local comparisons are bound by the local component adapter contract. Cross-component comparisons are bound by the selected versioned orchestration adapter contract and include both component bindings. Symmetric purposes SHALL use canonical operand ordering; directional purposes SHALL preserve semantic source/target direction.

Registry resolution SHALL produce exactly one matching implementation:

```text
0 matches  => FAIL
1 match    => use it
>1 matches => FAIL
```

Iteration order, newest-version choice, declaration order, or validator-specific preference SHALL NOT resolve ambiguity.

### 6. Executable PlanAction contract

MappingPlan execution dependencies SHALL be expressed at executable `PlanAction` level rather than as backend-object nodes.

Each `PlanAction` SHALL have or resolve to:

```text
PlanAction
├── stable action identity
├── executable descriptor
├── requires[]
├── produces[]
├── normalized effects[]
├── component binding
└── idempotency contract/evidence
```

The selected adapter contract SHALL expose a deterministic `describe_effects(executable_descriptor, target_state_snapshot)` procedure. The descriptor executed MUST be the descriptor whose effects were validated.

Every derived semantic effect MUST correspond to a declared semantic effect, and every declared semantic effect MUST correspond to a derived effect, using the uniquely bound effect-equivalence comparator. Unmatched effects MAY be excluded from semantic equivalence only when the selected versioned adapter contract classifies the exact effect kind/path as non-semantic bookkeeping. Bookkeeping effects that mutate runtime state remain visible to ordering/collision validation.

### 7. Dependency, producer, ordering, idempotency, and atomicity invariants

Every required handle SHALL resolve to an external binding, exactly one accepted producer, or multiple producers proven equivalent and deterministically coalesced.

- valid external binding unresolved => `BLOCKED`;
- no producer/source in an otherwise complete plan => `FAIL`;
- exclusive duplicate producer without equivalence => `FAIL`.

Ordering direction SHALL come only from prerequisite/producer relations or provenance-bearing versioned `must_precede` evidence. Noncommutativity alone SHALL NOT invent A→B or B→A. The final action dependency graph MUST be acyclic.

`state_independent_idempotent` SHALL require a versioned adapter guarantee. State-dependent idempotency SHALL use exactly one bound procedure and immutable pre-state evidence. Bare author/Profile assertions do not establish idempotency.

A compound action MAY be treated as atomic only when the selected adapter contract guarantees semantic all-or-none visibility: all declared semantic effects are committed on success and none are externally visible on failure. Otherwise the action must be decomposed; unavailable required decomposition is `FAIL`.

### 8. Evaluation lifecycle is separate from representability

The evaluation lifecycle SHALL be:

```text
pending
blocked
indeterminate
complete
```

and SHALL remain separate from terminal representability:

```text
exact
transformed
lossy
unsupported
```

and from operational execution permission:

```text
permitted
prohibited
```

A terminal representability outcome SHALL be asserted only when plan validation is `PASS` and evaluation is `complete`.

Representability SHALL first be evaluated per required semantic mapping obligation, then aggregated:

```text
any unsupported  => unsupported
else any lossy   => lossy
else any transformed => transformed
else exact
```

Optional non-semantic reporting omissions SHALL NOT weaken semantic representability.

### 9. Loss policy and execution permission

Every identified semantic loss SHALL have a canonical order-independent loss key derived from source model-instance identity, obligation, and lost semantic aspect identity/path.

Applicable Profile execution policies SHALL compose conjunctively per loss key. Any deny denies; absence of explicit allowance denies by default. Declaration order SHALL NOT affect permission.

- `exact` / `transformed`: permitted when no independent validated execution blocker exists;
- `lossy`: permitted only when every canonical identified loss is allowed;
- `unsupported`: prohibited;
- evaluation not complete: prohibited.

### 10. BackendTarget requirement and resolved runtime are distinct

A Profile SHALL declare `BackendTargetRequirement`; adapter discovery SHALL produce `ResolvedBackendTarget`.

Every target component SHALL have a stable Profile-local `component_id`. Release requirements, actual releases, capability requirements/observations, formulation bindings, adapter contracts, and PlanAction/effect bindings SHALL be component-keyed.

A vendor/framework umbrella name alone does not satisfy executable target completeness unless the selected adapter contract resolves it to required concrete components.

For a composite target:

- exactly one orchestration component/adapter SHALL be identified;
- every PlanAction SHALL bind to one execution component;
- explicit transfer descriptors SHALL identify source, destination, and orchestration components;
- unscoped release/capability/formulation records are invalid.

### 11. Release, capability, and formulation resolution

For each component, observed release SHALL satisfy the declared release compatibility contract. Missing runtime release evidence is `BLOCKED`; malformed/ambiguous matching or a resolved target mismatch is `FAIL` before execution.

Capabilities remain adapter-owned canonical identities. Per-component effective requirements are the normalized conjunction/union of target-floor, selected-rule, formulation, and applicable orchestration requirements. Conflicting requirement intersections are `FAIL`; unresolved runtime evidence is `BLOCKED`; authoritative absence produces terminal `unsupported` after successful validation/evaluation completion.

SOL mathematical/analysis semantics remain authoritative. Backend formulation identifiers are realization bindings only. Relevant target components are derived from the selected MappingRule → PlanAction graph, not vendor heuristics. For each relevant formulation obligation/component pair, exactly one compatible binding SHALL resolve.

### 12. Backend semantics remain outside Core

`PlanAction`, `RealizationEffect`, comparator registries, executable descriptors, backend resource keys, target products/releases/modules, and orchestration mechanisms are MappingPlan/Profile/Adapter contracts. They SHALL NOT become SOL Core ontology entities merely because validation requires them.

## Consequences

### Positive

- Cross-claim collisions can be detected even when source/obligation identities differ.
- Plan construction and execution ordering are deterministic and inspectable.
- Missing evidence, invalid contracts, and true undecidability have distinct outcomes.
- Composite backend targets cannot silently bind release/capability/formulation information to arbitrary components.
- MappingRule and MappingClaim remain minimal.

### Costs

- Adapters must provide normalized effect descriptors, versioned comparator/normalizer/idempotency registries, component-keyed target resolution, and explicit orchestration contracts.
- MappingPlan validation is intentionally conservative when disjointness cannot be proven.
- Executable evidence is required before SOL v0.1 architecture freeze.

## Deferred

This ADR does not define:

- universal backend operation vocabulary;
- backend-native resource schemas;
- one universal comparator implementation;
- concrete serialization syntax for MappingPlan;
- distributed transaction protocols across products;
- automatic winner-selection among incompatible mappings.

## Validation requirement

Acceptance of this contract does not authorize SOL v0.1 architecture freeze. Freeze requires executable evidence for machine-readable schemas, MappingPlan generation, comparator resolution, BackendTarget resolution, and Thermal/Plasma fixtures showing deterministic decisions and no hidden MappingRule/MappingClaim side channel.

## Decision summary

SOL v0.1 supplements ADR-0010 with deterministic effect-pair validation, unique local/cross-component comparator binding, executable PlanAction dependency semantics, explicit evaluation lifecycle, component-keyed BackendTarget resolution, and strict separation between contract validation, terminal representability, and execution permission while preserving five-field `MappingRule` and four-field `MappingClaim` public shapes.
