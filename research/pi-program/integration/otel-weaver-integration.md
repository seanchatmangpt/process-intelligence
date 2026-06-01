# Integration Point 3: All Systems ↔ OTel Weaver

**Authority:** Process Intelligence Research Foundry  
**Status:** Integration Specification  
**Date:** 2026-06-01

---

## Purpose

OpenTelemetry (OTel) Weaver provides semantic convention validation and telemetry normalization for all systems (ZOEapp, Blue River Dam, Expo/Supabase). This integration establishes the **feedstock ingestion pipeline**: raw OTel traces → Weaver semantic validation → OCEL 2.0 event log → Conformance checking.

OTel Weaver is **not** process evidence. Traces are feedstock. Only when normalized to OCEL and replayed against the declared process model do they become admissible process evidence.

---

## Architecture

```
┌──────────────────────────────────────┐
│     ZOEapp (Proof Cell)              │
│  - Emits OTel spans for each stage   │
│  - process.start, stage.complete     │
│  - Sends to OTel Collector (gRPC)    │
└──────────────────────────────────────┘
           ↓
┌──────────────────────────────────────┐
│  Blue River Dam (Governor)           │
│  - Emits governance.decision spans   │
│  - Sends to OTel Collector (gRPC)    │
└──────────────────────────────────────┘
           ↓
┌──────────────────────────────────────┐
│  Expo/Supabase (Mobile UX)           │
│  - Emits ui.action, data.mutation    │
│  - Sends to OTel Collector (gRPC)    │
└──────────────────────────────────────┘
           ↓
┌──────────────────────────────────────────────┐
│      OTel Collector (Aggregation Point)      │
│  - Endpoint: 0.0.0.0:4317 (gRPC)             │
│  - Protocol: OTLP (OpenTelemetry Protocol)   │
│  - Memory limit: 512 MB                      │
└──────────────────────────────────────────────┘
           ↓
┌──────────────────────────────────────────────┐
│    Weaver Semantic Convention Validator      │
│  - Validate span attributes against schema   │
│  - Check for required attributes per span    │
│  - Validate attribute types and formats      │
│  - Emit validation.result{Pass|Fail}         │
└──────────────────────────────────────────────┘
           ↓
┌──────────────────────────────────────────────┐
│    OCEL 2.0 Event Log Transformer            │
│  - Map OTel spans to OCEL events             │
│  - Extract object identifiers (process_id)   │
│  - Create event timestamps + attributes      │
│  - Output OCEL JSON-LD                       │
└──────────────────────────────────────────────┘
           ↓
┌──────────────────────────────────────────────┐
│    Conformance Checker (wasm4pm)             │
│  - Load declared process model (PNML)        │
│  - Replay OCEL events                        │
│  - Calculate conformance fitness             │
│  - Detect drift patterns                     │
│  - Emit conformance_report.json              │
└──────────────────────────────────────────────┘
           ↓
┌──────────────────────────────────────────────┐
│    Blue River Dam (Governance Loop)          │
│  - Receive conformance_report                │
│  - If fitness < 0.95: trigger repair cycle  │
│  - Otherwise: continue monitoring            │
└──────────────────────────────────────────────┘
```

---

## Instrumentation Standards

All systems must emit OTel spans using the following standard:

### 1. ZOEapp Instrumentation

