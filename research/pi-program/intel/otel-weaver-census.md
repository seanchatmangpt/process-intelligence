# OTel Weaver Research Census
## Process Intelligence Integration Authority Map

**Authority:** `/Users/sac/process-intelligence/otel-weaver`  
**Status:** GGEN_OTEL_WEAVER_PI_ALIVE_001  
**Date:** 2026-06-01  
**Researcher:** Process Intelligence Research Foundry  

---

## 1. Executive Summary

This census documents the complete OTel Weaver integration layer for Process Intelligence (PI), establishing the formal authority and classification boundaries for telemetry feedstock as it transits from raw observability signals to court-bound evidence artifacts.

**Critical Thesis:** Telemetry is feedstock; process consequence is court. This boundary is enforced through nominal category isolation doctrine, preventing the collapse of observability (systems engineering) into conformance verification (business law & auditable truth).

---

## 2. Weaver Registry Model

### 2.1 Schema Structure and Authority

**Location:** `/Users/sac/process-intelligence/otel-weaver/intel/weaver-registry-model.yaml`

The Weaver Registry defines the **schema of feedstock**, establishing the contract between telemetry emitters and the intake boundary.

**Manifest Schema:**
- **File:** `manifest.yaml` (root of registry)
- **Required Fields:**
  - `name`: Canonical registry identifier (e.g., "otel-process-conventions")
  - `schema_url`: Base URL defining the registry version (e.g., `https://opentelemetry.io/schemas/1.25.0`)
  - `dependencies`: External registries extended or merged

**Definition Schema:**
- **Location:** Subdirectories (typically `groups/` or `semconv/`)
- **File Pattern:** `*.yaml` files containing semantic convention groups
- **Group Types:** `span`, `event`, `metric`, `resource`, `attribute_group`, `entity`
- **Attribute Structure:**
  - `id`: Unique identifier within group
  - `type`: Data type (string, int, double, boolean, or array variants)
  - `requirement_level`: required, recommended, opt_in, conditional
  - `stability`: development, alpha, beta, release_candidate, stable
  - `examples`: Instance values for developers

**Process Intelligence Semantic Conventions (exp-001):**
The custom registry defines process-aware telemetry:
- `process.pi.activity` (span): Discrete execution activities
  - `process.pi.instance_id`: Case ID (required)
  - `process.pi.activity.name`: Petri net transition or BPMN task (required)
  - `process.pi.lifecycle`: Schedule, start, suspend, resume, complete, abort (required)
  - `process.pi.token.state_before/after`: Markings before/after transition (recommended)
  - `process.pi.witness.id`: Asserting witness node (required)
  - `process.pi.witness.hash`: BLAKE3 seal on transition trace (required)

### 2.2 Schema URL as Registry Identity

**Location:** `/Users/sac/process-intelligence/otel-weaver/mappings/schema-url-to-receipt-context-map.yaml`

Schema URLs function as **feedstock identity documents**. Each schema URL resolves to a specific receipt context governing how telemetry feedstock may be admitted into court.

**Registry Mappings:**
- `https://opentelemetry.io/schemas/1.25.0`
  - Receipt context: `ctx_otel_1_25_0_conformance`
  - Board-admissible: YES
  - Loss policy: `OtelToOcelLossPolicy`
  - Cryptographic signing: BLAKE3, 256-bit
  - Standards: ISO-IEC-23894:2024, OCEL 2.0

- `https://opentelemetry.io/schemas/1.24.0`
  - Receipt context: `ctx_otel_1_24_0_legacy`
  - Board-admissible: YES
  - Loss policy: `OtelToOcelLegacyLossPolicy`
  - Standards: OCEL 1.0

- `https://opentelemetry.io/schemas/1.20.0`
  - Receipt context: `ctx_otel_1_20_0_deprecated`
  - Board-admissible: NO (hearsay only)
  - Standards: XES 1.849
  - Loss policy: `OtelToXesLossPolicy`

**Unknown Schema URLs:** Raise validation warning, default to `ctx_unknown_telemetry_hearsay` (non-board-admissible).

---

## 3. Collector Integration Surfaces

### 3.1 Architecture

**Location:** `/Users/sac/process-intelligence/otel-weaver/intel/collector-integration-model.yaml`

The OpenTelemetry Collector serves as the **ingestion pipeline for raw telemetry feedstock**. Weaver validation is integrated at the processor or exporter stage, intercepting invalid telemetry before it affects court consequences.

