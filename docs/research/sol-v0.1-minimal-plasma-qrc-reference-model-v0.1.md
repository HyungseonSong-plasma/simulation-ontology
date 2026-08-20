# SOL v0.1 Minimal Plasma / QRC Reference Model v0.1

**Role:** Research  
**Date:** 2026-08-20  
**Stage:** Design-stage stress test after Thermal PASS  
**Scope:** closed semantic reaction/species graph + QRC + official backend sanity; no production Adapter or backend execution

## 1. Objective

Stress accepted SOL contracts that the Thermal fixture did not exercise strongly:

- qualified relation cardinality over a closed model snapshot;
- subtype-qualified target counting;
- multiple conjunctive QRC obligations on one Relation requirement;
- Interface-targeted reusable Constraints;
- reaction/species semantic identity independent of backend chemistry representation;
- representability classification when one backend has explicit reaction records and another lowers the same semantics into coupled PDE source terms.

## 2. Reference plasma chemistry

Use the deliberately small dissociative attachment topology:

```text
e + Cl2 -> Cl- + Cl
```

The reference fixture does not attempt a complete chlorine plasma discharge. It does not specify cross-section/rate-law numbers, electron energy transport, Poisson coupling, gas flow, wall reactions, or solver settings.

The stress question is only whether a reaction with typed reactants/products and charged/neutral/electron species can be represented deterministically, including qualified product-count obligations.

## 3. Official backend evidence

### MOOSE

MOOSE supports reaction/source terms and coupled-variable residual contributions. Official `CoupledForce` documentation explicitly describes a source term proportional to a coupled variable and notes that, in a species-transport context, the coefficient can be regarded as a reaction-rate coefficient. MOOSE also provides Reaction/ADReaction/ADMatReaction-style residual kernels and general coupled-variable machinery.

Therefore a SOL reaction semantic entity need not map to one native MOOSE reaction object; a backend adapter can lower it into multiple coupled residual/source contributions while preserving the source reaction identity/provenance in MappingPlan/MappingClaim evidence.

Official references:

- https://mooseframework.inl.gov/moose/source/kernels/CoupledForce.html
- https://mooseframework.inl.gov/moose/syntax/Kernels/index.html

### COMSOL Plasma Module

COMSOL Plasma Chemistry explicitly organizes chemistry into species properties and reaction groups, including electron-impact and heavy-species reactions. The Heavy Species Transport documentation supports neutral, ion, and electron species and chemical/electron-impact reactions.

Official references:

- https://doc.comsol.com/6.4/doc/com.comsol.help.plasma/plasma_ug_data.05.12.html
- https://doc.comsol.com/6.4/doc/com.comsol.help.plasma/PlasmaModuleUsersGuide.pdf

### Ansys Chemkin / Fluent

Ansys Chemkin official API documentation includes a sample plasma mechanism with the reaction:

```text
E + CL2 => CL- + CL
```

identified as dissociative attachment. Ansys Fluent also supports species transport and finite-rate volumetric reactions, including charged-species metadata in relevant chemistry workflows.

Official references:

- https://ansyshelp.ansys.com/public/Views/Secured/corp/v252/en/pdf/Ansys_Chemkin_Application_Programming_Interface_Manual.pdf
- https://ansyshelp.ansys.com/public/Views/Secured/corp/v252/en/flu_ug/flu_ug.html

The exact native object/file decomposition differs, but the species/reaction topology is supported across the reference systems at the level needed for this design-stage sanity test.

## 4. Plasma reference extension

The focused normalized package may define:

```text
PlasmaPhysics                  is_a PhysicsModel
Reaction                       is_a Process
DissociativeAttachmentReaction is_a Reaction

ElectronSpecies     is_a Species
NeutralSpecies      is_a Species
NegativeIonSpecies  is_a Species

Cl2Species    is_a NeutralSpecies
ClAtomSpecies is_a NeutralSpecies
ClMinusSpecies is_a NegativeIonSpecies
```

The Electron model instance uses `ElectronSpecies` directly; a separate chlorine-specific electron subtype is unnecessary.

No multiple taxonomic inheritance is introduced.

## 5. Plasma relations

Two solver-independent RelationDefinitions belong to the plasma reference package:

```text
reactants : Reaction -> Species
products  : Reaction -> Species
```

Both use canonical type-or-subtype endpoint matching.

No stoichiometric coefficient is inferred from repeated graph edges. Identical relation triples remain forbidden by ADR-0029 normalized snapshot semantics.

The selected reaction has four distinct model-instance endpoints:

```text
AttachmentReaction --reactants--> Electron_1
AttachmentReaction --reactants--> Cl2_1
AttachmentReaction --products--> ClMinus_1
AttachmentReaction --products--> ClAtom_1
```

The QRC stress case deliberately uses distinct product identities/types rather than attempting to encode a coefficient such as `2 Cl` through duplicate edges.

## 6. Qualified product cardinality contract

Define two normalized reusable Cardinality/QRC ConstraintDefinitions over `products`:

