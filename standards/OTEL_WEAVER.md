# OTel and Weaver — OpenTelemetry as Process Event Log Source

**Authority:** process-intelligence  
**Source Standard:** OpenTelemetry (CNCF), OTel Weaver semantic conventions framework  
**wasm4pm-compat surface:** `src/interop.rs`

---

## What OpenTelemetry Is in This Context

OpenTelemetry (OTel) is the CNCF standard for distributed system observability: traces, metrics, and logs. An OTel trace is a tree of spans. Each span records:

- A named operation (`SpanName` → activity name)
- A start and end timestamp
- A parent span reference (causality)
- Attributes (key-value metadata)
- Resource attributes (service name, version, environment)

OTel traces are the primary source of process event evidence in software systems. Every distributed transaction — an order placed, a payment processed, a shipment dispatched — leaves an OTel trace if the system is instrumented.

**OTel Weaver** is the OTel semantic conventions framework: a typed schema for what attributes a span of a given kind must or should carry. Weaver provides machine-verifiable semantic conventions — the SHACL equivalent for OTel telemetry.

---

## Why OTel Belongs in Process Intelligence Authority

Process mining requires event logs. In modern software systems, the canonical source of event evidence is OTel traces, not hand-crafted XES files. The path from system behavior to process intelligence runs through OTel.

Process intelligence authority over OTel means:

1. We define what OTel spans constitute valid process events (via Weaver semantic conventions)
2. We define the transformation from OTel spans to OCEL events
3. We own the loss accounting for what is discarded in that transformation

---

## The OTel Span → OCEL Event Transformation

An OTel span can be projected to an OCEL event when three conditions are met:

| Required field | OTel source | OCEL target |
|---|---|---|
| Activity name | `span.name` or `span.attributes["process.activity"]` | `OcelEvent.activity` |
| Timestamp | `span.startTime` (ISO 8601 nanosecond-precision) | `OcelEvent.timestamp_ns` |
| Object references | `span.attributes["object.type"]` + `span.attributes["object.id"]` | `EventObjectLink` |

This is a **projection with loss**, not a lossless round-trip.

---

## What Is Lost in OTel → OCEL

The following information present in OTel has no direct OCEL equivalent or is not preserved by default:

| Lost in transformation | Why it is lost | Impact |
|---|---|---|
| Span duration (end time) | OCEL events are point-in-time; `endTime` has no direct mapping | Performance analysis requires explicit `OcelEvent` for span start and span end |
| Parent-child span causality (O2O) | Vanilla OTel has no object-to-object relation type; `parentSpanId` is a span relation, not an object relation | Object-to-object relationships (`ObjectObjectLink`) cannot be derived from span parentage without domain-specific conventions |
| Span status (OK/ERROR) | OCEL has no built-in event success/failure attribute | Must be encoded as a named `OcelAttribute` or a separate `OcelEvent` |
| Resource attributes (service, host) | Resource-level attributes apply to many spans; there is no OCEL object type for "service" by default | Must define an `OcelObject` type for service instances if resource provenance is required |
| Trace-level baggage | Distributed context propagation metadata has no OCEL equivalent | Dropped unless encoded in object attributes |

**This loss must be documented via `LossPolicy::AllowLossWithReport` and a `LossReport<OtelSpan, OcelEvent, Items>` in the wasm4pm-compat loss accounting surface.**

---

## wasm4pm-compat Bridge: `src/interop.rs`

`src/interop.rs` provides the structural shapes for OTel → OCEL bridging:

- The import surface declares what an OTel-sourced OCEL log looks like structurally
- Loss accounting shapes record what was dropped and why
- The `LossPolicy` enforces that silent structure loss is a defect, not a default

The actual transformation pipeline (reading OTel protobuf/JSON, resolving span parentage, materializing `EventObjectLink`s from span attributes) is engine logic that graduates to `wasm4pm` under `GraduationReason::NeedsObjectCentricQueryExecution`.

---

## OTel Weaver: Typed Semantic Conventions

OTel Weaver defines semantic conventions as typed schemas. For example, the `db.client.operation` semantic convention specifies:

```yaml
groups:
  - id: db.client.operation
    type: span
    attributes:
      - ref: db.system
        requirement_level: required
      - ref: db.operation.name
        requirement_level: required
```

This is a Weaver-enforced contract: a span claiming to represent a database operation must carry `db.system` and `db.operation.name`. Process intelligence can define analogous Weaver conventions for process activities:

```yaml
groups:
  - id: process.activity
    type: span
    attributes:
      - id: process.activity.name
        type: string
        requirement_level: required
      - id: process.object.type
        type: string
        requirement_level: required
      - id: process.object.id
        type: string
        requirement_level: required
```

When these conventions are enforced by Weaver, the OTel → OCEL projection becomes lossless for the required fields.

---

## Relationship to Other Standards

- **OCEL 2.0** — OTel spans project into OCEL events; OCEL is the target format
- **PROV-O** — OTel traces can be lifted to PROV-O provenance graphs; the span tree maps to `prov:Activity` chains
- **XES** — OTel spans can also project to flat XES traces if only a single object type per case is relevant (with documented convergence loss)
- **SHACL** — Weaver semantic conventions play the same role for OTel that SHACL shapes play for RDF: named, machine-verifiable structural constraints

---

## Board Claim Contribution

> "Our process intelligence ingests real system telemetry from the CNCF OpenTelemetry standard."

This grounds process intelligence in live system behavior, not synthetic event logs. The OTel → OCEL transformation is documented, loss-accounted, and grounded in public CNCF standards. No proprietary telemetry format required.
