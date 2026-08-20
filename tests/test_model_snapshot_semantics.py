import copy
import json
import unittest
from pathlib import Path

from tests.model_snapshot_semantics import ModelSnapshotError, validate_resolved_model_snapshot


ROOT = Path(__file__).resolve().parents[1]
CORE_PACKAGE = json.loads((ROOT / "examples" / "reference-packages" / "core-reference-fixture-v0.1.json").read_text())
THERMAL_PACKAGE = json.loads((ROOT / "examples" / "reference-packages" / "thermal-reference-v0.1.json").read_text())
THERMAL_MODEL = json.loads((ROOT / "examples" / "thermal-reference-model-v0.1.json").read_text())
PACKAGES = [CORE_PACKAGE, THERMAL_PACKAGE]

PREFIX = "https://simulation-ontology.org/id/"
INCLUDES_COMPONENT = PREFIX + "kio7kpq2bu"
HAS_MODEL = PREFIX + "lpihscphvu"
APPLIED_TO = PREFIX + "msrwiztb4t"
SIMULATION_MODEL = PREFIX + "n6hnqkv3di"
FIELD = PREFIX + "zp6ffdj7ng"


class ModelSnapshotSemanticsTests(unittest.TestCase):
    def test_valid_thermal_snapshot_passes(self):
        result = validate_resolved_model_snapshot(THERMAL_MODEL, PACKAGES)
        self.assertIn("model:thermal-simulation", result["instance_index"])
        self.assertEqual(len(result["relation_triples"]), len(THERMAL_MODEL["relations"]))

    def test_snapshot_environment_must_equal_supplied_packages(self):
        snapshot = copy.deepcopy(THERMAL_MODEL)
        snapshot["ontology_environment"].pop()
        with self.assertRaisesRegex(ModelSnapshotError, "MODEL_ONTOLOGY_ENVIRONMENT_MISMATCH"):
            validate_resolved_model_snapshot(snapshot, PACKAGES)

    def test_duplicate_entity_id_fails(self):
        snapshot = copy.deepcopy(THERMAL_MODEL)
        snapshot["entities"].append(copy.deepcopy(snapshot["entities"][0]))
        with self.assertRaisesRegex(ModelSnapshotError, "MODEL_ENTITY_ID_DUPLICATE"):
            validate_resolved_model_snapshot(snapshot, PACKAGES)

    def test_unknown_entity_type_fails(self):
        snapshot = copy.deepcopy(THERMAL_MODEL)
        snapshot["entities"][0]["type"] = PREFIX + "missing-type"
        with self.assertRaisesRegex(ModelSnapshotError, "MODEL_ENTITY_TYPE_UNRESOLVED"):
            validate_resolved_model_snapshot(snapshot, PACKAGES)

    def test_duplicate_property_assignment_fails(self):
        snapshot = copy.deepcopy(THERMAL_MODEL)
        entity = next(item for item in snapshot["entities"] if item["id"] == "model:left-fixed-temperature")
        entity["properties"].append(copy.deepcopy(entity["properties"][0]))
        with self.assertRaisesRegex(ModelSnapshotError, "MODEL_PROPERTY_ASSIGNMENT_DUPLICATE"):
            validate_resolved_model_snapshot(snapshot, PACKAGES)

    def test_unknown_property_definition_fails(self):
        snapshot = copy.deepcopy(THERMAL_MODEL)
        entity = next(item for item in snapshot["entities"] if item["id"] == "model:left-fixed-temperature")
        entity["properties"][0]["property"] = PREFIX + "missing-property"
        with self.assertRaisesRegex(ModelSnapshotError, "MODEL_PROPERTY_DEFINITION_UNRESOLVED"):
            validate_resolved_model_snapshot(snapshot, PACKAGES)

    def test_duplicate_relation_triple_fails(self):
        snapshot = copy.deepcopy(THERMAL_MODEL)
        snapshot["relations"].append(copy.deepcopy(snapshot["relations"][0]))
        with self.assertRaisesRegex(ModelSnapshotError, "MODEL_RELATION_TRIPLE_DUPLICATE"):
            validate_resolved_model_snapshot(snapshot, PACKAGES)

    def test_illegal_includes_component_pair_fails(self):
        snapshot = copy.deepcopy(THERMAL_MODEL)
        snapshot["relations"].append(
            {
                "relation": INCLUDES_COMPONENT,
                "source": "model:thermal-model",
                "target": "model:temperature-field",
            }
        )
        with self.assertRaisesRegex(ModelSnapshotError, "MODEL_RELATION_ALLOWED_PAIR_MISMATCH"):
            validate_resolved_model_snapshot(snapshot, PACKAGES)

    def test_missing_has_model_edge_fails_cardinality(self):
        snapshot = copy.deepcopy(THERMAL_MODEL)
        snapshot["relations"] = [edge for edge in snapshot["relations"] if edge["relation"] != HAS_MODEL]
        with self.assertRaisesRegex(ModelSnapshotError, "MODEL_CARDINALITY_VIOLATION"):
            validate_resolved_model_snapshot(snapshot, PACKAGES)

    def test_two_has_model_targets_fail_cardinality(self):
        snapshot = copy.deepcopy(THERMAL_MODEL)
        snapshot["entities"].append({"id": "model:thermal-model-2", "type": SIMULATION_MODEL, "properties": []})
        snapshot["relations"].append(
            {
                "relation": HAS_MODEL,
                "source": "model:thermal-simulation",
                "target": "model:thermal-model-2",
            }
        )
        with self.assertRaisesRegex(ModelSnapshotError, "MODEL_CARDINALITY_VIOLATION"):
            validate_resolved_model_snapshot(snapshot, PACKAGES)

    def test_applied_to_missing_for_one_condition_fails_even_with_no_edge(self):
        snapshot = copy.deepcopy(THERMAL_MODEL)
        snapshot["relations"] = [
            edge
            for edge in snapshot["relations"]
            if not (edge["relation"] == APPLIED_TO and edge["source"] == "model:left-fixed-temperature")
        ]
        with self.assertRaisesRegex(ModelSnapshotError, "MODEL_CARDINALITY_VIOLATION"):
            validate_resolved_model_snapshot(snapshot, PACKAGES)

    def test_backend_local_relation_token_is_not_a_definition(self):
        snapshot = copy.deepcopy(THERMAL_MODEL)
        snapshot["relations"][0]["relation"] = "boundary"
        with self.assertRaisesRegex(ModelSnapshotError, "MODEL_RELATION_DEFINITION_UNRESOLVED"):
            validate_resolved_model_snapshot(snapshot, PACKAGES)

    def test_order_permutation_preserves_semantic_identity_sets(self):
        first = validate_resolved_model_snapshot(THERMAL_MODEL, PACKAGES)
        snapshot = copy.deepcopy(THERMAL_MODEL)
        snapshot["entities"].reverse()
        snapshot["relations"].reverse()
        for entity in snapshot["entities"]:
            entity["properties"].reverse()
        second = validate_resolved_model_snapshot(snapshot, list(reversed(PACKAGES)))
        self.assertEqual(set(first["instance_index"]), set(second["instance_index"]))
        self.assertEqual(first["relation_triples"], second["relation_triples"])


if __name__ == "__main__":
    unittest.main()
