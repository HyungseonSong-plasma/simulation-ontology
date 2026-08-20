# simulation-ontology

A solver-independent simulation ontology for multiphysics backends.

## Project baseline

- Architecture / implementation plan: [`docs/plans/core-simulation-ontology-v0.1-implementation-plan.md`](docs/plans/core-simulation-ontology-v0.1-implementation-plan.md)
- Current implementation milestone: [M0.1 — Semantic Core Bootstrap](../../issues/2)

## Current strategy

The project uses a Rust language-independent semantic Core, a TypeScript/React primary client layer, and polyglot backend adapters aligned with native solver ecosystems. Development is CI-first and counterexample-driven. Until M0.1 is stable, implementation is limited to the Rust Core, architecture fixtures, Mock Adapter, BackendTarget resolver, and `sol-cli`; real backend adapters and TypeScript binding remain out of scope.
