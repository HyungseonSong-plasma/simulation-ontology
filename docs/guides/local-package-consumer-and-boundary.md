# SOL Package Boundary — Contract SDK pilot and workstation acceptance

**Status:** Draft implementation/consumer guide, not a release announcement  
**Scope:** SOL Platform Boundary #176, proposed ADR-007  
**Baseline:** Public Contract 0.1/0.2 supported; Public Contract 0.3 result branch #173 and Adapter Protocol 0.3 are gated separately.  
**Related adapter:** [sol-adapter-moose package #40](https://github.com/HyungseonSong-plasma/sol-adapter-moose/issues/40)

## Ownership and support classification

There are two **different** types of public surface. A released *contract format and its normative meaning* can be stable while the incidental Rust API of the library that parses it remains Experimental. A `pub` Rust type is not automatically a stable package promise.

| Existing SOL crate/group | Owners and intended consumers | Release classification at this proposal |
| --- | --- | --- |
| `sol-core-model`, `sol-core-identity`, `sol-core-qrc`, `sol-core-mapping`, `sol-core-plan`, `sol-core-effect`, `sol-core-evaluation` | Solver-independent Core engine; not a necessary adapter dependency | Internal Core; expose only separately documented facade |
| `sol-public-contract` + `schemas/public-contract/` | SOL normative JSON/DTO contract; Adapter/SDK consumer | Contract **meaning** versioned by Public Contract; Rust crate release/API tier to be separately accepted |
| `sol-adapter-protocol` + `schemas/adapter-protocol/` | SOL Adapter interoperability definitions | Contract **meaning** versioned by Adapter Protocol; crate ABI/API review still required |
| `sol-adapter-transport` | Reusable stdio/JSON-RPC client/server process plumbing | Experimental reusable library; accepted framing/no-replay invariants cannot change silently |
| `sol-adapter-conformance*` | Runnable tests, published fixture corpus and harness | Versioned conformance assets; serialized runner CLI/report controls remain provisional |
| `sol-adapter-runtime`, `sol-target-resolver` | SOL Runtime process selection and canonical result acceptance | SOL Runtime implementation; no direct adapter-owned canonical meaning |
| `sol-cli` | SOL user/SDK-facing facade operations | Stable only for explicitly accepted CLI commands/JSON exit behavior |
| `sol-mock-adapter` | Reference/conformance executable | Test/reference only, not a mandatory adapter dependency |
| `sol-ontology-thermal`, `sol-ontology-plasma` (future) | Scientific/domain semantics, versioned independently | Not currently published; **do not** pretend these artifacts already exist |
| `sol-adapter-moose` (external) | MOOSE binary, HIT generation, backend execution/result decoding | Separate independent distribution/ABI/installer, adapter Issue #40 |

### Versioning invariants

- **Crate package version**, **SOL Runtime release**, **Public Contract**, **Adapter Protocol**, **Ontology Package** and **Adapter executable version** are distinct independent axes.
- The current `[workspace.package] version = "0.1.0"` is not an interoperability compatibility promise.
- On actual handoff, record `{sol_release_sha, crate_package_version, supported_public_contract_versions, supported_adapter_protocol_versions, ontology_package_dependencies, supported_capabilities}`.
- Original Protocol/Public Contract 0.1/0.2 meanings cannot be changed through package repackaging. 0.3 semantics in Draft PR #173 are not stable until independently accepted.
- Avoid floating `main` and a locally patched `vendor/sol-*` contract: the Adapter should consume a reviewed immutable SOL source/release revision. Until release parity has been established, keep the current vendored adapter behavior unchanged.

## D0: immediately runnable isolated source-checkout smoke

This **tests SOL's three existing Contract SDK crates from a separate temporary Cargo workspace**. It deliberately does not build or require MOOSE, change an Adapter repository, publish a package, or claim a stable release.

Preconditions: Git, Python 3, Rust/Cargo (MSRV declared as Rust 1.80, CI pins 1.80.1); network access to resolve uncached dependencies. From a checked-out SOL repository:

```bash
git rev-parse HEAD

# To require exactly the checked-out commit:
export SOL_EXPECTED_SHA="$(git rev-parse HEAD)"

# Optional evidence (retained outside temporary build):
export SOL_EVIDENCE_DIR="$HOME/sol-local-evidence"

bash scripts/local-contract-consumer-smoke.sh
```

Alternatively, supply a different checked-out SOL source tree:

```bash
bash scripts/local-contract-consumer-smoke.sh /path/to/simulation-ontology
```

The smoke script creates a fresh temporary **external** Cargo project that depends on `sol-public-contract`, `sol-adapter-protocol`, `sol-adapter-transport` using explicit source-checkout paths, generates its own `Cargo.lock`, and runs three compiler-backed tests: frozen Protocol/Contract version constants and method names, the original 0.2 thermal `RealizationSpec`, and the 0.3 *scalar-only* thermal run record (already accepted in Phase 1). It checks an expected Git SHA if provided and records machine/toolchain metadata.

The CI job runs the same script on its own runner. **A local path source consumer is NOT a validated published Git package, crates.io package, or real external-adapter result.** `SOL_EVIDENCE_DIR` receives environment, consumer Cargo.lock and test stdout for later Human workstation attestation. The script does not write or delete user content in the SOL checkout.

### Platform-specific notes

- Linux and macOS: run with bash. The script uses Python 3 for escaping absolute paths (including whitespace) in Cargo TOML. Standard Rust installation for the target host must exist.
- Windows: D0 can be run from Git Bash with Cargo/Python 3 on PATH, or from WSL with Linux Rust. **Native PowerShell and Windows ABI release validation are not yet implemented** and must be tested independently before making a Windows support claim.
- The script does not use sudo, install a solver, access personal experiment folders or change the default Rust toolchain.

## P1 pilot: independently consumed immutable upstream Git revision (not a release)

The P1 pilot is stronger than D0's same-checkout `path` dependencies. A genuinely independent Cargo project pins all three SOL contract crates to **the same immutable upstream Git SHA** `b6924e6556684eec9dd93677518821cc7adc84b6` in `examples/external-contract-git-consumer/Cargo.toml`.

This SHA is a currently accepted upstream `main` baseline, **not a signed release/tag**. It contains Public Contract 0.1/0.2 plus previously accepted **scalar-only** Public Contract 0.3, and Adapter Protocol 0.1/0.2. This pilot does not include experimental M0.10 Field/Dataset Draft PR #173 or Adapter Protocol 0.3.

On the CI runner or a local macOS/Linux workstation with Git, Cargo/Rust and Python 3 installed:

```bash
# From the SOL source checkout of Draft PR #177:
export SOL_EVIDENCE_DIR="$HOME/sol-local-evidence"
bash scripts/verify-git-contract-consumer.sh
```

The script copies an external Cargo project to a temporary directory, downloads its `git+https` dependencies from SOL's pinned upstream commit (never `vendor/sol-*`, current workspace crates or local `path` replacements), checks Cargo metadata for all three Git source IDs and exact commit/Crate version `0.1.0`, and reads the test fixtures **from that pinned upstream Git checkout**. It runs four real Rust tests against supported contract behavior, then deliberately requires an incompatible upstream crate version `=0.2.0` and checks Cargo rejects that dependency resolution. A `cargo test --locked` is used after constructing the isolated lockfile.

Optional evidence under `SOL_EVIDENCE_DIR`: `git-consumer.Cargo.lock`, Git package metadata, four-test stdout, negative dependency resolution log and environment metadata. These files may be supplied by the user to review the actual workstation test.

**Limitations:** The test creates `Cargo.lock` at execution time rather than consuming a reviewed official release lock; the tested upstream commit is immutable but no package archive/checksum/SBOM, signed tag, crates.io publication, downstream adapter binary or installer exists from this pilot. The mismatch negative tests a Crate **package version** failure; a distinct runtime Adapter Protocol/Public Contract compatibility test remains mandatory. The user's own local computer has not been tested simply because GitHub Actions passes.

## D1: immutable source/contract package release (not done)

Before promoting the source-checkout smoke to release evidence, a future follow-up must implement:
1. Explicit release/tag and full commit SHA, release manifest and artifact/schema/fixture SHA-256 checksums, dependency inventory and support-tier declaration.
2. Registry/package-ready manifests with any required versioned transitive `path` dependency declarations; prove independent consumer installation from **released immutable packages** or checked immutable Git revision and review of licensing/SBOM.
3. Negative compatibility tests (wrong contract/protocol pair or unsupported capability) and old 0.1/0.2 golden parity; no 0.2→0.3 auto-promotion.
4. Upstream SOL source/package parity before the Adapter replaces `vendor/sol-*`. Adapter binary install/uninstall parity remains tracked in Adapter #40.

Do not create a tag, publish crates or rewrite existing adapter vendor crates until these gates are passed.

## D2: actual local SOL → Adapter → MOOSE verification (separate acceptance)

A real local end-to-end acceptance must be run **on the user's workstation**, not simulated in a cloud workflow. After immutable SOL and Adapter releases exist:

1. Confirm OS+architecture+ABI+Rust/MOOSE application versions, and verify the release SHA/checksums.
2. Install the Adapter **binary** using the Adapter #40 installer in a user-selected prefix; check `describe_adapter` and the explicit Adapter Protocol / Public Contract compatibility pair.
3. Exercise expected refusal for mismatched pair, missing MOOSE executable, or unsupported capability. Missing executable must never become scientific PASS.
4. If `SOL_MOOSE_EXECUTABLE` points to a tested MOOSE app, invoke pinned external-runtime-v02 example with the same accepted request as the Adapter; verify actual thermal solve, output/provenance and no-replay/cleanup. The current external runtime v0.2 command is documented in `examples/external-runtime-v02/README.md`, and uses `cargo run --manifest-path examples/external-runtime-v02/Cargo.toml -- run <adapter-command> <consumer-request.json>`.
5. After approved 0.3 Protocol/Contract adapters exist, test two requested ObservationModels (Scalar MaximumTemperature and distributed TemperatureField), explicit terminal coverage, real DataReference authorization and independent Physics signoff.
6. Uninstall only adapter-owned files; external MOOSE executable, experiments, Solver outputs and unrelated versions must remain untouched.

Record **distinct** judgments for Package/ABI install, Contract/Protocol compatibility, Runtime interoperability, Scientific V&V and Human Validator acceptance. Do not equate green CI with the local-test evidence.

## Exit / project status

- D0 external source-consumer CI: candidate branch only; prove through exact-head CI.
- D1 immutable distribution and real adapter vendor removal: **OPEN**.
- D2 actual workstation end-to-end run: **OPEN**, Human to attest.
- M0.10 #156/#175 independent human result meaning/validator: **OPEN**.
