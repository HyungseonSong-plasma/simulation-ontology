# Constraint Intersection Algebra v0.1

**Status:** Research/design candidate  
**Date:** 2026-08-20  
**Purpose:** Define machine-computable intersection semantics for SOL constraint primitives so conjunctive composition can be implemented by a validator.

## 1. Core principle

SOL constraint composition is conjunctive. For constraints applying to the same semantic subject/use-site, the effective constraint is their intersection whenever such an intersection is defined.

```text
EffectiveConstraint = C1 ∩ C2 ∩ ... ∩ Cn
```

If the intersection is empty or unsatisfiable, validation reports a conflict rather than silently overriding one constraint with another.

This operationalizes the candidate principles:

- C6 — Conjunctive Composition
- C7 — Monotonic Refinement
- C8 — No Silent Override
- C9 — Explicit Conflict Detection

## 2. Cardinality intersection

Represent cardinality as an interval over non-negative integers:

```text
[min_count, max_count]
```

where `max_count = *` means unbounded.

Intersection:

```text
[min1, max1] ∩ [min2, max2]
= [max(min1,min2), min(max1,max2)]
```

with unbounded maximum handled as +∞.

Examples:

```text
[1,*] ∩ [0,1] = [1,1]
[0,3] ∩ [2,5] = [2,3]
[2,2] ∩ [0,1] = ∅   -> conflict
```

Requiredness is normalized to cardinality:

```text
required = true  -> min_count >= 1
functional       -> max_count <= 1
```

Thus requiredness and functional constraints do not require separate intersection rules.

## 3. Type intersection

Type constraints operate over semantic type sets and the SOL subtype relation.

For simple single-type constraints:

```text
Field ∩ TemperatureField = TemperatureField
```

when:

```text
TemperatureField <: Field
```

If two types are unrelated and no entity can satisfy both under the v0.1 single-taxonomic-inheritance model:

```text
TemperatureField ∩ ElectricField = ∅
```

For v0.1, the validator SHOULD use subtype compatibility rather than general multiple-type satisfiability.

Algorithm for simple type constraints:

1. If A = B, result = A.
2. If A <: B, result = A.
3. If B <: A, result = B.
4. Otherwise, result = conflict unless an explicit Interface/capability rule makes the conjunction satisfiable on a separate axis.

Entity type and Interface requirements MUST remain separate semantic axes; Interface composition is not converted into type intersection.

## 4. Value constraint intersection

Value constraints are normalized into allowed-value sets or numeric intervals where possible.

### Numeric intervals

```text
[a,b] ∩ [c,d] = [max(a,c), min(b,d)]
```

Examples:

```text
[0,1] ∩ [0.2,0.8] = [0.2,0.8]
[0,1] ∩ [2,3] = ∅
```

Open/closed endpoints MAY be supported in the canonical representation if reference models require them. v0.1 authoring syntax may initially use inclusive bounds only.

### Enumerated values

Allowed-value sets intersect by set intersection:

```text
{A,B,C} ∩ {B,C,D} = {B,C}
```

Empty set means conflict.

### Scalar comparisons

Comparisons such as `< x`, `<= x`, `> x`, `>= x`, `=` SHOULD be normalized to interval/set form before composition when possible.

Examples:

```text
x >= 0
x < 10
=> [0,10)
```

`not_equals` MAY require a disjoint-set/range representation and can remain a validator predicate rather than a fully normalized interval in v0.1.

## 5. Dimension intersection

Dimension constraints use canonical normalized `DimensionVector` equality.

```text
D1 ∩ D2 = D1   if D1 == D2
D1 ∩ D2 = ∅    otherwise
```

Examples:

```text
Temperature ∩ Temperature = Temperature
Temperature ∩ Pressure = ∅
```

Dimension constraints do not perform semantic-quantity intersection. Distinct semantic quantities may share the same DimensionVector.

## 6. Compatibility intersection

Compatibility is not itself reduced to a single scalar value in all cases. It represents a relation among semantic contracts.

For v0.1, compatibility constraints SHOULD compose conjunctively as a set of obligations:

```text
Compat(A,B) ∩ Compat(A,C)
=> require Compat(A,B) AND Compat(A,C)
```

