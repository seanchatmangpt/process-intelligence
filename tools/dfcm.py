#!/usr/bin/env python3
"""Design for Combinatorial Maximalism (DFCM) construct-space compiler.

This tool is intentionally CONSTRUCT_ONLY. It enumerates and scores reversible
candidate designs. Any candidate requesting machine-state actuation is REFUSED.
"""
from __future__ import annotations

import argparse
import hashlib
import itertools
import json
import subprocess
import sys
import tomllib
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Iterable

STATUS_ALIVE = "ALIVE"
STATUS_REFUSED = "REFUSED"
REASON_DO = "REFUSED_DO_PATH"
REASON_CONSTRAINT = "REFUSED_CONSTRAINT"


@dataclass(frozen=True)
class Axis:
    name: str
    values: tuple[str, ...]


@dataclass(frozen=True)
class Rule:
    when: tuple[tuple[str, str], ...]
    require_axis: str
    allowed: tuple[str, ...]
    reason: str


@dataclass(frozen=True)
class Exclusion:
    match: tuple[tuple[str, str], ...]
    reason: str


def canonical_json(value: Any) -> bytes:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode()


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def git_value(root: Path, *args: str) -> str | None:
    try:
        proc = subprocess.run(
            ["git", *args], cwd=root, text=True, capture_output=True, check=False
        )
    except OSError:
        return None
    if proc.returncode != 0:
        return None
    return proc.stdout.strip() or None


def load_manifest(path: Path) -> dict[str, Any]:
    with path.open("rb") as handle:
        data = tomllib.load(handle)
    validate_manifest(data)
    return data


def validate_manifest(data: dict[str, Any]) -> None:
    meta = data.get("meta", {})
    if meta.get("mode") != "CONSTRUCT_ONLY":
        raise ValueError("DFCM manifest must declare meta.mode = 'CONSTRUCT_ONLY'")
    axes = data.get("axes")
    if not isinstance(axes, list) or not axes:
        raise ValueError("manifest must define at least one [[axes]] entry")
    names: set[str] = set()
    for axis in axes:
        name = axis.get("name")
        values = axis.get("values")
        if not isinstance(name, str) or not name:
            raise ValueError("axis name must be non-empty")
        if name in names:
            raise ValueError(f"duplicate axis: {name}")
        names.add(name)
        if not isinstance(values, list) or not values or not all(isinstance(v, str) and v for v in values):
            raise ValueError(f"axis {name} must define non-empty string values")
        if len(values) != len(set(values)):
            raise ValueError(f"axis {name} contains duplicate values")
    policy = data.get("policy", {})
    actuation_axis = policy.get("actuation_axis")
    safe_value = policy.get("construct_value")
    if actuation_axis not in names:
        raise ValueError("policy.actuation_axis must name an axis")
    if safe_value not in next(a["values"] for a in axes if a["name"] == actuation_axis):
        raise ValueError("policy.construct_value must be a value of actuation_axis")


def axes_from(data: dict[str, Any]) -> tuple[Axis, ...]:
    return tuple(Axis(a["name"], tuple(a["values"])) for a in data["axes"])


def rules_from(data: dict[str, Any]) -> tuple[Rule, ...]:
    result: list[Rule] = []
    for raw in data.get("rules", []):
        when = tuple(sorted((str(k), str(v)) for k, v in raw.get("when", {}).items()))
        result.append(
            Rule(
                when=when,
                require_axis=str(raw["require_axis"]),
                allowed=tuple(str(v) for v in raw["allowed"]),
                reason=str(raw.get("reason", "constraint")),
            )
        )
    return tuple(result)


def exclusions_from(data: dict[str, Any]) -> tuple[Exclusion, ...]:
    return tuple(
        Exclusion(
            match=tuple(sorted((str(k), str(v)) for k, v in raw.get("match", {}).items())),
            reason=str(raw.get("reason", "excluded")),
        )
        for raw in data.get("exclusions", [])
    )


def ratings_from(data: dict[str, Any]) -> dict[tuple[str, str], dict[str, int]]:
    ratings: dict[tuple[str, str], dict[str, int]] = {}
    for raw in data.get("ratings", []):
        key = (str(raw["axis"]), str(raw["value"]))
        ratings[key] = {
            "reversibility": int(raw.get("reversibility", 0)),
            "evidence": int(raw.get("evidence", 0)),
            "coverage": int(raw.get("coverage", 0)),
            "cost": int(raw.get("cost", 0)),
        }
    return ratings


def matches(candidate: dict[str, str], clauses: Iterable[tuple[str, str]]) -> bool:
    return all(candidate.get(axis) == value for axis, value in clauses)


