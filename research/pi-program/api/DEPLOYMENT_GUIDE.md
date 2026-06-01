# Process Intelligence Global Knowledge API — Deployment & Testing Guide

**Authority:** Process Intelligence Research Directorate  
**Version:** 1.0.0  
**Date:** 2026-06-01  
**Phase:** 24 Product Hardening & Market Launch

---

## Deployment Checklist

### Pre-Deployment (Week 1)

- [ ] **Infrastructure Review**
  - [ ] Kubernetes cluster 1.27+ available
  - [ ] RDF4J repository pre-provisioned
  - [ ] PostgreSQL instance ready (event logs)
  - [ ] Redis cluster deployed (caching, rate limiting)
  - [ ] Kafka brokers online (OCEL streaming)
  - [ ] PM4Py service deployed

- [ ] **Security Preparation**
  - [ ] TLS certificates generated (/etc/pi-api/certs/)
  - [ ] API key signing keys created
  - [ ] OAuth 2.0 authorization server configured
  - [ ] JWKS endpoint published
  - [ ] Database credentials securely stored

- [ ] **Configuration**
  - [ ] api-server.yaml reviewed and customized
  - [ ] rate-limiter.yaml configured for expected load
  - [ ] health-check.sparql validated against RDF schema
  - [ ] CORS origins configured
  - [ ] Logging backends configured

- [ ] **Testing Environment**
  - [ ] Staging cluster deployed
  - [ ] Test data populated
  - [ ] Load testing tools installed (k6, locust)
  - [ ] Monitoring dashboards created (Prometheus, Grafana)

---

### Deployment (Week 2)

**Step 1: Create Kubernetes Secrets**

```bash
# 1. TLS certificates
kubectl create secret tls pi-api-tls \
  --cert=/path/to/server.crt \
  --key=/path/to/server.key \
  -n pi-api

# 2. Database credentials
kubectl create secret generic pi-api-db \
  --from-literal=user=$DB_USER \
  --from-literal=password=$DB_PASSWORD \
  -n pi-api

# 3. API signing keys
kubectl create secret generic pi-api-keys \
  --from-file=signing.key=/path/to/signing.key \
  --from-file=verifying.key=/path/to/verifying.key \
  -n pi-api
```

**Step 2: Create ConfigMaps**

```bash
# 1. API configuration
kubectl create configmap pi-api-config \
  --from-file=openapi.yaml \
  --from-file=api-server.yaml \
  --from-file=rate-limiter.yaml \
  -n pi-api

# 2. SPARQL health checks
kubectl create configmap pi-api-health \
  --from-file=health-check.sparql \
  -n pi-api
```

**Step 3: Deploy API Server**

```bash
# Apply Kubernetes manifests
kubectl apply -f deployment/namespace.yaml
kubectl apply -f deployment/service.yaml
kubectl apply -f deployment/deployment.yaml
kubectl apply -f deployment/hpa.yaml
kubectl apply -f deployment/ingress.yaml

# Verify deployment
kubectl rollout status deployment/pi-api -n pi-api --timeout=5m
kubectl get pods -n pi-api
kubectl get svc -n pi-api
```

**Step 4: Verify Startup**

```bash
# 1. Check health endpoint
kubectl port-forward service/pi-api 8080:8080 -n pi-api &
curl http://localhost:8080/v1/health

# 2. Check liveness probes
kubectl describe pod -n pi-api | grep -A 5 "Liveness"

# 3. Check readiness probes
kubectl get endpoints pi-api -n pi-api

# Stop port-forward
kill %1
```

---

### Post-Deployment (Week 3)

- [ ] **Smoke Tests**
  - [ ] Health endpoint responds (200)
  - [ ] Metrics endpoint responds (200)
  - [ ] Entity listing works
  - [ ] SPARQL query succeeds
  - [ ] Event stream connects
  - [ ] Receipt verification works

- [ ] **Integration Tests**
  - [ ] Kubernetes probes pass
  - [ ] Load balancer routes traffic correctly
  - [ ] TLS certificate valid
  - [ ] CORS headers present
  - [ ] Rate limiting enforced
  - [ ] Logging to persistent volumes

- [ ] **Monitoring**
  - [ ] Prometheus metrics scraped
  - [ ] Grafana dashboards visible
  - [ ] Jaeger traces collected
  - [ ] Log aggregation working (ELK/Loki)
  - [ ] Alerts configured

- [ ] **Documentation**
  - [ ] API docs published (swagger-ui)
  - [ ] Runbooks created
  - [ ] SLA targets documented
  - [ ] On-call procedure defined

