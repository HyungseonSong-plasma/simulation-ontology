# SOL v0.1 InlineValueDefinition Payload Boundary Proposal v0.2

**Status:** Focused Research revision  
**Date:** 2026-08-20  
**Input:** VDI-02  
**Base:** `docs/research/sol-v0.1-inline-valuedefinition-payload-boundary-proposal-v0.1.md`

## 1. Preserved decisions

Unchanged:

- literal is Core-native `Value`;
- expression/function/tabular use explicit stable `format` identity;
- format provider owns payload syntax/evaluation meaning;
- Core does not scan raw payload for dependency-like strings;
- inline definitions have no graph identity or Relation endpoints;
- backend-local handles are not canonical Core format identities.

## 2. VDI-02 — format-provider normalization contract

Every nonliteral format contract that can participate in normalized SOL authoring SHALL provide a deterministic normalization operation conceptually equivalent to:

```text
normalize_inline_payload(
    authored_payload,
    semantic_resolution_context
)
  -> {
       normalized_local_payload,
       resolved_semantic_dependencies[]
     }
```

### Result rules

`normalized_local_payload`
- is the format-owned payload emitted after format-specific structural/semantic normalization;
- contains mechanism-local data;
- is not scanned by Core to infer graph dependencies.

`resolved_semantic_dependencies[]`
- is an order-independent collection of canonical resolved SOL model/schema semantic identities referenced by the definition;
- contains only references the provider has semantically classified as SOL dependencies using the supplied resolution context;
- is deduplicated by canonical resolved identity;
- carries provider/normalization provenance outside the Core semantic payload when diagnostics require it.

## 3. Inline/reified selection

After provider normalization:

```text
resolved_semantic_dependencies is empty
    -> InlineValueDefinition remains eligible

resolved_semantic_dependencies is nonempty
    -> InlineValueDefinition SHALL NOT be emitted
    -> reified ValueDefinition is required
    -> every dependency becomes/preserves a semantic Relation under ADR-0003/0026
```

Other ADR-0026 reification triggers remain conjunctively applicable, including reuse, provenance/lifecycle, independent Constraint/mapping, and explicit identified declaration.

## 4. Resolution failure

When provider normalization is required:

- missing/invalid format identity -> `FAIL`;
- format identity valid but required provider unavailable -> `INDETERMINATE`;
- provider cannot uniquely resolve a semantic reference -> `INDETERMINATE` if evidence is incomplete, or `FAIL` if the resolved input is definitively ambiguous/invalid under ADR-0009;
- provider returns malformed normalization result -> `FAIL` as format-contract/tooling violation.

Core SHALL NOT replace an unavailable provider with raw string scanning.

## 5. Determinism boundary

For a fixed authored payload, resolved package environment, format provider version/identity, and semantic resolution context, provider normalization SHALL return the same normalized local payload and canonical dependency set independent of declaration/import iteration order.

A format/provider revision that changes semantic dependency extraction creates a new normalization evidence revision and SHALL NOT silently mutate an already validated immutable resolved model snapshot.

## 6. Normalized Core envelope

If the dependency set is empty and no other reification trigger applies:

```yaml
mechanism: expression | function | tabular
format: <canonical format identity>
payload: <normalized_local_payload>
```

The dependency set is not serialized inside this inline Core payload because a nonempty set would have forced reification. Normalization evidence may retain the empty-set/provider provenance externally.

Literal remains:

```yaml
mechanism: literal
value: <normalized Value>
```

## 7. Focused boundary cases

1. provider returns empty dependency set -> inline eligible;
2. provider returns one canonical dependency -> reified required;
3. duplicate dependency aliases resolving to one canonical identity -> one dependency;
4. unresolved provider -> INDETERMINATE, no raw string fallback;
5. definite ambiguous SOL name under provider resolution -> FAIL;
6. declaration-order permutation -> identical normalized payload/dependency set;
7. same payload under changed provider semantic revision -> new normalization evidence revision, no silent snapshot mutation.

## 8. Finding closure

| Finding | Proposed status | Resolution |
|---|---|---|
| VDI-02 provider semantic-reference extraction | Resolved | deterministic provider normalization returns normalized local payload + canonical dependency set; nonempty set forces reification |

## 9. Research verdict

**Ready for final focused Validation.**