def refusal_reason(candidate: dict[str, str], data: dict[str, Any]) -> dict[str, str] | None:
    policy = data["policy"]
    if candidate[policy["actuation_axis"]] != policy["construct_value"]:
        return {"code": REASON_DO, "detail": "DFCM compiler is CONSTRUCT_ONLY; DO requires an external receipted actuator"}
    for exclusion in exclusions_from(data):
        if matches(candidate, exclusion.match):
            return {"code": REASON_CONSTRAINT, "detail": exclusion.reason}
    for rule in rules_from(data):
        if matches(candidate, rule.when) and candidate.get(rule.require_axis) not in rule.allowed:
            return {"code": REASON_CONSTRAINT, "detail": rule.reason}
    return None


def score(candidate: dict[str, str], data: dict[str, Any]) -> int:
    weights = {k: int(v) for k, v in data.get("weights", {}).items()}
    ratings = ratings_from(data)
    total = 0
    for axis, value in candidate.items():
        r = ratings.get((axis, value), {})
        total += weights.get("reversibility", 0) * r.get("reversibility", 0)
        total += weights.get("evidence", 0) * r.get("evidence", 0)
        total += weights.get("coverage", 0) * r.get("coverage", 0)
        total -= weights.get("cost", 0) * r.get("cost", 0)
    return total


def enumerate_candidates(data: dict[str, Any]) -> tuple[list[dict[str, Any]], list[dict[str, Any]]]:
    axes = axes_from(data)
    admitted: list[dict[str, Any]] = []
    refused: list[dict[str, Any]] = []
    for values in itertools.product(*(axis.values for axis in axes)):
        candidate = dict(zip((axis.name for axis in axes), values, strict=True))
        digest = sha256_bytes(canonical_json(candidate))
        refusal = refusal_reason(candidate, data)
        if refusal:
            refused.append({"candidate": candidate, "digest": digest, "refusal": refusal})
        else:
            admitted.append({"candidate": candidate, "digest": digest, "score": score(candidate, data)})
    admitted.sort(key=lambda row: (-row["score"], row["digest"]))
    refused.sort(key=lambda row: (row["refusal"]["code"], row["digest"]))
    return admitted, refused


def compile_plan(manifest_path: Path, root: Path) -> dict[str, Any]:
    manifest_bytes = manifest_path.read_bytes()
    data = load_manifest(manifest_path)
    admitted, refused = enumerate_candidates(data)
    limit = int(data["meta"].get("select_limit", 64))
    selected = admitted[:limit]
    do_refusals = sum(1 for row in refused if row["refusal"]["code"] == REASON_DO)
    constraint_refusals = len(refused) - do_refusals
    cartesian_total = len(admitted) + len(refused)
    min_admitted = int(data["meta"].get("min_admitted", 1))
    status = STATUS_ALIVE if len(admitted) >= min_admitted and do_refusals > 0 else STATUS_REFUSED
    payload = {
        "schema": "chatmangpt.dfcm.receipt.v1",
        "status": status,
        "mode": "CONSTRUCT_ONLY",
        "manifest": {
            "path": manifest_path.as_posix(),
            "sha256": sha256_bytes(manifest_bytes),
            "name": data["meta"]["name"],
            "version": str(data["meta"]["version"]),
        },
        "subject": {
            "head": git_value(root, "rev-parse", "HEAD"),
            "tree": git_value(root, "rev-parse", "HEAD^{tree}"),
        },
        "space": {
            "cartesian_total": cartesian_total,
            "admitted_constructs": len(admitted),
            "refused_do": do_refusals,
            "refused_constraints": constraint_refusals,
            "selected": len(selected),
        },
        "selected": selected,
        "falsifiers": {
            "do_path_admitted": False,
            "minimum_admitted": len(admitted) >= min_admitted,
            "deterministic_ordering": True,
        },
    }
    payload["receipt_sha256"] = sha256_bytes(canonical_json(payload))
    return payload


def cmd_plan(args: argparse.Namespace) -> int:
    manifest = Path(args.manifest).resolve()
    root = Path(args.root).resolve()
    receipt = compile_plan(manifest, root)
    encoded = json.dumps(receipt, indent=2, sort_keys=True) + "\n"
    if args.output:
        Path(args.output).write_text(encoded, encoding="utf-8")
    else:
        sys.stdout.write(encoded)
    if args.check and receipt["status"] != STATUS_ALIVE:
        return 2
    return 0


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="DFCM reversible construct-space compiler")
    sub = parser.add_subparsers(dest="command", required=True)
    plan = sub.add_parser("plan", help="enumerate, constrain, score, select, and receipt a design space")
    plan.add_argument("manifest")
    plan.add_argument("--root", default=".")
    plan.add_argument("--output")
    plan.add_argument("--check", action="store_true")
    plan.set_defaults(func=cmd_plan)
    return parser


def main() -> int:
    args = build_parser().parse_args()
    try:
        return int(args.func(args))
    except (OSError, ValueError, tomllib.TOMLDecodeError) as exc:
        print(f"REFUSED_MANIFEST: {exc}", file=sys.stderr)
        return 3


if __name__ == "__main__":
    raise SystemExit(main())
