---
artifact: MANIFESTO
project: linkedin-public-canon
date: 2026-06-02
gate: Sheep Gate (secondary)
status: DRAFT
public_url: PENDING
doctrine_sources:
  - doctrine/PROCESS_INTELLIGENCE_DEFINED.md
  - doctrine/BLUE_RIVER_DAM.md
  - doctrine/CONFORMANCE_AS_LAW.md
  - doctrine/OBJECT_CENTRIC_SUPREMACY.md
  - doctrine/PROCESS_INTELLIGENCE_IS_NOT.md
---

# Process Intelligence Manifesto

## The Category Claim

Full-lifecycle process intelligence is a new discipline. It is not business intelligence. It is not
observability. It is not workflow automation. It is not a monitoring dashboard. It is the systematic
application of process mining, conformance checking, and evidence-grounded execution control across
the full lifecycle of a process — from design-time intent to decommission-time receipt.

Source: doctrine/PROCESS_INTELLIGENCE_DEFINED.md

---

## What Is at Stake

Software systems make decisions that determine outcomes for people, organizations, and markets.
Most of those systems cannot prove that a lawful process happened. They can record that events
occurred. They cannot prove that the execution conformed to what was declared. They cannot refuse
non-conforming paths. They cannot emit a receipt proving lawful closure.

This is not an edge case. This is the default condition of consequential software today.

The gap between recording activity and adjudicating process truth is the central technical and
governance problem of the next decade.

Source: doctrine/BLUE_RIVER_DAM.md

---

## The Distinction That Matters

There is a hard distinction between an activity record and process evidence:

- An **activity record** says something happened.
- **Process evidence** says what happened under a witness, with typed identity, lawful boundary,
  admissible structure, possible refusal, and a replayable receipt.

A log is not truth. A receipt is a proof record.

Consequential software cannot be governed by downstream interpretation of activity records. It must
manufacture admissible process truth upstream. Whoever controls admissible process truth controls
downstream data, audit, governance, automation, and intelligence.

This is the upstream dam principle. The dam must be upstream.

Source: doctrine/BLUE_RIVER_DAM.md

---

## Conformance as Law Enforcement

A process model is a law. An event log is a court record. Conformance checking is the judge.

A conformance problem is not a "finding" or an "observation" or an "anomaly." It is a structural
defect in the process — either the process is not behaving as designed, or the design does not
reflect what the process actually does.

Accepting a conformance gap as "normal variance" without a named repair action is itself a
governance defect.

Source: doctrine/CONFORMANCE_AS_LAW.md — grounded in van der Aalst (2016) and Carmona, van Dongen,
Solti, Weidlich (2018).

---

## Object-Centric Is Not Optional

Classic process mining is trace-centric. Every conformance check, every discovery algorithm assumes
exactly one case identifier per process execution. This works for simple processes. It fails
structurally everywhere else — producing divergence (overcounting) and convergence (undercounting)
that cannot be corrected by data quality improvements. They are structural defects caused by forcing
object-centric data into a case-centric mold.

OCEL 2.0 (van der Aalst et al., 2023) resolves these defects by making event-to-object and
object-to-object relations first-class. This is not a feature extension. It is a different paradigm.

When a process has multiple object types, case-centric tools cannot represent it without structural
loss. That loss must be named, policy-governed, and reported — or the downstream claims are
distorted.

Source: doctrine/OBJECT_CENTRIC_SUPREMACY.md

---

## The Proof Gates Exist

The wall is being built. Proof gates exist. Claims are issued only when proof gates are satisfied.

An ALIVE verdict requires all gate criteria to be met. A PARTIAL verdict documents what is present
and what is absent. No verdict is issued on assumption or inference alone.

This manifesto is a PARTIAL artifact — it declares the category claim for full-lifecycle process
intelligence. The proof gates are the mechanism by which that claim is verified. The wall is
the record of verification.

---

## The Five Maturity Levels

Process maturity is the progressive removal of unresolved process uncertainty from execution.

| Level | Identity | What It Can Do |
|-------|----------|----------------|
| 1 | Records Activity | Says something happened |
| 2 | Structures Evidence | Types, witnesses, admits, refuses |
| 3 | Judges Evidence Claims | Applies named law violations at boundaries |
| 4 | Prepares Execution Authority | Hands grounded evidence to execution layer |
| 5 | Adjudicates Process Truth | Mines, conforms, replays, receipts, benchmarks |

Most systems operate at Level 1. Full-lifecycle process intelligence operates at Level 5.

Source: doctrine/PROCESS_INTELLIGENCE_DEFINED.md

---

## What Full-Lifecycle Means

A process is not finished until it has a closure receipt. The full lifecycle spans:

Design → Monitoring → Optimization → Simulation → Repair → Decommissioning

Each phase produces formal artifacts: process models, conformance scores, variant distributions,
simulation results, repair proposals, and final receipts. The receipt chain closes the loop.

No phase is complete without evidence. No evidence is admissible without a witness. No witness
is sufficient without a refusal capability. No refusal is complete without a named law.

Source: doctrine/PROCESS_INTELLIGENCE_DEFINED.md

---

## The Wall

The public canon of process intelligence is a wall — a sequence of stones, each grounded in an
upstream receipt, each naming the law it satisfies, each publishable because the proof gates behind
it are satisfied.

LinkedIn is the city gate. Posts are wall stones. The newsletter is Water Gate teaching. The
manifesto is the declaration of what the wall stands for.

The nations witness the wall — not because the wall is claimed, but because it is verified.

---

## Publication Target

This manifesto is intended for publication at a public URL (GitHub Pages or Notion) and linked
from all posts in the Process Intelligence Public Canon series.

Public URL: PENDING — to be recorded in PUBLICATION_REGISTRY.yaml upon deployment.