**Integration Architecture:**

```
App/Service -> [OTel Collector] -> [Processors: transform/weaver_alignment] 
            -> [Exporters: otlp/weaver_live_check] -> [Weaver Live-Check]
```

### 3.2 Collector Configuration Template

**OTLP Receivers:**
- gRPC: `0.0.0.0:4317` (standard OTel port)
- HTTP: `0.0.0.0:4318` (alternative HTTP protocol)

**Transform Processor:** `transform/weaver_alignment`
- Mode: `propagate` error handling
- Function: Aligns incoming telemetry attributes with target schema version
- Example transformations:
  - Set `attributes["sdk.version"] = "1.25.0"`
  - Rename deprecated keys: `http.status_code` → `http.response.status_code`
  - Delete obsolete attributes
  - Use OpenTelemetry Transformation Language (OTTL) for complex mappings

**OTLP Exporters:** `otlp/weaver_live_check`
- Endpoint: `localhost:4317` (Weaver live-check gRPC listener)
- TLS: Insecure mode (development; secure in production)
- Sending queue: 2 consumers, enabled
- Retry: Enabled on failure

**Service Pipelines:**
- `traces`: receivers [otlp] → processors [transform/weaver_alignment] → exporters [otlp/weaver_live_check]
- `metrics`: receivers [otlp] → exporters [otlp/weaver_live_check]
- `logs`: receivers [otlp] → exporters [otlp/weaver_live_check]

---

## 4. Telemetry-to-PI Witness Mappings

### 4.1 Witness Definition

**Location:** `/Users/sac/process-intelligence/otel-weaver/mappings/otel-to-pi-witness-map.yaml`

A **witness is a telemetry feedstock source**, not a process consequence. Witnesses are classified by how and where telemetry originates.

**Witness Mapping Rules:**

1. **Service-to-Witness (wit_rule_01):**
   - Source: `service.name`, `service.namespace`
   - Target: `pi.feedstock.witness.id`
   - Format: `wit_{service.namespace}_{service.name}`
   - Fallback: `wit_anonymous_service`

2. **SDK-to-Source (wit_rule_02):**
   - Source: `telemetry.sdk.name`, `telemetry.sdk.language`, `otel.library.name`
   - Target: `pi.feedstock.source`
   - Format: `{sdk.name}:{sdk.language}:{otel.library.name}`
   - Fallback: `unknown_otel_source`

3. **Payload-Size (wit_rule_03):**
   - Source: `http.request.body.size`, `messaging.message.body.size`, `rpc.grpc.request.length`
   - Target: `pi.feedstock.payload_size`
   - Validation: Min value 0 bytes

**Witness Registry (Named Feedstock Sources):**

- **wit_sys_exec_01**: eBPF System Call Witness
  - Service: kernel-monitor
  - Library: ebpf-tracer
  - Feedstock type: ebpf_syscall
  - Security context: root_namespace

- **wit_db_audit_02**: Database Audit Log Witness
  - Service: postgres-db
  - Library: db-audit-tracer
  - Feedstock type: auditd
  - Security context: db_admin_role

- **wit_app_gateway_03**: API Gateway Edge Witness
  - Service: kong-gateway
  - Library: http-traffic-tracer
  - Feedstock type: application_log
  - Security context: gateway_proxy

**Conformance Isolation:** Never map feedstock directly to court consequences. A witness only asserts observations of feedstock, not conformity verdicts.

---

## 5. Finding-to-Refusal Mappings

### 5.1 Live-Check Validation Findings

**Location:** `/Users/sac/process-intelligence/otel-weaver/mappings/live-check-finding-to-refusal-map.yaml`

A **Weaver finding** is a structural compatibility check on telemetry feedstock. Findings map to **refusal codes**, which are the nominal category for intake court admissions.

### 5.2 Finding-to-Refusal Codex

