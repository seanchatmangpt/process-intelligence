# REPAIR — Process Intelligence Lifecycle Phase

## Triggering repair: conformance drops below threshold

Repair is triggered when monitoring detects a sustained conformance violation:

- **Fitness threshold breach:** Token replay fitness on the rolling window drops below the
  declared threshold (e.g., fitness < 0.85 for three consecutive monitoring cycles).
- **Precision threshold breach:** The model over-generalizes relative to observed behavior —
  many traces use only a subset of the modeled paths.
- **Named law violation:** A named refusal law fires repeatedly (e.g., `MissingFinalMarking`,
  `DanglingEventObjectLink`) across multiple cases.
- **Variant explosion:** The number of distinct trace variants exceeds the threshold set in
  the repair policy, indicating uncontrolled process drift.

A single anomaly is not sufficient to trigger repair. The monitoring phase accumulates evidence;
the repair trigger is a typed threshold condition, not a human judgment.

## Repair must be logged as events

The repair action is itself a process event and must be admitted as evidence:

1. **Repair intent event:** Logged at the moment the MAPE-K Plan phase produces a repair plan.
   Contains: plan ID, trigger condition, proposed actions, risk assessment.
2. **Repair execution event:** Logged at the moment the MAPE-K Execute phase actuates each
   repair action. Contains: action type, target (model, process instance, resource allocation),
   timestamp, executor identity.
3. **Repair outcome event:** Logged after the next monitoring cycle confirms whether conformance
   improved. Contains: post-repair fitness, precision, verdict (resolved / partially resolved /
   escalated).

All three events are admitted through the standard admission path:
`Evidence<T, Raw, W>` → `Admit::admit()` → `Evidence<T, Admitted, W>`.
A repair event that bypasses admission is not admissible evidence of repair.

## Repair action taxonomy

| Action type | When triggered | What it does |
|---|---|---|
| Model update | Discovered model diverges from baseline | Re-discover process from recent log; update the reference model |
| Resource reallocation | Bottleneck at specific activity | Increase capacity assigned to bottleneck activity |
| Process instance intervention | Individual case is stuck | Inject a compensating event (with documented justification) |
| Constraint relaxation | Overly strict model rejecting valid behavior | Loosen a Declare constraint or WF-net arc |
| Constraint tightening | Model too permissive (precision drop) | Add a constraint to exclude spurious behavior |
| Escalation | Automated repair cannot restore conformance | Human review required; process flagged as PARTIAL |

## MAPE-K Execute phase: actuating repair

The Execute phase is the MAPE-K component responsible for translating a repair plan into
operational reality:

- **Execute reads the Plan:** An ordered list of repair actions with typed parameters.
- **Execute actuates each action:** Calls the appropriate repair operator (model update,
  resource reallocation, etc.).
- **Execute observes effects:** Captures the immediate output of each action (success,
  partial, failed).
- **Execute emits receipts:** Each action produces a repair receipt — a typed artifact
  proving that this specific action was taken at this time for this reason.

The Execute phase does not decide what to repair. That is the Plan phase. The Execute phase
does not analyze whether repair is needed. That is the Analyze phase. The separation is strict
and typed: crossing phase boundaries requires a typed handoff artifact.

## wasm4pm-compat surfaces relevant to repair

| Type | Module | Repair role |
|---|---|---|
| `Evidence<T, Raw, W>` | `src/evidence.rs` | Carrier for repair events before admission |
| `Admit::admit()` | `src/admission.rs` | Mandatory admission path for repair events |
| `Refusal<R, W>` | `src/admission.rs` | Named refusal if repair event is malformed |
| `Receipt` | `src/receipt.rs` | Typed proof that a repair action was executed |
| `DiagnosticReport` | `src/diagnostic.rs` | Structured report of violation that triggered repair |

## wasm4pm obligations for full repair phase

- Threshold evaluation engine: compare monitoring metrics against declared thresholds.
- Repair plan generator: map violation types to repair action types.
- Actuation layer: execute repair actions against live process instances.
- Repair receipt emission: produce receipted evidence of every repair action taken.
- Escalation path: typed escalation when automated repair cannot restore conformance.