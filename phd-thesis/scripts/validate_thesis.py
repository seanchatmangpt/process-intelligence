#!/usr/bin/env python3
"""
validate_thesis.py — PhD Thesis Manufacturing Validation Script
Sean Chatman, Process Intelligence Research Foundry

Checks:
  1. All 13 chapter files exist
  2. All project directories have exactly 8 TeX files
  3. main.tex exists
  4. claim_ledger.yaml exists in each project dir
  5. WORKFLOW_RECEIPT.yaml exists in ledgers/

Prints ALIVE / PARTIAL / BLOCKED verdict with details.

Usage:
  python3 validate_thesis.py
  python3 validate_thesis.py --receipt-only
"""

import argparse
import sys
from pathlib import Path
from typing import NamedTuple

# ============================================================
# Configuration
# ============================================================

THESIS_DIR = Path("/Users/sac/process-intelligence/phd-thesis")

REQUIRED_CHAPTERS = [
    "chapters/00_preface.tex",
    "chapters/01_origin_2016_language_model.tex",
    "chapters/02_prediction_is_not_coordination.tex",
    "chapters/03_enterprise_process_gap.tex",
    "chapters/04_chatman_equation.tex",
    "chapters/05_process_evidence_and_receipts.tex",
    "chapters/06_ggen_and_open_ontologies.tex",
    "chapters/07_command_grammar_and_execution.tex",
    "chapters/08_post_cyberpunk_pcp.tex",
    "chapters/09_ai_xynz_and_capital_flow.tex",
    "chapters/10_industry_complete_architecture.tex",
    "chapters/11_evaluation_and_receipts.tex",
    "chapters/12_conclusion.tex",
]

REQUIRED_FRONTMATTER = [
    "frontmatter/abstract.tex",
    "frontmatter/dedication.tex",
    "frontmatter/acknowledgements.tex",
    "frontmatter/declaration.tex",
]

REQUIRED_ROOT_FILES = [
    "main.tex",
    "glossary.tex",
    "bibliography.bib",
    "Makefile",
]

WORKFLOW_RECEIPT = "ledgers/WORKFLOW_RECEIPT.yaml"

# Expected number of TeX files in each project directory
EXPECTED_TEX_COUNT = 8

# All project directories (from projects/ subdirectory)
ALL_PROJECTS = [
    "adversarial",
    "atlas",
    "audits",
    "blue-river-dam",
    "checkpoints",
    "comparisons",
    "crosswalks",
    "doctrine",
    "experiments-core",
    "experiments-visualizer",
    "experiments-visualizer-nextjs",
    "gaps",
    "ggen",
    "lifecycle",
    "livestream",
    "ma",
    "otel-weaver",
    "otel-weaver-exp-001-custom-pi-registry",
    "otel-weaver-exp-002-diff-to-residual",
    "otel-weaver-exp-003-live-check-to-refusal",
    "otel-weaver-exp-004-registry-to-witness",
    "otel-weaver-exp-005-collector-to-intake",
    "phd-thesis",
    "prompts",
    "receipts",
    "research-open-ontologies",
    "research-pi-program",
    "research-prompt-manufactory",
    "schemas",
    "sources-papers",
    "sources-pm4py",
    "sources-wasm4pm",
    "sources-wasm4pm-compat",
    "standards",
]


# ============================================================
# Result types
# ============================================================

class Gate(NamedTuple):
    name: str
    passed: bool
    details: list[str]


class ValidationResult(NamedTuple):
    gates: list[Gate]

    @property
    def all_passed(self) -> bool:
        return all(g.passed for g in self.gates)

    @property
    def any_passed(self) -> bool:
        return any(g.passed for g in self.gates)

    @property
    def verdict(self) -> str:
        if self.all_passed:
            return "ALIVE"
        elif self.any_passed:
            return "PARTIAL"
        else:
            return "BLOCKED"


# ============================================================
# Gate checks
# ============================================================

def check_main_tex() -> Gate:
    """Gate 1: main.tex and required root files exist."""
    missing = []
    for rel in REQUIRED_ROOT_FILES:
        path = THESIS_DIR / rel
        if not path.exists():
            missing.append(str(rel))

    return Gate(
        name="root-files",
        passed=len(missing) == 0,
        details=missing if missing else ["All root files present"],
    )


def check_chapters() -> Gate:
    """Gate 2: All 13 chapter files exist."""
    missing = []
    for rel in REQUIRED_CHAPTERS:
        path = THESIS_DIR / rel
        if not path.exists():
            missing.append(str(rel))

    return Gate(
        name="chapters",
        passed=len(missing) == 0,
        details=missing if missing else [
            f"All {len(REQUIRED_CHAPTERS)} chapter files present"
        ],
    )


def check_frontmatter() -> Gate:
    """Gate 3: All frontmatter files exist."""
    missing = []
    for rel in REQUIRED_FRONTMATTER:
        path = THESIS_DIR / rel
        if not path.exists():
            missing.append(str(rel))

    return Gate(
        name="frontmatter",
        passed=len(missing) == 0,
        details=missing if missing else [
            f"All {len(REQUIRED_FRONTMATTER)} frontmatter files present"
        ],
    )


