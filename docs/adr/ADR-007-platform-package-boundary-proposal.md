# ADR-007 (proposal) — SOL Platform/Contract Package Boundary and Consumer Isolation

**Status:** PROPOSED / Human acceptance and release verification required  
**Date:** 2026-11-06  
**Issue:** #176  
**Relationship:** Implements the existing product/adapter boundary and versioning policy; does not supersede ADR-001–ADR-006 or authorize M0.10 #156/#173 merge.

## Problem

Separate solver adapters can advance faster than SOL's canonical contract implementation. Today `sol-adapter-moose` carries three path-based SOL crates under `vendor/` while SOL itself has a versioned multi-crate Cargo workspace. In the absence of an immutable consumer release, adapter work risks establishing unauthorized parallel meaning for canonical entities, outcomes, units, result interfaces, and protocols. Source-tree compilation also does not prove that a consumer can use a distributed package on a clean local workstation.

## Proposal — ownership is frozen before distribution

### SOL canonical semantic authority
- SOL Core owns canonical entity and relation vocabulary, identity, reference resolution, scopes, QRC/semantic constraints, lifecycle and accepted MappingPlan/RealizationSpec semantics.
- SOL Public Contract owns public DTO and JSON meaning, schemas, versions, canonical reference/equality rules and Result occurrence/terminal-outcome semantics.
- SOL Adapter Protocol owns interoperable request/response, explicit compatibility, error, preflight, execution and no-replay semantics. Transport owns framing/session process errors and cannot reinterpret the logical Protocol.
- SOL Conformance owns versioned positive/negative fixtures, source compatibility tests and determinations. SOL Runtime owns live selection, contract verification and governed canonical result acceptance. Storage backend implementations may be delegated but must implement the SOL-defined resolver and authorization boundary.
- Domain ontologies (thermal/plasma/etc.) are independently versioned packages of accepted scientific/semantic concepts. They can request a new SOL concept but must not redefine an existing stable ID.

### Separate solver/Physics ownership
- `sol-adapter-moose` independently owns MOOSE input/HIT, solver invocation, Exodus/CSV read/extraction, native-to-approved mapping implementation, runtime capability reporting and opaque backend provenance. It cannot define or change SOL canonical meaning.
- `moose-test-repo` (Physics) owns scientific models, experiments, benchmarks and scientific-domain validation. Scientific signoff and adapter conformance are separate.
- Native file SHA/paths, MOOSE variables, COMSOL tags, ANSYS handles and solver-specific IDs never serve as a canonical SOL Observation identity or Dataset meaning. No solver executable/library is a SOL Core dependency.

### Official consumer-facing packages (initial proposal)
Release the versioned **Contract SDK** as independently consumable Rust crates plus language-neutral schemas and fixture bundles:

| Surface | Candidate artifact | Public support treatment |
| --- | --- | --- |
| Canonical DTO/reference/schema compatibility | `sol-public-contract` + schema bundle | Supported only for explicitly declared **Public Contract** versions |
| Adapter request/response and compatibility | `sol-adapter-protocol` + fixtures | Supported only for explicitly declared **Adapter Protocol** versions |
| JSON-RPC/stdin-stdout implementation | `sol-adapter-transport` | Reusable source distribution; public Rust source/API stabilization requires explicit review |
| Conformance model/harness/fixtures | `sol-adapter-conformance*` | Reusable test distribution, versioned fixtures; provisional CLI/report flags are not stable API |
| Semantic Core/internal runtime crate graph | `sol-core-*`, `sol-adapter-runtime` and `sol-target-resolver` | Engine implementation; expose only separately declared stable facade/CLI operations |
| Domain ontology | e.g. `sol-ontology-thermal` in later release | Independent semantic package version and accepted references; not bundled into Core by naming alone |
| Real MOOSE adapter executable | External `sol-adapter-moose` distribution | Adapter Team release, distinct from Contract SDK; see Adapter #40 |

