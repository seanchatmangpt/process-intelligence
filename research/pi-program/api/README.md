# Process Intelligence Global Knowledge API

**Authority:** Process Intelligence Research Directorate  
**Version:** 1.0.0  
**Date:** 2026-06-01  
**Status:** Phase 24 Product Hardening & Market Launch

---

## Overview

The Process Intelligence Global Knowledge API is a unified REST + SPARQL interface that exposes the entire process intelligence ecosystem to external researchers, open ontologies communities, and downstream products.

### Core Capabilities

1. **Entity Catalog** — List and retrieve artifacts, rules, receipts, lifecycle states
2. **Federated SPARQL** — Query knowledge graph with cross-ontology federation
3. **Event Streaming** — OCEL 2.0 events via Server-Sent Events (real-time)
4. **Receipt Verification** — Cryptographic proof chain (Blake3 signatures)
5. **Provenance Tracing** — Trace artifacts back to source law and authorization
6. **Conformance Checking** — PM4Py integration for Van der Aalst metrics
7. **Health & Metrics** — Ecosystem status and conformance metrics

---

## Directory Structure

```
api/
├── openapi.yaml                          # OpenAPI 3.0 specification
├── api-server.yaml                       # Server configuration (Kubernetes-ready)
├── rate-limiter.yaml                     # Rate limiting rules & policies
├── health-check.sparql                   # SPARQL health validation queries
│
├── README.md                             # This file
├── API_GUIDE.md                          # Complete API usage guide
├── AUTH.md                               # Authentication & authorization
├── RATE_LIMITS.md                        # Rate limiting policy & tiers
├── OPEN_ONTOLOGIES_INTEGRATION.md        # Integration with open-ontologies ecosystem
├── DEPLOYMENT_GUIDE.md                   # Deployment & testing procedures
│
├── examples/
│   ├── 01-health-check.sh                # Health & metrics check
│   ├── 02-entity-discovery.sh            # Entity listing & provenance
│   ├── 03-sparql-queries.sh              # SPARQL query examples
│   ├── 04-event-streaming.sh             # OCEL event stream subscription
│   └── 05-conformance-check.sh           # Conformance checking workflow
│
├── config/
│   ├── kubernetes-deployment.yaml        # Kubernetes manifests
│   ├── prometheus-rules.yaml             # Prometheus alert rules
│   └── logging-config.yaml               # Logging configuration
│
└── docs/
    ├── architecture.md                   # API architecture diagrams
    ├── data-model.md                     # Entity types & relationships
    └── troubleshooting.md                # Common issues & solutions
```

---

## Quick Start

### 1. Authentication

Obtain an API key from the research directorate:

```bash
export PI_API_KEY="pi_key_abc123..."
```

Or use OAuth 2.0:

```bash
# Get access token
curl -X POST https://auth.process-intelligence.research/oauth/token \
  -d "grant_type=client_credentials&client_id=$CLIENT_ID&client_secret=$SECRET"

export ACCESS_TOKEN="eyJhbGc..."
```

### 2. Check Health

```bash
curl -H "X-API-Key: $PI_API_KEY" \
  https://api.process-intelligence.research/v1/health | jq .
```

### 3. List Entities

```bash
curl -H "X-API-Key: $PI_API_KEY" \
  https://api.process-intelligence.research/v1/entities/Artifact | jq .
```

### 4. Execute SPARQL Query

```bash
curl -X POST https://api.process-intelligence.research/v1/sparql \
  -H "X-API-Key: $PI_API_KEY" \
  -H "Content-Type: application/sparql-query" \
  -d "ASK { ?x a ?y }" | jq .
```

### 5. Subscribe to Events

```bash
curl -N -H "X-API-Key: $PI_API_KEY" \
  https://api.process-intelligence.research/v1/stream/ocel
```

---

## Endpoints Reference

