#!/usr/bin/env python3
"""Validate published SOL Public Contract schema/fixture boundaries."""

from __future__ import annotations

import json
from pathlib import Path

from jsonschema import Draft202012Validator, RefResolver

ROOT = Path(__file__).resolve().parents[1]
SCHEMA_DIR = ROOT / "schemas" / "public-contract" / "0.1"
FIXTURE_DIR = ROOT / "fixtures" / "public-contract" / "0.1"
SCHEMA_DIR_V02 = ROOT / "schemas" / "public-contract" / "0.2"
FIXTURE_DIR_V02 = ROOT / "fixtures" / "public-contract" / "0.2"
COUNTEREXAMPLE_DIR = ROOT / "fixtures" / "counterexamples"
DIALECT = "https://json-schema.org/draft/2020-12/schema"


def load_json(path: Path):
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def make_validator(schema_dir: Path, schema_name: str) -> Draft202012Validator:
    schema_path = schema_dir / schema_name
    schema = load_json(schema_path)
    if schema.get("$schema") != DIALECT:
        raise AssertionError(f"{schema_name}: expected Draft 2020-12 dialect")
    Draft202012Validator.check_schema(schema)
    resolver = RefResolver(base_uri=schema_path.resolve().as_uri(), referrer=schema)
    return Draft202012Validator(schema, resolver=resolver)


def require_valid_in(schema_dir: Path, schema_name: str, fixture: Path) -> None:
    validator = make_validator(schema_dir, schema_name)
    errors = sorted(validator.iter_errors(load_json(fixture)), key=lambda error: list(error.path))
    if errors:
        details = "\n".join(f"  - {error.json_path}: {error.message}" for error in errors)
        raise AssertionError(f"{fixture.relative_to(ROOT)} should satisfy {schema_name}:\n{details}")


def require_valid(schema_name: str, fixture: Path) -> None:
    require_valid_in(SCHEMA_DIR, schema_name, fixture)


def require_invalid(schema_name: str, fixture: Path) -> None:
    validator = make_validator(SCHEMA_DIR, schema_name)
    errors = list(validator.iter_errors(load_json(fixture)))
    if not errors:
        raise AssertionError(
            f"{fixture.relative_to(ROOT)} should be structurally rejected by {schema_name}"
        )


def validate_schema_set(schema_dir: Path, expected: set[str]) -> None:
    schema_files = sorted(schema_dir.glob("*.schema.json"))
    actual = {path.name for path in schema_files}
    if actual != expected:
        raise AssertionError(
            f"{schema_dir.relative_to(ROOT)} schema set mismatch: "
            f"expected={sorted(expected)}, actual={sorted(actual)}"
        )
    for schema_path in schema_files:
        schema = load_json(schema_path)
        if schema.get("$schema") != DIALECT:
            raise AssertionError(f"{schema_path.name}: expected Draft 2020-12 dialect")
        Draft202012Validator.check_schema(schema)


