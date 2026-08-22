# simulation-ontology

SOL is a solver-independent semantic platform for describing simulation intent, validating it, planning how it may be realized, and connecting the same canonical model to multiple simulation backends through versioned adapter contracts.

SOL does not replace MOOSE, COMSOL, Ansys, or other solvers. It provides a semantic layer above them so that simulation meaning is not defined by a single solver's object model, file format, or API.

## Why SOL exists

Simulation workflows usually mix physical intent, mathematical representation, constitutive assumptions, spatial applicability, numerical configuration, and backend realization details. SOL separates those concerns into canonical semantic concepts and keeps solver-native realization outside semantic truth.

```text
Canonical simulation intent
          |
          v
Validation / constraints
          |
          v
Mapping claims + MappingPlan
          |
          v
BackendTarget / Adapter Protocol
          |
     +----+----+
     |         |
     v         v
   MOOSE     other adapters
```

The platform is intended to enable solver-independent model exchange, semantic validation before backend selection, deterministic mapping plans, first-class spatial applicability, explicit realization quality, and adapters that can evolve independently from Core.

## Current status

The first six foundation milestones have completed their implementation and acceptance gates. M0.7 is the active Core milestone:

```text
M0.1  Semantic Core Bootstrap                  COMPLETE
M0.2  Canonical Public Contract 0.1            COMPLETE
M0.3  Adapter Protocol 0.1                     COMPLETE
M0.4  MockAdapter Protocol Conformance         COMPLETE
M0.5  JSON-RPC / stdio Transport               COMPLETE
M0.6  Adapter Conformance Tooling              COMPLETE
M0.7  Adapter Runtime & Registry               ACTIVE — 2/6 phases accepted complete
```

M0.3 closed after exact-head CI, a Validator exit audit with verdict **APPROVE**, and PR #63 publication merge. Adapter Protocol 0.1 is now a published, versioned, transport-independent v0.x interoperability baseline.

M0.4 closed after Phase 0–5 implementation, exact-head Rust Core CI #542, a Validator exit verdict of **APPROVE**, PR #93 merge, `main` verification, parent/Phase closure, and GitHub Milestone #1 closure. MockAdapter now provides executable in-process reference behavior for Adapter Protocol 0.1 description, advisory preflight, authoritative execution, failure/state/replay boundaries, provenance, and dependency-safe scheduling.

M0.5 closed after Phase 0–5 implementation, exact-head Rust Core CI #617, a Validator exit verdict of **APPROVE**, PR #101 merge, `main` verification, parent/Phase closure, and user-confirmed GitHub Milestone #5 closure. The completed local JSON-RPC/stdio boundary now carries the unchanged Protocol 0.1 describe, validate, execute, and ProtocolFailure semantics through a typed subprocess session with deterministic error, reconnect, response-loss, and no-replay behavior.

M0.6 completed Phase 0–5 implementation, exact-head Rust Core CI #730, a Validator exit verdict of **APPROVE**, PR #108 merge as `a014216b957b3a2128fd1f516f2490f38caf8fac`, `main` verification, Phase #82–#87 closure, and parent #81 closure. Reusable conformance tooling now exercises external adapter commands through the published process transport, executes positive and adversarial Protocol/Public Contract fixtures, distinguishes adapter non-conformance from harness/transport failure, provides a solver-independent authoring skeleton, and proves a standalone external-project-style workflow with deterministic provisional machine-readable CI evidence.

GitHub Milestone #6 contains `0` open and `6` closed canonical Phase issues. Its final UI **Close milestone** action remains administrative because the GitHub connector available to Operator does not expose milestone-state writes; this does not alter the accepted Validator, CI, Phase, parent, or `main` evidence.

