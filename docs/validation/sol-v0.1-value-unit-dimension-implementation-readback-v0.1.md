# SOL v0.1 Value / Unit / Dimension Implementation Readback v0.1

**Status:** Independent implementation Validation  
**Date:** 2026-08-20  
**Inputs:** ADR-0026, ADR-0027, focused schemas, Core registry transcription, semantic helper/tests

## Verdict

**Revise — tooling defect only**

The structural schemas and graph-boundary transcription are faithful to the accepted contract. One helper default can bypass required provider evidence.

## VUI-01 — nonliteral provider evidence defaults to PASS

`choose_value_definition_representation()` currently declares:

```python
provider_state: str = "PASS"
```

For `expression | function | tabular`, ADR-0027 requires format-provider normalization before inline eligibility can be established. A caller omitting provider evidence can currently obtain `("INLINE", None)`.

### Required remediation

For nonliteral mechanisms, absence of provider evidence SHALL NOT imply PASS. The helper must either:

- require explicit provider state/result; or
- treat omitted provider evidence as `INDETERMINATE / VALUE_DEFINITION_FORMAT_PROVIDER_UNRESOLVED`.

Literal behavior remains unchanged.

## Accepted readback

No defect found in:

- shared DimensionVector schema;
- UnitReference structural identity;
- Value scalar/vector/tensor closed union;
- UnitReference restricted to numeric Value branches;
- tensor product delegated to semantic helper;
- InlineValueDefinition literal/nonliteral structural branch;
- no inline graph identity/dependency fields;
- shared exact-decimal extraction and reuse;
- `ValueDefinition` Entity transcription only for reified definitions;
- `has_value_definition` / `depends_on` graph-boundary intent.

Repository-root execution remains operationally unavailable because the execution container cannot resolve `github.com`; this is `RESOURCE_INTERRUPTED`, not a domain verdict.

## Next State

```text
Verdict: Revise
Finding: VUI-01 tooling only
Next State: focused helper/test remediation -> readback Validation
```
