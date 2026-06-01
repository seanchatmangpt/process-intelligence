# Due Diligence Requirements by Claim Tier

**Doctrine:** Buyer reliance on process claims requires independently verifiable evidence.
Evidence that cannot be re-derived by the buyer is representation, not proof.

---

## Overview

Due diligence requirements scale with the claim tier (see BOARD_CLAIM_TAXONOMY.md).
This document specifies what the buyer must receive, what the buyer must be able to produce independently,
and what most targets cannot provide.

---

## T3 Conformance Claims — Full Requirements

### What the buyer must receive

| Artifact | Format | Provenance Required |
|---|---|---|
| Event log | OCEL 2.0 (.json or .sqlite) or XES (.xes or .xes.gz) | Full operational population, not sampled for demonstration |
| Process model | WF-net (.pnml), POWL, process tree, or BPMN with formal semantics | Discovered from the event log via named algorithm with parameters |
| Conformance report | Token-replay or alignment-based output | Produced by wasm4pm, pm4py, or ProM — not a hand-written summary |
| Model soundness certificate | WF-net: soundness verification output | Produced by a verifier, not asserted |
| Discovery parameters | Algorithm name, configuration, date of run | Reproducibility of discovery |

### What the buyer must be able to do independently

1. Re-run token-based replay of the seller's log against the seller's model
2. Derive fitness, precision, generalization, and simplicity scores
3. Verify that the buyer-derived scores match the seller's claimed scores within tolerance (±0.02)
4. Re-run WF-net soundness verification and confirm zero dead transitions, no deadlocks
5. Re-run discovery against the log with the seller's parameters and confirm model similarity

### OCEL log — the specific requirement

**The single most important artifact for T3 verification is an OCEL 2.0 log with object-centric relations.**

An OCEL 2.0 log must contain:
- Event types with attributes
- Object types with attributes
- Event-to-object relations (E2O): which objects each event references
- Object-to-object relations (O2O): which objects reference other objects
- Object attribute change history

**What flat XES logs cannot prove:**
- Object-centric conformance: whether object lifecycles (orders, cases, line items) are individually conforming
- Cross-object causality: whether events on one object type are causally consistent with events on another
- Flattening loss: what structural information was silently dropped when converting OCEL to XES for analysis

A target that provides XES logs but not OCEL logs is providing **flattened evidence**. Flattened evidence cannot prove object-centric conformance. The buyer cannot determine what was lost in the flattening.

### What most companies cannot produce for T3

| Requirement | Why most companies fail |
|---|---|
| OCEL 2.0 log | No system produces OCEL natively; requires extraction pipeline with E2O/O2O relation reconstruction |
| Object-centric relations | ERP/CRM systems store relations in foreign keys — reconstructing them into OCEL requires explicit engineering |
| Discovered process model | Most companies have hand-drawn models; discovery from logs is a process mining capability few have deployed |
| Reproducible conformance report | Requires wasm4pm or pm4py integrated into data pipeline, not one-off analysis |
| Model soundness certificate | Requires WF-net formalization of the process model, which most BPMN authors have never done |

**The OCEL gap is the deepest.** Companies that have XES logs have made a start. Companies that have OCEL 2.0 logs with E2O and O2O relations have crossed the structural threshold that makes object-centric process mining possible.

---

## T5 Control Claims — Full Requirements

### What the buyer must receive

| Artifact | Format | Provenance Required |
|---|---|---|
| Refusal log | Typed refusal events with named law violations | Produced by wasm4pm admission layer in production |
| Admission evidence | Typed admitted events with witness markers | Produced by wasm4pm-compat evidence chain |
| Control surface description | Which process boundaries are under real-time control | Mapped to wasm4pm feature surfaces |
| Trial access agreement | Terms for buyer to run wasm4pm against buyer data | Required for independent T5 verification |

### What the buyer must be able to do independently

1. Submit known-non-conforming test events to the seller's process control layer
2. Observe refusal evidence with named law violations (not generic "invalid input" errors)
3. Verify that refusals are logged, typed, and traceable to specific law violations
4. Run wasm4pm against a buyer-provided dataset during due diligence trial period
5. Verify that buyer data produces the same control behavior as seller-described

### The wasm4pm trial requirement

