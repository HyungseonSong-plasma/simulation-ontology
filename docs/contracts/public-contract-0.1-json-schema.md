# Public Contract 0.1 — JSON Schema and Golden Fixtures

**Status:** M0.2 Phase 4 normative baseline  
**Issue:** #33  
**Date:** 2026-08-21

## Decision

Public Contract 0.1 uses **JSON Schema Draft 2020-12**. The checked-in, language-neutral schema set under `schemas/public-contract/0.1/` is the normative structural representation of the adopted Public Contract DTOs.

The schema layout is versioned by Public Contract version rather than SOL Runtime version, ontology version, Adapter Protocol version, or adapter implementation version.

## Layout

```text
schemas/public-contract/0.1/
  shared.schema.json
  simulation.schema.json
  validation-report.schema.json
  mapping-claims.schema.json
  mapping-plan.schema.json
  backend-target.schema.json
  realization-effect.schema.json
  evaluation-result.schema.json
```

`shared.schema.json` contains reusable definitions. Supported top-level canonical documents each receive their own schema and use relative `$ref` links to shared definitions.

A single monolithic schema is not the Public Contract 0.1 layout. Rust struct definitions or generated schemas may assist implementation, but they are not the normative schema authority.

## `$schema`, `$id`, and references

Every checked-in schema declares:

```json
"$schema": "https://json-schema.org/draft/2020-12/schema"
```

Public Contract 0.1 does not invent a network-hosted canonical `$id` before SOL has a stable schema publication host. Relative `$ref` links are resolved within the versioned schema directory. A later publication process may assign stable external identifiers without silently changing the 0.1 semantic contract.

## Extensibility

The schemas preserve the Phase 1 extensibility rule:

- Public Contract objects remain open to unknown optional members unless a specific adopted DTO is explicitly closed.
- Closed enums remain closed and reject unknown enum tokens.
- Unknown optional members do not become semantic identity merely because schema validation permits and round-tripping preserves them.
- Backend-native semantic identity remains prohibited by the runtime Public Contract rules even though open-object schema validation alone may structurally accept an unknown backend-native field.

The last point is intentional: schema is structural validation, not the entire semantic validator.

## Structural validation versus semantic validity

JSON Schema answers whether a document has an accepted structural shape. It does **not** establish SOL semantic validity.

Examples of rules intentionally enforced outside JSON Schema include:

- relation endpoint resolution;
- duplicate canonical node identity;
- SpatialScope member resolution and spatial kind;
- MappingPlan dependency existence and cycle rejection;
- EvaluationResult comparison/status consistency;
- rejection of backend-native object identity as canonical semantic payload.

Therefore the fixture suite explicitly contains documents that are structurally schema-valid but semantically invalid. This boundary is executable in CI and prevents consumers from treating `schema valid` as equivalent to `SOL valid`.

## Golden fixtures

The Thermal reference contract fixture set is:

```text
fixtures/public-contract/0.1/thermal-simulation.json
fixtures/public-contract/0.1/thermal-validation-report.json
fixtures/public-contract/0.1/thermal-mapping-claims.json
fixtures/public-contract/0.1/thermal-mapping-plan.json
fixtures/public-contract/0.1/thermal-backend-target.json
fixtures/public-contract/0.1/thermal-realization-effect.json
fixtures/public-contract/0.1/thermal-evaluation.json
```

Rust regression tests parse each fixture through its typed DTO and compare canonical JSON emission against the fixture's canonical JSON representation. These tests intentionally do not depend on Rust `Debug` or `Display` formatting.

## Structural negative fixtures

The Phase 4 structural-negative set includes:

- `public-contract-schema-missing-version.json` — missing required Public Contract version;
- `public-contract-malformed-diagnostic.json` — closed diagnostic severity enum violation;
- `public-contract-schema-invalid-evaluation-status.json` — closed evaluation status enum violation.

These must fail their corresponding Draft 2020-12 schemas.

## Schema-valid semantic counterexamples

The CI schema harness additionally requires the following to **pass schema validation** while remaining semantic/runtime counterexamples:

- unresolved semantic relation reference;
- invalid SpatialScope membership;
- duplicate canonical ID;
- MappingPlan cycle;
- inconsistent evaluation false-pass declaration;
- backend-native identity leakage.

This is a normative test of the structural/semantic boundary, not a gap to be closed by making the schema encode graph evaluation or backend policy.

## CI

CI installs a pinned `jsonschema==4.23.0` validator and runs:

```text
python3 scripts/validate_public_contract_schemas.py
```

The harness:

1. checks every checked-in schema against Draft 2020-12;
2. validates every supported Thermal top-level fixture;
3. asserts structural-negative fixtures are rejected;
4. asserts selected semantic counterexamples remain structurally valid.

The existing Rust CI sequence remains authoritative for typed DTO, semantic, canonical-output, Clippy, and architecture tests. Schema validation is additive and does not replace those gates.

## Compatibility rule

Changing the schema dialect, versioned layout, required structural fields, closed-enum values, or schema meaning in a way that breaks existing Public Contract 0.1 consumers is a compatibility-sensitive change and must not be introduced silently under the same contract version.
