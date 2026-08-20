# SOL v0.1 Phase 1 reference harness — independent validation v0.1

**Status:** Independent Validation role  
**Date:** 2026-08-20  
**Branch:** `validation/sol-v0.1-independent-review`

## Inputs

Validation used only the Proposed ADR contracts and durable implementation/evidence artifacts:

- `docs/decisions/0011-mapping-plan-determinism-and-executable-backend-target.md`
- `docs/decisions/0012-qualified-relation-cardinality.md`
- `schema/qrc-v0.1.schema.json`
- `schema/mapping-contract-v0.1.schema.json`
- `tests/reference_validator.py`
- `tests/test_reference_validator.py`
- `tests/test_json_schemas.py`
- `tests/phase1-test-results.txt`

The recorded evidence reports 19 passing tests under Python 3.13.5 and jsonschema 4.26.0. Passing existing tests is evidence, not the validation verdict.

## Verdict

**REVise — Phase 1 is not yet contract-complete.**

No new architecture defect is established. The findings below are reference-implementation / validation-tooling defects because the Proposed ADR text already specifies the required deterministic behavior.

## Findings

### P1-01 — Comparator schema and runtime resolver are not interoperable

**Classification:** Validation-tooling defect.

`ComparatorRegistryEntry` schema stores structured fields (`scope`, component binding(s), `comparison_purpose`, `left_kind`, `right_kind`). `resolve_comparator()` instead expects each registry entry to contain a precomputed opaque `key` field. That field is not allowed by the schema (`additionalProperties: false`). Therefore an object valid under the machine-readable registry schema cannot be passed directly to the reference resolver.

**Counterexample:** validate a registry entry with the JSON Schema, then pass the same object to `resolve_comparator`; the resolver has no conforming `key` to compare.

**Required correction:** derive the canonical lookup key from schema-valid registry entry fields, or make the schema and resolver share one explicit canonical representation. Add round-trip tests for local, symmetric cross-component, and directional cross-component keys.

### P1-02 — Effect-pair pruning accepts unproven disjointness

**Classification:** Validation-tooling defect.

`retained_pairs(effects, proven_disjoint)` accepts arbitrary effect-ID tuples and prunes them without comparator identity/version, normalized input fingerprint, immutable target snapshot, or conclusive `separate + non-overlap` evidence. A caller can therefore remove a conflicting pair by assertion, violating ADR-0011's proof-only pruning contract.

**Required correction:** pruning input must be a provenance-bearing comparison evidence record produced by exactly-one comparator resolution. Unknown, missing, BLOCKED, or INDETERMINATE evidence must retain the pair. Add a negative test proving a bare tuple cannot prune.

### P1-03 — Executable effect coverage and bookkeeping are not bound to an Adapter contract

**Classification:** Validation-tooling defect.

`validate_effect_coverage()` receives already-derived effects plus a caller-supplied list of `bookkeeping_kinds`. It does not exercise the contract that the validated executable descriptor is passed to a deterministic `describe_effects()` procedure, nor does it prove that bookkeeping classification comes from the selected versioned Adapter contract. The same issue exists for state-independent idempotency: a boolean `adapter_guarantee` can be supplied without adapter identity/version evidence.

**Required correction:** introduce a synthetic versioned AdapterContract fixture that owns `describe_effects`, bookkeeping classification, state-independent idempotency guarantees, and state-dependent procedure bindings. The validator must resolve these through the adapter contract rather than accepting free caller policy. This remains synthetic contract-mechanics evidence, not backend truth.

### P1-04 — Action dependency validation omits accepted producer/external/order boundaries

**Classification:** Validation-tooling defect.

`validate_action_graph()` always fails duplicate producers and does not implement the accepted paths for producers proven equivalent and deterministically coalesced. It also treats an external handle as either available or absent; it cannot represent a declared-but-unresolved external binding as `BLOCKED`. `must_precede` is accepted directly from action input without provenance-bearing adapter/lifecycle evidence, allowing an author to invent ordering direction. Atomic compound-action semantics are not exercised.

