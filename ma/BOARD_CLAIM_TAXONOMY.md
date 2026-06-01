# Board Claim Taxonomy by Tier

**Doctrine:** A claim made to a board is only as strong as the evidence that can survive independent replay.
Without a replayable receipt, a board claim is narration with a slide deck attached.

---

## Overview

Board claims in M&A contexts follow a maturity ladder. Claims at higher tiers require stronger evidence.
Most companies make T5 and T6 claims while possessing only T1 evidence.

| Tier | Name | Core Claim | Evidence Minimum | Buyer Verifiable? |
|---|---|---|---|---|
| T1 | Existence | "This process exists" | Documentation, org chart, SOP | No — cannot replay |
| T2 | Coverage | "This process covers N% of our operations" | System metrics, row counts | Partial — statistical only |
| T3 | Conformance | "This process runs as designed" | OCEL/XES log + process model + conformance report | Yes — replay required |
| T4 | Performance | "This process achieves X cycle time / throughput / error rate" | Event log with timestamps + case duration analysis | Yes — computed from log |
| T5 | Control | "We can detect and halt non-conforming process execution in real time" | wasm4pm running on live data + refusal evidence | Yes — requires live trial |
| T6 | Recovery | "When a process variant emerges, we can identify root cause, quantify impact, and correct upstream" | Process variant analysis + conformance delta + remediation receipt | Yes — requires full replay chain |

---

## Tier 1 — Existence

**Claim pattern:** "We have a formal order-to-cash process."

**Evidence requirement:**
- Process documentation (BPMN diagram, SOP, runbook)
- Org chart showing process ownership
- System-of-record identification (which ERP, CRM, or platform owns the case)

**Which tool produces evidence:**
- Any documentation tool: Confluence, Notion, Visio, Lucidchart
- No process mining required

**What buyer can verify:**
- That documentation exists
- That a system-of-record is named
- Nothing about actual execution conformance

**Limitation:** T1 is necessary but not sufficient for acquisition-ready process claims.
T1 evidence is universally available and provides no structural differentiation.

---

## Tier 2 — Coverage

**Claim pattern:** "95% of our revenue flows through the formally documented process."

**Evidence requirement:**
- System metrics from the named system-of-record (transaction counts, case volumes)
- Mapping from system activity to documented process steps
- Statistical sampling methodology if full-population data is unavailable

**Which tool produces evidence:**
- BI/analytics tools: Tableau, Looker, PowerBI, Salesforce reports
- Process mining tools can produce this but are not required

**What buyer can verify:**
- Volume claims against system data extracts
- Mapping logic from system activity to process steps
- Statistical validity of sampling methodology

**Limitation:** Coverage claims are statistical. They do not prove that covered cases conform to the process model.
A process can have 100% coverage and 40% conformance fitness.

---

## Tier 3 — Conformance

**Claim pattern:** "Token-based replay of our order-to-cash OCEL log against our discovered process model yields fitness 0.87."

**Evidence requirement:**
- OCEL 2.0 or XES event log covering the claimed process
- Process model in a verifiable format (WF-net, POWL, process tree, BPMN with formal semantics)
- Conformance report produced by replay or alignment-based checking
- Fitness, precision, generalization, simplicity scores
- Model soundness certificate (for WF-nets)

**Which tool produces evidence:**
- wasm4pm (primary) — token-based replay, alignment-based conformance, soundness checking
- pm4py — open-source reference implementation
- ProM — academic reference implementation

**What buyer can verify:**
- Buyer receives the log and model
- Buyer re-runs conformance checking in their environment
- Buyer derives the same fitness score independently
- Any deviation in fitness score is a due diligence finding

**Distinction from T2:** T3 claims are **machine-verifiable**. The buyer does not need to trust the seller's methodology.
The seller's claim is either reproducible or it is false.

---

## Tier 4 — Performance

**Claim pattern:** "Median case duration for order-to-cash is 4.2 days. P95 is 11 days. Throughput is 340 cases/week."