| Finding ID | Description | Named Law | Standard | Verdict | Refusal Action | Remediation |
|---|---|---|---|---|---|---|
| `find_empty_payload` | Telemetry has zero-byte payload | EmptyTrace | XES 1849 | violation_halt | reject_feedstock_ingestion | Verify emitter instrumentation active state |
| `find_missing_trace_id` | Span lacks valid TraceID | MissingCaseId | XES 1849 | violation_halt | reject_feedstock_ingestion | Check W3C trace context propagation in HTTP headers |
| `find_non_monotone_timestamps` | Child span start < parent start, or child end > parent end | InvalidTimestampOrder | OCEL 2.0 | violation_halt | reject_and_quarantine_trace | Ensure synchronized clocks (NTP) across hosts |
| `find_dangling_span_parent` | Span references nonexistent parent span ID | DanglingEventObjectLink | OCEL 2.0 | escalation_warning | admit_with_quarantine_warning | Verify collector buffer size and tail-based sampling |
| `find_schema_url_missing` | Feedstock submitted without defining schema URL | ProjectionNameRequired | wasm4pm-compat loss covenant | violation_halt | reject_feedstock_ingestion | Ensure exporter populates schema_url property |
| `find_strict_schema_mismatch` | Feedstock schema version differs from system registry version | SchemaVersionMismatch | Weaver Semantic Registry Protocol | escalation_warning | admit_and_route_to_weaver_diff_log | Run weaver code generator to sync parser types |

**Verdict Classes:**
- `violation_halt`: Critical violation; reject intake
- `escalation_warning`: Admit with quarantine flag; escalate to monitoring
- `admission_conditional`: Admit pending external verification

---

## 6. Receipt Boundary Doctrine

### 6.1 Receipt Context Resolution

**Location:** `/Users/sac/process-intelligence/otel-weaver/mappings/schema-url-to-receipt-context-map.yaml`

A **receipt** is a court artifact; a **finding** is a feedstock assessment. The receipt boundary defines where telemetry crosses from feedstock into evidence.

**Receipt Context Structure:**
- `context_id`: Unique identifier (e.g., `ctx_otel_1_25_0_conformance`)
- `standards_conformance`: List of applicable standards (ISO, OCEL, XES, board-admissibility)
- `cryptographic_signing`: Algorithm (BLAKE3), hash length (256-bit), key derivation (HKDF-SHA256)
- `loss_policy_reference`: Named policy governing lossy projections (e.g., `OtelToOcelLossPolicy`)
- `compliance_assertion_bridge`: Link to Slide-to-Receipt map and board-admissible requirements

**Admission Covenant:** No feedstock enters the execution path directly. All feedstock must be:
1. **Admitted** via explicit `Admit` boundary (maps OTel spans to `OcelEvent` using verified projections)
2. **Loss-Reported** (every projection generates a `LossReport` accounting for dropped data)
3. **Witness-Sealed** (transformation sealed with zero-sized witness type, e.g., `OtelSpanToOcelEventProjection`)

---

## 7. Schema Versioning Surfaces

### 7.1 Schema Version Model

**Location:** `/Users/sac/process-intelligence/otel-weaver/intel/schema-url-version-model.yaml`

**Schema Translation Definition Format:**
- **Extension:** `.yaml`
- **Canonical filename:** `schema.yaml`
- **Root fields:**
  - `file_format`: Version of schema definition format itself (e.g., "1.0.0")
  - `schema_url`: Target schema URL identifying the latest version
  - `versions`: Map of individual schema versions to changes

**Version Changes (Transformations):**
- `rename_attributes`: Old attribute key → new attribute key mapping
- `rename_metrics`: Specific metric name renames
- `rename_spans`: Specific span name renames
- `metrics`: New/updated metric definitions
- `spans`: New/updated span definitions
- `logs`: New/updated log/event definitions
- `resources`: Resource-level attribute transformations

**Example Version Change:**
```yaml
versions:
  "1.25.0":
    changes:
      - rename_attributes:
          old.attribute.name: new.attribute.name
      - rename_metrics:
          - from: http.duration
            to: http.client.duration
```

---

## 8. Registry Diff Model

### 8.1 Diff-to-Isolation Mapping

**Location:** `/Users/sac/process-intelligence/otel-weaver/mappings/registry-diff-to-pi-drift-map.yaml`

**Critical Boundary:** Weaver schema diffs are NOT process drift.

**Schema Diff Categories:**

| Diff Type | Definition | Process Drift Impact | Reconciliation | Recompile? |
|---|---|---|---|---|
| `schema_addition` | New feedstock attributes added to registry | none | Extend feedstock parsers | NO |
| `attribute_deprecation` | Attributes marked deprecated | none | Mark parser keys optional | NO |
| `type_change` | Data type conversion of feedstock attribute | none_unless_unmapped | Update codec coercions | YES |

