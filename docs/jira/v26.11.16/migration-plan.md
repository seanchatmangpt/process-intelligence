# process-intelligence migration plan v26.11.16

Standing: UNVERIFIED

## Identity
- Path: /Users/sac/process-intelligence
- Remote: https://github.com/seanchatmangpt/process-intelligence.git
- Branch@sha: main@f816a37 (origin/main, ahead 0/behind 0), last 2026-09-29, 998 commits
- Size: 47 MB
- Languages/build: doctrine/markdown research program; ggen.toml, ggen-unified.toml, .tera
  templates; sub-projects construct8-market-physics, blue_river_dam, dfcm, firmament
- Nested repos: none

## Observed state
- Dirty 165: `M README.md` + mass `D construct8-market-physics/docs/agents/AGENT_*.md`
  (agent-doc deletions). Untracked 9.
- Stash 903 (census) — extreme; stash list could not be read (hook-blocked).
- 17 unpushed local branches incl. pcp-physical-extraction-001,
  security/rotate-signing-keys-v26.9.24, many subagent-*-zoeapp-framework-agi branches;
  phd-thesis-corpus-manufacture-001 is pushed but behind 19.
- No build dirs.

## What it is
Research foundry / authority layer for full-lifecycle process intelligence doctrine; wasm4pm is
one downstream product. Evidence-first, receipt-bearing.

## Classification
ACTIVE (census).

## Migration risk
HIGH. 903 stashes and 17 unpushed branches (including a signing-key rotation branch) exist only
here; 165 dirty paths. Branch name `security/rotate-signing-keys` implies key material may be
under version control — inspect before pushing.

## Recommended action
COMMIT-PUSH
1. `cd /Users/sac/process-intelligence && git branch -vv | grep -v origin` (list the 17)
2. DECISION: audit `security/rotate-signing-keys-v26.9.24` for committed private keys before
   any push; purge with git-filter-repo if present.
3. `git add -A && git commit -m "chore: prune construct8 agent docs; README"`
4. `git push origin main && git push origin --all`
5. DECISION: 903 stashes — triage top 20 with `git stash list | head -20`; the stash ref chain
   does not survive clone/bundle, so the only lossless carrier is the whole `.git` dir:
   `rsync -a /Users/sac/process-intelligence/ <archive>/process-intelligence/`.
6. The rsync in step 5 is the safety net; clone fresh for daily work on the new machine.

## First commands on the new machine
`ls doctrine gaps checkpoints && ggen --version` (no build system; doc-check only)

## See Also
- ../v26.9.19/
- /Users/sac/docs/jira/v26.11.16/README.md
