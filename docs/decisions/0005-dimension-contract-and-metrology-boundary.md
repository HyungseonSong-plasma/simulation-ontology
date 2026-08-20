# ADR 0005 — Dimension Contract and Metrology Boundary

**Status:** Accepted  
**Date:** 2026-08-20  
**Scope:** Simulation Ontology Language v0.1

## Context

The Unit/PhysicalDimension study established that semantic quantities, physical dimensions, units, and concrete values are distinct concepts. A subsequent round-trip test used thermal conductivity across MOOSE, COMSOL, Ansys Mechanical, SOL, and QUDT to determine which dimensional/metrology responsibilities belong to SOL Core.

The test showed that backend unit semantics vary substantially. SOL must preserve dimensional semantics even when a target backend cannot serialize all of them, while avoiding ownership of a complete metrology vocabulary.

## Decision

### D1 — Dimension Contract

> **SOL Core SHALL define physical-dimensional compatibility as part of its semantic contract.**

Dimensional consistency is simulation semantics and remains valid independently of backend capabilities.

### D2 — Canonical Dimension Representation

> **Physical dimensions SHALL have a canonical machine-comparable representation based on exponents of base dimensions.**

Conceptually, a dimension may be represented as a vector over base dimensions, for example thermal conductivity as `M^1 L^1 T^-3 Θ^-1`. The exact serialization and whether a PhysicalDimension is represented as an Entity or typed structure remain implementation decisions.

### D3 — External Metrology Vocabulary Boundary

> **SOL Core SHALL NOT own or require a complete unit registry. Units MAY be resolved through an external metrology vocabulary or adapter.**

QUDT is a suitable reference integration target, but it is not a mandatory Core dependency. Other standards, registries, or application-specific unit systems may be supported through the same adapter boundary.

## Architecture

```text
                 SOL Core
                    │
          semantic contracts
                    │
        ┌───────────┴───────────┐
        ▼                       ▼
 Backend adapters        Metrology adapter
 MOOSE/COMSOL/Ansys              │
                                 ▼
                          QUDT / other registry
```

## Consequences

1. SOL validators can perform dimensional consistency checks independently of backend-native unit systems.
2. SOL can normalize MOOSE, COMSOL, Ansys, and future backend representations without adopting any backend's unit model as the Core model.
3. Unit conversion constants, aliases, symbols, offsets, and complete unit catalogues need not be maintained by SOL Core.
4. QUDT identifiers and metadata may enrich or normalize SOL models through an adapter without becoming required SOL identifiers.
5. A backend that omits unit metadata does not weaken the SOL dimensional contract.
6. The canonical dimension representation must remain machine-comparable even when no external metrology adapter is installed.

## Round-trip evidence

The reference test normalized thermal conductivity such as `400 W/(m·K)` from MOOSE, COMSOL, and Ansys into a common SOL semantic representation and mapped that representation to an external metrology vocabulary. Mapping back to each backend preserved the backend-representable subset while SOL retained the full semantic contract.

This demonstrated that QUDT can strengthen unit normalization without being a hard dependency of SOL Core.

## Deferred decisions

This ADR does not decide:

- whether `PhysicalDimension` is a graph Entity or canonical typed value;
- whether `Unit` is represented internally as an Entity, URI/reference, or adapter-owned object;
- the default metrology adapter implementation;
- the exact base-dimension identifier scheme;
- serialization syntax for dimension vectors;
- offset-unit and logarithmic-unit edge cases.
