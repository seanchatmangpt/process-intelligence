# process-intelligence: land or delete remote branch `fanout/r80-forced-top25`

- Standing: OPEN
- Created: 2026-09-19 (v26.9.19 gh survey wave)
- Source: remote branch `fanout/r80-forced-top25` — not merged into `main`, no open PR
- Evidence: `git branch -r --no-merged origin/main` lists it; absent from `gh pr list` heads

## Work to complete
- Decide: open a PR (`gh pr create -R seanchatmangpt/process-intelligence --head fanout/r80-forced-top25`) or delete (`git push origin --delete fanout/r80-forced-top25`).
- If superseded, delete; otherwise land through review.

## Acceptance
- After `git fetch --prune`, `git branch -r --no-merged origin/main` no longer lists `fanout/r80-forced-top25`.

## History
- 2026-09-19 | OPEN | survey found PR-less unmerged branch | fanout/r80-forced-top25 | decision pending