**Process Drift Definitions:**

| Drift Type | Definition | Telemetry Trigger | Category |
|---|---|---|---|
| `activity_skip` | Normative model activity bypassed at runtime | Missing expected span event | operational_runtime_drift |
| `sequence_violation` | Activities executed in violation of temporal constraints | Span timestamps violate parent-child ordering | operational_runtime_drift |
| `conformance_score_decay` | Aggregate conformance index drops below threshold | Token replay fitness < 0.95 | operational_runtime_drift |

**Isolation Rules:**

1. If a field is missing due to `schema_diff` (attribute_deprecation): Route to Weaver Diff Log (`/checkpoints/weaver_diff_registry.json`), NOT process drift court.

2. If a field is missing but present in current schema: Report Process Drift (Activity Skip / Data Under-provisioning) to process drift court (`/checkpoints/process_drift_court.json`).

---

## 9. Telemetry-to-Evidence Projection (OTel Signal Mapping)

### 9.1 OTel Span to OCEL Event Projection

**Location:** `/Users/sac/process-intelligence/otel-weaver/mappings/otel-signal-to-process-evidence-map.yaml`

**Projection Name:** `OtelSpanToOcelEventProjection` (witness marker)  
**Version:** 1.0.3  
**Immutable Signature:** `SIG_ED25519_550e8400e29b41d4a716446655440000`

**Structural Mappings:**

| OTel Source | PI Target | Description | Transform |
|---|---|---|---|
| TraceID | CaseCorrelationID | Execution flow across services → Case ID/Object ID | hex_to_string |
| SpanID | EventTransactionID | Specific execution step identifier | hex_to_string |
| ParentSpanID | ParentChildRelation | Preceding execution context; process model edges | hex_to_string |
| SpanName | ActivityLabel | Operation name → Process Mining activity | direct_copy |
| StartTime | StartEndTimestamp.Start | Execution beginning | epoch_ns_to_rfc3339 |
| EndTime | StartEndTimestamp.End | Execution completion | epoch_ns_to_rfc3339 |
| Attributes | DynamicAttributes | Contextual span metadata → event properties | map_key_value_pairs |

**Timing Constraints:**
- **Parent-Child Timing:** `t_start(parent) ≤ t_start(child) ≤ t_end(child) ≤ t_end(parent)`
  - Enforcement: Reject violating spans
  - Action: Emit timing refusal

- **Acyclic Dependencies:** Directed Acyclic Graph (DAG)
  - Validation: Cycle detection (DFS)
  - Action: Reject entire trace

### 9.2 Loss Policy

**Policy Name:** `OtelToOcelLossPolicy`  
**Output:** `LossReport` → `/checkpoints/otel_loss_reports.json`

**Accounting Rules:**
- **Span Duration Truncation:** Nanoseconds → microseconds (loss documented)
- **Resource Scope Collapse:** Resource attribute objects → flat string (loss documented)
- **Causality Degradation:** Async spans marked as correlated, not strictly causal (loss documented)

### 9.3 Conformance Target

- **Petri Net Reference:** `OrderPipelineNet`
- **POWL Reference:** `/experiments/powl_projection_sample.md`
- **Verification Engine:** `wasm4pm`
- **Court Verdict Mappings:**
  - `compliant_replay` → admissible receipt
  - `unfit_replay` → violation_halt refusal
  - `missing_witness` → escalation_warning

---

## 10. Live-Check Model (Real-Time Validation Engine)

### 10.1 Weaver Live-Check Ingestion

**Location:** `/Users/sac/process-intelligence/otel-weaver/intel/live-check-model.yaml`

The live-check command (`weaver registry live-check`) is a **real-time feedstock validation gateway**.

**Ingestion Sources:**
- **OTLP/gRPC:** `0.0.0.0:4317` (standard OTel protocol)
- **OTLP/HTTP:** Alternative HTTP protocol
- **stdin:** Piped JSON or line-delimited text
- **file:** Offline telemetry files

**Input Payload Schema (OTLP JSON):**
- `resource_spans`: Nested structure with resource attributes, scope spans, spans
  - `trace_id`: Hex-encoded string
  - `span_id`: Hex-encoded string
  - `parent_span_id`: Hex-encoded string
  - `name`: Activity name
  - `start_time_unix_nano`: Epoch nanoseconds
  - `end_time_unix_nano`: Epoch nanoseconds
  - `attributes`: Key-value pairs
  - `events`: Event objects within span