def main() -> None:
    schema_files = sorted(SCHEMA_DIR.glob("*.schema.json"))
    if not schema_files:
        raise AssertionError("no Public Contract 0.1 schemas found")
    for schema_path in schema_files:
        schema = load_json(schema_path)
        if schema.get("$schema") != DIALECT:
            raise AssertionError(f"{schema_path.name}: expected Draft 2020-12 dialect")
        Draft202012Validator.check_schema(schema)

    positive = [
        ("simulation.schema.json", FIXTURE_DIR / "thermal-simulation.json"),
        ("validation-report.schema.json", FIXTURE_DIR / "thermal-validation-report.json"),
        ("mapping-claims.schema.json", FIXTURE_DIR / "thermal-mapping-claims.json"),
        ("mapping-plan.schema.json", FIXTURE_DIR / "thermal-mapping-plan.json"),
        ("backend-target.schema.json", FIXTURE_DIR / "thermal-backend-target.json"),
        ("realization-effect.schema.json", FIXTURE_DIR / "thermal-realization-effect.json"),
        ("evaluation-result.schema.json", FIXTURE_DIR / "thermal-evaluation.json"),
    ]
    for schema_name, fixture in positive:
        require_valid(schema_name, fixture)

    structurally_invalid = [
        (
            "simulation.schema.json",
            COUNTEREXAMPLE_DIR / "public-contract-schema-missing-version.json",
        ),
        (
            "validation-report.schema.json",
            COUNTEREXAMPLE_DIR / "public-contract-malformed-diagnostic.json",
        ),
        (
            "evaluation-result.schema.json",
            COUNTEREXAMPLE_DIR / "public-contract-schema-invalid-evaluation-status.json",
        ),
    ]
    for schema_name, fixture in structurally_invalid:
        require_invalid(schema_name, fixture)

    # These fixtures intentionally satisfy JSON Schema but violate semantic/runtime
    # rules. This executable boundary proves that schema success is not SOL semantic
    # validity.
    schema_valid_semantic_counterexamples = [
        (
            "simulation.schema.json",
            COUNTEREXAMPLE_DIR / "public-contract-unresolved-reference.json",
        ),
        (
            "simulation.schema.json",
            COUNTEREXAMPLE_DIR / "public-contract-invalid-scope-membership.json",
        ),
        (
            "simulation.schema.json",
            COUNTEREXAMPLE_DIR / "public-contract-duplicate-id.json",
        ),
        (
            "mapping-plan.schema.json",
            COUNTEREXAMPLE_DIR / "public-contract-plan-cycle.json",
        ),
        (
            "evaluation-result.schema.json",
            COUNTEREXAMPLE_DIR / "public-contract-evaluation-mismatch-false-pass.json",
        ),
        (
            "backend-target.schema.json",
            COUNTEREXAMPLE_DIR / "public-contract-backend-native-leakage.json",
        ),
    ]
    for schema_name, fixture in schema_valid_semantic_counterexamples:
        require_valid(schema_name, fixture)

    # Public Contract 0.2 is additive: retain every 0.1 gate above and validate the
    # explicit realization subset independently.
    validate_schema_set(
        SCHEMA_DIR_V02,
        {
            "shared.schema.json",
            "mapping-plan.schema.json",
            "backend-target.schema.json",
            "realization-spec.schema.json",
        },
    )

    positive_v02 = [
        ("mapping-plan.schema.json", FIXTURE_DIR_V02 / "thermal-mapping-plan.json"),
        ("backend-target.schema.json", FIXTURE_DIR_V02 / "thermal-backend-target.json"),
        ("realization-spec.schema.json", FIXTURE_DIR_V02 / "thermal-realization-spec.json"),
        (
            "realization-spec.schema.json",
            FIXTURE_DIR_V02 / "thermal-realization-spec-alternate-values.json",
        ),
    ]
    for schema_name, fixture in positive_v02:
        require_valid_in(SCHEMA_DIR_V02, schema_name, fixture)

    # Resolve every external $ref with a minimal independent payload as well as the
    # published thermal fixtures. Semantic invariants beyond JSON shape remain Rust
    # contract/conformance responsibilities.
    smoke_payloads = {
        "mapping-plan.schema.json": {
            "public_contract_version": "0.2",
            "actions": [{"id": "smoke.action", "dependencies": []}],
        },
        "backend-target.schema.json": {
            "public_contract_version": "0.2",
            "target": "mock",
            "required_capabilities": [],
        },
        "realization-spec.schema.json": {
            "public_contract_version": "0.2",
            "ontology_version": "0.1",
            "source_model": "model.smoke",
            "entities": [
                {
                    "id": "domain.smoke",
                    "kind": "spatial_model",
                    "semantic_type": "Domain",
                    "parameters": [],
                }
            ],
            "scopes": [{"id": "scope.smoke", "members": ["domain.smoke"]}],
            "relations": [],
            "action_bindings": [
                {
                    "action_id": "smoke.action",
                    "subjects": [{"subject_kind": "entity", "id": "domain.smoke"}],
                    "scopes": ["scope.smoke"],
                }
            ],
        },
    }
    for schema_name, payload in smoke_payloads.items():
        validator = make_validator(SCHEMA_DIR_V02, schema_name)
        errors = sorted(validator.iter_errors(payload), key=lambda error: list(error.path))
        if errors:
            details = "\n".join(f"  - {error.json_path}: {error.message}" for error in errors)
            raise AssertionError(f"0.2 schema smoke failed for {schema_name}:\n{details}")

    print(
        "Public Contract schema validation passed: "
        f"0.1={len(positive)} positive/{len(structurally_invalid)} structural-negative/"
        f"{len(schema_valid_semantic_counterexamples)} semantic-boundary fixtures; "
        f"0.2 realization subset=4 schemas/{len(positive_v02)} thermal fixtures + ref-resolution smoke"
    )


if __name__ == "__main__":
    main()