```rust
// zoeapp/src/telemetry.rs

use opentelemetry::sdk::trace::TracerProvider;
use opentelemetry::trace::Tracer;
use opentelemetry_otlp::WithExportConfig;

pub fn init_tracer() -> TracerProvider {
    let exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint("http://otel-collector:4317");
    
    TracerProvider::builder()
        .with_exporter(exporter)
        .with_resource(Resource::new(vec![
            KeyValue::new("service.name", "zoeapp"),
            KeyValue::new("service.version", "1.0.0"),
        ]))
        .build()
}

pub struct ProcessEventTracer {
    tracer: Tracer,
}

impl ProcessEventTracer {
    pub fn span_process_start(&self, process_id: &str) -> Span {
        let span = self.tracer.start("process.start");
        span.set_attribute("process.id", process_id.to_string());
        span.set_attribute("process.type", "OrderToInvoice"); // From ZOEapp config
        span.set_attribute("otel.library.name", "process-intelligence");
        span.set_attribute("otel.library.version", "1.0.0");
        span
    }

    pub fn span_stage_begin(&self, process_id: &str, stage_name: &str, span_parent: &Span) -> Span {
        let span = self.tracer.start_with_context("stage.begin", span_parent);
        span.set_attribute("process.id", process_id);
        span.set_attribute("stage.name", stage_name);
        span.set_attribute("stage.start_time", format!("{}", Utc::now().to_rfc3339()));
        span
    }

    pub fn span_stage_complete(&self, process_id: &str, stage_name: &str, duration_ms: u64, span_parent: &Span) -> Span {
        let span = self.tracer.start_with_context("stage.complete", span_parent);
        span.set_attribute("process.id", process_id);
        span.set_attribute("stage.name", stage_name);
        span.set_attribute("stage.duration_ms", duration_ms as f64);
        span.set_attribute("stage.status", "success");
        span
    }

    pub fn span_stage_failed(&self, process_id: &str, stage_name: &str, error: &str, span_parent: &Span) -> Span {
        let span = self.tracer.start_with_context("stage.failed", span_parent);
        span.set_attribute("process.id", process_id);
        span.set_attribute("stage.name", stage_name);
        span.set_attribute("stage.status", "failed");
        span.set_attribute("error.type", error);
        span.record_exception(&Exception::from(error));
        span
    }
}

// Usage in process execution
pub async fn execute_order_fulfillment(tracer: &ProcessEventTracer, order_id: &str) -> Result<()> {
    let root_span = tracer.span_process_start(order_id);
    let _guard = root_span.clone();
    
    // Stage 1: OrderReceived
    let stage1_span = tracer.span_stage_begin(order_id, "OrderReceived", &root_span);
    let start = Instant::now();
    // ... execute stage ...
    let duration = start.elapsed().as_millis() as u64;
    tracer.span_stage_complete(order_id, "OrderReceived", duration, &root_span);
    stage1_span.end();
    
    // Stage 2: PaymentProcessing
    let stage2_span = tracer.span_stage_begin(order_id, "PaymentProcessing", &root_span);
    let start = Instant::now();
    match process_payment(order_id).await {
        Ok(receipt) => {
            let duration = start.elapsed().as_millis() as u64;
            tracer.span_stage_complete(order_id, "PaymentProcessing", duration, &root_span);
        }
        Err(e) => {
            tracer.span_stage_failed(order_id, "PaymentProcessing", &e.to_string(), &root_span);
            return Err(e);
        }
    }
    stage2_span.end();
    
    root_span.end();
    Ok(())
}
```

### 2. Blue River Dam Instrumentation

