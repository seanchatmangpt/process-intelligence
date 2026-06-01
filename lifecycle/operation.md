# OPERATION — Process Intelligence Lifecycle Phase

## Streaming event logs: EventStream in wasm4pm-compat

The operation phase is where the process runs and evidence accumulates in real time. Evidence
arrives as a stream of events, not as a complete, pre-collected log.

`EventStream` (in `wasm4pm-compat`) is the append-only streaming buffer:
- Events are appended via `EventStream::append(event)`.
- The stream is monotonic: appended events cannot be retracted.
- The stream can be snapshotted into an `EventLog` for conformance analysis.
- A snapshot is a point-in-time view; the stream continues after snapshotting.

The append-only discipline enforces the Blue River Dam doctrine at the operational layer:
an event that entered the stream is evidence of what happened. Retracting it would corrupt the
process record.

## Online conformance: checking traces as they arrive

Online conformance checking evaluates conformance incrementally as events arrive, rather than
waiting for a complete log:

1. **Prefix replay:** For each new event, extend the current replay state. The replay state is
   the set of markings reachable given the observed prefix.
2. **Alarm on deviation:** If a new event cannot fire in any reachable marking, emit a
   conformance alarm immediately — not at trace end.
3. **Incremental fitness:** Maintain running counts of consumed, produced, missing, and remaining
   tokens so fitness can be computed at any point in time.

Online conformance is the operational form of the token replay technique defined in the
simulation phase. The model is fixed (from the design phase); the log is the operational stream.

## MAPE-K loop: Monitor → Analyze → Plan → Execute

The MAPE-K autonomic control loop applied to process intelligence:

### Monitor
- Consume the `EventStream`.
- Compute conformance metrics on the current window.
- Detect anomalies: missing token events, unexpected activity occurrences, timing violations.
- Emit `Observation` records: typed evidence of what the monitor detected.

### Analyze
- Take `Observation` records as input.
- Match patterns: high missing-token rate, variant explosion, cycle detection, bottleneck.
- Identify root cause using the process model as reference.
- Produce `Analysis` records: typed evidence of what the analysis concluded, with confidence.

### Plan
- Take `Analysis` records as input.
- Consult repair policy (from the repair phase).
- Produce `Plan` records: an ordered sequence of repair actions, with risk assessment.
- Plans above a risk threshold require explicit authorization.

### Execute
- Take `Plan` records as input.
- Actuate the repair actions (see REPAIR phase for the full repair lifecycle).
- Record execution outcomes as events in the stream — repair is itself a process event.
- Feed execution results back to the Knowledge component.

### Knowledge
- The discovered process model (from the design and simulation phases).
- Historical conformance metrics and their timestamps.
- Known patterns and their successful repair actions.
- Predictive models trained on operational history.

## wasm4pm-compat surfaces relevant to operation

| Type | Module | Operation role |
|---|---|---|
| `EventStream` | `src/eventlog.rs` | Append-only operational stream |
| `Evidence<T, Raw, W>` | `src/evidence.rs` | Carrier for incoming events pre-admission |
| `Admission<T, W>` | `src/admission.rs` | Admits events that conform to law |
| `Refusal<R, W>` | `src/admission.rs` | Refuses events that violate named laws |
| `OcelLog` | `src/ocel.rs` | Object-centric operational log |
| `EventObjectLink` | `src/ocel.rs` | E2O relationship in operational stream |

## wasm4pm obligations for full operation phase

- Online conformance engine: incremental token replay against a streaming log.
- Alarm generation: typed conformance alarms with named violation laws.
- MAPE-K loop closure: Monitor/Analyze/Plan/Execute as first-class execution stages.
- Receipt emission: each MAPE-K cycle produces a receipt proving the loop executed.

The operation phase never closes — it runs for the lifetime of the process. It closes only at
decommissioning.