M0.7 is tracked by parent #111 and GitHub Milestone #7 `M0.7 — Adapter Runtime & Registry`. Canonical Phase issues #112–#117 are assigned to the milestone while parent #111 remains outside it by convention. Phase 0 (#112) and Phase 1 (#113) are accepted complete; Phase 2 (#114) is the current implementation Phase. The accepted `main` head after Phase 1 is `dfbc78d0fd7dba3792c8d805a815eccf85d1e18e`. Work on Phase 2 PR #122 is not accepted repository state until exact-head CI, merge, and post-merge `main` verification complete.

The repository currently contains:

- solver-independent Core entity/relation semantics and canonical identity;
- first-class `SpatialScope` distinct from `SpatialModel` and backend-native selections;
- QRC/type/relation/cardinality/reference constraints;
- `MappingRule`, `MappingClaim`, `RealizationEffect`, and lifecycle semantics;
- deterministic DAG-based `MappingPlan` with cycle rejection;
- independently versioned Public Contract 0.1 DTOs, schemas, fixtures, and compatibility harness;
- an intentional Rust public facade plus explicit `sol-cli --json` machine-facing paths;
- independently versioned Adapter Protocol 0.1 bootstrap, compatibility, preflight, execution, failure, idempotency, and provenance semantics;
- canonical Adapter Protocol 0.1 Draft 2020-12 schemas and executable counterexamples;
- MockAdapter executable in-process Adapter Protocol 0.1 reference behavior and positive/adversarial conformance matrix;
- deterministic local JSON-RPC/stdio method mapping, framing, request correlation, process session, and typed error/recovery boundaries;
- canonical in-process/subprocess parity across all MockAdapter reference states and key transport counterexamples;
- reusable conformance result/harness libraries for external adapter commands;
- versioned positive and 23-case adversarial conformance fixture execution;
- a solver-independent Rust/stdin-stdout adapter-authoring skeleton and guide;
- a standalone external-project conformance consumer that proves fixture reuse, semantic counterexample detection, response-loss separation, and deterministic provisional CI evidence;
- a Core-local `sol-adapter-runtime` boundary that keeps `BackendTarget`, `AdapterRegistration`, and `AdapterInstance` structurally and semantically distinct; and
- deterministic explicit local adapter registration with register/get/list/remove/enable-disable behavior and no static compatibility/capability authority.

## Public Contract 0.1

The canonical external semantic boundary is versioned independently from the runtime and is represented by language-neutral JSON, schemas, fixtures, compatibility rules, and executable Rust DTO behavior.

Primary contract documents:

- [`docs/contracts/public-contract-0.1-json-rules.md`](docs/contracts/public-contract-0.1-json-rules.md)
- [`docs/contracts/public-contract-0.1-model-validation-dtos.md`](docs/contracts/public-contract-0.1-model-validation-dtos.md)
- [`docs/contracts/public-contract-0.1-realization-dtos.md`](docs/contracts/public-contract-0.1-realization-dtos.md)
- [`docs/contracts/public-contract-0.1-json-schema.md`](docs/contracts/public-contract-0.1-json-schema.md)
- [`docs/contracts/public-contract-0.1-compatibility.md`](docs/contracts/public-contract-0.1-compatibility.md)
- [`docs/contracts/public-contract-0.1-facade-cli.md`](docs/contracts/public-contract-0.1-facade-cli.md)

Machine-facing CLI examples:

```text
sol-cli validate fixtures/public-contract/0.1/thermal-simulation.json --json
sol-cli plan fixtures/public-contract/0.1/thermal-simulation.json --target mock --json
```

Human-readable CLI output remains distinct from the machine-facing Public Contract representation.

## Adapter Protocol 0.1

Adapter Protocol 0.1 is the published language-neutral, transport-independent contract between SOL Core and adapters.

Normative entry point:

- [`docs/contracts/adapter-protocol-0.1.md`](docs/contracts/adapter-protocol-0.1.md)

Supporting contracts:

- [`docs/adr/ADR-002-adapter-protocol-boundary.md`](docs/adr/ADR-002-adapter-protocol-boundary.md)
- [`docs/contracts/adapter-protocol-0.1-bootstrap-description.md`](docs/contracts/adapter-protocol-0.1-bootstrap-description.md)
- [`docs/contracts/adapter-protocol-0.1-plan-preflight.md`](docs/contracts/adapter-protocol-0.1-plan-preflight.md)
- [`docs/contracts/adapter-protocol-0.1-execution-realization.md`](docs/contracts/adapter-protocol-0.1-execution-realization.md)
- [`docs/contracts/adapter-protocol-0.1-error-idempotency-state.md`](docs/contracts/adapter-protocol-0.1-error-idempotency-state.md)
- [`schemas/adapter-protocol/0.1/`](schemas/adapter-protocol/0.1/)

Published logical operation surface:

```text
describe_adapter -> AdapterDescription | ProtocolFailure
validate_plan    -> ValidatePlanResponse | ProtocolFailure
execute_plan     -> ExecutePlanResponse | ProtocolFailure
```

These are logical protocol responsibilities rather than transport-derived semantics. M0.5 maps them onto constrained local JSON-RPC/stdio while keeping request IDs, framing, process lifecycle, transport errors, reconnect, and replay policy outside Protocol payload meaning.

### Compatibility

Adapter interoperability requires both:

```text
Adapter Protocol compatibility
AND
Public Contract compatibility
```

Adapter package version alone never proves compatibility. Missing compatibility evidence means interoperability is not established.

### Validation and execution

`validate_plan` is advisory and side-effect free. `execute_plan` is authoritative and re-checks the complete current request/state before side effects. `execute_plan` is non-idempotent by default; response loss therefore does not imply safe replay.

Core owns MappingPlan DAG/dependency semantics and canonical deterministic representation. Adapters own backend scheduling and may reorder or parallelize independent actions while preserving dependencies.

### Lifecycle separation

Protocol/bootstrap/request/operational failures, preflight state, and execution state are not SOL lifecycle states. They do not directly become `PASS`, `FAIL`, `BLOCKED`, or `INDETERMINATE`. Only suitable canonical semantic evidence may later participate in Core lifecycle evaluation.

### Backend-data boundary

Solver-native object models and vendor API structures cannot become canonical semantic identity. Opaque namespaced job/artifact references are allowed only as provenance/evidence when semantic meaning and equality do not depend on them.

## JSON-RPC / stdio transport

M0.5 provides the default local v0.x transport as a child-process session using constrained JSON-RPC 2.0 over standard input/output. A fresh session must bootstrap with `describe_adapter` before typed validation or execution requests are accepted.

JSON-RPC IDs are correlation only. ProtocolFailure remains a normal logical Protocol result, while framing, malformed envelope, timeout, EOF, broken pipe, abnormal exit, stderr, correlation, and payload errors remain typed transport/process evidence.

Reconnect creates a new bootstrapped process session. Lost description may be reissued through bootstrap, equivalent validation may be explicitly reissued by the caller under equivalent relevant state, and lost execute response is conservatively ambiguous. The transport never replays `execute_plan` automatically.

The M0.5 completion record is [`docs/implementation/m0.5-completion-handoff.md`](docs/implementation/m0.5-completion-handoff.md).

## Adapter conformance tooling

M0.6 adds reusable tooling around the published Protocol/Public Contract boundary without introducing a new semantic contract.

Conformance reports distinguish:

```text
Conformant
NonConformant
NotEstablished   # harness / fixture / transport evidence prevents a conformance determination
```

`NotEstablished` is not adapter non-conformance and none of these determinations is a SOL lifecycle state or backend physical-validation result.

The reusable fixture runner covers positive behavior and adversarial cases for compatibility, targets/capabilities, preflight negatives, execution outcomes, ProtocolFailure behavior, scheduling, prior execution, transport loss, aggregate effects, and provenance identity. Parsable request-relative semantic violations become `NonConformant`; execute response loss remains transport/harness evidence and does not authorize replay.

