# GAP_P15_002: Missing OTel Trace Integration

**Severity:** HIGH  
**Status:** OPEN  
**Discovered:** 2026-06-01 (Phase 15 Audit)  
**Authority:** Dr. Wil van der Aalst Process Mining Constitution

---

## Problem Statement

The PI research program's event log is derived from **git history reconstruction** rather than **automatic operational traces**. This violates the Van der Aalst principle that process evidence must be **instrumentally extracted**, not manually reconstructed.

### Current State

**Event log source:** git commit history
```
git log --pretty=format:"%H|%an|%ai|%s" --reverse
```

**Missing:** OpenTelemetry (OTel) spans that capture:
- Exact activity timestamps
- Span IDs for trace correlation
- Causality links (parent-child spans)
- Success/failure attributes
- Structured event data (objects, qualifiers)

### Required State

**Event log source:** OTel traces automatically extracted from:
1. ggen execution (query execution, template rendering)
2. Checkpoint issuance (gate checks, verdict emission)
3. Audit execution (audit runs, result aggregation)
4. Receipt signing (BLAKE3 hash generation, witness chain)

---

## Van der Aalst Constitutional Requirement

From `~/.claude/rules/process-mining-chicago-tdd.md`:

### Event Log Derivation

> "Event log derivation — Convert OTel traces to object-centric OCEL event logs"

This is a **REQUIRED** step in the Van der Aalst validation workflow:

1. ✅ Event log derivation — Extract OTel traces to OCEL (MISSING)
2. ✅ Model-vs-log comparison — Discover actual process (DONE)
3. ❌ Temporal conformance — Verify stage ordering (DEPENDS ON OTel)
4. ❌ Object lifecycle soundness — Verify artifacts have lawful histories (DEPENDS ON OTel)
5. ❌ Negative testing — Verify impossible logs rejected (DEPENDS ON OTel)

**Status:** Audit cannot fully validate without OTel traces.

---

## Impact on Phase 15 Audit

### What Cannot Be Verified Without OTel Traces

| Verification | Requires | Current Status |
|---|---|---|
| **Exact timing** of activities | OTel timestamps | Inferred from commit dates (approximate) |
| **Causal ordering** of events | OTel parent-child spans | Inferred from git history (incomplete) |
| **Cross-object causality** | E2O/O2O links in traces | Reconstructed from metadata (partial) |
| **Hidden failures** | OTel error events | Not captured |
| **Concurrent execution** | OTel overlapping spans | Collapsed to sequential commits |
| **Automatic conformance check** | Real-time trace validation | Manual audit only |

### Fitness Impact

Without OTel traces, we **cannot automatically verify** that:
1. Declared activities (in ggen.toml) actually executed
2. Actual events match declared order
3. No activities were skipped or hidden

This is why Fitness = 8% — we are measuring against git history, not actual operational traces.

---

## Required OTel Instrumentation Points

### 1. ggen (Governance Generation Engine)

**File:** `/Users/sac/process-intelligence/ggen/` (or equivalent)

**Instrumentation needed:**
```rust
// Before: SPARQL query execution
let span = tracer.start("ggen.execute_query");
span.set_attribute("query_file", "extract-lifecycle-governance.rq");
span.set_attribute("ontology_namespace", "process-intelligence");
// ... execute query
span.set_attribute("event_count", events.len());
span.end();

// Before: Tera template rendering
let span = tracer.start("ggen.render_template");
span.set_attribute("template_file", "blue-river.tera");
span.set_attribute("output_format", "rust");
// ... render template
span.set_attribute("output_hash", blake3_hash);
span.end();

// Before: Output write
let span = tracer.start("ggen.write_output");
span.set_attribute("output_file", "../blue_river_dam/src/lib.rs");
span.set_attribute("mode", "Overwrite");
// ... write output
span.set_attribute("receipt_hash", blake3_receipt);
span.end();
```

### 2. Checkpoint System

**File:** `/Users/sac/process-intelligence/checkpoints/` (or equivalent)

**Instrumentation needed:**
```rust
// Before: Gate check
let span = tracer.start("checkpoint.gate_check");
span.set_attribute("phase", "research-paper");
span.set_attribute("criteria_count", 5);
// ... check criteria
span.set_attribute("all_passed", true);
span.end();

// Before: Checkpoint issuance
let span = tracer.start("checkpoint.issue_verdict");
span.set_attribute("checkpoint_id", "PROCESS_INTELLIGENCE_ALIVE_001");
span.set_attribute("verdict", "ALIVE");
// ... sign checkpoint
span.set_attribute("signature", "ed25519...");
span.end();
```

### 3. Audit System

**File:** `/Users/sac/process-intelligence/audits/` (or equivalent)

**Instrumentation needed:**
```rust
// Before: Audit execution
let span = tracer.start("audit.run");
span.set_attribute("audit_name", "NO_DTO_FLATTENING");
span.set_attribute("target", "wasm4pm-compat");
// ... run audit
span.set_attribute("violation_count", 1);
span.set_attribute("result", "FAIL");
span.end();
```

### 4. Receipt Signing

**File:** Receipt generation code

**Instrumentation needed:**
```rust
// Before: BLAKE3 hash generation
let span = tracer.start("receipt.hash_content");
span.set_attribute("content_type", "artifact");
span.set_attribute("artifact_id", "blue_river_dam.rs");
// ... compute hash
span.set_attribute("hash_value", "31422cd6...");
span.end();

// Before: Witness chain emission
let span = tracer.start("receipt.emit_witness");
span.set_attribute("previous_receipt_hash", "612d222...");
// ... create witness
span.set_attribute("witness_signature", "31422cd6a051f...");
span.end();
```

---

## Expected OCEL Output After Instrumentation