A package's Rust `0.1.0` version does **not** prove support for Public Contract `0.2` or Adapter Protocol `0.2`. The supported Protocol/Public Contract *pair* is explicit and must be tested at runtime; Public Contract `0.3` and Adapter Protocol `0.3` remain gated by M0.10 until accepted. No silent 0.2 to 0.3 promotion.

### Contract gap handling (fail closed)
If an Adapter requires a missing canonical concept or operation:
1. Open a SOL Contract Gap with use case, proposed canonical semantics, rejected backend-specific leakage and backward-compatibility counterexamples.
2. Backend-private experimental code may continue **only** behind an explicitly experimental/private capability. It cannot assert canonical `produced`, inject undisclosed shared fields, reinterpret identifiers or claim new Protocol support.
3. Until SOL publishes the new meaning and versioned conformance, respond with an appropriate typed unsupported/unavailable/not-produced result **where the accepted contract defines that outcome**; otherwise reject interoperability, not fabricate a new status.
4. Publish an accepted SOL Contract/Protocol package version and update the Adapter's immutable dependency pin; run both unit and cross-repo conformance/Physics V&V before enabling the capability.

### Progressive distribution, not premature registry publication
**D0:** Separate external Cargo consumer using SOL source checkout + a pinned SHA; run on CI and a developer PC. Path dependencies are a *source-checkout probe*, not package-release proof.
**D1:** Immutable Git commit/tag distribution with a machine-readable release manifest (source SHA, artifact SHA/checksum, package version, supported protocol/contract pair, toolchain/MSRV, license/SBOM/provenance, schema+fixture hashes). Verify an independent git-revision consumer and exact-lock reproducibility.
**D2:** Audit and adapt Cargo manifest dependency versions and packaging closure before deciding crates.io vs private registry. Do not assume transitive path-only crates already satisfy registry publishing. Published Rust APIs must carry an explicit support tier; avoid stabilizing every internal type simply because it is `pub`.
**D3:** Coordinate adapter vendored-contract removal/parity testing against Adapter #40. Adapter native OS/ABI installer, external MOOSE preflight and safe uninstall are *not* SOL release implementation responsibilities.

### Local acceptance as a mandatory release gate
On the actual target workstation (macOS/Linux/Windows using supported shell/toolchain), record OS/arch, `rustc -V`, `cargo -V`, exact SOL source/release SHA, dependency lock, execution commands, exit codes and stdout/stderr evidence.

Minimum stages:
1. External **source-checkout** Contract SDK consumer compiles and passes semantic fixture tests in a temporary workspace (pre-release evidence only).
2. External **immutable release-consumer** uses the published pinned revision/archive, without a vendor fork, with old-contract fixtures and explicit incompatibility negatives.
3. Separately install/describe/validate/uninstall an exact adapter **binary release** on the same workstation (Adapter #40), without altering MOOSE or user experiment data.
4. When a supported MOOSE executable is present, invoke the existing pinned Runtime 0.2 + Adapter 0.2 steady thermal integration; later repeat with approved 0.3 results. Record interoperability and scientific V&V separately.
5. Human confirms results and decides final package acceptance. CI is necessary but does not substitute for tests on the user's actual workstation, nor for Human independent Validator verdict.

## Rejected shortcuts

- Copy/vendoring SOL contract code permanently in each adapter as a second normative owner.
- Letting adapters invent public ontology IDs/quantities, protocol status values, or `produced` results.
- Monolithic SOL executable packaging as the only contract distribution path.
- Treating successful Cargo workspace CI, a temporary path-dependent smoke test, or a Git SHA as proof of a registry-published package.
- Inferring compatibility from crate package versions, or shipping unapproved 0.3 semantics as stable.
- Shipping or installing the MOOSE backend as part of the SOL contract package.

## Decision gates

This document is a **proposal** with an already user-approved *direction* but is not a signed-off packaging specification. Human/Manager must separately confirm the first official release support matrix, registry/distribution channel, target OS/ABI matrix and promotion rules. Subsequent implementation work must pass exact-head CI and existing branch-protection/review requirements. Keep M0.10 Phase 2 and Human Validator Issue #175 independent.
