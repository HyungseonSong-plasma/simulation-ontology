# SOL v0.1 Canonical Package / Schema Integration — Focused Review v0.2

**Status:** Focused independent Validation  
**Date:** 2026-08-20  
**Input:** `docs/research/sol-v0.1-canonical-package-schema-integration-proposal-v0.2.md`

## Verdict

**Revise — PKG-04 only**

PKG-01 through PKG-03 are closed. The exact RelationDefinition union, ConstraintDefinition wrapper, and durable canonical-ID readback rules are deterministic. One identity/version boundary needs clarification.

## PKG-04 — same identity across package versions may evolve in content

The v0.2 conflict rule states that the same canonical ID bound to two conflicting normalized resource definitions is an identity/content conflict. This is correct **within one immutable resolved environment/snapshot**, but can be misread as forbidding a resource with stable semantic identity from evolving its definition across compatible or breaking package versions.

ADR-0009 explicitly separates canonical identity from package version. Therefore:

- two simultaneously active conflicting definitions for one canonical ID in one resolved environment -> FAIL;
- one canonical ID appearing in historical/prior and new package snapshots with changed definition content -> permitted as versioned semantic/schema evolution, subject to package compatibility/versioning rules;
- prior snapshot is identity continuity evidence, not an immutable-content requirement across all versions.

**Required remediation:** scope `CANONICAL_RESOURCE_IDENTITY_CONTENT_CONFLICT` to one resolved immutable environment/snapshot (or simultaneous active definitions). Clarify that cross-version content evolution may retain canonical identity.

## Accepted closure

- PKG-01: closed RelationDefinition typed/allowed-pair union and projection wrapper.
- PKG-02: closed ConstraintDefinition wrapper around unchanged family payload.
- PKG-03: durable identity source/readback, mint-once persistence, no heuristic rename/content matching.

## Next State

```text
Verdict: Revise
Finding: PKG-04 only
Next State: focused Research clarification -> final Validation
```