- `resource_metrics`: Nested structure with resource attributes, scope metrics, metrics
  - `name`: Metric name
  - `unit`: UCUM-compliant unit (ms, By, s, 1)
  - `data`: Gauge, sum, or histogram data

### 10.2 Evaluation Policies

**Type:** Rego (Open Policy Agent)  
**Execution Mode:** Stream-based (runs per incoming packet)  
**Rules:** Advice rules flag missing attributes, bad naming patterns, incorrect data types

### 10.3 Output Report Schema

**Formats:** ANSI, JSON, YAML, JSONL

**Fields:**
- `statistics`: Total entities processed, valid, invalid, processing time
- `violations`: Array of violation objects
  - `telemetry_type`: span, metric, log
  - `entity_id`: TraceID/SpanID or metric name
  - `policy_id`: Violated policy identifier
  - `severity`: error, warning
  - `message`: Human-readable violation text
  - `invalid_attributes`: Array of offending attribute names
  - `missing_attributes`: Array of required but absent attributes
- `status`: pass, fail, partial_pass

---

## 11. Weaver Policy Model

### 11.1 Policy Enforcement

**Location:** `/Users/sac/process-intelligence/otel-weaver/intel/weaver-policy-model.yaml`

Rego policies act as **code-level gates** enforcing schema conformity at validation time.

**Policy Execution Flow:**
1. Resolve imports and dependencies
2. Convert resolved registry to JSON
3. Run OPA Rego engine, passing registry JSON as `input`
4. Query `data.weaver.deny` and `data.weaver.warn` rules
5. Output violations to stderr or diagnostic format (ANSI, JSON, GitHub Workflow Command)

**Policy Rules:**

- **deny**: Critical violations
  - Evaluation to true/non-empty → validation fails, exit code 1
  - Example: "Metric group must define metric_name"

- **warn**: Non-fatal violations
  - Evaluation to true/non-empty → warning printed, exit code 0 (unless strict mode)
  - Example: "Attribute missing brief description"

**Policy Input Schema (Resolved Registry as JSON):**
```json
{
  "name": "registry_name",
  "groups": [
    {
      "id": "group_id",
      "type": "span|metric|event|attribute_group|resource|entity",
      "brief": "...",
      "note": "...",
      "prefix": "...",
      "stability": "...",
      "attributes": [...]
    }
  ],
  "attributes": [
    {
      "id": "...",
      "type": "string|int|double|boolean|...",
      "brief": "...",
      "stability": "...",
      "requirement_level": "required|recommended|opt_in|conditional",
      "examples": [...]
    }
  ]
}
```

---

## 12. Weaver CLI Command Map

### 12.1 Registry Subcommands

**Location:** `/Users/sac/process-intelligence/otel-weaver/intel/weaver-command-map.yaml`

**weaver registry check:**
- Validates registry syntax, structure, and Rego policies
- Options: `--registry`, `--policy`, `--skip-policies`, `--diagnostic-format`
- Exit codes: 0 (valid), 1 (invalid)

**weaver registry generate:**
- Generates code, documentation, or target-specific artifacts
- Arguments: TARGET (go, rust, markdown), OUTPUT (directory path)
- Options: `--templates`, `--config`, `--param`, `--registry`
- Exit codes: 0 (success), 1 (generation failure)

**weaver registry diff:**
- Computes differences between two registry versions
- Tracks schema evolution (contract differences), NOT runtime drift
- Options: `--baseline-registry` (required), `--registry`, `--format` (ansi, json, markdown)
- Output: Schema transition report
- Exit codes: 0 (success), 1 (evaluation error)

**weaver registry live-check:**
- Real-time telemetry validation gateway
- Options:
  - `--input-source`: file, stdin, otlp (default)
  - `--input-format`: text, json (default)
  - `--format`: ansi, json, yaml, jsonl, or template name
  - `--otlp-grpc-address`: Bind address (default `0.0.0.0`)
  - `--otlp-grpc-port`: Port (default `4317`)
  - `--output`: Report path (default stdout)
  - `--inactivity-timeout`: Seconds before shutdown (default `10`)
- Exit codes: 0 (clean stop), 1 (critical violation or config error)

**weaver registry stats:**
- Calculates registry statistics (group counts, metric counts, span counts, attribute counts)

