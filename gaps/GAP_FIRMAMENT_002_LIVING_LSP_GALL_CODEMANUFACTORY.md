---
gap: FIRMAMENT_002_LIVING_LSP_GALL_CODEMANUFACTORY
project: living-lsp-gall-codemanufactory
date: 2026-06-02
status: OPEN
severity: BLOCKING
gate: Inspection Gate
---

# Gap: living-lsp-gall-codemanufactory

## Summary

The firmament doctrine for the Living LSP / GALL / CodeManufactory project is
systematically misrouted: every boundary rule, receipt specification, and ALIVE
condition in the SPR ledger and gate assignment matrix points to `/Users/sac/ostar`,
which contains only OCEL/process-mining stubs. The actual implementation lives at
`/Users/sac/ggen`. This path mismatch renders every downstream receipt dependency
unresolvable and blocks the Inspection Gate entirely. Compounding the path error,
no wall-level ALIVE receipt exists under any path, the GALL-CONFORM-001 migration
to wasm4pm as the execution oracle is BLOCKED on three unresolved sub-dependencies,
modified source files in the ggen working tree are not committed, and the doctrine
link between ggen-lsp living-loop proofs and the RevOps test case claim is implicit
rather than receipted. Until the path is corrected and the receipt chain is grounded,
this project cannot be admitted to the Firmament wall.

## Gap Register

### GAP_LIVING_LSP_GALL_CODEMANUFACTORY_001 — Wrong canonical path in all firmament doctrine

- **Severity:** BLOCKING
- **Category:** WRONG_PATH
- **Specific Blocker:** `/Users/sac/process-intelligence/firmament/FIRMAMENT_PROJECT_SPR_LEDGER_002.md`
  line 382 requires `ostar/receipts/LIVING_LSP_ALIVE_001.yaml`. The directory
  `/Users/sac/ostar` contains only OCEL/process-mining stubs
  (`artifacts/`, `process_mining_bridge_stubs.beam`, `target/`). The Living LSP /
  GALL / CodeManufactory implementation is at `/Users/sac/ggen`
  (`crates/ggen-lsp`, `crates/ggen-lsp-a2a`, `crates/ggen-lsp-mcp`,
  `plugins/ggen-lsp`, `docs/gall/`). Every downstream receipt specification,
  boundary rule, and ALIVE condition in `FIRMAMENT_PROJECT_SPR_LEDGER_002.md`,
  `PROJECT_GATE_ASSIGNMENT_MATRIX_002.md`, `PUBLIC_PRIVATE_CONNECTION_MAP_002.md`,
  and `C4_SYSTEM_OF_SYSTEMS_002.md` references the wrong root.
- **Remediation:** Update all four firmament documents to replace the canonical path
  from `/Users/sac/ostar` to `/Users/sac/ggen` for Living LSP / GALL /
  CodeManufactory. All receipt dependency paths must be updated to
  `ggen/receipts/LIVING_LSP_ALIVE_001.yaml` (or the equivalent path under the ggen
  repo).
- **Effort:** < 1 hour

### GAP_LIVING_LSP_GALL_CODEMANUFACTORY_002 — No wall-level ALIVE receipt exists under any path

- **Severity:** BLOCKING
- **Category:** MISSING_RECEIPTS
- **Specific Blocker:** The required artifact
  `ggen/receipts/LIVING_LSP_ALIVE_001.yaml` (or equivalent) does not exist.
  The firmament SPR ledger explicitly states: "No qualifying receipt currently found.
  ALIVE requires a new `ostar/receipts/LIVING_LSP_ALIVE_001.yaml` or equivalent
  after LSP and GALL artifacts are confirmed present." The ggen-lsp crate has
  internal GALL-CHECKPOINT receipts (001B, 002) but none of these satisfy the
  Firmament Inspection Gate criteria, which require: LSP server operational + GALL
  violations detectable and receipted + CodeManufactory manufacturing pipeline in
  executable form + RevOps test case manufactured and receipted.