Authoring and external-project entry points:

- [`docs/guides/adapter-authoring-0.1.md`](docs/guides/adapter-authoring-0.1.md)
- [`docs/guides/adapter-conformance-external-project.md`](docs/guides/adapter-conformance-external-project.md)
- [`examples/adapter-authoring/rust-stdio/`](examples/adapter-authoring/rust-stdio/)
- [`examples/external-project-conformance/`](examples/external-project-conformance/)

The example JSON observation, environment-variable names, profile controls, command spelling, and exit behavior remain explicitly provisional; M0.6 did not stabilize a public conformance CLI/report contract.

The M0.6 final audit and completion records are:

- [`docs/implementation/m0.6-adapter-conformance-exit-candidate.md`](docs/implementation/m0.6-adapter-conformance-exit-candidate.md)
- [`docs/implementation/m0.6-completion-handoff.md`](docs/implementation/m0.6-completion-handoff.md)

## Adapter Runtime / Registry

M0.7 adds a solver-neutral runtime layer above the published adapter process/contract boundary.

The accepted architecture is:

```text
BackendTarget
  = canonical solver-independent target semantics

AdapterRegistration
  = Core-local invocation/configuration state

AdapterInstance
  = live operational process/session state
```

`AdapterRegistration` and `AdapterInstance` are not canonical ontology entities and do not define Public Contract semantic identity.

Phase 0 implemented the runtime-local type boundary and architecture counterexamples. Phase 1 implemented deterministic explicit local registration. Later M0.7 phases add process/session composition, live compatibility/capability discovery from `describe_adapter`, solver-neutral selection/projection, and external-adapter integration.

The runtime does not stabilize a plugin marketplace, package/signing/update contract, automatic discovery mechanism, or solver-specific mapping API. Real solver adapters remain external processes and separate repositories.

Primary M0.7 references:

- [`docs/adr/ADR-003-adapter-runtime-registration-boundary.md`](docs/adr/ADR-003-adapter-runtime-registration-boundary.md)
- [`docs/plans/m0.7-adapter-runtime-registry-plan.md`](docs/plans/m0.7-adapter-runtime-registry-plan.md)
- [`docs/implementation/m0.7-runtime-boundary-and-registration-model.md`](docs/implementation/m0.7-runtime-boundary-and-registration-model.md)
- [`docs/implementation/m0.7-explicit-local-adapter-registration.md`](docs/implementation/m0.7-explicit-local-adapter-registration.md)

## Roadmap

```text
M0.1  Semantic Core Bootstrap                  complete
  |
  v
M0.2  Canonical Public Contract 0.1            complete
  |
  v
M0.3  Adapter Protocol 0.1                     complete
  |
  v
M0.4  MockAdapter Protocol Conformance         complete
  |
  v
M0.5  JSON-RPC / stdio Transport               complete
  |
  v
M0.6  Adapter Conformance Tooling              complete
  |
  +-------------------------------+
  |                               |
  v                               v
M0.7 Core runtime track        Real-adapter track
Adapter Runtime & Registry     MOOSE adapter
ACTIVE — Phase 0–1 complete    separate repository/team
  |
  v
future stable SDK / GUI
integration surface
```

The ordering is deliberate: canonical semantics precede SDK ergonomics; Adapter Protocol precedes transport; MockAdapter proves reference behavior before transport; reusable conformance tooling precedes the first SOL reference real adapter; solver-neutral runtime evidence precedes stabilization of broad SDK/GUI adapter-selection surfaces.

## MockAdapter responsibility

MockAdapter is the Core repository's reference conformance implementation, not evidence of solver-native physical correctness. M0.4 completed its promotion from deterministic reference helpers into an executable in-process implementation of the published Adapter Protocol 0.1 contract.

