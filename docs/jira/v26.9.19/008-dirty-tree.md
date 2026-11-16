# process-intelligence: commit or clean uncommitted changes

- Standing: OPEN
- Created: 2026-09-19 (v26.9.19 gh survey wave)
- Source: working tree dirty at survey time
- Evidence: `git status --porcelain` → 163 path(s) (tracked-modified: 163, untracked: 0); sample:  D construct8-market-physics/docs/agents/AGENT_01_CORPUS_AND_DOCTRINE.md; D construct8-market-physics/docs/agents/AGENT_02_CORE.md; D construct8-market-physics/docs/agents/AGENT_02_TYPE_LAW_FOUNDER.md

## Work to complete
- Review the 163 tracked-modified path(s); commit them as atomic pieces on a purpose branch, or revert what is transient.
- Note: this survey's ticket files under docs/jira/v26.9.19/ are intentionally uncommitted; include or exclude them deliberately in the commit plan.

## Acceptance
- `git status --porcelain` is clean (except items deliberately deferred and recorded here).

## History
- 2026-09-19 | OPEN | survey found dirty tree | 163 paths (T163/U0) | commit/clean pending
