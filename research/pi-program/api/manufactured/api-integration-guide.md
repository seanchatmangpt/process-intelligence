# API Integration Guide

**Version:** 3.0.0
**Authority:** PI Research Program Manufacturing Phase

---

## Quick Start

### 1. Authenticate

Obtain a JWT token or API key from the PI authentication service.

```bash
export AUTH_TOKEN="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
```

### 2. Test Connectivity

```bash
curl -X GET https://api.process-intelligence.org/api/v1/health
```

### 3. Make Your First Request

```bash
curl -X POST https://api.process-intelligence.org/api/v1/processes/manufacture \
  -H "Authorization: Bearer $AUTH_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "processName": "test-process",
    "specification": {}
  }'
```

---

## Integration Patterns

### Request-Response

Standard synchronous API calls for quick operations:

```javascript
const response = await fetch(
  'https://api.process-intelligence.org/api/v1/processes/{processId}',
  {
    headers: { 'Authorization': `Bearer ${token}` }
  }
);
const data = await response.json();
```

### Async Operations

For long-running operations (conformance checks):

1. Submit request to `/api/v1/conformance/check`
2. Receive `processId` in response
3. Poll `/api/v1/processes/{processId}` for status
4. Handle timeouts with exponential backoff

```javascript
async function waitForCompletion(processId, maxWaitMs = 300000) {
  let elapsed = 0;
  let backoff = 100;
  
  while (elapsed < maxWaitMs) {
    const response = await fetch(
      `https://api.process-intelligence.org/api/v1/processes/${processId}`,
      { headers: { 'Authorization': `Bearer ${token}` } }
    );
    const data = await response.json();
    
    if (data.status === 'COMPLETE') return data;
    if (data.status === 'FAILED') throw new Error('Process failed');
    
    await new Promise(r => setTimeout(r, backoff));
    elapsed += backoff;
    backoff = Math.min(backoff * 2, 30000);
  }
  throw new Error('Timeout');
}
```

### Batch Operations

Submit multiple processes and track in parallel:

```javascript
async function batchManufacture(specifications) {
  const promises = specifications.map(spec =>
    fetch(
      'https://api.process-intelligence.org/api/v1/processes/manufacture',
      {
        method: 'POST',
        headers: { 'Authorization': `Bearer ${token}` },
        body: JSON.stringify(spec)
      }
    ).then(r => r.json())
  );
  
  return Promise.all(promises);
}
```

---

## Error Handling

### Implement Retry Logic

```javascript
async function retryWithBackoff(fn, maxRetries = 5) {
  for (let i = 0; i < maxRetries; i++) {
    try {
      return await fn();
    } catch (error) {
      if (error.status !== 429) throw error; // Not a rate limit
      
      const retryAfter = parseInt(
        error.headers.get('Retry-After') || '60'
      );
      const delay = retryAfter * 1000 * Math.pow(2, i);
      
      await new Promise(r => setTimeout(r, delay));
    }
  }
  throw new Error('Max retries exceeded');
}
```

### Parse Error Responses

```javascript
function parseError(response) {
  const error = response.json();
  return {
    code: error.code,        // e.g., 'RATE_001'
    message: error.message,
    retryAfter: error.retry_after,
    timestamp: error.timestamp
  };
}
```

---

## Security Best Practices

### 1. Token Management

```javascript
// Store in secure location
const token = process.env.PI_API_TOKEN;

// Implement refresh logic
async function refreshToken() {
  const response = await fetch(
    'https://api.process-intelligence.org/auth/refresh',
    {
      method: 'POST',
      headers: { 'Authorization': `Bearer ${token}` }
    }
  );
  return response.json().new_token;
}
```

### 2. Request Signing (Optional)

For additional security, sign requests with HMAC:

```javascript
const crypto = require('crypto');

function signRequest(method, path, body, secret) {
  const timestamp = Math.floor(Date.now() / 1000);
  const message = `${method}|${path}|${timestamp}|${body}`;
  const signature = crypto
    .createHmac('sha256', secret)
    .update(message)
    .digest('hex');
  
  return {
    'X-Signature': signature,
    'X-Timestamp': timestamp.toString()
  };
}
```

### 3. Rate Limit Awareness

```javascript
function handleRateLimit(response) {
  const limit = parseInt(response.headers.get('X-RateLimit-Limit'));
  const remaining = parseInt(response.headers.get('X-RateLimit-Remaining'));
  const reset = parseInt(response.headers.get('X-RateLimit-Reset'));
  
  if (remaining < limit * 0.1) {
    console.warn('Approaching rate limit');
  }
  
  return { limit, remaining, reset };
}
```

---

## Monitoring & Observability

### Log Requests

```javascript
function createLogger() {
  return {
    request: (method, path, headers) => {
      console.log({
        timestamp: new Date().toISOString(),
        method,
        path,
        hasAuth: !!headers['Authorization']
      });
    },
    response: (status, headers, body) => {
      console.log({
        timestamp: new Date().toISOString(),
        status,
        rateLimit: headers['X-RateLimit-Remaining'],
        duration: headers['X-Response-Time']
      });
    }
  };
}
```

### Metrics Collection

```javascript
class APIMetrics {
  constructor() {
    this.requests = 0;
    this.errors = 0;
    this.totalLatency = 0;
  }
  
  recordRequest(latency, success) {
    this.requests++;
    this.totalLatency += latency;
    if (!success) this.errors++;
  }
  
  getMetrics() {
    return {
      totalRequests: this.requests,
      errorRate: this.errors / this.requests,
      avgLatency: this.totalLatency / this.requests
    };
  }
}
```

---

## Environment Variables

```bash
# Authentication
export PI_API_TOKEN="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
export PI_API_KEY="pi_live_abc123def456xyz"

# Configuration
export PI_API_BASE_URL="https://api.process-intelligence.org"
export PI_API_TIMEOUT_MS=30000
export PI_API_MAX_RETRIES=5

# Logging
export PI_API_LOG_LEVEL="info"
export PI_API_LOG_REQUESTS=true
```

---

## Troubleshooting

### Connection Issues
- Verify HTTPS connectivity
- Check firewall rules
- Confirm API endpoint availability via `/api/v1/health`

### Authentication Errors
- Validate token expiration
- Confirm token includes required claims
- Check API key prefix and format

### Rate Limiting
- Implement exponential backoff
- Monitor `X-RateLimit-Remaining` header
- Consider increasing subscription tier

### Timeouts
- Increase timeout for intensive operations
- Use async patterns for long-running tasks
- Check server status at `/api/v1/health`

---

## Support

For issues or questions:
- **Documentation:** https://docs.process-intelligence.org/api
- **Status:** https://status.process-intelligence.org
- **Support:** pi-api-support@process-intelligence.org

---

**Authority:** PI Research Program
**Last Updated:** 2026-06-01
