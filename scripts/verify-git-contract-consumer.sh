#!/usr/bin/env bash
# P1 candidate: contract consumer MUST resolve an immutable Git commit from SOL.
# This is not a released crate/registry package nor a MOOSE execution.
set -euo pipefail
script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
root="$(cd "$script_dir/.." && pwd -P)"
example="$root/examples/external-contract-git-consumer"
for exe in python3 cargo rustc git; do
  command -v "$exe" >/dev/null || { echo "Missing $exe" >&2; exit 2; }
done

tmp="$(mktemp -d "${TMPDIR:-/tmp}/sol-git-consumer.XXXXXX")"
trap 'rm -rf "$tmp"' EXIT INT TERM
mkdir -p "$tmp/positive" "$tmp/negative"
cp "$example/Cargo.toml" "$tmp/positive/Cargo.toml"
cp -R "$example/tests" "$tmp/positive/tests"
cp "$example/Cargo.toml" "$tmp/negative/Cargo.toml"
cp -R "$example/tests" "$tmp/negative/tests"

pin="$(python3 - "$tmp/positive/Cargo.toml" <<'PY'
import pathlib
import re
import sys
src=pathlib.Path(sys.argv[1]).read_text()
names=("sol-public-contract","sol-adapter-protocol","sol-adapter-transport")
pins=[]
for name in names:
    found=re.search(r'^'+re.escape(name)+r'\s*=\s*\{([^\n]+)\}',src,re.M)
    if not found:
        raise SystemExit(f"missing dependency: {name}")
    entry=found.group(1)
    if 'git = "https://github.com/HyungseonSong-plasma/simulation-ontology.git"' not in entry:
        raise SystemExit(f"not official SOL Git source: {name}")
    v=re.search(r'\brev = "([a-f0-9]{40})"',entry)
    if not v:
        raise SystemExit(f"missing 40-digit immutable SHA: {name}")
    if 'version = "=0.1.0"' not in entry:
        raise SystemExit(f"missing expected crate version lock: {name}")
    pins.append(v.group(1))
if len(set(pins))!=1:
    raise SystemExit("SOL crate source revisions do not match")
print(pins[0])
PY
)"
echo "[SOL P1] Immutable Git source: $pin"
echo "[SOL P1] Toolchain: $(rustc --version) | $(cargo --version)"
echo "[SOL P1] Platform: $(uname -s)/$(uname -m)"

cargo generate-lockfile --manifest-path "$tmp/positive/Cargo.toml" --quiet
cargo metadata --manifest-path "$tmp/positive/Cargo.toml" --locked --format-version 1 > "$tmp/metadata.json"

# Verify Cargo's ACTUAL resolved source IDs; the manifest alone is not proof.
upstream="$(python3 - "$tmp/metadata.json" "$pin" <<'PY'
import json
import pathlib
import sys
info=json.loads(pathlib.Path(sys.argv[1]).read_text())
pin=sys.argv[2]
name_set={"sol-public-contract","sol-adapter-protocol","sol-adapter-transport"}
found={}
for package in info["packages"]:
    if package["name"] in name_set:
        found[package["name"]]=package
if set(found)!=name_set:
    raise SystemExit(f"not all SOL crates resolved: {set(found)}")
roots=set()
for name, package in found.items():
    source=package.get("source") or ""
    if not source.startswith("git+https://github.com/HyungseonSong-plasma/simulation-ontology.git"):
        raise SystemExit(f"{name} did not resolve from official Git: {source}")
    if f"rev={pin}" not in source or not source.endswith("#"+pin):
        raise SystemExit(f"{name} resolved a different source commit: {source}")
    if package["version"]!="0.1.0":
        raise SystemExit(f"{name} has unexpected crate version: {package['version']}")
    manifest=pathlib.Path(package["manifest_path"]).resolve()
    roots.add(str(manifest.parents[2]))
if len(roots)!=1:
    raise SystemExit(f"the SDK crates did not share one upstream checkout: {roots}")
root=pathlib.Path(next(iter(roots)))
for rel in (
 "fixtures/public-contract/0.2/thermal-realization-spec.json",
 "fixtures/public-contract/0.3/thermal-run-record-produced.json",
):
    if not (root/rel).is_file():
        raise SystemExit(f"pinned upstream checkout is missing fixture {rel}")
print(root)
PY
)"
echo "[SOL P1] All 3 crates resolved to same immutable upstream Git SHA"
echo "[SOL P1] Test fixtures loaded from pinned Git checkout, not local branch"
export SOL_THERMAL_SPEC_FIXTURE="$upstream/fixtures/public-contract/0.2/thermal-realization-spec.json"
export SOL_THERMAL_SCALAR_FIXTURE="$upstream/fixtures/public-contract/0.3/thermal-run-record-produced.json"
cargo test --manifest-path "$tmp/positive/Cargo.toml" --locked 2>&1 | tee "$tmp/test.log"

# Negative packaging case: incompatible crate version must be refused even
# when same upstream Git source is cached. A newer version is not silently used.
python3 - "$tmp/negative/Cargo.toml" <<'PY'
import pathlib
import sys
path=pathlib.Path(sys.argv[1])
content=path.read_text()
line=next(line for line in content.splitlines() if line.startswith("sol-public-contract = "))
if 'version = "=0.1.0"' not in line:
    raise SystemExit("expected version constraint absent")
content=content.replace(line,line.replace('version = "=0.1.0"','version = "=0.2.0"'))
path.write_text(content)
PY
if CARGO_NET_OFFLINE=true cargo generate-lockfile --manifest-path "$tmp/negative/Cargo.toml" > "$tmp/negative.log" 2>&1; then
  echo "[SOL P1] ERROR: deliberately incompatible crate version was accepted" >&2
  exit 7
fi
echo "[SOL P1] Expected incompatible version =0.2.0 was rejected"

if [ -n "${SOL_EVIDENCE_DIR:-}" ]; then
  mkdir -p "$SOL_EVIDENCE_DIR"
  cp "$tmp/positive/Cargo.lock" "$SOL_EVIDENCE_DIR/git-consumer.Cargo.lock"
  cp "$tmp/metadata.json" "$SOL_EVIDENCE_DIR/git-consumer-metadata.json"
  cp "$tmp/test.log" "$SOL_EVIDENCE_DIR/git-consumer-tests.log"
  cp "$tmp/negative.log" "$SOL_EVIDENCE_DIR/git-consumer-negative-version.log"
  printf 'git_revision=%s\nplatform=%s/%s\nrustc=%s\ncargo=%s\nmode=git-pinned-source-not-release\n' \
    "$pin" "$(uname -s)" "$(uname -m)" \
    "$(rustc --version)" "$(cargo --version)" > "$SOL_EVIDENCE_DIR/git-consumer-environment.txt"
fi
echo "[SOL P1] PASS: 4 independently resolved Git SDK tests and 1 incompatible-version negative"
