# simulation-ontology

A solver-independent simulation ontology and semantic runtime for multiphysics backends.

## Purpose

Simulation Ontology (SOL) provides a common semantic layer for describing simulation intent independently of any particular solver or backend object model. Its goal is to make simulation models, constraints, mappings, and backend requirements explicit and machine-interpretable so that the same domain model can be validated, reasoned about, and translated toward different simulation frameworks without embedding solver-native concepts into the Core.

SOL is intended to support reusable and portable simulation knowledge: domain ontologies can describe what a model means, the Core can validate whether that meaning is internally consistent and representable for a target, and independently developed adapters can realize the resulting plan in concrete backends. This enables applications such as solver-independent model authoring, automated model generation, cross-backend mapping, reproducible simulation workflows, adapter/tool development, and higher-level AI/automation systems that need structured simulation semantics rather than backend-specific input syntax.

## Project baseline

- Architecture / implementation plan: [`docs/plans/core-simulation-ontology-v0.1-implementation-plan.md`](docs/plans/core-simulation-ontology-v0.1-implementation-plan.md)
- Product boundary and adapter roadmap: [`docs/plans/product-boundary-and-adapter-roadmap.md`](docs/plans/product-boundary-and-adapter-roadmap.md)
- Versioning and compatibility policy: [`docs/plans/versioning-and-compatibility-policy.md`](docs/plans/versioning-and-compatibility-policy.md)
- Implementation notes: [`docs/implementation/`](docs/implementation/)
- Current implementation milestone: [M0.1 — Semantic Core Bootstrap](../../issues/2)

## Product direction

SOL is intended to be a language-neutral platform rather than a Rust-only library. The semantic Core is implemented in Rust, while official user-facing language support is introduced progressively during v0.x:

| Release stage | Official support direction |
|---|---|
| v0.1 | Rust Core + `sol` CLI |
| v0.2 | TypeScript SDK promoted after integration validation |
| v0.3 | Python SDK promoted after scientific/automation validation |
| v1.0 target | Rust + CLI + TypeScript + Python as stable Tier-1 interfaces |

Experimental SDKs may appear before promotion and do not carry stable compatibility guarantees.

The long-term public distributions are planned as:

```text
Cargo   -> sol-core
npm     -> @simulation-ontology/core
PyPI    -> simulation-ontology
binary  -> sol
```

Internal Rust crates remain implementation modules behind a public facade rather than separate concepts that every SOL user must understand.

## Public contract

The normative external boundary is a language-neutral Canonical Public Contract represented through versioned JSON Schema / canonical JSON, not the internal Rust struct layout. Rust, TypeScript, Python, CLI output, and adapter payloads must preserve the same canonical SOL semantics while allowing language-native API ergonomics.

Public Contract, Adapter Protocol, ontology packages, and external adapters have independent compatibility/version axes. See the versioning policy for the breaking-change rules.

## Adapter boundary

This repository owns the language-neutral Adapter Protocol, MockAdapter reference conformance implementation, conformance tooling, and adapter authoring guidance. Real solver adapters are separate projects and release independently from the Core.

Reference adapter roadmap:

```text
v0.1  -> MOOSE
v0.2  -> Zapdos / CRANE
v0.3+ -> selected additional open-source backends
```

COMSOL and Ansys are supported as external/community adapter targets rather than required official integrations while repeatable licensed CI environments are unavailable. Licensed developers should be able to implement them against the published protocol and conformance suite.

For v0.x, the default local adapter transport is canonical JSON using JSON-RPC over stdio. Solver-native object models and vendor dependencies remain outside the Core repository.

## Current implementation strategy

Development is CI-first and counterexample-driven. The Core workspace currently focuses on semantic data models, identity/resolution, constraints/QRC, mapping/effect/plan semantics, evaluation lifecycle, BackendTarget resolution, MockAdapter behavior, canonical fixtures, and `sol-cli` output.

Real backend execution is not a prerequisite for Core unit/contract validation. MockAdapter is the executable reference for adapter-protocol conformance inside this repository; physical correctness and solver-native integration belong to each real adapter project.
