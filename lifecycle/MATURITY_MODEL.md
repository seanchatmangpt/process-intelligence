# MATURITY_MODEL — Process Intelligence Maturity Model (5 Levels)

## Overview

Process intelligence maturity measures the progressive removal of unresolved process uncertainty
from execution. At L1, nothing is known. At L5, the system is autonomic: it monitors itself,
analyzes deviations, plans repairs, and executes them — with every step receipted.

This model applies to a specific process (not an organization overall). An organization may have
processes at different maturity levels simultaneously.

---

## L1 — Ad-hoc

**Identity:** No formal models, no event logs.

**Characteristics:**
- Process behavior is described in prose, slide decks, or tribal knowledge.
- No event log exists. There is no systematic record of what happened.
- Conformance cannot be checked because there is no model to check against.
- Process improvement is based on anecdote and subjective observation.
- Audit responses are narrative: "we believe the process works as follows."

**wasm4pm-compat type law:** None applicable. No typed process structures exist.

**wasm4pm execution required:** None. There is nothing to execute against.

**Risk:** Governance claims are unverifiable. M&A due diligence cannot replay the process.
Any claim about process performance is narration.

---

## L2 — Documented

**Identity:** Formal models exist, but no event log connection.

**Characteristics:**
- A process model exists in a standard notation: WF-net, BPMN, POWL, Declare.
- The model has been verified for soundness (no dead transitions, proper completion).
- No operational event log is connected to the model. The model is designed, not observed.
- Conformance cannot be checked because no log exists.
- The model may diverge from actual operational behavior without detection.

**wasm4pm-compat type law:** Fully applicable.
- `WfNetConst<SOUNDNESS>` encodes soundness class at the type level.
- `TreeProjectable` seals POWL nodes.
- `TypedLoopNode<ARITY>` enforces loop structure law.
- Witnesses (`Ocel20`, `Xes1849`, `WfNetSoundnessPaper`) identify the formal grounding.

**wasm4pm execution required:** None for this level. The model is the artifact.

**Risk:** The model is a claim about intended behavior. Without an event log, no one knows
whether the claim matches reality. L2 is necessary but not sufficient.

---

## L3 — Monitored

**Identity:** Event logs exist and basic discovery is active.

**Characteristics:**
- An event log (XES or OCEL) is collected from the operational system.
- Process discovery has been run: an inductive miner or heuristic miner produces a model
  from the observed log.
- Basic conformance metrics (token replay fitness) are available.
- Monitoring is periodic, not continuous.
- No automated alerting. Conformance reports are reviewed manually.

**wasm4pm-compat type law:** Applicable for log structure and admission.
- `EventLog`, `Trace`, `Event` with builder chains.
- `OcelLog` with E2O/O2O links and object changes.
- `Admission<T, W>` and `Refusal<R, W>` for event admission.
- `LossReport<From, To, Items>` for OCEL→XES projections.

**wasm4pm execution required:**
- Discovery algorithm (inductive miner or equivalent) — `wasm4pm`.
- Token replay fitness computation — `wasm4pm`.
- Log import (XES/OCEL parsing) — `wasm4pm`.

**Risk:** Discovery and conformance are historical, not real-time. Drift goes undetected
between periodic checks. Repair is manual and undocumented.

---

## L4 — Conformance-driven

**Identity:** Alignment-based conformance, automated alerts, threshold-gated operations.

**Characteristics:**
- Alignment-based conformance checking (cost-optimal alignment) replaces or supplements
  token replay. Alignment is more precise; it identifies the minimum edit to make each
  trace conforming.
- Conformance thresholds are declared and enforced: operations below threshold trigger alerts.
- Monitoring is continuous or near-continuous (short polling intervals or event-driven).
- Concept drift detection is active: windowed conformance trends are tracked.
- Conformance reports are machine-generated, not manual.
- Some repair actions are automated (e.g., model update on sustained drift).

**wasm4pm-compat type law:** Strict features active.
- `ExportBoundaryConst<HAS_WITNESS, HAS_ROUND_TRIP>` for boundary claims.
- `StrictViolation` for named boundary violations.
- `ProcessBoundary` for declared process scope.

**wasm4pm execution required:**
- Alignment engine — `wasm4pm`.
- Continuous conformance loop — `wasm4pm`.
- Automated alert emission — `wasm4pm`.
- Threshold policy engine — `wasm4pm`.

**Risk:** Repair is partially automated but not fully receipted. The MAPE-K loop is incomplete:
Monitor and Analyze are present; Plan and Execute may still be manual.

---

## L5 — Autonomic

**Identity:** Full MAPE-K loop, self-repairing, provenance-complete.

**Characteristics:**
- The complete MAPE-K loop is operational: Monitor, Analyze, Plan, Execute, Knowledge.
- Every loop cycle produces typed artifacts at each stage.
- Every executed repair action produces a receipt.
- The Knowledge component accumulates historical patterns and improves repair decisions.
- Decommissioning produces a complete, replayable archive record.
- Any past loop cycle can be replayed from the archived artifacts.
- Audit responses are artifact-backed: "here is the receipt for every conformance check
  and every repair action taken over the process lifetime."

**wasm4pm-compat type law:** All features in use.
- `GraduationCandidate` (`src/graduation.rs` via `wasm4pm` feature) — identifies artifacts
  ready for full `wasm4pm` execution authority.
- `GraduateToWasm4pm` trait — the graduation bridge from compat to execution.
- Complete lifecycle from `Evidence<T, Raw, W>` through `Evidence<T, Receipted, W>`.

**wasm4pm execution required:**
- Full MAPE-K engine — `wasm4pm`.
- Receipt emitter for every loop component — `wasm4pm`.
- Replay verification for historical loop cycles — `wasm4pm`.
- Decommissioning archive assembler — `wasm4pm`.
- Adversarial benchmark judgment — `wasm4pm`.

**Board admissibility:** At L5, every governance claim about a process is backed by a
replayable, receipted artifact. "Our processes are formally modeled, continuously monitored,
self-repairing, and provenance-complete" is an auditor-admissible statement, not a narrative.

---

## Summary table

| Level | Identity | compat type law | wasm4pm required | Audit response |
|---:|---|---|---|---|
| L1 | Ad-hoc | None | None | Narrative |
| L2 | Documented | Full base | None | Model artifact |
| L3 | Monitored | Base + admission | Discovery, replay | Log + fitness report |
| L4 | Conformance-driven | Base + strict | Alignment, alerts | Alignment reports |
| L5 | Autonomic | All features | Full MAPE-K + receipts | Replayable receipt chain |

Process maturity = the progressive removal of unresolved process uncertainty from execution.
