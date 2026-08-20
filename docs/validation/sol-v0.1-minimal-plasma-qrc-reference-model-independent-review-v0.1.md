# SOL v0.1 Minimal Plasma / QRC Reference Model — Independent Review v0.1

**Role:** Validation  
**Date:** 2026-08-20  
**Input:** `docs/research/sol-v0.1-minimal-plasma-qrc-reference-model-v0.1.md`  
**Normative basis:** ADR-0007/0008/0009/0012/0016/0024/0025/0028/0029

## Verdict

**ACCEPT at contract/reference-design level.**

No new Architecture defect or schema-contract gap is exposed by the proposed reaction/QRC structure.

## QRC contract review

The proposed product obligations are proper qualified cardinalities rather than a flattened unqualified count:

```text
products target_type NegativeIonSpecies -> exactly 1
products target_type NeutralSpecies     -> exactly 1
```

The two obligations compose conjunctively and preserve distinct target-type meaning.

The closed ADR-0029 snapshot provides the closure evidence required by ADR-0012. Target counting uses distinct model-instance identity and canonical EntityType/subtype resolution. Duplicate identical relation triples are rejected before QRC evaluation rather than treated as stoichiometric multiplicity.

## Interface application review

Applying the two QRC ConstraintDefinitions through an Interface relation requirement is consistent with ADR-0025:

```text
Interface
  relation requirement: products
  -> QRC application 1
  -> QRC application 2

DissociativeAttachmentReaction
  implements Interface
  maps requirement products -> concrete products
```

No special model-instance QRC attachment syntax is needed.

The chosen reference mapping uses the same `products` RelationDefinition as requirement and concrete relation, avoiding any unresolved subrelation/rewrite question in this fixture.

## Taxonomy / allowed-pair review

The proposed extension respects single-parent Entity inheritance:

```text
DissociativeAttachmentReaction <: Reaction <: Process
Cl2Species <: NeutralSpecies <: Species
ClAtomSpecies <: NeutralSpecies <: Species
ClMinusSpecies <: NegativeIonSpecies <: Species
ElectronSpecies <: Species
```

This makes the existing ADR-0016 memberships valid through subtype closure:

```text
PhysicsModel --includes_component--> Process subtype
MaterialModel --includes_component--> Species subtype
```

No artificial multiple inheritance is required.

## Official backend sanity review

### MOOSE

Official MOOSE Kernel documentation supports coupled source/reaction residuals and explicitly describes coupled-force coefficients as reaction-rate-like in species transport. The same SOL reaction may therefore lower into several Kernel/source contributions rather than one native chemistry record.

Design-stage judgment: **transformed**.

### COMSOL Plasma Module

Official Plasma Chemistry and Heavy Species Transport documentation supports species definitions including neutral/ion/electron categories and reaction groups including electron-impact/heavy-species chemistry.

Design-stage judgment: **exact or transformed depending on adapter decomposition; no demonstrated semantic loss**.

### Ansys

The official Chemkin API manual includes the plasma reaction:

```text
E + CL2 => CL- + CL
```

as dissociative attachment. Fluent also supports finite-rate reaction/species transport structures.

Design-stage judgment: **exact or transformed depending on selected Ansys product/adapter; no demonstrated semantic loss**.

The cross-backend aggregate remains **transformed** because MOOSE is expected to realize the reaction through lower-level residual/source objects.

## Required implementation counterexamples

Implementation Validation must include at least:

1. baseline one negative-ion + one neutral product -> PASS;
2. missing negative-ion -> FAIL;
3. second distinct negative-ion -> FAIL;
4. second neutral product -> FAIL;
5. generic Species replacing negative-ion subtype -> FAIL;
6. subtype targets count correctly;
7. duplicate relation triple fails before QRC;
8. unknown qualifier target type fails as unresolved reference, not zero count;
9. order permutation preserves result;
10. non-closed snapshot fails before QRC;
11. backend/runtime/license state is absent from the evaluation input.

## Decision readiness

The proposal is ready for a minimal implementation slice:

- normalized `@simulation-ontology/plasma-reference` package;
- closed Plasma model snapshot reusing the committed Core fixture;
- focused QRC snapshot semantic helper/tests;
- independent implementation readback.

No new ADR is required unless implementation exposes a contract ambiguity. The fixture exercises already accepted ADR-0012/0025/0029 semantics rather than introducing a new language construct.
