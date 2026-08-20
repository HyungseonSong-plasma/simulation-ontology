# SOL v0.1 Value / Unit / PhysicalDimension Graph Transcription Proposal v0.2

**Status:** Focused Research revision  
**Date:** 2026-08-20  
**Revision input:** VUD-01 through VUD-03 from independent Validation v0.1  
**Base:** `docs/research/sol-v0.1-value-unit-dimension-graph-transcription-proposal-v0.1.md`

## 1. Scope preservation

The following v0.1 decisions remain unchanged and are not reopened:

- `PhysicalDimension` is represented canonically by ADR-0020 `DimensionVector`, not a mandatory Core Entity;
- `Unit` is represented by external/metrology `UnitReference`, not a Core unit catalogue;
- `Value` is typed evaluated data, not a Core Entity by default;
- `requires_dimension`, `has_dimension`, `expressed_in`, and `evaluates_to` are not blindly serialized as Core Entity-to-Entity Relations;
- missing Unit does not imply DimensionOne, SI, backend default, or validity;
- affine/quantity-specific semantics belong to the semantic quantity/use-site contract plus metrology service;
- `ReferenceDefinition` remains excluded;
- inline versus reified ValueDefinition remains independent of evaluation mechanism;
- backend installation/license/runtime state does not participate in semantic metrology validity.

This revision changes only unresolved metrology-state semantics, dependency-resolution boundary, and reified ValueDefinition identity consistency.

## 2. VUD-01 — semantic metrology validation states

For a numeric physical Value whose use site requires dimensional/unit validation:

### PASS

`PASS` requires enough resolved semantic evidence to establish that:

- the expected Dimension Constraint is known;
- any explicit UnitReference required for validation is resolved, or a permitted contextual unit policy is resolved;
- resolved unit dimension equals the expected DimensionVector;
- any quantity-specific Compatibility criterion that is applicable is satisfied.

### FAIL

Definite semantic contradiction yields `FAIL`, for example:

- resolved UnitReference dimension differs from required DimensionVector;
- resolved quantity-specific unit Compatibility criterion fails;
- the use-site contract explicitly forbids unit omission and the Value omits unit information.

Stable diagnostic examples:

```text
VALUE_UNIT_DIMENSION_MISMATCH
VALUE_UNIT_COMPATIBILITY_FAIL
VALUE_UNIT_REQUIRED_MISSING
```

### INDETERMINATE

Missing evidence required to decide semantic validity yields `INDETERMINATE`, for example:

- explicit UnitReference cannot currently be resolved by the required metrology semantic service;
- unit is omitted and the relevant contextual unit policy is unresolved;
- quantity-specific semantic conversion context required for affine validation is unresolved.

Stable diagnostics:

```text
VALUE_UNIT_REFERENCE_UNRESOLVED
VALUE_CONTEXTUAL_UNIT_POLICY_UNRESOLVED
VALUE_AFFINE_SEMANTIC_CONTEXT_UNRESOLVED
```

These states participate in ADR-0024 common aggregation:

```text
FAIL > INDETERMINATE > PASS
```

Backend installation, solver runtime, backend license, and backend release availability are not evidence for these diagnostics and do not alter this semantic verdict.

## 3. VUD-02 — authoring/compiler boundary for semantic dependencies

A normalized inline ValueDefinition SHALL NOT contain canonical semantic dependency references to independently identifiable SOL Entities.

The normalized compiler boundary is:

```text
Human authoring mechanism payload
        |
        | parse / resolve names where language syntax requires
        v
Compiler-resolved definition
        |
        +-- local/opaque mechanism variables only
        |       -> inline representation may remain eligible
        |
        +-- one or more canonical SOL semantic dependency refs
                -> reified ValueDefinition required
                -> dependencies become semantic Relations
```

### Normative rule

A semantic validator SHALL NOT infer SOL graph dependencies merely by scanning raw expression/function/tabular strings for identifier-like tokens.

Only references that the authoring/compiler/name-resolution phase has resolved as canonical SOL semantic identities count as semantic dependencies for the inline-versus-reified decision.

Therefore:

```text
expression: "2*pi*f"
```

may remain inline if `f` is a local mechanism argument with no canonical SOL binding.

If `f` resolves to canonical model/schema semantic concept `model-A:frequency-1` (or equivalent resolved identity), the compiler SHALL NOT emit normalized inline form. It SHALL emit/refer to a reified ValueDefinition and preserve the dependency as a Relation.

### Normalized inline contract

Focused normalized `InlineValueDefinition` contains mechanism-local content only and has no `semantic_dependencies`, `depends_on`, canonical SOL dependency IDs, or generated graph node identity field.

If an authoring form exposes explicit dependency declarations, normalization resolves them before selecting inline/reified representation.

Opaque local expression variables and function arguments are mechanism-local syntax unless explicitly/canonically bound to SOL semantic identities by the compiler.

