---
artifact: NEWSLETTER_ISSUE_001
series: Process Intelligence Foundry Newsletter
issue_number: 1
title: "What is conformance checking — and why it is not optional"
status: DRAFT
gate: Water Gate (secondary)
doctrine_source: doctrine/CONFORMANCE_AS_LAW.md
upstream_receipts:
  - construct8-market-physics/receipts/C8_MARKET_PHYSICS_ALIVE_002.yaml
platform_url: PENDING
issue_url: PENDING
draft_date: 2026-06-02
---

# Issue 001 — What is conformance checking, and why it is not optional

*Process Intelligence Foundry Newsletter — Issue 1*

---

## Opening

Most software teams know whether their code ran. Very few know whether a lawful process happened.

These are not the same thing.

Code running is an activity record. A lawful process happening is a conformance result.
The gap between them is where audit failures, governance defects, and compliance breaches live.

Conformance checking is the discipline that closes this gap. This issue teaches what it is,
how it works, and why treating it as optional is itself a governance defect.

---

## What Conformance Checking Is

Conformance checking compares an event log — what actually happened — against a formal process
model — what should have happened. It produces quantitative verdicts in two dimensions:

- **Fitness:** How much of what happened was permitted by the model?
- **Precision:** How much of what the model permits actually happened?

Both metrics are formally bounded in [0, 1]. Both are grounded in published process mining theory
(van der Aalst, 2016; Carmona, van Dongen, Solti, Weidlich, 2018).

This is not diagnostics. This is not observation. This is **law enforcement at the process level.**

A process model is a law. An event log is a court record. Conformance checking is the judge.

Source: doctrine/CONFORMANCE_AS_LAW.md

---

## Two Algorithms, One Purpose

There are two primary conformance checking algorithms. They produce different values and serve
different purposes:

### Token Replay Fitness

The algorithm fires each trace event-by-event through a Petri net model. It counts:
- Produced tokens (work done)
- Consumed tokens (work acknowledged)
- Missing tokens (work that had to be artificially created — a defect)
- Remaining tokens (work left unclosed — a defect)

A fitness of 1.0 means every trace replays perfectly. A fitness of 0.0 means no trace can
replay at all.

Token replay is polynomial in trace length — fast enough for real-time monitoring.

### Alignment-Based Conformance

The algorithm constructs a synchronous product net of the event log trace and the process model,
then finds the minimum-cost path through it using A* search. Each move is classified:

- **Synchronous move:** log and model agree — no cost
- **Move-on-log:** something happened that the model does not allow — fitness defect
- **Move-on-model:** the model requires something that did not happen — precision defect

Alignment-based conformance is NP-hard in general, but produces the exact optimal alignment —
the precise accounting of every deviation between execution and declaration.

Source: doctrine/CONFORMANCE_AS_LAW.md

---

## The Fitness-Precision Tradeoff

Fitness and precision are in fundamental tension. Every process model faces this tradeoff:

- **High fitness, low precision:** The model allows everything the log contains — and much more.
  The "flower model" (a single place connected to all transitions in any order) has fitness 1.0
  and precision near 0.0. It fits everything because it allows everything.

- **High precision, low fitness:** The model is tightly specified but doesn't cover all observed
  behavior. New cases will fail to replay.

A conformance checking system that hides this tradeoff in a single aggregate score is lying.
The F1 harmonic mean is a convenience — it must never replace the decomposition into the two
component metrics.

Source: doctrine/CONFORMANCE_AS_LAW.md

---

## The Defect Frame

The Van der Aalst Constitution (Chicago TDD doctrine) states:

> **Model-vs-log mismatch is not a discrepancy. It is a defect.**

A conformance problem is not a "finding" or an "observation" or an "anomaly." It is a structural
defect in the process — either the process is not behaving as designed, or the design does not
reflect what the process actually does. Either way, the gap between model and log requires a
response.

Accepting a conformance gap as "normal variance" without a named repair action is itself a
governance defect.

This is why conformance checking is not optional. A system that cannot detect model-log mismatch
cannot prove a lawful process happened. And a system that cannot prove a lawful process happened
cannot issue board-admissible audit claims.

---

## What This Means for AI

Van der Aalst (2025) establishes that generative, predictive, and prescriptive AI over business
processes is groundless without an underlying process intelligence layer.

An AI system that claims to optimize a business process but grounds its claims in unverified logs
is operating on activity records — not process evidence. Its claims are narration, not process
intelligence.

An AI system grounded in admitted evidence with lawful conformance results can trace every claim
to a receipt. Its claims are process intelligence.

The difference is not a feature. It is the presence or absence of the upstream dam.

Source: doctrine/OBJECT_CENTRIC_SUPREMACY.md — "No AI Without PI" (van der Aalst, 2025)

---

## Next Issue

Issue 002 will cover object-centric process mining: why XES is insufficient for multi-object
processes, what OCEL 2.0 resolves structurally, and what the divergence and convergence defects
mean for your event data today.

---

## Upstream Evidence

All claims in this issue are grounded in ALIVE-gated doctrine:
- doctrine/CONFORMANCE_AS_LAW.md — Water Gate doctrine, conformance law surface
- doctrine/OBJECT_CENTRIC_SUPREMACY.md — OCEL 2.0 supremacy doctrine
- doctrine/BLUE_RIVER_DAM.md — Upstream dam principle, five maturity levels

No claims appear in this newsletter that are not backed by an ALIVE-gated doctrine file or a
published process mining paper citation.

---

## About This Newsletter

**Process Intelligence Foundry Newsletter** teaches Water Gate doctrine — the formal, teachable
process mining content that underpins the claims in the Process Intelligence Public Canon LinkedIn
series.

Each issue teaches one doctrine topic. Each topic cites its source doctrine file. No issue is
published until the upstream doctrine is ALIVE-gated.

Platform URL: PENDING
To subscribe: PENDING
