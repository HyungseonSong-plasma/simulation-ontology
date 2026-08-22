# ADR-003 — Adapter Runtime and Registration Boundary

**Status:** Accepted  
**Date:** 2026-08-22  
**Milestone:** M0.7 Adapter Runtime & Registry  
**Issue:** #111

## Context

M0.3 published Adapter Protocol 0.1, M0.5 established the default local JSON-RPC-over-stdio process transport, and M0.6 completed reusable external-adapter conformance tooling. The Core can therefore communicate with and test an external adapter process, but it does not yet own a product/runtime model for registering available adapter commands, managing live adapter sessions, inspecting compatibility/capabilities, or selecting an eligible adapter for a canonical backend target.

The first real-adapter track now proceeds independently from Core. SOL therefore needs a solver-neutral runtime boundary before future SDK/GUI work can present adapters as selectable backend capabilities without embedding solver-specific dependencies or turning local process/install state into ontology semantics.

The Manager meeting on 2026-08-22 accepted the M0.7 direction and the distinction between canonical backend meaning, local adapter registration, and live adapter process state.

## Decision

### 1. `BackendTarget`, `AdapterRegistration`, and `AdapterInstance` are distinct

SOL SHALL keep three concepts separate:

```text
BackendTarget
  = canonical solver-independent target semantics

AdapterRegistration
  = Core-local runtime/configuration record used to locate and invoke an adapter command

AdapterInstance
  = operational state for a live adapter process/session
```

`BackendTarget` remains part of canonical SOL/Public Contract meaning where already published.

`AdapterRegistration` and `AdapterInstance` are runtime/platform concepts. They are not ontology entities, do not define canonical semantic identity, and MUST NOT be promoted into Public Contract semantic payloads merely because a GUI or runtime needs them.

Executable paths, command arguments, working directories, local registration IDs, PIDs, transport request IDs, and session identifiers are therefore non-semantic runtime data.

### 2. Adapters remain external processes

M0.7 preserves the accepted process boundary:

```text
SOL Core / runtime
      <-> Adapter Protocol 0.1
      <-> JSON-RPC over stdio
external adapter process
      <-> backend-native runtime/API/artifacts
```

A real solver adapter SHALL NOT become an in-process plugin dependency of Core merely to support registration or GUI selection.

M0.7 MUST NOT add MOOSE, COMSOL, Ansys, Zapdos, CRANE, or another solver runtime dependency to the Core workspace.

### 3. Initial registration is explicit and local

M0.7 starts with an explicit local registration model. A registration may contain only the local information required to locate/invoke an adapter command and manage that registration.

The milestone does not stabilize a universal adapter package format, marketplace, directory-scanning convention, signing/trust distribution model, download/install protocol, or auto-update system.

Those product surfaces require demonstrated requirements and an explicit later decision.

The exact persistence file shape, CLI spelling, environment-variable names, and storage location used by an initial implementation remain **Experimental** unless separately reviewed and stabilized.

### 4. Registration is a locator, not compatibility/capability truth

Static registration metadata MUST NOT be treated as authoritative proof that an adapter is compatible with a requested SOL contract/target/capability set.

After launch/bootstrap, the runtime SHALL use the published `describe_adapter` result as the live protocol evidence for:

- adapter implementation identity/version;
- supported Adapter Protocol versions;
- supported Public Contract versions;
- addressed backend targets;
- declared capabilities.

Interoperability still requires both:

```text
Adapter Protocol compatibility
AND
Public Contract compatibility
```

Adapter package version or local registration identity proves neither.

If static registration hints contradict the live protocol description, the runtime MUST NOT silently elevate the registration hint over the protocol evidence.

### 5. Runtime selection remains solver-neutral

Core may use canonical `BackendTarget` requirements plus live compatibility/capability evidence to identify eligible adapter instances.

Selection behavior MUST be deterministic for equivalent registry/runtime state and MUST define explicit outcomes for at least:

- no compatible candidate;
- exactly one compatible candidate;
- multiple compatible candidates.

The generic runtime SHALL NOT encode solver-specific semantic branches such as `if backend == MOOSE` in order to decide canonical meaning.

A future GUI or SDK may project adapter availability and selection through a solver-neutral runtime view, but exact GUI layout and stable SDK API shape are not established by this ADR.

### 6. Process/session lifecycle reuses M0.5 semantics

M0.7 manages adapter lifecycle by composing the already-accepted M0.5 subprocess/session transport rather than redefining it.

The following boundaries remain normative:

