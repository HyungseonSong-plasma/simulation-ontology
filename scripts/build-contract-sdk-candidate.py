#!/usr/bin/env python3
"""Build/verify a *candidate* SOL Contract SDK source archive, never publish it.

No release tag or official package is created. The archive contains SOL-owned
contract packages, schemas, fixtures and docs, with normalized tar metadata.
Requires only Python 3 standard library (except Cargo for subsequent tests).
"""
from __future__ import annotations

import argparse
import gzip
import hashlib
import io
import json
import os
from pathlib import Path
import subprocess
import tarfile

ROOT = Path(__file__).resolve().parent.parent
NAME = "sol-contract-sdk-candidate"
SUBDIRS = (
    "crates/sol-public-contract",
    "crates/sol-adapter-protocol",
    "crates/sol-adapter-transport",
    "schemas",
    "fixtures",
    "docs/contracts",
    "docs/implementation",
)
WORKSPACE = """[workspace]
resolver = "2"
members = [
    "crates/sol-public-contract",
    "crates/sol-adapter-protocol",
    "crates/sol-adapter-transport",
]

[workspace.package]
version = "0.1.0"
edition = "2021"
license = "MIT OR Apache-2.0"
rust-version = "1.80"
"""


def sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def archive_files() -> dict[str, bytes]:
    result = {"Cargo.toml": WORKSPACE.encode("utf-8")}
    for section in SUBDIRS:
        base = ROOT / section
        if not base.is_dir():
            raise SystemExit(f"Source tree directory missing: {section}")
        for file in sorted(base.rglob("*")):
            if not file.is_file() or file.is_symlink():
                continue
            rel = file.relative_to(ROOT).as_posix()
            if file.name.endswith(".pyc") or "__pycache__" in file.parts:
                continue
            result[rel] = file.read_bytes()
    return result


def tar_members(payload: dict[str, bytes]) -> bytes:
    buf = io.BytesIO()
    with gzip.GzipFile(filename="", fileobj=buf, mode="wb", mtime=0, compresslevel=9) as gz:
        with tarfile.open(fileobj=gz, mode="w", format=tarfile.USTAR_FORMAT) as archive:
            for name, value in sorted(payload.items()):
                tarinfo = tarfile.TarInfo(f"{NAME}/{name}")
                tarinfo.size = len(value)
                tarinfo.mode = 0o644
                tarinfo.mtime = 0
                tarinfo.uid = tarinfo.gid = 0
                tarinfo.uname = tarinfo.gname = ""
                archive.addfile(tarinfo, io.BytesIO(value))
    return buf.getvalue()


def make_manifest(files: dict[str, bytes], source_sha: str) -> bytes:
    entries = {name: {"bytes": len(data), "sha256": sha(data)} for name, data in sorted(files.items())}
    manifest = {
        "kind": "SOL_CONTRACT_SDK_SOURCE_CANDIDATE_NOT_A_RELEASE",
        "schema_version": 1,
        "source_repository": "HyungseonSong-plasma/simulation-ontology",
        "source_commit": source_sha,
        "crate_package_version": "0.1.0",
        "supported_adapter_protocol_versions": ["0.1", "0.2"],
        "supported_public_contract_versions": ["0.1", "0.2", "0.3-scalar-only"],
        "note": "This is a candidate source archive: 0.3 dataset and Adapter Protocol 0.3 are NOT accepted. Distribution and security approval pending.",
        "requires": "Rust >=1.80, Cargo; no solver runtime bundled.",
        "license_declaration": "MIT OR Apache-2.0 (review upstream license files/metadata before release)",
        "files": entries,
    }
    return (json.dumps(manifest, sort_keys=True, indent=2) + "\n").encode("utf-8")


def verify(out: Path) -> None:
    target = out / f"{NAME}.tar.gz"
    checks = out / f"{NAME}.sha256"
    if not target.is_file() or not checks.is_file():
        raise SystemExit("Candidate tarball/checksum absent")
    expected = checks.read_text(encoding="utf-8").strip().split()
    if len(expected) != 2 or expected[1] != target.name or expected[0] != sha(target.read_bytes()):
        raise SystemExit("Archive checksum mismatch")
    with tarfile.open(target, "r:gz") as src:
        content = {}
        for member in src.getmembers():
            if not member.isfile() or not member.name.startswith(NAME + "/"):
                raise SystemExit(f"Invalid archive member: {member.name}")
            if member.mtime != 0 or member.uid != 0 or member.gid != 0:
                raise SystemExit(f"Nonreproducible archive metadata: {member.name}")
            path = member.name[len(NAME) + 1:]
            if path.startswith("/") or ".." in Path(path).parts or path in content:
                raise SystemExit(f"Unsafe or repeated path: {path}")
            content[path] = src.extractfile(member).read()
    manifest_bytes = content.pop("release-candidate-manifest.json", None)
    if manifest_bytes is None:
        raise SystemExit("Manifest absent")
    manifest = json.loads(manifest_bytes)
    if manifest["kind"] != "SOL_CONTRACT_SDK_SOURCE_CANDIDATE_NOT_A_RELEASE":
        raise SystemExit("Unexpected package status")
    if not isinstance(manifest["source_commit"], str) or len(manifest["source_commit"]) != 40:
        raise SystemExit("Missing exact source revision")
    entries = manifest["files"]
    if set(entries) != set(content):
        raise SystemExit("Manifest path coverage mismatch")
    for path, data in content.items():
        if entries[path] != {"bytes": len(data), "sha256": sha(data)}:
            raise SystemExit(f"Digest mismatch for {path}")
    for root in ("crates/sol-public-contract/Cargo.toml",
                 "crates/sol-adapter-protocol/Cargo.toml",
                 "crates/sol-adapter-transport/Cargo.toml",
                 "Cargo.toml"):
        if root not in content:
            raise SystemExit(f"Required crate missing: {root}")
    # Re-serialize sorted bytes to verify reproducibility and no additional payloads.
    payload = dict(content)
    payload["release-candidate-manifest.json"] = manifest_bytes
    if tar_members(payload) != target.read_bytes():
        raise SystemExit("Deterministic rebuild mismatch")
    print(f"[SOL P2] PASS: {len(content)} verified source/fixture/schema files; archive sha256 {expected[0]}")
    print(f"[SOL P2] Candidate only, source_commit={manifest['source_commit']}")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--output-dir", type=Path, required=True)
    parser.add_argument("--verify-only", action="store_true")
    args = parser.parse_args()
    out = args.output_dir
    if args.verify_only:
        verify(out)
        return
    out.mkdir(parents=True, exist_ok=True)
    source_sha = subprocess.check_output(["git", "-C", str(ROOT), "rev-parse", "HEAD"], text=True).strip()
    if len(source_sha) != 40:
        raise SystemExit("Invalid source Git SHA")
    files = archive_files()
    files["release-candidate-manifest.json"] = make_manifest(files, source_sha)
    data = tar_members(files)
    target = out / f"{NAME}.tar.gz"
    target.write_bytes(data)
    (out / f"{NAME}.sha256").write_text(f"{sha(data)}  {target.name}\n", encoding="utf-8")
    verify(out)


if __name__ == "__main__":
    main()
