# SOL Versioning and Compatibility Policy

**Status:** Planning decision  
**Date:** 2026-08-21  
**Scope:** Runtime releases, public contracts, adapter protocol, ontology packages, compatibility, and breaking-change classification

## Purpose

This document records the versioning and compatibility policy for SOL. SOL is a language-neutral platform with independently evolving runtime distributions, public semantic contracts, adapter interoperability contracts, ontology packages, and external adapters. Compatibility therefore SHALL NOT be inferred from a single package version.

## 1. Version axes

SOL distinguishes the following version axes.

| Axis | Example | Meaning |
|---|---|---|
| SOL Runtime Release | `0.3.0` | Coordinated release of Core, CLI, and official SDK distributions available in that release |
| Public Contract | `0.2` | Version of canonical JSON/public semantic representation |
| Adapter Protocol | `0.1` | Version of Core-to-adapter interoperability contract |
| Ontology Package | `thermal 0.4.1` | Independently versioned ontology/schema content |
| Adapter implementation | `moose-adapter 0.5.0` | Independently released external/reference adapter |

Runtime version, Public Contract version, Adapter Protocol version, ontology package version, and adapter implementation version are distinct concepts.

## 2. Official runtime release train

Official SOL distributions SHALL share a coordinated SOL release version when they are included in a release train.

Conceptually:

```text
SOL 0.3.0
├── Rust facade/Core distribution       0.3.0
├── sol CLI                             0.3.0
├── TypeScript SDK                      0.3.0
└── Python SDK                          0.3.0
```

An SDK that has not yet reached its planned support tier MAY be absent or explicitly marked Experimental in a release. Experimental interfaces do not receive the same compatibility guarantee as Tier-1 interfaces.

## 3. Public Contract version

The Public Contract version SHALL be independent of the SOL Runtime Release.

Multiple runtime releases MAY implement the same Public Contract version.

```text
SOL 0.3.0 -> Public Contract 0.2
SOL 0.3.1 -> Public Contract 0.2
SOL 0.4.0 -> Public Contract 0.2
```

The Public Contract governs canonical public semantic representations such as Model-facing DTOs, ValidationReport, Diagnostic, MappingPlan, BackendTarget-facing public data, EvaluationResult, and other canonical JSON/API boundary objects adopted by the project.

Internal Rust structs and crate organization SHALL NOT define Public Contract compatibility by themselves.

## 4. Adapter Protocol version

The Adapter Protocol SHALL have an independent version axis from both Runtime and Public Contract.

Adapters SHALL declare protocol compatibility explicitly. Compatibility SHALL NOT be inferred from the adapter package version.

Conceptually:

```text
adapter: sol-adapter-moose
adapter_version: 0.5.0
supported_protocol: >=0.1,<0.2
```

SOL runtime and adapter interoperability requires an explicitly compatible protocol range. An adapter package version such as `1.4.0` has no interoperability meaning unless its supported Adapter Protocol range is also known.

Protocol negotiation and incompatibility SHALL be detected before plan execution where the required information is available.

## 5. Ontology package versioning

Ontology packages SHALL be independently versioned.

Example:

```text
sol-ontology-core     0.5.0
sol-ontology-thermal  0.3.0
sol-ontology-plasma   0.2.0
```

Ontology package evolution SHALL NOT require a Runtime release solely because ontology content changed. Packages SHOULD declare relevant dependencies and compatibility requirements, including dependencies on other ontology packages and Public Contract versions where applicable.

## 6. Adapter release independence

Real adapters live in separate projects and SHALL be versioned independently from SOL Core.

Example:

```text
SOL Runtime       0.3.0
Public Contract   0.2
Adapter Protocol  0.1

MOOSE Adapter     0.5.0 -> supports protocol 0.1
Zapdos Adapter    0.2.0 -> supports protocol 0.1
```

Compatibility SHALL be established by explicit contracts and ranges, not matching release numbers.

## 7. v0.x compatibility policy

SOL v0.x uses SemVer-style release numbering, but v1.0-level long-term compatibility is not implied.

