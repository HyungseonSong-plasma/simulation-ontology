# Plasma Predicate Vocabulary Stress Test v0.1

**Status:** Research/design candidate  
**Date:** 2026-08-20  
**Domain:** Low-temperature plasma simulation  
**Reference systems:** COMSOL Plasma Module; MOOSE ecosystem (Zapdos/CRANE as plasma-oriented applications); SOL predicate candidate model

## 1. Purpose

Stress-test the SOL v0.1 predicate vocabulary in a plasma-specific domain before freezing the Constraint/Predicate architecture.

Candidate predicate vocabulary:

```text
Predicate
├── Compare        (=, !=, <, <=, >, >=)
├── Membership     (one_of)
├── Exists         (path/relation existence)
└── Boolean        (and, or, not)
```

The question is whether common plasma-model validity conditions require additional primitive predicates.

## 2. External evidence

### COMSOL Plasma Module

COMSOL plasma models expose several condition-dependent structures relevant to SOL predicates:

- the electron mean-energy equation is solved for drift-diffusion electron transport, but under local-field approximation (LFA) the mean-energy equation may not be solved and mean energy can instead be supplied from electric-field/Boltzmann data;
- EEDF inputs become available only under a combination of settings such as Global diffusion model, Local field approximation, and Boltzmann two-term EEDF;
- Electron Impact Reaction and ordinary Reaction features have different electron-energy accounting semantics;
- reaction/collision types such as excitation, attachment, and ionization require different supporting data;
- species electroneutrality initialization is enabled for selected ionic species;
- surface reactions target explicit wall/boundary selections.

These patterns require comparisons, membership tests, existence checks, and boolean composition but do not intrinsically require a new primitive predicate.

### MOOSE ecosystem

MOOSE lists Zapdos as a low-temperature plasma simulation application and CRANE as a plasma-chemistry/thermochemistry application. MOOSE input semantics generally expose named parameters, references, and application-level validity rules that can be mapped into the same predicate model. Backend C++ implementation logic is not copied into SOL; SOL captures only the semantic conditions.

## 3. Plasma-domain test cases

### P1 — Electron energy model selection

Semantic rule:

```text
IF electron_transport_model = local_field
THEN electron_mean_energy_equation is not required
```

SOL-style predicate:

```yaml
if:
  path: electron_transport_model
  equals: local_field
then:
  - type: cardinality
    relation: solved_by
    target: ElectronMeanEnergyEquation
    max: 0
```

Required predicate primitive: **Compare**.

### P2 — Boltzmann EEDF activation

Representative semantic condition:

```text
IF diffusion_model = global
AND electron_energy_model = local_field
AND eedf_model is one of {boltzmann_linear, boltzmann_quadratic}
THEN reduced_electric_field input must exist
```

SOL-style predicate:

```yaml
if:
  and:
    - path: diffusion_model
      equals: global
    - path: electron_energy_model
      equals: local_field
    - path: eedf_model
      one_of: [boltzmann_linear, boltzmann_quadratic]
then:
  - type: cardinality
    property: reduced_electric_field
    min: 1
```

Required primitives: **Compare + Membership + Boolean**.

### P3 — Ionization reaction requirements

Semantic rule:

```text
IF collision_type = ionization
THEN energy_loss must exist
AND product species must contain an ionized product
```

First part:

```yaml
if:
  path: collision_type
  equals: ionization
then:
  - type: cardinality
    property: energy_loss
    min: 1
```

The existence of a required product relation can be represented with `Exists` or relation cardinality.

Required primitives: **Compare + Exists**.

### P4 — Attachment semantics

Semantic rule:

```text
IF collision_type = attachment
THEN a negative-ion product must exist
```

The charge/category test itself is a Type/Compatibility constraint on the product species. The condition is still a Compare predicate.

```yaml
if:
  path: collision_type
  equals: attachment
then:
  - type: compatibility
    left: products[*]
    right: NegativeIonSpecies
```

No new predicate primitive required.

### P5 — Electroneutrality initialization

