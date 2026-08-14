#!/usr/bin/env python3
from __future__ import annotations

import argparse
import hashlib
import json
import subprocess
import tomllib
from pathlib import Path


def canonical_bytes(value: object) -> bytes:
    return (json.dumps(value, sort_keys=True, separators=(",", ":")) + "\n").encode()


def git_value(root: Path, *args: str) -> str:
    try:
        p = subprocess.run(["git", *args], cwd=root, text=True, capture_output=True, check=True)
        return p.stdout.strip()
    except Exception:
        return ""


def load_manifest(path: Path) -> dict:
    with path.open("rb") as f:
        return tomllib.load(f)


def inspect(root: Path, manifest_path: Path) -> dict:
    manifest = load_manifest(manifest_path)
    defs = manifest.get("definitions", [])
    scopes = [d.get("scope", "") for d in defs]
    required_scopes = manifest["meta"]["required_scopes"]
    duplicate_scopes = sorted({s for s in scopes if scopes.count(s) > 1})
    missing_scopes = [s for s in required_scopes if s not in scopes]

    required_files = manifest.get("required_files", [])
    file_results = []
    missing_files = []
    for item in required_files:
        rel = item["path"]
        path = root / rel
        exists = path.is_file()
        digest = hashlib.sha256(path.read_bytes()).hexdigest() if exists else ""
        file_results.append({"path": rel, "kind": item["kind"], "exists": exists, "sha256": digest})
        if not exists:
            missing_files.append(rel)

    definition_errors = []
    for d in defs:
        if not d.get("requires"):
            definition_errors.append(f"{d.get('scope','?')}: no requirements")
        if not d.get("falsifiers"):
            definition_errors.append(f"{d.get('scope','?')}: no falsifiers")

    head = git_value(root, "rev-parse", "HEAD")
    tree = git_value(root, "rev-parse", "HEAD^{tree}")

    thresholds = manifest.get("thresholds", {})
    enough_defs = len(defs) >= int(thresholds.get("required_definitions", 0))
    enough_files = len(required_files) >= int(thresholds.get("required_files", 0))

    falsifiers = {
        "missing_scope": bool(missing_scopes),
        "duplicate_scope": bool(duplicate_scopes),
        "definition_without_requirements_or_falsifiers": bool(definition_errors),
        "missing_required_file": bool(missing_files),
        "definition_threshold_failed": not enough_defs,
        "required_file_threshold_failed": not enough_files,
    }
    alive = not any(falsifiers.values())

    receipt = {
        "name": "PROCESS_INTELLIGENCE_DEFINITION_OF_DONE",
        "version": manifest["meta"]["version"],
        "status": "ALIVE" if alive else "REFUSED",
        "standing": "RELEASE_CANDIDATE_DONE" if alive else "NOT_DONE",
        "subject": {"head": head, "tree": tree},
        "manifest": {
            "path": str(manifest_path.relative_to(root)),
            "sha256": hashlib.sha256(manifest_path.read_bytes()).hexdigest(),
        },
        "definitions": {
            "required": required_scopes,
            "observed": scopes,
            "missing": missing_scopes,
            "duplicates": duplicate_scopes,
            "errors": definition_errors,
        },
        "required_files": file_results,
        "falsifiers": falsifiers,
    }
    digest_base = dict(receipt)
    receipt["receipt_sha256"] = hashlib.sha256(canonical_bytes(digest_base)).hexdigest()
    return receipt


def main() -> int:
    p = argparse.ArgumentParser(description="Verify process-intelligence Definition of Done structure.")
    p.add_argument("--root", default=".")
    p.add_argument("--manifest", default="dod/process-intelligence.toml")
    p.add_argument("--output", default="")
    p.add_argument("--check", action="store_true")
    args = p.parse_args()

    root = Path(args.root).resolve()
    manifest_path = (root / args.manifest).resolve()
    receipt = inspect(root, manifest_path)
    data = canonical_bytes(receipt)
    if args.output:
        out = root / args.output
        out.parent.mkdir(parents=True, exist_ok=True)
        out.write_bytes(data)
    else:
        print(data.decode(), end="")
    if args.check and receipt["status"] != "ALIVE":
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
