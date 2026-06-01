# BPMN 2.0 — Business Process Model and Notation

**Authority:** OMG BPMN 2.0 specification (ISO/IEC 19510:2013)
**Witness key:** `bpmn-2.0` — `WitnessFamily::Standard`

---

## Standard Overview

BPMN 2.0 is the OMG/ISO graphical process-modeling language. It defines a graph of flow
nodes connected by sequence flows, with semantics for how tokens move through the graph.

### Core Structural Objects

| BPMN 2.0 object | Description |
|---|---|
| **FlowNode** | The abstract supertype: a node in the process graph. Subtypes: Task, Gateway, Event. |
| **Task** | A unit of atomic work (activity). May be typed: Service, User, Script, Send, Receive, Manual, BusinessRule, Call. |
| **Gateway** | A flow divergence or convergence point. Kinds: Exclusive (XOR), Parallel (AND), Inclusive (OR), EventBased, Complex. |
| **Event** | A start, intermediate, or end node that triggers or responds to something. |
| **SubProcess** | A collapsed or expanded block of nested flow nodes. |
| **SequenceFlow** | A directed edge from one FlowNode to another, optionally bearing a condition expression. |

### Relationship to WF-net Semantics

BPMN 2.0 and WF-nets (workflow nets) are semantically related:

- An **Exclusive Gateway** (XOR split/join) maps to a WF-net XOR choice place.
- A **Parallel Gateway** (AND split/join) maps to a WF-net AND-split/AND-join transition pair.
- A **Task** maps to a WF-net transition.
- **Start** and **End events** map to the WF-net source and sink places.

Soundness in BPMN is therefore derived from WF-net soundness: a BPMN model is sound if
and only if its equivalent WF-net is sound (option to complete, proper completion, no dead
transitions). This equivalence is a theorem, not a structural guarantee — it requires
token-game analysis, which graduates to `wasm4pm`.

---

## wasm4pm-compat Implementation (bpmn.rs)

`src/bpmn.rs` models the BPMN **graph shape** — structure only, no token semantics:

- `BpmnProcess` — a set of `BpmnNode`s and `BpmnEdge`s (sequence flows).
- `BpmnNode` — an enum discriminating `Task(BpmnTask)`, `Gateway(BpmnGatewayNode)`,
  `Event(BpmnEventNode)`.
- `BpmnTask` — holds the task `name` and optional `task_type`.
- `BpmnGateway` enum — `Exclusive`, `Parallel`, `Inclusive`, `EventBased`, `Complex`.
- `BpmnEvent` enum — `Start`, `Intermediate`, `End`, `Boundary`.
- `BpmnEdge` — a directed sequence flow from source node ID to target node ID, with an
  optional condition label.
- `BpmnProcess::validate()` — checks graph laws: nodes are identified, edges connect
  declared nodes, at least one start and one end exist. Does **not** execute tokens,
  simulate, or convert BPMN to Petri nets.

---

## What wasm4pm Must Provide

| Capability | Graduates to |
|---|---|
| BPMN ↔ WF-net/Petri-net translation | `wasm4pm` |
| Token-game simulation | `wasm4pm` |
| Soundness analysis (via WF-net equivalence) | `wasm4pm` |
| Conformance checking of a log against a BPMN model | `wasm4pm` |
| BPMN 2.0 XML import/export | `wasm4pm` (import/export engine) |
| Sub-process expansion and flattening | `wasm4pm` |

---

## Board Placement

BPMN 2.0 is the universal language for communicating process models to business
stakeholders. Every process discovery result — Inductive Miner, POWL, DFG — can be
projected into BPMN for board-level presentation. The wasm4pm-compat crate defines the
structural shape of every BPMN 2.0 node and edge type, ensuring that the discovery
graduation path (log → admitted BPMN shape → wasm4pm engine → soundness analysis) is
type-safe and traceable.
