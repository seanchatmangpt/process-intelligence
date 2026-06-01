# PROCESS_INTELLIGENCE_ALIVE_001 — Gate Criteria

## Verdict Definition

ALIVE_001 certifies that the process-intelligence research foundry has reached minimum
viable doctrine density for authorizing downstream implementations.

PARTIAL is not failure — PARTIAL is the bill of materials for the next transition.

---

## Gate Criteria

All thresholds must be met simultaneously. A single unmet threshold renders the verdict PARTIAL.

| Criterion | Directory | Minimum Count | Verification Command |
|---|---|---:|---|
| Doctrine density | `doctrine/` | 15 | `ls doctrine/ \| wc -l` |
| Standards coverage | `standards/` | 10 | `ls standards/ \| wc -l` |
| Paper classifications | `sources/papers/` | 8 | `ls sources/papers/ \| wc -l` |
| PM4Py capability maps | `sources/pm4py/` | 5 | `ls sources/pm4py/ \| wc -l` |
| wasm4pm authority maps | `sources/wasm4pm/` | 3 | `ls sources/wasm4pm/ \| wc -l` |
| compat type-law maps | `sources/wasm4pm-compat/` | 3 | `ls sources/wasm4pm-compat/ \| wc -l` |
| Lifecycle states | `lifecycle/` | 8 | `ls lifecycle/ \| wc -l` |
| Comparison matrices | `comparisons/` | 5 | `ls comparisons/ \| wc -l` |
| Type-law crosswalks | `crosswalks/` | 4 | `ls crosswalks/ \| wc -l` |
| M&A claim taxonomy | `ma/` | 6 | `ls ma/ \| wc -l` |
| Adversarial cases | `adversarial/` | 3 | `ls adversarial/ \| wc -l` |
| Documented gaps | `gaps/` | 2 | `ls gaps/ \| wc -l` |
| Total commits | `.git/` | 80 | `git log --oneline \| wc -l` |

---

## Verification Script

Run this script from the repository root to check all criteria:

```bash
#!/usr/bin/env bash
set -e

echo "=== PROCESS_INTELLIGENCE_ALIVE_001 Gate Check ==="
echo ""

PASS=0
FAIL=0

check() {
  local label="$1"
  local dir="$2"
  local min="$3"
  local count
  count=$(ls "$dir" 2>/dev/null | wc -l | tr -d ' ')
  if [ "$count" -ge "$min" ]; then
    echo "  PASS  $label: $count >= $min ($dir)"
    PASS=$((PASS + 1))
  else
    echo "  FAIL  $label: $count < $min ($dir)"
    FAIL=$((FAIL + 1))
  fi
}

check "doctrine"              doctrine/                 15
check "standards"             standards/                10
check "papers"                sources/papers/            8
check "pm4py"                 sources/pm4py/             5
check "wasm4pm"               sources/wasm4pm/           3
check "wasm4pm-compat"        sources/wasm4pm-compat/    3
check "lifecycle"             lifecycle/                 8
check "comparisons"           comparisons/               5
check "crosswalks"            crosswalks/                4
check "ma"                    ma/                        6
check "adversarial"           adversarial/               3
check "gaps"                  gaps/                      2

COMMITS=$(git log --oneline | wc -l | tr -d ' ')
MIN_COMMITS=80
if [ "$COMMITS" -ge "$MIN_COMMITS" ]; then
  echo "  PASS  commits: $COMMITS >= $MIN_COMMITS"
  PASS=$((PASS + 1))
else
  echo "  FAIL  commits: $COMMITS < $MIN_COMMITS"
  FAIL=$((FAIL + 1))
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"
echo ""

if [ "$FAIL" -eq 0 ]; then
  echo "Verdict: ALIVE_001 — All criteria met"
  echo "Authorization: Downstream implementations may proceed"
else
  echo "Verdict: PARTIAL_001 — $FAIL criteria not met"
  echo "Bill of materials for next transition: see FAIL lines above"
fi
```

---

## PARTIAL Doctrine

> PARTIAL is not failure — PARTIAL is the bill of materials for the next transition.

A PARTIAL verdict:
1. Documents exactly which criteria are unmet
2. Lists the specific artifacts needed to meet each unmet criterion
3. Authorizes continuation of research work toward those artifacts
4. Does NOT authorize downstream implementation work

A PARTIAL verdict is preferable to a false ALIVE verdict.
False ALIVE is breach. PARTIAL is honorable.

---

## Additional ALIVE Requirements

Beyond the count thresholds, an ALIVE_001 verdict requires:

1. **No open CRITICAL gaps** — All gaps with severity CRITICAL must have a documented
   remediation path (not necessarily executed, but planned)

2. **Doctrine cross-referencing** — All doctrine files must cite at least one source
   (paper, experiment, or prior checkpoint)

3. **Receipt chain integrity** — The commit log must have no gaps (no orphaned working
   tree commits, no force-pushed history)

4. **Downstream prompt completeness** — `prompts/` must contain prompts for all
   authorized downstream implementations

5. **Checkpoint file present** — `checkpoints/PROCESS_INTELLIGENCE_ALIVE_001.md` must
   exist with the ALIVE verdict signed by commit hash

---

## Current Status

Run the verification script above to get current status.

As of bootstrap: criteria checks in progress. GAP_001 (CRITICAL) documented.
Verdict: PARTIAL — research foundry bootstrapped, criteria population in progress.
