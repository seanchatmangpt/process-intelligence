#!/usr/bin/env python3
"""Executable PROCESS_INTELLIGENCE_ALIVE_002 gate; see checkpoint addendum 004."""
from __future__ import annotations

import argparse, hashlib, json, re, subprocess, sys
from pathlib import Path

WORD = re.compile(r"\b[\w’'-]+\b", re.UNICODE)
STATUS = re.compile(r"^(?:\*\*Status(?::\*\*|\*\*:)|status:)\s*(.+?)\s*$", re.I | re.M)
OPEN = re.compile(r"\b(?:OPEN|PARTIAL(?:_ALIVE)?|BLOCKED|BUILD_BROKEN|UNKNOWN)\b", re.I)
CLOSED = re.compile(r"\b(?:CLOSED|RESOLVED|ALIVE)\b", re.I)
DOI = re.compile(r"(?:doi\.org/|\bdoi\s*:\s*|\b)(10\.\d{4,9}/[-._;()/:A-Z0-9]+)", re.I)
AUTHOR_YEAR = re.compile(r"\([^\n)]*[A-Za-z][^\n)]*(?:19|20)\d{2}[a-z]?[^\n)]*\)")
SOURCE = re.compile(r"(?:\*\*(?:Paper|Source|Authority)|checkpoints/|sources/papers/|doi\b)", re.I)
SEMANTIC_STANDARD = re.compile(r"\b(?:coverage|compliance|implementation|mapping|standard overview)\b", re.I)
AUTHORITY = re.compile(r"^\*\*Authority:\*\*", re.I | re.M)
RESOLUTION = re.compile(r"^## (?:Resolution Path|Required Remediation Path|Remediation Path|Mitigation Path)\s*$", re.M)


def files(root: Path, rel: str) -> list[Path]:
    d = root / rel
    return sorted(d.glob("*.md")) if d.is_dir() else []


def text(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def ev(root: Path, path: Path, detail: str) -> dict:
    return {"path": path.relative_to(root).as_posix(), "sha256": sha(path), "detail": detail}


def git(root: Path, expr: str) -> str | None:
    try:
        return subprocess.run(["git", "-C", str(root), "rev-parse", expr], check=True,
                              capture_output=True, text=True).stdout.strip() or None
    except (FileNotFoundError, subprocess.CalledProcessError):
        return None


def status_of(s: str) -> str | None:
    m = STATUS.search(s)
    return m.group(1).strip() if m else None


def is_open(s: str) -> bool:
    status = status_of(s)
    return bool(status and not CLOSED.search(status) and OPEN.search(status))


def doctrine(root: Path) -> list[dict]:
    out = []
    for p in files(root, "doctrine"):
        s = text(p); words = len(WORD.findall(s))
        hs = re.findall(r"^#{1,2}\s+(.+?)\s*$", s, re.M)
        law = [h for h in hs if re.search(r"\b(?:law|definition)\b", h, re.I)]
        if words >= 200 and law and SOURCE.search(s):
            out.append(ev(root, p, f"words={words}; heading={law[0][:120]}; source=true"))
    return out


def standards(root: Path) -> list[dict]:
    out = []
    for p in files(root, "standards"):
        s = text(p); hs = re.findall(r"^##\s+(.+?)\s*$", s, re.M)
        sem = [h for h in hs if SEMANTIC_STANDARD.search(h)]
        if AUTHORITY.search(s) and sem:
            out.append(ev(root, p, f"authority=true; heading={sem[0][:120]}"))
    return out


def papers(root: Path) -> list[dict]:
    out = []
    for p in files(root, "sources/papers"):
        s = text(p); doi = DOI.search(s); ay = AUTHOR_YEAR.search(s)
        if doi or ay:
            out.append(ev(root, p, f"citation={doi.group(1) if doi else ay.group(0)[:120]}"))
    return out


def gaps(root: Path) -> tuple[list[dict], list[dict], list[str]]:
    opened, proven, missing = [], [], []
    for p in files(root, "gaps"):
        s = text(p)
        if not is_open(s):
            continue
        row = {"path": p.relative_to(root).as_posix(), "status": status_of(s), "sha256": sha(p)}
        opened.append(row)
        m = RESOLUTION.search(s)
        if m:
            proven.append(ev(root, p, f"status={status_of(s)}; heading={m.group(0)}"))
        else:
            missing.append(row["path"])
    return opened, proven, missing


def build_receipt(root: Path) -> dict:
    evidence = {"doctrine": doctrine(root), "standards": standards(root), "papers": papers(root)}
    opened, gap_evidence, missing = gaps(root)
    evidence["open_gaps_with_resolution_path"] = gap_evidence
    counts = {k: len(v) for k, v in evidence.items()}
    criteria = {
        "doctrine": counts["doctrine"] >= 5,
        "standards": counts["standards"] >= 10,
        "papers": counts["papers"] >= 7,
        "all_open_gaps_have_resolution_path": len(gap_evidence) == len(opened) and not missing,
    }
    return {
        "schema": "process-intelligence.alive-002.receipt/v2",
        "gate": "PROCESS_INTELLIGENCE_ALIVE_002",
        "status": "ALIVE" if all(criteria.values()) else "PARTIAL_ALIVE",
        "subject": {"repository": "seanchatmangpt/process-intelligence", "git_head": git(root, "HEAD"),
                    "git_tree": git(root, "HEAD^{tree}")},
        "criteria": criteria,
        "thresholds": {"doctrine": 5, "standards": 10, "papers": 7},
        "counts": counts,
        "open_gaps": opened,
        "unmitigated_open_gaps": missing,
        "evidence": evidence,
    }


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    ap.add_argument("--receipt", type=Path)
    ap.add_argument("--expect-head")
    a = ap.parse_args(); root = a.root.resolve(); r = build_receipt(root)
    if a.expect_head and r["subject"]["git_head"] != a.expect_head:
        r["criteria"]["expected_head_matches"] = False; r["status"] = "PARTIAL_ALIVE"
        r["expected_head"] = a.expect_head
    payload = json.dumps(r, indent=2, sort_keys=True) + "\n"
    if a.receipt:
        a.receipt.parent.mkdir(parents=True, exist_ok=True); a.receipt.write_text(payload, encoding="utf-8")
    sys.stdout.write(payload)
    return 0 if r["status"] == "ALIVE" else 2


if __name__ == "__main__":
    raise SystemExit(main())
