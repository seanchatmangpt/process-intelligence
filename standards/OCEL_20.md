# OCEL 2.0 — Object-Centric Event Log Standard

**Authority:** OCEL 2.0 specification (Ghahfarokhi et al., 2023)
**Witness key:** `ocel-2.0` — `WitnessFamily::Standard`
**Board claim:** "Our process intelligence is built on OCEL 2.0 — the IEEE-standard for object-centric process logs."

---

## Formal Objects

OCEL 2.0 drops the single-case-notion assumption of classical process mining. An event
may relate to many objects of many types. Objects relate to each other and change over
time. The six canonical objects of OCEL 2.0 are:

| OCEL 2.0 object | Description |
|---|---|
| **OcelEvent** | A timestamped activity instance that occurred; references zero or more objects via E2O links. |
| **ObjectInstance** | A typed process object (e.g. order, item, customer) with a typed attribute map and a change history. |
| **EventObjectLink (E2O)** | A directed, optionally qualified link from an event to an object, recording the object's role in the event. |
| **ObjectObjectLink (O2O)** | A directed, optionally qualified link between two objects, recording inter-object relationships. |
| **ObjectAttributeChange** | A timestamped record of a value change on one attribute of one object. |
| **OcelLog** | The container: indexed events, objects, E2O links, O2O links, and change records. |

---

## wasm4pm Implementation

The implementation in [ocel.rs](file:///Users/sac/process-intelligence/sources/wasm4pm/src/ocel.rs) implements OCEL 2.0 as a **first-class** structural canon — not "event log plus side tables":

- `OcelLog` holds `events: Vec<OcelEvent>`, `objects: Vec<OcelObject>`, `e2o_links: Vec<EventObjectLink>`, `o2o_links: Vec<ObjectObjectLink>`, `object_changes: Vec<ObjectChange>`.
- `OcelEvent::new(id, activity).at_ns(ts)` — full builder chain.
- `EventObjectLink::new(event_id, object_id).qualified(role)` — optional qualifier.
- `ObjectObjectLink::new(source_id, target_id).qualified(relationship)` — optional qualifier.
- `ObjectChange::new(object_id, attr_key, value).at_ns(ts)` — timestamped attribute change.
- `OcelAttributeValue` is typed: String, Integer, Float, Boolean, TimestampNs, List, Map.
- `OcelLog::validate()` performs structural integrity: all links reference declared events and objects; IDs are unique. It does **not** discover OC-Petri-nets or check conformance.

### The Flattening Trap

Flattening OCEL to a single case notion is lossy by construction — it drops convergence and divergence information. `OcelRefusal::FlatteningLoss` names this law. Any flattening projection must carry a `LossPolicy` and emit a `LossReport`. Silent structure loss is a defect.

### Witness tagging

`Ocel20` (in `witness.rs`) is the empty-enum witness marker:

```rust
assert_eq!(Ocel20::KEY, "ocel-2.0");
assert_eq!(Ocel20::TITLE, "OCEL 2.0");
assert_eq!(Ocel20::YEAR, Some(2023));
assert_eq!(Ocel20::FAMILY, WitnessFamily::Standard);
```

An `Admission<T, Ocel20>` cannot be silently mistaken for an `Admission<T, Xes1849>` at the type level.

---

## Algorithmic Architecture and Formal Relations

The engine implements zero-copy operations and builds transient indexes to execute queries and mine process graphs over OCEL 2.0 logs as implemented in [query.rs](file:///Users/sac/process-intelligence/sources/wasm4pm/src/query.rs) and [ocel.rs](file:///Users/sac/process-intelligence/sources/wasm4pm/src/ocel.rs).

### 1. Object-Centric Index Construction
To query and mine the event log without quadratic scanning overhead, the engine constructs a transient Event-to-Object ($E2O$) index and an inverted Object-to-Event ($O2E$) index:
- **E2O Traversal ($O(1)$)**: Maps an event index $e \in [0, N_{events}-1]$ to its set of related object indices:
  $$E2O(e) = \{ o_i \mid (e, o_i) \in E2O \}$$
- **Transient O2E Index ($O(N_{events})$ Construction)**: Aggregates occurrences of objects in events, building contiguous arrays with index offsets:
  $$O2E(o) = \{ e_j \mid (e_j, o) \in E2O \}$$

### 2. Zero-Copy Bitmask Projections
For sub-process mining and interactive analysis, events are filtered using a bitmask $B$ where $B[e] \in \{0, 1\}$. The projected event stream is defined as:
  $$E_{projected} = \{ e \mid B[e] = 1 \}$$
This filter guarantees $O(1)$ additional heap allocations during DFG (Directly-Follows Graph) generation.

### 3. Multi-Perspective Process Mining
Unlike single-perspective process mining which flattens logs, the engine performs multi-perspective DFG computation. For a selected target object type $\tau$, the Directly-Follows relation between activity $A_1$ and activity $A_2$ is established if and only if there exists an object $o$ of type $type(o) = \tau$ that sequentially participates in both events:
  $$A_1 \to_{\tau} A_2 \iff \exists o \in \text{Objects}, \exists e_1, e_2 \in E_{projected} \text{ s.t. } type(o) = \tau \wedge o \in E2O(e_1) \wedge o \in E2O(e_2) \wedge e_1 < e_2 \wedge \nexists e_3 \in E_{projected} \text{ s.t. } o \in E2O(e_3) \wedge e_1 < e_3 < e_2$$

### 4. Event-to-Object Relations and OCPQ
Object-Centric Process Queries (OCPQ) search for transaction patterns across shared objects. A query $Q(A_1, A_2, \Delta t_{max})$ matches pairs of events $(e_1, e_2)$ related by a shared object $o$:
  $$\text{Match}(e_1, e_2, o) \iff o \in E2O(e_1) \cap E2O(e_2) \wedge activity(e_1) = A_1 \wedge activity(e_2) = A_2 \wedge 0 < timestamp(e_2) - timestamp(e_1) \le \Delta t_{max}$$

---

## What wasm4pm Must Provide

The structure-only contract in this crate guarantees an OCEL log is well-shaped enough to mine. The engine graduation responsibilities are:

| Capability | Graduates to |
|---|---|
| OC-Petri-net discovery | `wasm4pm` |
| OC-DFG discovery | `wasm4pm` |
| Object-centric conformance checking | `wasm4pm` |
| OCPQ (object-centric process query) evaluation | `wasm4pm` |
| Flattening with convergence/divergence accounting | `wasm4pm` |
| Streaming OCEL ingestion and incremental log update | `wasm4pm` |

---

## Board Placement

> "Our process intelligence is built on OCEL 2.0 — the IEEE-standard for object-centric process logs."

This claim is grounded: wasm4pm-compat defines every OCEL 2.0 structural shape, enforces link integrity at the structural level, and names every deviation as a typed law. The runtime execution and discovery that make OCEL intelligence actionable graduate to the `wasm4pm` engine.
