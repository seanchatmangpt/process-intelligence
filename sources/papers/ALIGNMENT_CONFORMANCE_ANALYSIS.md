# Alignment-Based Conformance Checking — Deep Formal Analysis

**Analyst:** Dr. OCEL Specialist (AGI)
**Date:** 2026-05-31
**Source:** van der Aalst, Adriansyah, de Medeiros — "Replaying History on Process Models for Conformance Checking and Performance Analysis" (2012); Adriansyah PhD thesis (2014)

---

## Formal Objects

### AlignmentResult
- `trace_id: CaseId` — the trace being aligned
- `optimal_alignment: OptimalAlignment` — the cost-minimizing alignment
- `cost: MoveCost` — total cost of the optimal alignment
- `fitness: f64` — normalized fitness score in [0, 1]
- Derived per trace; aggregated over log to produce log-level fitness

### MoveCost
- `log_move_cost: u32` — cost of log moves (activity in trace, not in model)
- `model_move_cost: u32` — cost of model moves (activity in model, not in trace)
- `sync_move_cost: u32` — cost of synchronous moves (always 0 in standard cost function)
- Standard cost function: log_move = 1, model_move = 1, sync_move = 0
- Domain-specific cost functions can assign higher penalties to critical deviations

### OptimalAlignment
- Sequence of `AlignmentMove` entries covering the full trace
- Total moves = |trace| + |model path| - |synchronous moves|

### AlignmentMove (discriminated union)
- `SynchronousMove(activity)` — trace and model agree; cost 0
- `LogMove(activity)` — activity in trace, no matching model transition; cost = log_move_cost
- `ModelMove(transition)` — model fires a transition not in trace; cost = model_move_cost
- `SilentMove(transition)` — model fires a silent (τ) transition; cost 0

### SynchronousMove
- The "happy path" move: trace token and model transition aligned
- Counting synchronous moves as fraction of total gives intuitive fitness

---

## Key Insight: Optimal Alignment via Cost Minimization

Token replay answers "can the model reproduce this trace?" — it is a reachability question.

Alignment answers "what is the **minimum edit distance** between this trace and any accepting run of the model?" — it is an optimization problem solved via A* over the synchronous product of trace automaton and Petri net reachability graph.

**Consequence:** Alignment-based fitness is strictly more informative than token replay fitness:
- Token replay can produce false positives (missing tokens repaired silently)
- Alignment makes all deviations explicit and costed
- Alignment identifies whether a deviation is a log move (data quality issue) or model move (model incompleteness)

**Soundness requirement:** Alignment requires the model to be a **sound WF-net**. Unsound nets produce infinite reachability graphs and alignment does not terminate.

---

## wasm4pm Coverage Assessment

### Present: Token Replay
wasm4pm implements token replay conformance. Token replay:
- Fires transitions following the trace
- Counts missing tokens (consumed but not available) and remaining tokens (not consumed)
- Produces a fitness metric but not an alignment

Token replay fitness formula: `fitness = 0.5 * (1 - m/c) + 0.5 * (1 - r/p)` where m=missing, c=consumed, r=remaining, p=produced.

### Missing: Alignment-Based Conformance
**Status: MISSING from wasm4pm.**

Alignment-based conformance requires:
1. Construction of synchronous product net (trace automaton × Petri net)
2. A* search with move cost function over the product net state space
3. Extraction of optimal alignment from the A* solution path
4. Normalization of cost to fitness in [0, 1]

Current wasm4pm has no implementation of any of these steps.

---

## wasm4pm-compat Coverage

| Alignment Concept | wasm4pm-compat Module | Coverage |
|---|---|---|
| Fitness metric shape | `src/conformance.rs` — `Metric<FITNESS, NUM, DEN>` with `Between01` | Full |
| WF-Net input shape | `src/petri.rs` — `WfNetConst<SOUNDNESS>` | Full |
| Receipt for conformance result | `src/receipt.rs` | Full |
| AlignmentResult shape | Not present — needs addition | None |
| MoveCost shape | Not present | None |
| OptimalAlignment sequence | Not present | None |

wasm4pm-compat must add `AlignmentResult`, `MoveCost`, and `OptimalAlignment` as typed output shapes in a new module or extension of `src/conformance.rs`.

---

## PM4Py Coverage Assessment

| Capability | PM4Py Module | Maturity |
|---|---|---|
| Alignment-based conformance | `pm4py.conformance.alignments` | Mature — A* with configurable cost |
| Token replay | `pm4py.conformance.tokenreplay` | Mature |
| Alignment diagnostics | Move-level deviation report | Full |
| Performance overlay | Alignment extended with timestamps | Available |
| Object-centric alignment | Not present — requires flattening | None |

PM4Py's alignment implementation is the reference. The critical gap is that PM4Py requires flattening OCEL to XES before alignment, losing cross-object causality.

---

## Gap Action

wasm4pm must implement alignment-based conformance:
- Input: `Admitted<OcelLog, Ocel20>` + `WfNetConst<Sound>` per object type
- Algorithm: A* over synchronous product net per object type
- Output: `AlignmentResult` typed with object type and witness
- Aggregation: per-object-type fitness → `Metric<FITNESS, NUM, DEN>`
- Receipt: typed `Receipt<AlignmentResult, Ocel20>` for board claim attestation

The receipt chain for "Invoice processing conforms 94.2% to process model" must trace to an `AlignmentResult`, not a token replay result, to be formally attested.

---

## Action Items

| Priority | Action | Owner |
|---|---|---|
| P0 | Add `AlignmentResult`, `MoveCost`, `OptimalAlignment` shapes to wasm4pm-compat | wasm4pm-compat |
| P0 | Implement A*-based alignment in wasm4pm consuming `WfNetConst<Sound>` | wasm4pm |
| P0 | Thread `Ocel20` witness through alignment result to receipt | wasm4pm |
| P1 | Implement configurable cost function (domain-specific move costs) | wasm4pm |
| P1 | Add object-centric alignment (per-object-type, not flattened) | wasm4pm |