---

## Testing Strategy

### Unit Tests

**Test Coverage Target:** 80%+

```bash
# Run unit tests
pytest tests/unit/ -v --cov=pi_api --cov-report=html

# Key test areas:
# - Authentication (API keys, JWT, mTLS)
# - Rate limiting (token bucket, burst allowances)
# - Entity serialization (JSON schema validation)
# - SPARQL query parsing (validation, injection prevention)
# - Receipt verification (Blake3 signature)
```

### Integration Tests

**Environment:** Staging cluster with real databases

```bash
# 1. Entity CRUD operations
pytest tests/integration/test_entities.py -v

# 2. SPARQL federation
pytest tests/integration/test_sparql.py -v

# 3. Event streaming
pytest tests/integration/test_streaming.py -v

# 4. Receipt chain integrity
pytest tests/integration/test_receipts.py -v

# 5. Conformance checking
pytest tests/integration/test_conformance.py -v

# 6. Provenance tracing
pytest tests/integration/test_provenance.py -v
```

### Load Testing

**Scenario:** 1000 RPS across all endpoints, 10 minute duration

```bash
# k6 load test
k6 run tests/load/api-load-test.js \
  --vus 100 --duration 10m \
  --out json=results.json

# Expected results:
# - P50 latency < 100ms
# - P99 latency < 500ms
# - Error rate < 0.1%
# - 0 429 rate limit errors (proper throttling)
```

**Load Profile:**

```javascript
// tests/load/api-load-test.js
import http from 'k6/http';
import { check, sleep } from 'k6';

const BASE_URL = 'https://api.process-intelligence.research/v1';

export const options = {
  stages: [
    { duration: '2m', target: 100 },   // Ramp-up
    { duration: '5m', target: 100 },   // Sustain
    { duration: '2m', target: 0 },     // Ramp-down
  ],
  thresholds: {
    http_req_duration: ['p(95)<500', 'p(99)<1000'],
    http_req_failed: ['rate<0.001'],
  },
};

export default function () {
  // 50% entity lookups
  const entity_res = http.get(`${BASE_URL}/entities?limit=10`);
  check(entity_res, { 'entity lookup 200': (r) => r.status === 200 });
  
  // 30% SPARQL queries
  const sparql_res = http.post(`${BASE_URL}/sparql`, 
    'ASK { ?x a ?y }',
    { headers: { 'Content-Type': 'application/sparql-query' } }
  );
  check(sparql_res, { 'SPARQL 200': (r) => r.status === 200 });
  
  // 20% receipt verification
  const receipt_res = http.get(`${BASE_URL}/receipt/blake3:abc123`);
  check(receipt_res, { 'receipt 200 or 404': (r) => [200, 404].includes(r.status) });
  
  sleep(0.5);
}
```

### Chaos Testing

**Goal:** Verify graceful degradation under faults

```bash
# 1. Knowledge graph unavailable
gremlin run tests/chaos/kg-unavailable.yaml
# Expected: API returns cached results or 503

# 2. Database connection pool exhausted
gremlin run tests/chaos/db-pool-exhausted.yaml
# Expected: Requests queued, 503 after timeout

# 3. OTEL exporter down
gremlin run tests/chaos/otel-down.yaml
# Expected: API continues, telemetry buffered

# 4. Rate limiter Redis down
gremlin run tests/chaos/redis-down.yaml
# Expected: Rate limiting disabled (permissive fallback)

# 5. Network latency spike (500ms → 5s)
gremlin run tests/chaos/network-latency.yaml
# Expected: Timeouts after 30s, graceful cleanup
```

---

## Validation Gates (30 Total)

### Gate 1-5: Infrastructure (5 gates) ✓ COMPLETE

- [x] **Gate 1:** API server deployed and running
- [x] **Gate 2:** TLS endpoints accessible
- [x] **Gate 3:** Health endpoint responds 200
- [x] **Gate 4:** Rate limiting active and enforced
- [x] **Gate 5:** Logging to persistent storage

### Gate 6-10: API Endpoints (5 gates) — VALIDATE POST-DEPLOY

- [ ] **Gate 6:** Entity listing works (GET /entities)
- [ ] **Gate 7:** SPARQL queries execute (POST /sparql)
- [ ] **Gate 8:** Event streaming works (GET /stream/ocel)
- [ ] **Gate 9:** Receipt verification works (GET /receipt/{id})
- [ ] **Gate 10:** Provenance tracing works (GET /provenance/{id})

### Gate 11-15: Authentication (5 gates)

