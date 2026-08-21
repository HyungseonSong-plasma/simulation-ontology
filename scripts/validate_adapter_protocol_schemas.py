#!/usr/bin/env python3
"""Validate the published Adapter Protocol 0.1 schema/fixture boundary."""

from __future__ import annotations

import json
from pathlib import Path

from jsonschema import Draft202012Validator, RefResolver

ROOT = Path(__file__).resolve().parents[1]
SCHEMA_DIR = ROOT / "schemas" / "adapter-protocol" / "0.1"
FIXTURE_DIR = ROOT / "fixtures" / "adapter-protocol" / "0.1"
COUNTEREXAMPLE_DIR = ROOT / "fixtures" / "counterexamples"
DIALECT = "https://json-schema.org/draft/2020-12/schema"
TRANSPORT_KEYS = {
    "jsonrpc",
    "request_id",
    "stdio_frame",
    "process_id",
    "retry_policy",
    "reconnect_policy",
    "transport",
}


def load_json(path: Path):
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def make_validator(schema_name: str) -> Draft202012Validator:
    schema_path = SCHEMA_DIR / schema_name
    schema = load_json(schema_path)
    if schema.get("$schema") != DIALECT:
        raise AssertionError(f"{schema_name}: expected Draft 2020-12 dialect")
    Draft202012Validator.check_schema(schema)
    resolver = RefResolver(base_uri=schema_path.resolve().as_uri(), referrer=schema)
    return Draft202012Validator(schema, resolver=resolver)


def require_valid(schema_name: str, fixture: Path, payload=None) -> None:
    validator = make_validator(schema_name)
    value = load_json(fixture) if payload is None else payload
    errors = sorted(validator.iter_errors(value), key=lambda error: list(error.path))
    if errors:
        details = "\n".join(f"  - {error.json_path}: {error.message}" for error in errors)
        raise AssertionError(f"{fixture.relative_to(ROOT)} should satisfy {schema_name}:\n{details}")


def require_invalid(schema_name: str, fixture: Path) -> None:
    validator = make_validator(schema_name)
    errors = list(validator.iter_errors(load_json(fixture)))
    if not errors:
        raise AssertionError(
            f"{fixture.relative_to(ROOT)} should be structurally rejected by {schema_name}"
        )


def walk_keys(value):
    if isinstance(value, dict):
        for key, child in value.items():
            yield key
            yield from walk_keys(child)
    elif isinstance(value, list):
        for child in value:
            yield from walk_keys(child)


def require_transport_independent(path: Path) -> None:
    leaked = sorted(set(walk_keys(load_json(path))) & TRANSPORT_KEYS)
    if leaked:
        raise AssertionError(
            f"{path.relative_to(ROOT)} contains transport-specific keys: {', '.join(leaked)}"
        )