**weaver registry json-schema:**
- Outputs JSON schema of resolved registries

**weaver registry package:**
- Packages registry files and templates into resolved distribution

**weaver registry mcp:**
- Starts a Model Context Protocol (MCP) server for querying semantic convention definitions

**weaver registry infer:**
- Auto-generates registry schema by sniffing OTLP payload shapes

**weaver diagnostic init:**
- Scaffolds local `diagnostic_templates` directory

**weaver serve:**
- Starts HTTP API server for querying and resolving semantic registries
- Default: `127.0.0.1:8080`

---

## 13. Telemetry Lifecycle: Ingest → Parse → Classify → Admit/Refuse

### 13.1 End-to-End Flow

**Stage 1: Ingest**
- Telemetry emitted by app/service instrumentation
- Collected by OTel Collector (OTLP/gRPC or HTTP)
- Feedstock enters ingestion pipeline

**Stage 2: Transform**
- OTel Collector transform processor aligns attributes with target schema version
- Uses OTTL (OpenTelemetry Transformation Language)
- Renames deprecated keys, deletes obsolete attributes

**Stage 3: Live-Check Validation**
- Weaver live-check validates feedstock against resolved registry
- Runs Rego policies to check schema conformity
- Generates violation reports (missing attributes, type mismatches, naming violations)

**Stage 4: Finding Classification**
- Findings mapped to named laws and refusal codes
- Examples:
  - `find_empty_payload` → `violation_halt` (reject feedstock)
  - `find_dangling_span_parent` → `escalation_warning` (admit with quarantine)
  - `find_schema_url_missing` → `violation_halt` (reject feedstock)

**Stage 5: Admission/Refusal Decision**
- **Admission:** Feedstock passes all checks, routed to projections
  - Wrapped in `Admission<T, W>` type
  - Proceeds to OTel-to-OCEL projection
- **Refusal:** Feedstock fails critical checks
  - Wrapped in immutable `Refusal` record (with diagnostic payload)
  - Logged to quarantine (not used for court verdict computation)

**Stage 6: Projection (If Admitted)**
- OTel spans projected to OCEL event log using `OtelSpanToOcelEventProjection`
- Loss accounting: `LossReport` generated for truncated/collapsed data
- Witness seal: Zero-sized witness type marks transformation

**Stage 7: Conformance Checking (Court)**
- Projected OCEL log replayed against normative process model (`OrderPipelineNet`)
- Verification engine: `wasm4pm`
- Court verdict: compliant_replay (admission) or violation_halt (refusal)

---

## 14. Doctrine Authority Index

**Location:** `/Users/sac/process-intelligence/otel-weaver/doctrine/`

| Doctrine | File | Status | Purpose |
|---|---|---|---|
| Feedstock Theorem | `otel-weaver-is-feedstock.md` | ALIVE (OTEL_WEAVER_FEEDSTOCK_001) | Establishes boundary: schema validation ≠ conformance checking |
| Telemetry vs. Evidence | `telemetry-is-not-process-evidence.md` | ALIVE | Distinguishes feedstock from court artifacts |
| Finding vs. Receipt | `weaver-finding-is-not-receipt.md` | ALIVE (WEAVER_FINDING_NOT_RECEIPT_001) | Separates structural checks from operational validity |
| Registry Diff vs. Drift | `registry-diff-is-not-process-drift.md` | ALIVE | Prevents schema transitions from being misclassified as process drift |
| Observability vs. PI | `observability-by-design-vs-process-intelligence.md` | ALIVE | Contrasts engineering observability with outcome-centric PI |
| Dashboard Truth | `no-dashboard-truth.md` | ALIVE | Rejects visual metrics as ground truth; grounds truth in auditable evidence chains |

---

## 15. Experimental Implementations

**Location:** `/Users/sac/process-intelligence/otel-weaver/experiments/`

| Experiment | Purpose | Key Deliverables |
|---|---|---|
| **EXP-001** | Custom PI Weaver Registry | `process_pi.yaml` semantic conventions for process telemetry |
| **EXP-002** | Weaver Diff to PI Residual | `bridge.rs` (BridgeRx) protecting conformance court from schema diffs |
| **EXP-003** | Live-Check to Refusal | `validator.rs` translating raw feedstock to `Admission<T, W>` or `Refusal` |
| **EXP-004** | Registry to wasm4pm-compat Witness | `generator.rs` automating witness type generation from resolved schema |
| **EXP-005** | Collector to PI Intake | Production OTel Collector config; ingestion pipeline to intake JSON |