- [ ] **Gate 11:** API key authentication enforced
- [ ] **Gate 12:** JWT bearer token validation works
- [ ] **Gate 13:** RBAC scopes enforced
- [ ] **Gate 14:** Invalid credentials rejected (401)
- [ ] **Gate 15:** Rate limit per tier enforced

### Gate 16-20: Data Integrity (5 gates)

- [ ] **Gate 16:** Receipt chain verified (Blake3 signatures)
- [ ] **Gate 17:** Entity provenance complete (all artifacts traced)
- [ ] **Gate 18:** OCEL stream validation enforced
- [ ] **Gate 19:** Conformance metrics authenticated (signed receipts)
- [ ] **Gate 20:** Audit logging complete (all calls logged)

### Gate 21-25: Integration (5 gates)

- [ ] **Gate 21:** Open Ontologies service registry integration
- [ ] **Gate 22:** Federated SPARQL queries work
- [ ] **Gate 23:** Bidirectional OCEL streaming works
- [ ] **Gate 24:** Knowledge graph federation active
- [ ] **Gate 25:** Cross-project entity linking works

---

## Monitoring & Alerting

### Key Metrics

```
# Availability
api_uptime_percent                    (target: 99.5%)
api_health_checks_passing             (target: 30/30)

# Performance
api_request_p50_latency_ms            (target: <100ms)
api_request_p99_latency_ms            (target: <500ms)
api_sparql_query_duration_ms          (target: <3000ms)

# Rate Limiting
api_429_errors_total                  (target: <1% of requests)
api_rate_limit_exceeded_percent       (target: <0.1% per tier)

# Errors
api_5xx_errors_total                  (target: <0.01%)
api_sparql_federation_failures        (target: <1%)

# Data
api_entity_count                      (growing over time)
api_receipt_chain_length              (immutable, growing)
ocel_events_ingested_total            (rate > 100/min)
```

### Alert Rules

```yaml
# api-alerts.yaml
groups:
  - name: pi-api
    rules:
      - alert: APIHighErrorRate
        expr: rate(api_5xx_errors_total[5m]) > 0.001
        for: 5m
        annotations:
          summary: "API error rate > 0.1%"
          
      - alert: APIRateLimitAbuse
        expr: rate(api_429_errors[5m]) > 0.01
        for: 5m
        annotations:
          summary: "Rate limit violations > 1%"
          
      - alert: SPARQLFederationDown
        expr: rate(api_sparql_federation_failures[5m]) > 0.01
        for: 10m
        annotations:
          summary: "Remote SPARQL endpoint unavailable"
          
      - alert: APIHighLatency
        expr: histogram_quantile(0.99, api_request_duration_ms) > 500
        for: 5m
        annotations:
          summary: "API P99 latency > 500ms"
```

---

## Rollback Procedure

If deployment fails or introduces critical issues:

```bash
# 1. Identify failed deployment
kubectl rollout history deployment/pi-api -n pi-api

# 2. Rollback to previous version
kubectl rollout undo deployment/pi-api -n pi-api

# 3. Verify rollback
kubectl rollout status deployment/pi-api -n pi-api

# 4. Investigate root cause
kubectl logs deployment/pi-api -n pi-api --tail=100
kubectl describe pod -l app=pi-api -n pi-api | grep -A 10 "Events"

# 5. Create incident post-mortem
# (required within 24 hours)
```

---

## Post-Mortem Process

**Template:** `post-mortem-YYYYMMDD-ISSUE.md`

**Required Sections:**
1. Timeline (when detected, when mitigated, when resolved)
2. Root cause
3. Impact (requests affected, data loss, SLA breach)
4. Remediation (how it was fixed)
5. Prevention (how to prevent in future)
6. Action items (follow-up tasks)

---

## SLA & Success Criteria

### Deployment Success

✓ **PASS** if:
- All 30 validation gates PASS
- Zero errors in 24-hour monitoring period
- All conformance metrics >= thresholds
- All integration tests pass

⚠️ **PARTIAL** if:
- 25+ gates PASS
- <0.1% error rate
- Blocking issues identified and escalated

❌ **FAILED** if:
- <25 gates PASS
- Critical data integrity issues
- SLA violations (>0.5% error rate)

---

## Support & Escalation

- **Deployment Issues:** devops@process-intelligence.research
- **Performance Issues:** performance@process-intelligence.research
- **Security Issues:** security@process-intelligence.research
- **Emergency:** api-emergency@process-intelligence.research

---

**Authority:** Process Intelligence Research Directorate  
**Last Updated:** 2026-06-01  
**Next Review:** 2026-07-01