**Evidence requirement:**
- Event log with timestamps (case start and end events are required)
- Case duration computation from log (not from system-reported metrics)
- Cycle time distribution (not just mean — distribution reveals rework and hidden loops)
- Bottleneck analysis identifying which activities accumulate waiting time

**Which tool produces evidence:**
- wasm4pm — performance mining, cycle time analysis, bottleneck identification
- pm4py — dotted chart, case duration statistics

**What buyer can verify:**
- Buyer re-computes case durations from the event log timestamps
- Buyer verifies that reported median/P95 match log-derived computation
- Buyer identifies hidden loops and rework cycles not visible in headline metrics

**Trap:** Sellers often report system-derived performance metrics (ERP-generated SLA reports) rather than log-derived metrics.
System-derived metrics reflect the system's view of process completion, which may exclude rework events, failed attempts, and bypasses.
Log-derived metrics reflect actual execution.

---

## Tier 5 — Control

**Claim pattern:** "Our process control layer detects non-conforming execution in real time and can halt or route to remediation before downstream propagation."

**Evidence requirement:**
- wasm4pm running against live operational data (not historical batch)
- Evidence of refused events: events that were rejected by the admission layer with named law violations
- Refusal log showing: timestamp, event type, violated law, disposition
- Evidence that refusals propagated upstream (not silently swallowed)

**Which tool produces evidence:**
- wasm4pm — admission layer, refusal evidence, real-time conformance
- wasm4pm-compat — typed evidence structures, named refusal reasons

**What buyer can verify:**
- Buyer requires wasm4pm running on buyer data during due diligence trial
- Buyer submits known-non-conforming test events
- Buyer observes refusal evidence with named law violations
- Buyer verifies refusals are logged and traceable

**Why T5 is rare:** T5 requires the upstream admission layer to be integrated into operational systems —
not a post-hoc analytics tool but a runtime process gate. Almost no companies have built this.
Companies that claim T5 without wasm4pm integration are claiming T3 and calling it T5.

---

## Tier 6 — Recovery

**Claim pattern:** "When a process variant emerges, we identify root cause within 2 hours, quantify downstream impact within 4 hours, and emit a remediation receipt within 24 hours."

**Evidence requirement:**
- Process variant analysis: log-derived identification of deviating traces
- Conformance delta: before/after conformance scores showing the variant's impact
- Root cause trace: which activities and objects caused the deviation
- Remediation receipt: a cryptographic or typed record that the variant was addressed and the lawful path restored
- Replay proof: the corrected process can be replayed from the remediation receipt forward

**Which tool produces evidence:**
- wasm4pm — variant analysis, conformance delta, receipt emission
- wasm4pm-compat — receipt types, evidence chain from raw to remediated

**What buyer can verify:**
- Buyer examines historical variant events and their corresponding remediation receipts
- Buyer verifies that remediation receipts exist for all documented variants
- Buyer re-runs conformance checking post-remediation to verify restoration
- Buyer tests recovery time claims against the receipt timestamps

**Why T6 is the crown:** T6 closes the loop. T1–T5 establish that process execution is structured, measured, and controlled.
T6 proves the system is **self-correcting under witness**. This is the closest available analog to a formal process correctness guarantee.

---

## Claim Tier vs. Evidence Reality: The Gap Table

| Board Claim Made | Tier Claimed | Evidence Tier Actually Held | Gap |
|---|---|---|---|
| "Our processes are SOC2-compliant" | T5 | T1–T2 | 3–4 tiers |
| "We have a formal order-to-cash process" | T3 | T1 | 2 tiers |
| "Our process efficiency improved 18% YoY" | T4 | T2 (BI metrics) | 2 tiers |
| "We detect and remediate process drift" | T6 | T1 (manual review) | 5 tiers |
| "Our OCEL log shows fitness 0.87" | T3 | T3 | 0 tiers — defensible |

The gap between claimed tier and held evidence tier is the **process claim liability** of the target.
Buyers who identify this gap during diligence have grounds to reprice or restructure.

---

*Grounded in: Van der Aalst process mining conformance checking, Blue River Dam doctrine, wasm4pm-compat type surfaces, OCEL 2.0, IEEE XES 1849.*
