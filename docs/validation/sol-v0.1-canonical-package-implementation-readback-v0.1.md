# SOL v0.1 Canonical Package Implementation — Independent Readback v0.1

**Status:** Independent implementation Validation  
**Date:** 2026-08-20  
**Inputs:** ADR-0028; five package/resource schemas; `tests/package_semantics.py`; package structural/semantic regression artifacts

## Verdict

**Revise — implementation/tooling defects only**

No new architecture or package-contract defect was found. Three implementation gaps remain before the ADR-0028 slice can be accepted.

## PKI-01 — resolved dependency declarations are not checked against the resolved environment

`OntologyPackageNormalized.resolved_dependencies` structurally carries exact `{package,version}` pairs, but `validate_resolved_environment()` never verifies that each declared pair corresponds to exactly one active package in the supplied resolved environment.

Counterexample:

```text
Package A declares resolved dependency B@0.1.0
Resolved environment contains only A
Current helper -> may PASS
ADR-0028 resolved environment -> dependency is not resolved
```

**Required remediation:** build an active package identity index, reject duplicate active `(name,version)` package identities, and require every declared resolved dependency to match exactly one active package identity. A declared package with a different active version is not a match.

Suggested diagnostics:

```text
PACKAGE_IDENTITY_DUPLICATE_ACTIVE
RESOLVED_DEPENDENCY_UNRESOLVED
```

**Classification:** Validation-tooling defect.

## PKI-02 — normalized allowed-pair contracts can retain duplicate sources

The accepted package contract requires repeated allowed-pair source entries to normalize by target union/deduplication before the immutable normalized snapshot is published. The JSON Schema permits repeated `source` values across `pairs`, and the current helper only validates reference kinds.

Thus two semantically equivalent but structurally different normalized snapshots can remain:

```yaml
pairs:
  - source: A
    targets: [B]
  - source: A
    targets: [C]
```

versus:

```yaml
pairs:
  - source: A
    targets: [B, C]
```

**Required remediation:** because this helper validates an already-normalized package, reject retained duplicate canonical sources with a stable diagnostic such as:

```text
RELATION_ALLOWED_PAIRS_NOT_NORMALIZED
```

Target duplicates are already structurally prevented by `uniqueItems`.

**Classification:** Normalization/tooling defect.

## PKI-03 — normalized package SemVer regex is weaker than SemVer syntax

ADR-0009 requires SOL-controlled package versions to use Semantic Versioning syntax. The current schema regex accepts invalid SemVer forms such as numeric prerelease identifiers with leading zeros and malformed dotted prerelease/build identifier sequences.

Examples that must fail include:

```text
1.0.0-01
1.0.0-alpha..1
```

**Required remediation:** replace the simplified regex with a strict SemVer 2.0.0-compatible pattern for normalized package and resolved dependency versions, and add regression cases for leading-zero and empty identifiers.

**Classification:** Structural-schema tooling defect.

## Accepted implementation readback

No defect was found in:

- closed EntityTypeDefinition and PropertyDefinition slices;
- typed versus allowed-pairs RelationDefinition union;
- intentionally open typed domain/range semantics;
- external ConstraintDefinition wrapper around unchanged family payloads;
- source-cardinality projection authority/reference concept;
- package collection envelope and exact-version rather than range representation;
- namespace export alias-to-same-ID semantics;
- active namespace-provider ambiguity detection;
- resource-kind and Interface reference checks;
- same-ID cross-version evolution being validated as separate historical snapshots rather than simultaneous definitions.

The package helper is a package-integration validator and does not replace the already accepted family-specific Constraint validators or ADR-0025 Interface conformance validator; those remain separate validation passes in the composed pipeline.

Repository-root execution is currently unavailable because the execution environment cannot resolve `github.com`. This remains `RESOURCE_INTERRUPTED`, not a domain verdict.

## State transition

```text
Current State: ADR-0028 implementation readback
Role Invoked: Validation
Verdict: Revise
Findings: PKI-01..03 tooling only
Next State: focused implementation remediation -> focused Validation
```
