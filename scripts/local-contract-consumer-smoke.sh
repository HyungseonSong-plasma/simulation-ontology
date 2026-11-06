#!/usr/bin/env bash
# Preliminary D0 smoke test: external project consumes SOL source checkout via
# path dependencies. This does NOT certify an official release artifact.
set -euo pipefail

usage() {
  echo "Usage: bash scripts/local-contract-consumer-smoke.sh [SOL_SOURCE_CHECKOUT]" >&2
  echo "Optional: SOL_EXPECTED_SHA=<full git SHA> SOL_EVIDENCE_DIR=<directory>" >&2
}

if [ "$#" -gt 1 ]; then usage; exit 2; fi
script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
sol_root="$(cd "${1:-${script_dir}/..}" && pwd -P)"

for path in \
  "crates/sol-public-contract/Cargo.toml" \
  "crates/sol-adapter-protocol/Cargo.toml" \
  "crates/sol-adapter-transport/Cargo.toml" \
  "fixtures/public-contract/0.2/thermal-realization-spec.json" \
  "fixtures/public-contract/0.3/thermal-run-record-produced.json"; do
  if [ ! -f "${sol_root}/${path}" ]; then
    echo "ERROR: missing SOL source component ${sol_root}/${path}" >&2
    exit 3
  fi
done
for executable in cargo rustc python3 git; do
  if ! command -v "$executable" >/dev/null 2>&1; then
    echo "ERROR: missing $executable. Install Rust/Cargo, Python 3 and Git." >&2
    exit 4
  fi
done

sol_revision="$(git -C "$sol_root" rev-parse HEAD 2>/dev/null || true)"
if [ -n "${SOL_EXPECTED_SHA:-}" ]; then
  if [ -z "$sol_revision" ] || [ "$sol_revision" != "$SOL_EXPECTED_SHA" ]; then
    echo "ERROR: SOL_EXPECTED_SHA did not match checkout HEAD." >&2
    echo "Expected: $SOL_EXPECTED_SHA; actual: ${sol_revision:-unknown}" >&2
    exit 5
  fi
fi
if ! git -C "$sol_root" diff --quiet -- 2>/dev/null ||
   ! git -C "$sol_root" diff --cached --quiet -- 2>/dev/null; then
  echo "NOTICE: local SOL working tree has uncommitted changes." >&2
fi

echo "[SOL D0] Source: $sol_root"
echo "[SOL D0] Revision: ${sol_revision:-not-a-git-checkout}"
echo "[SOL D0] Rust: $(rustc --version)"
echo "[SOL D0] Cargo: $(cargo --version)"
echo "[SOL D0] Platform: $(uname -s)/$(uname -m)"
echo "[SOL D0] Mode: source-path external consumer (NOT release package certification)"

tmp="$(mktemp -d "${TMPDIR:-/tmp}/sol-contract-smoke.XXXXXX")"
trap 'rm -rf "$tmp"' EXIT INT TERM
mkdir -p "$tmp/tests"

python3 - "$sol_root" "$tmp" <<'PY'
import json
import pathlib
import sys

root, consumer = (pathlib.Path(s).resolve() for s in sys.argv[1:])
deps = ["sol-public-contract", "sol-adapter-protocol", "sol-adapter-transport"]
manifest = [
    "[package]",
    'name = "sol-external-contract-consumer-smoke"',
    'version = "0.0.0"',
    'edition = "2021"',
    "publish = false",
    "",
    "[workspace]",
    "",
    "[dependencies]",
]
for name in deps:
    source = str(root / "crates" / name)
    manifest.append(f"{name} = {{ path = {json.dumps(source)} }}")
(consumer / "Cargo.toml").write_text("\n".join(manifest) + "\n")
(consumer / "src").mkdir(parents=True, exist_ok=True)
(consumer / "src" / "lib.rs").write_text("// Intentionally empty: external contract consumer.\n")
PY