**Required correction:** add explicit external-binding state, producer-equivalence/coalescing evidence, provenance-bearing `must_precede` evidence, and synthetic semantic-all-or-none atomicity checks. Add PASS/FAIL/BLOCKED/INDETERMINATE boundary tests.

### P1-05 — BackendTarget validator can publish BLOCKED when a known FAIL exists

**Classification:** Validation-tooling defect.

`validate_backend_target()` returns immediately on the first component problem. Example: component A is unresolved (`BLOCKED`) and component B has an adapter-contract mismatch (`FAIL`). If A sorts first, the function returns `BLOCKED` and never evaluates B. ADR-0011 requires plan validation aggregation `FAIL > BLOCKED > INDETERMINATE > PASS` across independently evaluable checks.

**Required correction:** collect independently evaluable structural/runtime decisions and aggregate them canonically. Preserve diagnostics for all evaluated findings. Add a mixed `BLOCKED + FAIL` multi-component test whose final decision is always `FAIL`, independent of component ordering.

### P1-06 — QRC stable identity test proves duplicate-edge dedupe, not alias resolution

**Classification:** Validation-tooling defect.

`evaluate_qrc()` deduplicates the serialized target IDs with a Python set. This proves duplicate identical IDs count once, but the accepted contract requires counting after canonical identity / alias resolution. Two different serialized aliases referring to one canonical target currently count twice.

Additionally, when `qualifier_candidates` resolves an authored qualifier to a canonical type ID, diagnostics are built from the original authored string rather than the resolved canonical qualifier identity, contrary to ADR-0012's failure-evidence contract.

**Required correction:** add canonical target-identity resolution evidence before counting and report the resolved canonical qualifier identity. Add alias-equivalent-target and canonical-qualifier diagnostic tests.

### P1-07 — Canonical loss key encoding is collision-prone

**Classification:** Validation-tooling defect.

`canonical_loss_key()` concatenates three identities with `|`. Without escaping or length-prefix/canonical structured encoding, different triples can produce the same key, e.g. `("a|b", "c", "d")` and `("a", "b|c", "d")`. This violates the requirement for a stable canonical loss identity.

**Required correction:** use collision-safe canonical structured encoding and add the counterexample test.

### P1-08 — Machine-readable PlanAction/idempotency contract remains too permissive for the claimed Phase-1 scope

**Classification:** Validation-tooling defect.

The Mapping schema requires `PlanAction.idempotency` but leaves it unconstrained (`{}`), and executable `descriptor` is also structurally unconstrained. The Phase-1 plan explicitly requires machine-readable PlanAction/evaluation contract evidence. A malformed idempotency object can therefore pass structural validation and only fail if a particular runtime path happens to inspect it.

**Required correction:** define a minimal synthetic-harness schema for state-independent versus state-dependent idempotency and for executable descriptor identity/component binding sufficient to exercise the accepted contract. Do not standardize backend-native descriptor vocabulary.

## Accepted evidence retained

The following Phase-1 evidence is useful and should be preserved through revision:

- MappingRule five-field and MappingClaim four-field envelopes reject extra top-level fields;
- QRC mixed-bound normalization and closed-snapshot BLOCKED behavior are exercised;
- cross-component comparator context distinguishes symmetric from directional purposes;
- decision precedence, representability precondition, loss-policy conjunction, cycle detection, and schema meta-validation have positive test evidence;
- repository-root import reproducibility has been corrected.

## Gate decision

| Item | Verdict |
|---|---|
| Phase-1 schema evidence | **Revise** |
| Phase-1 reference validator | **Revise** |
| Existing 19-test execution evidence | **Valid but insufficient** |
| Architecture defect discovered | **No** |
| ADR-0011 / ADR-0012 contract reopened | **No** |
| Proceed to Thermal backend execution as freeze evidence | **Not yet** |

## Next state

Return findings P1-01 through P1-08 to Research as the only revision scope. Produce a Phase-1 v0.2 harness, rerun from repository root, record immutable execution evidence, then perform another independent Phase-1 validation. Architecture freeze remains blocked.
