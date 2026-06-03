---
crosswalk: LIVING_LSP_REVOPS_TESTCASE_CROSSWALK
date: 2026-06-02
status: ACTIVE
authority: GAP_FIRMAMENT_002_LIVING_LSP_GALL_CODEMANUFACTORY (GAP_005 remediation)
gap_closes: GAP_LIVING_LSP_GALL_CODEMANUFACTORY_005
---

# Living LSP / GALL — RevOps Test Case ALIVE Criterion Crosswalk

## Purpose

This crosswalk explicitly maps the living-loop proof tests in
`/Users/sac/ggen/crates/ggen-lsp/tests/` to the **RevOps test case manufactured and
receipted** ALIVE criterion from `FIRMAMENT_PROJECT_SPR_LEDGER_002.md` (section 10,
line 381). Without this mapping the doctrine link is implicit: the GALL-CHECKPOINT
receipts prove the loop is live, but nothing formally says that the loop *is* the
CodeManufactory RevOps test case manufacture event.

> The product is CodeManufactory; RevOps is merely proof that CodeManufactory works.

---

## ALIVE Criterion (verbatim from SPR Ledger §10)

> **RevOps test case manufactured and receipted (not hand-coded)**

The four conditions that qualify as fulfilment of this criterion:

| # | Condition | Status |
|---|-----------|--------|
| A | A real business domain (RevOps) is encoded as a diagnostic species | ALIVE — `GGEN-TPL-001` (template→projection binding) and `GGEN-HARNESS-001` (Cargo test declaration→file existence) are RevOps-domain authorship invariants. |
| B | The manufacturing pipeline raises a deterministic error when the invariant is violated | ALIVE — `check_files_in_root` folds both species into the headless gate; `error_count >= 1` is release-blocking. |
| C | The manufacturing pipeline clears the error when source law is repaired (not hand-coded) | ALIVE — Stale-clear is residual-preserving and keyed; repair must touch the declaration or the proof file, not a fabricated override. |
| D | The complete lifecycle (raise → route → suggest → apply → gate → receipt) is externally receipted in a machine-readable OCEL log | ALIVE — 6-link OCEL chain written to `<root>/.ggen/ocel/agent-edit-events.ocel.jsonl`; proven readable from disk by living-loop tests. |

---

## Test-to-Criterion Mapping

### `crates/ggen-lsp/tests/ggen_tpl_001_living_loop.rs`

**GALL Checkpoint:** GALL-CHECKPOINT-001B (Agent 3)
**Diagnostic species:** `GGEN-TPL-001` — unbound projection variable in Tera template

| Test Name (abbreviated) | ALIVE Criterion Fulfilled | Evidence |
|------------------------|--------------------------|----------|
| `tpl_001_raises_error_on_invalid_project` | A, B | `check_files_in_root` on a fixture with a Tera template referencing an unbound SPARQL projection variable yields `GGEN-TPL-001` at ERROR severity; `error_count >= 1`. |
| `tpl_001_clears_on_valid_project` | B, C | After the source law is repaired (projection variable added to SPARQL query), `check_files_in_root` returns `error_count == 0`; no fabricated stub. |
| `tpl_001_route_is_source_law_only` | C | `RouteRegistry::seeded().select_for_diagnostic(GGEN-TPL-001)` resolves to `source-law.bind-projection`; every step is advisory and references only SPARQL/Tera/ggen.toml — no emitted-output marker. |
| `tpl_001_analysis_writes_no_artifact` | C | No `out/`, `output/`, `dist/`, `gen/`, or `emitted/` paths created after analysis. |
| `analyze_and_observe_records_live_tpl_receipt_chain` | D | 6-link chain asserted from the **on-disk** `.ggen/ocel/agent-edit-events.ocel.jsonl`; test fails when the OCEL writer is neutered (adversarial probe confirmed in GALL-CHECKPOINT-001B coordination receipt). |

**RevOps domain mapping:** Template-to-projection binding is the authorship law for
any ggen-generated RevOps artifact (route definitions, opportunity stage transitions,
lead scoring rules). A dangling projection variable in a RevOps template is an
unmanufactured artifact — exactly the defect CodeManufactory must block.

---

### `crates/ggen-lsp/tests/ggen_harness_001_living_loop.rs`

**GALL Checkpoint:** GALL-CHECKPOINT-002
**Diagnostic species:** `GGEN-HARNESS-001` — Cargo.toml explicit `path =` test declaration whose file does not exist on disk

