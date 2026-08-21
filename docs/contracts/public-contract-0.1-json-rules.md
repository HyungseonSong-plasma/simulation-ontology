# SOL Public Contract 0.1 — Version Envelope and Canonical JSON Rules

**Status:** M0.2 Phase 1 normative baseline  
**Issue:** #30  
**Date:** 2026-08-21

## Purpose

This document defines the Public Contract 0.1 version envelope and generic canonical JSON rules. It is language-neutral and intentionally independent from Rust crate layout, SOL Runtime release numbers, Adapter Protocol versions, and ontology package versions.

Exact DTO shapes are defined in later M0.2 phases. These rules apply to every canonical top-level Public Contract 0.1 DTO unless a later adopted schema gives a more specific rule.

## 1. Version envelope

Every top-level canonical Public Contract document MUST contain:

```json
{
  "public_contract_version": "0.1"
}
```

Nested DTOs inherit the contract version of their top-level document and do not repeat this field unless a later contract explicitly requires it.

The Public Contract version is independent from:

- SOL Runtime release version;
- Adapter Protocol version;
- ontology package version such as `ontology_version`;
- adapter implementation version.

`ontology_version` MUST NOT be interpreted as a substitute for `public_contract_version`.

## 2. Version syntax and support

Public Contract version syntax is canonical `MAJOR.MINOR` decimal form.

For Public Contract 0.1:

- `0.1` is supported;
- a missing version is a distinct error;
- a non-string version is a distinct error;
- malformed syntax is a distinct error;
- a syntactically valid but unsupported version is a distinct error;
- `0.1.0` MUST NOT silently normalize to `0.1`;
- leading-zero components such as `00.1` or `0.01` are malformed;
- whitespace or other lexical rewriting is not performed.

No generic `{ "kind": ..., "payload": ... }` wrapper is introduced in 0.1 solely for versioning.

## 3. Required and optional fields

A field is required only when its adopted DTO/schema declares it required.

For optional fields:

- absence and explicit `null` are distinct unless the field is explicitly nullable and its DTO semantics define them as equivalent;
- defaults MUST NOT be inferred by generic JSON normalization;
- producers MUST NOT rely on an optional extension being understood unless the relevant compatibility/capability contract establishes support.

## 4. Extensible objects and unknown optional fields

Public Contract objects are extensible by default unless an adopted schema explicitly declares them closed.

For a declared extensible object:

- consumers SHOULD tolerate unknown optional fields;
- generic Public Contract parsing/normalization MUST preserve unknown fields opaquely;
- an unknown field cannot redefine or override the normative meaning of a known 0.1 field;
- unknown fields do not become semantic identity merely because they are preserved.

A later DTO/schema may explicitly close an object when strict rejection is required.

## 5. Canonical JSON normalization

Generic Public Contract normalization applies the following rules:

1. JSON object member order is semantically irrelevant.
2. Canonical emission sorts object keys recursively by their UTF-8 string ordering as represented by the implementation's ordinary string comparison.
3. Array order is preserved by default and therefore remains significant unless a specific DTO explicitly declares set-like semantics.
4. Generic normalization does not case-fold, trim, rename, or otherwise rewrite string values.
5. Generic normalization does not coerce JSON types. A string value is not converted to a number, boolean, or null, and vice versa.
6. Generic normalization does not introduce field defaults.
7. An absent optional field remains distinct from an explicit `null` value unless a later DTO gives a more specific rule.

The generic normalizer deliberately does not declare arbitrary arrays to be sets. A DTO that owns set semantics must define its own deterministic ordering rule and compatibility tests.

Numeric domain-specific equivalence is likewise not invented by the generic normalizer. If a later DTO requires unit conversion, tolerance, or other numeric semantic equivalence, that rule belongs to the DTO/ontology contract rather than generic JSON normalization.

## 6. Equality and golden output

Public Contract generic structural comparison may ignore object key order after normalization, but it MUST preserve array order and JSON value types unless a specific DTO defines stronger canonical semantics.

Golden fixtures that depend on canonical JSON emission therefore use recursively sorted object keys while preserving array positions and scalar types.

Changing a normalization rule in a way that changes normative identity/equality for an existing supported document is a semantic breaking change under the project versioning policy.

## 7. Enum and state evolution

Enums/states are closed by default in Public Contract 0.1.

Therefore:

- unknown values of a closed enum are rejected;
- a value is extensible only when the adopted DTO/schema explicitly declares that enum open/extensible;
- an unknown enum token MUST NOT be silently mapped to another existing semantic state;
- in particular, an unknown token MUST NOT automatically become `Unknown`, `BLOCKED`, or `INDETERMINATE` merely to preserve parsing.

Adding a new value to a closed enum may therefore require a new compatible contract rule or explicit version boundary depending on consumer guarantees.

## 8. Executable Phase 1 evidence

The `sol-public-contract` crate implements only the generic version/envelope and canonicalization behavior defined here. It deliberately has no dependency on internal `sol-core-*`, target-resolver, MockAdapter, or CLI crates.

Phase 1 fixtures cover:

- supported `0.1` document parsing;
- missing version;
- malformed `0.1.0` version;
- unsupported `0.2` version;
- unknown optional-field preservation;
- object-key-order equivalence;
- default array-order significance;
- JSON-type non-coercion;
- absence versus explicit null;
- closed-enum rejection.

## 9. Non-goals

Phase 1 does not define:

- model/validation/diagnostic DTO shapes;
- mapping/plan/target/evaluation DTO shapes;
- Adapter Protocol operations or negotiation;
- JSON-RPC or stdio transport framing;
- TypeScript/Python SDK ergonomics;
- backend-native payloads.

Those remain assigned to later milestones/phases.
