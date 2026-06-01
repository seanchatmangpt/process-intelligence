# Process Intelligence API — Open Ontologies Integration Guide

**Authority:** Process Intelligence Research Directorate  
**Version:** 1.0.0  
**Date:** 2026-06-01  
**Integration Partner:** Open Ontologies Community

---

## Overview

This document describes how the Process Intelligence Global Knowledge API integrates with the Open Ontologies ecosystem, enabling federated knowledge graph access, bidirectional OCEL streaming, and cross-project semantic queries.

---

## Integration Architecture

```
┌──────────────────────────────────────────────────────────────┐
│       Process Intelligence Global Knowledge API               │
│              (REST + SPARQL)                                  │
│                                                               │
│  ┌────────────────┐  ┌──────────────┐  ┌────────────────┐  │
│  │ Entity Catalog │  │ SPARQL Query │  │ Event Streaming│  │
│  │ (REST)         │  │ (Federated)  │  │ (OCEL 2.0)     │  │
│  └────────────────┘  └──────────────┘  └────────────────┘  │
│         ↓                   ↓                    ↓            │
│  ┌──────────────────────────────────────────────────────┐   │
│  │     Service Registry & Discovery                     │   │
│  │  (Open Ontologies Service Discovery Protocol)        │   │
│  └──────────────────────────────────────────────────────┘   │
│         ↓                                                     │
│  ┌──────────────────────────────────────────────────────┐   │
│  │     Federated Knowledge Graph Access                 │   │
│  │  (SPARQL endpoint federation + caching)              │   │
│  └──────────────────────────────────────────────────────┘   │
│         ↓                                                     │
│  ┌──────────────────────────────────────────────────────┐   │
│  │     Bidirectional OCEL Streaming                     │   │
│  │  (Kafka topic bridging + schema validation)          │   │
│  └──────────────────────────────────────────────────────┘   │
└──────────────────────────────────────────────────────────────┘
           ↓
┌──────────────────────────────────────────────────────────────┐
│  Open Ontologies Ecosystem                                    │
│                                                               │
│  ┌─────────────────┐  ┌──────────────┐  ┌──────────────┐   │
│  │ Ontology #1     │  │ Ontology #2  │  │ Ontology #3  │   │
│  │ (SPARQL)        │  │ (SPARQL)     │  │ (SPARQL)     │   │
│  └─────────────────┘  └──────────────┘  └──────────────┘   │
│         ↑                    ↑                    ↑           │
│  ┌─────────────────────────────────────────────────────────┐│
│  │  Service Registry (shared across ecosystem)             ││
│  │  Metadata: service ID, SPARQL endpoint, capabilities   ││
│  └─────────────────────────────────────────────────────────┘│
└──────────────────────────────────────────────────────────────┘
```

---

## Service Registration

### 1. Register in Service Registry

The Process Intelligence API registers itself with the Open Ontologies Service Registry for discoverability.

**Registration Metadata:**

```yaml
service_id: process-intelligence-api
version: 1.0.0
endpoint: https://api.process-intelligence.research/v1
description: |
  Unified API for Process Intelligence research ecosystem.
  Exposes entity catalog, federated SPARQL queries, OCEL events, 
  receipt verification, and provenance tracing.

capabilities:
  - type: entity_catalog
    method: REST
    endpoint: /entities
    operations: [list, get_by_id]
    formats: [JSON]
    
  - type: sparql_query
    method: POST
    endpoint: /sparql
    operations: [ASK, SELECT, CONSTRUCT]
    formats: [SPARQL Results JSON, Turtle]
    federation: true
    
  - type: event_streaming
    method: GET
    endpoint: /stream/ocel
    format: OCEL2.0
    protocol: Server-Sent Events
    
  - type: receipt_verification
    method: REST
    endpoint: /receipt/{id}
    format: JSON
    
  - type: provenance_tracing
    method: REST
    endpoint: /provenance/{id}
    format: JSON

ontologies:
  - Process Intelligence Doctrine (TTL)
  - Process Evidence Type Law (TTL)
  - Process Lifecycle Authority (TTL)
  - Van der Aalst Conformance Metrics (TTL)

authentication:
  methods: [api_key, jwt_bearer, mtls]
  
rate_limiting:
  enabled: true
  tiers: [free, standard, premium, enterprise]

health_check:
  endpoint: /health
  interval_seconds: 60

contact:
  email: api-support@process-intelligence.research
```

