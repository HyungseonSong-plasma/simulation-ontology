# Value and ValueDefinition Identity Study v0.1

**Status:** Research evidence  
**Date:** 2026-08-20  
**Reference systems:** MOOSE, COMSOL, Ansys Mechanical, Palantir Ontology  
**Purpose:** Stress-test whether evaluated Value and ValueDefinition should be graph Entities or typed data/definition structures.

## 1. Research questions

1. Should an evaluated `Value` be a semantic Entity or typed data by default?
2. Should every `ValueDefinition` be a semantic Entity?
3. Is the evaluation mechanism (`Literal`, `Expression`, `Function`, `Tabular`) sufficient to determine whether a definition is reified?

## 2. Value result

Across the reviewed systems, concrete evaluated values are naturally represented as typed data rather than independent semantic objects. Examples include scalar quantities, vector/tensor data, and backend magnitude/quantity representations.

This supports the candidate rule:

> **V5 — Value as Typed Data:** An evaluated `Value` SHOULD be represented as typed data rather than as a semantic Entity by default. Reification is justified only when a particular value instance requires independent semantic identity or graph relations.

Conceptually:

```text
Value {
  data
  shape
  unit?
}
```

## 3. ValueDefinition stress test

### 3.1 MOOSE

MOOSE demonstrates that even a simple evaluation mechanism may be a named reusable object. The Functions system contains `ConstantFunction`, parsed functions, piecewise functions, interpolation functions, and other Function objects. A `PiecewiseLinear` Function is a named MOOSE object and may obtain its tabulated data from inline arrays or an external CSV file.

Therefore `Function` or `Tabular` does not automatically imply either inline data or reified semantic identity.

### 3.2 COMSOL

COMSOL material properties can contain direct expressions, while Analytic, Interpolation, and Piecewise functions are explicit function nodes. Analytic functions have names, arguments, expressions, argument units, and output units and can be reused by material properties. Interpolation functions may use a local table, file, result table, or another function as their data source.

This demonstrates a real distinction between a local property expression and a reusable function node, but the distinction is not equivalent to the mathematical evaluation mechanism alone.

### 3.3 Ansys Mechanical

Mechanical `DataRepresentation` distinguishes constant, table, function, and tabular-data magnitude representations. A constant may be returned as a Quantity, a function as an expression string, and tabular representations as table objects. This provides evidence that lightweight definitions are practical and that evaluation mechanism and object identity are separate concerns.

### 3.4 Palantir Ontology

Palantir provides an architectural analogy: property values and Value Types carry typed/semantic data without becoming Object instances, while reusable Functions/Actions are separate computational artifacts. The useful design lesson is that semantic typing does not imply independent object identity.

## 4. Main finding

The initial candidate split

```text
Literal / Expression / Tabular -> inline
Function / Model -> Entity
```

is too rigid.

Cross-system evidence shows that **evaluation mechanism and reification are orthogonal**.

A literal can be wrapped by a named reusable backend function, while a function-like expression can be represented inline. Therefore SOL should not decide graph identity solely from `Literal`, `Expression`, `Function`, or `Tabular` classification.

## 5. Revised candidate rules

> **VD1 — Definition Form:** A `ValueDefinition` MAY be represented as a lightweight typed definition when its evaluation semantics are local to the owning semantic concept and no independent identity is required.

> **VD2 — Reification by Semantic Need:** A `ValueDefinition` SHOULD be reified as an Entity when it requires independent identity, reuse, provenance, lifecycle, relations, independent constraints, or backend mapping.

> **VD3 — Reification Orthogonality:** The decision to reify a `ValueDefinition` SHALL be independent of its evaluation mechanism. `Literal`, `Expression`, `Function`, `Tabular`, and related mechanisms describe evaluation form; they do not by themselves determine graph identity.

## 6. Examples

```text
Local constant
TemperatureBC
  value_definition:
    mechanism: literal
    value: 300 K

Local expression
Source
  value_definition:
    mechanism: expression
    expression: 2*pi*f
    depends_on: f

Reusable model
ThermalConductivity
  has_value_definition -> ConductivityModel_01

ConductivityModel_01   [reified Entity]
  mechanism: function
  depends_on -> Temperature
  provenance -> DataSource
  valid_range -> ...
```

A backend may still implement a local literal through a named ConstantFunction or a reusable model through an expression. Backend object structure does not dictate SOL Core identity.

## 7. Architectural implication

The preferred model is:

```text
SemanticEntity
   |
   +-- local value_definition --> typed ValueDefinition
   |
   +-- has_value_definition ---> reified ValueDefinition Entity

ValueDefinition
   +-- mechanism: literal | expression | function | tabular | ...
   +-- dependencies
   +-- output contract

Value
   +-- typed evaluated datum
```

The distinction between inline and reified definitions is therefore an identity/lifecycle decision, not a taxonomy decision.

## 8. Relationship to existing ADRs

- ADR 0002 remains valid: Value and ValueDefinition are distinct; value shape and evaluation mechanism are orthogonal.
- ADR 0003 remains valid: references/dependencies are Relations rather than a generic ReferenceDefinition.
- This study adds a second orthogonality: **ValueDefinition evaluation mechanism is independent of ValueDefinition graph identity/reification.**

## 9. Remaining decision

Before acceptance into SOL v0.1, decide whether the language should expose:

1. one `ValueDefinition` typed structure with optional identity/reification;
2. separate `InlineValueDefinition` and `ValueDefinitionEntity` constructs; or
3. a general SOL reification rule applicable to any typed construct, not only ValueDefinition.

The third option may provide the most general architecture but requires validation against the existing Entity/Property/Relation boundary rules.
