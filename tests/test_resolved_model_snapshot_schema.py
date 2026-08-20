import copy
import json
import unittest
from pathlib import Path

from jsonschema import Draft202012Validator, ValidationError
from referencing import Registry, Resource


ROOT = Path(__file__).resolve().parents[1]
SCHEMA_DIR = ROOT / "schema"
SCHEMAS = {path.name: json.loads(path.read_text()) for path in SCHEMA_DIR.glob("*.schema.json")}
THERMAL_MODEL = json.loads((ROOT / "examples" / "thermal-reference-model-v0.1.json").read_text())
CORE_PACKAGE = json.loads((ROOT / "examples" / "reference-packages" / "core-reference-fixture-v0.1.json").read_text())
THERMAL_PACKAGE = json.loads((ROOT / "examples" / "reference-packages" / "thermal-reference-v0.1.json").read_text())


def registry():
    result = Registry()
    for schema in SCHEMAS.values():
        if "$id" in schema:
            result = result.with_resource(schema["$id"], Resource.from_contents(schema))
    return result


def validator(name):
    return Draft202012Validator(SCHEMAS[name], registry=registry())


class ResolvedModelSnapshotSchemaTests(unittest.TestCase):
    def test_snapshot_schema_is_valid_draft_2020_12(self):
        Draft202012Validator.check_schema(SCHEMAS["resolved-model-snapshot-v0.1.schema.json"])

    def test_thermal_snapshot_resolves_transitive_refs(self):
        validator("resolved-model-snapshot-v0.1.schema.json").validate(THERMAL_MODEL)

    def test_reference_packages_conform_to_normalized_package_schema(self):
        package_validator = validator("ontology-package-normalized-v0.1.schema.json")
        package_validator.validate(CORE_PACKAGE)
        package_validator.validate(THERMAL_PACKAGE)

    def test_snapshot_must_be_closed(self):
        value = copy.deepcopy(THERMAL_MODEL)
        value["snapshot_state"] = "open"
        with self.assertRaises(ValidationError):
            validator("resolved-model-snapshot-v0.1.schema.json").validate(value)

    def test_package_environment_rejects_version_range(self):
        value = copy.deepcopy(THERMAL_MODEL)
        value["ontology_environment"][0]["version"] = ">=0.1.0 <0.2.0"
        with self.assertRaises(ValidationError):
            validator("resolved-model-snapshot-v0.1.schema.json").validate(value)

    def test_entity_properties_field_is_required(self):
        value = copy.deepcopy(THERMAL_MODEL)
        value["entities"][0].pop("properties")
        with self.assertRaises(ValidationError):
            validator("resolved-model-snapshot-v0.1.schema.json").validate(value)

    def test_property_assignment_requires_inline_value_definition(self):
        value = copy.deepcopy(THERMAL_MODEL)
        assignment = next(item for item in value["entities"] if item["properties"])["properties"][0]
        assignment.pop("value_definition")
        with self.assertRaises(ValidationError):
            validator("resolved-model-snapshot-v0.1.schema.json").validate(value)

    def test_model_type_must_use_canonical_resource_shape(self):
        value = copy.deepcopy(THERMAL_MODEL)
        value["entities"][0]["type"] = "sol:Simulation"
        with self.assertRaises(ValidationError):
            validator("resolved-model-snapshot-v0.1.schema.json").validate(value)

    def test_extra_backend_field_is_rejected(self):
        value = copy.deepcopy(THERMAL_MODEL)
        value["entities"][0]["backend_handle"] = "moose-local"
        with self.assertRaises(ValidationError):
            validator("resolved-model-snapshot-v0.1.schema.json").validate(value)


if __name__ == "__main__":
    unittest.main()