```text
Core repository
  -> semantic/public-contract validation
  -> protocol/schema conformance
  -> deterministic reference behavior
  -> MockAdapter Protocol conformance
  -> reusable external-adapter conformance tooling
  -> solver-neutral Adapter Runtime / Registry

Real adapter repository
  -> backend-native realization correctness
  -> solver API integration
  -> solver-specific regression tests
  -> physical/numerical validation where applicable
```

Real solver adapters remain separate repositories so solver dependencies and release cycles do not contaminate the Core workspace. The first SOL reference real-adapter target is MOOSE, followed by Zapdos/CRANE-focused work as justified by real-system evidence. SOL reference-adapter status does not imply official MOOSE Framework or Idaho National Laboratory component status.

## Version axes

SOL intentionally keeps these versions independent:

- SOL Runtime Release;
- Public Contract;
- Adapter Protocol;
- ontology package version;
- adapter implementation version.

Compatibility must be expressed explicitly through the relevant contract/version evidence rather than inferred from package-version similarity.

## Development method and logical agents

Development is CI-first and counterexample-driven. The canonical logical roles are development/reasoning roles, not runtime SOL concepts:

| Role | Responsibility | Mode/output |
|---|---|---|
| Project session | restore canonical roles and current repository state without mutation | `init` |
| Manager | orchestrate decision-heavy work and decision state | `meeting` |
| Planner | objectives, sequencing, dependencies, milestones | plans and acceptance criteria |
| Researcher | ontology/architecture/contract investigation | decisions, invariants, ADRs |
| Validator | adversarial semantic/compatibility review | APPROVE / APPROVE WITH GATES / REJECT-REVISE |
| Operator | GitHub/CI implementation and documentation synchronization | `resume`, `update` |

In a new chat or after context loss, `init` is the canonical entry mode. It reloads the role model and inspects current GitHub evidence without making changes, then recommends `meeting`, `resume`, or `update`. See [the project session initialization contract](docs/operations/project-session-init.md).

Typical decision-heavy flow:

```text
Manager meeting
 -> Planner
 -> Researcher
 -> ADR when architecture is fixed/changed
 -> Validator
 -> Planner critical revision
 -> user confirmation when required
 -> ACCEPTED
 -> Operator resume
```

`resume` continues accepted implementation through the current GitHub Milestone, parent tracker, Phase issue, PR, CI/fix loop, bounded merge, main verification, and the next eligible Phase until a real gate appears. A planned successor GitHub Milestone does not bypass accepted predecessor dependencies.

`update` synchronizes user/developer-facing documentation with accepted repository state and does not promote unmerged PR work into accepted implementation. A milestone-handoff `update` normally runs after milestone closure; in-progress synchronization is also valid when accepted Phase state or durable GitHub milestone metadata materially changes. Update PRs are not counted as milestone Phase progress units.

Operator may auto-merge already accepted Phase implementation only when exact-head required CI is green, evidence is complete, the PR is mergeable, and no unresolved semantic/architecture/Public Contract/Adapter Protocol/compatibility decision exists. Milestone exit audits, blocking reviews, conflicts, permission failures, non-green CI, and unresolved semantic choices remain real gates.

GitHub Milestone percentage is informational rather than acceptance authority. Phase issues are the canonical milestone progress units; parent tracker issues define normative gates, PRs provide implementation evidence, and Validator/main verification govern closure.

See:

- [`docs/operations/project-session-init.md`](docs/operations/project-session-init.md) for the read-only `init` bootstrap contract;
- [`docs/operations/logical-agent-workflow.md`](docs/operations/logical-agent-workflow.md) for the logical agent operating convention;
- [`docs/operations/github-milestone-convention.md`](docs/operations/github-milestone-convention.md) for GitHub Milestone naming, membership, progress, closure, backfill, and `resume`/`update` integration.

## Repository baselines