**Registration Command:**

```bash
# Register in Open Ontologies service registry
curl -X POST https://registry.open-ontologies.research/api/v1/services \
  -H "Authorization: Bearer $REGISTRY_TOKEN" \
  -H "Content-Type: application/yaml" \
  -d @pi-api-service-registration.yaml
```

---

## 2. Federated SPARQL Queries

### Cross-Ontology Queries

External clients can query the PI ontologies via federated SPARQL:

```sparql
PREFIX pi: <http://process-intelligence.research/pi#>
PREFIX ggen: <http://process-intelligence.research/ggen#>
PREFIX owl: <http://www.w3.org/2002/07/owl#>

# Local PI ontology query
SELECT ?artifact ?rule ?doctrine WHERE {
  ?artifact a ggen:Artifact ;
    ggen:generated-by ?rule .
  
  ?rule a ggen:GenerationRule ;
    ggen:sourced-from ?doctrine .
  
  # Federation: cross-query to external ontology
  SERVICE <https://ontology-external.open-ontologies.research/sparql> {
    ?doctrine a owl:Ontology ;
      owl:imports ?external_standard .
    
    ?external_standard owl:versionInfo ?version .
  }
}
```

### Query Timeout & Caching

- **Query Timeout:** 120 seconds (configurable per tier)
- **Result Caching:** 1 hour (TTL configurable)
- **Cache Invalidation:** Event-driven (cache cleared on entity update)

---

## 3. Bidirectional OCEL Streaming

### Inbound: Listen to PI Events

External services subscribe to PI event streams:

```python
import requests
import json

def subscribe_to_pi_events():
    """Subscribe to OCEL events from Process Intelligence"""
    response = requests.get(
        'https://api.process-intelligence.research/v1/stream/ocel',
        headers={'X-API-Key': 'your-api-key'},
        stream=True
    )
    
    for line in response.iter_lines():
        if line.startswith(b'data: '):
            event = json.loads(line[6:])
            
            # Process event
            print(f"Event: {event['ocel:activity']}")
            print(f"Objects: {event['ocel:object-refs']}")
            
            # Forward to local OCEL log
            forward_to_local_system(event)

if __name__ == '__main__':
    subscribe_to_pi_events()
```

### Outbound: Publish to PI Stream

External services publish events to PI:

```bash
# Kafka topic: pi-external-events
kafka-console-producer --bootstrap-server kafka-1.pi-cluster:9092 \
  --topic ocel.events.external \
  --property "parse.key=false"

# Send OCEL event JSON
{
  "ocel:event-id": "event:external-123",
  "ocel:activity": "external_process_started",
  "ocel:timestamp": "2026-06-01T16:00:00Z",
  "ocel:object-refs": [
    {"ocel:object-id": "proc:ext-001", "ocel:object-type": "ExternalProcess"}
  ]
}
```

### OCEL Stream Schema Validation

All events must conform to OCEL 2.0 spec. PI API validates:

```yaml
# OCEL 2.0 validation rules
validation:
  - field: ocel:event-id
    type: string
    required: true
    unique: true
    
  - field: ocel:activity
    type: string
    required: true
    
  - field: ocel:timestamp
    type: string (ISO 8601)
    required: true
    
  - field: ocel:object-refs
    type: array
    required: true
    min_items: 1
    
  - field: ocel:attributes
    type: object
    value_types: [string, number, boolean]
    
# Rejection handling
invalid_events:
  max_per_minute: 10
  action_at_threshold: alert_and_skip
```

---

## 4. Cross-Project Entity Linking

### Entity ID Schemes

Entities across projects can be linked via URL identifiers:

```
Process Intelligence entities:
  artifact: https://api.process-intelligence.research/entity/artifact:abc123
  rule:     https://api.process-intelligence.research/entity/rule:gen-001

External project entities:
  workflow: https://ontology-1.open-ontologies.research/entity/workflow:xyz789
  standard: https://ontology-2.open-ontologies.research/entity/standard:iso-001
```

