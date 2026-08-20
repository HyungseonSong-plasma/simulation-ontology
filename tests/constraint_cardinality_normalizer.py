"""Minimal non-normative Cardinality normalization helper for ADR-0018 tests.

This helper exercises interval normalization only. Canonical relation/type identity
resolution is supplied by the caller/package normalization layer and is not invented here.
"""
from __future__ import annotations

from typing import Any, Dict, Optional, Tuple

PASS = "PASS"
FAIL = "FAIL"


def normalize_bounds(authoring: Dict[str, Any]) -> Tuple[str, str, Optional[Tuple[int, Optional[int]]]]:
    values = {key: authoring[key] for key in ("min", "max", "exact") if key in authoring}
    for value in values.values():
        if isinstance(value, bool) or not isinstance(value, int) or value < 0:
            return FAIL, "QRC_BOUND_INVALID", None

    lowers = [0]
    uppers = []
    if "min" in values:
        lowers.append(values["min"])
    if "max" in values:
        uppers.append(values["max"])
    if "exact" in values:
        lowers.append(values["exact"])
        uppers.append(values["exact"])

    lower = max(lowers)
    upper = min(uppers) if uppers else None
    if upper is not None and lower > upper:
        return FAIL, "QRC_EMPTY_INTERVAL", None
    return PASS, "OK", (lower, upper)


def normalize_cardinality_payload(
    authoring: Dict[str, Any],
    *,
    canonical_relation: str,
    canonical_target_type: Optional[str] = None,
) -> Dict[str, Any]:
    """Return the normalized semantic payload after external identity resolution.

    Raises ValueError with the accepted diagnostic code when interval normalization fails.
    """
    status, code, interval = normalize_bounds(authoring)
    if status != PASS or interval is None:
        raise ValueError(code)

    lower, upper = interval
    result: Dict[str, Any] = {
        "type": "cardinality",
        "relation": canonical_relation,
        "direction": "source",
        "min": lower,
        "max": "unbounded" if upper is None else upper,
    }
    if "qualifier" in authoring:
        if not canonical_target_type:
            raise ValueError("QRC_QUALIFIER_CANONICAL_ID_REQUIRED")
        result["qualifier"] = {"target_type": canonical_target_type}
    return result
