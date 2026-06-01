# Crosswalk: BPMN to WF-net Semantic Projection

## The Semantic Mapping

BPMN is a notation for human-readable process models. WF-nets (Workflow Petri
nets) are a formal execution model with token semantics. The projection from
BPMN to WF-net maps BPMN's notation to Petri net structure.

## Node Mapping

| BPMN element | WF-net element | Notes |
|---|---|---|
| `Start Event` | Source place (single token at process start) | One per process |
| `End Event` | Sink place (final marking) | One per process |
| `Intermediate Event` | Place between surrounding transitions | Catch/throw modeled as tau |
| `Task` | Transition (labeled) | Activity name becomes transition label |
| `XOR Gateway (split)` | Transition per branch (or place + silent transitions) | Non-deterministic choice |
| `XOR Gateway (join)` | Merge place | Single-token merge |
| `AND Gateway (split)` | Single transition producing tokens in parallel places | Parallel production |
| `AND Gateway (join)` | Single transition consuming from all parallel places | Synchronization barrier |
| `Inclusive (OR) Gateway` | Over-approximation or loss | OR-join has no exact WF-net counterpart |
| `Sequence Flow` | Arc (place→transition or transition→place) | Direction preserved |
| `Boundary Event` | Additional arc from task-place to handler transition | Approximate mapping |

## Source and Sink Places

Every WF-net has exactly one source place `p_i` (initial marking = {p_i})
and one sink place `p_o` (final marking = {p_o}). In the BPMN → WF-net
projection:

- All BPMN `Start Event` nodes collapse to `p_i`.
- All BPMN `End Event` nodes collapse to `p_o`.

If the BPMN process has multiple start events (a valid BPMN construct), they
are merged into `p_i` by introducing silent transitions. This is a structural
change but not a semantic loss for soundness analysis.

## What Is Lost

| Lost item | BPMN source | WF-net destination | Loss type |
|---|---|---|---|
| Human-readable task names | `BpmnTask { name: "Approve Order" }` | Transition label only | Label preserved; name trims to label |
| Annotations / documentation | `<bpmn:documentation>` | Absent | Hard drop |
| Data objects / data stores | `<bpmn:dataObjectReference>` | Absent | Hard drop |
| Message flows | Cross-pool `<bpmn:messageFlow>` | Absent (single-pool only) | Hard drop |
| Lane / pool structure | `<bpmn:laneSet>` | Absent | Hard drop |
| Gateway conditions | `conditionExpression` on sequence flows | Absent | Hard drop |
| Inclusive (OR) gateway | Three-way OR semantics | Over-approximated as XOR or AND | Semantic approximation |
| Event types (catching/throwing) | Timer, Message, Signal events | Silent tau transitions | Semantic approximation |
| Loop markers | `standardLoopCharacteristics` | Additional place + arc | Structural change |

The `BpmnGateway::Inclusive` case is the most significant semantic loss. An
OR-gateway allows one-or-more outgoing branches to fire. WF-nets have no direct
counterpart — any approximation (treat as XOR or AND) is semantically incorrect
for some BPMN processes. This loss must be named.

## wasm4pm-compat Enforcement

### Admission First

Before projection begins, the `BpmnProcess` must be admitted:

```rust
use wasm4pm_compat::bpmn::BpmnProcess;
use wasm4pm_compat::witness::Bpmn20;
use wasm4pm_compat::admission::Admit;

// Only after admission can projection proceed
let admission: Admission<BpmnProcess, Bpmn20> = Admit::admit(raw_bpmn)?;
```

### Loss Detection Before Policy

The projection must scan for OR-gateways, data objects, and multi-pool flows
before setting the loss policy:

```rust
let has_inclusive_gateways = process.nodes().any(|n| matches!(n.gateway(), Some(BpmnGateway::Inclusive)));
let has_data_objects = process.has_data_objects();

let policy = if has_inclusive_gateways || has_data_objects {
    LossPolicy::AllowLossWithReport
} else {
    LossPolicy::RefuseLoss
};
```

### Refusal for Unacceptable Loss

If the caller chose `RefuseLoss` and an OR-gateway is detected:

```rust
// Refusal type names the specific law
Refusal::<BpmnToWfNetRefusal, Bpmn20>::new(
    BpmnToWfNetRefusal::InclusiveGatewayNotRepresentable { gateway_id }
)
```

## PM4Py Comparison

PM4Py's `pm4py.convert.convert_to_petri_net(bpmn)` performs BPMN → Petri net:
- Silently handles inclusive gateways by approximation (typically as XOR).
- Drops data objects, annotations, and lane information without record.
- Returns `(PetriNet, Marking, Marking)` with no loss report.
- The caller has no way to know whether the returned Petri net is an exact or
  approximate representation of the BPMN.

## Summary

The BPMN → WF-net projection is lossy for BPMN processes with OR-gateways,
data objects, message flows, or multi-pool constructs. wasm4pm-compat names each
loss type with a specific refusal reason, requires an explicit `LossPolicy` before
the projection begins, and mandates a `LossReport` when loss is permitted.
PM4Py performs the projection silently with no structural loss record.