## 4. VUD-03 — canonical model-instance identity for reified ValueDefinition

Every reified ValueDefinition is one model-graph Entity instance and SHALL have exactly one canonical model-instance identity under ADR-0009's schema/model/backend identity separation.

Conceptually:

```text
ValueDefinitionInstanceId = canonical model-instance identity
```

All graph statements referring to that definition use the same resolved identity:

```text
OwnerA --has_value_definition--> VD_1
OwnerB --has_value_definition--> VD_1
VD_1  --depends_on-----------> Frequency_1
VD_1  --depends_on-----------> Temperature_1
```

The same definition SHALL NOT be duplicated into anonymous/generated graph nodes merely because multiple owners reference it.

### Graph consistency rules

1. `has_value_definition` target SHALL resolve to exactly one reified ValueDefinition model instance.
2. `depends_on` source SHALL be that same resolved ValueDefinition instance.
3. A reified ValueDefinition may have multiple owners and multiple dependencies.
4. Inline ValueDefinitions have no graph identity and SHALL NOT be a source or target of semantic Relation edges.
5. Backend-local function/property/object names SHALL NOT replace the SOL model-instance identity.
6. Two distinct reified ValueDefinition model-instance IDs remain distinct even if their mechanism payloads are structurally equal; payload equality does not merge model-instance identity.
7. One model-instance ID SHALL NOT resolve to two conflicting ValueDefinition payloads in the same immutable resolved model snapshot; that is an identity/content consistency failure.

Suggested diagnostics:

```text
VALUE_DEFINITION_REFERENCE_UNRESOLVED
VALUE_DEFINITION_REFERENCE_AMBIGUOUS
VALUE_DEFINITION_RELATION_ENDPOINT_INVALID
VALUE_DEFINITION_IDENTITY_CONTENT_CONFLICT
```

## 5. Reification decision after canonical resolution

The focused decision procedure is:

```text
Input: authored ValueDefinition at a value-bearing use site

1. parse/normalize mechanism-local syntax;
2. resolve any explicit semantic references using ADR-0009 resolution;
3. determine whether independent graph identity is required by any of:
   - resolved semantic dependency Relation;
   - reuse by multiple owners;
   - independent provenance/lifecycle;
   - independent Constraint;
   - independent backend mapping;
   - explicit identified declaration;
4. if none apply -> InlineValueDefinition is permitted;
5. if any apply -> reified ValueDefinition model instance is required;
6. all owner/dependency Relations use that reified model-instance identity.
```

This procedure does not infer identity need from `literal | expression | function | tabular` mechanism alone.

## 6. Validation sequence integration

For normalized package/model validation:

```text
1. canonical identity/name resolution
2. ValueDefinition inline/reified representation consistency
3. ValueDefinition graph endpoint consistency
4. Value structural validation
5. Dimension Constraint resolution
6. UnitReference/contextual-unit semantic resolution
7. dimensional comparison
8. quantity-specific Compatibility validation
9. aggregate PASS/FAIL/INDETERMINATE under ADR-0024
```

No backend runtime or license check occurs in this sequence.

## 7. Focused boundary cases

### VUD-01

1. explicit UnitReference resolves and dimension matches -> PASS;
2. resolved dimension mismatch -> FAIL `VALUE_UNIT_DIMENSION_MISMATCH`;
3. explicit UnitReference metrology resolution unavailable -> INDETERMINATE `VALUE_UNIT_REFERENCE_UNRESOLVED`;
4. unit absent + contextual policy unresolved -> INDETERMINATE;
5. unit required explicitly but absent -> FAIL;
6. backend license absent while semantic metrology evidence complete -> unchanged semantic verdict.

### VUD-02

7. inline expression uses local argument `f` with no canonical SOL binding -> inline permitted;
8. same token `f` canonically resolves to SOL Entity -> inline output forbidden / reification required;
9. raw string scanning alone does not create a dependency Relation;
10. explicit authoring dependency resolves before inline/reified selection.

### VUD-03

11. two owners reference same reified VD identity -> valid shared definition;
12. dependency edge originates from the same resolved VD identity -> valid;
13. inline definition used as Relation source/target -> invalid;
14. two equal payloads with distinct model-instance IDs -> remain distinct;
15. one model-instance ID with conflicting payloads -> identity/content conflict;
16. backend-local function name differs while SOL VD identity remains same -> semantic identity preserved.

## 8. Finding closure matrix

| Finding | Proposed status | Resolution |
|---|---|---|
| VUD-01 unresolved metrology state | Resolved | explicit PASS/FAIL/INDETERMINATE semantics + diagnostics + ADR-0024 aggregation |
| VUD-02 inline semantic-dependency detection | Resolved | compiler-resolved canonical semantic refs only; no raw string dependency inference |
| VUD-03 reified VD identity/link consistency | Resolved | one canonical model-instance identity shared by all owner/dependency graph statements |

## 9. Research verdict

**Ready for focused independent Validation.**