---

## 16. Checkpoint Status

**Location:** `/Users/sac/process-intelligence/otel-weaver/checkpoints/`

| Checkpoint | Date | Status | Verdict |
|---|---|---|---|
| GGEN_OTEL_WEAVER_PI_ALIVE_001 | 2026-06-01 | COMPLETE & ACTIVE | All OTel Weaver integration assets verified and complete |
| GGEN_OTEL_WEAVER_PI_PARTIAL_001 | 2026-06-01 | Superceded | Earlier partial findings |
| GGEN_OTEL_WEAVER_PI_RUNTIME_001 | 2026-06-01 | Superceded | Runtime integration notes |

**Audit Authority:** Dr. Wil van der Aalst AGI Swarm Court  
**Lead Auditor Signature (BLAKE3):** `e6bc59b8d210e309228d4bf09c0fa4db55fe310efc35ad1a89b3f021e85a67f1`

---

## 17. Integration Authority Summary

### 17.1 Nominal Category Enforcements

**Teleomtry is Feedstock:**
- Raw spans, logs, metrics, semantic tags carry no execution authority
- Weaver validates **structure only**, not process compliance
- Schema validation ≠ conformance checking

**Process Consequence is Court:**
- Conformance checking, trace alignment, token replay exclusive to wasm4pm-compat / wasm4pm
- Only the court emits binding verdicts (receipts)
- Collector and instrumentation libraries must never compute conformance

**Weaver Diffs are Not Process Drift:**
- Diff: Design-time schema/attribute namespace modification
- Drift: Runtime behavioral deviation from process model
- Isolation rule: Route diffs to Weaver Diff Log, not process drift court

### 17.2 Boundary Invariants

1. **No Direct Feedstock Entry:** All feedstock wrapped in `Admission<T, W>` or `Refusal` at boundary
2. **Loss Reporting Covenant:** Every projection generates `LossReport` accounting for data loss
3. **Witness Sealing:** Transformations sealed with zero-sized witness types (cryptographically verifiable)
4. **Receipt Context Resolution:** Schema URL maps to receipt context governing admission rules
5. **Rego Policy Enforcement:** Violations trigger deterministic refusal codes

---

## 18. References

### Authority Documents
- `/Users/sac/process-intelligence/otel-weaver/README.md` — Architectural overview
- `/Users/sac/process-intelligence/otel-weaver/doctrine/` — Seven foundational doctrines
- `/Users/sac/process-intelligence/otel-weaver/mappings/` — Nominalcategory isolation maps
- `/Users/sac/process-intelligence/otel-weaver/intel/` — Technical specification models

### Standards Ledgers
- `/Users/sac/process-intelligence/standards/otel_weaver_projection_placement.md`
- `/Users/sac/process-intelligence/standards/otel_trace_integration_schema.json`
- `/Users/sac/process-intelligence/standards/OTEL_WEAVER.md`

### Process Intelligence Core
- `/Users/sac/process-intelligence/doctrine/RECEIPT_DOCTRINE.md`
- `/Users/sac/process-intelligence/ma/define_slide-to-receipt_map.md`
- `/Users/sac/process-intelligence/ma/define_board-admissible_claim_requirements.md`

### Process Mining Authority
- `~/.claude/rules/process-mining-chicago-tdd.md` — Van der Aalst Constitution
- `/Users/sac/process-intelligence/sources/pm4py/capability-atlas.md`
- `/Users/sac/process-intelligence/sources/wasm4pm/` — wasm4pm execution authority

---

## 19. Conclusion

The OTel Weaver integration layer establishes a strict, enforceable boundary between systems engineering (observability, telemetry feedstock) and business-legal reality (process intelligence, conformance court). Every telemetry packet that enters the system encounters a three-stage gatekeeper:

1. **Structural Validation** (Live-Check): Does the feedstock conform to schema?
2. **Finding Classification** (Finding-to-Refusal): Is the violation admissible?
3. **Type Law Enforcement** (Admission/Refusal): Can this feedstock be wrapped as evidence?

Telemetry is feedstock. Process consequence is court. This boundary must be preserved immutably.

**GGEN_OTEL_WEAVER_PI_INTEL_001 — CENSUS COMPLETE & AUTHORITATIVE**
