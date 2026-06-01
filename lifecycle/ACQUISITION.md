# ACQUISITION — Process Intelligence in M&A Context

## What a buyer needs: process maps replayable against historical logs

An M&A buyer who is serious about process due diligence needs one capability above all others:
the ability to replay the seller's process claims against the seller's historical event logs.

This means:
- The seller's process model (WF-net, BPMN, POWL) must be available as a typed artifact.
- The seller's historical event logs must be admitted OCEL or XES — not raw CSVs, not
  database exports, not PDF audit reports.
- The conformance check (token replay or alignment) must be reproducible: the buyer runs
  the replay independently and gets the same conformance scores the seller reported.
- The replay result must match the seller's deck claims within declared tolerance.

If replay is impossible, the seller's process claims are unverifiable. The buyer is purchasing
narrative, not evidence.

## What a seller provides: admitted OCEL logs, conformance reports, graduation-ready types

A seller at process intelligence maturity L3 or above can provide:

| Artifact | Type | How produced |
|---|---|---|
| Process model | `WfNetConst<SOUNDNESS>` or BPMN/POWL typed artifact | Design phase |
| Historical event log | Admitted `OcelLog` or `EventLog` | Operation phase, admitted through `Admit::admit()` |
| Conformance reports | Time-series of `Metric<Fitness, NUM, DEN>` | Monitoring phase |
| Repair receipts | `Receipt` per repair action | Repair phase |
| Refusal log | `Refusal<R, W>` with named laws | Operation phase |
| Archive record | Typed decommissioning artifact (if relevant) | Decommissioning phase |
| Graduation-ready types | `GraduationCandidate` instances | `wasm4pm` feature bridge |

Graduation-ready types are particularly valuable to a sophisticated buyer who operates their
own `wasm4pm` execution environment. The seller's `GraduationCandidate` instances can be
graduated into the buyer's execution context without re-admission from scratch.

## The diligence gap: most companies cannot replay their M&A deck claims

The standard M&A process due diligence workflow:
1. Seller prepares a process deck: "our order-to-cash cycle time is 3.2 days; our compliance
   rate is 98.7%; our process is ISO 9001 certified."
2. Buyer reviews the deck, interviews management, reviews audit reports.
3. Buyer accepts or rejects the claims based on documentation and interviews.

What is missing:
- The event log that produced the "3.2 days" number is not provided.
- The "98.7% compliance rate" cannot be reproduced because no conformance check is available.
- The ISO certification refers to an audit at a point in time, not continuous conformance.
- The process model used in the certification may not match the process actually running.

This is the diligence gap: the claims are real to the seller (someone computed them), but
they are not replayable by the buyer. The buyer cannot distinguish an accurate claim from
an aspirational one.

## The diligence gap as a risk premium

A buyer who cannot replay claims applies a risk premium to cover unverifiable uncertainty:
- "Process claims unverifiable" → higher integration cost assumption.
- "Compliance rate unverifiable" → regulatory risk reserve.
- "Cycle time unverifiable" → customer SLA risk reserve.
- "No event logs available" → process re-discovery cost assumption.

The risk premium is not punitive. It reflects real uncertainty. A seller with replayable
process evidence eliminates the uncertainty and eliminates the risk premium.

## The process intelligence diligence package

A seller operating at L3+ can produce a process intelligence diligence package:

### Layer 1: Model evidence
- Typed process models for all material processes.
- Soundness receipts (type-level for WF-net soundness, structural verification for others).
- Witness annotations: which papers and standards ground each model.

### Layer 2: Log evidence
- Admitted OCEL logs for the due diligence period (typically 24 months).
- Log coverage statement: what processes are covered, what is excluded and why.
- Refusal log: events that were refused admission and the named law violated.

### Layer 3: Conformance evidence
- Conformance reports for the due diligence period.
- Trend analysis: fitness and precision over time.
- Concept drift analysis: where the process changed and how.

### Layer 4: Repair evidence
- Repair receipt chain: every automated repair action with its trigger, action, and outcome.
- Escalation log: cases where automated repair failed and human intervention occurred.
- Post-repair conformance confirmation: proof that repair restored conformance.

### Layer 5: Replay verification
- Instruction set for the buyer to reproduce conformance scores independently.
- Buyer runs `wasm4pm` (or equivalent) against the admitted logs and the provided models.
- Buyer-computed scores must match seller-reported scores within declared tolerance.

## What a buyer can demand vs. what a seller can honestly provide

| Buyer demand | L1 seller | L3 seller | L5 seller |
|---|---|---|---|
| "Show me your process model." | Slide deck | Typed model artifact | Typed model + soundness receipt |
| "Replay your conformance claim." | Cannot | Token replay possible | Full alignment replay |
| "Show your repair history." | None | Partial (manual) | Complete receipt chain |
| "Independent verification?" | Impossible | Possible with effort | Immediate: replay the artifacts |
| "Process provenance to decommission?" | None | Partial | Complete archive record |

## The Blue River Dam doctrine applied to acquisition

The Blue River Dam doctrine: whoever controls admissible process truth controls the downstream
data, audit, governance, automation, and intelligence below it.

In acquisition:
- A seller with admitted process evidence controls the narrative of their own business.
- A buyer without process intelligence tools cannot independently verify what they are buying.
- A buyer with `wasm4pm` replay capability can verify any claim the seller makes — and
  detect claims the seller cannot support.

Process intelligence is not a compliance feature. It is a transactional instrument. The seller
who cannot produce admitted logs and replayable conformance checks is selling unverifiable
narrative. The buyer who cannot replay claims is accepting unquantified risk.

The dam is upstream. In acquisition, the dam is the admitted evidence package. Everything
downstream — valuation, integration cost, regulatory risk — flows from what the dam contains.
