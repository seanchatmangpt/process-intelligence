# MAPE_K_MAP — Mapping the Autonomic Loop to Process Intelligence

## Overview

MAPE-K (Monitor, Analyze, Plan, Execute, Knowledge) is the IBM autonomic computing reference
architecture for self-managing systems. This document maps each MAPE-K component to the
specific wasm4pm capabilities required for full loop closure.

## Monitor: wasm4pm streaming conformance, event stream intake

| MAPE-K function | wasm4pm capability | wasm4pm-compat type |
|---|---|---|
| Consume event stream | `EventStream` intake | `EventStream` (`src/eventlog.rs`) |
| Compute conformance on window | Online token replay | `Metric<KIND, NUM, DEN>` |
| Detect anomalies | Named violation detection | `Refusal<R, W>` with named law |
| Emit observations | Typed observation records | `Evidence<T, Raw, W>` pre-admission |
| Timestamp observations | Nanosecond-precision events | `Event::at_ns(ns)` |

The Monitor component does not interpret observations. It collects and structures them.
Every observation is a typed artifact: uninterpreted, timestamped, admitted.

## Analyze: conformance analysis, alignment, variant analysis

| MAPE-K function | wasm4pm capability | wasm4pm-compat type |
|---|---|---|
| Conformance analysis | Token replay fitness computation | `Metric<Fitness, NUM, DEN>` |
| Alignment | Cost-optimal alignment (wasm4pm) | Alignment score artifact |
| Variant analysis | Variant enumeration, frequency | Log-to-model comparison |
| Root cause identification | Named law violation mapping | `DiagnosticReport` |
| Confidence scoring | Metric bounds enforcement | `Between01<NUM, DEN>` |
| Concept drift detection | Windowed model comparison | Rolling `Metric` computation |

The Analyze component does not decide what to do. It produces `Analysis` records — typed
artifacts with a diagnosis, a confidence score, and candidate repair actions.

## Plan: repair actions for violation types

| MAPE-K function | wasm4pm capability | Notes |
|---|---|---|
| Policy lookup | Repair policy store | Maps violation type → action type |
| Action selection | Repair action taxonomy | Model update, resource realloc, escalation |
| Risk assessment | Action risk scoring | High-risk actions require authorization |
| Plan sequencing | Ordered action list | Actions have typed preconditions |
| Authorization gate | Threshold check | Plans above risk threshold are held |

The Plan component does not execute. It produces `Plan` records — ordered, typed, risk-scored
action sequences. A Plan that was never executed is still an evidence artifact.

## Execute: actuation of repair

| MAPE-K function | wasm4pm capability | wasm4pm-compat type |
|---|---|---|
| Model update | Re-discovery + model replacement | New `WfNetConst<SOUNDNESS>` artifact |
| Resource reallocation | Capacity adjustment actuation | External to process mining |
| Event injection | Compensating event with documented reason | `Evidence<T, Raw, W>` → admission |
| Constraint change | Model mutation with receipt | New typed model artifact |
| Escalation | Typed escalation record | `Refusal<Escalation, W>` |
| Receipt emission | Proof of action | `Receipt` per executed action |

The Execute component does not plan. Each action produces a receipt. A receipt-less execution
is not an execution for process intelligence purposes — it is an unwitnessed intervention.

## Knowledge: the discovered process model

The Knowledge component is the persistent store that makes MAPE-K a learning loop, not just
a reactive loop:

| Knowledge element | wasm4pm representation | Lifecycle |
|---|---|---|
| Reference process model | `WfNetConst<SOUNDNESS>` or BPMN/POWL artifact | Updated at repair |
| Historical conformance metrics | Time-series of `Metric` values | Accumulates over operation |
| Violation patterns | Named law → frequency → repair outcome mapping | Updated at repair close |
| Successful repair actions | Action type → outcome → confidence | Updated at repair close |
| Predictive models | Trained on historical data | Graduated to `wasm4pm` |

The Knowledge component is the only MAPE-K component that persists across loop cycles.
All other components are stateless within a cycle; their state is carried by typed artifacts.

## What wasm4pm must provide for full MAPE-K loop closure

The following capabilities are required in `wasm4pm` (execution layer) and are not present
in `wasm4pm-compat` (structure layer):

| Capability | Why required | Status |
|---|---|---|
| Online token replay engine | Monitor: conformance on streaming events | wasm4pm |
| Alignment computation | Analyze: cost-optimal alignment | wasm4pm |
| Variant enumeration | Analyze: behavioral fingerprint | wasm4pm |
| Repair plan evaluator | Plan: policy-to-action mapping | wasm4pm |
| Actuation layer | Execute: repair action invocation | wasm4pm |
| Receipt emitter | Execute: per-action proof | wasm4pm |
| Model store | Knowledge: version-controlled model artifact | wasm4pm |
| Metric time-series store | Knowledge: historical conformance | wasm4pm |

`wasm4pm-compat` provides the types. `wasm4pm` provides the engine. Neither substitutes for
the other. A crate that conflates the two is neither a doorway nor a throne room — it is a
structural defect.

## Loop closure criterion

The MAPE-K loop is closed when:
1. Every Monitor observation is a typed, admitted artifact.
2. Every Analyze conclusion is a typed artifact with a confidence score bounded by `Between01`.
3. Every Plan is a typed, ordered, risk-scored action sequence.
4. Every Execute action produces a receipt.
5. The Knowledge component can replay any past loop cycle from its typed artifact store.

Without condition 5, the system is reactive. With condition 5, it is autonomic.
