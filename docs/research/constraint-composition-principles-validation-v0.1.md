# Constraint Composition Principles Validation v0.1

**Status:** Research/design candidate  
**Date:** 2026-08-20  
**Reference systems:** MOOSE, COMSOL, Ansys, plasma-domain cases  
**Purpose:** Validate candidate SOL constraint-composition rules C6-C9 before promotion to architecture/ADR status.

## Candidate rules under test

### C6 — Conjunctive Composition
Constraints contributed by inheritance, interfaces, local type definitions, profiles, and active conditional rules are accumulated conjunctively unless the language explicitly defines another composition operator.

### C7 — Monotonic Refinement
A child type, interface implementation, or profile may narrow an inherited semantic contract but must not silently weaken it.

### C8 — No Silent Override
Conflicting semantic constraints must not be resolved by declaration order, profile priority, or last-write-wins semantics.

### C9 — Explicit Conflict Detection
If the effective constraint set is unsatisfiable, SOL validation must report a schema conflict or a configuration-specific conflict.

## Evidence and stress tests

### 1. MOOSE — accumulated parameter contracts

MOOSE `InputParameters` accumulates metadata such as requiredness and range checks and validates the resulting parameter state in `checkParams()`. Required parameters and range-checked parameters are checked rather than silently overridden by input order. This supports C6 and C9 at the semantic-normalization layer.

SOL must not copy MOOSE's implementation details, but a MOOSE profile can safely normalize independent native requirements into a conjunctive effective constraint set.

### 2. COMSOL — activation and usage conditions

COMSOL Physics Builder explicitly supports Activation Conditions and Usage Conditions. Conditions may be combined using Boolean AND/OR and negation, and allowed values/references can themselves be conditionally active.

This directly supports the distinction between:

- static constraints that are always active, and
- conditional constraints that enter the effective set only when their predicate evaluates true.

Once active, the requirements accumulate rather than becoming implicit override rules. This supports C6 and the need for C9 configuration-conflict detection.

### 3. Ansys — option-dependent properties

Ansys Mechanical repeatedly exposes properties only under particular selections. Examples include Geometry vs Named Selection scoping and `Transform Original` becoming available only when Number Of Copies is at least one.

These cases normalize naturally as implications such as:

```text
ScopingMethod = GeometrySelection
  -> Geometry scope is applicable/required

ScopingMethod = NamedSelection
  -> NamedSelection reference is applicable/required

NumberOfCopies >= 1
  -> TransformOriginal is applicable
```

No evidence requires last-write-wins semantic constraint resolution. The rules are configuration-dependent applicability contracts, supporting C6, C8, and C9.

### 4. Plasma-domain case — local field approximation

COMSOL Plasma provides a strong domain stress test. Under Local Field Approximation, the mean electron energy equation is not solved; reduced-electric-field-based information is required instead. Additional inputs become available for particular combinations of diffusion model, mean-electron-energy model, EEDF model, and heavy-species-energy settings.

A normalized SOL rule can be represented as:

```text
IF
  diffusion_model = Global
  AND mean_electron_energy_model = LocalField
  AND eedf_model IN {BoltzmannLinear, BoltzmannQuadratic}
THEN
  ReducedElectricField input is required
```

If another active rule forbids the same input for the same configuration, the configuration is unsatisfiable. Silently selecting one rule would hide a physically meaningful modeling contradiction. This strongly supports C8 and C9.

## Refinement test

Consider:

```text
BoundaryCondition.targets -> Field
TemperatureBoundaryCondition.targets -> TemperatureField
TemperatureField <: Field
```

The child constraint is a subset of the parent admissible set. Their conjunction is satisfiable and equals the narrower set. This supports monotonic refinement.

By contrast:

```text
Parent target type -> TemperatureField
Child target type  -> arbitrary Field
```

interpreting the child declaration as replacement would weaken the parent guarantee. SOL should not give a child declaration implicit replacement semantics. If widening is ever required, it must be modeled as an explicit schema change rather than ordinary specialization.

## Interface-composition conflict test

If two implemented interfaces constrain the same single-valued relation as:

```text
Interface A: target must be TemperatureField
Interface B: target must be ElectricField
```

and the type system provides no type inhabiting both requirements, the effective intersection is empty. The implementing type is invalid. Interface order must not determine the result.

This supports C6, C8, and C9.

## Profile test

A backend profile may add representability restrictions, for example requiring a canonical serialization unit or limiting a supported model form. Such a restriction may narrow the set of representable SOL models.

A backend profile must not redefine the domain semantics to make an otherwise-invalid model valid. When a valid SOL model cannot be represented by the backend, the correct result is an unsupported/lossy mapping diagnostic rather than weakening Core/domain constraints.

This supports C7 with an important qualification: profile refinement is allowed only for backend applicability/representability and must not mutate upstream semantic truth.

## Conflict classification

C9 should distinguish at least:

1. **Schema conflict** — the declared type/interface/profile contract is intrinsically unsatisfiable.
2. **Configuration conflict** — the schema is satisfiable in general, but the predicates active for a particular model instance produce an unsatisfiable effective set.
3. **Backend representability conflict** — the SOL model is semantically valid but the selected backend/profile cannot encode it without unsupported or lossy translation.

These are different diagnostics and must not be collapsed into one generic validation failure.

## Result

All four candidate principles survive the cross-backend and plasma-domain stress tests, with one clarification to C7.

### C6 — Conjunctive Composition: ACCEPT candidate

Effective semantic constraints are accumulated conjunctively. Conditional constraints contribute only when their predicates are active.

### C7 — Monotonic Refinement: ACCEPT candidate with profile qualification

Specialization may narrow inherited admissible sets. Backend profiles may narrow representability/applicability but must not weaken or rewrite upstream semantic truth.

### C8 — No Silent Override: ACCEPT candidate

Constraint conflicts are never resolved by order or implicit priority. Any future explicit override mechanism would require separate semantics and justification.

### C9 — Explicit Conflict Detection: ACCEPT candidate

Unsatisfiable effective constraints must produce explicit diagnostics, classified as schema, configuration, or backend-representability conflicts.

## Architectural implication

The effective constraint model is:

```text
EffectiveSemanticConstraints
  = inherited
  AND interface
  AND local
  AND active conditional

BackendEffectiveContract
  = EffectiveSemanticConstraints
  AND profile representability/applicability restrictions
```

A profile restriction cannot turn an invalid semantic model into a valid one.

## Next question

The next design issue is how to represent and compute the intersection/refinement relation for each primitive family (Cardinality, Type, Value, Dimension, Compatibility, Conditional) and how diagnostics identify the source constraints responsible for an empty intersection.
