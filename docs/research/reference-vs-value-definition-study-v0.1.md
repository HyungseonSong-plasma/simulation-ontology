# Reference vs ValueDefinition Study v0.1

**Status:** Research evidence  
**Date:** 2026-08-20  
**Reference backends:** MOOSE, COMSOL, Ansys Mechanical  
**Purpose:** Test whether `Reference` should be modeled as a `ValueDefinition`, or whether references should remain semantic `Relation`s in SOL.

## 1. Research question

SOL already models semantic connections between independently identifiable concepts as `Relation`s. During the Value study, `ReferenceDefinition` was proposed as a possible `ValueDefinition` subtype.

This creates a boundary question between semantic object references, value dependencies, and pure value aliases.

## 2. Cross-backend evidence

### MOOSE

MOOSE coupled-variable and material-property-name mechanisms use backend identifiers to connect consumers to independently meaningful Fields or MaterialProperties. These normalize naturally to semantic Relations. MOOSE also provides cases where evaluated values from coupled fields are used to produce another property, demonstrating that object reference and value derivation are distinct semantics.

### COMSOL

COMSOL permits expressions such as `c = 1 + a`, where the expression is the evaluation mechanism and `a` is a dependency. A pure alias such as `b = a` can be represented natively as an expression containing only another symbol, but backend syntax does not require SOL to introduce an ExpressionDefinition when no transformation semantics need to be preserved. It can normalize to a direct value-dependency Relation.

### Ansys Mechanical

Ansys distinguishes dependent magnitudes from independent variables in function and tabular definitions. This supports modeling the function/table as a ValueDefinition and its inputs as dependency Relations. Mechanical/backend representations may also collapse semantically different UI representations into a common lower-level representation, reinforcing that backend representation is not itself the SOL semantic classification.

## 3. Semantic cases

### Case A — Semantic object reference

```text
TemperatureBC ── targets ──> TemperatureField
Equation ── defined_on ──> Domain
HeatConduction ── parameterized_by ──> ThermalConductivity
```

The target is an independently meaningful semantic concept. This is a Relation.

### Case B — Value dependency with transformation

```text
A = f(B)

A
 └── has_value_definition
        └── FunctionDefinition / ExpressionDefinition
               └── depends_on ──> B
```

The transformation/evaluation mechanism has semantic content and therefore requires a ValueDefinition.

### Case C — Pure value alias

```text
A = B
```

If the intended semantics are exactly "obtain A's value unchanged from B", no separate evaluation mechanism needs to be represented:

```text
A ── derives_value_from ──> B
```

A `ReferenceDefinition` node would add graph structure without adding semantic information.

## 4. Cross-backend semantic matrix

| Case | MOOSE | COMSOL | Ansys | SOL interpretation |
|---|---|---|---|---|
| Backend name points to Field | coupled variable | variable/field namespace | field/context references | semantic Relation to Field |
| Backend name points to MaterialProperty | `MaterialPropertyName` | `From material` / material namespace | Engineering Data property usage | semantic Relation to MaterialProperty |
| Value computed from another quantity | coupled field value used to produce property | expression/function using model input | function/table using independent variable | ValueDefinition + dependency Relation |
| Pure alias / unchanged forwarding | direct consumption/forwarding of referenced property or value | expression containing only another symbol | direct dependent-value sourcing where backend permits | `derives_value_from` Relation; no mandatory ValueDefinition |
| Spatial target | `block`, `boundary` | selection | Location/Named Selection | semantic Relation to Scope |

## 5. Main insight

The word `reference` hides at least three semantic concerns:

```text
Object dependency
A ── uses / targets / defined_on ──> B

Pure value dependency
A ── derives_value_from ──> B

Computational value dependency
ValueDefinition(A) ── depends_on ──> B
```

These differences are better represented by Relation semantics than by a generic `ReferenceDefinition` type.

## 6. Pure-alias validation result

The pure-alias test does not provide evidence requiring `ReferenceDefinition` as a first-class ValueDefinition subtype.

The minimal representation is:

```text
B = A
→ Relation

B = f(A)
→ ValueDefinition + dependency Relation
```

This keeps graph complexity proportional to semantic complexity and avoids encoding backend syntax as core ontology structure.

## 7. Candidate rules

> **R1 — Reference as Relation:** A reference to an independently identifiable semantic concept SHALL normally be represented by a semantic `Relation`, not by a `ReferenceDefinition`.

> **R2 — Value Dependency:** When one concept obtains its value from another concept, the dependency SHALL be represented semantically as a value-dependency `Relation`. A `ValueDefinition` is required only when an evaluation or transformation mechanism itself must be represented.

These rules are supported by the MOOSE, COMSOL, and Ansys cross-backend study and the pure-alias stress test.

## 8. Implications for ValueDefinition

The default candidate taxonomy therefore excludes `ReferenceDefinition`:

```text
ValueDefinition
├── LiteralDefinition
├── ExpressionDefinition
├── FunctionDefinition
├── TabularDefinition
├── InterpolationDefinition   [boundary unresolved]
└── ExternalDataDefinition    [provisional]
```

Semantic dependencies remain graph Relations rather than value shapes or generic reference definitions.

## 9. Representation-collapse insight

Backend representation may collapse multiple semantic authoring forms into a common implementation representation. Ansys provides examples where higher-level constant/tabular/function-style definitions may be translated into lower-level table-oriented solver input. Therefore SOL normalization must preserve semantic intent rather than infer ontology categories solely from generated backend syntax.

This reinforces the existing rule:

```text
Backend representation ≠ semantic representation
```

## 10. Remaining questions

1. Is `depends_on` one generic relation, or should more specific dependency relations specialize it?
2. Should expression dependencies be authored explicitly or derived from an expression AST?
3. Are `TabularDefinition` and `InterpolationDefinition` semantically distinct?
4. Is external data a ValueDefinition mechanism or provenance/source metadata?
5. How should cyclic value-definition dependencies be detected and constrained?
