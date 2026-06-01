# Phase 20: Ecosystem Integration Architecture

**Authority:** Process Intelligence Research Foundry (ALIVE_001)  
**Status:** Architecture Planning & Specification  
**Updated:** 2026-06-01

---

## Executive Summary

Phase 20 integrates five independent systems into a unified **Process Intelligence Ecosystem**:

| System | Role | Authority |
|--------|------|-----------|
| **ZOEapp** | Proof Cell | Real-world process execution (source of evidence) |
| **Blue River Dam** | Autonomic Governor | MAPE-K feedback control loop (lifecycle authority) |
| **Expo/Supabase Framework** | Mobile UX & Data Layer | User experience + policy enforcement (RLS constraints) |
| **OTel Weaver** | Telemetry Semantics | Span/metric ingestion + OCEL normalization (feedstock) |
| **ggen Manufacturing** | Unified Generation | Single source of truth (TTL ontologies → artifacts) |

The ecosystem follows a **layered architecture** where each system operates at its assigned level, with explicit **data and control flow contracts** between layers.

---

## Unified Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────────┐
│           Claude Code Orchestration Layer                              │
│  (Dynamic workflows, health checks, scaling, remediation scheduling)   │
│                                                                         │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │ Workflow Health Monitor: Startup → Drift Detection → Healing     │  │
│  │ Scheduling: Conformance checks (5min), MAPE cycles (10min)       │  │
│  │ Scaling: Auto-scale based on process volume + rework rate        │  │
│  └──────────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────────┘
                                      ↓
┌─────────────────────────────────────────────────────────────────────────┐
│                   Integration Backbone (API Gateway)                    │
│  ┌──────────────────────────────┬──────────────────────────────────┐  │
│  │ Event Subscription Service   │ Policy Sync Service             │  │
│  │ (Pub/Sub for lifecycle)      │ (RLS ↔ Blue River decisions)    │  │
│  └──────────────────────────────┴──────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────────┘
           ↓                              ↓                           ↓
┌──────────────────┐       ┌──────────────────────┐      ┌──────────────────┐
│    ZOEapp        │       │  Blue River Dam      │      │  Expo/Supabase   │
│  (Proof Cell)    │←─────→│  (Autonomic Governor)│←────→│  (Mobile UX)     │
│                  │       │                      │      │                  │
│ - Process exe    │       │ - Monitor (evidenc)  │      │ - Auth (RLS)     │
│ - Event capture  │       │ - Analyze (drift)    │      │ - Role policies  │
│ - Telemetry emit │       │ - Plan (corrections) │      │ - UX governance  │
│ - Refusal logic  │       │ - Execute (actuate)  │      │ - Data storage   │
│                  │       │ - Knowledge (repair) │      │                  │
└──────────────────┘       └──────────────────────┘      └──────────────────┘
           ↓                              ↓                           ↓
         OTel Traces         MAPE-K Decisions                 RLS Policy Acts
           ↓                              ↓                           ↓
┌────────────────────────────────────────────────────────────────────────┐
│            OTel Weaver Registry (Telemetry Normalization)              │
│                                                                        │
│  ┌─────────────────────────────────────────────────────────────────┐  │
│  │ Semantic Validation: Schema conformance, attribute mapping       │  │
│  │ OCEL Event Log Pipeline: Traces → OCEL 2.0 object events        │  │
│  │ Drift Signals: Conformance fitness, variant discovery            │  │
│  └─────────────────────────────────────────────────────────────────┘  │
└────────────────────────────────────────────────────────────────────────┘
           ↓
┌────────────────────────────────────────────────────────────────────────┐
│          ggen Unified Manufacturing Pipeline (TTL → Artifacts)         │
│                                                                        │
│  ┌─────────────────────────────────────────────────────────────────┐  │
│  │ Ontology Authority: TTL facts (process law, type law)           │  │
│  │ Generation Rules: ggen.toml + SPARQL queries                    │  │
│  │ Artifact Synthesis: TERA templates → JSON, MD, RS, artifacts    │  │
│  │ Receipt Chain: BLAKE3 hashes for all generated artifacts        │  │
│  └─────────────────────────────────────────────────────────────────┘  │
└────────────────────────────────────────────────────────────────────────┘
```

---

## Integration Points (5 Wiring Contracts)

### Integration Point 1: ZOEapp ↔ Blue River Dam

**Contract:** Real-world proof cell connects to autonomic governor via lifecycle event subscription.

#### ZOEapp Interface (Producer)
```typescript
// src/lib.rs (ZOEapp)
pub trait ProcessEventSource {
    fn subscribe_lifecycle_events(&self) -> EventStream;
    // Events: ProcessStarted, StageCompleted, StageSkipped, ErrorDetected, ProcessTerminated
    
