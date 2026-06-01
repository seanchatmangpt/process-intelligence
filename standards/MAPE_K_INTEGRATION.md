# MAPE-K — Autonomic Process Intelligence

**Authority:** IBM Autonomic Computing Blueprint (Kephart & Chess 2003); MAPE-K loop
**Context:** ~/chatmangpt/knhk/MAPE-K_AUTONOMIC_INTEGRATION.md

---

## Overview

MAPE-K (Monitor–Analyze–Plan–Execute–Knowledge) is the standard autonomic computing
feedback loop for self-managing systems. In the process intelligence context, MAPE-K
governs how a running process system observes its own execution, identifies deviations,
decides on interventions, and acts — continuously and without manual escalation.

---

## The Five Components

### 1. Monitor

Continuously collects evidence from the running process:
- Performance metrics: latency, throughput, resource utilization.
- Reliability metrics: error rate, failure rate, SLA compliance.
- Process metrics: activity frequency, case duration, rework rate.
- Object-centric signals: E2O link counts, object change rates, O2O relationship health.

In the wasm4pm stack: Monitor maps to **streaming OTel trace ingestion** →
`OTelToProcessInstance` bridge → incremental OCEL log update. Each arriving trace span
is a `Monitor` observation.

### 2. Analyze

Takes observations and applies analysis rules:
- Match observed patterns against discovered process models (DFG, POWL, Petri net).
- Identify root causes via SPARQL queries over the process intelligence ontology.
- Assess severity: which Declare constraints are violated, which WF-net soundness
  conditions are at risk.
- Compute conformance metrics (fitness, precision) over the current event window.

In the wasm4pm stack: Analyze maps to **streaming conformance checking** —
`StreamingConformanceSession` (from the process intelligence ontology). The `wasm4pm`
engine replays arriving events against the admitted process model and emits
`ConformanceResult` values in real time.

### 3. Plan

Takes analysis results and decides on interventions:
- Apply autonomic policies (e.g. "if fitness drops below 0.85, alert and quarantine").
- Select actions based on historical success rates stored in the Knowledge base.
- Sequence actions to respect the process model's declared constraints (Declare).
- Assess risk of each intervention against the WF-net soundness guarantee.

In the wasm4pm stack: Plan maps to **prescriptive process intelligence** —
`prediction.rs` outcome models and `diagnostic.rs` remediation shapes provide the
structural vocabulary for plan artifacts. A plan is a `GraduationCandidate` that names
its grounding.

### 4. Execute

Takes the ordered plan and actuates:
- Fires process interventions (reassign, escalate, compensate, skip).
- Monitors the effect of each action and adjusts if needed.
- Records the execution as a new event in the OCEL log — closing the evidence loop.

In the wasm4pm stack: Execute maps to **lifecycle actuation** — the `wasm4pm` engine
emits events that feed back into the Monitor layer. No actuation occurs without an
evidence record; every action produces an `OcelEvent` that can be mined.

### 5. Knowledge

Persistent learning store:
- Patterns learned from past conformance results.
- Success rates for remediation actions by process variant.
- Trained prediction models for outcome prediction.
- Refined autonomic policies based on execution history.

In the wasm4pm stack: Knowledge maps to the **process intelligence ontology** at
`~/chatmangpt/ostar/ontology/core/process_intelligence.nt` — `POWLSynthesisRun`,
`POWLConformanceResult`, `complianceRate`, `reworkRate`, and the broader class hierarchy
are the persistent knowledge schema.

---

## MAPE-K in the wasm4pm Architecture

```
OTel Traces
    │
    ▼ Monitor
OcelLog (incremental)
    │
    ▼ Analyze
StreamingConformanceSession
    │
    ▼ Plan
GraduationCandidate (prescriptive shape)
    │
    ▼ Execute
OcelEvent (actuation record)
    │
    ▼ back to Monitor
```

The feedback loop is closed entirely through the OCEL event log. No actuation is hidden:
every MAPE-K Execute step is observable as a process event, making the autonomic loop
itself mineable.

---

## Relationship to wasm4pm-compat

The wasm4pm-compat crate defines the structural shapes that travel through the MAPE-K
loop:

| MAPE-K phase | wasm4pm-compat shapes used |
|---|---|
| Monitor | `OcelLog`, `OcelEvent`, `EventObjectLink` |
| Analyze | `Dfg`, `Powl`, `WfNet`, `DeclareConstraint`, `Metric<KIND, NUM, DEN>` |
| Plan | `GraduationCandidate`, `DiagnosticShape`, `PredictionShape` |
| Execute | `OcelEvent` (actuation record), `ReceiptShape` (provenance) |
| Knowledge | `Witness` markers, `Receipt`, process intelligence ontology |

None of the MAPE-K logic (rule matching, policy evaluation, actuation dispatch) lives in
wasm4pm-compat. It is all engine responsibility that graduates to `wasm4pm`.

---

## Board Placement

The MAPE-K autonomic loop transforms process intelligence from a retrospective analytics
tool into a real-time self-managing system. wasm4pm-compat provides the type-safe
structural vocabulary that travels through every MAPE-K phase — ensuring that the
autonomic loop operates on admitted, well-shaped evidence rather than raw strings and
untyped maps.