- **Remediation:** After resolving GAP_001 (correct path), issue a formal ALIVE
  verdict receipt at `/Users/sac/ggen/receipts/LIVING_LSP_ALIVE_001.yaml`
  referencing: (1) ggen-lsp test passage evidence, (2) GALL-CHECKPOINT-002 receipt
  as GALL conformance proof, (3) GALL-CONFORM-001 pre-inventory as the
  CodeManufactory migration plan, and (4) the living-loop proof tests
  (`ggen_tpl_001_living_loop.rs`, `ggen_harness_001_living_loop.rs`) as receipted
  RevOps-equivalent test case evidence.
- **Effort:** 1-4 hours

### GAP_LIVING_LSP_GALL_CODEMANUFACTORY_003 — GALL-CONFORM-001 migration pre-inventory only, no implementation receipt

- **Severity:** MAJOR
- **Category:** MISSING_DOCTRINE
- **Specific Blocker:**
  `/Users/sac/ggen/docs/receipts/GALL_CONFORM_001_PRE_INVENTORY.md` (dated
  2026-05-30) documents the CodeManufactory manufacturing pipeline migration
  (retiring ggen's internal PM algorithms in favor of wasm4pm as the execution
  oracle) but no `GALL_CONFORM_001_RECEIPT.md` exists. The migration is a 4-stage
  plan BLOCKED on three unresolved sub-dependencies: (1) `wpm` CLI oracle subcommand
  contract not verified to exist, (2) NDJSON truncated-line tolerance gap between
  `IntelLog::read` and `ocel-core::intake::NDJsonStream`, and (3) `ocel-core` git
  dependency not yet pinned in ggen.
- **Remediation:** Execute the 4-stage OCEL-retirement migration defined in
  `GALL_CONFORM_001_PRE_INVENTORY.md`: Stage 0 (pin `ocel-core` git dep), Stage 1
  (type swap), Stage 2 (swap reader), Stage 3 (externalize discovery/conformance to
  `wpm` CLI oracle, resolving the truncated-line gap first), Stage 4 (retire
  `ocel_types.rs`). Issue `GALL_CONFORM_001_RECEIPT.md` upon completion of all four
  stages passing `cargo make test` and `clippy -D warnings`.
- **Effort:** 3-5 days

### GAP_LIVING_LSP_GALL_CODEMANUFACTORY_004 — Uncommitted modified source files in ggen working tree

- **Severity:** MAJOR
- **Category:** MISSING_ARTIFACTS
- **Specific Blocker:** At `/Users/sac/ggen`, `git status` shows modified but
  unstaged files: `crates/ggen-cli/tests/sync_command_test.rs`,
  `crates/ggen-core/src/domain/generation/headers.rs`,
  `crates/ggen-core/src/genesis.rs`,
  `crates/ggen-core/src/validation/syntax_validator.rs`. The working tree does not
  match the last sealed receipt (GALL-CHECKPOINT-002 at commit `4aaa20a7` on main).
  Any ALIVE receipt issued against this working tree state is not reproducible.
- **Remediation:** Audit the four modified source files for intentional versus
  accidental changes. If changes are valid, commit them under conventional commit
  format. If stale diffs, restore them. Deleted target cache files
  (`cpmp/target_implementer_m6`, `target_check`, `target_check_new`,
  `target_fresh`) are build artifacts and may be omitted from staging.
- **Effort:** < 1 hour

### GAP_LIVING_LSP_GALL_CODEMANUFACTORY_005 — Living-loop proofs not explicitly mapped to RevOps test case ALIVE criterion

- **Severity:** MINOR
- **Category:** MISSING_DOCTRINE
- **Specific Blocker:** No document at `/Users/sac/ggen` or
  `/Users/sac/process-intelligence` explicitly maps
  `crates/ggen-lsp/tests/ggen_tpl_001_living_loop.rs` and
  `crates/ggen-lsp/tests/ggen_harness_001_living_loop.rs` to the "RevOps test case
  manufactured and receipted" ALIVE criterion from
  `FIRMAMENT_PROJECT_SPR_LEDGER_002.md`. The doctrine link is implicit and not
  receipted.
