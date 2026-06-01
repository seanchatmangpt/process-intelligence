# Acquisition-Ready Process Intelligence

**Doctrine:** Blue River Dam — the upstream closure layer is the control point of consequential software.

> A company cannot be acquisition-ready on process claims it cannot replay.
> Claims that cannot be replayed are narration. Narration is not evidence.

---

## The Five Criteria

### Criterion 1 — Formal Process Models

The target must have process models that are **discovered from event logs**, not hand-drawn.

**Requirement:**
- Process models produced by a recognized discovery algorithm (Inductive Miner, Split Miner, Heuristics Miner)
- Model format grounded in a public standard: BPMN 2.0, Petri net (WF-net with soundness certificate), POWL, process tree
- Models are versioned, dated, and tied to the log from which they were discovered

**What disqualifies:**
- Manually authored BPMN diagrams with no log provenance
- Process documentation in Word or Visio
- "As-designed" models with no evidence of "as-executed" conformance

---

### Criterion 2 — Replayable Event Logs

The target must have event logs that can be **replayed against a process model** to produce fitness scores.

**Requirement:**
- Logs in XES (IEEE 1849) or OCEL 2.0 format, or derivable to these formats via a documented projection
- Each event carries: case ID, activity, timestamp — minimum XES attributes
- Object-centric logs (OCEL 2.0) carry: object types, object IDs, event-to-object relations
- Logs span at least one full process cycle (discovery → closure)

**What disqualifies:**
- Flat relational tables with no case structure
- Logs with case IDs that cannot be reconstructed
- Logs where activity names are system codes not mapped to process semantics
- Sampling artifacts: logs exported for demonstration, not operational completeness

---

### Criterion 3 — Calculated Conformance

The target must have **conformance reports** produced by token-based replay or alignment-based conformance checking.

**Requirement:**
- Fitness score: fraction of log traces the model can replay without missing tokens
- Precision score: fraction of model behavior observed in the log (no silent over-fitting)
- Generalization and simplicity scores where available
- Reports are reproducible: given the same log + model, the same scores emerge

**What disqualifies:**
- Self-reported "process compliance" from internal audits
- Compliance percentages from rule engines (not process models)
- Fitness numbers without the underlying log and model available for re-computation

---

### Criterion 4 — Soundness Certificates

The target's process models must carry **structural soundness proofs**.

**Requirement for WF-nets:**
- Every transition is reachable from the source place
- Every transition can reach the sink place
- No dead transitions, no deadlocks, no livelocks
- Soundness certificate produced by a verifier (wasm4pm, pm4py, ProM) — not asserted by hand

**Requirement for POWL / process trees:**
- Tree structure is valid (no invalid operator arity)
- Loop nodes have exactly two children (do-body and redo-body)
- All leaf activities are reachable

**What disqualifies:**
- WF-nets described in documentation but not verified against soundness axioms
- Process trees with arity violations caught only by visual inspection

---

### Criterion 5 — Named Loss Policies and Auditable Evidence Chain

Every transformation of process evidence must carry a **named loss policy** and every admitted structure must have an **auditable evidence chain**.

**Requirement:**
- When OCEL 2.0 is projected to XES for analysis, the projection must carry:
  - A `ProjectionName` identifying what was projected
  - A `LossPolicy` decided before projection: `RefuseLoss`, `AllowNamedProjection`, or `AllowLossWithReport`
  - A `LossReport` enumerating what was dropped and why
- The evidence chain from raw event source to admitted process truth must be traversable:
  - Raw source → parsed structure → admitted evidence (typed, witnessed) → projected/exported artifact
- Every refusal must name the violated law (e.g., `MissingObjectRelation`, `FlatteningLoss`)

**What disqualifies:**
- Format conversions with no loss accounting
- "We export to XES for our tools" with no record of what object-centric relations were silently dropped
- Evidence chains that terminate at a dashboard export with no upstream traceability

---

## The Gap: Almost No Company Meets Criteria 2–5

### What companies typically produce:

| Criterion | Typical State | Acquisition-Ready State |
|---|---|---|
| 1 — Formal models | Hand-drawn BPMN in Visio/Lucidchart | Discovered models from logs via Inductive Miner |
| 2 — Replayable logs | Flat CSV exports, sampled for demos | Full OCEL 2.0 or XES logs with complete case structure |
| 3 — Calculated conformance | Internal audit pass/fail | Token-replay fitness ≥ 0.80, precision measured, reproducible |
| 4 — Soundness certificates | "Our process is sound" (assertion) | WF-net soundness verified, dead transitions = 0 |
| 5 — Named loss policies | Silent format conversion | ProjectionName + LossPolicy + LossReport on every transform |

The gap between Criterion 1 and Criteria 2–5 is not a tooling gap. It is a **doctrine gap**. Companies invest in process documentation (Criterion 1 partial) and compliance reporting (Criterion 3 self-asserted) but have never built the upstream admission layer that makes Criteria 2–5 possible.

---

## The Blue River Dam Advantage

Blue River Dam theory establishes: **whoever controls admissible process truth controls the downstream data, audit, governance, automation, and intelligence below it.**

A target that has implemented the wasm4pm stack does not merely have better process documentation. It has:

1. A typed admission layer that refuses weak evidence before it enters the system
2. Replayable logs that prove process consequence, not just record activity
3. Conformance reports that a buyer can independently re-run against the same data
4. Soundness certificates that are machine-verified, not human-asserted
5. Loss policies that make every format transformation auditable

This is not a feature advantage. This is a **structural evidence advantage**. The buyer does not need to trust the seller's claims — the buyer can replay the evidence and derive the same conclusions.

**The dam must be upstream.** A target without the upstream closure layer cannot make acquisition-ready process claims, regardless of how many downstream dashboards it produces.

---

## Buyer Test

For each criterion, the buyer's test is:

| Criterion | Buyer Test |
|---|---|
| 1 | "Show me the log you ran the discovery algorithm against and the algorithm parameters." |
| 2 | "Give me the OCEL 2.0 file. I will replay it myself." |
| 3 | "Give me the log and the model. I will re-run conformance checking and verify your fitness score." |
| 4 | "Run the WF-net soundness verifier against your model in my environment." |
| 5 | "Show me the LossReport for every OCEL→XES projection in your analytics pipeline." |

A target that can pass all five buyer tests is acquisition-ready. A target that fails any test past Criterion 1 is providing narration, not evidence.

---

*Grounded in: Blue River Dam doctrine, wasm4pm-compat type surfaces, OCEL 2.0 standard, IEEE XES 1849, Van der Aalst process mining foundations.*
