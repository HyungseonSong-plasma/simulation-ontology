# SOL v0.1 Type Constraint Schema Proposal v0.1

**Role:** Research  
**Date:** 2026-08-20

## 1. Objective

Define the minimum machine-readable payload for the accepted SOL `Type` Constraint family without inventing a universal graph-path language or mixing Interface capability composition into taxonomic type intersection.

This proposal is limited to the v0.1 use-case already established by the Relation/Constraint studies: **context-local narrowing of the allowed target Entity Type for a semantic Relation**.

## 2. Existing semantic contract

Accepted SOL rules already require:

- Relation endpoints accept the declared type or a subtype;
- subtype/use-site narrowing belongs to Constraint rather than RelationDefinition override;
- one Entity Type has at most one direct taxonomic `is_a` parent;
- Interface requirements are a separate semantic axis;
- active constraints compose conjunctively;
- declaration order and profile priority do not override semantic truth.

Therefore the Type family does not need a generic Boolean/type-expression language in v0.1.

## 3. Constraint context is container-owned

A Type Constraint is declared inside or contributed by a semantic context such as:

```text
Entity Type
Interface
Profile/local refinement
```

The containing declaration identifies the constrained source/use-site. The Type payload does not introduce a universal `subject`, `path`, or graph-query field.

Example context:

```text
TemperatureBoundaryCondition
  constraint:
    relation = applied_to
    target_type = TemperatureField
```

The semantic meaning is:

> for this contributing context, every target considered under the applicable `applied_to` target-type restriction must conform to `TemperatureField` or a subtype.

Context provenance is carried by the NormalizedConstraintEvidence/package layer under ADR-0018.

## 4. Authoring Type payload

Minimal authoring form:

```yaml
type: type
relation: <relation-id>
target_type: <semantic-type-id>
```

No list/union of target types is introduced.

Authoring identifiers may be package-local resolvable names; normalization resolves them to canonical semantic identities.

## 5. Normalized Type payload

Canonical normalized payload:

```yaml
type: type
relation: <canonical-relation-id>
target_type: <canonical-semantic-type-id>
```

The normalized payload has the same structural fields as authoring but differs by identity state: relation and target type are canonical/resolved.

JSON Schema can enforce non-empty string structure but cannot prove canonical identity or subtype relationships; those remain semantic-validator responsibilities.

## 6. Relation compatibility precondition

Before composition, a Type Constraint target must be compatible with the underlying RelationDefinition/use-site contract.

For a relation with simple declared range `R`:

```text
valid target_type T
iff
T = R OR T <: R
```

If `T` widens the relation range or is unrelated:

```text
Schema Conflict
```

For a relation governed by an allowed-pair matrix such as `includes_component`, compatibility is evaluated against the contributing source context plus the authoritative allowed-pair matrix from ADR-0016.

A Type Constraint never broadens the base relation contract.

## 7. Type Constraint composition

For two active normalized Type Constraints on the same semantic use-site/relation:

```text
C(A) ∩ C(B)
```

use canonical Entity subtype closure:

1. if `A = B`, result = `A`;
2. if `A <: B`, result = `A`;
3. if `B <: A`, result = `B`;
4. otherwise, Type-axis intersection is empty -> Schema/Configuration Conflict according to contributing constraint activation context.

Because v0.1 Entity taxonomy has single direct taxonomic inheritance, unrelated Entity types do not gain satisfiability through Interface implementation. Interface requirements remain separate conjunctive axes and SHALL NOT be converted into a synthetic multiple-inheritance Type intersection.

## 8. Multiple typing boundary

An instance may carry multiple consistent semantic classifications only where the language/runtime representation permits them without violating the single taxonomic parent rule (for example, Entity Type plus implemented Interfaces).

For Type Constraint satisfaction, the taxonomic Entity Type chain is authoritative. Interface identity is not treated as an alternate `target_type` match unless a future contract explicitly defines an Interface-target Constraint family/field.

This keeps:

```text
Entity taxonomic type
```

orthogonal to:

```text
implements Interface
```

## 9. Specialization/refinement behavior

Example:

```text
RelationDefinition:
  applied_to -> Field | Equation | Scope   # ADR-0016 family

TemperatureBoundaryCondition constraint:
  type: type
  relation: applied_to
  target_type: TemperatureField

TemperatureField <: Field
```

The local constraint is a monotonic narrowing and is valid.

By contrast:

```text
Parent/local constraint target_type: TemperatureField
Child/profile constraint target_type: Field
```

The second declaration does not weaken/replace the first. Conjunctive composition remains `TemperatureField`.

If the second declaration is intended as a widening override, SOL v0.1 provides no implicit override mechanism.

## 10. QRC interaction

QRC qualifier and Type Constraint are different uses of semantic type identity:

- QRC `qualifier.target_type` chooses the subset counted by a Cardinality constraint;
- Type Constraint restricts which relation targets are valid at a use-site.

They may coexist and compose independently.

Example:

```text
Type constraint: products targets must be Species
QRC: products must contain at least 1 NegativeIonSpecies
NegativeIonSpecies <: Species
```

Both can be satisfied simultaneously.

QRC does not automatically create a Type Constraint requiring every relation target to match the qualifier.

## 11. Schema implementation slice

After contract acceptance implement:

1. `schema/constraint-type-authoring-v0.1.schema.json`
2. `schema/constraint-type-normalized-v0.1.schema.json`
3. minimal semantic helper/tests for:
   - authoring structure;
   - canonical-id resolution boundary supplied externally;
   - equal type intersection;
   - subtype/supertype narrowing order independence;
   - unrelated-type conflict;
   - relation-range compatibility;
   - Interface identity not accepted as taxonomic target type merely because implemented;
   - Type + QRC coexistence does not turn QRC qualifier into universal target restriction.

No backend runtime is required.

## 12. Counterexamples

### TC-01 — child widening treated as override

```text
C1 target = TemperatureField
C2 target = Field
```

Expected effective Type constraint: `TemperatureField`, independent of order.

### TC-02 — unrelated types resolved by declaration priority

```text
TemperatureField
ElectricField
```

Expected: explicit Type-axis conflict; no last-write-wins.

### TC-03 — Interface mistaken for Entity type

```text
target_type = Scoped
```

where `Scoped` is an Interface, not an Entity Type.

Expected semantic validation failure for this Type payload; Interface requirements use Interface semantics, not taxonomic Type Constraint.

### TC-04 — QRC qualifier becomes universal range rule

```text
QRC qualifier = NegativeIonSpecies
```

Expected: only counted subset is qualified; unrelated product targets may still exist unless a separate Type Constraint forbids them.

## 13. Research verdict

Adopt relation-target narrowing as the minimal v0.1 machine-readable Type Constraint payload using `type + relation + target_type`, canonicalize identities during normalization, and use single-inheritance subtype intersection for deterministic composition.

**Ready for independent Validation.**