### Entity Mapping

SPARQL queries can map entities:

```sparql
PREFIX pi: <http://process-intelligence.research/pi#>
PREFIX ext: <http://external-project.research/ext#>

SELECT ?pi_artifact ?ext_workflow WHERE {
  ?pi_artifact a pi:Artifact ;
    pi:conforms-to ?standard .
  
  SERVICE <https://ontology-external.open-ontologies.research/sparql> {
    ?ext_workflow a ext:Workflow ;
      ext:implements ?standard .
  }
  
  FILTER (?standard = ext:iso-27001)
}
```

---

## 5. Knowledge Graph Federation

### Remote Endpoint Configuration

PI API connects to external ontologies for federation:

```yaml
# api-server.yaml
integrations:
  federation:
    enabled: true
    remote_endpoints:
      - name: "Open Ontologies Hub 1"
        url: https://ontology-1.open-ontologies.research/sparql
        timeout_seconds: 60
        fallback: local_only
        
      - name: "Open Ontologies Hub 2"
        url: https://ontology-2.open-ontologies.research/sparql
        timeout_seconds: 60
        fallback: local_only
```

### Fallback Strategies

If remote endpoint fails:

```
1. Timeout: Return local results after 60s
2. 5xx Error: Retry with exponential backoff (1s, 2s, 4s)
3. Connection Error: Return cached results (if available)
4. Persistent Failure: Alert and disable endpoint temporarily
```

---

## 6. Conformance & Compliance

### API Compliance with Standards

- **OCEL 2.0:** Full compliance, validation enforced
- **SPARQL 1.1:** SELECT, ASK, CONSTRUCT support
- **OpenID Connect:** OAuth 2.0 integration
- **CORS:** Cross-origin requests enabled for web clients

### Capability Attestation

PI publishes its capabilities via capability ontology:

```ttl
@prefix pi: <http://process-intelligence.research/pi#> .
@prefix capa: <http://capability.open-ontologies.research/capa#> .

<https://api.process-intelligence.research/v1>
  a capa:Service ;
  capa:name "Process Intelligence Global Knowledge API" ;
  capa:endpoint <https://api.process-intelligence.research/v1> ;
  capa:supports_sparql true ;
  capa:supports_ocel_streaming true ;
  capa:supports_receipt_verification true ;
  capa:conformance_metrics (
    [ capa:metric "fitness" ; capa:min 0.95 ]
    [ capa:metric "precision" ; capa:min 0.90 ]
    [ capa:metric "simplicity" ; capa:min 0.80 ]
    [ capa:metric "generalization" ; capa:min 0.85 ]
  ) .
```

---

## 7. Integration Patterns

### Pattern 1: Research Data Export

Researcher wants to analyze PI artifacts using external tools:

```python
import requests

API = "https://api.process-intelligence.research/v1"
EXPORT = "https://ontology-external.open-ontologies.research/import"

# 1. Get all artifacts from PI
artifacts = requests.get(
    f"{API}/entities/Artifact",
    headers={"X-API-Key": key}
).json()["entities"]

# 2. Build RDF representation
rdf_data = """
@prefix pi: <http://process-intelligence.research/pi#> .
"""

for artifact in artifacts:
    provenance = requests.get(
        f"{API}/provenance/{artifact['id']}",
        headers={"X-API-Key": key}
    ).json()
    
    rdf_data += f"""
<{API}/entity/{artifact['id']}> 
  a pi:Artifact ;
  pi:name "{artifact['name']}" ;
  pi:sourced-from <{provenance['authorization']['checkpoint_id']}> .
"""

# 3. Export to external ontology
requests.post(
    f"{EXPORT}/datasets",
    data=rdf_data,
    headers={"Content-Type": "text/turtle"}
)
```

---

### Pattern 2: Cross-Project Conformance Analysis

Analyst checks if external processes conform to PI standards:

```sparql
PREFIX pi: <http://process-intelligence.research/pi#>
PREFIX ext: <http://external-project.research/ext#>

# Query: Find external artifacts that DON'T conform to PI standards
SELECT ?ext_artifact ?ext_name WHERE {
  SERVICE <https://external-project.open-ontologies.research/sparql> {
    ?ext_artifact a ext:Artifact ;
      ext:name ?ext_name ;
      ext:event-log ?log .
  }
  
  # Check conformance against PI standards
  OPTIONAL {
    ?pi_standard a pi:ConformanceStandard ;
      pi:fitness_min 0.95 .
    
    # This would require calling PI conformance service
    # (integration point for future enhancement)
  }
}
```

---

## 8. Security & Access Control

### Service-to-Service Authentication

External services authenticate via OAuth 2.0:

```bash
# 1. Register external service with PI
curl -X POST https://api.process-intelligence.research/v1/admin/services \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -d '{
    "name": "external-research-platform",
    "client_id": "ext-platform-123",
    "scopes": ["entities:read", "sparql:execute", "stream:read"]
  }'

# 2. Obtain service credentials
# Response includes client_secret (never share!)

# 3. Authenticate API calls
curl -X POST https://auth.process-intelligence.research/oauth/token \
  -d "grant_type=client_credentials&client_id=ext-platform-123&client_secret=..."
```

### RBAC for Cross-Project Access

Fine-grained permissions for external services:

```yaml
# Grant external service read access to public entities only
service: external-research-platform
scope: entities:read
constraints:
  - entity_type: Artifact
    visibility: public  # Only public artifacts
  - entity_type: Receipt
    visibility: public
  
  # Deny access to internal doctrine
  - entity_type: Doctrine
    visibility: private
```

---

## 9. Monitoring & Health Checks

### Service Health Endpoint

External services monitor PI availability:

```bash
curl https://api.process-intelligence.research/v1/health | jq .

# Response:
{
  "status": "healthy",
  "systems": {
    "zoeapp": { "status": "healthy", "uptime_seconds": 86400 },
    "blue-river-dam": { "status": "healthy", "uptime_seconds": 86400 },
    ...
  },
  "gates": [
    { "gate_id": 1, "status": "PASS" },
    ...
  ]
}
```

### SLA Monitoring

PI publishes SLA metrics via Prometheus:

```
# Availability
pi_api_uptime_percent{tier="standard"} 99.0
pi_api_uptime_percent{tier="premium"} 99.5
pi_api_uptime_percent{tier="enterprise"} 99.9

# Latency
pi_api_p50_latency_ms{endpoint="/v1/entities"} 45
pi_api_p99_latency_ms{endpoint="/v1/entities"} 250

# SPARQL federation
pi_api_sparql_federation_success_rate 0.987
pi_api_sparql_federation_avg_latency_ms 2450
```

---

## 10. Troubleshooting Integration Issues

### Issue: Federated Query Times Out

**Symptom:** SPARQL queries with SERVICE clauses return 504

**Solution:**
1. Check remote endpoint availability: `curl https://remote-endpoint.org/sparql`
2. Reduce query complexity (limit result sets)
3. Increase timeout (requires premium tier or custom SLA)
4. Use caching for repeated queries

### Issue: OCEL Event Validation Fails

**Symptom:** Events rejected with "schema validation error"

**Solution:**
1. Validate OCEL structure against OCEL 2.0 spec
2. Ensure all required fields present (ocel:event-id, ocel:activity, ocel:timestamp)
3. Check attribute types (string, number, boolean only)
4. Test with example event first

### Issue: Service Registration Not Discovered

**Symptom:** Service doesn't appear in registry

**Solution:**
1. Verify service endpoint is reachable
2. Check /health endpoint returns 200
3. Ensure service is registered with correct metadata
4. Contact registry admin: registry@open-ontologies.research

---

## Support

- **Integration Questions:** integration@process-intelligence.research
- **Technical Issues:** api-support@process-intelligence.research
- **Service Registry:** registry@open-ontologies.research
- **Slack Channel:** #process-intelligence-integration (invite required)

---

**Authority:** Process Intelligence Research Directorate  
**Last Updated:** 2026-06-01  
**Next Review:** 2026-09-01