    fn expose_blue_river_api(&self) -> BlueRiverApi {
        BlueRiverApi {
            base_url: "http://zoeapp:3000/blue-river",
            health_check: "/health",
            refusal_endpoint: "/refuse", // Refuse non-compliant transitions
            telemetry_sink: "/otel/traces"
        }
    }
}
```

#### Blue River Dam Interface (Consumer)
```rust
// src/lib.rs (Blue River Dam)
impl Governor {
    pub fn connect_proof_cell(&mut self, zoeapp_api: &BlueRiverApi) -> Result<(), GovError> {
        // Subscribe to ZOEapp lifecycle events
        let events = zoeapp_api.subscribe_lifecycle_events();
        
        // For each event: Monitor → Analyze → Plan → Execute → Knowledge
        for event in events {
            self.monitor(&event)?;        // Capture as Evidence
            self.analyze()?;                // Drift detection
            self.plan_correction()?;        // Generate Plan artifact
            self.execute_correction(event)?; // Send refusal or proceed signal
            self.update_knowledge()?;       // Persist repair outcome
        }
        Ok(())
    }
}
```

**Data Flow:**
```
ZOEapp Process Event (JSON)
    ↓
    {
        "process_id": "proc_123",
        "stage_name": "OrderFulfillment",
        "timestamp": "2026-06-01T14:30:00Z",
        "state": "InProgress|Completed|Failed|Refused",
        "otel_trace_id": "0x4bf92f3577b34da6a3ce929d0e0e4736"
    }
    ↓
Blue River Dam Monitor (Evidence<ProcessEvent, State, Witness>)
    ↓
Analyze: Is stage_name in declared model? Is timestamp logically consistent?
    ↓
Plan: If drift detected → Send /refuse to ZOEapp
    ↓
Execute: ZOEapp receives refusal, stops execution (no re-attempt)
    ↓
Knowledge: Store repair decision + outcome in Blue River registry
```

**Error Handling:**
- ZOEapp network error → Blue River reverts to last-known state, logs gap
- Signature mismatch on refusal command → ZOEapp ignores, escalates to operator
- Blue River overloaded → ZOEapp buffers events (max 1000 in-flight), retries with exponential backoff

---

### Integration Point 2: Blue River Dam ↔ Expo/Supabase Framework

**Contract:** Autonomic governor deploys governance policies to mobile UX via RLS policy sync.

#### Blue River Output (Governance Decisions)
```rust
pub struct GovernancePolicy {
    pub policy_id: String,                          // "gov_pol_456"
    pub lifecycle_state: LifecycleState,             // Design|Simulate|Monitor|Repair|Optimize
    pub allowed_roles: Vec<String>,                  // ["operator", "auditor"]
    pub forbidden_transitions: Vec<(State, State)>, // [(Pending, Terminal)]
    pub rate_limit: RateLimit,                       // { per_minute: 100 }
    pub receipt: Receipt,                            // BLAKE3 hash + Ed25519 signature
}

pub fn emit_governance_policy(&self, policy: GovernancePolicy) -> Result<String, GovError> {
    // 1. Sign with governor's key
    let sig = sign_ed25519(&policy, &self.signing_key);
    
    // 2. POST to Supabase policy endpoint
    let policy_json = serde_json::to_string(&policy)?;
    let response = http_post(
        "https://supabase.example.com/api/governance/policies",
        &policy_json,
        &sig
    )?;
    
    // 3. Return policy_id for tracking
    Ok(response.policy_id)
}
```

#### Supabase RLS Enforcer (Consumer)
```sql
-- supabase/rls/process_governance.sql

