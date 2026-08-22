#!/usr/bin/env python3
"""Validate the milestone-local M0.6 external-project CI observation.

This validator intentionally checks only the Phase 5 evidence shape. It does not define a
stable conformance report schema, CLI contract, or exit-code contract.
"""

import json
import sys
from pathlib import Path

EXPECTED = [
    ("positive", "conformant"),
    ("adversarial_valid_negative", "conformant"),
    ("semantic_violation_detection", "non_conformant"),
    ("execute_response_loss", "not_established"),
]


def main() -> int:
    if len(sys.argv) != 2:
        raise SystemExit("usage: validate_m06_external_conformance_snapshot.py SNAPSHOT.json")

    path = Path(sys.argv[1])
    data = json.loads(path.read_text(encoding="utf-8"))

    assert data["format"] == "m0.6-provisional-ci-observation-v1"
    assert data["stability"] == "provisional_not_a_public_cli_or_report_contract"
    assert data["backend_validation"] == "not_assessed_by_conformance"

    observations = data["observations"]
    actual = [(item["label"], item["determination"]) for item in observations]
    assert actual == EXPECTED, actual
    assert all(item["determination"] == item["expected_determination"] for item in observations)

    for item in observations:
        ids = [case["id"] for case in item["cases"]]
        assert ids == sorted(ids), (item["label"], ids)
        assert len(ids) == len(set(ids)), (item["label"], ids)

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
