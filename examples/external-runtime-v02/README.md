# External Adapter Runtime 0.2 Integration Example

This standalone Cargo project is intentionally outside the repository workspace. It consumes the public `sol-adapter-runtime` API and proves the M0.9 solver-neutral runtime path against an external adapter command.

The positive path is:

```text
explicit registration
  -> external process launch
  -> describe_adapter bootstrap
  -> Protocol 0.2 + Public Contract 0.2 live compatibility
  -> target/capability projection
  -> deterministic profile-aware selection
  -> ValidatePlanRequestV02(target + plan + realization_spec)
  -> ExecutePlanRequestV02(target + plan + realization_spec)
  -> shutdown
```

The test matrix also verifies that a live 0.1 adapter is not eligible for an explicit 0.2 request, two eligible 0.2 adapters remain ambiguous, bootstrap/process failure is not converted into semantic success, and execute response loss does not authorize transport replay.

The adapter binaries and canonical thermal Protocol 0.2 request fixture are supplied through environment variables so the consumer remains independent of workspace-private fixture paths:

- `SOL_RUNTIME_V02_ADAPTER`
- `SOL_RUNTIME_V01_ADAPTER`
- `SOL_RUNTIME_V02_FIXTURE`

This is runtime/protocol interoperability evidence only. It is not solver-native physical or numerical V&V, and it does not introduce MOOSE, COMSOL, Ansys, or any other solver dependency into SOL Core.
