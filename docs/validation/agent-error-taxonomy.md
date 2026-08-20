# Agent Error Taxonomy

**Status:** Draft  
**Scope:** Agent-produced changes to the Simulation Ontology, validation system, adapters, generated backend models, tests, and documentation  
**Related policy:** [Agent-First Validation Principles](agent-first-validation-principles.md)

## Purpose

This taxonomy gives agent-originated failures stable identifiers so requirements, validators, tests, review findings, and acceptance decisions can refer to the same failure concepts.

It classifies the **observable defect or validation risk**, not the presumed internal reasoning of an agent. A single change may contain several error classes, and an error may be assigned a higher severity when it affects scientific meaning, stable identity, migration, or solver output.

## Error record

Each finding should record:

```ts
interface AgentErrorFinding {
  errorId: AgentErrorId;
  severity: "info" | "warning" | "error" | "fatal";
  stage: ValidationStage;
  entityId?: string;
  claim: string;
  expected: unknown;
  observed: unknown;
  evidence: Evidence[];
  provenance: Provenance;
  remediation?: string;
}
```

The taxonomy ID identifies the class of failure. A validation rule uses its own stable `ruleId` and may detect one or more taxonomy IDs.

## Taxonomy overview

| Family | Name | Primary question |
|---|---|---|
| INT | Intent | Did the change solve the requested problem? |
| SCP | Scope | Did it change only what was authorised? |
| ASM | Assumption | Did it invent or hide unsupported assumptions? |
| SEM | Semantics | Was domain meaning preserved? |
| IDN | Identity | Were stable entities and references preserved? |
| MAP | Mapping | Was meaning preserved across adapters and backends? |
| TST | Testing | Is the validation evidence capable of detecting failure? |
| EVD | Evidence | Are completion claims supported and reproducible? |
| ENV | Environment | Was success dependent on an undisclosed environment? |
| REP | Reproducibility | Does repeated execution preserve equivalent results? |
| GOV | Governance | Were required authority and acceptance gates respected? |
| DOC | Documentation | Do documentation and implementation describe the same contract? |

## Detailed error classes

### INT — Intent errors

| ID | Error | Typical signal | Required control | Default severity |
|---|---|---|---|---|
| INT-001 | Wrong-problem implementation | Tests pass, but acceptance examples or user intent fail | Predeclared intent and acceptance cases | Error |
| INT-002 | Partial intent coverage | Only the primary happy path is implemented | Requirement-to-test traceability | Error |
| INT-003 | Constraint omission | A stated invariant or forbidden change is ignored | Explicit invariant and forbidden-scope checks | Error |
| INT-004 | Unauthorised reinterpretation | Agent silently changes the meaning of an ambiguous request | Ambiguity gate and recorded decision | Error |

### SCP — Scope errors

| ID | Error | Typical signal | Required control | Default severity |
|---|---|---|---|---|
| SCP-001 | Unrequested change | Unrelated files, entities, or behaviours change | Change-scope manifest and diff review | Error |
| SCP-002 | Opportunistic refactor | Structural rewrite is bundled with a targeted fix | Separate proposal or separately authorised change | Warning |
| SCP-003 | Accidental deletion | Existing capability, metadata, alias, or test disappears | Deletion inventory and regression comparison | Error |
| SCP-004 | Dependency expansion | A new package, service, or backend dependency appears without need | Dependency-diff policy | Warning |

### ASM — Assumption errors

| ID | Error | Typical signal | Required control | Default severity |
|---|---|---|---|---|
| ASM-001 | Fabricated default | Missing input is replaced with an undocumented value | Explicit default registry or hard failure | Error |
| ASM-002 | Hidden unit assumption | A numeric value is interpreted without declared units | Dimensional validation | Fatal |
| ASM-003 | Ambiguity guessing | One of several valid references or mappings is selected silently | Ambiguity diagnostic | Error |
| ASM-004 | Unsupported factual assumption | Backend or physics behaviour is asserted without authoritative evidence | Provenance requirement | Error |
| ASM-005 | Stale assumption | A rule is based on an obsolete backend version or deprecated capability | Version-bounded compatibility evidence | Error |

### SEM — Semantic errors

