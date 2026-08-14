#!/usr/bin/env python3
"""Machine-verifiable PROCESS_INTELLIGENCE_ALIVE_002 quality gate.

This verifier implements the prospective ALIVE_002 criteria recorded in
checkpoints/ALIVE_GATE_ASSESSMENT.md and the open-gap mitigation invariant in
COVENANT.md. It is intentionally dependency-free and emits a deterministic JSON
receipt suitable for replay and checkpoint attachment.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import subprocess
import sys
from dataclasses import asdict, dataclass
from pathlib import Path
from typing import Iterable

WORD_RE = re.compile(r"\b[\w’'-]+\b", re.UNICODE)
DOI_RE = re.compile(r"(?:https?://(?:dx\.)?doi\.org/|\bdoi\s*:\s*|\b)(10\.\d{4,9}/[-._;()/:A-Z0-9]+)", re.I)
AUTHOR_YEAR_RE = re.compile(r"\([^\n)]*[A-Za-z][^\n)]*(?:19|20)\d{2}[a-z]?[^\n)]*\)")
STATUS_RE = re.compile(r"^(?:\*\*Status(?::\*\*|\*\*:)|status:)\s*(.+?)\s*$", re.I | re.M)
OPEN_STATUS_RE = re.compile(r"\b(OPEN|PARTIAL(?:_ALIVE)?|BLOCKED|BUILD_BROKEN|UNKNOWN)\b", re.I)
CLOSED_STATUS_RE = re.compile(r"\b(CLOSED|RESOLVED|ALIVE)\b", re.I)


@dataclass(frozen=True)
class Evidence:
    path: str
    sha256: str
    detail: str


def markdown_files(root: Path, relative: str) -> list[Path]:
    directory = root / relative
    return sorted(p for p in directory.glob("*.md") if p.is_file()) if directory.is_dir() else []


def read(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def evidence(root: Path, path: Path, detail: str) -> Evidence:
    return Evidence(path.as_posix().removeprefix(root.as_posix().rstrip("/") + "/"), digest(path), detail)


def git_value(root: Path, *args: str) -> str | None:
    try:
        cp = subprocess.run(
            ["git", "-C", str(root), *args],
            check=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.DEVNULL,
            text=True,
        )
        return cp.stdout.strip() or None
    except (FileNotFoundError, subprocess.CalledProcessError):
        return None


def doctrine_probe(root: Path) -> list[Evidence]:
    out: list[Evidence] = []
    for path in markdown_files(root, "doctrine"):
        text = read(path)
        words = len(WORD_RE.findall(text))
        header = "## Definition" if re.search(r"^## Definition\s*$", text, re.M) else (
            "## Law" if re.search(r"^## Law\s*$", text, re.M) else None
        )
        if words >= 200 and header:
            out.append(evidence(root, path, f"words={words}; header={header}"))
    return out


def standards_probe(root: Path) -> list[Evidence]:
    out: list[Evidence] = []
    for path in markdown_files(root, "standards"):
        text = read(path)
        headers = [h for h in ("## Coverage", "## Compliance") if re.search(rf"^{re.escape(h)}\s*$", text, re.M)]
        if headers:
            out.append(evidence(root, path, "headers=" + ",".join(headers)))
    return out


def paper_probe(root: Path) -> list[Evidence]:
    out: list[Evidence] = []
    for path in markdown_files(root, "sources/papers"):
        text = read(path)
        doi = DOI_RE.search(text)
        author_year = AUTHOR_YEAR_RE.search(text)
        if doi or author_year:
            kind = f"doi={doi.group(1)}" if doi else f"author_year={author_year.group(0)[:120]}"
            out.append(evidence(root, path, kind))
    return out


def status_of(text: str) -> str | None:
    match = STATUS_RE.search(text)
    return match.group(1).strip() if match else None


def is_open_gap(text: str) -> bool:
    status = status_of(text)
    if not status:
        return False
    if CLOSED_STATUS_RE.search(status):
        return False
    return bool(OPEN_STATUS_RE.search(status))


def open_gap_inventory(root: Path) -> list[dict]:
    accepted = re.compile(r"^## (?:Resolution Path|Required Remediation Path|Remediation Path|Mitigation Path)\s*$", re.M)
    exact = re.compile(r"^## Resolution Path\s*$", re.M)
    rows: list[dict] = []
    for path in markdown_files(root, "gaps"):
        text = read(path)
        if is_open_gap(text):
            rows.append({
                "path": path.relative_to(root).as_posix(),
                "status": status_of(text),
                "has_exact_resolution_path": bool(exact.search(text)),
                "has_mitigation_path": bool(accepted.search(text)),
                "sha256": digest(path),
            })
    return rows


def exact_resolution_path_probe(root: Path) -> list[Evidence]:
    out: list[Evidence] = []
    for path in markdown_files(root, "gaps"):
        text = read(path)
        if is_open_gap(text) and re.search(r"^## Resolution Path\s*$", text, re.M):
            out.append(evidence(root, path, f"status={status_of(text)}; header=## Resolution Path"))
    return out


def unmitigated_open_gaps(root: Path) -> list[str]:
    accepted = re.compile(r"^## (?:Resolution Path|Required Remediation Path|Remediation Path|Mitigation Path)\s*$", re.M)
    missing: list[str] = []
    for path in markdown_files(root, "gaps"):
        text = read(path)
        if is_open_gap(text) and not accepted.search(text):
            missing.append(path.relative_to(root).as_posix())
    return missing


def build_receipt(root: Path) -> dict:
    probes = {
        "doctrine": doctrine_probe(root),
        "standards": standards_probe(root),
        "papers": paper_probe(root),
        "open_gaps_with_exact_resolution_path": exact_resolution_path_probe(root),
    }
    thresholds = {
        "doctrine": 5,
        "standards": 10,
        "papers": 7,
        "open_gaps_with_exact_resolution_path": 2,
    }
    counts = {name: len(items) for name, items in probes.items()}
    criteria = {name: counts[name] >= threshold for name, threshold in thresholds.items()}
    open_gaps = open_gap_inventory(root)
    unmitigated = [row["path"] for row in open_gaps if not row["has_mitigation_path"]]
    criteria["all_open_gaps_have_mitigation_path"] = not unmitigated
    passed = all(criteria.values())

    return {
        "schema": "process-intelligence.alive-002.receipt/v1",
        "subject": {
            "repository": "seanchatmangpt/process-intelligence",
            "git_head": git_value(root, "rev-parse", "HEAD"),
            "git_tree": git_value(root, "rev-parse", "HEAD^{tree}"),
        },
        "gate": "PROCESS_INTELLIGENCE_ALIVE_002",
        "status": "ALIVE" if passed else "PARTIAL_ALIVE",
        "criteria": criteria,
        "thresholds": thresholds,
        "counts": counts,
        "open_gaps": open_gaps,
        "unmitigated_open_gaps": unmitigated,
        "evidence": {name: [asdict(item) for item in items] for name, items in probes.items()},
    }


def main(argv: Iterable[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    parser.add_argument("--receipt", type=Path)
    parser.add_argument("--expect-head", help="fail if the current git HEAD differs")
    args = parser.parse_args(list(argv) if argv is not None else None)

    root = args.root.resolve()
    receipt = build_receipt(root)
    actual_head = receipt["subject"]["git_head"]
    if args.expect_head and actual_head != args.expect_head:
        receipt["criteria"]["expected_head_matches"] = False
        receipt["status"] = "PARTIAL_ALIVE"
        receipt["expected_head"] = args.expect_head

    payload = json.dumps(receipt, indent=2, sort_keys=True) + "\n"
    if args.receipt:
        args.receipt.parent.mkdir(parents=True, exist_ok=True)
        args.receipt.write_text(payload, encoding="utf-8")
    sys.stdout.write(payload)
    return 0 if receipt["status"] == "ALIVE" else 2


if __name__ == "__main__":
    raise SystemExit(main())
