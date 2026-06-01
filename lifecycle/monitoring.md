# MONITORING — Process Intelligence Lifecycle Phase

## Concept drift detection

Concept drift occurs when the underlying process changes over time, causing the discovered model
to become an inaccurate description of current behavior. Types of drift:

- **Sudden drift:** The process changes abruptly at a point in time (e.g., a new regulation,
  a system migration). Conformance fitness drops sharply.
- **Gradual drift:** New behavior is introduced alongside old behavior; the mix shifts over time.
  Variant frequency distributions change; fitness degrades slowly.
- **Recurring drift:** Seasonal or cyclical variation causes process behavior to shift and return.
- **Incremental drift:** Small cumulative changes accumulate into a significantly different process.

Detection methods:
- **Windowed conformance:** Compute fitness and precision on rolling time windows. A sustained
  drop below threshold signals drift.
- **Variant distribution shift:** Track the frequency distribution of process variants over time.
  A Chi-squared or KL-divergence test on variant frequencies detects distributional change.
- **Model-log gap:** Rediscover the process from recent logs and compare the discovered model
  against the baseline model. Structural differences indicate drift.

## Performance perspective: bottleneck detection

Beyond conformance (does the process follow the model?), the performance perspective asks:
how efficiently does the process execute?

- **Sojourn time:** Time a case spends in a state (waiting + service). High sojourn time at
  a specific activity indicates a bottleneck.
- **Waiting time:** Time between completion of one activity and start of the next.
- **Service time:** Duration of the activity itself.
- **Throughput time:** End-to-end case duration. Distribution over cases reveals outliers.

Bottleneck detection:
1. Annotate the process model with mean sojourn times per activity (from the event log).
2. Identify activities with sojourn times disproportionately high relative to the model average.
3. Distinguish between waiting-time bottlenecks (resource contention) and service-time
   bottlenecks (activity complexity).

## Object-centric monitoring: multiple objects interacting simultaneously

Standard process mining assumes one case per trace. Object-centric monitoring (OCEL-based)
handles the reality that multiple objects interact in a single process instance:

- An order involves an order object, multiple item objects, a customer object, a delivery object.
- Events relate to multiple objects simultaneously via E2O (event-to-object) links.
- Object lifecycles may run in parallel and interact: an item cannot ship before its order is
  confirmed.

Object-centric monitoring:
- **Per-object conformance:** Check each object type's lifecycle against its own model.
- **Cross-object causality:** Verify that inter-object dependencies are respected (e.g., item
  shipment event requires a prior order confirmation event for the same order object).
- **Convergence detection:** Multiple objects merging into one event (e.g., a payment covering
  multiple invoices). Convergence creates O2O (object-to-object) links.
- **Divergence detection:** One event affecting multiple downstream objects. Divergence must
  be named and typed — `EventObjectLink::new(…).qualified(…)` in `wasm4pm-compat`.

## wasm4pm-compat surfaces relevant to monitoring

| Type | Module | Monitoring role |
|---|---|---|
| `OcelLog` | `src/ocel.rs` | Object-centric log for multi-object monitoring |
| `EventObjectLink` | `src/ocel.rs` | E2O link typed with qualifier |
| `ObjectObjectLink` | `src/ocel.rs` | O2O link for convergence/divergence tracking |
| `Metric<KIND, NUM, DEN>` | `src/conformance.rs` | Windowed fitness/precision metrics |
| `DiagnosticReport` | `src/diagnostic.rs` | Typed report of monitoring findings |

## wasm4pm obligations for full monitoring phase

- Drift detection engine: windowed conformance, variant distribution tracking.
- Performance annotation: sojourn/waiting/service time computation on event logs.
- Object-centric monitoring: per-object lifecycle replay, cross-object causality checking.
- Monitoring receipts: each monitoring cycle produces a receipt with the observed metrics
  and the timestamp of the check.