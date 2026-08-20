# SOL v0.1 InlineValueDefinition Payload Boundary — Focused Review v0.1

**Status:** Independent focused Validation  
**Date:** 2026-08-20  
**Input:** `docs/research/sol-v0.1-inline-valuedefinition-payload-boundary-proposal-v0.1.md`

## Verdict

**Revise**

The explicit `format` ownership boundary is preferable to inventing a universal Core expression/function/tabular DSL and avoids treating an unqualified opaque payload as Core semantic truth. One determinism gap remains.

## VDI-02 — format provider must expose semantic-reference normalization

The proposal requires the authoring/compiler phase to promote any payload binding that resolves to a SOL semantic Entity into a reified ValueDefinition. However, Core deliberately does not understand format-specific payload syntax.

Without a provider contract, two implementations can differ:

- compiler A treats a format-specific token as local and emits inline form;
- compiler B understands the same token as a SOL semantic binding and emits reified form.

### Required remediation

A conforming nonliteral format provider SHALL expose a deterministic normalization boundary conceptually equivalent to:

```text
normalize_inline_payload(authored_payload, resolution_context)
  -> normalized_local_payload
     + resolved_semantic_dependencies[]
```

The returned semantic dependencies use canonical resolved SOL identities. If the list is nonempty, Core normalization SHALL choose the reified ValueDefinition form and preserve those dependencies as Relations. If empty, inline form remains eligible.

The Core validator still does not scan payload text itself.

Provider unavailability when this normalization is required yields semantic/normalization `INDETERMINATE`, not a guessed dependency result.

## Accepted directions retained

- literal remains Core-native `Value`;
- nonliteral mechanisms use explicit stable `format` identity;
- format provider owns payload syntax/evaluation semantics;
- raw payload scanning by Core is forbidden;
- backend-local handles are not canonical Core format identities;
- provider unavailability does not justify inventing semantics.

## Next State

```text
Current State: InlineValueDefinition payload proposal v0.1
Role Invoked: Validation
Verdict: Revise
Finding: VDI-02 only
Next State: Research focused provider normalization/reference-extraction contract
```