| ID | Error | Typical signal | Required control | Default severity |
|---|---|---|---|---|
| SEM-001 | Meaning substitution | A related but non-equivalent concept is used | Canonical semantic contract | Fatal |
| SEM-002 | Unit or dimension corruption | Magnitude or dimensionality changes across a transformation | Unit-aware type and dimensional tests | Fatal |
| SEM-003 | Equation corruption | Sign, coefficient, term, frame, or convention changes | Analytical reference, benchmark, or manufactured solution | Fatal |
| SEM-004 | Domain-condition mismatch | Material, source, initial, interface, or boundary condition targets the wrong domain | Domain-reference integrity checks | Fatal |
| SEM-005 | Coupling loss | A declared dependency between physics components is dropped or made one-way | Coupling graph validation | Fatal |
| SEM-006 | Invalid default semantics | A default is legal syntactically but wrong for the model context | Context-specific validation rule | Error |

### IDN — Identity and reference errors

| ID | Error | Typical signal | Required control | Default severity |
|---|---|---|---|---|
| IDN-001 | Stable identity replacement | Rename creates a new entity instead of preserving identity | Identity-preservation test | Fatal |
| IDN-002 | Alias collision | One alias resolves to multiple identities | Uniqueness validation | Error |
| IDN-003 | Scope collision | Equal local names resolve incorrectly across scopes | Scope-aware resolver tests | Error |
| IDN-004 | Broken reference | A reference targets a missing, deprecated, or wrong-kind entity | Reference-integrity validation | Fatal |
| IDN-005 | Rename propagation failure | Some dependent references retain stale names | Dependency traversal and migration test | Error |
| IDN-006 | Deprecation bypass | Deprecated syntax is accepted without the declared warning or migration behaviour | Adapter-specific deprecation tests | Warning |

### MAP — Adapter and backend mapping errors

| ID | Error | Typical signal | Required control | Default severity |
|---|---|---|---|---|
| MAP-001 | Silent semantic loss | Canonical information disappears in generated backend form | Loss report and fail-closed policy | Fatal |
| MAP-002 | False compatibility claim | Adapter reports support but cannot preserve required meaning | Capability contract test | Error |
| MAP-003 | Backend construct mismatch | Correct concept is emitted as the wrong backend object or option | Backend-native structural assertion | Fatal |
| MAP-004 | Version-sensitive mismatch | Mapping works only for an undeclared backend version | Compatibility matrix and version pin | Error |
| MAP-005 | Round-trip corruption | Export and re-import change canonical meaning | Normalised round-trip comparison | Error |
| MAP-006 | Backend leakage into core | Backend-specific naming or behaviour becomes a core semantic requirement | Architecture boundary test | Error |

Compatibility is adapter-defined, but an adapter must explicitly classify each required canonical feature as supported, unsupported, conditionally supported, or lossy.

### TST — Test and oracle errors

| ID | Error | Typical signal | Required control | Default severity |
|---|---|---|---|---|
| TST-001 | Self-confirming test | Test reproduces the implementation's same mistaken assumption | Independent oracle | Error |
| TST-002 | Happy-path-only coverage | Invalid or boundary inputs are absent | Valid, boundary, and expected-failure triplet | Warning |
| TST-003 | Weak assertion | Test checks execution or snapshots but not intended semantics | Behavioural or invariant assertion | Error |
| TST-004 | Overfitted fixture | Test passes only for one name, order, mesh, or backend example | Metamorphic or parameterised cases | Warning |
| TST-005 | Mock-reality divergence | Mocked backend behaviour differs from the real backend | Contract or integration test | Error |
| TST-006 | Regression-baseline contamination | Agent updates expected output to match an unexplained change | Independent review of golden-data changes | Fatal |
| TST-007 | Missing expected failure | Invalid input succeeds or fails for the wrong reason | Error-code and diagnostic assertion | Error |

### EVD — Evidence errors

| ID | Error | Typical signal | Required control | Default severity |
|---|---|---|---|---|
| EVD-001 | Unsupported completion claim | Completion is reported without inspectable evidence | Evidence package requirement | Error |
| EVD-002 | Unexecuted-test claim | Test existence or inferred success is reported as execution | Captured command and result | Error |
| EVD-003 | Selective evidence | Failing, skipped, or irrelevant results are omitted | Complete test summary | Error |
| EVD-004 | Untraceable evidence | Result cannot be tied to input, commit, version, or environment | Provenance metadata | Error |
| EVD-005 | Authority laundering | Agent-generated explanation is cited as independent confirmation | Independent source or oracle requirement | Error |

### ENV — Environment errors