def check_project_tex_files() -> Gate:
    """Gate 4: All project directories have exactly 8 TeX files."""
    issues = []
    projects_dir = THESIS_DIR / "projects"

    if not projects_dir.exists():
        return Gate(
            name="project-tex-files",
            passed=False,
            details=["projects/ directory does not exist"],
        )

    for project in ALL_PROJECTS:
        project_dir = projects_dir / project
        if not project_dir.exists():
            issues.append(f"MISSING project dir: projects/{project}/")
            continue

        tex_files = list(project_dir.glob("*.tex"))
        count = len(tex_files)
        if count != EXPECTED_TEX_COUNT:
            issues.append(
                f"projects/{project}/: expected {EXPECTED_TEX_COUNT} TeX files, "
                f"found {count}"
            )

    return Gate(
        name="project-tex-files",
        passed=len(issues) == 0,
        details=issues if issues else [
            f"All {len(ALL_PROJECTS)} projects have exactly {EXPECTED_TEX_COUNT} TeX files"
        ],
    )


def check_claim_ledgers() -> Gate:
    """Gate 5: claim_ledger.yaml exists in each project directory."""
    missing = []
    projects_dir = THESIS_DIR / "projects"

    if not projects_dir.exists():
        return Gate(
            name="claim-ledgers",
            passed=False,
            details=["projects/ directory does not exist"],
        )

    for project in ALL_PROJECTS:
        project_dir = projects_dir / project
        if not project_dir.exists():
            missing.append(f"projects/{project}/ (directory missing)")
            continue

        ledger = project_dir / "claim_ledger.yaml"
        if not ledger.exists():
            missing.append(f"projects/{project}/claim_ledger.yaml")

    return Gate(
        name="claim-ledgers",
        passed=len(missing) == 0,
        details=missing if missing else [
            f"All {len(ALL_PROJECTS)} claim_ledger.yaml files present"
        ],
    )


def check_workflow_receipt() -> Gate:
    """Gate 6: WORKFLOW_RECEIPT.yaml exists in ledgers/."""
    receipt_path = THESIS_DIR / WORKFLOW_RECEIPT
    exists = receipt_path.exists()

    return Gate(
        name="workflow-receipt",
        passed=exists,
        details=(
            [str(receipt_path)]
            if exists
            else [f"MISSING: {WORKFLOW_RECEIPT}"]
        ),
    )


# ============================================================
# Output formatting
# ============================================================

VERDICT_COLORS = {
    "ALIVE":   "\033[92m",   # green
    "PARTIAL": "\033[93m",   # yellow
    "BLOCKED": "\033[91m",   # red
}
RESET = "\033[0m"
BOLD  = "\033[1m"


def fmt_verdict(verdict: str) -> str:
    color = VERDICT_COLORS.get(verdict, "")
    return f"{BOLD}{color}{verdict}{RESET}"


def fmt_gate(gate: Gate) -> str:
    status = "PASS" if gate.passed else "FAIL"
    color  = "\033[92m" if gate.passed else "\033[91m"
    lines  = [f"  {color}[{status}]{RESET} {gate.name}"]
    if not gate.passed:
        for detail in gate.details[:20]:   # cap output at 20 lines per gate
            lines.append(f"         {detail}")
        if len(gate.details) > 20:
            lines.append(f"         ... and {len(gate.details) - 20} more")
    return "\n".join(lines)


def print_report(result: ValidationResult, receipt_only: bool = False) -> None:
    print()
    print(f"{BOLD}=== PhD Thesis Validation Report ==={RESET}")
    print(f"Thesis dir : {THESIS_DIR}")
    print()

    if not receipt_only:
        for gate in result.gates:
            print(fmt_gate(gate))
            print()

    verdict = result.verdict
    print(f"Verdict    : {fmt_verdict(verdict)}")
    passed  = sum(1 for g in result.gates if g.passed)
    total   = len(result.gates)
    print(f"Gates      : {passed}/{total} passed")
    print()

    if verdict == "ALIVE":
        print(
            "All proof gates passed. The thesis manufacturing pipeline is verified."
        )
    elif verdict == "PARTIAL":
        failed = [g.name for g in result.gates if not g.passed]
        print(
            f"Some gates failed: {', '.join(failed)}. "
            "Resolve open items before submission."
        )
    else:
        print(
            "Core gates failed. The thesis manufacturing pipeline is BLOCKED. "
            "Remediate all failures before proceeding."
        )

    print()


# ============================================================
# Entry point
# ============================================================

def main() -> int:
    parser = argparse.ArgumentParser(
        description="Validate the PhD thesis manufacturing pipeline."
    )
    parser.add_argument(
        "--receipt-only",
        action="store_true",
        help="Check only the WORKFLOW_RECEIPT.yaml gate (skip full validation).",
    )
    args = parser.parse_args()

    if args.receipt_only:
        gates = [check_workflow_receipt()]
    else:
        gates = [
            check_main_tex(),
            check_chapters(),
            check_frontmatter(),
            check_project_tex_files(),
            check_claim_ledgers(),
            check_workflow_receipt(),
        ]

    result = ValidationResult(gates=gates)
    print_report(result, receipt_only=args.receipt_only)

    # Exit code: 0 = ALIVE, 1 = PARTIAL, 2 = BLOCKED
    exit_codes = {"ALIVE": 0, "PARTIAL": 1, "BLOCKED": 2}
    return exit_codes[result.verdict]


if __name__ == "__main__":
    sys.exit(main())
