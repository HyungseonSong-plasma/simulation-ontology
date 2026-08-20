# Reference vs ValueDefinition Study v0.1

**Status:** Research evidence  
**Date:** 2026-08-20  
**Reference backends:** MOOSE, COMSOL, Ansys Mechanical  
**Purpose:** Test whether `Reference` should be modeled as a `ValueDefinition`, or whether references should remain semantic `Relation`s in SOL.

## 1. Research question

SOL already models semantic connections between independently identifiable concepts as `Relation`s. During the Value study, `ReferenceDefinition` was proposed as a possible `ValueDefinition` subtype.

This creates a boundary question:

```text
Semantic reference:
Entity A ── Relation ──> Entity B

Value reference:
Entity A
   └── has_value_definition
          └── ReferenceDefinition
                 └── source → B / B's evaluated value
```

The goal of this study is to determine whether the second form represents a genuinely different semantic operation.

## 2. MOOSE evidence

### 2.1 Coupled variable reference

MOOSE `InputParameters::addCoupledVar` registers a coupled variable name and allows the consuming object to retrieve the referenced field. A coupled variable may also have a numeric default if no actual variable is supplied.

Semantic normalization:

```text
Native metadata:
  some_variable = velocity_field

SOL:
  Consumer ── depends_on / couples_to ──> VelocityField
```

The important semantic fact is the connection to a Field Entity. The native variable name is a backend reference mechanism, not a value-definition category.

### 2.2 Material-property name reference

MOOSE uses `MaterialPropertyName` parameters so producers and consumers can refer to named material properties. The Materials documentation also shows that the same native parameter mechanism can accept a numeric/default parsed value in some cases.

Semantic normalization when a property is referenced:

```text
HeatConduction
    ── parameterized_by ──> ThermalConductivity
```

Again, the reference is best interpreted as a semantic relation between simulation concepts.

### 2.3 Value forwarding

`CoupledVariableValueMaterial` explicitly stores values of a coupled variable into a material property.

This case contains two distinct semantics:

```text
MaterialProperty
    ── value_defined_from ──> Field

Field
    ── evaluated at simulation context ──> Value
```

The source Field remains a semantic Entity. The material property's definition, however, is derived from the evaluated value of that Field. This is the strongest MOOSE evidence that **semantic reference** and **value derivation from another semantic source** are not identical operations.

## 3. COMSOL evidence

### 3.1 Parameter expressions

COMSOL permits one global parameter to be defined in terms of another, for example conceptually:

```text
c = 1 + a
```

COMSOL distinguishes parameter expressions from more general variable expressions. Parameter expressions may depend on model parameters and supported functions, while variable expressions may additionally depend on solution-dependent and spatial variables.

The semantic content is not merely:

```text
c ── references ──> a
```

because the defining expression `1 + a` is itself part of how `c` is evaluated.

Candidate normalization:

```text
Parameter c
    └── has_value_definition
          └── ExpressionDefinition("1 + a")
                 └── depends_on ──> Parameter a
```

The dependency is a Relation, while the expression is a ValueDefinition.

### 3.2 Material properties and model inputs

COMSOL material property groups define material properties, functions, and model inputs. Temperature-dependent material properties can use a temperature model input; physics features set to `From material` retrieve material properties and substitute the appropriate model-input variables.

This suggests:

```text
ThermalConductivity
    └── has_value_definition
          └── Function/ExpressionDefinition
                 └── depends_on ──> Temperature
```

The Temperature dependency is not itself a `ReferenceDefinition`; it is a semantic dependency of the value definition.

### 3.3 `From material`

COMSOL physics features can choose `From material` instead of a user-defined value. This is a backend-level source-selection mechanism.

Semantic normalization should preserve the semantic relationship to the material property rather than introduce a generic reference-valued datum:

```text
HeatConductionModel
    ── parameterized_by ──> ThermalConductivity
```

The backend profile may separately record that COMSOL realizes this relation through `From material`.

## 4. Ansys evidence

### 4.1 Constant expression

Ansys Mechanical allows a boundary-condition Magnitude to contain a mathematical expression. Mechanical evaluates that expression and applies the resulting magnitude.

Candidate normalization:

```text
BoundaryCondition
    └── prescribed_value_definition
          └── ExpressionDefinition
                 └── evaluates_to ──> Value
```

If the expression depends on semantic variables, those dependencies should be represented explicitly as relations from the definition to the relevant semantic concepts.

### 4.2 Function loads

Mechanical supports mathematical function loads such as time- or spatially-varying functions. The function is a value-evaluation mechanism, while time, coordinates, temperature, or other supported primary variables are independent variables/dependencies.

