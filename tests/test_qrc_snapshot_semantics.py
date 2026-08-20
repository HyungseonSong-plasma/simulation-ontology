import copy
import json
import unittest
from pathlib import Path

from tests.qrc_snapshot_semantics import SnapshotQRCError, validate_snapshot_qrc


ROOT = Path(__file__).resolve().parents[1]
CORE_PACKAGE = json.loads((ROOT / "examples" / "reference-packages" / "core-reference-fixture-v0.1.json").read_text())
PLASMA_PACKAGE = json.loads((ROOT / "examples" / "reference-packages" / "plasma-reference-v0.1.json").read_text())
PLASMA_MODEL = json.loads((ROOT / "examples" / "plasma-qrc-reference-model-v0.1.json").read_text())
PACKAGES = [CORE_PACKAGE, PLASMA_PACKAGE]

PREFIX = "https://simulation-ontology.org/id/"
INCLUDES_COMPONENT = PREFIX + "kio7kpq2bu"
PRODUCTS = PREFIX + "o7hkersiye"
CL_MINUS = PREFIX + "yyoyushsmg"
SPECIES = PREFIX + "zrbefujchf"
NEGATIVE_QRC = PREFIX + "gvta6gvuoj"
NEUTRAL_QRC = PREFIX + "3a7vxafn2k"


def remove_product(snapshot, target):
    snapshot["relations"] = [
        edge
        for edge in snapshot["relations"]
        if not (edge["relation"] == PRODUCTS and edge["source"] == "model:attachment-reaction" and edge["target"] == target)
    ]


class QRCSnapshotSemanticsTests(unittest.TestCase):
    def test_baseline_plasma_qrc_passes_two_exact_one_obligations(self):
        result = validate_snapshot_qrc(PLASMA_MODEL, PACKAGES)
        self.assertEqual(result["state"], "PASS")
        by_constraint = {item["constraint"]: item for item in result["evaluated"]}
        self.assertEqual(by_constraint[NEGATIVE_QRC]["count"], 1)
        self.assertEqual(by_constraint[NEUTRAL_QRC]["count"], 1)
        self.assertEqual(by_constraint[NEGATIVE_QRC]["targets"], frozenset({"model:cl-minus"}))
        self.assertEqual(by_constraint[NEUTRAL_QRC]["targets"], frozenset({"model:cl-atom"}))

    def test_missing_negative_ion_product_fails_min(self):
        snapshot = copy.deepcopy(PLASMA_MODEL)
        remove_product(snapshot, "model:cl-minus")
        with self.assertRaisesRegex(SnapshotQRCError, f"QRC_CARDINALITY_VIOLATION:{NEGATIVE_QRC}"):
            validate_snapshot_qrc(snapshot, PACKAGES)

    def test_second_distinct_negative_ion_product_fails_max(self):
        snapshot = copy.deepcopy(PLASMA_MODEL)
        snapshot["entities"].append({"id": "model:cl-minus-2", "type": CL_MINUS, "properties": []})
        snapshot["relations"].extend(
            [
                {"relation": INCLUDES_COMPONENT, "source": "model:plasma-material-model", "target": "model:cl-minus-2"},
                {"relation": PRODUCTS, "source": "model:attachment-reaction", "target": "model:cl-minus-2"},
            ]
        )
        with self.assertRaisesRegex(SnapshotQRCError, f"QRC_CARDINALITY_VIOLATION:{NEGATIVE_QRC}"):
            validate_snapshot_qrc(snapshot, PACKAGES)

    def test_second_neutral_product_fails_max(self):
        snapshot = copy.deepcopy(PLASMA_MODEL)
        snapshot["relations"].append(
            {"relation": PRODUCTS, "source": "model:attachment-reaction", "target": "model:cl2"}
        )
        with self.assertRaisesRegex(SnapshotQRCError, f"QRC_CARDINALITY_VIOLATION:{NEUTRAL_QRC}"):
            validate_snapshot_qrc(snapshot, PACKAGES)

    def test_generic_species_does_not_satisfy_negative_ion_qualifier(self):
        snapshot = copy.deepcopy(PLASMA_MODEL)
        entity = next(item for item in snapshot["entities"] if item["id"] == "model:cl-minus")
        entity["type"] = SPECIES
        with self.assertRaisesRegex(SnapshotQRCError, f"QRC_CARDINALITY_VIOLATION:{NEGATIVE_QRC}"):
            validate_snapshot_qrc(snapshot, PACKAGES)

    def test_subtypes_satisfy_qualified_target_types(self):
        result = validate_snapshot_qrc(PLASMA_MODEL, PACKAGES)
        evaluated = {item["constraint"]: item for item in result["evaluated"]}
        self.assertIn("model:cl-minus", evaluated[NEGATIVE_QRC]["targets"])
        self.assertIn("model:cl-atom", evaluated[NEUTRAL_QRC]["targets"])

    def test_duplicate_product_triple_fails_before_qrc_counting(self):
        snapshot = copy.deepcopy(PLASMA_MODEL)
        edge = next(edge for edge in snapshot["relations"] if edge["relation"] == PRODUCTS)
        snapshot["relations"].append(copy.deepcopy(edge))
        with self.assertRaisesRegex(SnapshotQRCError, "QRC_MODEL_PRECONDITION_FAIL:MODEL_RELATION_TRIPLE_DUPLICATE"):
            validate_snapshot_qrc(snapshot, PACKAGES)

    def test_unknown_qualifier_type_is_unresolved_not_zero_count(self):
        packages = copy.deepcopy(PACKAGES)
        plasma = packages[1]
        constraint = next(item for item in plasma["constraint_definitions"] if item["id"] == NEGATIVE_QRC)
        constraint["payload"]["qualifier"]["target_type"] = PREFIX + "missing-type"
        with self.assertRaisesRegex(SnapshotQRCError, "QRC_QUALIFIER_TARGET_TYPE_UNRESOLVED"):
            validate_snapshot_qrc(PLASMA_MODEL, packages)

    def test_non_closed_snapshot_fails_precondition(self):
        snapshot = copy.deepcopy(PLASMA_MODEL)
        snapshot["snapshot_state"] = "open"
        with self.assertRaisesRegex(SnapshotQRCError, "QRC_MODEL_PRECONDITION_FAIL:MODEL_SNAPSHOT_NOT_CLOSED"):
            validate_snapshot_qrc(snapshot, PACKAGES)

    def test_order_permutation_preserves_qrc_counts(self):
        first = validate_snapshot_qrc(PLASMA_MODEL, PACKAGES)
        snapshot = copy.deepcopy(PLASMA_MODEL)
        snapshot["entities"].reverse()
        snapshot["relations"].reverse()
        second = validate_snapshot_qrc(snapshot, list(reversed(PACKAGES)))
        first_counts = sorted((item["constraint"], item["count"]) for item in first["evaluated"])
        second_counts = sorted((item["constraint"], item["count"]) for item in second["evaluated"])
        self.assertEqual(first_counts, second_counts)

    def test_backend_runtime_state_is_not_qrc_input(self):
        result = validate_snapshot_qrc(PLASMA_MODEL, PACKAGES)
        self.assertEqual(result["state"], "PASS")
        self.assertNotIn("backend", result)


if __name__ == "__main__":
    unittest.main()
