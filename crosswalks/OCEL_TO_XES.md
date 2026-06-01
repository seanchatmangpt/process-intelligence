# Crosswalk: OCEL 2.0 to XES Projection with Named Loss Policy

## The Structural Mismatch

OCEL 2.0 and XES represent fundamentally different ontologies:

| Dimension | OCEL 2.0 | XES |
|---|---|---|
| Case notion | Multiple object types, many-to-many | Single case ID per trace |
| Event structure | Event relates to N objects of M types | Event belongs to exactly one trace |
| Object relationships | O2O links (typed, qualified) | None |
| Object attribute changes | `ObjectChange` with timestamp | None |
| Extension system | Object types as first-class schema | Extension declarations (concept:name, time:timestamp, etc.) |
| Multi-perspective | Built-in (objects, events, relations) | External (requires custom extensions) |

Projecting OCEL to XES requires choosing **one object type** as the case notion.
All other object types, O2O edges, and object attribute change events are
structurally dropped. This is not a serialization detail — it is information
destruction.

## What Is Lost

| Lost item | OCEL source | XES destination | Loss type |
|---|---|---|---|
| Non-chosen object types | `Object { object_type: "item" }` | Absent | Hard drop |
| E2O links to non-chosen objects | `EventObjectLink { object_type: "item" }` | Absent | Hard drop |
| O2O links | `ObjectObjectLink { source_id, target_id }` | Absent | Hard drop |
| Object attribute changes | `ObjectChange { object_id, timestamp, attr }` | Absent | Hard drop |
| Convergence/divergence structure | When event links to 2+ objects of same type | Flattened | Semantic loss |
| Object qualifiers | `EventObjectLink::qualified(q)` | Absent | Hard drop |
| Case-level divergence | One event spawning two case streams | One trace only | Semantic loss |

The number of lost O2O edges is computable as `ocel_log.o2o_links.len()`.
The number of dropped object types is `ocel_log.object_types.len() - 1`.

## wasm4pm-compat LossPolicy: The Lawful Contract

In `wasm4pm-compat/src/loss.rs`, the `LossPolicy` enum names three stances:

```rust
pub enum LossPolicy {
    /// Refuse the projection if any loss would occur.
    RefuseLoss,
    /// Allow loss if the projection is named and the loss is declared.
    AllowNamedProjection { projection: ProjectionName },
    /// Allow loss and attach a full LossReport.
    AllowLossWithReport,
}
```

For OCEL → XES, `RefuseLoss` will always refuse (because loss is inherent to the
projection). The lawful stance is `AllowNamedProjection` with a `LossReport`.

### The Named Projection Contract

```rust
use wasm4pm_compat::loss::{LossPolicy, ProjectionName, LossReport};
use wasm4pm_compat::xes::XesExportRefusal;

let policy = LossPolicy::AllowNamedProjection {
    projection: ProjectionName::new("ocel_to_xes_order_case_notion"),
};

// The loss report must name: from type, to type, dropped items
let report: LossReport<OcelLog, XesLog, DroppedItems> = LossReport::new(
    policy,
    dropped_object_types,
    dropped_o2o_count,
    dropped_object_changes,
);
```

The `ProjectionName` is a `&'static str` newtype. It is the audit trail: the
name appears in the `LossReport`, travels with the exported XES, and is the
answer to "why are my objects missing?"

### XesExportRefusal: Named Laws for Refusal

The refusal type for XES export is not a string. It is a named enum:

```rust
pub enum XesExportRefusal {
    /// No case notion was specified and no single object type dominates.
    NoCaseNotion,
    /// The chosen case notion maps to zero events.
    EmptyCaseNotion { chosen_type: String },
    /// Loss is total: no events survive the case notion filter.
    TotalLoss,
    /// The LossPolicy is RefuseLoss but loss is inherent to this projection.
    LossRefused { object_type_count: usize, o2o_count: usize },
}
```

## PM4Py's Approach

PM4Py's `pm4py.convert.convert_to_event_log(ocel)` performs the OCEL → XES
projection silently:
- It picks a default case notion (often the first object type alphabetically).
- It drops all other object types without any record.
- It drops all O2O links without any record.
- No `LossReport` is emitted. No `ProjectionName` is recorded.
- The resulting `EventLog` carries no evidence that it was projected from OCEL.

This means: if you project `orders`, `items`, `packages` OCEL to XES using
PM4Py, you get an XES log with only `orders`. The `items` and `packages` are
gone, and the XES carries no indication that they ever existed.

## wasm4pm-compat: No Silent Loss

`src/formats.rs` enforces the boundary covenant:

> There is no `import_then_export` that skips the typed admitted middle.
> Every translation is `external → admitted compat → external`.

For OCEL → XES this means:
1. Raw OCEL bytes → `FormatEnvelope { kind: FormatKind::OcelJson, … }`
2. `ImportFormat::import` → `Admission<OcelLog, Ocel20>` or `Refusal<OcelRefusal, Ocel20>`
3. Admitted `OcelLog` → lossy `Project` with `LossPolicy::AllowNamedProjection`
4. `LossReport` emitted, `ProjectionName` attached
5. Projected `XesLog` → `ExportFormat::export` → `FormatExport { kind: XesXml, … }`

No step can be skipped. No loss is silent. The `LossReport` is a first-class
type companion to the exported XES bytes.

## Summary

OCEL → XES projection is inherently lossy. wasm4pm-compat names that loss with
a `ProjectionName`, quantifies it in a `LossReport`, and refuses silently-dropped
object types via `XesExportRefusal::LossRefused`. PM4Py performs the same
projection silently, with no loss record, no refusal surface, and no audit trail.
