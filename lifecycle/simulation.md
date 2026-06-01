# SIMULATION — Process Intelligence Lifecycle Phase

## Token replay as simulation

Token replay is the foundational simulation mechanism in process mining. Given a process model
(WF-net, Petri net, BPMN mapped to a Petri net) and an event log, token replay executes each
trace against the model:

1. Place a token on the source place.
2. For each event in the trace, fire the corresponding transition if enabled.
3. If the transition is not enabled, produce a missing token (non-conformance).
4. At trace end, check whether the sink place holds the token and all other places are empty.

Token replay yields:
- **Fitness:** Fraction of behavior in the log that the model can reproduce.
- **Consumed tokens / produced tokens / missing tokens / remaining tokens** — the four counters
  used in the token-based fitness formula.

Token replay is a conformance checking technique, not full simulation. It diagnoses where observed
behavior deviates from the model. It does not generate new traces.

## Stochastic simulation via StochasticPetriNet with timing distributions

Stochastic Petri nets extend WF-nets with:
- **Firing rates** (exponential, deterministic, or general distributions) on transitions.
- **Weights** for competing transitions at a choice point.

Stochastic simulation:
1. Start from the initial marking (token on source place).
2. At each step, sample which enabled transition fires based on weights and timing distributions.
3. Record the generated trace as a synthetic event log entry.
4. Repeat N times to produce a synthetic log.

Synthetic logs are used to:
- Stress-test conformance checkers against known-conforming and known-deviating traces.
- Estimate throughput time distributions.
- Identify bottleneck transitions (high sojourn time).
- Generate training data for prediction models.

## wasm4pm obligations: discovery yields model, simulation requires model + rates

The simulation phase depends on the design phase artifact (the process model) and adds rates:

```
Discovery (operation phase) ──produces──▶ Process Model
                                               │
                                               ▼
Rate Estimation (simulation phase) ──produces──▶ StochasticPetriNet
                                                       │
                                                       ▼
                                          Synthetic Log Generation
```

wasm4pm must provide:
- **Model intake:** Accept a `WfNetConst<SOUNDNESS>` or BPMN-derived net as simulation substrate.
- **Rate annotation:** Attach timing distributions to transitions (not in `wasm4pm-compat`,
  which is structure-only; rates are execution-layer concerns).
- **Trace generation:** Monte Carlo sampling from the stochastic model.
- **Synthetic log receipt:** A typed, receipted `EventLog` artifact produced by simulation,
  distinguishable from an observed log by its provenance witness.

## wasm4pm-compat surfaces relevant to simulation

| Type | Module | Simulation role |
|---|---|---|
| `WfNetConst<SOUNDNESS>` | `src/petri.rs` | Input model for token replay |
| `Evidence<T, Raw, W>` | `src/evidence.rs` | Carrier for synthetic trace before admission |
| `EventLog` | `src/eventlog.rs` | Output container for simulated traces |
| `Metric<KIND, NUM, DEN>` | `src/conformance.rs` | Fitness and precision metrics on replay |
| `LossReport<From, To, Items>` | `src/loss.rs` | Required if simulation truncates or simplifies |

## What does NOT belong here

- The stochastic engine (Monte Carlo sampling, exponential random variate generation) belongs in
  `wasm4pm`, not in `wasm4pm-compat`. This crate is structure-only.
- Rate parameters are not typed in `wasm4pm-compat`. They are execution-layer values.
- Conformance checking against the simulated log belongs in the operation phase.

The simulation phase closes when a stochastic model exists and synthetic logs are receipted as
simulation artifacts, distinguishable from observed operational logs.