The validator evaluates each obligation against the relevant compatibility relation or adapter.

If one obligation requires compatibility and another explicitly forbids the same compatibility relation for the same operands/context, the result is conflict.

Compatibility may delegate to:

- semantic quantity compatibility;
- unit/metrology compatibility;
- backend/profile capability compatibility;
- relation endpoint compatibility.

These SHOULD remain separate diagnostics even when represented by the same primitive family.

## 7. Conditional intersection

Conditional constraints are implications:

```text
P -> C
```

They are not intersected by merging their predicates or bodies blindly.

Instead, validation proceeds in two phases:

1. Evaluate predicates for the current configuration/context.
2. Collect all active consequent constraints.
3. Intersect the active ordinary constraints using the primitive-specific rules above.

Example:

```text
P1 -> require X
P2 -> forbid X
```

If only P1 is true, no conflict.
If only P2 is true, no conflict.
If both P1 and P2 are true, the active cardinality/value constraints intersect to ∅ and produce a configuration conflict.

This preserves Conditional as a meta-constraint.

## 8. Predicate composition

Predicate vocabulary remains:

```text
Compare
Membership
Exists
Boolean(and/or/not)
```

Predicate composition determines applicability, not constraint meaning.

Two conditional constraints with different predicates remain distinct rules unless a later normalization pass proves them logically equivalent.

SOL v0.1 SHOULD NOT require general theorem proving for predicate equivalence.

## 9. Constraint source composition

Effective constraints may originate from:

```text
Inherited type constraints
Interface constraints
Local type/entity constraints
Profile constraints
Active conditional constraints
```

All active constraints are combined conjunctively.

```text
Effective = Inherited ∩ Interface ∩ Local ∩ Profile ∩ ActiveConditional
```

Source precedence MUST NOT change semantic truth.

A source label/provenance SHOULD be retained for diagnostics so conflicts can report which constraints contributed to the unsatisfiable intersection.

## 10. Conflict classes

### Schema Conflict

The schema is intrinsically unsatisfiable.

Example:

```text
Interface A: targets exactly 1 TemperatureField
Interface B: same target must be ElectricField
```

with no satisfiable common type.

### Configuration Conflict

The schema is satisfiable in general, but a selected configuration activates incompatible constraints.

Example:

```text
P1 -> require X
P2 -> forbid X
```

when P1 and P2 are both true.

### Backend Representability Conflict

The SOL semantic model is valid, but the selected backend/profile cannot realize the effective semantic contract.

This is not an empty semantic-constraint intersection and MUST remain distinct from ontology/schema invalidity.

## 11. Candidate validator algorithm

```text
1. Resolve entity/type inheritance.
2. Collect implemented Interface contracts.
3. Collect local constraints.
4. Apply Profile refinements without weakening semantic contracts.
5. Evaluate Conditional predicates.
6. Add active consequent constraints.
7. Group constraints by subject/path/semantic axis.
8. Normalize each primitive where possible.
9. Compute intersections.
10. If any intersection is empty -> schema/configuration conflict.
11. Validate compatibility obligations.
12. Separately evaluate backend representability.
```

## 12. Candidate principles

### C10 — Primitive-Specific Intersection

Each constraint primitive SHALL define deterministic composition/intersection semantics where feasible.

### C11 — Normalize Before Compose

Constraints SHOULD be normalized into canonical forms before intersection when that reduces equivalent syntax to the same semantic representation.

Examples include cardinality intervals, value intervals/sets, canonical DimensionVectors, and subtype-normalized type constraints.

### C12 — Conditional Activation Before Intersection

Conditional predicates SHALL be evaluated before intersecting their consequent constraints for a concrete configuration.

### C13 — Preserve Constraint Provenance

The validator SHOULD retain the source of each effective constraint so conflicts identify inherited, interface, local, profile, and conditional contributors.

## 13. Deferred issues

The following are intentionally deferred from v0.1 unless reference models require them:

- arbitrary union/intersection type expressions;
- full predicate theorem proving;
- symbolic algebra for nonlinear value constraints;
- general disjoint numeric regions from multiple `not_equals` constraints;
- probabilistic/fuzzy constraints;
- optimization objectives (not validation constraints).
