# Adapter Protocol 0.2 — Realization Conformance Profile

Status: **Normative M0.8 conformance profile**

This profile completes the Public Contract 0.2 / Adapter Protocol 0.2 realization boundary introduced to resolve #110. It is additive. Public Contract 0.1 and Adapter Protocol 0.1 remain frozen in meaning and continue to be tested by their existing publication/conformance gates.

## 1. Version pair

Realization 0.2 conformance requires both axes:

```text
Adapter Protocol = 0.2
Public Contract  = 0.2
```

Compatibility on only one axis is insufficient. Mixed 0.1/0.2 request or response payloads are invalid and MUST NOT be upgraded, downgraded, or coerced implicitly.

## 2. Required request information

Both `validate_plan` and `execute_plan` carry:

```text
BackendTarget 0.2
MappingPlan 0.2
RealizationSpec 0.2
```

`RealizationSpec` is required. It MUST resolve canonically and MUST cover the `MappingPlan` actions exactly through action bindings before adapter-specific realization begins.

`MappingPlan` remains planning/dependency structure. Action-ID spelling and opaque PlanAction extensions are not realization semantics.

## 3. Response semantics

Protocol 0.2 response envelopes carry `adapter_protocol_version = "0.2"`.

The established Protocol 0.1 operation invariants are intentionally inherited rather than redefined:

- preflight consistency;
- action-report coverage;
- dependency-safe execution scheduling;
- terminal execution-state consistency;
- aggregate realization-effect equality;
- failure/state separation;
- opaque provenance separation;
- no durable validation authority;
- no execute replay authority after response loss.

Protocol 0.2 additionally checks opaque execution references against the canonical semantic references made explicit by `RealizationSpec` (entity IDs, scope IDs/members, source model, semantic parameter/unit references, relation endpoints, and bound subjects). Opaque backend provenance MUST NOT become canonical SOL identity.

## 4. Required executable cases

The language-neutral case manifest is:

```text
fixtures/adapter-conformance/0.2/realization-profile.json
```

A conformant implementation MUST satisfy all listed cases:

1. `realization-v02.required-realization-spec`
2. `realization-v02.distinguishability`
3. `realization-v02.hidden-semantics-rejected`
4. `realization-v02.referential-integrity`
5. `realization-v02.dual-axis-compatibility`
6. `realization-v02.mixed-version-rejected`
7. `realization-v02.validate-roundtrip`
8. `realization-v02.execute-roundtrip`
9. `realization-v02.no-physical-correctness-claim`

The repository executes these rules in `sol-adapter-conformance-fixtures` CI.

## 5. Distinguishability criterion

The published thermal positive and alternate-value fixtures intentionally use the same MappingPlan while differing in canonical realization values.

Conformance requires:

```text
canonical MappingPlan(A) == canonical MappingPlan(B)
canonical RealizationSpec(A) != canonical RealizationSpec(B)
```

This is the executable counterexample that prevents a real adapter from deriving physical realization meaning from action IDs alone.

## 6. Backend validation boundary

A conformance result establishes only that an adapter obeys the published SOL contract/protocol semantics for the tested profile.

It does **not** establish:

- correct MOOSE Kernel/Material/BC selection;
- correct discretization;
- numerical convergence;
- physical accuracy;
- verification or validation against analytical, experimental, or benchmark data.

Those are backend-specific physical/numerical V&V responsibilities. The conformance report therefore retains:

```text
BackendValidationScope::NotAssessedByConformance
```

## 7. Published artifacts

Public Contract 0.2:

```text
schemas/public-contract/0.2/
fixtures/public-contract/0.2/
docs/contracts/public-contract-0.2-realization-spec.md
```

Adapter Protocol 0.2:

```text
schemas/adapter-protocol/0.2/
fixtures/adapter-protocol/0.2/
docs/contracts/adapter-protocol-0.2-realization-requests.md
```

Conformance:

```text
fixtures/adapter-conformance/0.2/realization-profile.json
crates/sol-adapter-conformance-fixtures/tests/realization_v02_profile.rs
```

## 8. Compatibility claim

An external adapter may claim Realization 0.2 compatibility only after it can:

- advertise both `0.2` compatibility axes through bootstrap;
- parse the published Public Contract 0.2 thermal RealizationSpec;
- reject the specified negative/mixed-version cases;
- produce Protocol 0.2 validate/execute responses satisfying the published operation invariants.

Package/repository version numbers do not imply this compatibility claim.
