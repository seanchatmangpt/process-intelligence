# OCEL 2.0 — Deep Formal Analysis

**Analyst:** Dr. OCEL Specialist (AGI)
**Date:** 2026-05-31
**Source:** OCEL 2.0 Standard (IEEE), van der Aalst et al.

---

## Formal Objects

### OcelEvent
- `event_id: EventId` — unique identifier
- `activity: ActivityName` — the activity label
- `timestamp: Timestamp` — point-in-time occurrence
- `attributes: Map<AttributeName, AttributeValue>` — event-level properties
- Constraint: `timestamp` must be totally ordered within a trace but only partially ordered across object lifecycles

### ObjectInstance
- `object_id: ObjectId` — unique identifier
- `object_type: ObjectType` — type classification (e.g., Order, Invoice, Payment)
- `attributes: Map<AttributeName, AttributeValue>` — current attribute snapshot
- Constraint: object identity is stable; type is invariant after creation

### EventObjectLink (E2O)
- `event_id: EventId` — source event
- `object_id: ObjectId` — target object
- `qualifier: Option<QualifierName>` — named role of this object in this event
- Constraint: links are directional and non-duplicated per (event, object, qualifier) triple
- Semantic: "this event affected or involved this object in this qualified role"

### Object-to-Object Link (O2O)
- `source_object_id: ObjectId`
- `target_object_id: ObjectId`
- `qualifier: Option<QualifierName>`
- Constraint: qualifier disambiguates multiple O2O edges between the same pair
- Semantic: "source object is structurally related to target object in this role"

### ObjectAttributeChange
- `object_id: ObjectId`
- `timestamp: Timestamp`
- `attribute: AttributeName`
- `old_value: AttributeValue`
- `new_value: AttributeValue`
- Constraint: changes must be ordered by timestamp; initial state derives from ObjectInstance baseline

---

## Key Insight: Multi-Type Concurrent Object Involvement

The defining advancement of OCEL 2.0 over flat XES traces is that **processes involve multiple TYPES of objects simultaneously**. A single event (e.g., `invoice_sent`) may link to:
- An `Order` object (which order triggered this invoice)
- A `Customer` object (who receives the invoice)
- An `Invoice` object (the artifact being sent)

This is structurally impossible to represent in XES without information loss. The OCEL 2.0 model captures this natively via E2O links with qualifiers.

**Consequence for process mining:** Any algorithm that flattens OCEL to a single-case-notion trace loses cross-object causality. Object-centric discovery algorithms (OC-DFG, POWL, Inductive Miner variants) must operate on the full OCEL graph.

---

## wasm4pm-compat Coverage

| OCEL 2.0 Concept | wasm4pm-compat Module | Coverage |
|---|---|---|
| OcelEvent | `src/ocel.rs` — `OcelEvent` struct with builder chain | Full |
| ObjectInstance | `src/ocel.rs` — `OcelObject` struct | Full |
| EventObjectLink (E2O) | `src/ocel.rs` — `EventObjectLink` with `qualified()` | Full |
| Object-to-Object Link (O2O) | `src/ocel.rs` — `ObjectObjectLink` with `qualified()` | Full |
| ObjectAttributeChange | `src/ocel.rs` — `ObjectChange` with timestamp | Full |
| OcelLog (container) | `src/ocel.rs` — `OcelLog` with `validate()` | Full |
| Admission gate | `src/admission.rs` — `Admit<OcelLog, Ocel20>` | Full |
| Witness typing | `src/witness.rs` — `Ocel20` witness marker | Full |

The crate provides structure and admission law. It does **not** execute discovery or conformance — those graduate to wasm4pm.

---

## wasm4pm Execution Obligations

The following capabilities are **not present in wasm4pm-compat** and must be implemented in the wasm4pm execution engine:

### OC-DFG Discovery
- Input: `Admitted<OcelLog, Ocel20>`
- Output: Per-object-type directly-follows graphs with frequency and performance annotations
- Algorithm: For each object type, extract the sequence of activities for each object instance; aggregate into a weighted DFG
- Current state: **MISSING** in wasm4pm

### Multi-Perspective Conformance
- Input: `Admitted<OcelLog, Ocel20>` + a discovered or normative process model
- Output: `Metric<FITNESS, NUM, DEN>` per object type + cross-type causal conformance
- Current state: **MISSING** in wasm4pm

### OCPQ Evaluation
- Input: `Admitted<OcelLog, Ocel20>` + OCPQ query (Object-Centric Process Query)
- Output: Boolean or quantitative answer with evidence binding
- Reference: van der Aalst OCPQ paper
- Current state: **MISSING** in wasm4pm; `src/ocpq.rs` in wasm4pm-compat defines query shapes only

---

## PM4Py Coverage Assessment

| Capability | PM4Py Module | Maturity |
|---|---|---|
| OCEL 2.0 parsing | `pm4py.ocel` | Moderate — supports JSON-OCEL and XML-OCEL |
| OC-DFG discovery | `pm4py.discovery.discover_oc_dfg` | Limited — frequency only, no performance overlay |
| OCPQ evaluation | Not present | None |
| Multi-perspective conformance | Not present (only per-flattened-type) | None |

PM4Py treats OCEL as a collection of flat logs (one per object type). It does not natively handle cross-type causality in conformance.

---

## Board Claim

> "Our process intelligence is built on OCEL 2.0 — IEEE-standard object-centric process logs."

**Evidence chain required:**
1. `OcelLog::admit()` via `Admit<OcelLog, Ocel20>` — proves log is structurally valid OCEL 2.0
2. `Ocel20` witness marker — names the specific standard (IEEE)
3. Every discovery and conformance result must carry `Ocel20` in its witness type — otherwise the claim is unattested
4. The receipt chain must show `Admitted<OcelLog, Ocel20>` as the root, not a flattened XES trace

**Gap:** If wasm4pm accepts a flattened `EventLog` instead of `Admitted<OcelLog, Ocel20>`, the board claim is **unattested**. This is GAP_001 in the evidence chain.

---

## Action Items

| Priority | Action | Owner |
|---|---|---|
| P0 | Implement OC-DFG discovery in wasm4pm consuming `Admitted<OcelLog, Ocel20>` | wasm4pm |
| P0 | Implement multi-perspective conformance with `Ocel20` witness threading | wasm4pm |
| P1 | Implement OCPQ evaluation | wasm4pm |
| P1 | Add `Ocel20`-typed receipt to every OCEL discovery result | wasm4pm |
