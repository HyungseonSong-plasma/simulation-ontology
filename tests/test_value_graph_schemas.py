import json
import unittest
from pathlib import Path

from jsonschema import Draft202012Validator, ValidationError
from referencing import Registry, Resource

ROOT = Path(__file__).resolve().parents[1]
SCHEMA_DIR = ROOT / "schema"
NAMES = [
    "dimension-vector-v0.1.schema.json",
    "unit-reference-v0.1.schema.json",
    "exact-decimal-v0.1.schema.json",
    "value-v0.1.schema.json",
    "value-definition-inline-v0.1.schema.json",
    "constraint-dimension-normalized-v0.1.schema.json",
    "constraint-value-normalized-v0.1.schema.json",
]
SCHEMAS = {name: json.loads((SCHEMA_DIR / name).read_text()) for name in NAMES}


def registry():
    result = Registry()
    for schema in SCHEMAS.values():
        result = result.with_resource(schema["$id"], Resource.from_contents(schema))
    return result


def validator(name):
    return Draft202012Validator(SCHEMAS[name], registry=registry())


class ValueGraphSchemaTests(unittest.TestCase):
    def test_schemas_are_valid_draft_2020_12(self):
        for schema in SCHEMAS.values():
            Draft202012Validator.check_schema(schema)

    def test_exact_decimal_is_canonical(self):
        v = validator("exact-decimal-v0.1.schema.json")
        v.validate({"coefficient": "3", "exponent10": 2})
        v.validate({"coefficient": "0", "exponent10": 0})
        with self.assertRaises(ValidationError):
            v.validate({"coefficient": "30", "exponent10": 1})
        with self.assertRaises(ValidationError):
            v.validate({"coefficient": "0", "exponent10": 1})

    def test_dimension_constraint_reuses_shared_vector(self):
        v = validator("constraint-dimension-normalized-v0.1.schema.json")
        vector = {
            "time": 0,
            "length": 1,
            "mass": 0,
            "electric_current": 0,
            "thermodynamic_temperature": 0,
            "amount_of_substance": 0,
            "luminous_intensity": 0,
        }
        v.validate({"type": "dimension", "vector": vector})
        with self.assertRaises(ValidationError):
            v.validate({"type": "dimension", "vector": {"length": 1}})

    def test_numeric_value_may_have_unit(self):
        v = validator("value-v0.1.schema.json")
        v.validate({
            "shape": "scalar",
            "scalar_kind": "number",
            "data": {"coefficient": "3", "exponent10": 2},
            "unit": {"namespace": "qudt", "id": "K"},
        })

    def test_non_numeric_value_cannot_have_unit(self):
        v = validator("value-v0.1.schema.json")
        with self.assertRaises(ValidationError):
            v.validate({
                "shape": "scalar",
                "scalar_kind": "string",
                "data": "hot",
                "unit": {"namespace": "qudt", "id": "K"},
            })

    def test_tensor_schema_requires_explicit_shape(self):
        v = validator("value-v0.1.schema.json")
        v.validate({
            "shape": "tensor",
            "scalar_kind": "boolean",
            "tensor_shape": [2, 2],
            "data": [True, False, False, True],
        })
        with self.assertRaises(ValidationError):
            v.validate({"shape": "tensor", "scalar_kind": "boolean", "data": [True]})

    def test_literal_inline_definition_is_core_native(self):
        v = validator("value-definition-inline-v0.1.schema.json")
        v.validate({
            "mechanism": "literal",
            "value": {
                "shape": "scalar",
                "scalar_kind": "number",
                "data": {"coefficient": "3", "exponent10": 2},
            },
        })
        with self.assertRaises(ValidationError):
            v.validate({
                "mechanism": "literal",
                "value": {"shape": "scalar", "scalar_kind": "boolean", "data": True},
                "format": "sol:expr",
            })

    def test_nonliteral_requires_format_and_payload(self):
        v = validator("value-definition-inline-v0.1.schema.json")
        v.validate({"mechanism": "expression", "format": "sol:expr-v1", "payload": {"text": "2*pi*x"}})
        with self.assertRaises(ValidationError):
            v.validate({"mechanism": "expression", "payload": {"text": "2*pi*x"}})

    def test_inline_definition_cannot_carry_graph_identity_or_dependency(self):
        v = validator("value-definition-inline-v0.1.schema.json")
        with self.assertRaises(ValidationError):
            v.validate({
                "mechanism": "function",
                "format": "domain:function-v1",
                "payload": {},
                "depends_on": ["model:T"],
            })
        with self.assertRaises(ValidationError):
            v.validate({
                "mechanism": "tabular",
                "format": "domain:table-v1",
                "payload": [],
                "id": "vd:inline",
            })

    def test_value_constraint_reuses_shared_exact_decimal(self):
        v = validator("constraint-value-normalized-v0.1.schema.json")
        v.validate({
            "type": "value",
            "form": "allowed_set",
            "scalar_kind": "number",
            "values": [{"coefficient": "1", "exponent10": 0}],
        })
        with self.assertRaises(ValidationError):
            v.validate({
                "type": "value",
                "form": "allowed_set",
                "scalar_kind": "number",
                "values": [{"coefficient": "10", "exponent10": 0}],
            })


if __name__ == "__main__":
    unittest.main()
