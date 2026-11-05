#!/usr/bin/env python3
"""Verify source-preserving MOOSE Exodus field extracts without claiming SOL semantics.

Source: moose-test-repo Actions run 41739266674, artifact 13296715986.
The embedded 1D interface is NOT an independent 1D simulation.
"""
from __future__ import annotations

import json
import math
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EVIDENCE = ROOT / "fixtures" / "source-evidence" / "moose" / "m010"
SHA = "ae62510bcff30d68759633e099be9fa75164cda69dd893c7ec2481885799223c"


def read(name: str) -> dict:
    return json.loads((EVIDENCE / name).read_text(encoding="utf-8"))


def verify(record: dict, *, node_count: int, cell_name: str, cell_count: int,
           value_count: int, cell_type: str, entity_dimension: int) -> None:
    assert record["record_kind"] == "exodus-field-extract-unapproved-sol-fixture"
    assert record["source_sha256"] == SHA
    assert record["embedding_dimension"] == 2
    assert record["topological_dimension"] in (1, 2)
    assert record["entity_dimension"] == entity_dimension
    assert record["time_unit_from_file"] is None
    assert record["unit_from_file"] is None
    assert record["time_value_as_stored"] == 0.0001

    coordinates = record["coordinates_xy"]
    assert len(coordinates) == node_count
    assert all(len(xy) == 2 and all(math.isfinite(v) for v in xy) for xy in coordinates)

    cells = record[cell_name]
    assert len(cells) == cell_count
    arity = 2 if cell_type == "line2" else 3
    for cell in cells:
        indices = cell["vertices"]
        assert len(indices) == arity and len(set(indices)) == arity
        assert all(type(i) is int and 0 <= i < node_count for i in indices)

    values = record["values"]
    assert len(values) == value_count
    assert all(isinstance(v, (int, float)) and math.isfinite(v) for v in values)
    assert len(record["node_source_ids_1based"]) == node_count
    assert len(set(record["node_source_ids_1based"])) == node_count


def main() -> None:
    origin = read("source_provenance.json")
    assert origin["repository"] == "HyungseonSong-plasma/moose-test-repo"
    assert origin["run_id"] == 41739266674
    assert origin["artifact_id"] == 13296715986
    assert origin["source_sha256"] == SHA
    assert origin["exodus_dimensions"]["coordinate_dimensions"] == 2

    volume = read("field_2d_potential_exodus.json")
    interface = read("field_1d_interface_sigma_s_exodus.json")

    verify(volume, node_count=1721, cell_name="cells_tri3",
           cell_count=3155, value_count=1721, cell_type="tri3", entity_dimension=0)
    verify(interface, node_count=54, cell_name="cells_edge2",
           cell_count=53, value_count=53, cell_type="line2", entity_dimension=1)

    assert volume["exodus_field_name"] == "potential"
    assert volume["exodus_variable_kind"] == "nodal"
    assert interface["exodus_field_name"] == "sigma_s"
    assert interface["exodus_variable_kind"] == "elemental"
    assert interface["exodus_block_name"] == "cover_surface"
    assert volume["time_value_as_stored"] == interface["time_value_as_stored"]
    assert origin["exodus_dimensions"]["total_elements"] == 3236
    assert origin["exodus_dimensions"]["nodes"] == 1721
    print("M0.10 MOOSE source evidence: structural arrays PASS; semantic/physics gates OPEN")


if __name__ == "__main__":
    main()
