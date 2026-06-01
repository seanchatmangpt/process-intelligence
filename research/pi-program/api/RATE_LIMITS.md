# Process Intelligence API — Rate Limiting Policy

**Authority:** Process Intelligence Research Directorate  
**Version:** 1.0.0  
**Date:** 2026-06-01  
**Effective:** Immediately upon API launch

---

## Overview

Rate limiting protects the ecosystem from abuse, prevents cascading failures, and ensures fair access for all clients.

### Principles

1. **Fair Access:** All tiers get documented limits; no hidden surprises
2. **Transparent:** Rate limit headers included in all responses
3. **Gradual Enforcement:** Burst allowances (1.5x) for short-term spikes
4. **Service Protection:** Different limits per endpoint based on resource cost
5. **Progressive Penalties:** Repeated violations result in escalating restrictions

---

## Global Rate Limits (by Tier)

### Free Tier
- **Requests/Minute:** 10
- **Requests/Hour:** 100
- **Concurrent Requests:** 1
- **Cost:** $0
- **Use Case:** Public testing, evaluation
- **Upgrade Path:** Contact sales@process-intelligence.research

### Standard Tier
- **Requests/Minute:** 100
- **Requests/Hour:** 5,000
- **Concurrent Requests:** 10
- **Cost:** $499/month
- **Use Case:** Development, small integrations
- **SLA:** 99.0% uptime

### Premium Tier
- **Requests/Minute:** 1,000
- **Requests/Hour:** 100,000
- **Concurrent Requests:** 100
- **Cost:** $2,499/month
- **Use Case:** Production applications
- **SLA:** 99.5% uptime
- **Benefits:** Priority support, advanced monitoring

### Enterprise Tier
- **Requests/Minute:** Unlimited
- **Requests/Hour:** Unlimited
- **Concurrent Requests:** Unlimited
- **Cost:** Custom (per SLA)
- **Use Case:** High-scale deployments
- **SLA:** 99.9% uptime with custom terms
- **Benefits:** Dedicated support, custom integrations, priority security patches

---

## Endpoint-Specific Limits

### High-Cost Endpoints (Resource-Intensive)

#### `/v1/sparql` — SPARQL Queries

```
Free:       5/min, 50/hour
Standard:   60/min, 1K/hour
Premium:    500/min, 10K/hour
Enterprise: Unlimited
```

**Why Lower?**
- SPARQL queries can be expensive (complex join, large result sets)
- Federation across remote endpoints adds latency
- Maximum execution time: 120 seconds

**Example:**
```bash
curl -X POST https://api.process-intelligence.research/v1/sparql \
  -H "X-API-Key: $KEY" \
  -H "Content-Type: application/sparql-query" \
  -d "SELECT * WHERE { ?s ?p ?o }"

# Response Headers:
# RateLimit-Limit: 60
# RateLimit-Remaining: 59
# RateLimit-Reset: 1685701860
```

---

#### `/v1/conformance/check` — PM4Py Conformance

```
Free:       2/min, 20/hour
Standard:   30/min, 500/hour
Premium:    100/min, 2K/hour
Enterprise: Unlimited
```

**Why Lower?**
- Conformance checking uses external PM4Py service
- Timeout: 300 seconds (5 minutes)
- Resource-heavy algorithm (fitness, precision, simplicity, generalization)

**Queued Behavior:**
- Requests >5min wait time go to queue
- Queue max depth: 100
- Queue timeout: 1 hour

---

#### `/v1/stream/ocel` — Event Streaming

```
All tiers: Unlimited requests, max 100 concurrent streams
```

**Why Higher?**
- Server-Sent Events (SSE) is connection-based, not request-based
- No heavy processing per connection
- Supports long-lived connections for real-time monitoring

**Connection Limits:**
- Max 100 concurrent streams per account
- Idle timeout: 24 hours
- Auto-reconnect with exponential backoff recommended

---

### Moderate-Cost Endpoints (Standard)

#### `/v1/entity/{id}` — Entity Lookup

```
Free:       50/min, 500/hour
Standard:   500/min, 10K/hour
Premium:    2000/min, 100K/hour
Enterprise: Unlimited
```

**Characteristics:**
- Direct index lookup, cached heavily
- P50 latency: <50ms
- No external service calls

---

#### `/v1/entities?type=...` — Entity Listing

```
Free:       20/min, 200/hour
Standard:   300/min, 5K/hour
Premium:    1000/min, 50K/hour
Enterprise: Unlimited
```

**Note:** Pagination required; max 1000 results per request

---

#### `/v1/receipt/{receipt-id}` — Receipt Verification

```
Free:       30/min, 300/hour
Standard:   300/min, 5K/hour
Premium:    1000/min, 50K/hour
Enterprise: Unlimited
```

**Characteristics:**
- SQLite lookup (immutable receipt chain)
- Signature verification (Blake3)
- P99 latency: <100ms

---

#### `/v1/provenance/{artifact-id}` — Provenance Tracing