-- Enforce Blue River governance policies at the RLS level
CREATE POLICY enforce_blue_river_lifecycle
    ON public.process_instances
    USING (
        EXISTS (
            SELECT 1 FROM public.governance_policies gp
            WHERE gp.policy_id = current_setting('app.active_policy_id')
            AND gp.allowed_roles && string_to_array(current_setting('app.user_role'), ',')
            AND gp.lifecycle_state = (
                SELECT lifecycle_state FROM public.process_instances pi
                WHERE pi.id = process_instances.id
            )
        )
    )
    WITH CHECK (
        NOT EXISTS (
            SELECT 1 FROM public.governance_policies gp
            WHERE gp.policy_id = current_setting('app.active_policy_id')
            AND (
                (process_instances.current_state, process_instances.next_state)
                IN (SELECT from_state, to_state FROM gp.forbidden_transitions)
            )
        )
    );

-- Enforce rate limits from Blue River
CREATE POLICY enforce_blue_river_rate_limit
    ON public.process_events
    USING (
        (
            SELECT COUNT(*) FROM public.process_events pe
            WHERE pe.user_id = auth.uid()
            AND pe.created_at > now() - interval '1 minute'
        ) < (
            SELECT (gp.rate_limit ->> 'per_minute')::int
            FROM public.governance_policies gp
            WHERE gp.policy_id = current_setting('app.active_policy_id')
        )
    );
```

**Policy Sync Cycle (10-minute heartbeat):**
```
Blue River Governor (Every 10 minutes):
    1. Query current governance state
    2. Serialize all active policies + signatures
    3. POST to Supabase policy endpoint
    4. Receive policy_id + confirmation receipt
    5. Store receipt in Blue River knowledge registry

Expo App (On each user action):
    1. Fetch current policies from Supabase (cached, 5-min TTL)
    2. Apply RLS filters in Supabase
    3. Display allowed transitions in UI
    4. Block disallowed state changes at both API and DB level
```

**Error Handling:**
- Supabase unreachable → Blue River continues local governance, logs drift alert
- Policy signature invalid → Supabase rejects update, Blue River retries with escalation
- RLS conflict with app logic → Log incident, operator must reconcile manually

---

### Integration Point 3: All Systems ↔ OTel Weaver

**Contract:** All systems emit OTel traces; Weaver normalizes to OCEL event log format.

#### Instrumentation Pattern (All Systems)

```rust
// In ZOEapp, Blue River, Expo backend: Initialize OTel
use opentelemetry::sdk::trace::TracerProvider;
use opentelemetry::trace::Tracer;

let tracer_provider = TracerProvider::builder()
    .with_exporter(
        opentelemetry_otlp::new_exporter()
            .tonic()
            .with_endpoint("http://otel-weaver:4317")
    )
    .build();

let tracer = tracer_provider.tracer("process-intelligence");

// Example: ZOEapp emits process start event
pub fn start_process(&self, process_id: &str) {
    let span = tracer.start("process.start");
    span.set_attribute("process.id", process_id);
    span.set_attribute("process.type", "OrderFulfillment");
    span.set_attribute("otel.library.name", "process-intelligence");
    // ... rest of execution
    span.end();
}

// Example: Blue River emits governance decision
pub fn execute_refusal(&self, process_id: &str, reason: &str) {
    let span = tracer.start("governance.refusal");
    span.set_attribute("governance.action", "refuse");
    span.set_attribute("process.id", process_id);
    span.set_attribute("governance.reason", reason);
    span.set_attribute("governance.authority", "blue_river_dam");
    span.end();
}
```

#### OTel Weaver Normalization Pipeline

```bash
# otel-weaver/pipeline.sh

# Step 1: Collect OTel traces from all systems
otel-collector \
    --receivers.otlp.protocols.grpc.endpoint=0.0.0.0:4317 \
    --exporters.logging.loglevel=debug

# Step 2: Validate against Weaver semantic conventions
weaver manifest validate \
    --manifest ./semantic-conventions.yaml \
    --traces ./traces.json

# Step 3: Transform to OCEL 2.0 format
python3 otel-to-ocel.py \
    --input traces.json \
    --output event_log.ocel \
    --schema ocel-2.0.json

# Step 4: Ingest into conformance checker
wasm4pm conformance \
    --event_log event_log.ocel \
    --model declared_process_model.pnml \
    --output conformance_report.json