Normative policy:

- PATCH releases within the same v0.x minor line SHALL NOT intentionally introduce breaking changes to declared supported contracts.
- MINOR releases within v0.x MAY contain breaking changes where architecture/product maturation requires them.
- Every intentional breaking change in v0.x SHALL be explicitly documented with affected contract, previous behavior, new behavior, and migration path where practical.
- Compatibility is guaranteed only for interfaces/contracts explicitly declared supported for the relevant release and support tier.
- Experimental interfaces MAY change without backward-compatibility guarantees, but changes SHOULD still be documented when they affect active development consumers.

After v1.0, the project SHOULD apply strict semantic-versioning expectations to declared stable public interfaces: MAJOR for breaking changes, MINOR for backward-compatible capability additions, and PATCH for backward-compatible fixes.

## 8. General definition of a breaking change

A change is breaking when a previously conforming consumer, SDK client, adapter, ontology package, canonical model, or other declared compatible participant can no longer be interpreted or executed with the same normative meaning under the declared compatibility range.

Breaking changes are classified as either structural or semantic.

### 8.1 Structural breaking change

Examples include:

- removing or renaming a required public field;
- changing a public field type incompatibly;
- removing a public SDK function;
- making an optional SDK argument required;
- removing an Adapter Protocol method;
- adding a required protocol field that older conforming peers cannot supply;
- changing an import/module path promised as stable.

### 8.2 Semantic breaking change

A syntactically unchanged contract MAY still be breaking when its normative meaning changes.

Examples include:

- changing the meaning of `BLOCKED`, `INDETERMINATE`, or another canonical state;
- changing constraint interpretation while preserving the same serialized shape;
- changing canonical identity/reference semantics;
- changing a diagnostic code to denote a materially different condition;
- changing MappingPlan/effect compatibility semantics such that previously equivalent inputs produce a different normative result;
- changing an ontology concept's normative meaning while retaining the same canonical identity.

Rule:

```text
same syntax + incompatible normative meaning = breaking change
```

## 9. Breaking Change Classification Table

| Surface | Breaking examples | Normally non-breaking examples |
|---|---|---|
| Public Contract | required field removal/rename; incompatible type change; enum/state removal; normative state meaning change; canonical identity or normalization semantics change | optional field addition where unknown optional fields are permitted; new backward-compatible object/type |
| Rust SDK | removal/rename of stable public API; incompatible signature/return/error behavior change | additive API; implementation refactor hidden behind facade |
| TypeScript SDK | stable export removal; required argument change; synchronous API changed to async; incompatible return semantics | additive optional argument; new export; ergonomic helper preserving semantics |
| Python SDK | stable import/function removal; incompatible parameter/return/exception semantics | additive helper or optional parameter preserving existing behavior |
| CLI | removal/rename of stable command/flag; incompatible exit-code or canonical output semantics | additive command/flag; human-readable formatting change outside a declared machine contract |
| Adapter Protocol | method removal; required payload addition; incompatible handshake/result/comparator semantics | optional capability extension explicitly negotiable by old peers |
| Ontology Package | normative meaning change under same stable identity; removal of required concept/relation without compatibility path; incompatible constraint interpretation | additive concept/relation; rename preserving stable identity and supported alias resolution where semantics remain unchanged |

## 10. Public Contract evolution rules

The Public Contract SHOULD be designed for controlled additive evolution.

Unless a schema explicitly declares a closed object, consumers of a declared extensible canonical object SHOULD tolerate unknown optional fields. Producers SHALL NOT rely on an unknown optional field being understood unless compatibility/capability negotiation establishes that support.

The following changes SHALL be treated as breaking unless a more specific compatibility rule proves otherwise:

- required field deletion or rename;
- incompatible field type/cardinality change;
- removal of a valid enum/state value;
- incompatible change to default interpretation;
- canonical JSON normalization change that alters normative identity/equality;
- diagnostic/state semantic redefinition.

## 11. SDK compatibility

SDK compatibility includes both source/API compatibility and behavioral compatibility.