- **Remediation:** Create a crosswalk document at
  `/Users/sac/process-intelligence/crosswalks/LIVING_LSP_REVOPS_TESTCASE_CROSSWALK.md`
  that explicitly maps each living-loop test to the RevOps CodeManufactory test case
  claim. Reference GALL-CHECKPOINT-001B and GALL-CHECKPOINT-002 coordination
  receipts as the evidence trail. No new code required.
- **Effort:** < 1 hour

## ALIVE Conditions Blocked

The following ALIVE conditions from `FIRMAMENT_PROJECT_SPR_LEDGER_002.md` cannot
be met until these gaps are closed:

1. **LSP server operational** — Unblocked at the code level, but cannot be receipted
   until GAP_001 (path correction) and GAP_004 (clean working tree) are resolved.
2. **GALL violations detectable and receipted** — GALL-CHECKPOINT-002 exists but
   is not admissible at the wall until GAP_001 corrects the canonical path.
3. **CodeManufactory manufacturing pipeline in executable form** — Blocked by
   GAP_003 (GALL-CONFORM-001 migration incomplete, three sub-dependencies
   unresolved).
4. **RevOps test case manufactured and receipted** — Blocked by GAP_002 (no
   wall-level receipt) and GAP_005 (crosswalk from living-loop proofs to this
   criterion is implicit, not documented).

## Resolution Path

Ordered steps to bring this project from its current state to ALIVE:

1. **Correct the canonical path** (GAP_001): Update `FIRMAMENT_PROJECT_SPR_LEDGER_002.md`,
   `PROJECT_GATE_ASSIGNMENT_MATRIX_002.md`, `PUBLIC_PRIVATE_CONNECTION_MAP_002.md`,
   and `C4_SYSTEM_OF_SYSTEMS_002.md` to replace `/Users/sac/ostar` with
   `/Users/sac/ggen` for all Living LSP / GALL / CodeManufactory references.
   Effort: < 1 hour.

2. **Clean the ggen working tree** (GAP_004): Audit the four modified source files,
   commit or restore as appropriate. The working tree must match a sealed receipt
   before any ALIVE verdict is issued. Effort: < 1 hour.

3. **Create the RevOps living-loop crosswalk** (GAP_005): Write
   `crosswalks/LIVING_LSP_REVOPS_TESTCASE_CROSSWALK.md` mapping living-loop tests
   to the RevOps test case ALIVE criterion. Effort: < 1 hour.

4. **Resolve GALL-CONFORM-001 sub-dependencies** (GAP_003, prerequisite):
   - Pin `ocel-core` git dependency in ggen (Stage 0).
   - Confirm or define the `wpm` CLI oracle subcommand contract.
   - Resolve NDJSON truncated-line tolerance gap between `IntelLog::read` and
     `ocel-core::intake::NDJsonStream`.
   Effort: 1-2 days.

5. **Execute GALL-CONFORM-001 migration** (GAP_003): Complete Stages 1-4 of the
   migration defined in `GALL_CONFORM_001_PRE_INVENTORY.md`. Issue
   `GALL_CONFORM_001_RECEIPT.md` upon all stages passing `cargo make test` and
   `clippy -D warnings`. Effort: 2-3 days.

6. **Issue wall-level ALIVE receipt** (GAP_002): Write
   `ggen/receipts/LIVING_LSP_ALIVE_001.yaml` referencing ggen-lsp test evidence,
   GALL-CHECKPOINT-002, GALL-CONFORM-001 receipt, and the living-loop crosswalk.
   Submit for Firmament Inspection Gate admission. Effort: 1-4 hours.

## Doctrine Note

Evidence must precede authorization: a receipt chain that references a non-existent
path is not a receipt — it is an unverified claim, and unverified claims are
PARTIAL findings that belong in `gaps/`, not in firmament doctrine.
