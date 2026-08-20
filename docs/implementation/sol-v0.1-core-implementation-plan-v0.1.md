# SOL v0.1 Core Implementation Plan v0.1

**Status:** Implementation planning baseline  
**Date:** 2026-08-20  
**Design baseline:** ADR-0001..ADR-0030  
**Design state:** `SOL v0.1 DESIGN-STAGE CLOSED`  
**Implementation state:** planning / pre-bootstrap

## 1. Purpose

This document starts the post-design implementation stage for SOL v0.1 without reopening the accepted semantic architecture.

The implementation objective is to build a language-independent semantic Core in Rust, a TypeScript/React primary client and GUI layer, and backend adapters in each backend's native automation ecosystem.

The first implementation milestone is intentionally narrower than full product integration:

1. Rust Core builds and passes CI;
2. accepted architecture counterexamples execute as automated Rust contract tests;
3. `sol-cli` reads the Thermal reference fixture and emits deterministic canonical validation and MappingPlan results.

Real MOOSE/COMSOL/Ansys adapters and TypeScript Node/native bindings are prohibited before this milestone is stable.

## 2. Fixed implementation architecture

```text
TypeScript / React GUI + primary client
                |
                | future JSON-RPC / IPC protocol
                v
          Rust SOL Core
    semantic model + validation
    resolver + mapping planner
                |
                | serializable protocol DTOs
                v
          Adapter boundary
       /          |          \
      v           v           v
 MOOSE Adapter  COMSOL Adapter  Ansys Adapter
 Python/C++       Java           Python
```

Initial development substitutes a deterministic `MockAdapter` for all real backend adapters.

### 2.1 Rust Core

Rust owns backend-independent deterministic semantics:

- normalized SOL data model and serialization;
- canonical identity and package/reference resolution;
- Constraint and QRC evaluation;
- MappingRule / MappingClaim composition;
- RealizationEffect normalization and comparison orchestration;
- PlanAction / MappingPlan DAG construction and validation;
- PASS / FAIL / BLOCKED / INDETERMINATE lifecycle semantics;
- BackendTarget requirement/resolution contracts;
- deterministic diagnostics and canonical output.

Rust MUST NOT contain backend-native object models, COMSOL model-tree classes, MOOSE object types, Ansys Workbench objects, or vendor-specific capability vocabularies in the semantic Core.

### 2.2 TypeScript / React

TypeScript is the primary product/client layer and will eventually own:

- GUI state and interaction;
- editor/view-model types generated or mirrored from stable protocol contracts;
- protocol client;
- visualization of validation diagnostics, semantic graph, MappingPlan, and execution state.

TypeScript MUST NOT become an independent source of semantic truth. Canonical validation and plan construction remain Rust responsibilities.

TypeScript binding work is deferred until the CLI milestone is stable.

### 2.3 Backend adapters

Adapters use native ecosystems:

- MOOSE: Python/C++ integration surface;
- COMSOL: Java API;
- Ansys: Python/PyAnsys ecosystem.

The native-language choice is an Adapter concern and does not alter SOL Core semantics.

Official ecosystem sanity references:

- MOOSE application development: https://mooseframework.inl.gov/application_development/
- COMSOL API / Java: https://www.comsol.com/support/learning-center/article/overview-of-the-comsol-api-107912
- Ansys PyAnsys: https://docs.pyansys.com/

### 2.4 Protocol boundary

Core and adapters SHALL NOT be strongly coupled through direct language FFI.

The target integration boundary is process/protocol based:

```text
Rust Core <-> JSON-RPC / IPC <-> Adapter process
```

During bootstrap, MockAdapter MAY execute in-process for simplicity, but only through serializable protocol-shaped request/response DTOs. Backend-native runtime handles MUST NOT cross into Core data structures.

This allows the initial implementation to remain simple without turning the temporary in-process test path into a permanent FFI architecture.

## 3. Repository layout strategy

Rust introduction cost is minimized by starting with a small workspace rather than one crate per semantic concern.

Proposed initial layout:

```text
rust/
  Cargo.toml
  crates/
    sol-core/
      src/
        lib.rs
        diagnostic.rs
        model/
        identity/
        resolver/
        constraint/
        mapping/
        plan/
        target/
        lifecycle/
    sol-mock-adapter/
      src/
    sol-cli/
      src/

protocol/
  README.md
  # JSON-RPC/IPC schemas added only after DTOs stabilize

examples/
  # existing accepted Thermal / Plasma reference fixtures remain authoritative

tests/
  # existing Validation Lab artifacts remain evidence and migration source
```

Do not split `sol-core` into many crates until module boundaries are stable and there is a demonstrated compilation/dependency reason to do so.

## 4. Rust implementation principles

### 4.1 Determinism first

