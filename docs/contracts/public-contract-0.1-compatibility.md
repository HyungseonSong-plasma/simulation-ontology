# Public Contract 0.1 — Compatibility Harness

**Status:** M0.2 Phase 5 normative regression baseline  
**Issue:** #34  
**Date:** 2026-08-21

## Purpose

This document turns the project versioning policy into executable Public Contract 0.1 compatibility regression rules.

The governing question is:

```text
old conforming input
       +
current/new implementation or candidate contract behavior
       ↓
same required canonical semantics?
```

If the answer is no within a declared compatibility range, the change is breaking unless an adopted contract rule explicitly permits the difference.

## Compatibility outcomes

The compatibility harness uses the machine-readable outcomes:

```text
compatible
incompatible
unknown
```

These are compatibility-assessment outcomes only. They are **not** `PASS`, `FAIL`, `BLOCKED`, or `INDETERMINATE`, and they do not create a new SOL evaluation lifecycle.

- `compatible` means the old supported input remains interpretable with the same required semantics, allowing differences explicitly permitted by the contract.
- `incompatible` means a previously supported input or meaning would be rejected or would receive materially different required semantics.
- `unknown` means the evidence needed to establish compatibility is absent or insufficient. Unknown never silently becomes compatible.

## Machine-readable case manifest

Executable compatibility cases are declared in:

```text
fixtures/compatibility/public-contract-0.1/manifest.json
```

Each case identifies:

- a stable case ID;
- structural/semantic/evidence category;
- the harness rule used for assessment;
- an old conforming fixture;
- a candidate fixture when evidence exists;
- the expected compatibility outcome.

The Rust integration harness reads this manifest and asserts that the current implementation still produces the declared classification.

## Required semantic projection versus byte equality

Compatibility is not defined as byte-for-byte equality of two JSON documents.

Public Contract 0.1 explicitly permits additive unknown optional members on extensible objects. Such fields are preserved by generic canonical JSON round-tripping, so the canonical bytes may differ while the previously required semantic fields remain unchanged.

For compatibility assessment, the harness therefore compares the adopted known semantic DTO fields while disregarding opaque extension maps only for the additive-extension case. This does **not** redefine generic canonical JSON equality and does not make extension members semantic identity.

## Executable classifications

The Phase 5 manifest covers the following baseline classes.

### Compatible

- additive unknown optional fields on extensible model objects;
- JSON object-key reordering that canonical normalization already declares non-semantic.

### Incompatible structural changes

- removal/rename of a required field;
- incompatible JSON type/cardinality change;
- replacement/removal of a closed enum/state value.

### Incompatible semantic changes

- canonical identity change while the remaining structure stays valid;
- changing `Unsupported -> BLOCKED` evaluation meaning to `Unsupported -> FAIL` while keeping valid enum tokens and the same general shape;
- changing an existing diagnostic code from `error` meaning to `warning` meaning.

The evaluation and diagnostic examples enforce the rule:

```text
same or still-valid syntax + incompatible normative meaning = breaking
```

### Unknown

A compatibility case with missing external compatibility evidence produces `unknown`, not `compatible` and not an evaluation lifecycle state.

## Old-input regression rule

Every non-missing-evidence compatibility case first parses/evaluates its old fixture using the current Public Contract 0.1 implementation. If an old fixture that was declared conforming stops being interpretable, the compatibility test fails before candidate classification can be accepted.

This prevents a future implementation from silently dropping support for an already adopted 0.1 input while updating only the newer fixtures.

## Canonical identity and normalization

Public Contract 0.1 canonical references remain case-sensitive strings. Object-member order is not semantic, while canonical identity spelling is semantic. Consequently:

- reordering JSON object keys remains compatible;
- changing `model.compatibility` to `Model.compatibility` changes canonical identity and is incompatible when the old identity was part of the supported semantic contract;
- a future case-folding or identity-rewriting normalization rule would therefore require an explicit compatibility/version boundary.

## Diagnostic and lifecycle stability

Existing diagnostic-code meaning, diagnostic severity semantics, evaluation comparison meaning, and evaluation lifecycle classification are compatibility-protected.

In particular, Public Contract 0.1 retains:

```text
Exact / Compatible       -> PASS
Degraded / SubjectMismatch -> FAIL
Unsupported              -> BLOCKED
Unknown                  -> INDETERMINATE
```

Compatibility assessment itself is not mapped onto that lifecycle.

## Breaking-change documentation

Every intentional breaking change to a supported surface must use or satisfy the fields in:

```text
docs/templates/breaking-change.md
```

At minimum the documentation must identify:

- `BREAKING CHANGE`;
- affected surface/contract;
- affected version/range;
- old behavior;
- new behavior;
- affected consumers;
- migration path or an explicit no-automatic-migration statement.

The compatibility integration test checks that the repository template retains these required policy fields.

## CI evidence

The compatibility harness is an ordinary `sol-public-contract` Rust integration test and therefore runs in the required workspace test gate. It is deliberately executable without Adapter Protocol or transport assumptions.

Phase 5 does not define protocol negotiation, JSON-RPC, stdio, retry, or adapter execution semantics.

## Breaking-change rule

A change to Public Contract 0.1 is incompatible when a previously conforming participant can no longer be interpreted with the same required normative meaning, even if its serialized shape still parses. An incompatible change must not silently replace Public Contract 0.1 semantics under the same declared compatibility promise.
