# Process Intelligence Global Knowledge API — Implementation Guide

**Authority:** Process Intelligence Research Directorate  
**Version:** 1.0.0  
**Date:** 2026-06-01  
**Status:** Phase 24 Product Hardening & Market Launch

---

## Table of Contents

1. [Overview](#overview)
2. [Architecture](#architecture)
3. [API Endpoints](#api-endpoints)
4. [Authentication](#authentication)
5. [Rate Limiting](#rate-limiting)
6. [Integration Patterns](#integration-patterns)
7. [Error Handling](#error-handling)
8. [Monitoring & Observability](#monitoring--observability)
9. [Deployment](#deployment)
10. [Testing](#testing)

---

## Overview

The Process Intelligence Global Knowledge API exposes the unified ecosystem as a public, open-standards-compatible interface. It serves five primary constituencies:

1. **Open Ontologies Community** — Federated knowledge graph access
2. **External Researchers** — OCEL stream consumption, conformance checking
3. **Downstream Products** — ggen validation, Blue River lifecycle governance
4. **Process Mining Platforms** — PM4Py integration, event log export
5. **Compliance/Audit Systems** — Receipt verification, provenance tracing

### Core Principles

- **Evidence-Driven:** Every claim traces to source documentation or cryptographic receipt
- **Immutable Record:** All operational history recorded in OCEL 2.0 event logs
- **Compliant:** Van der Aalst Constitution enforced; fitness metrics certified
- **Federated:** Knowledge graph accessed via SPARQL; cross-project queries enabled
- **Transparent:** All API calls logged, audited, and receipt-chained

---

## Architecture

### System Components

```
┌──────────────────────────────────────────────────────┐
│     Process Intelligence Global Knowledge API        │
│                  (REST + SPARQL)                     │
├──────────────────────────────────────────────────────┤
│                                                      │
│  Routes:                                             │
│  - Entity catalog (GET /entities)                    │
│  - SPARQL federated queries (POST /sparql)           │
│  - Event streaming (GET /stream/ocel)                │
│  - Receipt verification (GET /receipt/{id})          │
│  - Provenance tracing (GET /provenance/{id})         │
│  - Conformance checking (POST /conformance/check)    │
│  - Health/metrics (GET /health, /metrics)            │
│                                                      │
├──────────────────────────────────────────────────────┤
│                 Authentication Layer                 │
│  (API Keys, JWT Bearer, Rate Limiting)               │
├──────────────────────────────────────────────────────┤
│                                                      │
│  Data Layer:                                         │
│  - RDF4J (knowledge graph)                           │
│  - PostgreSQL (event logs)                           │
│  - SQLite (immutable receipt chain)                  │
│  - Redis (caching, rate limiting)                    │
│  - Kafka (bidirectional OCEL streaming)              │
│                                                      │
└──────────────────────────────────────────────────────┘
```

### Data Flow

```
External Client
     ↓
[API Gateway: Auth + Rate Limiting]
     ↓
[Route Handler]
     ├─→ Entity Lookup: RDF4J query
     ├─→ Event Streaming: Kafka subscription
     ├─→ SPARQL: RDF4J + federation
     ├─→ Receipts: SQLite chain lookup
     ├─→ Provenance: SPARQL proof graph
     └─→ Conformance: PM4Py service call
     ↓
[Response Transformation]
     ├─→ JSON serialization
     ├─→ OCEL 2.0 format (if applicable)
     └─→ Receipt signature (Blake3)
     ↓
External Client
```

---

## API Endpoints

### 1. Health & System Status

#### `GET /v1/health`

Returns ecosystem health across all 5 systems and 30 deployment gates.

**Response:**
```json
{
  "status": "healthy|degraded|critical",
  "timestamp": "2026-06-01T15:30:00Z",
  "systems": {
    "zoeapp": { "status": "healthy", "uptime_seconds": 86400 },
    "blue-river-dam": { "status": "healthy", "uptime_seconds": 86400 },
    "ggen": { "status": "healthy", "uptime_seconds": 86400 },
    "otel-weaver": { "status": "healthy", "uptime_seconds": 86400 },
    "expo-supabase": { "status": "healthy", "uptime_seconds": 86400 }
  },
  "gates": [
    { "gate_id": 1, "name": "ggen validator deployed", "status": "PASS" },
    { "gate_id": 2, "name": "15 startup health checks", "status": "PASS" },
    ...
  ]
}
```

**Use Case:** Kubernetes liveness probes, dashboard monitoring

---

#### `GET /v1/metrics`

Van der Aalst conformance metrics (fitness, precision, simplicity, generalization).

**Response:**
```json
{
  "timestamp": "2026-06-01T15:30:00Z",
  "measurement_date": "2026-06-01",
  "fitness": 0.98,
  "precision": 0.95,
  "simplicity": 0.88,
  "generalization": 0.92,
  "overall_status": "PASS",
  "thresholds": {
    "fitness_min": 0.95,
    "precision_min": 0.90,
    "simplicity_min": 0.80,
    "generalization_min": 0.85
  },
  "receipt_id": "blake3:abc123..."
}
```

---

### 2. Entity Management

#### `GET /v1/entities?type=Artifact&limit=10&offset=0`

List all entities or filter by type.

**Response:**
```json
{
  "entities": [
    {
      "id": "artifact:abc123",
      "type": "Artifact",
      "name": "ggen-validation-rule-001",
      "created_at": "2026-06-01T15:00:00Z",
      "updated_at": "2026-06-01T15:00:00Z",
      "content_hash": "blake3:..."
    }
  ],
  "total": 523,
  "limit": 10,
  "offset": 0
}
```

---

#### `GET /v1/entity/{id}`

Get entity details with provenance trace.

**Response:**
```json
{
  "id": "artifact:abc123",
  "type": "Artifact",
  "name": "ggen-validation-rule-001",
  "created_at": "2026-06-01T15:00:00Z",
  "content_hash": "blake3:...",
  "provenance": {
    "artifact_id": "artifact:abc123",
    "generated_by": "rule:generate-validation",
    "rule_definition": {
      "rule_id": "rule:generate-validation",
      "rule_name": "Generate Validation Rules",
      "sparql_query": "SELECT ?rule WHERE ...",
      "template_name": "validation-rule.tera"
    },
    "sourced_from": [
      {
        "source_type": "doctrine",
        "source_id": "doctrine/process-mining-chicago-tdd.md",
        "claim": "Van der Aalst Constitution",
        "citation": "If code says it worked but event log cannot prove..."
      }
    ],
    "authorization": {
      "checkpoint_id": "PHASE_18_OTEL_INSTRUMENTATION_ALIVE_001",
      "verdict": "ALIVE",
      "authority": "Process Intelligence Research Directorate"
    }
  },
  "receipts": [
    {
      "receipt_id": "blake3:def456",
      "type": "artifact_created",
      "issued_at": "2026-06-01T15:00:00Z",
      "verified": true
    }
  ]
}
```

---

### 3. SPARQL Queries

#### `POST /v1/sparql`

Execute federated SPARQL queries against the unified knowledge graph.

**Request:**
```
Content-Type: application/sparql-query

ASK {
  ?artifact a ggen:Artifact ;
    ggen:generated-by ?rule .
  
  ?rule a ggen:GenerationRule ;
    ggen:sourced-from ?doctrine .
  
  ?doctrine a pi:Doctrine ;
    dc:title "Process Intelligence Doctrine" .
}
```

**Response (ASK):**
```json
{
  "boolean": true
}
```

**Response (SELECT):**
```json
{
  "head": {
    "vars": ["artifact", "rule", "doctrine"]
  },
  "results": {
    "bindings": [
      {
        "artifact": { "type": "uri", "value": "artifact:abc123" },
        "rule": { "type": "uri", "value": "rule:gen-001" },
        "doctrine": { "type": "uri", "value": "doctrine:001" }
      }
    ]
  }
}
```

**Use Case:** Research queries, compliance verification, knowledge graph analysis

---

### 4. Event Streaming

#### `GET /v1/stream/ocel?since=2026-06-01T00:00:00Z&object-type=Artifact`

Stream OCEL 2.0 events using Server-Sent Events (SSE).

**Response (text/event-stream):**
```
event: ocel-event
data: {
  "ocel:event-id": "event:12345",
  "ocel:activity": "artifact_created",
  "ocel:timestamp": "2026-06-01T15:30:45Z",
  "ocel:object-refs": [
    {
      "ocel:object-id": "artifact:abc123",
      "ocel:object-type": "Artifact"
    }
  ],
  "ocel:attributes": {
    "generator": "rule:gen-001",
    "status": "completed"
  }
}

event: ocel-event
data: {
  "ocel:event-id": "event:12346",
  ...
}
```

**Use Case:** Real-time process monitoring, external log aggregation, live dashboards

---

### 5. Receipt Verification

#### `GET /v1/receipt/{receipt-id}`

Retrieve and verify cryptographic receipt.

**Response:**
```json
{
  "receipt_id": "blake3:abc123...",
  "type": "artifact_created",
  "artifact_id": "artifact:xyz789",
  "content_hash": "blake3:def456...",
  "signature": "blake3-signature:...",
  "issued_at": "2026-06-01T15:00:00Z",
  "verified": true,
  "chain_parent": "blake3:parent123...",
  "chain_next": "blake3:next456...",
  "verification_details": {
    "signature_valid": true,
    "chain_valid": true,
    "timestamp_valid": true,
    "artifact_exists": true
  }
}
```

---

### 6. Provenance Tracing

#### `GET /v1/provenance/{artifact-id}`

Trace artifact back to source law and authorization.

**Response:**
```json
{
  "artifact_id": "artifact:abc123",
  "artifact_type": "Validation Rule",
  "generated_by": "rule:generate-validation",
  "rule_definition": {
    "rule_id": "rule:generate-validation",
    "rule_name": "Generate Validation Rules",
    "sparql_query": "SELECT ?rule WHERE ...",
    "template_name": "validation-rule.tera"
  },
  "sourced_from": [
    {
      "source_type": "doctrine",
      "source_id": "doctrine/process-mining-chicago-tdd.md",
      "claim": "Van der Aalst Constitution: If the code says it worked...",
      "citation": "Doctrine section 2.1"
    },
    {
      "source_type": "paper",
      "source_id": "papers/van-der-aalst-2016-process-mining.pdf",
      "claim": "Process mining conformance checking methodology",
      "citation": "Van der Aalst et al., Section 4.3"
    }
  ],
  "authorization": {
    "checkpoint_id": "PHASE_18_OTEL_INSTRUMENTATION_ALIVE_001",
    "verdict": "ALIVE",
    "authority": "Process Intelligence Research Directorate"
  },
  "evidence_chain": [
    {
      "type": "gateway_verification",
      "gate_id": 7,
      "gate_name": "OCEL Event Log Generated",
      "status": "PASS",
      "evidence_path": "/research/pi-program/audits/pi-ecosystem-ocel.json"
    },
    {
      "type": "conformance_metric",
      "metric": "fitness",
      "value": 0.98,
      "threshold": 0.95,
      "status": "PASS"
    }
  ]
}
```

---

#### `POST /v1/provenance/search`

Search provenance graph by criteria.

**Request:**
```json
{
  "claim": "Van der Aalst Constitution",
  "source_type": "doctrine",
  "artifact_type": "ValidationRule",
  "time_range": {
    "start": "2026-06-01T00:00:00Z",
    "end": "2026-06-30T23:59:59Z"
  }
}
```

**Response:**
```json
{
  "results": [
    { "artifact_id": "...", "claim": "...", "source_id": "..." },
    ...
  ],
  "total": 42
}
```

---

### 7. Conformance Checking

#### `POST /v1/conformance/check`

Run conformance check via PM4Py integration.

**Request:**
```json
{
  "event_log": {
    "ocel:global-event": [...],
    "ocel:events": [...],
    "ocel:objects": [...]
  },
  "process_model": {
    "places": [...],
    "transitions": [...],
    "arcs": [...]
  },
  "include_negative_tests": false
}
```

**Response:**
```json
{
  "fitness": 0.97,
  "precision": 0.94,
  "simplicity": 0.85,
  "generalization": 0.91,
  "status": "PASS",
  "receipt_id": "blake3:conformance123...",
  "timestamp": "2026-06-01T15:30:00Z"
}
```

---

## Authentication

### API Key Authentication

```bash
curl -H "X-API-Key: your-api-key-here" \
  https://api.process-intelligence.research/v1/health
```

### Bearer Token (JWT)

```bash
curl -H "Authorization: Bearer eyJhbGc..." \
  https://api.process-intelligence.research/v1/health
```

### Obtaining Credentials

1. **API Key:** Contact research-directorate@process-intelligence.research
2. **JWT Token:** Authenticate via OAuth 2.0 at https://auth.process-intelligence.research

### RBAC Scopes

- `entities:read` — Read entity catalog
- `entities:write` — Create/update entities (admin)
- `sparql:execute` — Execute SPARQL queries
- `stream:read` — Subscribe to event streams
- `receipts:read` — Access receipt chain
- `provenance:read` — Trace provenance
- `conformance:execute` — Run conformance checks
- `lifecycle:read` — View lifecycle definitions
- `system:admin` — Administrative operations

---

## Rate Limiting

### Per-Tier Limits

| Tier | Req/Min | Req/Hour | Concurrent |
|------|---------|----------|-----------|
| Free | 10 | 100 | 1 |
| Standard | 100 | 5K | 10 |
| Premium | 1K | 100K | 100 |
| Enterprise | ∞ | ∞ | ∞ |

### Per-Endpoint Overrides

- **SPARQL:** 60/min (complex queries)
- **Conformance:** 30/min (PM4Py timeout: 300s)
- **OCEL Stream:** Unlimited (SSE streaming)
- **Entity Lookup:** 500/min (high throughput)

### Rate Limit Headers

```
RateLimit-Limit: 600
RateLimit-Remaining: 423
RateLimit-Reset: 1685705400
Retry-After: 60
```

### Exceeding Limits

```json
HTTP 429 Too Many Requests

{
  "error": "rate_limit_exceeded",
  "retry_after_seconds": 60,
  "reset_time": "2026-06-01T15:31:00Z"
}
```

---

## Integration Patterns

### Pattern 1: Entity Discovery

```python
import requests

api = "https://api.process-intelligence.research/v1"
headers = {"X-API-Key": "your-key"}

# List all artifacts
resp = requests.get(f"{api}/entities/Artifact?limit=100", headers=headers)
artifacts = resp.json()["entities"]

# Get provenance for each
for artifact in artifacts:
    prov = requests.get(f"{api}/provenance/{artifact['id']}", headers=headers)
    print(f"{artifact['name']} ← {prov.json()['sourced_from']}")
```

### Pattern 2: SPARQL Federated Query

```python
sparql_query = """
SELECT ?artifact ?rule ?doctrine WHERE {
  ?artifact a ggen:Artifact ;
    ggen:generated-by ?rule .
  
  ?rule a ggen:GenerationRule ;
    ggen:sourced-from ?doctrine .
  
  ?doctrine a pi:Doctrine .
  
  FILTER(?doctrine = <doctrine:process-mining-chicago-tdd>)
}
"""

resp = requests.post(
  f"{api}/sparql",
  headers={"X-API-Key": "your-key", "Content-Type": "application/sparql-query"},
  data=sparql_query
)
```

### Pattern 3: Event Stream Subscription

```python
import sseclient

url = f"{api}/stream/ocel?since=2026-06-01T00:00:00Z&object-type=Artifact"
response = requests.get(url, headers=headers, stream=True)

for event in sseclient.SSEClient(response):
    if event.event == "ocel-event":
        ocel_event = json.loads(event.data)
        print(f"Event: {ocel_event['ocel:activity']}")
```

### Pattern 4: Receipt Verification Chain

```python
# Start with artifact
artifact = requests.get(f"{api}/entity/{artifact_id}", headers=headers).json()
receipt_id = artifact['receipts'][0]['receipt_id']

# Trace receipt chain
current_receipt = requests.get(f"{api}/receipt/{receipt_id}", headers=headers).json()
while current_receipt.get('chain_next'):
  print(f"Receipt: {current_receipt['receipt_id']} → {current_receipt['chain_next']}")
  current_receipt = requests.get(
    f"{api}/receipt/{current_receipt['chain_next']}",
    headers=headers
  ).json()
```

---

## Error Handling

### Standard Error Response

```json
{
  "error": "error_code",
  "message": "Human-readable description",
  "request_id": "req:12345",
  "timestamp": "2026-06-01T15:30:00Z"
}
```

### Common Errors

| Code | Status | Meaning |
|------|--------|---------|
| `invalid_request` | 400 | Malformed request |
| `authentication_failed` | 401 | Invalid credentials |
| `access_denied` | 403 | Insufficient permissions |
| `not_found` | 404 | Resource not found |
| `rate_limit_exceeded` | 429 | Rate limit exceeded |
| `server_error` | 500 | Internal server error |
| `service_unavailable` | 503 | Knowledge graph offline |

---

## Monitoring & Observability

### Logging

All API calls logged to:
- **Access logs:** `/var/log/pi-api/access.log`
- **Error logs:** `/var/log/pi-api/error.log`
- **Audit logs:** `/var/log/pi-api/audit.log` (7-year retention)

### Metrics

OpenTelemetry metrics exported to Prometheus:
- `api.requests.total` (counter)
- `api.request.duration_ms` (histogram)
- `api.rate_limit.exceeded` (counter)
- `api.sparql.query_duration_ms` (histogram)

### Tracing

Distributed tracing via Jaeger:
- Request ID propagated via `traceparent` header
- All spans include request path, method, status code
- Service-to-service calls traced end-to-end

### Alerts

Slack notifications on:
- API errors (5xx rate > 1%)
- Rate limit abuse (>100 429s per minute)
- SPARQL timeout (>10s execution)
- Knowledge graph unavailability

---

## Deployment

### Prerequisites

- Kubernetes cluster (1.27+)
- RDF4J repository deployed
- PostgreSQL instance (event logs)
- Redis cluster (caching, rate limiting)
- Kafka brokers (OCEL streaming)
- PM4Py service deployed

### Installation

```bash
# 1. Create namespace
kubectl create namespace pi-api

# 2. Deploy secrets
kubectl create secret generic pi-api-secrets \
  --from-file=tls.crt=/path/to/cert \
  --from-file=tls.key=/path/to/key \
  --from-literal=db-password=... \
  -n pi-api

# 3. Deploy ConfigMap
kubectl create configmap pi-api-config \
  --from-file=openapi.yaml \
  --from-file=api-server.yaml \
  --from-file=rate-limiter.yaml \
  -n pi-api

# 4. Deploy API server
kubectl apply -f deployment.yaml -n pi-api

# 5. Verify startup
kubectl wait --for=condition=Ready pod -l app=pi-api -n pi-api --timeout=300s
```

### Kubernetes Configuration

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: pi-api
  namespace: pi-api
spec:
  replicas: 2
  template:
    spec:
      containers:
      - name: api
        image: pi-api:1.0.0
        ports:
        - containerPort: 8080
        env:
        - name: DB_USER
          valueFrom:
            secretKeyRef:
              name: pi-api-secrets
              key: db-user
        volumeMounts:
        - name: config
          mountPath: /etc/pi-api
        livenessProbe:
          httpGet:
            path: /v1/health
            port: 8080
          initialDelaySeconds: 10
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /v1/health/readiness
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
      volumes:
      - name: config
        configMap:
          name: pi-api-config
---
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: pi-api-hpa
  namespace: pi-api
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: pi-api
  minReplicas: 2
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
```

---

## Testing

### Unit Tests

```bash
# Test authentication
pytest tests/unit/test_auth.py

# Test rate limiting
pytest tests/unit/test_rate_limiting.py

# Test entity serialization
pytest tests/unit/test_entities.py
```

### Integration Tests

```bash
# Test SPARQL queries
pytest tests/integration/test_sparql.py

# Test receipt verification
pytest tests/integration/test_receipts.py

# Test provenance tracing
pytest tests/integration/test_provenance.py
```

### Load Testing

```bash
# Locust load test
locust -f tests/load/locustfile.py --host=https://api.process-intelligence.research

# k6 load test
k6 run tests/load/api-load-test.js
```

### Chaos Engineering

```bash
# Inject faults via Gremlin
chaos run tests/chaos/knowledge-graph-unavailable.yaml

# Verify graceful degradation
curl https://api.process-intelligence.research/v1/health
```

---

## Support

- **Documentation:** https://docs.process-intelligence.research
- **Issues:** https://github.com/process-intelligence/api/issues
- **Email:** api-support@process-intelligence.research
- **Slack:** #process-intelligence-api (invite required)

---

**Authority:** Process Intelligence Research Directorate  
**Last Updated:** 2026-06-01  
**Next Review:** 2026-07-01