Canonical output must not depend on declaration order, `HashMap` iteration order, filesystem ordering, or test execution ordering.

Preferred implementation defaults:

- explicit newtypes for canonical identity;
- `BTreeMap` / `BTreeSet` or explicit sorting for canonical collections;
- immutable input snapshots during evaluation;
- explicit stable diagnostic codes;
- deterministic topological ordering when more than one DAG ordering is valid, using stable action identity as tie-breaker;
- canonical JSON output for CLI golden tests.

### 4.2 Schemas are input contracts, not semantic authority

Existing accepted JSON Schemas remain structural contracts. Rust semantic validation implements the accepted ADR semantics where JSON Schema is insufficient.

The Rust implementation MUST NOT silently redefine accepted schema meaning.

### 4.3 Errors are typed outcomes

Implementation should distinguish:

```text
structural/contract error
semantic validation result
runtime evidence state
representability result
execution permission
```

Do not collapse them into one `Result<bool, String>` style API.

### 4.4 No backend leakage

The following belong outside semantic Core truth:

- backend installation state;
- license state;
- native object handles;
- MOOSE/COMSOL/Ansys release-specific operation names;
- backend resource identifiers except inside normalized Adapter/MappingPlan DTOs;
- vendor-specific comparator implementations.

## 5. CI-first development

Rust is introduced through GitHub Actions before substantive implementation grows.

The first CI workflow SHALL run on pull requests and branch pushes affecting Rust or architecture fixtures.

Minimum required jobs/steps:

```text
cargo build --workspace --all-targets
cargo test --workspace --all-targets
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
architecture/reference contract tests
```

Initial CI SHOULD use stable Rust and avoid optional tooling that increases bootstrap complexity.

Do not require `cargo-nextest`, code coverage services, release packaging, cross-compilation, or backend software installation in Milestone 0.

### 5.1 CI gate policy

A change cannot advance the implementation baseline if any of the following fails:

- build;
- fmt;
- clippy;
- unit tests;
- architecture counterexample tests;
- golden CLI output tests once introduced.

Backend availability MUST NOT be a CI prerequisite during the Mock Adapter stage.

## 6. Validation Lab artifact migration

Accepted Validation Lab positive and negative cases become executable implementation assets.

Migration classes:

| Existing evidence | Rust test form |
|---|---|
| accepted normalized schema fixtures | serialization/deserialization contract tests |
| identity/package counterexamples | resolver unit + contract tests |
| Constraint family counterexamples | table-driven semantic tests |
| QRC closed-snapshot cases | golden + property tests |
| Interface mapping counterexamples | resolver/composition contract tests |
| MappingPlan counterexamples | DAG/effect/comparator tests |
| Thermal reference model | first vertical-slice integration test |
| Plasma/QRC reference model | second vertical-slice regression test |

Every migrated negative fixture should assert a stable diagnostic code, not only that "an error occurred".

### 6.1 Golden tests

Golden tests are used for deterministic serialized outputs such as:

- canonical normalized model;
- validation report;
- MappingClaim set;
- MappingPlan;
- lifecycle/evaluation result;
- CLI JSON output.

Golden output changes require an explicit contract review. They are not automatically updated because implementation output changed.

### 6.2 Property tests

Property testing is introduced after deterministic unit behavior exists.

High-value initial invariants:

- declaration permutation does not change normalized result;
- repeated normalization is idempotent;
- resolver output is order-independent;
- QRC counts distinct target identity rather than duplicate edges;
- compatible MappingClaims compose independent of input order;
- incompatible same-source/same-obligation claims always conflict;
- MappingPlan topological output is deterministic for equivalent input DAGs;
- lifecycle aggregation respects accepted precedence.

Property-test tooling is not a Milestone 0 prerequisite.

## 7. Ordered Core implementation sequence

The implementation order is fixed unless implementation evidence exposes a dependency error.

### Stage 0 — Bootstrap and CI

Deliverables:

- Rust workspace;
- `sol-core` empty/minimal library;
- `sol-cli` bootstrap binary;
- `sol-mock-adapter` bootstrap library;
- GitHub Actions build/test/fmt/clippy workflow;
- fixture-loading test harness.

Exit gate:

```text
CI green on empty/minimal workspace
+ accepted JSON fixtures can be loaded by tests
```

### Stage 1 — Data model and serialization

Implement normalized DTOs required for the first vertical slice:

- ontology package/resource definitions;
- ResolvedModelSnapshot;
- Value / ExactDecimal / UnitReference / DimensionVector;
- Interface definitions/implementations;
- Constraint payload envelope;
- mapping contract DTO skeletons.

Primary dependencies should remain minimal. `serde` / `serde_json` are expected foundation choices.

Exit gate:

- Thermal/core reference package and Thermal snapshot deserialize;
- reserialization is deterministic;
- negative structural fixtures produce typed diagnostics.

