# Claude Code Configuration - Process Intelligence Research Foundry

## Mission

~/process-intelligence is the research foundry for full-lifecycle process intelligence.
~/process-intelligence is the research program; wasm4pm is one execution product of that research.

This repository is the authority layer for studying, validating, and manufacturing the complete
doctrine stack required for full-lifecycle process intelligence.

---

## Critical Rules

### Never Commit Unsupported Claims

- Every claim in doctrine/ files must cite a source: paper, experiment, or prior checkpoint
- Claims derived from assumption or inference alone are PARTIAL findings, not doctrine
- If evidence is absent, the finding is PARTIAL — document it in gaps/ not doctrine/
- ALIVE verdicts require all gate criteria to be met — never declare ALIVE on partial evidence

### Every Paper Classification Needs Evidence

- Paper-to-type-law mappings require reading the paper and confirming the mapping
- Paper-to-execution-law mappings require confirming the execution capability exists
- Paper citations in M&A claims require specific section/theorem/definition references
- Never cite a paper without confirming the cited claim appears in it

### Evidence Before Authorization

- No downstream wasm4pm refactor until this research program speaks
- No M&A claim manufactured without research program grounding
- No gap closed without a research finding documenting the gap first

---

## Commit Message Format

All commits must follow conventional commit format with research-specific scopes:

```
type(scope): description
```

### Commit Types

| Type | When to Use |
|---|---|
| `research-init` | Repository bootstrap, initial structure, configuration |
| `research-paper` | Paper analysis, paper-to-type-law mapping, paper classification |
| `research-pm4py` | PM4Py capability analysis, oracle benchmarking, comparative findings |
| `research-compat` | wasm4pm-compat type law analysis, witness lattice findings, compat gaps |
| `research-wasm4pm` | wasm4pm execution authority analysis, algorithm findings, gaps |
| `standards` | Public standards mapping (XES, OCEL, BPMN, ISO, SOC2, GDPR) |
| `doctrine` | Immutable process law definitions, foundational principles |
| `lifecycle` | Process lifecycle state definitions, transition rules |
| `m-and-a` | M&A claim taxonomy, board admissibility, synergy/debt projections |
| `experiment` | Benchmark fixtures, comparison experiments, capability tests |
| `audit` | Completeness audits, source authority audits, gap analyses |
| `prompt` | Downstream implementation prompts |
| `docs-law` | Repository documentation (CLAUDE.md, README.md, COVENANT.md) |
| `checkpoint` | Phase milestone verdicts (ALIVE, PARTIAL, FAILED) |
| `gap` | Gap documentation — structural defects requiring remediation |

### Examples

```
research-paper: classify Van der Aalst OCEL 2.0 spec → type-law surface
research-pm4py: document conformance checking capability atlas
research-wasm4pm: confirm algorithm function signatures accept raw EventLog
doctrine: define full-lifecycle process intelligence maturity levels
gap: document GAP_001 compat-to-wasm bridge missing
checkpoint: PROCESS_INTELLIGENCE_ALIVE_001
```

---

## Directory Conventions

| Directory | Purpose | Commit Type |
|---|---|---|
| `doctrine/` | Immutable process law — never rebase, only addend | `doctrine` |
| `standards/` | Public standards compliance maps | `standards` |
| `sources/papers/` | Paper analysis and classification | `research-paper` |
| `sources/pm4py/` | PM4Py capability atlas | `research-pm4py` |
| `sources/wasm4pm/` | wasm4pm execution authority map | `research-wasm4pm` |
| `sources/wasm4pm-compat/` | Type law atlas and witness lattices | `research-compat` |
| `lifecycle/` | Process lifecycle state definitions | `lifecycle` |
| `comparisons/` | Cross-system capability comparisons | `experiment` |
| `crosswalks/` | Type-law crosswalk mappings | `research-compat` |
| `ma/` | M&A claim taxonomy and projections | `m-and-a` |
| `gaps/` | Structural gap documentation | `gap` |
| `audits/` | Completeness audits | `audit` |
| `prompts/` | Downstream implementation prompts | `prompt` |
| `receipts/` | Cryptographic receipt chain | `checkpoint` |
| `checkpoints/` | Phase milestone verdicts | `checkpoint` |
| `adversarial/` | Adversarial test cases and hostile findings | `experiment` |
| `experiments/` | Benchmark fixtures and comparative tests | `experiment` |

---

## Immutability Doctrine

- Never rebase `doctrine/` files — only add dated addendums
- Never revert audits — only add corrective follow-up audits
- Never delete gap documents — only add resolution addendums
- Checkpoint files are permanent — ALIVE/PARTIAL/FAILED verdicts stand as issued

---

## See Also

- `COVENANT.md` — What may and may not be claimed
- `~/.claude/rules/process-mining-chicago-tdd.md` — Van der Aalst Constitution
- `~/.claude/CLAUDE.md` — Global tool preferences and git workflow rules
- `gaps/` — Open structural gaps requiring research attention