| ID | Error | Typical signal | Required control | Default severity |
|---|---|---|---|---|
| ENV-001 | Undeclared environment dependency | Success depends on a local tool, path, variable, or credential | Environment manifest | Error |
| ENV-002 | Version drift | Different package or solver versions change behaviour | Locking and compatibility checks | Error |
| ENV-003 | Platform-specific success | Change works on one operating system or architecture only | Required platform matrix | Warning |
| ENV-004 | Hidden state dependency | Cache, prior output, or working-tree residue affects success | Clean-environment execution | Error |
| ENV-005 | Permission-dependent behaviour | Validation passes only with broader authority than production allows | Least-privilege test | Error |

### REP — Reproducibility errors

| ID | Error | Typical signal | Required control | Default severity |
|---|---|---|---|---|
| REP-001 | Non-deterministic canonicalisation | Equivalent input produces different canonical identities or ordering | Repeated normalised comparison | Fatal |
| REP-002 | Non-idempotent transformation | Reapplying the same operation changes the model again | Idempotence test | Error |
| REP-003 | Uncontrolled numerical variance | Solver result exceeds declared tolerance across equivalent runs | Numerical tolerance policy | Error |
| REP-004 | Non-semantic diff instability | Generated files change without semantic input changes | Deterministic formatting and normalisation | Warning |
| REP-005 | Irreproducible evidence | A reported result cannot be regenerated from recorded provenance | Replay test | Error |

### GOV — Governance and authority errors

| ID | Error | Typical signal | Required control | Default severity |
|---|---|---|---|---|
| GOV-001 | Missing required approval | High-risk change proceeds without its designated authority | Risk-based acceptance gate | Fatal |
| GOV-002 | Self-approval | The producing agent is the only acceptance authority for a high-risk change | Independent acceptance step | Error |
| GOV-003 | Protected-policy modification | Agent weakens validation, permissions, or acceptance criteria to make a change pass | Protected controls and explicit approval | Fatal |
| GOV-004 | Irreversible action without authority | Migration, deletion, publication, or destructive operation exceeds granted scope | Explicit action-specific authorisation | Fatal |
| GOV-005 | Unrecorded exception | A validation failure is waived without rationale, owner, and expiry | Exception record | Error |

### DOC — Documentation and contract errors

| ID | Error | Typical signal | Required control | Default severity |
|---|---|---|---|---|
| DOC-001 | Documentation drift | Public behaviour and documentation disagree | Documentation contract test or review | Warning |
| DOC-002 | Example drift | Published example no longer parses, maps, or runs | Executable documentation | Error |
| DOC-003 | False capability documentation | Documentation promises unsupported backend behaviour | Capability matrix verification | Error |
| DOC-004 | Missing migration guidance | Rename or deprecation changes behaviour without a supported transition | Migration documentation gate | Warning |
| DOC-005 | Stale provenance | Reference source or backend version is absent or obsolete | Source and version metadata | Warning |

## Severity escalation

The default severity should be escalated when any of the following applies:

- the error can silently change scientific interpretation or solver output;
- stable identity or a persisted reference may be corrupted;
- the error crosses a backend boundary without a loss diagnostic;
- a golden result or acceptance criterion was modified to conceal failure;
- the change is irreversible or affects migration;
- a required approval or protected validation control was bypassed.

A warning becomes an error when acceptance depends on resolving it. An error becomes fatal when continuing could produce a plausible but scientifically invalid model or irreversibly corrupt persisted meaning.

## Validation stage mapping

| Stage | Priority families |
|---|---|
| Request and task contract | INT, SCP, ASM, GOV |
| Authoring syntax | INT, ASM, IDN, DOC |
| Canonicalisation | SEM, IDN, REP |
| Adapter compatibility | MAP, SEM, ENV |
| Backend generation | MAP, SEM, REP |
| Solver execution | SEM, ENV, REP |
| Test and review | TST, EVD, GOV |
| Release and migration | IDN, MAP, EVD, GOV, DOC |

## Minimum reporting rule

A validation report should not state only that a test failed. Where classification is possible, it should include:

```text
ruleId: ontology.identity.alias-unique
errorId: IDN-002
severity: error
entityId: material:oxygen
claim: alias "O2" resolves uniquely within the model scope
expected: exactly one stable identity
observed: two candidate identities
evidence: resolver trace and canonical model snapshot
```

## Taxonomy maintenance policy

- IDs are stable once published.
- Meanings may be clarified without changing the underlying failure class.
- Removed classes are deprecated, not silently reassigned.
- New classes use the next available number within their family.
- Adapter-specific subtypes may extend a core class but must retain the core ID.
- Taxonomy changes require examples and an explanation of their effect on existing validation rules.
