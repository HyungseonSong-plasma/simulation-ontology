# ADR 0006 — Semantic Preservation Across Backends

**Status:** Accepted  
**Date:** 2026-08-20  
**Scope:** Backend mapping architecture

## Context

Cross-backend round-trip testing showed that simulation backends differ in how much semantic information their native formats can represent explicitly.

For example, a SOL model may retain a thermal-conductivity value together with its physical dimension and unit, while a generated backend input may serialize only the numeric value because the backend assumes units by convention or does not encode them in that particular input field.

Treating the generated backend representation as the authoritative semantic model would therefore cause information loss.

## Decision

### D4 — Semantic Preservation Across Weak Backends

> **A backend's inability to represent all SOL semantic information in its native format SHALL NOT cause that information to be discarded from the SOL representation.**

Backend serialization is a projection of the semantic model, not the semantic model itself.

```text
SOL semantic model
        │
        │ backend projection
        ▼
Native backend representation
```

The projection MAY be lossy when the backend lacks equivalent constructs, but the authoritative SOL graph MUST preserve the richer semantics unless an explicit model transformation changes them.

## Example

Conceptually:

```text
SOL
ThermalConductivity
  value     = 400
  unit      = W/(m·K)
  dimension = M L T^-3 Θ^-1

        ↓ backend generation

MOOSE-like native input
thermal_conductivity = 400
```

The absence of explicit unit/dimension information in the generated representation does not remove that information from SOL.

## Consequences

1. SOL remains the semantic source of truth during backend generation.
2. Backend adapters must distinguish unsupported semantics from absent semantics.
3. Round-trip import/export cannot automatically be assumed lossless.
4. Adapters SHOULD report semantic information that cannot be represented by the target backend when that loss can affect interpretation, validation, reproducibility, or regeneration.
5. Backend-specific syntax MUST NOT redefine Core semantic identity merely because it is more restrictive or less expressive.
6. Future adapter capability metadata may describe which SOL constructs are losslessly representable by each backend.

## Rationale

MOOSE, COMSOL, and Ansys expose different native abstractions and unit/value mechanisms. A framework intended to span these systems cannot define its semantics as the intersection of what every backend can serialize. Doing so would make the least expressive backend determine the ontology's expressive power.

Instead, SOL defines the semantic model and backend adapters project that model into backend-native forms.

## Deferred decisions

This ADR does not decide:

- the exact warning/error policy for lossy mappings;
- how adapter capabilities are declared;
- whether unsupported semantic information is embedded in sidecar metadata;
- round-trip equivalence criteria;
- provenance requirements for generated backend artifacts.