Once OTel tracing is enabled, the Phase 15 audit should automatically extract:

```json
{
  "ocel:events": [
    {
      "ocel:event-id": "E-001",
      "ocel:activity": "ggen.execute_query",
      "ocel:timestamp": "2026-05-31T22:42:54.123456Z",
      "ocel:object-refs": [
        {
          "ocel:object-id": "GGEN_ENGINE_001",
          "ocel:qualifier": "executed_by"
        }
      ],
      "ocel:attributes": {
        "query_file": "extract-lifecycle-governance.rq",
        "event_count": 36,
        "trace_id": "a1b2c3d4e5f6..."
      }
    },
    {
      "ocel:event-id": "E-002",
      "ocel:activity": "ggen.render_template",
      "ocel:timestamp": "2026-05-31T22:42:54.234567Z",
      "ocel:object-refs": [
        {
          "ocel:object-id": "GGEN_ENGINE_001",
          "ocel:qualifier": "executed_by"
        }
      ],
      "ocel:attributes": {
        "template_file": "blue-river.tera",
        "output_format": "rust",
        "output_hash": "31422cd6...",
        "parent_span_id": "a1b2c3d4e5f6..."
      }
    },
    {
      "ocel:event-id": "E-003",
      "ocel:activity": "ggen.write_output",
      "ocel:timestamp": "2026-05-31T22:42:54.345678Z",
      "ocel:object-refs": [
        {
          "ocel:object-id": "BR_DAM_001",
          "ocel:qualifier": "generated_by"
        }
      ],
      "ocel:attributes": {
        "output_file": "../blue_river_dam/src/lib.rs",
        "mode": "Overwrite",
        "receipt_hash": "31422cd6a051f...",
        "parent_span_id": "a1b2c3d4e5f6..."
      }
    }
  ]
}
```

---

## Benefits of OTel Instrumentation

### 1. Automatic Event Log Extraction

No manual git history reconstruction needed. Event logs are continuously emitted and can be queried.

### 2. Causal Traceability

Parent-child span relationships prove causality:
- SPARQL execution → Template rendering → Output writing
- Checkpoint gate check → Verdict issuance → Receipt signing

### 3. Real-time Conformance Monitoring

Audit can run continuously, not just at phase boundaries.

### 4. Temporal Validation

Exact timestamps enable verification of:
- Stage ordering (E.g., research-paper must complete before synthesis-lifecycle)
- No impossible overlaps
- Expected latencies

### 5. Failure Detection

OTel error events captured automatically:
- Query execution failures
- Template rendering errors
- File I/O failures
- Gate check failures

### 6. Cross-System Tracing

Trace IDs link activities across:
- ggen (manufacturing)
- checkpoint (gating)
- audit (verification)
- receipt system (witnessing)

---

## Implementation Roadmap

### Phase 1: Add OTel Dependencies

**Files:** All Rust crates in ggen/, checkpoint/, audit/ subsystems

```toml
[dependencies]
opentelemetry = "0.21"
opentelemetry-jaeger = "0.21"
opentelemetry-sdk = { version = "0.21", features = ["rt-tokio"] }
tracing = "0.1"
tracing-opentelemetry = "0.22"
```

### Phase 2: Initialize Global Tracer

**File:** `main.rs` or crate root

```rust
use opentelemetry_jaeger as jaeger;
use tracing_subscriber;

fn init_tracing() {
    let tracer = jaeger::new_agent_pipeline()
        .install_simple()
        .expect("Failed to initialize tracer");
    
    tracing_subscriber::registry()
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .init();
}
```

### Phase 3: Instrument Hot Paths

Add `#[instrument]` macro to key functions:

```rust
#[instrument(skip(input))]
pub fn execute_sparql_query(input: &str) -> Result<Vec<Event>> {
    // ... implementation
}

#[instrument]
pub fn render_tera_template(template_path: &str) -> Result<String> {
    // ... implementation
}

#[instrument]
pub fn issue_checkpoint(verdict: &Verdict) -> Result<Receipt> {
    // ... implementation
}
```

### Phase 4: Export Traces

Configure export to Jaeger or OpenTelemetry Collector:

```toml
[dependencies.opentelemetry-jaeger]
version = "0.21"
features = ["rt-tokio"]
```

### Phase 5: Validate OCEL Output

Extract OCEL from traces and verify conformance.

---

## Acceptance Criteria

### Definition of Done

1. ✅ OTel tracing added to ggen execution path
2. ✅ OTel tracing added to checkpoint system
3. ✅ OTel tracing added to audit system
4. ✅ OTel tracing added to receipt signing
5. ✅ Traces exported to Jaeger or OTel Collector
6. ✅ OCEL event log automatically derived from traces
7. ✅ Temporal conformance checks passing
8. ✅ Cross-system causal traceability verified

---

## Related Gaps

- **GAP_P15_001:** Manufacturing model under-specification
- **GAP_P15_003:** Blue River Dam provenance unclear
- **GAP_P15_004:** Missing conformance metrics computation

---

## References

- **Phase 15 Audit:** `/Users/sac/process-intelligence/research/pi-program/audits/phase-15-process-mining-conformance-audit.md`
- **OCEL Standard:** `/Users/sac/process-intelligence/standards/ocel.md`
- **OTel Weaver:** `/Users/sac/process-intelligence/otel-weaver/`
- **OpenTelemetry Docs:** https://opentelemetry.io/docs/reference/specification/
- **Van der Aalst Constitution:** `~/.claude/rules/process-mining-chicago-tdd.md`

---

**Gap Opened:** 2026-06-01  
**Authority:** Dr. Wil van der Aalst Process Mining Commission  
**Status:** OPEN (awaiting instrumentation)
