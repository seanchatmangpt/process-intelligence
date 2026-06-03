# Agent 02: Doctrine and Naming Alignment Report

**Agent:** 2 — Doctrine and Naming Alignment
**Swarm:** coordinate-alpha
**Date:** 2026-06-01
**Mission:** Ensure all cross-project language preserves Sean Chatman doctrine.

---

## Scope

Searched the following paths for doctrine violations:
- `/Users/sac/process-intelligence` (1,905 MD files)
- `/Users/sac/knhk`
- `/Users/sac/ggen`
- `/Users/sac/truex`

Searched for the following forbidden substitution patterns:
- `knowledge hook` → middleware / callback / webhook / event listener / plugin / automation
- `CONSTRUCT8` → "just a query" / RDF insert batch / generic mutation
- `Autonomic Knowledge Actuation` → automation / AI workflow / lifecycle management
- `Blue River Dam` → payment system / trading system / ledger
- `ggen` → process miner / scaffolding tool
- `wasm4pm` → lite engine / mini PM4Py / stripped conformance checker
- `logic-chaos` → just business logic / conditional logic
- `Need9` → error / exception / failure / refusal
- `Receipt` → log / audit trail / record
- `Coordinate-System Alpha` → prediction edge / ML alpha / signal

---

## Findings

### Violations Found: 4 (all MINOR, all in process-intelligence)

#### VIOLATION-1
- **File:** `/Users/sac/process-intelligence/lifecycle/define_activation-state_process_intelligence.md:23`
- **Forbidden term:** `middleware`
- **Context:** "Transitions are bound to enterprise middleware endpoints"
- **Canonical replacement:** "enterprise message queue admission endpoints" or "enterprise event-stream admission boundaries"
- **Severity:** MINOR

#### VIOLATION-2
- **File:** `/Users/sac/process-intelligence/lifecycle/define_activation-state_process_intelligence.md:24`
- **Forbidden term:** `HTTP webhooks`
- **Context:** Example trigger binding enumeration includes "HTTP webhooks" alongside Kafka and RabbitMQ
- **Canonical replacement:** "HTTP admission endpoints"
- **Severity:** MINOR

#### VIOLATION-3
- **File:** `/Users/sac/process-intelligence/lifecycle/define_decommission-state_process_intelligence.md:7`
- **Forbidden term:** `event listeners`
- **Context:** "In the Execute phase, the system revokes execution authorizations and stops event listeners."
- **Canonical replacement:** "closes admission boundaries" or "deactivates knowledge hook enforcement points"
- **Severity:** MINOR

#### VIOLATION-4
- **File:** `/Users/sac/process-intelligence/prompts/execution-plans/lifecycle-state-authority.md:27`
- **Forbidden term:** `middleware guard`
- **Context:** "The execution engine employs the LSA verifier as an immutable middleware guard before granting Wasm linear memory access."
- **Canonical replacement:** "immutable admission gate" or "Knowledge Hook enforcement point"
- **Severity:** MINOR

---

## Clean Projects

| Project | Result |
|---------|--------|
| ggen | CLEAN — no forbidden substitutions detected |
| knhk | CLEAN — no forbidden substitutions detected |
| truex | CLEAN — no forbidden substitutions detected |

---

## Correct Canonical Usage Confirmed

The following uses of canonical terms were confirmed correct across the corpus:

- **Knowledge Hook:** 45+ correct uses across `phd-thesis/research/knowledge-hooks/`; frame preservation audit (both `audits/05_frame_preservation_audit.md` and `phd-thesis/research/knowledge-hooks/05_frame_preservation_audit.md`) explicitly names all forbidden substitutions and confirms correct usage at named anchors.
- **CONSTRUCT8:** 80+ correct uses across `phd-thesis/`, `knhk/`, `ggen/`; the 8-lane maximum is stated consistently.
- **Autonomic Knowledge Actuation:** 30+ correct uses; COVENANT.md and doctrine files explicitly state "NOT automation, NOT AI workflow, NOT lifecycle management."
- **Need9:** Used correctly in `phd-thesis/research/knowledge-hooks/04_construct8_motion_boundary_map.md` and `KNOWLEDGE_HOOKS_AND_AKA_DOCTRINE.md` — consistently defined as typed decomposition signal, not error.
- **Receipt:** Correctly distinguished from "log" in `KNOWLEDGE_HOOKS_AND_AKA_DOCTRINE.md:425` and `phd-thesis/research/knowledge-hooks/03_autoinstinct_lineage_map.md`.
- **ggen:** Used correctly as manufacturing engine; no instances of "process miner" or "scaffolding tool" applied to ggen.

---

## Previously Identified Violation (Carry-Forward)

The `audits/05_frame_preservation_audit.md` (line 183) and its phd-thesis counterpart both document a pre-existing MINOR violation:

- **Source:** A `.claude/settings.json` comment (Claude Code tool hooks, not Knowledge Hook doctrine)
- **Text:** "Hooks are Claude Code lifecycle automations that enforce refusal conditions."
- **Classification:** MINOR, mitigated by Claude Code context
- **Status:** Documented as BAD TRANSLATION 8 in frame preservation audit; no new remediation required.

This is not a new violation; it is carried forward for completeness.

---

## Artifact Written

`/Users/sac/process-intelligence/cross-project-coordinate-alpha/CONSTRUCT8_PROJECT_CONTRACTS.md`

Contains:
- Full canonical term table with forbidden substitutions
- Cross-project frame law (7 rules)
- All 4 violations with file:line, context, and remediation guidance
- Cross-project usage assessment table
- Enforcement rules for future cross-project work
- Gate verdict: PASS

---

## Summary

**4 violations found.** All are MINOR. All are in `process-intelligence` lifecycle and prompt files. The violations involve using "middleware," "webhooks," "event listeners," and "middleware guard" in contexts that describe integration boundaries — which should be described using Knowledge Hook admission boundary language. No critical violations were found. Projects ggen, knhk, and truex are clean.

The CONSTRUCT8_PROJECT_CONTRACTS.md has been written and supersedes the prior 2026-05-31 version.
