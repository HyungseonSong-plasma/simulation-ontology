import copy
import json
import unittest
from pathlib import Path

from tests.reference_metrology_semantics import ReferenceMetrologyError, validate_snapshot_metrology


ROOT = Path(__file__).resolve().parents[1]
CORE_PACKAGE = json.loads((ROOT / "examples" / "reference-packages" / "core-reference-fixture-v0.1.json").read_text())
THERMAL_PACKAGE = json.loads((ROOT / "examples" / "reference-packages" / "thermal-reference-v0.1.json").read_text())
THERMAL_MODEL = json.loads((ROOT / "examples" / "thermal-reference-model-v0.1.json").read_text())
METROLOGY = json.loads((ROOT / "examples" / "reference-data" / "thermal-metrology-evidence-v0.1.json").read_text())
PACKAGES = [CORE_PACKAGE, THERMAL_PACKAGE]


def entity(snapshot, identifier):
    return next(item for item in snapshot["entities"] if item["id"] == identifier)


class ReferenceMetrologySemanticsTests(unittest.TestCase):
    def test_thermal_reference_metrology_passes(self):
        result = validate_snapshot_metrology(THERMAL_MODEL, PACKAGES, METROLOGY)
        self.assertEqual(result["state"], "PASS")
        self.assertEqual(result["diagnostics"], [])

    def test_missing_metrology_evidence_is_indeterminate_not_fail(self):
        result = validate_snapshot_metrology(THERMAL_MODEL, PACKAGES, None)
        self.assertEqual(result["state"], "INDETERMINATE")
        self.assertIn("REFERENCE_METROLOGY_EVIDENCE_UNRESOLVED", result["diagnostics"])

    def test_missing_one_unit_resolution_is_indeterminate(self):
        evidence = copy.deepcopy(METROLOGY)
        evidence["units"] = [entry for entry in evidence["units"] if entry["unit"]["id"] != "K"]
        result = validate_snapshot_metrology(THERMAL_MODEL, PACKAGES, evidence)
        self.assertEqual(result["state"], "INDETERMINATE")
        self.assertIn("REFERENCE_VALUE_UNIT_UNRESOLVED", result["diagnostics"])

    def test_temperature_value_with_conductivity_unit_fails_dimension(self):
        snapshot = copy.deepcopy(THERMAL_MODEL)
        left = entity(snapshot, "model:left-fixed-temperature")
        left["properties"][0]["value_definition"]["value"]["unit"] = {
            "namespace": "si-ref",
            "id": "W_per_m_K",
        }
        with self.assertRaisesRegex(ReferenceMetrologyError, "REFERENCE_VALUE_UNIT_DIMENSION_MISMATCH"):
            validate_snapshot_metrology(snapshot, PACKAGES, METROLOGY)

    def test_conductivity_evidence_wrong_dimension_fails(self):
        evidence = copy.deepcopy(METROLOGY)
        conductivity = next(entry for entry in evidence["units"] if entry["unit"]["id"] == "W_per_m_K")
        conductivity["dimension"]["length"] = 2
        with self.assertRaisesRegex(ReferenceMetrologyError, "REFERENCE_VALUE_UNIT_DIMENSION_MISMATCH"):
            validate_snapshot_metrology(THERMAL_MODEL, PACKAGES, evidence)

    def test_backend_license_state_is_not_a_metrology_input(self):
        result = validate_snapshot_metrology(THERMAL_MODEL, PACKAGES, METROLOGY)
        self.assertNotIn("backend", result)
        self.assertEqual(result["state"], "PASS")


if __name__ == "__main__":
    unittest.main()