**T5 claims cannot be verified from historical artifacts alone.** Control claims are runtime claims.
The buyer must observe the control layer operating against live or buyer-provided data.

Minimum trial requirements:
- 2-week trial period with buyer-provided representative dataset
- Buyer submits 20 known-conforming events: all must be admitted with typed evidence
- Buyer submits 10 known-non-conforming events: all must be refused with named law violations
- Buyer submits 5 ambiguous events: seller must disclose disposition policy in advance
- Buyer independently verifies refusal log completeness and law violation naming

### What most companies cannot produce for T5

| Requirement | Why most companies fail |
|---|---|
| Named law violations in refusals | Most systems produce "validation failed" or error codes — not named structural laws |
| Typed admission evidence | Requires wasm4pm-compat evidence types; most systems have no equivalent structure |
| Real-time process control | Most process mining is post-hoc analytics, not runtime admission gates |
| Trial on buyer data | Requires portable wasm4pm deployment; most tools are cloud-only with vendor data residency |

---

## What Most Companies Cannot Produce: The OCEL Log Problem

The OCEL 2.0 log with object-centric relations is the single artifact that most companies lack and that
most acquisition-ready process claims depend on.

### Why OCEL is not produced by default

Modern enterprise systems (ERP, CRM, BPM) store process data in relational schemas optimized for transaction processing, not process mining. Extracting an OCEL 2.0 log requires:

1. **Case identification**: Which field or combination of fields constitutes a case ID for each object type?
2. **Event extraction**: Which database events map to which process activities?
3. **E2O relation reconstruction**: For each event, which objects does it reference? (An invoice event references: an order, a customer, a line item, a product — each is a separate object type with its own lifecycle.)
4. **O2O relation reconstruction**: Which objects reference other objects? (An order contains line items; a shipment fulfills an order.)
5. **Attribute history reconstruction**: When did object attributes change? (Price updates, status changes, owner reassignments.)

Steps 3–5 require deep knowledge of the source system schema and are almost never done during standard data extraction for analytics.

### The consequence

A company that provides flat XES logs for T3 verification has provided:
- Activity-level conformance evidence (can replay individual traces)
- But not object-centric conformance evidence (cannot verify cross-object causal consistency)

The buyer cannot determine from XES logs whether:
- Individual order lifecycles are conforming (orders can appear conforming when viewed in isolation but non-conforming in relation to their line items)
- Cross-object causal violations exist (an invoice was emitted before the corresponding shipment was confirmed)
- Object-centric variants exist (some customer types follow different process patterns)

**OCEL 2.0 is the admission threshold for object-centric process claims.**
A target that cannot produce OCEL 2.0 cannot make object-centric conformance claims to a buyer.

---

## Diligence Checklist by Tier

### T3 Conformance Diligence Checklist

- [ ] OCEL 2.0 log received (not XES-only)
- [ ] E2O relations present in OCEL log (verified by schema inspection)
- [ ] O2O relations present in OCEL log (if claimed)
- [ ] Process model received in formal format (PNML, POWL, process tree)
- [ ] Model discovered from log (not hand-drawn — provenance documented)
- [ ] Discovery algorithm and parameters documented
- [ ] Conformance report received (token-replay or alignment)
- [ ] Fitness score buyer-reproducible (re-run yields ±0.02 of claimed score)
- [ ] Model soundness verified (WF-net: zero dead transitions, no deadlocks)
- [ ] Log covers full operational population (not demonstration sample)

### T5 Control Diligence Checklist

- [ ] wasm4pm deployed in seller production environment (not just development)
- [ ] Refusal log available covering at least 90 days
- [ ] Named law violations present in refusals (not generic error codes)
- [ ] Admitted evidence carries typed witness markers
- [ ] Trial agreement executed for buyer-data testing
- [ ] 20 conforming test events: all admitted
- [ ] 10 non-conforming test events: all refused with named laws
- [ ] 5 ambiguous test events: disposition policy disclosed and observed
- [ ] Refusal log completeness verified (no silent swallowing)

---

*Grounded in: OCEL 2.0 standard, IEEE XES 1849, Blue River Dam doctrine, wasm4pm-compat type surfaces, Van der Aalst conformance checking.*
