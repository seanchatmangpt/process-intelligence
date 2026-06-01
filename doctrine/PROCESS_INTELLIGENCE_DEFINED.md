# Process Intelligence — Canonical Definition

## Van der Aalst Grounded Formal Definition

Process intelligence is the systematic application of process mining, conformance checking, and
evidence-grounded execution control across the full lifecycle of a process — from design-time
intent to decommission-time receipt.

Grounded in Wil van der Aalst's foundational work on process mining (van der Aalst, 2016;
van der Aalst et al., 2023 — OCEL 2.0), the canonical definition is:

> **Process Intelligence** is the discipline of extracting, admitting, and actuating on
> formally grounded process evidence — converting observable execution traces into typed,
> refusal-capable, receipt-bearing process truth that governs downstream data, audit,
> automation, and board-level claims.

The key terms carry precise meaning:

- **Formally grounded**: Claims derive from published process mining theory (Petri nets,
  WF-nets, POWL, process trees, Declare, OCEL, XES, conformance algorithms) not from
  informal observation
- **Admitting**: The act of accepting evidence under a named witness — not merely recording it
- **Refusal-capable**: A system that cannot refuse cannot govern; refusals must name the
  violated law, not say "invalid input"
- **Receipt-bearing**: Every critical state transition emits a cryptographic receipt proving
  lawful execution; no receipt means no closure
- **Process truth**: The outcome of admission + conformance + replay — not a log entry, not
  a dashboard metric

---

## What Process Intelligence Is NOT

- Process intelligence is NOT business intelligence (BI). BI aggregates reported data.
  Process intelligence mines actual execution behavior and detects model-log divergence.
- Process intelligence is NOT observability. Observability records what happened.
  Process intelligence proves whether what happened conforms to what was declared.
- Process intelligence is NOT workflow automation. Automation executes declared steps.
  Process intelligence validates that execution conforms to law and refuses non-conforming paths.
- Process intelligence is NOT monitoring dashboards. Dashboards display metrics.
  Process intelligence issues typed verdicts grounded in formal process models.

---

## Five Maturity Levels (Blue River Dam Doctrine)

