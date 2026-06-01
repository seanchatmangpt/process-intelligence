# Conformance as the Process Law Enforcement Surface

> "A conformance problem is a defect, not a discrepancy." — van der Aalst

---

## What Conformance Checking Is

Conformance checking compares an event log (what actually happened) against a formal process
model (what should have happened). It produces quantitative verdicts: fitness and precision,
each formally bounded in [0, 1].

This is not diagnostics. This is not observation. This is **law enforcement at the process
level**. A process model is a law. An event log is a court record. Conformance checking is
the judge.

---

## Token Replay Fitness

**Paper:** Berti, van der Aalst (2019). Token-based Replay.
**Paper:** Rozinat, van der Aalst (2008). Original token replay formulation.

**Algorithm:** Fire each trace event by event through the Petri net model. Count tokens:
- Produced tokens: tokens put on places during firing
- Consumed tokens: tokens removed during firing
- Missing tokens: tokens that had to be artificially created to enable a transition
- Remaining tokens: tokens left in the net after the trace ends

**Formula:**
```
fitness = 0.5 * (1 - missing/produced) + 0.5 * (1 - remaining/consumed)
```

**Range:** [0, 1]. A fitness of 1.0 means every trace replays perfectly. A fitness of 0.0
means no trace can replay at all.

**Type law in wasm4pm-compat:**
`Metric<FITNESS, NUM, DEN>` with `Between01<NUM, DEN>` const-generic bound. The bound
`Require<{ NUM <= DEN }>: IsTrue` is enforced at compile time. A fitness value that escapes
[0,1] is a type error, not a runtime exception. The `metric_out_of_bounds` compile-fail
fixture in `tests/ui/compile_fail/` seals this law.

**wasm4pm owns:** Token firing execution (fire transitions, count missing/remaining tokens,
apply formula). The firing traverses data; it must not be in compat.

---

## Alignment-Based Conformance

**Papers:** Adriansyah, van Dongen, Munoz-Gama, Carmona (2011). Aligning Observed and
Modeled Behavior.
**Book:** Carmona, van Dongen, Solti, Weidlich (2018). Conformance Checking.

**Algorithm:** Construct a synchronous product net of the event log trace and the process
model. Find the minimum-cost path through the product net using A* search. Moves are:
- Synchronous move: log event and model transition fire together (cost 0)
- Move-on-log: log event fires, no corresponding model transition (fitness defect)
- Move-on-model: model transition fires, no corresponding log event (precision defect)

**Metrics:**
- **Fitness** (alignment-based): fraction of synchronous moves vs total moves
- **Precision** (alignment-based): measures how much of the model's allowed behavior is
  actually seen in the log

**Type law in wasm4pm-compat:**
- `Metric<FITNESS, NUM, DEN>` — alignment fitness shape
- `Metric<PRECISION, NUM, DEN>` — alignment precision shape
- `Metric<F1, NUM, DEN>` — harmonic mean shape
- `Between01<NUM, DEN>` bounds all three
- `AlignmentResult` — shape of alignment output (move sequences, cost values)

**wasm4pm owns:** Synchronous product net construction; A* cost search; move sequence
computation; cost function evaluation. These are all data-traversal executions.

---

## The Fitness-Precision Tradeoff

Fitness and precision are in fundamental tension. Every process model faces this tradeoff:

**High fitness, low precision:** The model allows everything the log contains, but also
allows much more. The "flower model" (a single place connected to all transitions in any
order) has fitness 1.0 and precision near 0.0. It fits everything because it allows
everything.

**High precision, low fitness:** The model is tightly specified but doesn't cover all
observed behavior. Overfitting to the training log. New cases will fail to replay.

**Both high:** Generalization suffers — the model may be overfitted to the specific log
rather than to the underlying process class.

**Balance is the goal.** A conformance checking system that hides this tradeoff in a single
aggregate score is lying. The `F1Const` metric (harmonic mean of fitness and precision) is a
convenience — it must never replace the decomposition into the two component metrics.

---

## Replay Fitness vs. Alignment Fitness

These are different algorithms producing different fitness values:

| Dimension | Token Replay | Alignment-Based |
|---|---|---|
| Computational complexity | Polynomial in trace length | NP-hard in general |
| Approximation | Exact for fitness; imprecise for precision | Exact optimal alignment |
| Handles loops | Yes (with artificial tokens) | Yes (via A* search) |
| Precision measurement | Requires separate ETC pass | Integrated in alignment |
| Use case | Fast monitoring | Detailed audit |
| PM4Py | `conformance_diagnostics_token_based_replay` | `conformance_diagnostics_alignments` |

Both are owned by wasm4pm (execution). Both produce shapes owned by compat (`Metric` types).

---

## Repo Ownership Summary

| Conformance concern | wasm4pm-compat owns | wasm4pm owns |
|---|---|---|
| Fitness shape | `Metric<FITNESS, NUM, DEN>` with `Between01` | Token replay execution; alignment computation |
| Precision shape | `Metric<PRECISION, NUM, DEN>` with `Between01` | ETC precision computation; alignment precision |
| F1 shape | `Metric<F1, NUM, DEN>` with `Between01` | F1 formula evaluation |
| Alignment result shape | `AlignmentResult` struct with move-sequence fields | A* cost search; product net construction |
| Violation record | `ConformanceViolation` with named law | Violation detection logic |
| Log skeleton constraints | `DeclareConstraint` variants | Log skeleton derivation from event log |
| DECLARE checking | `DeclareConstraint` + `DeclareWitness` | LTL formula evaluation per trace |

---

## The Defect Frame

The Chicago TDD doctrine (Van der Aalst Constitution in this codebase) states:

> **Model-vs-log mismatch is not a discrepancy. It is a defect.**

A conformance problem is not a "finding" or an "observation" or an "anomaly." It is a
structural defect in the process — either the process is not behaving as designed, or the
design does not reflect what the process actually does. Either way, the gap between model
and log is a defect that requires a response.

Accepting a conformance gap as "normal variance" without a named repair action is itself a
governance defect.

In wasm4pm-compat: `ConformanceViolation` carries a named law. `StrictViolation` (in the
`strict` feature) carries `StrictViolation::law()` returning a `&'static str` human-readable
law name. There is no anonymous violation type. Every violation must name the law it violated.
