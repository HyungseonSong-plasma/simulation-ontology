# SOL v0.1 Canonical Package / Schema Integration — Independent Review v0.1

**Status:** Independent Validation  
**Date:** 2026-08-20  
**Input:** `docs/research/sol-v0.1-canonical-package-schema-integration-proposal-v0.1.md`

## Verdict

**Revise**

The package/namespace/canonical-identity separation is consistent with ADR-0009 and should be retained. Three serialization determinism gaps must be closed before a normalized package schema can be implemented.

## PKG-01 — RelationDefinition shape is not closed

The proposal defines RelationDefinition as `id` plus "accepted relation-semantic metadata that is actually present". This is not a schema contract: independent implementations can choose different field sets and encodings for ordinary typed endpoints, ADR-0016 allowed-pair endpoints, derived relations, and ADR-0017 cardinality projections.

**Required remediation:** define an exact focused normalized RelationDefinition shape. It must distinguish ordinary domain/range endpoint contracts from allowed-pair contracts, define the meaning of intentionally omitted domain/range, and preserve ADR-0017 source-cardinality projection without making it a second authority.

## PKG-02 — ConstraintDefinition wrapper shape is ambiguous

Existing normalized family schemas use closed objects (`additionalProperties: false`). The proposal says a reusable ConstraintDefinition "carries canonical id plus one normalized family payload and application metadata" but does not state whether `id` is injected into the family payload or wraps it.

Injecting `id` would violate existing family schemas and create duplicate authority.

**Required remediation:** define a wrapper, conceptually:

```text
ConstraintDefinition = {
  id,
  payload: <exactly one accepted normalized family payload>,
  interface_application_target_kinds?: [...]
}
```

Metadata must remain outside the closed family payload.

## PKG-03 — durable canonical-ID source/readback is under-specified

The compiler sequence allows "canonical-ID assignment/readback" but source fragments currently do not contain canonical IDs. A clean recompilation could mint different IDs unless a durable identity ledger or previously committed normalized package is explicitly authoritative.

**Required remediation:** canonical ID minting is allowed only for a resource with no prior identity binding. Once minted, the binding `(provider namespace + authoring identity history -> canonical ID)` must be persisted in a durable package identity registry or committed normalized resource and reused on subsequent compilation. Rename/path/package-version changes must reuse the prior ID; ambiguous/multiple prior bindings fail rather than remint.

## Accepted directions retained

Do not reopen:

- source fragments are not canonical identity authority;
- package and semantic namespace are distinct;
- normalized references use canonical IDs;
- namespace export tables map authoring names to canonical IDs/kinds;
- one active provider per namespace in v0.1;
- normalized dependencies use exact resolved package versions;
- package/version/path data do not enter canonical semantic IDs;
- InterfaceImplementation remains keyed by `(entity_type_id, interface_id)` rather than requiring a new canonical resource ID;
- model-instance ValueDefinition instances do not become ontology-package schema resources.

## Next State

```text
Current State: canonical package proposal v0.1
Role Invoked: Validation
Verdict: Revise
Findings: PKG-01..03
Next State: Research revision limited to exact RelationDefinition, ConstraintDefinition wrapper, durable identity readback
```
