import json
import unittest
from pathlib import Path

from jsonschema import Draft202012Validator, ValidationError

ROOT = Path(__file__).resolve().parents[1]
QRC = json.loads((ROOT / "schema" / "qrc-v0.1.schema.json").read_text())
MAPPING = json.loads((ROOT / "schema" / "mapping-contract-v0.1.schema.json").read_text())


class JsonSchemaContractTests(unittest.TestCase):
    def test_schemas_are_valid_draft_2020_12(self):
        Draft202012Validator.check_schema(QRC)
        Draft202012Validator.check_schema(MAPPING)

    def _mapping_validator(self, definition):
        return Draft202012Validator({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$defs": MAPPING["$defs"],
            "$ref": f"#/$defs/{definition}",
        })

    def test_mapping_rule_is_exactly_five_fields(self):
        v=self._mapping_validator("MappingRule")
        valid={"source":{},"applicability":{},"realization":{},"bindings":{},"capabilities":{}}
        v.validate(valid)
        with self.assertRaises(ValidationError):
            v.validate(dict(valid,priority=1))

    def test_mapping_claim_is_exactly_four_fields(self):
        v=self._mapping_validator("MappingClaim")
        valid={"obligation":"x","source":"s","realization":{},"provenance":{}}
        v.validate(valid)
        with self.assertRaises(ValidationError):
            v.validate(dict(valid,diagnostics={}))

    def test_comparator_scope_conditional_shape(self):
        v=self._mapping_validator("ComparatorRegistryEntry")
        binding={"component_id":"A","adapter_contract_id":"a","adapter_contract_version":"1"}
        local={"id":"c","version":"1","scope":"local","component_binding":binding,"comparison_purpose":"overlap","left_kind":"x","right_kind":"x"}
        v.validate(local)
        cross={"id":"c","version":"1","scope":"cross_component","left_component_binding":binding,"right_component_binding":dict(binding,component_id="B"),"orchestration_component_binding":dict(binding,component_id="O",adapter_contract_id="orch"),"comparison_purpose":"transfer","left_kind":"read","right_kind":"write"}
        v.validate(cross)
        with self.assertRaises(ValidationError):
            v.validate({k:v for k,v in cross.items() if k != "orchestration_component_binding"})

    def test_qrc_schema_bounds_and_extra_fields(self):
        v=Draft202012Validator(QRC)
        v.validate({"type":"cardinality","relation":"products","qualifier":{"target_type":"NegativeIonSpecies"},"min":1})
        with self.assertRaises(ValidationError):
            v.validate({"type":"cardinality","relation":"products","min":-1})
        with self.assertRaises(ValidationError):
            v.validate({"type":"cardinality","relation":"products","min":1,"unknown":True})


if __name__ == "__main__":
    unittest.main()