- a fresh session bootstraps through `describe_adapter` before typed validation/execution use;
- transport/process failures remain distinct from logical `ProtocolFailure`;
- reconnect creates a fresh bootstrapped process session;
- `validate_plan` remains advisory and side-effect free;
- `execute_plan` remains authoritative and non-idempotent by default;
- an ambiguous lost `execute_plan` response preserves possible side effects and grants no automatic replay authority;
- process IDs, stderr, JSON-RPC request IDs, reconnect state, and framing metadata do not become Protocol/Public Contract semantics.

### 7. M0.7 does not redefine Protocol 0.1 or Public Contract 0.1

Adapter Runtime & Registry is a Core product/runtime layer built on the already published contracts.

M0.7 SHALL NOT silently add operations, fields, lifecycle states, compatibility semantics, retry authority, backend identity meaning, or solver-native objects to Adapter Protocol 0.1/Public Contract 0.1.

If real-adapter or runtime evidence demonstrates that the published contract is insufficient, the affected path stops and returns to Manager/Researcher/Validator review. An incompatible change requires an explicit new version boundary rather than an in-place reinterpretation of 0.1.

### 8. Plugin-like UX is permitted; plugin semantics are not implied

A future GUI MAY present registered adapters with a plugin-like user experience—for example listing available backends, compatibility, capabilities, and selected adapter state.

That UX does not imply an in-process plugin architecture or make adapter registration/process state part of the ontology.

Conceptually:

```text
GUI / SDK
   -> solver-neutral runtime projection
   -> Adapter Runtime / Registry
   -> selected external adapter process
```

The GUI SHOULD edit/operate on canonical SOL concepts and runtime adapter availability rather than directly constructing solver-native objects.

## Responsibility matrix

| Concern | Owner / classification |
|---|---|
| Canonical backend target meaning | Core / Public Contract |
| Adapter command/path registration | Core-local runtime configuration |
| Registration persistence mechanics | Core implementation, Experimental unless stabilized |
| Live process/session state | Core runtime / M0.5 transport composition |
| Adapter identity/version | Adapter Protocol description evidence |
| Protocol/Public Contract compatibility | Adapter Protocol description + Core compatibility evaluation |
| Target/capability declaration | Adapter Protocol description evidence |
| Deterministic eligible-adapter selection | Core runtime |
| Backend-native mapping/execution | external adapter repository/runtime |
| Backend physical/numerical correctness | external adapter repository V&V |
| GUI presentation | future product/GUI layer, not canonical ontology |
| Plugin package/marketplace/signing/update | deferred |

## Rejected alternatives

The following designs are rejected for M0.7:

- treating `AdapterRegistration` or `AdapterInstance` as canonical ontology classes;
- using executable path, PID, local registration ID, or solver-native object IDs as canonical semantic identity;
- loading real solver adapters as in-process Core dependencies;
- trusting static registration metadata as authoritative compatibility/capability evidence;
- branching generic Core semantics on MOOSE/COMSOL/Ansys-native concepts;
- stabilizing a marketplace/package/signing/auto-update system before demonstrated need;
- redefining Adapter Protocol 0.1 to make runtime management convenient;
- automatically replaying `execute_plan` after ambiguous transport response loss.

## Consequences

This boundary gives SOL a product/runtime layer capable of supporting future adapter selection, SDKs, and GUI integration while preserving the solver-independent semantic architecture.

The cost is an additional explicit distinction between canonical target semantics, persistent/local registration state, and live process/session state. Implementations must not collapse those layers for convenience.

Because registration metadata is intentionally not trusted as interoperability truth, a live/bootstrap step is required before compatibility/capability-based selection is established.

## Validation obligations

M0.7 implementation MUST include counterexamples that reject at least:

- `BackendTarget == AdapterRegistration`;
- `AdapterRegistration == AdapterInstance`;
- executable path/PID/session identity used as canonical semantic identity;
- static registration claiming compatibility while live `describe_adapter` reports incompatibility or missing evidence;
- solver-specific branching in the generic selection layer;
- adapter process/runtime dependency leakage into Core;
- ambiguous execute-response loss converted into automatic replay authority.

The milestone exit audit must additionally verify that Protocol/Public Contract 0.1 meanings remain unchanged and that the external-process integration path works through the published transport boundary.

## Supersession

This ADR is normative for the M0.7 runtime/registration architecture. Changes that collapse these boundaries, stabilize new public plugin/package surfaces, or alter Adapter Protocol/Public Contract meaning require a new Manager/Researcher/Validator decision and explicit ADR update/supersession.