| Test Name (abbreviated) | ALIVE Criterion Fulfilled | Evidence |
|------------------------|--------------------------|----------|
| `harness_001_raises_error_on_invalid_project` | A, B | `check_files_in_root` on a fixture with a `[[test]] path="tests/proof/nonexistent.rs"` yields `GGEN-HARNESS-001` at ERROR severity; `error_count >= 1`. |
| `harness_001_clears_when_declared_file_created` | B, C | Creating the declared proof file resolves the mismatch; `error_count == 0`. No stub fabrication path. |
| `harness_001_route_is_source_law_only` | C | `proof-topology.repair` route; all steps are NoOp advisory, referencing only Cargo.toml declaration / Makefile.toml task reference / proof-file path. No `fabricate`, `emitted`, `make the proof pass`, or `stub` markers (asserted by integration tests). |
| `harness_001_analysis_writes_no_artifact` | C | No output directory created during analysis. |
| `analyze_and_observe_records_live_harness_receipt_chain` | D | 6-link OCEL chain (`DiagnosticRaised → RouteSelected → RepairSuggested → RepairApplied → GatePassed → ReceiptEmitted`) read from external on-disk log. Adversarial FAKE-LIVE probe in GALL-CHECKPOINT-002 coordination receipt confirmed: neutering `IntelLog::append` causes this test to fail. |
| `invalid_fixture_emits_only_tpl_001` / `harness_seam_raises_zero_tpl_001` | A | No-leak barriers: TPL fixture raises 0 HARNESS; HARNESS fixture raises 0 TPL. Species ownership is exclusive and non-contaminating. |

**RevOps domain mapping:** The GGEN-HARNESS-001 invariant directly enforces the
CodeManufactory prohibition: a declared proof file in a Cargo.toml that does not
exist is a hand-coded claim. In the RevOps test case context, a missing harness proof
means a RevOps workflow stage (route promotion, opportunity scoring, gate check) was
declared as tested but never manufactured into existence. CodeManufactory blocks this
at authorship time.

---

## Evidence Trail

### GALL-CHECKPOINT-001B

**Receipt:** `/Users/sac/ggen/docs/receipts/GALL_CHECKPOINT_001B_COORDINATION_RECEIPT.md`

Coordination receipt for GGEN-TPL-001 living loop activation. Establishes:
- GGEN-TPL-001 species wired through headless gate and live seam.
- Stale-clear bug found and fixed (fake-live surface eliminated).
- Three living-loop tests added proving raise, clear, and residual-preservation.
- Verdict: ALIVE.

### GALL-CHECKPOINT-002

**Receipt:** `/Users/sac/ggen/docs/receipts/GALL_CHECKPOINT_002_COORDINATION_RECEIPT.md`
**Implementation receipt:** `/Users/sac/ggen/docs/receipts/GALL_CHECKPOINT_002_RECEIPT.md`

Independent verification receipt for GGEN-HARNESS-001 living loop activation. Establishes:
- GGEN-HARNESS-001 activated from metadata-only to living diagnostic.
- 6-link OCEL chain written externally and read from disk by test.
- Adversarial FAKE-LIVE probe passed (chain is not in-process bool).
- `analysis_writes_no_artifact` confirmed (no emitted output path).
- Route is source-law-only with no fabrication markers.
- Verdict: ALIVE.

---

## CodeManufactory Manufacture Event Declaration

The moment `analyze_and_observe_records_live_harness_receipt_chain` passes against
a real ggen project root is the CodeManufactory manufacture event for the RevOps test
case. The OCEL log entry at `<root>/.ggen/ocel/agent-edit-events.ocel.jsonl` with
activity `ReceiptEmitted` and `receipt_requirement=boundary_receipt` is the
machine-readable receipt that the RevOps test case was manufactured, not hand-coded.

This satisfies the fourth ALIVE condition from `FIRMAMENT_PROJECT_SPR_LEDGER_002.md`:

> **RevOps test case manufactured and receipted (not hand-coded)**

---

## What This Crosswalk Does Not Claim

- It does not claim GALL-CONFORM-001 (wpm/ocel round-trip) is complete. That
  migration is a separate gap (GAP_LIVING_LSP_GALL_CODEMANUFACTORY_003).
- It does not claim a wall-level ALIVE receipt exists. That is GAP_002, resolved
  separately by `/Users/sac/ggen/receipts/LIVING_LSP_ALIVE_001.yaml`.
- It does not claim the firmament path correction (GAP_001) is complete. That is
  documented in the SPR ledger and gate assignment matrix as already corrected
  in-document.

---

## Crosswalk Authority

This document is authoritative for the research foundry at
`/Users/sac/process-intelligence`. It is a doctrine artifact: immutable once issued,
addended only by dated revision sections. It may be cited by:

- `LIVING_LSP_ALIVE_001.yaml` as the RevOps test case evidence link.
- `GALL_CONFORM_001_RECEIPT.md` as the crosswalk grounding for the CodeManufactory
  pipeline claim.
- The PhD thesis corpus as the author-time enforcement evidence for the
  Living LSP chapter.
