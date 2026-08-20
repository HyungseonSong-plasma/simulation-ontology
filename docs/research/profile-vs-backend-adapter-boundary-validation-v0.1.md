# Profile vs BackendAdapter Boundary Validation v0.1

**Status:** Research/design validation  
**Date:** 2026-08-20  
**Reference systems:** MOOSE, COMSOL, Ansys  
**Purpose:** Validate whether SOL should separate declarative backend mapping policy (`Profile`) from runtime backend execution (`BackendAdapter`).

## Candidate boundary

```text
Profile
= declarative mapping specification

BackendAdapter
= runtime backend integration/execution layer
```

The candidate assumes Profiles describe what a semantic construct maps to and under which conditions, while BackendAdapters know how to interrogate a concrete backend, create/update backend artifacts, evaluate backend-native capabilities, and report realized identities/results.

## Evidence

### MOOSE

MOOSE input files are declarative configurations composed of blocks such as Mesh, Variables, Kernels, BCs, Executioner, and Outputs. Individual objects are created by a `type` and parameter bindings, and semantic concepts such as a diffusion field may require multiple backend objects (for example Variable + Kernel + BC + Executioner context).

This supports separating:

- a declarative mapping rule saying which MOOSE object structure realizes an SOL semantic contract;
- runtime/generator logic that renders legal MOOSE syntax, selects object names, inspects available syntax/parameters, and writes the `.i` artifact.

### COMSOL

COMSOL exposes an imperative Java API around a model object graph. Physics interfaces, PhysicsFeatures, Materials, Selections, and StudyFeatures are created and mutated through API calls and tags. Selections and study activation are runtime model operations rather than static semantic definitions.

This supports a Profile that describes which COMSOL features/tags/properties should realize an SOL construct, while the BackendAdapter performs actual API calls, allocates tags, creates feature trees, applies selections, and handles API-version differences.

### Ansys

Ansys Mechanical exposes a runtime DataModel/Automation API. The Model object owns analyses, materials, named selections, mesh, and other children, and scripting methods create concrete analyses such as steady-state thermal analysis. Mechanical properties may be quantities, enumerations, selections, or references, and assignments are performed against runtime Automation objects.

This supports keeping semantic mapping policy out of the imperative adapter code. The Profile describes the intended mapping/scoping/property semantics, while the BackendAdapter interacts with `ExtAPI`/DataModel, creates objects, assigns `Location`/selection values, sets quantities, and observes backend availability.

## Validation result

The separation is supported by all three backends.

A backend-neutral Profile cannot itself perform artifact creation because:

1. backend APIs use different object creation/update mechanisms;
2. backend-local identifiers/tags/names must be allocated at runtime;
3. capability availability can depend on installed modules, application build, release, and runtime model state;
4. one SOL semantic object may realize as one or many backend objects;
5. creation order and backend lifecycle requirements are runtime concerns.

Conversely, placing mapping policy only in adapter code would hide semantic mapping decisions inside imperative implementation and make them difficult to inspect, compare, version, validate, and reuse.

## Proposed responsibility split

### Profile responsibilities

A Profile SHOULD own declarative, inspectable mapping policy:

```text
Profile
├── identity / version
├── required SOL/domain/backend ontology packages
├── target backend adapter identifier
├── backend release compatibility declaration
├── MappingRules
├── semantic/profile refinements
├── generation defaults/policy
└── expected backend capabilities
```

A MappingRule MAY contain:

```text
source semantic contract
applicability predicate
required capabilities
backend realization descriptor
property/relation/value bindings
representability expectations
mapping diagnostics metadata
```

The Profile SHALL NOT directly call backend APIs or own runtime backend handles.

### BackendAdapter responsibilities

A BackendAdapter SHOULD own executable backend integration:

```text
BackendAdapter
├── inspect backend/runtime version
├── discover capabilities
├── validate backend-native identifiers/references
├── allocate backend-local identities
├── create/update/delete backend artifacts
├── apply selections/scoping
├── render/serialize backend syntax where applicable
├── execute backend-native API operations
├── map runtime diagnostics/errors
└── report realized mapping/artifact identities
```

The BackendAdapter SHALL NOT silently redefine the semantic mapping contract declared by the Profile.

## Mapping execution model

Recommended flow:

```text
SOL Model
   ↓ semantic validation
Profile
   ↓ select/apply MappingRules
Mapping Plan
   ↓ capability + representability preflight
BackendAdapter
   ↓ execute realization
Backend Artifacts
   ↓
Realization Report
```

The intermediate `Mapping Plan` is useful because it makes transformation decisions inspectable before backend mutation.

Conceptually:

```text
MappingPlan
├── selected rules
├── source SOL identities
├── planned backend realization descriptors
├── expected representability class
├── required capabilities
└── unresolved/defaulted choices
```

## Representability ownership

Representability classification is shared but responsibilities differ:

- **Profile** declares the expected semantic mapping and any known limitations.
- **BackendAdapter** verifies current runtime capability and actual realizability.
- **Mapping engine/runtime** combines both into the final classification.

Final classes remain:

```text
exact
transformed
lossy
unsupported
```

`transformed` is semantically valid when structure changes but meaning is preserved.

A valid SOL model MUST NOT be weakened because the adapter cannot realize it. Unsupported or lossy mappings are reported as backend representability results in accordance with ADR-0006 and ADR-0007.

## Capability boundary

Capabilities SHOULD be declarative identifiers/contracts that Profiles may require, while discovery is an adapter operation.

Example:

```text
Profile rule requires:
  backend capability = temperature_dependent_material_property

BackendAdapter:
  discoverCapabilities() -> set of supported capability IDs
```

Capability vocabulary may be backend-specific unless/until cross-backend evidence justifies a Core semantic capability.

## Backend examples

### MOOSE

```text
Profile rule:
TemperatureField
  -> Variable + required heat-conduction realization

BackendAdapter:
- choose MOOSE block/object names
- render `[Variables]`, `[Kernels]`, etc.
- validate available object types/parameters
- emit `.i` syntax
```

### COMSOL

```text
Profile rule:
TemperatureField / HeatEquation
  -> Heat Transfer physics realization

BackendAdapter:
- allocate physics/feature tags
- call COMSOL model API
- create/select domains or boundaries
- set feature properties
- create/activate study steps
```

### Ansys

```text
Profile rule:
SteadyThermalStudy
  -> Steady-State Thermal analysis realization

BackendAdapter:
- call Mechanical Automation/DataModel API
- create analysis and child objects
- assign material/property quantities
- create/use Named Selections or Geometry selections
```

## Candidate normative rules

### PB1 — Declarative Profile

A Profile SHALL be a declarative, versioned mapping specification. It SHALL NOT require direct backend API execution semantics to be interpreted as an ontology artifact.

### PB2 — Executable BackendAdapter

A BackendAdapter SHALL own backend-specific runtime inspection, artifact realization, serialization/API invocation, backend-local identity allocation, and runtime diagnostics.

### PB3 — No hidden semantic policy

BackendAdapters SHALL NOT silently redefine Profile semantic mapping rules. Adapter-specific implementation choices MAY vary only within the semantic freedom left by the selected Profile rule.

### PB4 — Mapping Plan boundary

Profile evaluation SHOULD produce an inspectable Mapping Plan before mutating or emitting backend artifacts.

### PB5 — Shared representability evaluation

Final representability classification SHALL combine declarative Profile expectations with runtime BackendAdapter capability checks.

### PB6 — Capability discovery

Profiles MAY require named backend capabilities; BackendAdapters are responsible for discovering whether those capabilities are available in the active backend/runtime.

## Consequences

### Positive

- mapping semantics remain inspectable and testable without launching a backend;
- adapter code stays focused on backend integration rather than ontology policy;
- profiles can be compared across MOOSE, COMSOL, and Ansys;
- backend releases/API changes can often be contained in adapters;
- mapping plans enable dry-run validation and explainable diagnostics;
- semantic mapping can be reused across multiple adapter implementations or execution environments.

### Costs

- a mapping engine/runtime layer is needed between Profile and BackendAdapter;
- mapping descriptors need a backend-extensible schema;
- profile and adapter versions/capabilities must be checked together;
- runtime realization reports must preserve source-to-backend traceability.

## Decision recommendation

Adopt the boundary:

```text
Profile = declarative semantic mapping policy
BackendAdapter = executable backend integration
MappingPlan = inspectable bridge between them
```

This separation is strongly supported by MOOSE declarative input generation, COMSOL imperative model APIs, and Ansys Mechanical Automation/DataModel scripting.
