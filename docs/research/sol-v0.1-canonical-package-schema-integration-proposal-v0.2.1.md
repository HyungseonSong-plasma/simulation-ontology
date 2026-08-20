# SOL v0.1 Canonical Package / Schema Integration Proposal v0.2.1

**Status:** Focused Research clarification  
**Date:** 2026-08-20  
**Input:** PKG-04  
**Base:** `docs/research/sol-v0.1-canonical-package-schema-integration-proposal-v0.2.md`

## 1. Scope

All v0.2 package/namespace/resource/RelationDefinition/ConstraintDefinition decisions remain unchanged. This clarification only scopes identity/content conflict across package versions.

## 2. PKG-04 — canonical identity continuity versus versioned definition evolution

Canonical semantic identity and package version remain orthogonal under ADR-0009.

### 2.1 Same resolved immutable environment

Within one resolved immutable environment/snapshot:

```text
one canonical ID -> exactly one active normalized resource definition
```

If two simultaneously active resource contributions resolve to the same canonical ID but have different normalized definitions:

```text
FAIL: CANONICAL_RESOURCE_IDENTITY_CONTENT_CONFLICT
```

This includes duplicate conflicting declarations in one package snapshot or conflicting simultaneously active provider content.

### 2.2 Across package versions / historical snapshots

A canonical resource MAY preserve the same canonical ID while its definition evolves across package versions.

Examples include:

- adding backward-compatible metadata/constraints in a later package version;
- changing a definition in a breaking package version while preserving that it denotes the same semantic concept;
- moving the resource to a different file/package layout while retaining identity.

Such evolution is governed by package version/compatibility policy, not by minting a new identity automatically.

Therefore a prior committed package snapshot is authoritative evidence of identity continuity, but its historical resource content is **not** a requirement that all future versions be byte/structurally identical.

### 2.3 Identity change criterion

A new canonical ID is required only when the semantic concept itself is intentionally replaced by a distinct identity, not merely because its schema definition changes with version.

Tooling SHALL NOT infer either continuity or replacement solely from payload similarity/difference. The explicit durable identity binding remains authoritative.

## 3. Revised conflict rule

```text
same canonical ID + conflicting simultaneous definitions
within one resolved immutable environment
    -> FAIL: CANONICAL_RESOURCE_IDENTITY_CONTENT_CONFLICT

same canonical ID across historical/new package snapshots
with changed definition content
    -> identity continuity permitted
    -> package compatibility/version rules decide upgrade compatibility
```

## 4. Focused boundary cases

1. same ID, two different definitions simultaneously active -> FAIL;
2. same ID in v0.1.0 and changed definition in v0.2.0 -> identity may remain same;
3. package/file rename + same durable identity binding -> same identity;
4. different semantic concept explicitly declared -> new ID even if payload resembles prior resource;
5. changed payload with no durable identity binding -> no heuristic continuity inference.

## 5. Finding closure

**PKG-04 proposed closed.**

## 6. Research verdict

**Ready for final focused Validation.**
