# External Adapter Runtime 0.2 Consumer

This standalone Cargo project is the supported executable consumer surface for the accepted `sol-adapter-runtime` 0.2 API. It remains outside the workspace so downstream projects can pin and build it independently. It composes existing runtime semantics; it does not define a new SOL protocol.

## Supported invocation

```text
cargo run --manifest-path examples/external-runtime-v02/Cargo.toml -- \
  run <adapter-command> <consumer-request.json>
```

The consumer request is an operational wrapper:

```json
{
  "target": "backend-target-from-caller",
  "required_capabilities": ["capability.from.caller"],
  "plan_request": { "adapter_protocol_version": "0.2", "target": {}, "plan": {}, "realization_spec": {} }
}
```

`target` and `required_capabilities` drive runtime selection. They are never inferred from the adapter command and are not hard-coded by the generic consumer. `plan_request` is parsed by the existing Adapter Protocol 0.2 request types; the wrapper does not reinterpret canonical plan or realization semantics.

The owned path is:

```text
explicit registration
  -> external process launch
  -> mandatory describe_adapter bootstrap
  -> RuntimeContractProfile::realization_v02()
  -> live Protocol 0.2 + Public Contract 0.2 compatibility projection
  -> request-driven target/capability selection
  -> ValidatePlanRequestV02
  -> ExecutePlanRequestV02
  -> no-replay policy evidence
  -> shutdown
```

Structured JSON success output records selected adapter/profile/target, validation, execution, shutdown, and execute-response-loss replay policy. Non-compatible and ambiguous selection, protocol failure, validation rejection, execution non-completion, bootstrap/runtime failure, and shutdown failure remain non-success process outcomes.

The legacy no-argument environment-variable invocation is retained only for the original M0.9 evidence tests (`SOL_RUNTIME_V02_ADAPTER`, `SOL_RUNTIME_V01_ADAPTER`, `SOL_RUNTIME_V02_FIXTURE`). New external consumers should use `run`.

This surface owns process/session lifecycle, live compatibility discovery, selection, transport invocation, and shutdown upstream. A downstream Physics consumer should only assemble its canonical request, invoke this executable at an exact pinned revision/artifact, and decode the structured outcome. Runtime/protocol interoperability is not solver-native physical or numerical V&V.