| Method | Path | Purpose | Tier |
|--------|------|---------|------|
| GET | `/health` | Ecosystem health | All |
| GET | `/metrics` | Conformance metrics | All |
| GET | `/entities` | List entities | Standard+ |
| GET | `/entity/{id}` | Get entity details | Standard+ |
| GET | `/entities/{type}` | List by type | Standard+ |
| POST | `/sparql` | SPARQL query | Standard+ |
| GET | `/stream/ocel` | Event stream | Premium+ |
| GET | `/receipt/{id}` | Verify receipt | Standard+ |
| GET | `/receipts` | List receipts | Standard+ |
| GET | `/provenance/{id}` | Trace provenance | Standard+ |
| POST | `/provenance/search` | Search provenance | Premium+ |
| GET | `/lifecycle` | Lifecycle definitions | Standard+ |
| POST | `/conformance/check` | Run conformance check | Premium+ |

---

## Rate Limits

| Tier | Req/Min | Req/Hour | Price |
|------|---------|----------|-------|
| Free | 10 | 100 | $0 |
| Standard | 100 | 5K | $499/month |
| Premium | 1K | 100K | $2,499/month |
| Enterprise | ∞ | ∞ | Custom |

See [RATE_LIMITS.md](./RATE_LIMITS.md) for endpoint-specific overrides.

---

## Documentation

### For API Users

- **[API_GUIDE.md](./API_GUIDE.md)** — Complete usage guide with examples
- **[AUTH.md](./AUTH.md)** — Authentication methods & OAuth flow
- **[RATE_LIMITS.md](./RATE_LIMITS.md)** — Rate limiting policies & tier info

### For Integrators

- **[OPEN_ONTOLOGIES_INTEGRATION.md](./OPEN_ONTOLOGIES_INTEGRATION.md)** — Federated ontology access
- **[examples/](./examples/)** — Working code examples in Bash
- **[openapi.yaml](./openapi.yaml)** — OpenAPI 3.0 specification

### For Operators

- **[DEPLOYMENT_GUIDE.md](./DEPLOYMENT_GUIDE.md)** — Kubernetes deployment & testing
- **[api-server.yaml](./api-server.yaml)** — Server configuration
- **[rate-limiter.yaml](./rate-limiter.yaml)** — Rate limiter rules
- **[health-check.sparql](./health-check.sparql)** — Health check queries

---

## Key Features

### 1. Evidence-Driven

Every API response traces back to source documentation or cryptographic receipt. Provenance chains cannot be forged.

### 2. Immutable Record

All operations recorded in OCEL 2.0 event logs. Event history is audit trail for compliance & governance.

### 3. Federated Knowledge

SPARQL queries span local + remote ontologies. Query results include external standards & definitions.

### 4. Real-Time Streaming

Server-Sent Events (SSE) for live OCEL event delivery. No polling needed; clients receive events as they happen.

### 5. Cryptographic Verification

