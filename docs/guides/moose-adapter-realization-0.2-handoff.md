# MOOSE Adapter Handoff — SOL Realization 0.2

Status: **M0.8 handoff candidate; effective after Phase 4 merge**

Target repository: `HyungseonSong-plasma/sol-adaptor-moose`

Observed MOOSE Team `main` at handoff preparation:

```text
8c47065aa0f37e299ad1eb7f019d69de1e409ece
```

This SHA is evidence of the adapter-side baseline, not part of the SOL compatibility identity.

## 1. What is unblocked

After M0.8 is merged, the MOOSE Team may begin the first normative mapping layer:

```text
canonical SOL thermal semantics
        ↓
Public Contract 0.2 RealizationSpec
        ↓
Adapter Protocol 0.2 request
        ↓
sol-adaptor-moose
        ↓
adapter-local typed MOOSE IR
        ↓
deterministic MOOSE .i
        ↓
real MOOSE execution/artifact capture
```

The SOL Team defines the left side of this boundary. The MOOSE Team owns the MOOSE-specific realization on the right side.

## 2. Exact contract artifacts to consume

### Public Contract 0.2

Schemas:

```text
schemas/public-contract/0.2/shared.schema.json
schemas/public-contract/0.2/backend-target.schema.json
schemas/public-contract/0.2/mapping-plan.schema.json
schemas/public-contract/0.2/realization-spec.schema.json
```

Reference fixtures:

```text
fixtures/public-contract/0.2/thermal-backend-target.json
fixtures/public-contract/0.2/thermal-mapping-plan.json
fixtures/public-contract/0.2/thermal-realization-spec.json
fixtures/public-contract/0.2/thermal-realization-spec-alternate-values.json
```

Normative description:

```text
docs/contracts/public-contract-0.2-realization-spec.md
```

### Adapter Protocol 0.2

Schemas:

```text
schemas/adapter-protocol/0.2/shared.schema.json
schemas/adapter-protocol/0.2/bootstrap.schema.json
schemas/adapter-protocol/0.2/validate-plan-request.schema.json
schemas/adapter-protocol/0.2/validate-plan-response.schema.json
schemas/adapter-protocol/0.2/execute-plan-request.schema.json
schemas/adapter-protocol/0.2/execute-plan-response.schema.json
```

Reference fixtures:

```text
fixtures/adapter-protocol/0.2/realization-compatible-bootstrap.json
fixtures/adapter-protocol/0.2/thermal-realization-request.json
fixtures/adapter-protocol/0.2/validate-plan-accepted-response.json
fixtures/adapter-protocol/0.2/execute-plan-exact-response.json
```

Normative descriptions:

```text
docs/contracts/adapter-protocol-0.2-realization-requests.md
docs/contracts/adapter-protocol-0.2-conformance.md
```

Conformance profile:

```text
fixtures/adapter-conformance/0.2/realization-profile.json
```

## 3. Required compatibility declaration

The MOOSE adapter should advertise support explicitly, for example:

```json
{
  "bootstrap": {
    "adapter_id": "sol.adapter.moose",
    "adapter_version": "<adapter-package-version>",
    "supported_adapter_protocol_versions": ["0.1", "0.2"],
    "supported_public_contract_versions": ["0.1", "0.2"]
  }
}
```

Supporting Protocol 0.2 without Public Contract 0.2, or vice versa, is not Realization 0.2 compatibility.

## 4. First canonical thermal mapping input

The normative adapter input is the RealizationSpec, not action-ID spelling.

The published thermal fixture gives the adapter explicit solver-independent facts including:

- `ThermalTransport` physics;
- temperature `Field`;
- `SteadyHeatEquation`;
- `FourierLaw` closure;
- thermal conductivity `45 W/(m K)`;
- 1D domain length `1 m`;
- hot-wall position `0 m`;
- Dirichlet temperature `400 K`;
- `StationaryAnalysis`;
- `MaximumTemperature` observation;
- canonical domain and boundary scopes;
- semantic relations;
- explicit MappingPlan action-to-subject/scope bindings.

The MOOSE Team may translate these facts into its adapter-local typed MOOSE IR. It MUST NOT assign canonical meaning to strings such as `thermal.material` or `thermal.solve` beyond their published plan identity/dependency role.

## 5. Required adapter-side checks before Phase 3 canonical mapping

The MOOSE Team should add tests that establish:

1. bootstrap selects Protocol 0.2 + Public Contract 0.2 only when both are declared;
2. the published thermal request parses without adapter-local semantic invention;
3. the alternate-value RealizationSpec produces a different MOOSE IR/input where the changed canonical values require it while the MappingPlan remains unchanged;
4. missing RealizationSpec, broken action binding, hidden PlanAction physics fields, and mixed 0.1/0.2 payloads are rejected;
5. validate and execute responses carry `adapter_protocol_version = "0.2"`;
6. exact MOOSE backend execution remains behind the existing process/workspace isolation boundary;
7. backend-native IDs/paths remain opaque provenance and never become SOL canonical identity.

## 6. Ownership boundary

SOL Team owns:

```text
RealizationSpec meaning
Public Contract schemas/DTOs
Adapter Protocol semantics
conformance profile
```

MOOSE Team owns:

```text
SOL RealizationSpec -> typed MOOSE IR mapping
MOOSE object selection
.i serialization
MOOSE executable invocation
workspace/artifact capture
MOOSE-version compatibility
physical/numerical V&V
```

MOOSE-specific `Variables`, `Kernels`, `Materials`, `BCs`, `Executioner`, object names, file paths, executable paths, and native IDs are not canonical SOL semantics.

## 7. V&V boundary

Passing the SOL Realization 0.2 conformance profile proves contract/protocol behavior only. It does not prove that the generated MOOSE problem is physically or numerically correct.

The MOOSE Team remains responsible for backend-specific checks such as:

- MOOSE input validation / backend probe;
- deterministic `.i` golden/regression tests;
- analytical steady-conduction comparisons where applicable;
- mesh/discretization sensitivity;
- solver convergence;
- physical benchmark validation.

## 8. Recommended first MOOSE Team implementation step

Start with the existing typed MOOSE IR and add one deterministic translator:

```text
RealizationSpec thermal fixture
  -> validate canonical references/action bindings
  -> extract thermal field/equation/closure/material/scope/BC/analysis data
  -> construct typed MOOSE IR
  -> serialize deterministic .i
```

Do not widen the thermal physics scope until this exact published fixture passes adapter-side compatibility, mapping, serialization, and real-backend regression tests.