### Stage 2 — Canonical identity and resolver

Implement:

- canonical ID newtype;
- package/version identity;
- namespace export resolution;
- exact dependency environment resolution;
- kind-safe reference resolution;
- subtype closure;
- ambiguity/missing-reference diagnostics.

Exit gate:

- ADR-0009/0028 identity/package counterexamples migrated;
- order-invariance tests pass;
- Thermal reference environment resolves canonically.

### Stage 3 — Constraint and QRC engine

Implement accepted families in dependency order:

1. Cardinality / QRC;
2. Type;
3. Dimension;
4. Value;
5. Compatibility;
6. Predicate / Conditional;
7. cross-family result aggregation.

The engine must preserve accepted distinctions among `PASS / FAIL / BLOCKED / INDETERMINATE` where applicable.

Exit gate:

- migrated architecture counterexamples pass;
- Thermal Dimension/Value cases pass;
- Plasma QRC regression fixture can be evaluated even though Plasma is not yet the primary vertical slice.

### Stage 4 — MappingRule and MappingClaim

Implement the accepted public shapes exactly:

```text
MappingRule = source + applicability + realization + bindings + capabilities
MappingClaim = obligation + source + realization + provenance
```

No implementation convenience field may alter those public semantic contracts.

Implement:

- applicability evaluation;
- additive rule collection;
- claim normalization;
- same-source/same-obligation conflict detection;
- deterministic provenance preservation.

Exit gate:

- claim composition is declaration-order independent;
- conflict counterexamples are stable.

### Stage 5 — RealizationEffect and comparator contracts

Implement normalized RealizationEffect and comparator-binding model from ADR-0011.

Core responsibilities:

- normalized effect identity/data;
- complete candidate-pair generation;
- resource component ownership validation;
- exact comparator lookup context;
- comparison outcome handling.

MockAdapter responsibilities:

- deterministic mock comparator registry;
- deterministic `describe_effects` behavior;
- synthetic capability/evidence responses.

Exit gate:

- effect collision/disjointness counterexamples pass;
- missing comparator = FAIL;
- missing runtime evidence = BLOCKED;
- complete-but-undecidable mock comparator = INDETERMINATE.

### Stage 6 — PlanAction / MappingPlan DAG

Implement:

- stable PlanAction ID;
- executable descriptor DTO;
- `requires[]` / `produces[]`;
- effect list;
- component binding;
- producer resolution;
- dependency DAG;
- cycle detection;
- deterministic topological order;
- idempotency/atomicity contract hooks.

Exit gate:

- unresolved producer and cycle counterexamples fail deterministically;
- equivalent input order produces identical canonical plan JSON.

### Stage 7 — Evaluation lifecycle

Implement orthogonal axes:

```text
ValidationDecision = PASS | FAIL | BLOCKED | INDETERMINATE
EvaluationLifecycle = pending | blocked | indeterminate | complete
Representability = exact | transformed | lossy | unsupported
ExecutionPermission = permitted | prohibited
```

Preserve ADR-0011 precedence and revision immutability.

Exit gate:

- invalid transitions rejected;
- representability only exists after PASS + complete;
- loss-policy fixtures determine execution permission deterministically.

### Stage 8 — Mock Adapter

MockAdapter becomes the only executable adapter for the first milestone.

It supplies deterministic fixtures for:

- backend target discovery;
- release/capability evidence;
- comparator lookup;
- effect description;
- idempotency evidence;
- execution response without real backend software.

Mock behavior must be explicit fixture data, not hidden hard-coded special cases for Thermal IDs.

### Stage 9 — BackendTarget resolver

Implement component-keyed:

- BackendTargetRequirement;
- ResolvedBackendTarget;
- release matching;
- capability requirement/observation;
- formulation binding;
- orchestration component rules.

Initial Thermal target is a synthetic `mock-thermal` backend target.

Exit gate:

- exact/transformed/lossy/unsupported synthetic scenarios are reproducible without backend installation.

### Stage 10 — `sol-cli`

Initial commands should remain small:

```text
sol-cli validate <model.json> --package <package.json>...
sol-cli plan <model.json> --profile <profile.json> --target <mock-target.json>
```

Machine-readable JSON output is primary; concise human-readable output may be added as a view.

CLI must expose canonical diagnostics, not reconstruct semantic decisions independently.

Exit gate for initial milestone:

1. Rust CI green;
2. architecture counterexample suite runs in CI;
3. Thermal reference validation returns PASS;
4. `sol-cli plan` emits deterministic canonical MappingPlan JSON through MockAdapter;
5. negative Thermal fixtures emit expected stable diagnostics.

### Stage 11 — TypeScript protocol/client binding — DEFERRED GATE

Start only after Stage 10 exit gate is stable.

Preferred direction:

