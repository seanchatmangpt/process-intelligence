# API Authentication Specification

**Version:** 3.0.0
**Generated:** 2026-06-01T23:07:34Z
**Authority:** PI Research Program Manufacturing Phase

---

## Authentication Schemes

### 1. Bearer Token (JWT)

**Type:** HTTP
**Scheme:** Bearer
**Format:** JWT

**Description:**
JSON Web Token (JWT) bearer tokens are used for API authentication across all protected endpoints. Tokens must be included in the `Authorization` header as `Bearer <token>`.

**Usage:**
```
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

**Token Requirements:**
- Must be a valid JWT
- Must include `sub` (subject/user ID) claim
- Must include `aud` (audience) claim matching API target
- Must include `exp` (expiration) claim
- Recommended: Include `iss` (issuer) and `iat` (issued at) claims

**Token Validation:**
- Signature verification: HMAC-SHA256 or RS256
- Expiration check: Current time < exp
- Audience validation: Token aud matches API

---

### 2. API Key

**Type:** API Key
**Parameter Name:** `X-API-Key`
**Location:** HTTP Header

**Description:**
API keys provide an alternative authentication method for non-bearer use cases. Include the API key in the `X-API-Key` request header.

**Usage:**
```
X-API-Key: pi_live_abc123def456xyz...
```

**API Key Requirements:**
- Minimum 32 characters
- Alphanumeric with optional underscores
- Prefix: `pi_live_` (production) or `pi_test_` (testing)
- Must be kept confidential

**API Key Validation:**
- Format verification
- Prefix validation
- Active status check
- Rate limit association

---

## Protected Endpoints

The following endpoints require authentication:

| Endpoint | Method | Auth Required | Minimum Scope |
|----------|--------|---------------|---------------|
| `/api/v1/processes/manufacture` | POST | Yes | `process:manufacture` |
| `/api/v1/processes/{processId}` | GET | Yes | `process:read` |
| `/api/v1/conformance/check` | POST | Yes | `conformance:check` |
| `/api/v1/lifecycle/transition` | POST | Yes | `lifecycle:write` |
| `/api/v1/evidence/validate` | POST | Yes | `evidence:validate` |

---

## Public Endpoints

The following endpoints do NOT require authentication:

| Endpoint | Method | Rate Limit |
|----------|--------|-----------|
| `/api/v1/governance/rules` | GET | Standard |
| `/api/v1/health` | GET | Unlimited |

---

## OAuth 2.0 Scope Definitions

Scopes control what operations are permitted by an authenticated client:

### Manufacturing Scopes
- `process:manufacture` — Initiate process manufacturing
- `process:read` — Read process details and status
- `process:list` — List all processes

### Conformance Scopes
- `conformance:check` — Perform conformance checks
- `conformance:read` — View conformance results

### Lifecycle Scopes
- `lifecycle:read` — View artifact lifecycle states
- `lifecycle:write` — Transition artifacts through states
- `lifecycle:admin` — Manage lifecycle policies

### Evidence Scopes
- `evidence:validate` — Validate evidence blocks
- `evidence:read` — Read evidence details
- `evidence:admin` — Manage evidence policies

### Governance Scopes
- `governance:read` — Read governance rules
- `governance:write` — Create/modify governance rules
- `governance:admin` — Manage governance policies

---

## Security Considerations

### Token Management
- **Expiration:** Set short-lived tokens (15 minutes to 1 hour)
- **Refresh:** Use refresh tokens for long-running operations
- **Rotation:** Rotate API keys annually
- **Revocation:** Maintain revocation lists for compromised tokens

### Transport Security
- **HTTPS Only:** All API requests must use HTTPS (TLS 1.2+)
- **Certificate Pinning:** Recommended for mobile clients
- **Compression:** Avoid DEFLATE compression (BREACH vulnerability)

### Request Signing
- **Optional:** For high-security scenarios, include request signature
- **Algorithm:** HMAC-SHA256 or RSA-SHA256
- **Header:** `X-Signature`

---

## Error Responses

Authentication failures return standardized error responses:

### 401 Unauthorized

**Cause:** Missing or invalid authentication credentials

**Response:**
```json
{
  "error": "unauthorized",
  "message": "Authentication credentials missing or invalid",
  "code": "AUTH_001",
  "timestamp": "2026-06-01T12:34:56Z"
}
```

### 403 Forbidden

**Cause:** Authenticated but insufficient permissions

**Response:**
```json
{
  "error": "forbidden",
  "message": "Insufficient permissions for this operation",
  "code": "AUTH_002",
  "required_scopes": ["process:manufacture"],
  "timestamp": "2026-06-01T12:34:56Z"
}
```

### 429 Too Many Requests

**Cause:** Rate limit exceeded

**Response:**
```json
{
  "error": "rate_limit_exceeded",
  "message": "API rate limit exceeded",
  "code": "RATE_001",
  "retry_after": 60,
  "limit_reset": "2026-06-01T12:35:56Z"
}
```

---

## Best Practices

1. **Never commit secrets** — Store tokens in environment variables, not source code
2. **Use HTTPS** — Always encrypt credentials in transit
3. **Validate tokens** — Verify signatures and expiration on every request
4. **Monitor usage** — Track token usage and alert on unusual patterns
5. **Implement scopes** — Use principle of least privilege
6. **Handle refresh** — Implement automatic token refresh logic
7. **Test revocation** — Ensure revoked tokens are rejected quickly

---

## Compliance & Standards

- **JWT Format:** RFC 7519
- **OAuth 2.0:** RFC 6749
- **HTTP Bearer:** RFC 6750
- **TLS:** RFC 5246 (1.2+), RFC 8446 (1.3)

---

**Authority:** PI Research Program
**Last Updated:** 2026-06-01
**Next Review:** Q3 2026