```

**OCEL Event Log Schema (Generated from OTel):**
```json
{
  "ocel:global": {
    "ocel:version": "2.0",
    "ocel:attribute-names": ["timestamp", "otel.trace_id", "governance.authority"],
    "ocel:attribute-types": ["timestamp", "string", "string"],
    "ocel:object-types": ["process", "stage", "governance_action"]
  },
  "ocel:events": [
    {
      "ocel:eid": "evt_1",
      "ocel:activity": "process.start",
      "ocel:timestamp": "2026-06-01T14:30:00Z",
      "ocel:omap": [
        {"ocel:oid": "process:proc_123", "ocel:otype": "process"}
      ],
      "ocel:vmap": {
        "otel.trace_id": "0x4bf92f3577b34da6a3ce929d0e0e4736"
      }
    },
    {
      "ocel:eid": "evt_2",
      "ocel:activity": "governance.refusal",
      "ocel:timestamp": "2026-06-01T14:30:45Z",
      "ocel:omap": [
        {"ocel:oid": "process:proc_123", "ocel:otype": "process"},
        {"ocel:oid": "governance:gov_act_1", "ocel:otype": "governance_action"}
      ],
      "ocel:vmap": {
        "governance.authority": "blue_river_dam",
        "otel.trace_id": "0x5cf93g4678c45eb7b4df940f1f1f5837"
      }
    }
  ]
}
```

**Error Handling:**
- Weaver schema validation fails → Event logged as "ocel:non_compliant", human review required
- OTel trace missing required attributes → Event marked with `ocel:partial_data = true`
- Conformance check fails on OCEL → Alert Blue River to investigate (possible execution bug)

---

### Integration Point 4: All Systems ↔ Claude Workflows

**Contract:** Orchestration layer manages system lifecycle via dynamic workflow rules.

#### Workflow Orchestration (Claude Code)

```yaml
# workflow-orchestration.yaml

workflows:
  - name: "system-startup"
    triggers: ["manual", "cron(0 8 * * *)", "deployment.complete"]
    steps:
      - step: "health-check"
        action: "parallel"
        tasks:
          - "zoeapp.health_check()"
          - "blue_river.health_check()"
          - "expo.health_check()"
          - "otel_weaver.health_check()"
        timeout: 30s
        on_failure: "escalate_to_operator"
      
      - step: "initialize-connections"
        action: "sequence"
        tasks:
          - "blue_river.connect_proof_cell(zoeapp_api)"
          - "blue_river.sync_governance_policies(supabase)"
          - "otel_weaver.subscribe(zoeapp.otel_sink)"
          - "otel_weaver.subscribe(blue_river.otel_sink)"
        timeout: 60s
        on_failure: "rollback_startup"
      
      - step: "verify-system-integrity"
        action: "parallel"
        tasks:
          - "validate_otel_traces_flow()"
          - "validate_governance_policy_sync()"
          - "validate_refusal_path()"
        timeout: 60s
        on_failure: "notify_team"

  - name: "drift-detection-cycle"
    triggers: ["cron(*/5 * * * *)", "manual"]
    steps:
      - step: "collect-ocel-events"
        action: "fetch"
        source: "otel_weaver.ocel_event_log"
        since: "last_collection_time"
        timeout: 30s

      - step: "run-conformance-check"
        action: "invoke"
        service: "wasm4pm"
        method: "conformance_check"
        params:
          event_log: "{{ocel_events}}"
          model: "declared_process_model.pnml"
        timeout: 120s
        on_failure: "log_conformance_gap"

      - step: "analyze-fitness-metrics"
        action: "compute"
        formula: "fitness = 1 - (cost_moves / (cost_moves + cost_moves_empty))"
        threshold: 0.95
        if_below_threshold: "alert_drift"

      - step: "if-drift-detected"
        condition: "fitness < 0.95"
        then:
          - step: "invoke-blue-river-repair"
            action: "invoke"
            service: "blue_river_dam"
            method: "analyze_and_plan"
            params:
              conformance_report: "{{conformance_report}}"
            timeout: 300s
          
          - step: "execute-governance-correction"
            action: "invoke"
            service: "blue_river_dam"
            method: "execute_correction"
            params:
              plan: "{{repair_plan}}"
            timeout: 300s

  - name: "dashboard-refresh"
    triggers: ["cron(*/1 * * * *)", "manual"]
    steps:
      - step: "fetch-latest-metrics"
        action: "query"
        sources:
          - "blue_river.governance_state"
          - "otel_weaver.conformance_metrics"
          - "zoeapp.process_metrics"
      
      - step: "render-dashboards"
        action: "invoke"
        service: "ggen"
        template: "dashboard.tera"
        params:
          metrics: "{{metrics}}"
          policy_state: "{{governance_state}}"
      
      - step: "push-to-expo"
        action: "broadcast"
        channels:
          - "expo.push_notifications"
          - "supabase.realtime_broadcasts"
        payload: "{{dashboard_json}}"