- Core architecture / M0.1 plan: [`docs/plans/core-simulation-ontology-v0.1-implementation-plan.md`](docs/plans/core-simulation-ontology-v0.1-implementation-plan.md)
- Product boundary / adapter roadmap: [`docs/plans/product-boundary-and-adapter-roadmap.md`](docs/plans/product-boundary-and-adapter-roadmap.md)
- M0.7 runtime/registry plan: [`docs/plans/m0.7-adapter-runtime-registry-plan.md`](docs/plans/m0.7-adapter-runtime-registry-plan.md)
- Versioning / compatibility policy: [`docs/plans/versioning-and-compatibility-policy.md`](docs/plans/versioning-and-compatibility-policy.md)
- Logical agent workflow: [`docs/operations/logical-agent-workflow.md`](docs/operations/logical-agent-workflow.md)
- GitHub Milestone convention: [`docs/operations/github-milestone-convention.md`](docs/operations/github-milestone-convention.md)
- Spatial Scope ADR: [`docs/adr/ADR-001-first-class-spatial-scope.md`](docs/adr/ADR-001-first-class-spatial-scope.md)
- Adapter Protocol boundary ADR: [`docs/adr/ADR-002-adapter-protocol-boundary.md`](docs/adr/ADR-002-adapter-protocol-boundary.md)
- Adapter Runtime/Registration boundary ADR: [`docs/adr/ADR-003-adapter-runtime-registration-boundary.md`](docs/adr/ADR-003-adapter-runtime-registration-boundary.md)
- M0.2 completion handoff: [`docs/implementation/m0.2-completion-handoff.md`](docs/implementation/m0.2-completion-handoff.md)
- M0.3 exit audit: [`docs/implementation/m0.3-adapter-protocol-0.1-exit-audit.md`](docs/implementation/m0.3-adapter-protocol-0.1-exit-audit.md)
- M0.4 exit audit: [`docs/implementation/m0.4-mock-adapter-conformance-exit-audit.md`](docs/implementation/m0.4-mock-adapter-conformance-exit-audit.md)
- M0.4 completion handoff: [`docs/implementation/m0.4-completion-handoff.md`](docs/implementation/m0.4-completion-handoff.md)
- M0.5 exit audit: [`docs/implementation/m0.5-json-rpc-stdio-transport-exit-audit.md`](docs/implementation/m0.5-json-rpc-stdio-transport-exit-audit.md)
- M0.5 completion handoff: [`docs/implementation/m0.5-completion-handoff.md`](docs/implementation/m0.5-completion-handoff.md)
- M0.6 exit audit: [`docs/implementation/m0.6-adapter-conformance-exit-candidate.md`](docs/implementation/m0.6-adapter-conformance-exit-candidate.md)
- M0.6 completion handoff: [`docs/implementation/m0.6-completion-handoff.md`](docs/implementation/m0.6-completion-handoff.md)
- M0.7 Phase 0 record: [`docs/implementation/m0.7-runtime-boundary-and-registration-model.md`](docs/implementation/m0.7-runtime-boundary-and-registration-model.md)
- M0.7 Phase 1 record: [`docs/implementation/m0.7-explicit-local-adapter-registration.md`](docs/implementation/m0.7-explicit-local-adapter-registration.md)
- M0.1 tracker: [Issue #2](../../issues/2)
- M0.2 tracker: [Issue #28](../../issues/28)
- M0.3 tracker: [Issue #38](../../issues/38)
- M0.4 tracker: [Issue #65](../../issues/65)
- M0.5 tracker: [Issue #74](../../issues/74)
- M0.6 tracker: [Issue #81](../../issues/81)
- M0.7 tracker: [Issue #111](../../issues/111)

## Planned distributions

SOL remains language-neutral even though the current semantic Core is implemented in Rust.

```text
Cargo   -> Rust SOL facade/Core distribution
binary  -> sol / sol-cli
npm     -> TypeScript SDK after contract/protocol stabilization
PyPI    -> Python SDK after contract/protocol stabilization
```

Experimental SDK work may begin earlier, but stable SDKs must consume and preserve the canonical Public Contract and Adapter Protocol rather than freeze incidental Rust internals.