```rust
// blue_river_dam/src/telemetry.rs

pub struct GovernanceTracer {
    tracer: Tracer,
}

impl GovernanceTracer {
    pub fn span_monitor(&self, process_id: &str) -> Span {
        let span = self.tracer.start("governance.monitor");
        span.set_attribute("process.id", process_id);
        span.set_attribute("mape_k.phase", "monitor");
        span
    }

    pub fn span_analyze(&self, process_id: &str, conformance_fitness: f64) -> Span {
        let span = self.tracer.start("governance.analyze");
        span.set_attribute("process.id", process_id);
        span.set_attribute("mape_k.phase", "analyze");
        span.set_attribute("conformance.fitness", conformance_fitness);
        span.set_attribute("governance.drift_detected", conformance_fitness < 0.95);
        span
    }

    pub fn span_plan(&self, process_id: &str, correction_type: &str) -> Span {
        let span = self.tracer.start("governance.plan");
        span.set_attribute("process.id", process_id);
        span.set_attribute("mape_k.phase", "plan");
        span.set_attribute("governance.correction_type", correction_type); // "none" | "elastic" | "compliance"
        span
    }

    pub fn span_execute_decision(&self, process_id: &str, decision: &str) -> Span {
        let span = self.tracer.start("governance.execute");
        span.set_attribute("process.id", process_id);
        span.set_attribute("mape_k.phase", "execute");
        span.set_attribute("governance.decision", decision); // "proceed" | "refuse" | "restrict"
        span
    }

    pub fn span_refusal(&self, process_id: &str, reason: &str, signature: &str) -> Span {
        let span = self.tracer.start("governance.refusal");
        span.set_attribute("process.id", process_id);
        span.set_attribute("governance.action", "refuse");
        span.set_attribute("governance.reason", reason);
        span.set_attribute("governance.signature", signature); // Ed25519 sig for audit
        span
    }
}
```

### 3. Expo Instrumentation

```typescript
// expo/src/telemetry/opentelemetry.ts

import * as Sentry from '@sentry/react-native';
import { getResource } from '@opentelemetry/resources';
import { TracerProvider } from '@opentelemetry/sdk-trace-react-native';

const resource = getResource({ 'service.name': 'expo-app', 'service.version': '1.0.0' });
const tracerProvider = new TracerProvider({ resource });

export const tracer = tracerProvider.getTracer('expo-app', '1.0.0');

export function spanUIAction(actionType: string, processId: string) {
    const span = tracer.startSpan('ui.action');
    span.setAttributes({
        'process.id': processId,
        'ui.action.type': actionType, // "button_click", "form_submit", etc.
        'ui.component': currentComponent,
        'ui.timestamp': new Date().toISOString(),
    });
    return span;
}

export function spanDataMutation(operation: string, resource: string, processId: string) {
    const span = tracer.startSpan('data.mutation');
    span.setAttributes({
        'process.id': processId,
        'data.operation': operation, // "create", "update", "delete"
        'data.resource': resource,   // "process_instances", "governance_context"
        'data.timestamp': new Date().toISOString(),
    });
    return span;
}

// Usage in component
export function ProcessStageButton({ processId, targetStage }) {
    const handlePress = () => {
        const span = spanUIAction('stage_transition_attempt', processId);
        
        try {
            // Send mutation
            const mutationSpan = spanDataMutation('update', 'process_instances', processId);
            // ... Supabase mutation ...
            mutationSpan.end();
        } catch (error) {
            span.recordException(error);
        } finally {
            span.end();
        }
    };
    
    return <TouchableOpacity onPress={handlePress}>Transition</TouchableOpacity>;
}
```

---

## Weaver Semantic Conventions

Define semantic conventions for all span types:

```yaml
# weaver/semantic-conventions.yaml

attributes:
  process.id:
    brief: "Process instance identifier"
    type: "string"
    pattern: "^proc_[a-zA-Z0-9]{16}$"
    required: ["process.start", "stage.begin", "stage.complete", "governance.monitor"]

  stage.name:
    brief: "Process stage name"
    type: "string"
    pattern: "^[A-Za-z][A-Za-z0-9]*$"
    required: ["stage.begin", "stage.complete", "stage.failed"]

  stage.duration_ms:
    brief: "Stage execution duration in milliseconds"
    type: "number"
    min: 0
    required: ["stage.complete"]

  otel.trace_id:
    brief: "OpenTelemetry W3C Trace ID"
    type: "string"
    pattern: "^0x[a-f0-9]{32}$"
    required: []

  conformance.fitness:
    brief: "Process conformance fitness score (0-1)"
    type: "number"
    min: 0
    max: 1
    required: ["governance.analyze"]

  governance.decision:
    brief: "Governance decision type"
    type: "string"
    enum: ["proceed", "refuse", "restrict"]
    required: ["governance.execute"]

  governance.signature:
    brief: "Ed25519 signature for audit"
    type: "string"
    pattern: "^[a-f0-9]{128}$"
    required: ["governance.refusal"]

events:
  process.start:
    brief: "Process instance started"
    attributes:
      - process.id [required]
      - process.type [required]
      - otel.library.name [required]

  stage.begin:
    brief: "Process stage started"
    attributes:
      - process.id [required]
      - stage.name [required]
      - stage.start_time [optional]

  stage.complete:
    brief: "Process stage completed successfully"
    attributes:
      - process.id [required]
      - stage.name [required]
      - stage.duration_ms [required]
      - stage.status: "success" [required]

  stage.failed:
    brief: "Process stage failed"
    attributes:
      - process.id [required]
      - stage.name [required]
      - stage.status: "failed" [required]
      - error.type [required]

  governance.monitor:
    brief: "Blue River monitoring phase"
    attributes:
      - process.id [required]
      - mape_k.phase: "monitor" [required]

  governance.analyze:
    brief: "Blue River analysis phase"
    attributes:
      - process.id [required]
      - conformance.fitness [required]
      - governance.drift_detected [required]

  governance.execute:
    brief: "Blue River execution phase"
    attributes:
      - process.id [required]
      - governance.decision [required]

  governance.refusal:
    brief: "Governance refusal of non-compliant transition"
    attributes:
      - process.id [required]
      - governance.action: "refuse" [required]
      - governance.reason [required]
      - governance.signature [required]
```

---

## OCEL Transformation Pipeline

Transform OTel traces to OCEL 2.0 event logs:

```python
# otel-weaver/otel_to_ocel.py

import json
from typing import List, Dict, Any
from datetime import datetime

class OTelToOCEL:
    def __init__(self, semantic_conventions: Dict[str, Any]):
        self.semantic_conventions = semantic_conventions

    def transform(self, traces: List[Dict[str, Any]]) -> Dict[str, Any]:
        """Transform OTel traces to OCEL 2.0 format"""
        
        events = []
        object_types = set()
        
        # Sort traces by timestamp for OCEL ordering
        sorted_traces = sorted(traces, key=lambda t: t['timestamp'])
        
        for i, trace in enumerate(sorted_traces):
            # Extract required fields
            span_name = trace.get('span_name', 'unknown')
            process_id = trace.get('attributes', {}).get('process.id')
            timestamp = trace.get('timestamp')
            
            if not process_id:
                # Skip traces without process.id
                continue
            
            # Validate span attributes against semantic conventions
            if not self._validate_span(span_name, trace):
                # Emit non-compliant event with partial_data flag
                events.append({
                    'ocel:eid': f"evt_{i}",
                    'ocel:activity': span_name,
                    'ocel:timestamp': timestamp,
                    'ocel:omap': [{'ocel:oid': f"process:{process_id}", 'ocel:otype': 'process'}],
                    'ocel:vmap': {
                        'ocel:partial_data': True,
                        'ocel:validation_error': 'missing_required_attributes'
                    }
                })
                continue
            
            # Create OCEL event
            omap = [{'ocel:oid': f"process:{process_id}", 'ocel:otype': 'process'}]
            object_types.add('process')
            
            # Add other object types based on span type
            if 'governance' in span_name:
                omap.append({
                    'ocel:oid': f"governance:{trace['attributes'].get('governance.decision', 'unknown')}",
                    'ocel:otype': 'governance_action'
                })
                object_types.add('governance_action')
            
            # Extract relevant attributes
            vmap = {
                'otel.trace_id': trace.get('trace_id'),
                'otel.span_id': trace.get('span_id'),
            }
            
            # Add span-specific attributes
            for key, value in trace.get('attributes', {}).items():
                if key.startswith('otel.') or key.startswith('conformance.') or key.startswith('governance.'):
                    vmap[key] = value
            
            events.append({
                'ocel:eid': f"evt_{i}",
                'ocel:activity': span_name,
                'ocel:timestamp': timestamp,
                'ocel:omap': omap,
                'ocel:vmap': vmap
            })
        
        # Build OCEL structure
        ocel = {
            'ocel:global': {
                'ocel:version': '2.0',
                'ocel:attribute-names': list(set(
                    attr for event in events for attr in event['ocel:vmap'].keys()
                )),
                'ocel:attribute-types': ['string'] * len(set(
                    attr for event in events for attr in event['ocel:vmap'].keys()
                )),
                'ocel:object-types': list(object_types),
            },
            'ocel:events': events
        }
        
        return ocel

    def _validate_span(self, span_name: str, trace: Dict[str, Any]) -> bool:
        """Validate OTel trace against semantic conventions"""
        if span_name not in self.semantic_conventions.get('events', {}):
            return False
        
        required_attrs = self.semantic_conventions['events'][span_name].get('attributes', [])
        trace_attrs = trace.get('attributes', {})
        
        for req_attr in required_attrs:
            if req_attr.endswith('[required]'):
                attr_name = req_attr.split(' ')[0]
                if attr_name not in trace_attrs:
                    return False
        
        return True
```

