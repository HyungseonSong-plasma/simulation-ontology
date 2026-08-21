# Adapter Protocol 0.1 — Version, Negotiation, and Adapter Description

**Status:** M0.3 Phase 1 normative baseline  
**Issue:** #40  
**Architecture:** `docs/adr/ADR-002-adapter-protocol-boundary.md`  
**Date:** 2026-08-21

## Purpose

This document defines the transport-independent Adapter Protocol 0.1 bootstrap/description contract. It establishes protocol identity, dual Adapter Protocol/Public Contract compatibility, adapter implementation identity, target/capability declarations, and descriptor-scope capability stability.

It does not define JSON-RPC, request IDs, stdio framing, subprocess lifecycle, retry/backoff/reconnect behavior, or transport errors.

## Independent version axes

Adapter Protocol version is independent from:

- SOL Runtime release;
- Public Contract version;
- ontology package versions;
- adapter implementation version.

Protocol 0.1 uses canonical numeric `MAJOR.MINOR` syntax. `0.1` is valid. `0.1.0`, leading-zero forms, and whitespace-normalized forms are malformed and are not silently rewritten.

Protocol documents use `adapter_protocol_version` where a protocol-version field is required by later operation DTOs. Public Contract `public_contract_version` never substitutes for it.

## Exact supported-version sets

Protocol 0.1 represents support as exact version sets:

```json
{
  "supported_adapter_protocol_versions": ["0.1"],
  "supported_public_contract_versions": ["0.1"]
}
```

Support arrays are canonicalized numerically, sorted, and duplicate-free. Protocol 0.1 intentionally defines no string range grammar such as `>=0.1,<0.2` and no interval-object grammar.

A later protocol version may introduce a more compact representation only if it preserves unambiguous compatibility semantics or establishes an explicit new version boundary.

## Version-safe bootstrap

The AdapterDescription root contains a mandatory `bootstrap` object with the compatibility-discovery core:

```text
bootstrap.adapter_id
bootstrap.adapter_version
bootstrap.supported_adapter_protocol_versions?
bootstrap.supported_public_contract_versions?
```

There is no independent `bootstrap_version`. Bootstrap is part of Adapter Protocol architecture and does not create another version axis.

A caller may parse the bootstrap core without interpreting the rest of the full adapter description. This allows an incompatible peer to be rejected before the caller assumes compatibility with later/full description payload shapes.

Missing support declarations are permitted as representable compatibility evidence and produce `unknown`; malformed declarations are contract errors rather than `unknown`.

Unknown optional bootstrap/description object members are tolerated unless a later normative schema explicitly closes an object.

## Adapter identity and implementation version

`adapter_id` is a stable non-empty namespaced identifier for the adapter implementation family/distribution. It is not a canonical simulation semantic subject.

`adapter_version` is an opaque non-empty implementation release string. Protocol 0.1 does not require SemVer. No Adapter Protocol or Public Contract compatibility is inferred from adapter implementation version.

## Target and capability declarations

The full AdapterDescription carries deterministic target declarations. A target declaration contains:

```text
target
capabilities[]
```

A capability declaration contains:

```text
capability
revision?
```

Target and capability symbols use the same stable symbol vocabulary expected by Public Contract `BackendTargetDto` requirements. The adapter description declares claimed representability evidence; Core retains BackendTarget selection/resolution responsibility.

Capability `revision` is optional opaque adapter-owned provenance identifying a capability vocabulary/revision when required. Backend-native object IDs, vendor API types, mesh selection handles, and other solver-native structures are not canonical protocol semantics.

## Descriptor-scope stability

Capabilities in one parsed AdapterDescription are treated as a stable snapshot for that description instance/scope. A later call to obtain a description may return a different snapshot.

Capability declaration means claimed support/representability. It does not prove physical correctness and does not guarantee current execution availability. Transient environment/backend availability is evaluated by `validate_plan` and authoritatively re-checked by `execute_plan` in later phases.

Protocol 0.1 defines no live capability event/state machine.

## Compatibility assessment

Adapter Protocol and Public Contract axes are assessed independently.

For each axis:

```text
both support sets known + non-empty intersection -> compatible
both support sets known + empty intersection     -> incompatible
required support information missing             -> unknown
```

If multiple common versions exist, the numerically highest common `MAJOR.MINOR` version is selected deterministically.

Overall interoperability is:

```text
both axes compatible -> compatible
any axis incompatible -> incompatible
otherwise -> unknown
```

`compatible`, `incompatible`, and `unknown` are protocol compatibility outcomes. They are not SOL semantic lifecycle values and must not be converted directly to `PASS`, `FAIL`, `BLOCKED`, or `INDETERMINATE`.

Missing compatibility information yields `unknown` and does not authorize execution. Present-but-malformed syntax/type is a bootstrap/request-contract error. Adapter implementation version equality does not change either result.

## Canonicalization

Protocol 0.1 Phase 1 canonicalization rules used by the reference Rust implementation are:

- object key order is non-semantic and canonical emission sorts object keys recursively;
- supported version arrays are explicit set-like exceptions and sort numerically with duplicates removed;
- target declarations sort deterministically by target;
- capability declarations sort deterministically by capability/revision;
- arbitrary unknown extension arrays retain their order;
- strings are not implicitly trimmed, case-folded, or coerced.

## Transport exclusion

AdapterDescription and bootstrap semantic DTOs must not contain normative transport fields such as JSON-RPC envelopes, stdio frames, process IDs, retry policies, reconnect policies, or transport configuration.

The reference implementation rejects known transport markers at this semantic boundary. M0.5 owns transport framing/process mechanics.

## Executable fixtures

Positive fixtures:

- `fixtures/adapter-protocol/0.1/dual-compatible-description.json`
- `fixtures/adapter-protocol/0.1/highest-common-description.json`
- `fixtures/adapter-protocol/0.1/version-safe-bootstrap.json`

Counterexamples cover:

- Protocol incompatible / Public Contract compatible;
- Public Contract incompatible / Protocol compatible;
- missing Protocol support information;
- missing Public Contract support information;
- malformed version syntax such as `0.1.0`;
- adapter implementation version not implying compatibility;
- transport marker leakage.

## Compatibility boundary

Once Adapter Protocol 0.1 is published, the following are compatibility-sensitive normative meanings and must not be silently changed under the same protocol version:

- exact supported-version-set semantics;
- version-safe bootstrap discovery behavior;
- dual-axis compatibility rules and highest-common selection;
- missing versus malformed compatibility information;
- descriptor-scope capability semantics;
- the distinction between adapter implementation version and interoperability compatibility;
- transport exclusion from semantic protocol DTOs.