```

**Error Handling & Remediation:**
- Workflow step timeout → Retry with exponential backoff (3 retries), then escalate
- Service unreachable → Mark as "DEGRADED", continue with cached state
- Data consistency violation → Trigger manual reconciliation workflow + operator alert

---

### Integration Point 5: All Systems → ggen Manufacturing

**Contract:** Unified ggen.toml orchestrates artifact generation from TTL ontologies.

#### Unified ggen.toml Configuration

```toml
# ggen/ggen.toml

[ontology]
# Single source of truth: TTL law files
authority = "ttl-ontologies"
files = [
    "ontology/process-law.ttl",       # Process model definitions
    "ontology/type-law.ttl",          # Evidence<T, State, Witness> lattice
    "ontology/lifecycle-law.ttl",     # MAPE-K state transitions
    "ontology/governance-law.ttl",    # Blue River policies + rules
    "ontology/weaver-law.ttl"         # OTel semantic mappings
]

[generation]
# One pipeline for all artifacts
pipeline = "unified"

[[generation.rules]]
name = "process-model-definition"
trigger = "process-law.ttl updated"
sparql = """
  PREFIX proc: <http://process-intelligence/process#>
  SELECT ?processId ?stageName ?nextStage
  WHERE {
    ?p proc:id ?processId .
    ?p proc:hasStage ?stage .
    ?stage proc:name ?stageName .
    ?stage proc:nextStage ?nextStage .
  }
"""
template = "templates/process-model.tera"
output = "artifacts/declared_process_model.pnml"
receipt = true

[[generation.rules]]
name = "governance-policy-generation"
trigger = "governance-law.ttl updated"
sparql = """
  PREFIX gov: <http://process-intelligence/governance#>
  SELECT ?policyId ?allowedRole ?forbiddenFrom ?forbiddenTo
  WHERE {
    ?pol gov:policyId ?policyId .
    ?pol gov:allowedRole ?allowedRole .
    ?pol gov:forbidden ?forbid .
    ?forbid gov:fromState ?forbiddenFrom .
    ?forbid gov:toState ?forbiddenTo .
  }
"""
template = "templates/governance-policy.tera"
output = "artifacts/governance_policies.json"
receipt = true
sign_with_key = "blue_river_governor_key"

[[generation.rules]]
name = "otel-semantic-conventions"
trigger = "weaver-law.ttl updated"
sparql = """
  PREFIX weaver: <http://process-intelligence/weaver#>
  SELECT ?spanName ?requiredAttribute ?attributeType
  WHERE {
    ?span weaver:spanName ?spanName .
    ?span weaver:requires ?attr .
    ?attr weaver:name ?requiredAttribute .
    ?attr weaver:type ?attributeType .
  }
"""
template = "templates/otel-conventions.tera"
output = "artifacts/semantic-conventions.yaml"
receipt = true

[[generation.rules]]
name = "lifecycle-state-machine"
trigger = "lifecycle-law.ttl updated"
sparql = """
  PREFIX lifecycle: <http://process-intelligence/lifecycle#>
  SELECT ?state ?nextState ?condition
  WHERE {
    ?s lifecycle:state ?state .
    ?s lifecycle:transition ?t .
    ?t lifecycle:to ?nextState .
    ?t lifecycle:condition ?condition .
  }
"""
template = "templates/lifecycle-sm.tera"
output = "artifacts/lifecycle_state_machine.rs"
receipt = true
compile_check = true

[[generation.rules]]
name = "m-and-a-board-claims"
trigger = "governance-law.ttl updated OR process-law.ttl updated"
sparql = """
  PREFIX ma: <http://process-intelligence/ma#>
  PREFIX proc: <http://process-intelligence/process#>
  SELECT ?claimType ?metric ?value ?evidence
  WHERE {
    ?claim ma:type ?claimType .
    ?claim ma:metric ?metric .
    ?claim ma:value ?value .
    ?claim ma:evidenced_by ?evidence .
  }
"""
template = "templates/m-and-a-deck.tera"
output = "artifacts/m-and-a-projections.json"
receipt = true
sign_with_key = "board_audit_key"