### Negative-ion product obligation

```text
type = cardinality
relation = products
direction = source
qualifier.target_type = NegativeIonSpecies
min = 1
max = 1
```

### Neutral product obligation

```text
type = cardinality
relation = products
direction = source
qualifier.target_type = NeutralSpecies
min = 1
max = 1
```

These constraints are conjunctive but do not collapse into one unqualified `products = 2` rule. Their semantic purpose is to preserve target-type-qualified obligations.

## 7. Interface application

Define:

```text
DissociativeAttachmentProductContract
  relation requirement: products
  ConstraintApplication:
    negative-ion exact-one QRC -> products requirement
  ConstraintApplication:
    neutral exact-one QRC -> products requirement
```

`DissociativeAttachmentReaction` implements the Interface and maps the `products` requirement to the concrete plasma `products` RelationDefinition.

This reuses ADR-0025 target-admissibility and mapping-convergence semantics. No special QRC attachment field is added to the model snapshot.

## 8. Closed plasma model snapshot

Minimal instances:

```text
PlasmaSimulation : Simulation
PlasmaModel : SimulationModel
PlasmaTask : SimulationTask
PlasmaPhysics_1 : PlasmaPhysics
PlasmaMaterialModel : MaterialModel
AttachmentReaction_1 : DissociativeAttachmentReaction
Electron_1 : ElectronSpecies
Cl2_1 : Cl2Species
ClMinus_1 : ClMinusSpecies
ClAtom_1 : ClAtomSpecies
StationaryPlasmaAnalysis : StationaryAnalysis
```

Graph:

```text
PlasmaSimulation --has_model--> PlasmaModel
PlasmaSimulation --has_task--> PlasmaTask
PlasmaTask --uses_model--> PlasmaModel
PlasmaTask --has_analysis--> StationaryPlasmaAnalysis

PlasmaModel --includes_component--> PlasmaPhysics_1
PlasmaModel --includes_component--> PlasmaMaterialModel

PlasmaPhysics_1 --includes_component--> AttachmentReaction_1

PlasmaMaterialModel --includes_component--> Electron_1
PlasmaMaterialModel --includes_component--> Cl2_1
PlasmaMaterialModel --includes_component--> ClMinus_1
PlasmaMaterialModel --includes_component--> ClAtom_1

AttachmentReaction_1 --reactants--> Electron_1
AttachmentReaction_1 --reactants--> Cl2_1
AttachmentReaction_1 --products--> ClMinus_1
AttachmentReaction_1 --products--> ClAtom_1
```

All Entity instances have explicit `properties: []` because this focused fixture does not require rates or numeric state values.

## 9. QRC evaluation contract in the closed snapshot

For each active QRC applied to the concrete `products` requirement:

```text
1. use the current closed ADR-0029 relation set;
2. select distinct `products` target model-instance identities for the source reaction;
3. resolve each target's canonical EntityType;
4. include the target when target type = qualifier.target_type or is a subtype;
5. count each canonical model-instance identity once;
6. compare count to normalized min/max interval;
7. evaluate all active QRC obligations conjunctively.
```

No backend-local species ordering, reaction-file position, alias string, or repeated stoichiometric token affects QRC identity/counting.

## 10. Required adversarial cases

1. baseline `ClMinus_1 + ClAtom_1` products -> both QRCs PASS;
2. remove `ClMinus_1` product edge -> negative-ion QRC FAIL min;
3. add second distinct `ClMinusSpecies` instance as product -> negative-ion QRC FAIL max;
4. add `Cl2_1` as an additional product -> neutral QRC FAIL max;
5. replace `ClMinus_1` type with generic `Species` -> negative-ion QRC FAIL;
6. subtype `ClMinusSpecies <: NegativeIonSpecies` counts correctly;
7. subtype `ClAtomSpecies <: NeutralSpecies` counts correctly;
8. duplicate identical `products` triple -> normalized snapshot failure before QRC counting;
9. relation declaration/order permutation -> same QRC result;
10. snapshot not closed -> structural failure before QRC;
11. unknown qualifier target type -> Constraint/reference failure, not zero count;
12. backend license/runtime absent -> no effect on QRC semantic result.

## 11. Representability judgment

Expected design-stage result:

- COMSOL: explicit species/reaction chemistry supports direct or mildly transformed realization;
- Ansys Chemkin: explicit plasma reaction mechanism supports direct or mildly transformed realization;
- MOOSE: reaction semantics lower into coupled variable/source/reaction residual objects, therefore transformed realization is expected.

At the common SOL level the aggregate judgment is expected to be **transformed**, not `lossy`, provided reaction identity, reactant/product typing, and QRC obligations remain traceable through MappingClaim/provenance.

This fixture does not assert production Adapter support or numeric equivalence.

## 12. Research verdict

**Ready for independent contract/reference-model Validation.**

The Plasma case should be rejected as a design-stage gate only if it exposes an actual semantic/schema contradiction. Backend installation, license, or missing production Adapter implementation remain outside the current stage.