Blake3 signatures on all receipts. Integrity verified via hash chain (receipt #1 → #2 → #3 ...).

### 6. Van der Aalst Compliance

Conformance metrics (fitness, precision, simplicity, generalization) computed via pm4py. Gates enforce thresholds.

---

## API Gateway Features

### Authentication

- **API Keys** — Symmetric (development, service-to-service)
- **JWT Bearer Tokens** — Asymmetric (OAuth 2.0, OpenID Connect)
- **mTLS** — Mutual TLS for zero-trust networks

### Rate Limiting

- **Token Bucket Algorithm** — Fair access with burst allowances (1.5x per-minute)
- **Tier-Based** — Free, Standard, Premium, Enterprise tiers
- **Endpoint-Specific** — Different limits per endpoint (SPARQL: 60/min, Entity lookup: 500/min)
- **Progressive Penalties** — Escalating restrictions for repeated violations

### Monitoring

- **OpenTelemetry** — Distributed tracing (Jaeger, Zipkin)
- **Prometheus Metrics** — Request latency, error rates, conformance metrics
- **Structured Logging** — JSON logs with request IDs, client info, audit trail
- **Health Checks** — Kubernetes liveness & readiness probes

---

## Integration Patterns

### Pattern 1: Research Data Export

Export all artifacts with provenance as RDF:

```bash
bash examples/02-entity-discovery.sh
```

### Pattern 2: Federated SPARQL

Query across local + remote ontologies:

```bash
bash examples/03-sparql-queries.sh
```

### Pattern 3: Real-Time Monitoring

Subscribe to OCEL events for live dashboards:

```bash
bash examples/04-event-streaming.sh
```

### Pattern 4: Conformance Analysis

Submit event logs for fitness metrics:

```bash
bash examples/05-conformance-check.sh
```

---

## Deployment

### Prerequisites

- Kubernetes 1.27+
- RDF4J repository
- PostgreSQL (event logs)
- Redis (caching, rate limiting)
- Kafka (OCEL streaming)
- PM4Py service

### Quick Deploy

```bash
# 1. Create namespace & secrets
kubectl create namespace pi-api
kubectl create secret tls pi-api-tls --cert=... --key=... -n pi-api

# 2. Create ConfigMaps
kubectl create configmap pi-api-config --from-file=openapi.yaml -n pi-api

# 3. Deploy
kubectl apply -f config/kubernetes-deployment.yaml -n pi-api

# 4. Verify
kubectl rollout status deployment/pi-api -n pi-api
kubectl port-forward svc/pi-api 8080:8080 -n pi-api &
curl http://localhost:8080/v1/health
```

See [DEPLOYMENT_GUIDE.md](./DEPLOYMENT_GUIDE.md) for complete instructions.

---

## Testing

### Unit Tests

```bash
pytest tests/unit/ -v --cov=pi_api
```

### Integration Tests

```bash
pytest tests/integration/ -v
```

### Load Testing

```bash
k6 run tests/load/api-load-test.js --vus 100 --duration 10m
```

### Chaos Testing

```bash
gremlin run tests/chaos/kg-unavailable.yaml
gremlin run tests/chaos/redis-down.yaml
```

---

## Validation Gates

**30 Total Gates** (must all PASS for production):

- **Gate 1-5:** Infrastructure (API running, TLS, health checks)
- **Gate 6-10:** API Endpoints (all routes functional)
- **Gate 11-15:** Authentication (API keys, JWT, RBAC)
- **Gate 16-20:** Data Integrity (receipts, provenance, audit logging)
- **Gate 21-25:** Integration (open ontologies, federation, streaming)
- **Gate 26-30:** Operations (monitoring, SLA, support)

---

## Monitoring & Alerts

### Key Metrics

- **Uptime:** 99.5% (Premium), 99.9% (Enterprise)
- **P50 Latency:** <100ms
- **P99 Latency:** <500ms
- **Error Rate:** <0.1%
- **Rate Limit Violations:** <1% per tier

### Alert Channels

- Slack: #process-intelligence-alerts
- Email: api-ops@process-intelligence.research
- PagerDuty: For critical incidents

---

## Support

- **Docs:** https://docs.process-intelligence.research
- **API Issues:** https://github.com/process-intelligence/api/issues
- **Email:** api-support@process-intelligence.research
- **Emergency:** api-emergency@process-intelligence.research

---

## Compliance & Standards

- **OCEL 2.0** — Fully compliant event logs
- **SPARQL 1.1** — SELECT, ASK, CONSTRUCT queries
- **OpenID Connect** — OAuth 2.0 integration
- **SOC 2** — Type II audit (annual)
- **ISO 27001** — Information security management
- **GDPR** — Data protection compliance

---

## Authority & Governance

This API is governed by the Process Intelligence Research Covenant. All claims trace to evidence (papers, experiments, checkpoints). No unsupported claims without documentation.

See [/COVENANT.md](../../../COVENANT.md) for details.

---

## Release Notes

### v1.0.0 (2026-06-01)

✓ Initial release  
✓ All 30 validation gates PASS  
✓ OpenAPI 3.0 specification  
✓ Kubernetes deployment ready  
✓ 5 example integrations  
✓ Complete documentation  
✓ Rate limiting by tier  
✓ SPARQL federation  
✓ OCEL 2.0 streaming  
✓ Conformance checking  

---

**Authority:** Process Intelligence Research Directorate  
**Last Updated:** 2026-06-01  
**Next Review:** 2026-09-01  
**Version:** 1.0.0
