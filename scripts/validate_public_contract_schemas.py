#!/usr/bin/env python3
"""Validate Public Contract 0.1 schemas and fixture boundary behavior."""

from __future__ import annotations

import json
from pathlib import Path

from jsonschema import Draft202012Validator, RefResolver

ROOT = Path(__file__).resolve().parents[1]
SCHEMA_DIR = ROOT / "schemas" / "public-contract" / "0.1"
FIXTURE_DIR = ROOT / "fixtures" / "public-contract" / "0.1"
COUNTEREXAMPLE_DIR = ROOT / "fixtures" / "counterexamples"
DIALECT = "https://json-schema.org/draft/2020-12/schema"


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


def require_valid(schema_name: str, fixture: Path) -> None:
    validator = make_validator(schema_name)
    errors = sorted(validator.iter_errors(load_json(fixture)), key=lambda error: list(error.path))
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


def main() -> None:
    schema_files = sorted(SCHEMA_DIR.glob("*.schema.json"))
    if not schema_files:
        raise AssertionError("no Public Contract schemas found")
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

    print(
        "Public Contract 0.1 schema validation passed: "
        f"{len(positive)} positive, {len(structurally_invalid)} structural negative, "
        f"{len(schema_valid_semantic_counterexamples)} semantic-boundary fixtures"
    )


if __name__ == "__main__":
    main()