Semantic rule:

```text
IF initial_value_from_electroneutrality = true
THEN selected_species must be an IonSpecies
```

```yaml
if:
  path: initial_value_from_electroneutrality
  equals: true
then:
  - type: type
    path: species
    required_type: IonSpecies
```

Required primitive: **Compare**.

### P6 — Surface-reaction targeting

Semantic rule:

```text
IF reaction_scope = surface
THEN defined_on relation must exist
AND target must be Boundary/Wall-like Scope
```

```yaml
if:
  path: reaction_scope
  equals: surface
then:
  - type: cardinality
    relation: defined_on
    min: 1
  - type: type
    relation: defined_on
    required_type: BoundaryScope
```

Required primitive: **Compare**; validation itself uses Cardinality + Type constraints.

### P7 — Electron-impact reaction energy accounting

Semantic rule:

```text
IF reaction_kind = electron_impact
AND collision_type is one of {excitation, ionization, elastic}
THEN electron_energy_exchange semantics must be defined
```

```yaml
if:
  and:
    - path: reaction_kind
      equals: electron_impact
    - path: collision_type
      one_of: [excitation, ionization, elastic]
then:
  - type: cardinality
    property: electron_energy_exchange
    min: 1
```

Required primitives: **Compare + Membership + Boolean**.

### P8 — Optional species-dependent chemistry

Semantic rule:

```text
IF species O2 exists in chemistry
AND model_family = oxygen_plasma
THEN at least one oxygen-related reaction set must exist
```

```yaml
if:
  and:
    - exists:
        path: chemistry.species[O2]
    - path: model_family
      equals: oxygen_plasma
then:
  - type: cardinality
    relation: includes_reaction_set
    min: 1
```

Required primitives: **Exists + Compare + Boolean**.

## 4. Result

All tested plasma-domain conditions can be represented using the four candidate predicate primitives:

```text
Compare
Membership
Exists
Boolean(and/or/not)
```

No plasma-specific predicate primitive was required.

The more domain-specific semantics belong in the **Constraint**, **SemanticQuantity**, **Species**, **Reaction**, or **Relation** vocabularies rather than in Predicate itself.

For example:

```text
"product must be a negative ion"
```

is not a new predicate primitive. It is a Type/Compatibility constraint activated by a Compare predicate.

Similarly:

```text
"surface reaction must target a wall"
```

is not a special `surface_predicate`; it is a Cardinality + Type constraint conditioned by reaction scope.

## 5. Architectural implication

The predicate layer remains domain-neutral:

```text
Predicate
    ↓ determines applicability
Conditional Constraint
    ↓ validates
Plasma semantic model
```

This is consistent with SOL's orthogonal-axis principle: domain semantics should not be encoded into a generic logical predicate hierarchy.

## 6. Candidate rule

### PRED1 — Minimal Composable Predicate Vocabulary

SOL v0.1 SHOULD define Predicate using four primitive families only:

1. `Compare`
2. `Membership`
3. `Exists`
4. `Boolean` composition (`and`, `or`, `not`)

Domain-specific validation semantics SHALL be expressed by constraints activated by those predicates rather than by adding domain-specific predicate types.

## 7. Deferred cases

The following are not yet required by the tested plasma cases and remain deferred:

- collection quantifiers such as `all`, `any`, `none`;
- numerical tolerance/approximately-equal predicates;
- regex/string predicates;
- temporal predicates;
- graph-path transitive predicates;
- aggregate predicates such as count/sum/average in condition expressions.

These SHOULD be added only if future reference models demonstrate a concrete requirement.

## 8. Related documents

- [Constraint Primitive Stress Test](./constraint-primitive-stress-test-v0.1.md)
- [Relation Semantics Study](./relation-semantics-study-v0.1.md)
- [Interface Capability Contract Study](./interface-capability-contract-study-v0.1.md)
- [Entity Boundary and Orthogonal Axes Study](./entity-boundary-and-orthogonal-axes-study-v0.1.md)