```text
Load
    └── has_value_definition
          └── FunctionDefinition
                 ├── depends_on ──> Time
                 └── evaluates_to ──> LoadValue
```

### 4.3 Tabular data

Mechanical tabular loads distinguish dependent magnitude values from independent variables such as time, frequency, coordinates, normalized path position, or temperature. Engineering Data likewise represents material data using dependent and independent variables.

This supports:

```text
MaterialProperty / Load
    └── has_value_definition
          └── TabularDefinition
                 ├── independent_variable ──> Temperature / Time / ...
                 └── evaluates_to ──> Value
```

The independent-variable connection is a semantic relation inside the definition, not a separate `ReferenceValue` shape.

## 5. Cross-backend semantic matrix

| Case | MOOSE | COMSOL | Ansys | SOL interpretation |
|---|---|---|---|---|
| Backend name points to Field | coupled variable | variable/field namespace | field/context references | semantic `Relation` to Field |
| Backend name points to MaterialProperty | `MaterialPropertyName` | `From material` / material namespace | Engineering Data property usage | semantic `Relation` to MaterialProperty |
| Value is computed from another quantity | coupled field value used to produce property | expression/function using model input | function/table using independent variable | `ValueDefinition` + dependency `Relation` |
| Literal/expression syntax contains another parameter | parsed/default mechanisms | `c = 1 + a` | mathematical/function expression | Expression/Function definition; referenced concepts are dependencies |
| Spatial target | `block`, `boundary` | selection | Location/Named Selection | semantic `Relation` to Scope, not ValueDefinition |

## 6. Main insight

The cross-backend evidence indicates that the word **reference** hides two different concerns:

### A. Semantic object reference

A backend identifier points to another independently meaningful simulation concept.

```text
BC ── targets ──> TemperatureField
Equation ── defined_on ──> Domain
HeatConduction ── parameterized_by ──> ThermalConductivity
```

This belongs to SOL `Relation` semantics.

### B. Value-definition dependency

A value definition is evaluated using another semantic concept's current/evaluated value.

```text
ThermalConductivity
    └── has_value_definition
          └── FunctionDefinition
                 └── depends_on ──> Temperature
```

This belongs to `ValueDefinition` semantics **plus a Relation expressing dependency**.

The reference mechanism itself does not need to become a value type.

## 7. Working conclusion

Current evidence does **not** justify `ReferenceValue` or `ReferenceDefinition` as a general peer of `LiteralDefinition`, `ExpressionDefinition`, `FunctionDefinition`, and `TabularDefinition`.

Instead, the preferred model is:

```text
ValueDefinition
├── LiteralDefinition
├── ExpressionDefinition
├── FunctionDefinition
├── TabularDefinition
└── ...

ValueDefinition ── depends_on ──> SemanticEntity
Entity          ── semantic_relation ──> Entity
```

Thus, **reference is primarily a graph relation mechanism, not a value-evaluation mechanism**.

A future specialized alias/reference definition should only be introduced if a reference is found whose complete semantics are specifically "use the source's value unchanged" and this cannot be represented cleanly as an existing relation plus value-definition dependency.

## 8. Proposed candidate rule

> **R1 — Reference/Dependency Separation:** A reference to an independently identifiable simulation concept SHALL be represented by a semantic `Relation`. When a `ValueDefinition` uses another semantic concept during evaluation, the definition SHALL express that concept as a dependency relation rather than classifying the definition itself as a generic `ReferenceDefinition`.

`R1` is a research conclusion in this document, not yet an accepted normative SOL rule.

## 9. Implications for the Value taxonomy

The working ValueDefinition taxonomy can now be simplified:

```text
ValueDefinition
├── LiteralDefinition
├── ExpressionDefinition
├── FunctionDefinition
├── TabularDefinition
├── InterpolationDefinition   [boundary with TabularDefinition unresolved]
└── ExternalDataDefinition    [provisional]
```

`ReferenceDefinition` is removed from the default candidate taxonomy pending contrary evidence.

This also reinforces ADR 0002 V3: value shape and evaluation mechanism remain orthogonal, while semantic dependencies are represented as graph relations.

## 10. Next questions

1. Is `depends_on` one generic relation, or should SOL distinguish `depends_on_field`, `depends_on_parameter`, `depends_on_time`, and similar semantics?
2. Should an expression's dependency graph be explicit in authored ontology or derived by the compiler from the expression AST?
3. Are `TabularDefinition` and `InterpolationDefinition` genuinely distinct semantic mechanisms?
4. Does an external dataset define a value directly, or is it provenance/source information for a tabular/interpolation definition?
5. How should cyclic value-definition dependencies be detected and constrained?