```
Free:       10/min, 100/hour
Standard:   100/min, 1K/hour
Premium:    500/min, 10K/hour
Enterprise: Unlimited
```

**Characteristics:**
- Graph traversal (ancestry tracing)
- Multiple SPARQL queries per request
- Cached for 1 hour

---

### Low-Cost Endpoints (Excluded from Limits)

#### `/v1/health` — Health Status

```
All tiers: Unlimited
```

**Rationale:** Used by Kubernetes probes, load balancers, monitoring

---

#### `/v1/metrics` — Conformance Metrics

```
All tiers: Unlimited
```

**Rationale:** Used for monitoring dashboards, SLA verification

---

## Burst Allowances

All tiers include **burst capacity** of 1.5x the per-minute limit, applied over a 10-second window.

### Example: Standard Tier SPARQL

```
Per-minute limit:   60
Burst allowance:    90 (1.5x)
Per-10-second:      10

Scenario 1: Client sends 90 requests in 10 seconds
→ ALLOWED (within burst)
→ RateLimit-Remaining: 0
→ Wait 50 seconds for reset

Scenario 2: Client sends 100 requests in 10 seconds
→ FIRST 90 ALLOWED
→ REQUESTS 91-100 → 429 Too Many Requests
→ Retry-After: 50 seconds
```

---

## Rate Limit Enforcement

### Token Bucket Algorithm

```
┌─────────────────────────────────────────┐
│ Token Bucket (Redis)                    │
│                                         │
│ Capacity:   60 tokens (per minute)      │
│ Refill:     1 token per second          │
│ Cost:       1 token per request         │
│                                         │
│ Current:    45 tokens                   │
└─────────────────────────────────────────┘

Request arrives:
  ├─ Has tokens?
  │  ├─ YES → Consume 1, allow request
  │  └─ NO → Return 429, Retry-After
  └─ Refill happens passively (no background job)
```

### Rate Limit Headers

Every API response includes rate limit headers:

```
RateLimit-Limit:       60              # Per-minute limit for this endpoint
RateLimit-Remaining:   23              # Tokens remaining in current window
RateLimit-Reset:       1685701860      # Unix timestamp when limit resets
```

### When Limit Exceeded

**HTTP 429 Too Many Requests**

```json
{
  "error": "rate_limit_exceeded",
  "message": "You have exceeded the rate limit for this endpoint.",
  "limit": 60,
  "window": "minute",
  "retry_after_seconds": 45,
  "reset_time": "2026-06-01T15:31:45Z"
}
```

**Response Headers:**
```
Retry-After: 45
RateLimit-Limit: 60
RateLimit-Remaining: 0
RateLimit-Reset: 1685701905
```

---

## Quota vs. Rate Limit

| Aspect | Rate Limit | Quota |
|--------|-----------|-------|
| **Scope** | Per minute/hour | Per month/day |
| **Enforcement** | Hard (429 response) | Soft (warning) |
| **Reset** | Automatic | Time-based |
| **API Response** | HTTP 429 | HTTP 200 + quota_remaining header |

### Monthly Quota by Tier

| Tier | Monthly Quota | Notes |
|------|---------------|-------|
| Free | 100K requests | Resets 1st of month |
| Standard | 5M requests | Resets 1st of month |
| Premium | 100M requests | Overage: $0.01 per 1K |
| Enterprise | Custom | Per SLA agreement |

---

## Monitoring Rate Limits

### Check Current Usage

```bash
curl https://api.process-intelligence.research/v1/me/rate-limits \
  -H "X-API-Key: $KEY"
```

**Response:**
```json
{
  "tier": "standard",
  "limits": {
    "global": {
      "requests_per_minute": 100,
      "requests_per_hour": 5000,
      "remaining_minute": 67,
      "remaining_hour": 4923
    },
    "endpoints": {
      "/v1/sparql": {
        "requests_per_minute": 60,
        "remaining": 58,
        "reset_at": "2026-06-01T15:31:00Z"
      }
    }
  }
}
```

### Monitoring via Prometheus Metrics

```
pi_api_rate_limit_remaining{endpoint="/v1/sparql", tier="standard"} 58
pi_api_rate_limit_reset{endpoint="/v1/sparql"} 1685701860
pi_api_429_errors_total{endpoint="/v1/sparql"} 12
pi_api_quota_remaining_month{tier="standard"} 4234567
```

---

## Progressive Penalties

Repeated rate limit violations result in escalating restrictions:

### Violation Timeline

**Incident 1:** 1 violation → Warning email
**Incident 2-5:** Additional violations within 7 days → Temporary reduction (50% limit)
**Incident 6+:** Systematic abuse → Temporary suspension (1-7 days)
**Pattern:** Sustained abuse → Account review + potential termination

### Example: Standard Tier Account

