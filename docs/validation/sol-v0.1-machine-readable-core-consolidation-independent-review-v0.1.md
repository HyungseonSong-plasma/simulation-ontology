# SOL v0.1 Machine-Readable Core Consolidation — Independent Review v0.1

**Role:** Independent Validation  
**Date:** 2026-08-20  
**Branch:** `validation/sol-v0.1-independent-review`

## 1. Inputs and isolation

Validation used the frozen/accepted contracts and current durable artifacts:

- ADR-0008 — Inheritance and Interface Composition
- ADR-0009 — Identity, Namespace, Package, and Versioning
- ADR-0013 — SOL v0.1 Design-Stage Architecture Freeze
- `docs/architecture.md`
- `docs/ontology-language.md`
- `ontology/core/entities.yaml`
- `ontology/core/relations.yaml`
- `ontology/core/constraints.yaml`
- `docs/research/sol-v0.1-machine-readable-core-consolidation-gap-analysis-v0.1.md` only as the artifact under test, not as an assumed conclusion

Validation criterion: two independent conforming readers of the same frozen artifacts must not be permitted to assign different canonical semantic meanings to the same machine-readable construct.

## 2. Executive verdict

**Verdict: ACCEPT the focused remediation direction.**

The review confirms two architecture/language-level gaps and several transcription defects. None requires reopening backend mapping, MappingPlan, BackendTarget, or QRC architecture.

| Finding | Validation result | Classification |
|---|---|---|
| dual use of `Interface` | Confirmed | Architecture naming defect |
| `children` semantic ambiguity | Confirmed | Language/schema contract gap |
| missing `Result` declaration | Confirmed | Referential completeness defect |
| Interface still marked provisional | Confirmed | Documentation drift |
| stale core status/pending list | Confirmed | Documentation/schema drift |
| `produces` graph placement ambiguity | Confirmed | Documentation relation-domain drift |

## 3. Counterexample validation

### V-MC-01 — Canonical `Interface` collision

Current architecture uses `Interface` under `SpatialModel` for a spatial interface. ADR-0008 separately gives `Interface` normative capability-contract behavior: Entity Types `implement` Interfaces and Interfaces may `extend` Interfaces.

Counterexample:

```text
identifier = Interface
```

Reader A resolves it as a spatial entity type. Reader B resolves it as the abstract capability contract. Both interpretations are supported by current frozen documents.

Namespace rules do not remove the collision if both are exported from the same Core ontology package with the same local canonical identifier. Requiring contextual inference would make identity depend on use site rather than stable identifier.

**Result: confirmed architecture naming defect.**

The minimum deterministic repair is to keep `Interface` for the ADR-0008 language/capability construct and rename the spatial concept to `SpatialInterface`. This changes lexical identity only; it does not change the spatial semantics.

Palantir's official Ontology documentation is consistent supporting evidence: its Interface is an abstract ontology type describing shared shape/capabilities and is implemented by concrete object types. That evidence supports keeping the generic term `Interface` for the capability abstraction, but Palantir is not normative for SOL.

### V-MC-02 — `children` cannot be deterministically mapped to `is_a`

ADR-0008 defines taxonomic inheritance explicitly as `is_a -> 0..1`. `entities.yaml` instead encodes all architecture tree edges with `children`.

Counterexample:

```text
Simulation
  children:
    - SimulationModel
    - SimulationTask
```

Reader A treats this as two subtype edges. Reader B treats it as structural grouping/composition because the architecture prose says Simulation separates Model and Task rather than saying either is a subtype of Simulation.

No frozen rule uniquely selects one interpretation.

**Result: confirmed language/schema contract gap.**

`children` must not become canonical machine semantics. Explicit `is_a` may be emitted only where taxonomic inheritance is actually asserted. Composition requires explicit Relation semantics and must not be inferred from diagram indentation.

### V-MC-03 — `Result` reference is not resolvable from the entity registry

The architecture graph and `relations.yaml` use `Result`, but `entities.yaml` does not declare it.

A schema compiler that requires all relation endpoints to resolve must fail or invent a type.

**Result: confirmed referential completeness defect.**

Declaring bare `Result` as an existing Core semantic type, without adding subtype taxonomy, is a completion of already-published semantics rather than a new domain concept.

### V-MC-04 — Interface provisional status is stale

`docs/ontology-language.md` still labels Interface provisional, but ADR-0008 has status Accepted and defines normative Interface composition.

**Result: confirmed documentation drift.**

The language document must defer to ADR-0008 and mark Interface normative for v0.1. Executable action/method requirements remain deferred.

### V-MC-05 — stale `draft`/`pending` machine state

The machine-readable core still describes namespace/identity and unit/dimension semantics as broadly pending despite accepted ADRs. However, relation cardinalities and some exact serialization/composition rules genuinely remain unresolved.

**Result: confirmed state-classification defect.**

Do not mark the machine-readable package fully stable yet. Split accepted-but-not-transcribed work from genuinely open language/schema decisions.

### V-MC-06 — `produces` domain

`relations.yaml` explicitly states:

```text
SimulationTask -> produces -> Result
```

and the architecture top-level model/task section uses the same contract. A later relationship diagram visually indents `produces` below `SolverConfiguration`, but contains no text changing the relation domain.

An explicit relation declaration has stronger normative force than diagram indentation.

**Result: retain `SimulationTask -> produces -> Result`; correct the misleading graph.**

No evidence supports narrowing `produces` to `SolverConfiguration` in v0.1.

## 4. Scope-preservation review

The proposed remediation does not require:

- changing MappingRule or MappingClaim;
- changing MappingPlan lifecycle/effect contracts;
- changing QRC;
- adding backend-native vocabulary;
- adding Adapter runtime semantics;
- introducing Result subtypes;
- inventing full Simulation composition/cardinality semantics.

The fix must remain narrow. In particular, MC-02 must not be used as justification to fabricate `has_model`, `has_task`, or other composition relations without a separate evaluation contract.

## 5. Freeze effect

ADR-0013 allows a narrow freeze amendment when a normative contradiction or independent-reader ambiguity is discovered.

This review finds that condition satisfied for:

1. the `Interface` canonical naming collision; and
2. the undefined semantic interpretation of machine-readable `children`.

The rest of the frozen architecture remains intact.

## 6. Required result

Proceed with one focused ADR/amendment that:

1. reserves `Interface` for the ADR-0008 capability contract;
2. renames the spatial entity concept to `SpatialInterface`;
3. prohibits `children` from carrying normative inheritance semantics;
4. requires explicit `is_a` for taxonomic inheritance;
5. declares existing `Result` as a resolvable Core type without subtype expansion;
6. confirms `SimulationTask -> produces -> Result`;
7. updates stale language/core documentation accordingly;
8. leaves unresolved composition relation/cardinality design explicitly open rather than guessing.

## 7. Final verdict

| Question | Verdict |
|---|---|
| Research gap diagnosis | **Accept** |
| `SpatialInterface` disambiguation | **Accept** |
| `children` as canonical semantics | **Reject** |
| bare `Result` declaration | **Accept** |
| `SimulationTask -> produces -> Result` | **Accept** |
| Reopen ADR-0010/0011/0012 | **No** |
| Narrow ADR-0013 freeze amendment justified | **Yes** |

**Next state: Decision → focused ADR amendment, then transcription update and readback validation.**