- freeze protocol DTO JSON shape;
- define JSON-RPC/IPC request/response contract;
- generate or maintain TypeScript protocol types;
- implement TS client;
- integrate React GUI.

Do not start with Node native FFI/N-API binding. The architectural target is protocol separation, not in-process language coupling.

## 8. First vertical slice — Thermal

The accepted Thermal reference model is the first end-to-end slice because it exercises multiple semantic layers while remaining small.

Target flow:

```text
Thermal normalized packages
        +
Thermal ResolvedModelSnapshot
        |
        v
Rust deserialize
        |
        v
canonical resolver
        |
        v
Constraint / Interface / Dimension validation
        |
        v
Thermal MappingRules/Profile fixture
        |
        v
MappingClaims
        |
        v
RealizationEffects
        |
        v
MappingPlan DAG
        |
        v
MockAdapter target evaluation
        |
        v
PASS / complete / exact-or-transformed
        |
        v
sol-cli canonical JSON output
```

The first slice does not execute a physical thermal solve. Its purpose is to validate semantic compilation/planning against the frozen SOL architecture.

## 9. Thermal implementation fixtures to add

Existing reference package/model files remain input evidence. Implementation planning adds only new Profile/MockAdapter expectation fixtures.

Suggested future fixture set:

```text
examples/implementation/thermal/
  profile.json
  mock-target.json
  expected-validation.json
  expected-mapping-plan.json
  negative/
    missing-temperature-target.json
    wrong-conductivity-dimension.json
    unresolved-unit.json
    mapping-claim-conflict.json
    missing-capability.json
    mapping-cycle.json
```

Do not modify accepted design reference fixtures merely to make Rust implementation easier. If an implementation requires a semantic fixture change, classify it as either implementation defect, fixture defect, or potential ADR-0030 reopen candidate before changing the design baseline.

## 10. Design-baseline protection and reopen rule

Implementation inconvenience is not an architecture defect.

When Rust implementation conflicts with the design baseline, classify the issue first:

```text
implementation defect
serialization/tooling defect
fixture defect
Adapter/backend limitation
operational/environment issue
potential semantic architecture counterexample
```

Only the last category can trigger the ADR-0030 reopen procedure, and only after independent Validation confirms the counterexample.

## 11. Real backend adapter gate

Real backend adapter implementation is prohibited until all of these are true:

- Stage 10 CLI milestone is stable;
- Thermal golden contract tests are stable;
- MappingPlan output schema is stable enough for protocol exposure;
- MockAdapter demonstrates PASS/FAIL/BLOCKED/INDETERMINATE paths;
- no unresolved architecture counterexample exists.

Then adapters may begin independently:

```text
MOOSE Adapter   -> Python/C++ native ecosystem
COMSOL Adapter  -> Java API
Ansys Adapter   -> Python/PyAnsys
```

Adapter projects should consume serialized SOL MappingPlan/protocol DTOs rather than linking their native object model into Rust Core.

## 12. TypeScript/React gate

React/UI work may continue conceptually, but semantic integration with the new Core should not begin until the CLI/protocol shapes stabilize.

The primary integration sequence is:

```text
Rust CLI proves contract
        -> protocol DTO freeze candidate
        -> JSON-RPC/IPC protocol
        -> TypeScript client
        -> React integration
```

This prevents the GUI from becoming a second semantic engine during Core bootstrap.

## 13. Initial milestone definition

### Milestone M1 — Rust Semantic Core Bootstrap

**Must have**

- CI build/test/fmt/clippy;
- deterministic serialization;
- canonical resolver;
- architecture counterexample migration started and running in CI;
- complete Constraint/QRC path needed by Thermal;
- MappingRule/MappingClaim;
- MappingPlan pipeline through MockAdapter;
- BackendTarget mock resolution;
- `sol-cli validate` and `sol-cli plan`;
- deterministic Thermal golden outputs.

**Must not have**

- real MOOSE installation dependency;
- real COMSOL installation/license dependency;
- real Ansys installation/license dependency;
- production backend adapter;
- Node native FFI binding;
- GUI-owned semantic validation;
- semantic changes to the ADR-0030 baseline without reopen procedure.

## 14. Immediate next actions

The implementation stage should now proceed in this exact order:

```text
1. Create Rust workspace skeleton
2. Add GitHub Actions Rust CI
3. Add fixture loader for accepted package/snapshot JSON
4. Implement Stage 1 data model/serialization
5. Convert the first identity/resolver counterexamples
6. Continue through the ordered stages in this document
```

The next Operating Desk state is therefore:

```text
Current State: CORE_IMPLEMENTATION_PLANNING_ACCEPTED
Next Role: Implementation
Next Atomic Unit: Rust workspace + CI bootstrap
Blocking external backend requirement: none
Design reopen: no
```