```
Day 1, 10:00 AM:  Client exceeds SPARQL limit 3 times
                  → Email: "Rate limit exceeded on /v1/sparql"

Day 2, 2:00 PM:   Client exceeds global limit 5 times
                  → Automatic: SPARQL limit reduced to 30/min
                  → Email: "Rate limit temporarily reduced (50%)"

Day 3, 9:00 AM:   Client still exceeding reduced limits
                  → Automatic: Account suspended for 24 hours
                  → Email: "Account suspended pending review"

Day 4, 9:00 AM:   Suspension lifted; contact required for re-enablement
```

---

## Common Scenarios & Solutions

### Scenario 1: Polling for Events

**Problem:** Need to check for new events frequently

**Bad Approach:**
```python
for i in range(1000):
  response = requests.get("/v1/entities?type=Event")
  time.sleep(1)  # Poll every 1 second
```
Result: 1000 requests/1000 seconds = 1 req/sec = 60/min (hits limit immediately)

**Good Approach:**
```python
# Use event streaming instead
response = requests.get("/v1/stream/ocel?since=2026-06-01T00:00:00Z", stream=True)
for event in response.iter_lines():
  print(event)  # Real-time events, no rate limit
```

---

### Scenario 2: Batch Processing

**Problem:** Need to process 10,000 entities

**Bad Approach:**
```python
entities = requests.get("/v1/entities?limit=10000").json()
# Rate limit: 100 requests/min = ~1.7 seconds per request
# 100 entities: 100 requests * 1.7s = 170 seconds total
```

**Good Approach:**
```python
# Use pagination + concurrent requests (respecting limit)
import concurrent.futures
import time

def fetch_page(offset):
  return requests.get(f"/v1/entities?offset={offset}&limit=100")

with concurrent.futures.ThreadPoolExecutor(max_workers=5) as executor:
  futures = []
  for offset in range(0, 10000, 100):
    futures.append(executor.submit(fetch_page, offset))
    time.sleep(0.5)  # Stagger to stay under rate limit
  
  for future in concurrent.futures.as_completed(futures):
    entities = future.result().json()["entities"]
    # Process entities
```

---

### Scenario 3: SPARQL Federation

**Problem:** Complex query spans multiple endpoints

**Bad Approach:**
```sparql
SELECT * WHERE {
  ?artifact a ggen:Artifact .
  ?artifact ggen:generated-by ?rule .
  ?rule a ggen:GenerationRule .
  SERVICE <https://ontology-1.open-ontologies.research/sparql> {
    ?rule ggen:conforms-to ?standard .
  }
  SERVICE <https://ontology-2.open-ontologies.research/sparql> {
    ?standard dc:title ?title .
  }
}
```
Result: 1 query, 120 second timeout, federation adds latency

**Good Approach:**
```sparql
# Break into local + remote
SELECT * WHERE {
  ?artifact a ggen:Artifact ;
    ggen:generated-by ?rule .
  
  ?rule a ggen:GenerationRule ;
    ggen:conforms-to ?standard .
}
# Then in application code, fetch standard titles from remote
```

---

## Requesting Rate Limit Changes

### Temporary Increase

For known spikes (e.g., bulk data migration):

```bash
curl -X POST https://api.process-intelligence.research/v1/admin/rate-limit-increase \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "reason": "monthly data migration",
    "duration_days": 3,
    "requested_limit": 500,
    "endpoint": "/v1/conformance/check"
  }'
```

**Approval SLA:**
- Standard request: 24 hours
- Emergency: 1 hour
- Contact: api-support@process-intelligence.research

### Permanent Upgrade

Upgrade to next tier:

```bash
# Use CLI
pi-api-cli account upgrade --tier premium

# Or contact sales
# Email: sales@process-intelligence.research
# Phone: +1-555-0123 (EST business hours)
```

---

## Testing & Simulation

### Load Testing with Rate Limit Awareness

```bash
# k6 script respecting rate limits
import http from 'k6/http';
import { check, sleep } from 'k6';

const RATE_LIMIT_PER_MIN = 100;
const DELAY_MS = (60 * 1000) / RATE_LIMIT_PER_MIN;  // 600ms between requests

export const options = {
  stages: [
    { duration: '5m', target: 50 },  # Ramp to 50 users
    { duration: '10m', target: 100 },
    { duration: '5m', target: 0 },   # Ramp down
  ],
};

export default function () {
  const res = http.get('https://api.process-intelligence.research/v1/entities');
  check(res, {
    'status is 200': (r) => r.status === 200,
    'no 429': (r) => r.status !== 429,
  });
  sleep(DELAY_MS / 1000);
}
```

---

## SLA & Support

- **Service Uptime:** 99.0% (Standard), 99.5% (Premium), 99.9% (Enterprise)
- **Rate Limit Questions:** api-support@process-intelligence.research
- **Emergency Contact:** api-emergency@process-intelligence.research
- **Status Page:** https://status.process-intelligence.research

---

**Authority:** Process Intelligence Research Directorate  
**Last Updated:** 2026-06-01  
**Next Review:** 2026-09-01