[receipt_chain]
# All generated artifacts are cryptographically chained
algorithm = "BLAKE3"
chain_file = "receipts/ggen-receipt-chain.json"
include_timestamp = true
include_ontology_hash = true

[validation]
# Post-generation checks
[[validation.check]]
name = "process-model-executable"
cmd = "wasm4pm validate --model {output}"
on_failure = "reject_artifact"

[[validation.check]]
name = "governance-policy-signature"
cmd = "verify-signature --file {output} --key blue_river_governor_key"
on_failure = "reject_artifact"

[[validation.check]]
name = "lifecycle-state-machine-sound"
cmd = "petri-net-analyzer --model {output} --check soundness"
on_failure = "reject_artifact"

[deployment]
# Coordinate artifact distribution to all systems
[[deployment.target]]
name = "zoeapp"
artifacts = ["declared_process_model.pnml"]
endpoint = "http://zoeapp:3000/deploy/model"
trigger = "after_receipt_chain"

[[deployment.target]]
name = "blue_river_dam"
artifacts = ["governance_policies.json", "lifecycle_state_machine.rs"]
endpoint = "http://blue_river:5000/deploy/policies"
trigger = "after_receipt_chain"

[[deployment.target]]
name = "expo_supabase"
artifacts = ["governance_policies.json"]
endpoint = "https://supabase.example.com/api/governance/deploy"
trigger = "after_receipt_chain"

