# BOARD_PROJECTION — Turning Process Intelligence into Board-Admissible Claims

## The claim chain

Every board-admissible claim about a process must trace through a complete evidence chain:

```
Raw log
  → admitted evidence (Evidence<T, Admitted, W>)
    → execution result (conformance check, alignment, replay)
      → receipt (Receipt artifact)
        → board statement
```

A claim that cannot trace this chain is narration. A claim that traces this chain is
auditor-admissible, M&A-ready, and replayable.

The chain is not metaphorical. Each arrow is a typed transition in `wasm4pm-compat`:
- Raw log → admitted: `Admit::admit()` with a named witness.
- Admitted → execution result: `wasm4pm` conformance engine.
- Execution result → receipt: `Receipt` artifact production.
- Receipt → board statement: human-readable projection of the typed receipt.

## What an auditor demands

An auditor's questions are not about intentions. They are about artifacts:

| Auditor question | Required artifact | wasm4pm type |
|---|---|---|
| "Show me your process model." | Typed model in standard notation | `WfNetConst<SOUNDNESS>` or BPMN/POWL |
| "How do you know it's sound?" | Soundness receipt | `WfNetConst<Sound>` (type-level) |
| "Show me your event logs." | Admitted OCEL or XES log | `OcelLog` or `EventLog` (admitted) |
| "What is your fitness score?" | Conformance metric with bounds | `Metric<Fitness, NUM, DEN>` |
| "What changed and when?" | Repair receipt chain | `Receipt` per repair action |
| "Can you replay this claim?" | Replayable archive record | Archive artifact (wasm4pm) |
| "What did you refuse?" | Refusal log with named laws | `Refusal<R, W>` log |

An auditor who cannot get these artifacts has not completed a process intelligence audit.
A company that cannot produce these artifacts cannot make auditable process claims.

## What a CFO needs

A CFO's questions are about risk, cost, and performance — but they must trace to the same
artifact chain:

| CFO question | Process intelligence answer | Evidence chain |
|---|---|---|
| "What is our process efficiency?" | Throughput time distribution | Event log → performance metrics |
| "Where are our bottlenecks?" | Named bottleneck activities | Log → sojourn time annotation |
| "Are we compliant?" | Conformance score above threshold | Log → alignment → receipt |
| "What does non-compliance cost?" | Cost of deviations (alignment cost) | Alignment report |
| "How often do we self-repair?" | Repair frequency and outcome | Repair receipt chain |
| "Are our reported metrics real?" | Receipted metrics with provenance | Receipt chain |

The CFO does not need to understand alignment algorithms. The CFO needs to know: "this number
has a receipt; here is what would have to be false for this number to be wrong."

## What an M&A buyer requires

An M&A buyer performing process due diligence needs to verify claims in the seller's deck:

| Deck claim | Buyer verification | What breaks it |
|---|---|---|
| "Our processes are ISO-compliant." | Model + conformance report | No admitted log, no receipt |
| "Our cycle time is X days." | Event log throughput time | Log not admitted, timestamps unverified |
| "We have no process debt." | Variant analysis, repair log | Hidden variants, no repair receipts |
| "Our processes scale linearly." | Stochastic simulation results | No simulation artifacts |
| "We have full audit trails." | Receipt chain, archive record | Gaps in receipt chain |
| "Our processes are self-healing." | MAPE-K loop receipts | No Execute receipts |

The diligence gap: most companies present slide deck claims backed by anecdote. A buyer with
process intelligence discipline asks: "show me the receipt." If there is no receipt, the claim
is unverifiable and the risk premium increases accordingly.

## Projection rules

Converting a receipt chain into a board statement follows these rules:

1. **No orphan claims:** Every quantitative claim requires a receipt. "Our fitness is 0.91"
   requires a `Metric<Fitness, 91, 100>` receipt with a timestamp and a log reference.
2. **No silent aggregation:** If multiple process instances are aggregated into one number,
   the aggregation method must be stated and the constituent receipts must exist.
3. **No stale receipts:** A receipt older than the declared monitoring interval is stale.
   Stale receipts must be flagged, not presented as current.
4. **Named refusals are disclosures:** If the admission layer refused any events (malformed
   logs, missing object relations, etc.), those refusals are material facts, not internal errors.
5. **Repair actions are disclosed:** Every automated repair action is a process event.
   Undisclosed repairs that affected reported metrics are a governance finding.

## The projection artifact

The board projection is itself a typed artifact:
- It lists the claims being made.
- For each claim, it provides the receipt reference (artifact ID, timestamp, log hash).
- It states the monitoring interval (the currency of the receipts).
- It states the refusal count (how many events were refused and why).
- It states the repair count (how many automated repairs were executed in the period).

A board projection that cannot be generated from typed receipt artifacts is narration.
A board projection generated from receipt artifacts is a governance instrument.
