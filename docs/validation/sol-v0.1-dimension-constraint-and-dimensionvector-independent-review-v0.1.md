# SOL v0.1 Dimension Constraint and DimensionVector Independent Review v0.1

**Role:** Validation  
**Date:** 2026-08-20  
**Branch:** `validation/sol-v0.1-independent-review`

## Validation basis

The review used the proposal artifact, accepted ADR-0004/0005/0007/0018 contracts, and external dimensional-reference evidence. The Research conclusion was not treated as authority.

## Evaluation criteria

- deterministic normalization from authoring to normalized vector;
- explicit DimensionOne semantics;
- backend/unit-system independence;
- no accidental Unit registry ownership;
- independent-validator equality/intersection behavior;
- compatibility with accepted PhysicalDimension/Value boundaries;
- absence of silent architecture expansion.

## Findings

### DIM-V1 — Seven-axis canonical basis

**Accept.** Using the seven SI base dimensions as the v0.1 canonical comparison basis is compatible with ADR-0005's requirement for a machine-comparable exponent representation and does not require SI units for concrete Values. Other coherent unit systems can map to the same dimensional basis.

### DIM-V2 — Authoring omission semantics

**Accept.** The contract is deterministic:

```text
recognized axis omitted -> exponent 0
vector field omitted     -> no Dimension constraint payload
vector: {}               -> explicit DimensionOne
```

Independent validators therefore cannot infer DimensionOne merely from missing Unit metadata.

### DIM-V3 — Normalized representation

**Accept.** Requiring all seven axes in normalized form removes sparse-map comparison ambiguity. Object member order is correctly non-semantic.

### DIM-V4 — Integer exponent scope

**Accept as an explicit v0.1 scope boundary.** The accepted reference cases and current BIPM/QUDT-style engineering dimension vectors are representable with signed integer exponents. Fractional/rational exponent support is explicitly deferred rather than silently approximated with floating point.

This is a language-version limitation, not a backend limitation. A later need for exact rational exponents would require a new normalization contract.

### DIM-V5 — Dimension Constraint / PhysicalDimension identity boundary

**Accept.** The proposal defines the normalized machine-comparable validation payload without deciding whether graph-level PhysicalDimension is ultimately serialized as an Entity or typed structure. Any graph-level identifier must resolve to one canonical vector before composition.

No competing semantic authority is introduced in this slice.

### DIM-V6 — Intersection semantics

**Accept.** Equality returns the same normalized DimensionVector and inequality produces an empty Dimension-axis intersection. Conflict classification remains delegated to ADR-0007 activation context rather than declaration order.

### DIM-V7 — Unit/metrology boundary

**Accept.** Unit identifiers, conversion factors, offsets, symbols, and registries are excluded from the Dimension payload. Backend or Profile inability to serialize unit metadata cannot weaken upstream dimensional validity.

### DIM-V8 — External/runtime dependencies

**Not a blocker.** No MOOSE, COMSOL, Ansys, or QUDT runtime/installation is required for DimensionVector equality. External vocabularies remain optional integration evidence.

## Defect classification

- Architecture defect: none found.
- Profile/Adapter defect: none in this scope.
- Backend limitation: not relevant to semantic DimensionVector comparison.
- Reference-model defect: none found in the current integer-exponent Thermal/Plasma scope.
- Validation-tooling defect: none blocking contract promotion.

## Verdict

**Accept.**

The proposal is ready for a focused ADR and machine-readable authoring/normalized Dimension schema slice.

## Required implementation smoke cases

At minimum:

1. sparse authoring normalizes omitted axes to zero;
2. `{}` normalizes to seven zeros;
3. missing `vector` is structurally invalid;
4. unknown axis is rejected;
5. non-integer exponent is rejected in v0.1;
6. thermal-conductivity vector round-trips to the normalized form;
7. equal normalized vectors intersect successfully;
8. unequal vectors produce deterministic Dimension conflict;
9. backend/unit metadata absence does not alter the normalized semantic vector.