cat > "$tmp/tests/contract_surface.rs" <<'RUST'
use sol_adapter_protocol::{
    ADAPTER_PROTOCOL_VERSION_0_1, ADAPTER_PROTOCOL_VERSION_0_2,
};
use sol_adapter_transport::{
    DESCRIBE_ADAPTER_METHOD, EXECUTE_PLAN_METHOD, VALIDATE_PLAN_METHOD,
};
use sol_public_contract::{
    PUBLIC_CONTRACT_VERSION_0_1, PUBLIC_CONTRACT_VERSION_0_2,
    PUBLIC_CONTRACT_VERSION_0_3, RealizationSpecDtoV02, SimulationRunRecordDtoV03,
};

#[test]
fn package_consumer_sees_frozen_version_axes_and_transport_names() {
    assert_eq!(PUBLIC_CONTRACT_VERSION_0_1, "0.1");
    assert_eq!(PUBLIC_CONTRACT_VERSION_0_2, "0.2");
    assert_eq!(PUBLIC_CONTRACT_VERSION_0_3, "0.3");
    assert_eq!(ADAPTER_PROTOCOL_VERSION_0_1, "0.1");
    assert_eq!(ADAPTER_PROTOCOL_VERSION_0_2, "0.2");
    assert_eq!(DESCRIBE_ADAPTER_METHOD, "describe_adapter");
    assert_eq!(VALIDATE_PLAN_METHOD, "validate_plan");
    assert_eq!(EXECUTE_PLAN_METHOD, "execute_plan");
}

#[test]
fn package_consumer_interprets_original_thermal_realization_v02() {
    let path = std::env::var("SOL_THERMAL_SPEC_FIXTURE").unwrap();
    let source = std::fs::read_to_string(path).unwrap();
    let spec = RealizationSpecDtoV02::from_json(&source).unwrap();
    let canonical = spec.to_canonical_json().unwrap();
    assert!(canonical.contains("unit.kelvin"));
    assert!(canonical.contains("unit.watt_per_meter_kelvin"));
    assert!(canonical.contains("scope.main_domain"));
    assert!(canonical.contains("observation.maximum_temperature"));
}

#[test]
fn package_consumer_interprets_original_scalar_result_v03() {
    let path = std::env::var("SOL_THERMAL_SCALAR_FIXTURE").unwrap();
    let source = std::fs::read_to_string(path).unwrap();
    let record = SimulationRunRecordDtoV03::from_json(&source).unwrap();
    let canonical = record.to_canonical_json().unwrap();
    assert!(canonical.contains("observation.maximum_temperature"));
    assert!(canonical.contains("unit.kelvin"));
    assert!(canonical.contains("run.thermal_reference_001"));
}
RUST

cargo generate-lockfile --manifest-path "$tmp/Cargo.toml" --quiet
export SOL_THERMAL_SPEC_FIXTURE="$sol_root/fixtures/public-contract/0.2/thermal-realization-spec.json"
export SOL_THERMAL_SCALAR_FIXTURE="$sol_root/fixtures/public-contract/0.3/thermal-run-record-produced.json"
cargo test --manifest-path "$tmp/Cargo.toml" --locked 2>&1 | tee "$tmp/test.log"

if [ -n "${SOL_EVIDENCE_DIR:-}" ]; then
  mkdir -p "$SOL_EVIDENCE_DIR"
  cp "$tmp/Cargo.lock" "$SOL_EVIDENCE_DIR/Cargo.lock"
  cp "$tmp/test.log" "$SOL_EVIDENCE_DIR/contract-smoke-test.log"
  printf "source_revision=%s\nplatform=%s/%s\nrustc=%s\ncargo=%s\nmode=source-path-not-release\n" \
    "${sol_revision:-unknown}" "$(uname -s)" "$(uname -m)" \
    "$(rustc --version)" "$(cargo --version)" > "$SOL_EVIDENCE_DIR/environment.txt"
fi
echo "[SOL D0] PASS: 3 isolated external-consumer tests; source checkout only."