---

## Conformance Checking

After OCEL generation, run conformance check:

```bash
#!/bin/bash
# otel-weaver/conformance-pipeline.sh

# 1. Collect OTel traces
TRACES_FILE="/tmp/otel_traces.json"
curl -s http://otel-collector:5888/api/traces | jq . > $TRACES_FILE

# 2. Validate semantic conventions
python3 otel-weaver/semantic_validator.py \
    --traces $TRACES_FILE \
    --conventions weaver/semantic-conventions.yaml \
    --output /tmp/validation_report.json

# 3. Transform to OCEL
python3 otel-weaver/otel_to_ocel.py \
    --traces $TRACES_FILE \
    --output /tmp/event_log.ocel

# 4. Run conformance check
./wasm4pm conformance \
    --event_log /tmp/event_log.ocel \
    --model declared_process_model.pnml \
    --output /tmp/conformance_report.json \
    --detailed

# 5. Analyze conformance metrics
FITNESS=$(jq '.fitness' /tmp/conformance_report.json)
if (( $(echo "$FITNESS < 0.95" | bc -l) )); then
    echo "DRIFT DETECTED: fitness = $FITNESS"
    # Trigger Blue River governance cycle
    curl -X POST http://blue-river:5000/trigger-repair \
        -d "{ \"conformance_report\": $(cat /tmp/conformance_report.json) }"
else
    echo "CONFORMANCE OK: fitness = $FITNESS"
fi
```

---

## Error Handling

| Scenario | Detection | Recovery |
|----------|-----------|----------|
| OTel trace missing process.id | Semantic validation fails | Emit non-compliant event with partial_data flag |
| Weaver semantic validation fails | Validation report contains errors | Log violation, continue (don't block) |
| OCEL transformation fails | Python error | Log exception, return empty OCEL |
| Conformance check timeout (>5min) | wasm4pm process killed | Retry with smaller event window |
| Network error to OTel Collector | gRPC connection fails | Retry with exponential backoff |

---

## Acceptance Criteria

- [ ] All systems (ZOEapp, Blue River, Expo) emit OTel spans with required attributes
- [ ] Weaver semantic convention validator accepts all valid spans
- [ ] OCEL transformation produces valid OCEL 2.0 JSON-LD
- [ ] Conformance checker produces fitness score (0-1)
- [ ] Drift detection (fitness < 0.95) triggers Blue River repair cycle
- [ ] Non-compliant spans are marked with ocel:partial_data flag
- [ ] OTel to OCEL pipeline completes within 5 minutes
- [ ] All error cases are logged and don't block processing

---

**Authority:** Process Intelligence Research Foundry  
**Status:** Integration Specification (Ready for Implementation)
