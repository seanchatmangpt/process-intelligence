# OBJECT-CENTRIC RUNTIME ANALYSIS — wasm4pm

**Agent:** E3 — wasm4pm Execution Authority
**Date:** 2026-05-31

---

## Executive Summary

wasm4pm has a strong structural foundation for object-centric process mining (OCEL 2.0 types, OCPQ runtime, ocel-core validation, flattening). The critical gap is algorithmic: no OC-DFG discovery, no OC-Petri-net discovery, no object lifecycle conformance, and no multi-object conformance checking exists in `wasm4pm-algos`. OCPQ query evaluation exists in isolation but is disconnected from the evidence pipeline.

---

## 1. OCEL Type Foundation

### 1.1 ocel-core (wasm4pm-internal)

`crates/ocel-core/src/lib.rs` defines:

| Type | Description |
|---|---|
| `OCEL` | `{ event_types, object_types, events, objects }` — OCEL 2.0 JSON schema |
| `OCELEvent` | `{ id, event_type, time: DateTime<FixedOffset>, attributes, relationships }` |
| `OCELObject` | `{ id, object_type, attributes, relationships }` |
| `OCELRelationship` | `{ object_id, qualifier }` — E2O qualifier |
| `OCELType` | `{ name, attributes }` — type declaration |

These are OCEL 2.0 compliant (Ghahfarokhi et al. 2021 ICSOC).

### 1.2 wasm4pm-compat OCEL surface

`wasm4pm-compat/src/ocel.rs` defines:

| Type | Description |
|---|---|
| `OcelLog` | `{ object_types, events, objects, e2o_links, o2o_links, object_changes }` |
| `OcelEvent` | Builder API: `.at_ns()`, `.with_attr()` |
| `EventObjectLink` (E2O) | `{ event_id, object_id, qualifier }` with `.qualified()` |
| `ObjectObjectLink` (O2O) | `{ from_object_id, to_object_id, qualifier }` |
| `ObjectChange` | `{ object_id, event_id, attribute_name, new_value }` |
| `OcelRefusal` | Named laws: `FlatteningLoss`, `DanglingEventObjectLink`, `MissingObjectType`, etc. |

The compat OCEL is structure-only with loss-aware flattening. `OcelRefusal::FlatteningLoss` names flattening as a lawful loss requiring a `LossPolicy`.

### 1.3 Object Lifecycle (wasm4pm-compat)

`wasm4pm-compat/src/object_lifecycle.rs` defines `ObjectLifecyclePhase` as a `ConstParamTy` enum: `Created → Active → Modified → Archived → Deleted`. Lifecycle transitions are enforced at compile time via `LifecycledObject<PHASE>`. Discovery of lifecycle models from logs graduates to wasm4pm explicitly.

---

## 2. OCEL Validation

### ocel-core validate

`crates/ocel-core/src/validate.rs` implements validation grounded in OCEDO (Latif et al.) and OCPQ Def. 2. It checks:
- Every event has at least one qualified object reference
- No dangling E2O references (event references undeclared object)
- No dangling O2O references
- All object/event types declared up-front
- Object type cardinality constraints (`min_count`, `max_count`)
- `created_by`/`terminated_by` lifecycle event type declarations

**Gap:** `ValidationError.code` is a `String` (e.g. `"E2O_EMPTY"`, `"DANGLING_E2O"`). It uses machine-stable code strings rather than named law types. This is better than wasm4pm-types' `ValidationError(String)` but is not the named-law pattern of wasm4pm-compat.

---

## 3. OCEL Flattening

`crates/ocel-core/src/flatten.rs` implements deterministic OCEL-to-XES projection:
- Projects OCEL onto one object type → one case per object of that type
- Cases ordered by object id; events within case by `(time, event_id)`
- Events referencing multiple objects of the projected type are duplicated (convergence)
- Events not referencing any object of the projected type are dropped (divergence boundary)

**Type:** `flatten(ocel: &OCEL, object_type: &str) -> Result<FlatLog, String>`

**Gap:** Returns `Err(String)` on unknown type. No `LossReport` on the flattened output. Silent convergence/divergence — not named as loss events. This directly violates the wasm4pm-compat covenant: `OcelRefusal::FlatteningLoss` mandates a named loss policy and loss report on any OCEL-to-flat projection.

---

## 4. OCPQ Runtime

`crates/ocpq/src/lib.rs` implements the full OCPQ runtime from Küsters & van der Aalst arXiv:2506.11541v1 2025:

| Component | Paper Reference | Status |
|---|---|---|
| `Binding` | Def. 3 — variable bindings `b = b1 ∪ b2` | Present |
| `Binding::refines` | Def. 4 — parent-child relation `⊑_L` | Present |
| `BasicPredicate` | Def. 5 — E2O / O2O / TBE predicates | Present |
| `BindingBox` | Def. 6 — `b_L = (Var, Pred)` + `out_L` | Present |
| `BindingBox::refines` | Def. 7 — `a ⪯_L b` | Present |
| `QueryTree` | Def. 9 — `T = (V, F, r, l, box)` | Present |
| `ChildSet` | `CHILD SET_u^T (n_min, n_max)` Sect. 4 | Present |
| `Constraint` / `evaluate_constraint` | `constr(v)` → satisfied / violated (Fig. 6) | Present |

**This is a significant capability.** OCPQ evaluation at this formality level (paper Def. 3-9) is not present in PM4Py.

**Gap 1:** `ocpq` is a separate crate, not integrated into `wasm4pm-algos`. No algorithm pipeline connects admitted OCEL evidence to OCPQ query evaluation.

**Gap 2:** The OCPQ runtime takes `ocel_core::OCEL` directly — it bypasses the admission layer. There is no path from `wasm4pm-compat/src/ocel.rs` `OcelLog` (the typed, admitted evidence form) to OCPQ evaluation.

**Gap 3:** OCPQ query results carry no `ProvenanceChain`. Query evaluation leaves no receipt.

---

## 5. Object-Centric Algorithm Gaps

### 5.1 OC-DFG — MISSING

PM4Py `pm4py.algo.discovery.ocel.ocdfg` computes a per-object-type directly-follows graph from an OCEL. wasm4pm has no equivalent. The types are ready (`OCEL` with `e2o` relations via `OCELRelationship`), but no algorithm iterates object types to build per-type DFGs.

### 5.2 Object-Centric Petri Nets — MISSING

Van der Aalst & Berti 2020 (FI 175, arXiv:2010.02047) defines Object-Centric Petri Nets (OC-PNs) with variable arcs. No OC-PN type exists in wasm4pm-types or pm-core, and no OC-PN discovery algorithm exists in wasm4pm-algos.

### 5.3 Object Lifecycle Model Discovery — MISSING

Given an OCEL, discovering the lifecycle model of each object type (what event sequences objects of each type undergo) is absent. `wasm4pm-compat/src/object_lifecycle.rs` provides the typed phase markers but explicitly defers lifecycle discovery to wasm4pm.

### 5.4 Multi-Object Conformance — MISSING

Conformance checking where a trace spans multiple object types simultaneously (interleaved object lifecycles) does not exist. The token replay algorithm in `wasm4pm-algos/src/conformance.rs` operates on a flat `EventLog` and a flat `DFG` — it has no concept of object-centric execution.

### 5.5 Object-Centric Variant Analysis — MISSING

PM4Py provides `pm4py.algo.discovery.ocel.ocpn` and variant analysis over OCEL. wasm4pm has no equivalent.

---

## 6. PM4Py Object-Centric Comparison

| Capability | PM4Py | wasm4pm | Status |
|---|---|---|---|
| OCEL 2.0 JSON parse | Yes | Yes (ocel-core) | Present |
| OCEL structural validation | Yes | Yes (ocel-core/validate) | Present |
| OCEL flattening to XES | Yes | Yes (ocel-core/flatten) | Present (no LossReport) |
| OC-DFG discovery | Yes | No | MISSING |
| OC-Petri-Net discovery | Yes | No | MISSING |
| Object lifecycle model discovery | Yes | No | MISSING |
| Multi-object conformance | Yes | No | MISSING |
| OCPQ query evaluation | No | Yes (isolated) | Present, not connected |
| Object lifecycle type law | No | Yes (compat) | Compat only |
| Typed E2O/O2O with loss policy | No | Yes (compat) | Compat only |
| OCEL admission with named laws | No | Yes (compat) | Compat only, not bridged |

---

## 7. The Critical Integration Gap

wasm4pm-compat defines `OcelRefusal` with named structural laws:
- `FlatteningLoss` — flattening is lossy, must be named and reported
- `DanglingEventObjectLink` — E2O references unknown object
- `MissingObjectType` — event type not declared
- `EmptyObjectTypeSet` — OCEL has no declared object types

ocel-core/validate uses string-coded errors (`code: String`). The compat named-law surface and the wasm4pm runtime surface are parallel but never connected. An admitted `OcelLog` from compat cannot flow into OCPQ evaluation without manual re-serialization and re-validation — losing all type-law guarantees.

The covenant from graduation.rs: "Compat carries the evidence. wasm4pm adjudicates it." The current state: compat carries the evidence, and it stops there.