The five maturity levels define the progressive removal of unresolved process uncertainty
from execution. Source: [Blue River Dam doctrine](file:///Users/sac/process-intelligence/doctrine/BLUE_RIVER_DAM.md).

### Level 1 — Records Activity

**Identity:** No process mining.
**Law:** Records that something happened.
**Characteristic:** Activity logs, audit trails, event streams. No formal process model.
  No conformance checking. No fitness measurement. No refusal capability.
**Evidence shape:** Raw log entries. Timestamps. Actor IDs. No object lifecycle tracking.
**Limitation:** Cannot detect model-log divergence. Cannot prove a lawful process happened.
  Cannot refuse non-conforming execution.

### Level 2 — Structures Evidence

**Identity:** wasm4pm-compat doorway.
**Law:** Structures evidence under typed witness discipline.
**Characteristic:** Process-evidence types (EventLog, OcelLog, Petri nets, POWL, process trees).
  Admission/refusal with named law violations. Loss accounting with named projections.
  Witness markers proving which standard governs each structure.
**Evidence shape:** Typed, admitted evidence with witness tags. Refusals carrying named laws.
  Loss reports on projection paths.
**Limitation:** Structure only — no execution, no mining, no conformance checking, no receipts.
  The doorway must not become the throne room.

### Level 3 — Judges Evidence Claims

**Identity:** Strict compat covenant.
**Law:** Applies strict admission/refusal judgment at process boundaries.
**Characteristic:** Strict feature enabled. ProcessBoundary declarations. StrictViolation
  surfaces catching MissingLossPolicy, MissingRefusalPath, HiddenProcessMiningGrowth.
  Export boundary const-generic enforcement.
**Evidence shape:** Boundary-declared evidence with violation receipts on non-conforming paths.
**Limitation:** Judgment only — cannot execute, mine, or conform-check at scale. Prepares
  evidence for execution authority.

### Level 4 — Prepares Execution Authority

**Identity:** Graduation bridge.
**Law:** Prepares grounded evidence for full wasm4pm execution.
**Characteristic:** GraduationCandidate with grounded/ungrounded distinction. GraduateToWasm4pm
  trait. Hard signal detection. Evidence ready for wasm4pm consumption.
**Evidence shape:** Graduation-ready evidence with grounding status and graduation reason.
**Limitation:** Handoff only — the bridge does not execute. The throne room executes.

### Level 5 — Adjudicates Process Truth

**Identity:** Full wasm4pm.
**Law:** Branchless process mining, conformance, replay, receipts, benchmark gates.
**Characteristic:** Complete execution authority. Alpha/inductive/heuristic miner.
  Token-based and alignment-based conformance. Fitness, precision, generalization.
  BLAKE3 receipt emission. Adversarial benchmark judgment. Branchless hot path.
**Evidence shape:** Receipted, replayed, benchmarked process truth. Board-admissible claims.
**Capability:** Full process intelligence. Can issue ALIVE verdicts with cryptographic proof.

---

## Full-Lifecycle Scope

Process intelligence spans the complete process lifecycle:

```
Design → Monitoring → Optimization → Simulation → Repair → Decommissioning
   ↑                                                              |
   └──────────────── receipt chain ──────────────────────────────┘
```

Each phase produces:
- **Design**: Formal process model (Petri net / POWL / process tree / Declare)
- **Monitoring**: Conformance score (fitness, precision, generalization) against observed logs
- **Optimization**: Discovered variant distribution, bottleneck identification, rework loops
- **Simulation**: Predicted behavior under modified models, synthetic log generation
- **Repair**: Non-conformance remediation plans, model update proposals
- **Decommissioning**: Final receipt chain, archival log, closure proof

A process is not decommissioned until it has a closure receipt.

---

## The Upstream Dam Principle

> Consequential software cannot be governed by downstream interpretation of activity records.
> It must manufacture admissible process truth upstream.

Traditional flow: write code → deploy → observe → collect logs → explain later.

Blue River Dam flow: define lawful work → admit evidence → refuse weak claims →
execute through bounded cells → emit receipts → replay consequence → let downstream consume proof.

**Whoever controls admissible process truth controls downstream data, audit, governance,
automation, intelligence.**

This is the central law of process intelligence.

---

## Sources

- van der Aalst, W.M.P. (2016). *Process Mining: Data Science in Action*. Springer.
- van der Aalst et al. (2023). OCEL 2.0 Standard Specification.
- wasm4pm-compat/docs/BLUE_RIVER_DAM.md — Five maturity levels, law stack, upstream dam principle.
- wasm4pm-compat/docs/MATURITY.md — Full 7×5 maturity matrix.

---

## Section 10: Anti-Regression Audit Mesh (v30.1.1 Spec)

The repository enforces invariants via a mesh of 23 audit scripts divided into:
* **Hard Audits**: Immediate blockers (e.g. no engine creep, feature boundaries).
* **Soft Audits**: Quality metrics and coverage warning thresholds.

**Algorithm: Crown Audit Protocol (`audit\_crown\_gate\_all.sh`):**
1. Let $\mathcal{S} = \{s_1, \ldots, s_{22}\}$ be the subordinate audit scripts.
2. Initialize $\text{FAIL} \leftarrow 0$, $\text{defects} \leftarrow \emptyset$.
3. For each $s_i \in \mathcal{S}$:
   * If $\text{exit}(s_i(\mathcal{R})) \neq 0$:
     * $\text{FAIL} \leftarrow \text{FAIL} + 1$
     * $\text{defects} \leftarrow \text{defects} \cup \{s_i\}$
4. If $\text{FAIL} = 0$, return exit 0. Else, print defects and return exit 1.