Language-specific ergonomics MAY differ, but all official SDKs for a given canonical contract SHALL preserve the same SOL semantics.

Examples:

```text
TypeScript: ValidationReport -> Promise<ValidationReport>
```

is breaking if the stable API was previously synchronous.

Changing internal Rust implementation, ownership structure, or crate decomposition is not breaking when the stable facade and canonical behavior remain compatible.

## 12. Adapter Protocol breaking rules

Adapter Protocol changes SHALL be treated more conservatively because adapters are independently released.

An incompatible protocol semantic or structural change SHALL occur only across an explicit Adapter Protocol version boundary. Runtime PATCH/MINOR implementation work SHALL NOT silently redefine an existing protocol version.

Protocol-breaking examples include:

- method removal or incompatible rename;
- new mandatory request/response data without negotiated fallback;
- incompatible handshake behavior;
- changed meaning of target resolution, effect comparison, plan validation, execution, or realization reporting;
- changed error/result semantics that invalidate a previously conforming adapter.

Backward-compatible optional capability extensions MAY remain within a protocol line when old peers can deterministically negotiate or ignore them according to the protocol contract.

## 13. Ontology breaking rules

Ontology compatibility is based on stable identity and normative semantics, not display names alone.

A rename MAY be non-breaking when:

- stable canonical identity is preserved;
- previous aliases remain resolvable for the declared compatibility period where required;
- normative semantic meaning is unchanged.

A change SHALL be treated as breaking when a concept retains its canonical identity but its normative meaning changes such that an existing valid model receives a different semantic interpretation.

Additive ontology evolution is normally non-breaking only when it does not alter the interpretation or validity of existing models under the declared compatibility range.

## 14. Compatibility outcomes

Compatibility assessment MAY distinguish:

```text
Compatible
Incompatible
Unknown
```

These are compatibility assessment outcomes, not new SOL architecture lifecycle states.

Where mapped into existing evaluation behavior:

- `Compatible` permits evaluation to continue;
- `Incompatible` is handled by the applicable existing validation/representability contract;
- `Unknown` caused by missing required external compatibility information maps to the existing `BLOCKED` lifecycle semantics;
- an undecidable result with all required evidence present follows the existing `INDETERMINATE` semantics where applicable.

No new representability class is introduced by this policy.

## 15. Compatibility testing rule

When compatibility is uncertain, the project SHOULD prefer an executable compatibility fixture over intuition.

Conceptual test:

```text
old conforming input / client / adapter
              +
new implementation
              ↓
same required canonical semantics?
```

If the answer is no under a declared compatibility range, the change is breaking unless the relevant contract explicitly permits the difference.

Golden tests, protocol conformance tests, SDK parity tests, ontology migration fixtures, and canonical-output regression tests SHOULD be used to enforce this rule.

## 16. Breaking-change documentation

Every intentional breaking change in a supported surface SHALL identify at least:

- `BREAKING CHANGE` marker;
- affected surface/contract and version;
- old behavior;
- new behavior;
- affected consumers;
- migration path or explicit statement that no automatic migration exists.

For v0.x MINOR releases, this documentation is mandatory even though breaking changes are permitted.

## 17. Current decision summary

The current policy is:

```text
Official distributions
  -> coordinated SOL release train

Public Contract
  -> independent semantic-contract version

Adapter Protocol
  -> independent interoperability version
  -> explicit compatibility negotiation/range

Ontology packages
  -> independent SemVer-style versions

Real adapters
  -> independent project versions

v0.x
  PATCH -> no intentional breaking change to supported contracts
  MINOR -> breaking change allowed only when explicitly documented

v1.0+
  MAJOR -> breaking
  MINOR -> backward-compatible addition
  PATCH -> backward-compatible fix

Breaking change
  = structural incompatibility OR semantic incompatibility
    under a declared compatibility range
```

This document is a product/versioning policy. If a future change modifies canonical Core semantics rather than merely their release/compatibility governance, the semantic change SHALL be handled through the appropriate architecture/ADR process as well as this versioning policy.