[[deployment.target]]
name = "otel_weaver"
artifacts = ["semantic-conventions.yaml"]
endpoint = "http://otel-weaver:8080/deploy/conventions"
trigger = "after_receipt_chain"
```

#### TERA Template Example: Governance Policy Generation

```tera
{# templates/governance-policy.tera #}
{
  "policies": [
    {% for row in results %}
    {
      "policy_id": "{{ row.policyId }}",
      "allowed_roles": ["{{ row.allowedRole }}"],
      "forbidden_transitions": [
        {% set forbidden = query_forbidden_for(row.policyId) %}
        {% for f in forbidden %}
        {
          "from_state": "{{ f.forbiddenFrom }}",
          "to_state": "{{ f.forbiddenTo }}"
        }{{ "," if not loop.last else "" }}
        {% endfor %}
      ],
      "rate_limit": {
        "per_minute": {{ get_rate_limit(row.policyId) }}
      },
      "lifecycle_state": "{{ get_lifecycle_state(row.policyId) }}",
      "receipt": {
        "generated_at": "{{ now() }}",
        "ontology_hash": "{{ ontology_hash }}",
        "artifact_hash": "{{ blake3(self) }}",
        "signature": "{{ sign_ed25519(self, blue_river_governor_key) }}"
      }
    }{{ "," if not loop.last else "" }}
    {% endfor %}
  ]
}
```

---

## Data Flow Contracts

### Contract 1: Process Event Flow (ZOEapp → Blue River → Expo)

```
ZOEapp Process Execution
  ↓ (OTel Span)
  {
    "span_name": "stage.execute",
    "process_id": "proc_123",
    "stage_name": "OrderFulfillment",
    "status": "InProgress|Completed|Failed",
    "timestamp": "2026-06-01T14:30:00Z",
    "attributes": {
      "otel.library.name": "process-intelligence",
      "process.type": "OrderToInvoice"
    }
  }
  ↓
  Blue River Monitor (Evidence<ProcessEvent>)
  ↓
  Conformance Check via OCEL
  ↓
  Governance Policy Query
  ↓
  If drift or policy violation:
    Blue River sends /refuse to ZOEapp
    ZOEapp stops execution, awaits manual override
    Blue River emits OTel span (governance.refusal)
  Else:
    Blue River acknowledges, continues monitoring
  ↓
  Expo receives policy state update (RLS enforced)
  ↓
  Operator sees dashboard updated in real-time
```

### Contract 2: Governance Policy Sync Flow (Blue River → Supabase → Expo)

```
Blue River MAPE-K Cycle (Every 10 min)
  ↓
  Analyze: detect_drift()
  ↓
  Plan: generate_correction_policies()
  ↓
  Execute: emit_governance_policy() → POST to Supabase
  ↓
  Supabase receives policy + Ed25519 signature
  ↓
  Verify signature against blue_river_governor_key
  ↓
  Update RLS policies in PostgreSQL
  ↓
  Supabase broadcasts policy update via Realtime
  ↓
  Expo app receives update, refreshes RLS context
  ↓
  User UI reflects new constraints
```

### Contract 3: OCEL Event Log Flow (All Systems → OTel → Weaver → Conformance)

```
ZOEapp OTel Span
  ↓ (gRPC/HTTP to OTel Collector)
  Blue River OTel Span
  ↓
  Expo OTel Span
  ↓
  OTel Collector (Aggregates traces)
  ↓
  Weaver Semantic Validation
  ↓
  Transform to OCEL 2.0
  ↓
  Conformance Check (wasm4pm)
    - Load declared_process_model.pnml
    - Replay events against model
    - Calculate fitness
    - Generate conformance_report.json
  ↓
  Alert Blue River if fitness < 0.95
  ↓
  Trigger drift-detection-cycle workflow
```

---

## System Dependencies & Failure Modes

### Dependency Matrix

| Dependent | Dependency | Impact if Failed |
|-----------|-----------|------------------|
| Blue River | ZOEapp (proof cell) | Governance loop cannot execute, system becomes passive |
| Expo | Blue River (policies) | UI allows unsafe transitions, operator must manually enforce |
| OTel Weaver | All systems (OTel emitters) | Conformance checks incomplete, blind to drift |
| ggen | TTL ontologies | Cannot generate artifacts, stale models deployed |
| Claude Workflow | All systems | No health monitoring, manual operator-initiated only |

### Failure Recovery Strategies

| Failure | Detection | Recovery |
|---------|-----------|----------|
| ZOEapp unreachable | Blue River health check fails | Switch to "governor-only" mode; monitor via logs only |
| Supabase network error | Policy sync returns HTTP 503 | Blue River retries with exponential backoff; keep local policy cache |
| OTel Weaver overloaded | Trace ingestion latency > 5min | Buffer traces locally (max 100k), drop oldest if buffer full, alert operator |
| ggen artifact generation fails | Validation check fails | Reject artifact, keep previous version, log incident |
| Claude Workflow timeout | Step exceeds timeout threshold | Retry up to 3 times, escalate to operator, declare system "DEGRADED" |

---

## Acceptance Gate Criteria

### Gate 1: Integration Architecture Complete
- [ ] All 5 integration points have wiring contracts (data format, API, error handling)
- [ ] Unified architecture diagram is complete and reviewed
- [ ] All systems are documented with their roles and interfaces

### Gate 2: Data Flow Verified
- [ ] Process event flow (ZOEapp → Blue River → Expo) traced end-to-end
- [ ] Governance policy sync (Blue River → Supabase) tested
- [ ] OCEL event log pipeline (All systems → OTel → Weaver → Conformance) functional
- [ ] ggen artifact generation rules produce valid artifacts (with receipt chain)

### Gate 3: Error Handling Specified
- [ ] Network failures have explicit recovery strategies
- [ ] Signature mismatches are logged and escalated
- [ ] Rate limiting is enforced at Supabase RLS layer
- [ ] Conformance failures trigger Blue River repairs

### Gate 4: Unified ggen.toml Executable
- [ ] All TTL ontologies ingested successfully
- [ ] All SPARQL queries return expected results
- [ ] All TERA templates render without errors
- [ ] All generated artifacts pass validation checks
- [ ] Receipt chain is unbroken and cryptographically valid

### Gate 5: Documentation Complete
- [ ] All 5 integration points documented in separate files
- [ ] Workflow orchestration rules defined in YAML
- [ ] Data contracts specified in JSON schema
- [ ] Error recovery procedures documented

---

## Next Steps (Phase 20 Subtasks)

1. **Document ZOEapp ↔ Blue River Integration** → `zoeapp-blue-river-integration.md`
2. **Document Blue River ↔ Supabase Integration** → `blue-river-supabase-integration.md`
3. **Document OTel Weaver Integration** → `otel-weaver-integration.md`
4. **Document Claude Workflow Orchestration** → `claude-workflow-orchestration.md`
5. **Implement Unified ggen.toml** → `unified-ggen-ecosystem.toml`
6. **Implement Workflow YAML** → `workflow-orchestration.yaml`
7. **Test all integration points** → Integration test suite

---

**Authority:** Process Intelligence Research Foundry  
**Status:** Architecture Specification (Ready for Implementation)  
**Receipt:** [Pending: Generate after all integration docs complete]