def main() -> None:
    schema_files = sorted(SCHEMA_DIR.glob("*.schema.json"))
    expected_schemas = {
        "shared.schema.json",
        "bootstrap.schema.json",
        "adapter-description.schema.json",
        "validate-plan-request.schema.json",
        "validate-plan-response.schema.json",
        "execute-plan-request.schema.json",
        "execute-plan-response.schema.json",
        "protocol-failure.schema.json",
    }
    actual_schemas = {path.name for path in schema_files}
    if actual_schemas != expected_schemas:
        raise AssertionError(
            f"Adapter Protocol schema set mismatch: expected={sorted(expected_schemas)}, "
            f"actual={sorted(actual_schemas)}"
        )

    for schema_path in schema_files:
        schema = load_json(schema_path)
        if schema.get("$schema") != DIALECT:
            raise AssertionError(f"{schema_path.name}: expected Draft 2020-12 dialect")
        Draft202012Validator.check_schema(schema)
        require_transport_independent(schema_path)

    positive = [
        ("bootstrap.schema.json", FIXTURE_DIR / "version-safe-bootstrap.json"),
        ("adapter-description.schema.json", FIXTURE_DIR / "dual-compatible-description.json"),
        (
            "adapter-description.schema.json",
            FIXTURE_DIR / "schema-additive-extension-description.json",
        ),
        ("validate-plan-request.schema.json", FIXTURE_DIR / "validate-plan-accepted-request.json"),
        ("validate-plan-response.schema.json", FIXTURE_DIR / "validate-plan-accepted-response.json"),
        ("validate-plan-response.schema.json", FIXTURE_DIR / "validate-plan-rejected-response.json"),
        ("validate-plan-response.schema.json", FIXTURE_DIR / "validate-plan-unavailable-response.json"),
        ("execute-plan-request.schema.json", FIXTURE_DIR / "execute-plan-thermal-request.json"),
        ("execute-plan-request.schema.json", FIXTURE_DIR / "execute-plan-independent-request.json"),
        ("execute-plan-response.schema.json", FIXTURE_DIR / "execute-plan-exact-response.json"),
        ("execute-plan-response.schema.json", FIXTURE_DIR / "execute-plan-degraded-response.json"),
        ("execute-plan-response.schema.json", FIXTURE_DIR / "execute-plan-unsupported-response.json"),
        ("execute-plan-response.schema.json", FIXTURE_DIR / "execute-plan-partial-response.json"),
        ("execute-plan-response.schema.json", FIXTURE_DIR / "execute-plan-unavailable-response.json"),
        (
            "execute-plan-response.schema.json",
            FIXTURE_DIR / "execute-plan-authoritative-rejection-response.json",
        ),
        (
            "protocol-failure.schema.json",
            FIXTURE_DIR / "protocol-failure-adapter-protocol-incompatible.json",
        ),
        (
            "protocol-failure.schema.json",
            FIXTURE_DIR / "protocol-failure-public-contract-incompatible.json",
        ),
        (
            "protocol-failure.schema.json",
            FIXTURE_DIR / "protocol-failure-compatibility-missing.json",
        ),
        (
            "protocol-failure.schema.json",
            FIXTURE_DIR / "protocol-failure-malformed-bootstrap.json",
        ),
        (
            "protocol-failure.schema.json",
            FIXTURE_DIR / "protocol-failure-invalid-request.json",
        ),
        (
            "protocol-failure.schema.json",
            FIXTURE_DIR / "protocol-failure-validate-operational.json",
        ),
        (
            "protocol-failure.schema.json",
            FIXTURE_DIR / "protocol-failure-execute-before-side-effect.json",
        ),
        (
            "protocol-failure.schema.json",
            FIXTURE_DIR / "protocol-failure-execute-ambiguous.json",
        ),
    ]
    for schema_name, fixture in positive:
        require_valid(schema_name, fixture)

    structurally_invalid = [
        (
            "validate-plan-request.schema.json",
            COUNTEREXAMPLE_DIR / "adapter-protocol-schema-missing-required-plan.json",
        ),
        (
            "validate-plan-response.schema.json",
            COUNTEREXAMPLE_DIR / "adapter-protocol-schema-wrong-preflight-type.json",
        ),
        (
            "protocol-failure.schema.json",
            COUNTEREXAMPLE_DIR / "adapter-protocol-failure-retryable-field.json",
        ),
        (
            "protocol-failure.schema.json",
            COUNTEREXAMPLE_DIR / "adapter-protocol-failure-lifecycle-conflation.json",
        ),
    ]
    for schema_name, fixture in structurally_invalid:
        require_invalid(schema_name, fixture)

    # These shapes are structurally Protocol-shaped but intentionally violate
    # runtime/semantic invariants. Schema success is not semantic conformance.
    schema_valid_semantic_counterexamples = [
        (
            "validate-plan-response.schema.json",
            COUNTEREXAMPLE_DIR / "adapter-protocol-inconsistent-accepted-response.json",
        ),
        (
            "execute-plan-response.schema.json",
            COUNTEREXAMPLE_DIR / "adapter-protocol-execution-dependency-violation.json",
        ),
        (
            "execute-plan-response.schema.json",
            COUNTEREXAMPLE_DIR / "adapter-protocol-execution-aggregate-effect-mismatch.json",
        ),
        (
            "execute-plan-response.schema.json",
            COUNTEREXAMPLE_DIR / "adapter-protocol-execution-opaque-reference-semantic-identity.json",
        ),
    ]
    for schema_name, fixture in schema_valid_semantic_counterexamples:
        require_valid(schema_name, fixture)

    semantic_break = load_json(
        COUNTEREXAMPLE_DIR / "adapter-protocol-semantic-redefinition-same-syntax.json"
    )
    require_valid(
        "validate-plan-response.schema.json",
        COUNTEREXAMPLE_DIR / "adapter-protocol-semantic-redefinition-same-syntax.json",
        semantic_break["syntax_fixture"],
    )
    if semantic_break["published_meaning"] == semantic_break["incompatible_redefinition"]:
        raise AssertionError("semantic break fixture must model changed normative meaning")

    # Canonical protocol fixtures are transport-independent. Public Contract payloads may
    # legitimately contain semantic `id` fields, so only transport-qualified keys are banned.
    for fixture in sorted(FIXTURE_DIR.glob("*.json")):
        require_transport_independent(fixture)

    # Reused Public Contract DTOs must stay references, not protocol-local copies.
    reusable_schema_sources = "\n".join(
        (SCHEMA_DIR / name).read_text(encoding="utf-8")
        for name in [
            "shared.schema.json",
            "validate-plan-request.schema.json",
            "validate-plan-response.schema.json",
            "execute-plan-request.schema.json",
            "execute-plan-response.schema.json",
        ]
    )
    required_refs = [
        "../../public-contract/0.1/backend-target.schema.json",
        "../../public-contract/0.1/mapping-plan.schema.json",
        "../../public-contract/0.1/shared.schema.json#/$defs/diagnostic",
        "../../public-contract/0.1/shared.schema.json#/$defs/realizationEffect",
    ]
    for reference in required_refs:
        if reference not in reusable_schema_sources:
            raise AssertionError(f"missing Public Contract schema reuse reference: {reference}")

    print(
        "Adapter Protocol 0.1 publication schema gate passed: "
        f"{len(expected_schemas)} schemas, {len(positive)} positive fixtures, "
        f"{len(structurally_invalid)} structural negatives, "
        f"{len(schema_valid_semantic_counterexamples) + 1} semantic-boundary fixtures"
    )


if __name__ == "__main__":
    main()
