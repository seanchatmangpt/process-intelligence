# ALL_PROJECTS_STATUS_001 — Cross-Repository Fix and Push Sweep

**Date:** 2026-06-02
**Issued by:** Process Intelligence Research Foundry
**Receipt type:** Cross-project status checkpoint
**Scope:** All active PhD defense repositories

---

## Status Table

| Repo | Branch | Build | Tests | Pushed | Remaining Issues |
|------|--------|-------|-------|--------|-----------------|
| process-intelligence | phd-thesis-corpus-manufacture-001 | PASS | PASS | already-current | Untracked `.claude/scheduled_tasks.lock` (ephemeral runtime lock, not committed by policy) |
| truex | (main) | PASS | FAIL | already-current | Test `valid_fixture_batch_hash_matches_envelope` fails — hash mismatch is a pre-existing defect; 5028 deleted build artifacts tracked in git index but excluded by `.gitignore` (need `git rm --cached`); 2 modified tracked build files; untracked `crates/.truex/` directory |
| knhk | (main) | PASS | PASS | already-current | 32 untracked `.agents/` subdirs from gen3–gen5 agent runs; 3 dirty submodule pointers (spec-kit, vendors/clap-noun-verb, vendors/weaver); 3 untracked planning/notes files at root; 5 accumulated stash entries; 2 future-hard-error warnings for `default-features` in blake3 dependencies |
| wasm4pm | fix/debt-markers-and-gap-close | PASS | PASS | already-current | `WASM4PM_CAPABILITY_INVENTORY.md` untracked; `.npmrc` contains deprecated/unknown config keys; 5 stash entries on various branches |
| chatmangpt | (main) | PASS | PASS | already-current | 8 submodules with drifted HEAD SHAs; 7 submodules uninitialized or detached; 40+ untracked XES/CSV fixture files inside wasm4pm/ submodule; 1 untracked root CSV; 1 untracked build log; 5 stash entries with potential abandoned WIP |
| dteam | (main) | FAIL | FAIL | already-current | BUILD FAILURE: `unibit-kernel` requires `#![feature(generic_const_exprs)]` (nightly) but workspace defaults to stable; no `rust-toolchain.toml`; 35 dirty files including 18+ accumulated Python doc-generation scripts in `crates/ccog/`; `dev-worktree` submodule dirty |
| pcp | (main) | PASS | PASS | already-current | 43 dirty files — source edits across process-mining, UI, crypto, store, actor, vkg, and truex subsystems not yet committed; untracked `packages/` directory; modifications consistent with prior 2030 core best-practices enforcement work |
| speckit-ralph | 065-semanticos-full-stack | PASS | PASS | already-current | 1,176 deleted build artifacts tracked in git (target/ committed historically — needs `git rm --cached`); 15 modified tracked agent/portfolio/script files; 107 untracked agent subdirs from in-progress ralph-loop/swarm run |

---

## Actions Taken Per Repo

### process-intelligence
- Scout confirmed build and tests pass.
- Working tree has one untracked `.claude/scheduled_tasks.lock` — ephemeral runtime artifact, deliberately not committed.
- No fix required. Branch is current with remote.

### truex
- Scout identified pre-existing hash mismatch defect in `valid_fixture_batch_hash_matches_envelope`.
- 5028 deleted files in working tree are build artifacts that were previously committed before `.gitignore` was corrected — root cause identified.
- No commits were ahead of remote; no push attempted.
- Remediation deferred: defect is pre-existing, not introduced by this sweep. Requires dedicated `git rm --cached` pass and separate hash investigation.

### knhk
- Build and tests pass.
- Dirt is confined to agent orchestration artifacts and submodule pointer drift — no source regression.
- 5 stash entries flagged for manual review.
- No commits were ahead of remote; no push attempted.
- `cargo check` future-hard-error warnings for blake3 `default-features` documented for next maintenance window.

### wasm4pm
- Build and tests pass.
- `WASM4PM_CAPABILITY_INVENTORY.md` remains untracked — deferred to wasm4pm research team.
- `.npmrc` deprecation warnings documented.
- Branch `fix/debt-markers-and-gap-close` is current with remote.

### chatmangpt
- Build and tests pass.
- Submodule drift is structural — chatmangpt is an aggregation repo and submodules evolve independently.
- 40+ fixture files inside the wasm4pm submodule are invisible to wasm4pm's own git (detached submodule).
- No commits were ahead of remote; no push attempted.

### dteam
- BUILD FAILURE confirmed: nightly-only feature `generic_const_exprs` used by `unibit-kernel` cannot be compiled on stable toolchain.
- Fix path: add `rust-toolchain.toml` pinning nightly, or remove the `unibit-kernel` dependency from the workspace.
- No tests ran — build failure prevented execution.
- 18+ accumulated Python doc-generation scripts in `crates/ccog/` are untracked and represent WIP that was never committed.
- Remediation requires deliberate engineering decision on toolchain strategy.

### pcp
- Build and tests pass.
- 43 dirty files represent a substantial batch of uncommitted source edits — this is active WIP, not artifacts.
- Changes span process-mining, UI, crypto, store, actor, vkg, and truex subsystems.
- Commit and push deferred pending explicit authorship decision by Sean.

### speckit-ralph
- Build and tests pass on branch `065-semanticos-full-stack`.
- 1,176 deleted target/ files are the same class of defect as truex — build artifacts committed historically, now excluded by `.gitignore`, require `git rm --cached` cleanup.
- 107 untracked agent subdirs reflect an in-progress ralph-loop/swarm run — not committed by policy.
- Branch is current with remote.

---

## Ecosystem Verdict

**PARTIAL**

### Breakdown

| Category | Status |
|----------|--------|
| Build passing | 7/8 repos (dteam FAIL) |
| Tests passing | 6/8 repos (truex hash mismatch defect, dteam build failure) |
| Clean working trees | 0/8 repos (all have dirt) |
| Pushes executed | 0/8 (all repos already current with remote or had no commits to push) |

### Blocking Issues

1. **dteam**: Nightly-only Rust feature in a stable workspace — repository cannot build. Requires `rust-toolchain.toml` or dependency removal before any defense artifact work.
2. **truex**: Pre-existing hash mismatch in `valid_fixture_batch_hash_matches_envelope` — canonicalize test suite is non-green. Requires root cause investigation into fixture vs. algorithm drift.

### Non-Blocking Technical Debt

- **truex + speckit-ralph**: Build artifacts historically committed to git; `.gitignore` now excludes them but index not cleaned. `git rm --cached` required.
- **pcp**: 43 uncommitted source edits across 6+ subsystems — significant WIP batch awaiting commit.
- **knhk + chatmangpt + wasm4pm**: Accumulated stash entries and agent orchestration artifacts — hygiene debt, not functional regressions.

---

## Receipt Integrity

This receipt reflects the output of the following scout IDs:

- `process-intelligence-scout-001`
- `truex-scout-2026-06-02`
- `knhk-scout-2026-06-02`
- `wasm4pm-scout`
- `chatmangpt-scout`
- `dteam-scout-2026-06-02`
- `pcp-scout`
- `speckit-ralph-scout`

No fix results were applied in this sweep. All repos were at remote parity before this receipt was issued.

---

*This receipt is permanent. Per immutability doctrine, it must not be deleted or rebased. Corrective findings must be issued as ALL_PROJECTS_STATUS_002 or later.*
