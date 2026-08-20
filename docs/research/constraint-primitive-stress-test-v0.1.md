# Constraint Primitive Stress Test v0.1

**Status:** Research/design candidate  
**Date:** 2026-08-20  
**Reference systems:** MOOSE, COMSOL, Ansys  
**Purpose:** Decide whether SOL v0.1 needs five or six primitive constraint families.

## Candidate primitive set

Baseline five:

1. Cardinality
2. Type
3. Value
4. Dimension
5. Compatibility

Candidate sixth:

6. Conditional / Implication

## Evidence

### MOOSE

MOOSE `InputParameters` supports required parameters, optional parameters, range-checked parameters, and runtime inspection such as `isParamValid` and `isParamSetByUser`. Object logic can therefore enforce constraints that only become relevant when another parameter is present or set by the user. This shows that conditional validity is part of real backend semantics even when it is not always represented declaratively in metadata.

### COMSOL

COMSOL Physics Builder explicitly models `Activation Condition`, `Usage Condition`, and conditionally activated allowed values. These conditions can depend on other user inputs, whether another feature is active, Boolean expressions, and value comparisons. This is direct evidence for first-class implication-style validation semantics.

### Ansys

Ansys Mechanical documentation repeatedly describes properties that are displayed, enabled, or valid only when other properties have specific values. Examples include properties that become available only when an export option is selected or when the number of copies is nonzero. This is semantically equivalent to conditional applicability constraints.

## Decision from stress test

The five baseline primitives cannot express the recurring rule shape:

```text
IF predicate P holds
THEN constraint C applies
```

without embedding condition logic ad hoc inside every other primitive.

Therefore SOL v0.1 should use six primitive families:

```text
Constraint
├── Cardinality
├── Type
├── Value
├── Dimension
├── Compatibility
└── Conditional / Implication
```

## Conditional as a meta-constraint

Conditional is different from the other five because it controls whether another constraint applies.

Preferred conceptual form:

```yaml
- type: conditional
  if:
    property: mode
    equals: advanced
  then:
    - type: cardinality
      relation: auxiliary_field
      min: 1
```

This keeps condition logic orthogonal to the constrained subject.

The `then` branch SHOULD contain ordinary constraints rather than duplicate their semantics.

An `else` branch MAY be supported later if required, but is not necessary for the minimal v0.1 contract.

## Candidate rules

### C3 — Six primitive families

SOL v0.1 SHOULD provide Cardinality, Type, Value, Dimension, Compatibility, and Conditional/Implication as the minimum constraint vocabulary.

### C4 — Conditional as composition

Conditional/Implication SHOULD act as a meta-constraint that activates one or more ordinary constraints when its predicate is satisfied.

### C5 — Avoid conditional duplication

Cardinality, Type, Value, Dimension, and Compatibility constraints SHOULD NOT each define independent condition syntaxes when the same semantics can be expressed through the Conditional primitive.

## Remaining questions

1. Minimum predicate vocabulary for `if` expressions.
2. Whether Boolean `and/or/not` belong in v0.1 predicate syntax.
3. Whether `else` is necessary in Core or can be represented as a second implication.
4. How inherited/interface/profile constraints compose when their conditions overlap.
