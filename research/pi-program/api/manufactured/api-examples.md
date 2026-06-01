# API Usage Examples

**Version:** 3.0.0
**Authority:** PI Research Program Manufacturing Phase

---

## 1. Manufacture Process

### Request
```bash
curl -X POST https://api.process-intelligence.org/api/v1/processes/manufacture \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..." \
  -H "Content-Type: application/json" \
  -d '{
    "processName": "acquisition-workflow",
    "specification": {
      "stages": ["define", "design", "monitoring", "optimization", "repair"],
      "artifacts": 10,
      "receipts": true
    },
    "witness": "pi_witness_abc123"
  }'
```

### Response
```json
{
  "receiptId": "receipt_20260601_abc123",
  "manufactureStatus": "SUCCESS",
  "artifacts": [
    {
      "artifactId": "artifact_001",
      "type": "specification",
      "status": "manufactured",
      "hash": "blake3:abc123..."
    }
  ]
}
```

---

## 2. Get Process Details

### Request
```bash
curl -X GET https://api.process-intelligence.org/api/v1/processes/proc_20260601_001 \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
```

### Response
```json
{
  "processId": "proc_20260601_001",
  "status": "COMPLETE",
  "receipt": {
    "receiptId": "receipt_20260601_abc123",
    "timestamp": "2026-06-01T23:07:34Z",
    "hash": "blake3:xyz789...",
    "witness": "pi_witness_abc123"
  },
  "artifacts": [
    {
      "artifactId": "artifact_001",
      "type": "specification",
      "contentHash": "blake3:abc123...",
      "lifecycleState": "ADMITTED"
    }
  ]
}
```

---

## 3. Conformance Check

### Request
```bash
curl -X POST https://api.process-intelligence.org/api/v1/conformance/check \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..." \
  -H "Content-Type: application/json" \
  -d '{
    "eventLog": {
      "format": "OCEL",
      "objects": 150,
      "events": 5000,
      "traces": 300
    },
    "processModel": {
      "nodes": 25,
      "edges": 45,
      "gateways": 12
    },
    "fitnessCriteria": {
      "minFitness": 0.85,
      "allowPartial": false
    }
  }'
```

### Response
```json
{
  "conformant": true,
  "fitness": 0.92,
  "violations": [],
  "receipt": {
    "checkId": "conformance_check_20260601_xyz",
    "timestamp": "2026-06-01T23:10:45Z",
    "engine": "pm4py",
    "algorithm": "replay"
  }
}
```

---

## 4. Governance Rules

### Request
```bash
curl -X GET "https://api.process-intelligence.org/api/v1/governance/rules?category=manufacturing&limit=10"
```

### Response
```json
{
  "rules": [
    {
      "ruleId": "gov_001",
      "category": "manufacturing",
      "name": "All artifacts must have receipts",
      "severity": "error",
      "description": "Manufacturing artifacts must be receipted"
    },
    {
      "ruleId": "gov_002",
      "category": "manufacturing",
      "name": "Witness required for specifications",
      "severity": "warning",
      "description": "Specifications should include witness information"
    }
  ],
  "totalCount": 24,
  "pageInfo": {
    "hasMore": true,
    "cursor": "next_page_token_abc123"
  }
}
```

---

## 5. Lifecycle Transition

### Request
```bash
curl -X POST https://api.process-intelligence.org/api/v1/lifecycle/transition \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..." \
  -H "Content-Type: application/json" \
  -d '{
    "artifactId": "artifact_001",
    "fromState": "RAW",
    "toState": "ADMITTED",
    "reason": "Conformance check passed with fitness 0.92"
  }'
```

### Response
```json
{
  "artifactId": "artifact_001",
  "previousState": "RAW",
  "currentState": "ADMITTED",
  "transitionProof": "blake3:transition_proof_hash..."
}
```

---

## 6. Evidence Validation

### Request
```bash
curl -X POST https://api.process-intelligence.org/api/v1/evidence/validate \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..." \
  -H "Content-Type: application/json" \
  -d '{
    "evidence": {
      "payload": "...",
      "hash": "blake3:abc123...",
      "witness": "pi_witness_def456",
      "timestamp": "2026-06-01T23:07:34Z"
    },
    "verifyWitness": true,
    "checkReceiptChain": true
  }'
```

### Response
```json
{
  "valid": true,
  "witnessVerified": true,
  "receiptChainValid": true,
  "violations": []
}
```

---

## 7. Health Check

### Request
```bash
curl -X GET https://api.process-intelligence.org/api/v1/health
```

### Response
```json
{
  "status": "healthy",
  "version": "3.0.0",
  "manufacturingReady": true,
  "uptime": 864000
}
```

---

## Error Handling

### Rate Limit Exceeded
```json
{
  "error": "rate_limit_exceeded",
  "message": "API rate limit exceeded",
  "code": "RATE_001",
  "retry_after": 60,
  "limit_reset": "2026-06-01T23:09:00Z"
}
```

### Authentication Failed
```json
{
  "error": "unauthorized",
  "message": "Authentication credentials missing or invalid",
  "code": "AUTH_001",
  "timestamp": "2026-06-01T23:07:34Z"
}
```

### Insufficient Permissions
```json
{
  "error": "forbidden",
  "message": "Insufficient permissions for this operation",
  "code": "AUTH_002",
  "required_scopes": ["process:manufacture"],
  "timestamp": "2026-06-01T23:07:34Z"
}
```

---

## Best Practices

1. **Always use HTTPS** for API calls
2. **Store tokens securely** in environment variables
3. **Implement exponential backoff** for retries
4. **Monitor rate limits** via response headers
5. **Validate receipts** before processing artifacts
6. **Use batch operations** where possible for efficiency

---

**Authority:** PI Research Program
**Last Updated:** 2026-06-01
