# Adapter Protocol 0.2 — Realization Request Contract

Status: **Normative M0.8 request boundary**

This document defines the Adapter Protocol 0.2 request delta introduced to resolve #110. It is additive: Adapter Protocol 0.1 remains frozen and its existing DTOs, schemas, lifecycle meaning, and default version remain unchanged.

## 1. Version boundary

The already-published default remains:

- `ADAPTER_PROTOCOL_VERSION = "0.1"`
- `AdapterProtocolVersion::current() = 0.1`

The realization boundary is selected explicitly:

- `ADAPTER_PROTOCOL_VERSION_0_2 = "0.2"`
- `AdapterProtocolVersion::realization_v02() = 0.2`

A 0.1 request parser MUST NOT silently accept a 0.2 realization request, and a 0.2 request parser MUST reject any other Adapter Protocol version.

## 2. Required Public Contract pair

Adapter Protocol 0.2 realization requests require Public Contract 0.2 on all canonical payloads:

```text
Adapter Protocol 0.2
  target           -> Public Contract 0.2 BackendTarget
  plan             -> Public Contract 0.2 MappingPlan
  realization_spec -> Public Contract 0.2 RealizationSpec
```

`target`, `plan`, and `realization_spec` MUST carry the same `public_contract_version`, and that version MUST be `0.2`.

Mixed 0.1/0.2 canonical payloads are invalid requests. There is no implicit upgrade, downgrade, or cross-version coercion.

## 3. `validate_plan` request

The Protocol 0.2 request is:

```text
ValidatePlanRequestV02 {
  adapter_protocol_version,
  target,
  plan,
  realization_spec,
  extensions...
}
```

`realization_spec` is required. Validation of the request MUST establish Public Contract validity and `RealizationSpec.validate_against_plan(plan)` before adapter-specific validation begins.

`validate_plan` remains **advisory and side-effect free**. The request does not carry an acceptance token, lease, durable validation authority, replay authority, or execution authorization.

## 4. `execute_plan` request

The Protocol 0.2 request is:

```text
ExecutePlanRequestV02 {
  adapter_protocol_version,
  target,
  plan,
  realization_spec,
  extensions...
}
```

The full canonical target, MappingPlan, and RealizationSpec are resent at execution time. The adapter MUST re-evaluate current state and the current canonical payloads before producing backend side effects.

`execute_plan` remains **authoritative at execution time**. A prior accepted `validate_plan` response does not authorize execution and cannot substitute for any of the canonical request payloads.

A lost execute response does not authorize replay. Protocol 0.2 introduces no retry, replay, lease, or deduplication authority.

## 5. MappingPlan / RealizationSpec separation

Protocol 0.2 preserves the Public Contract 0.2 distinction:

```text
MappingPlan
  = action identity + dependency structure

RealizationSpec
  = solver-independent realization meaning
```

Adapters MUST NOT derive physical or realization meaning from action-ID spelling. Opaque PlanAction extensions MUST NOT be interpreted as a substitute for RealizationSpec.

## 6. Compatibility

Adapter Protocol compatibility and Public Contract compatibility remain independent axes. To consume this realization request boundary, both selected versions MUST be `0.2`.

`CompatibilitySupport::realization_v02()` expresses the exact core-side pair:

```text
adapter_protocol_versions = ["0.2"]
public_contract_versions  = ["0.2"]
```

An implementation version number does not imply semantic compatibility.

## 7. Backend-neutrality

The request is canonical SOL data. MOOSE, COMSOL, ANSYS, or other backend-native objects, IDs, kernels, boundary-condition objects, executable handles, process identifiers, or local workspace paths are not canonical realization identity and MUST NOT enter this request as semantic data.

Opaque operational evidence may be returned only through separately defined provenance/evidence surfaces where permitted; it must never become canonical subject identity.

## 8. Published Phase 3 schema surface

Adapter Protocol 0.2 request schemas are published under:

```text
schemas/adapter-protocol/0.2/
  shared.schema.json
  validate-plan-request.schema.json
  execute-plan-request.schema.json
```

Both request schemas reference, rather than duplicate, the Public Contract 0.2 target, plan, and RealizationSpec schemas.

The positive thermal request fixture is:

```text
fixtures/adapter-protocol/0.2/thermal-realization-request.json
```

The same canonical payload is valid as the semantic params object for either `validate_plan` or `execute_plan`; operation identity remains the Adapter Protocol method, not a field hidden in the canonical request.

## 9. Response/conformance publication boundary

Phase 3 publishes the 0.2 **request and compatibility boundary** required to unblock adapter realization mapping. It does not authorize treating a Protocol 0.1 response with a rewritten version string as a Protocol 0.2 response.

Full end-to-end Protocol 0.2 conformance, including the executable response/conformance profile and MOOSE handoff evidence, is the M0.8 Phase 4 gate. An adapter MUST NOT claim full Protocol 0.2 conformance solely from Phase 3 request parsing.
