# Process Intelligence API — Authentication & Authorization Guide

**Authority:** Process Intelligence Research Directorate  
**Version:** 1.0.0  
**Date:** 2026-06-01

---

## Table of Contents

1. [Authentication Methods](#authentication-methods)
2. [API Keys](#api-keys)
3. [JWT Bearer Tokens](#jwt-bearer-tokens)
4. [OAuth 2.0 Flow](#oauth-20-flow)
5. [RBAC & Scopes](#rbac--scopes)
6. [Service-to-Service Auth](#service-to-service-auth)
7. [Key Rotation](#key-rotation)
8. [Audit & Compliance](#audit--compliance)

---

## Authentication Methods

The API supports three authentication methods:

### 1. API Key (Symmetric)
- **Best for:** Development, service-to-service, legacy integrations
- **Security Level:** Medium (must be kept secret)
- **Expiry:** 1 year by default, configurable

### 2. JWT Bearer Token (Asymmetric)
- **Best for:** Web applications, user-facing clients, OpenID Connect flows
- **Security Level:** High (public key verification, expiry enforced)
- **Expiry:** 1 hour by default, refresh token available

### 3. mTLS (Mutual TLS)
- **Best for:** Service-to-service communication, zero-trust networks
- **Security Level:** Highest (bidirectional certificate verification)
- **Expiry:** Configurable per deployment

---

## API Keys

### Creating an API Key

**Via CLI (for development):**
```bash
pi-api-cli keys create \
  --name "my-integration" \
  --tier standard \
  --scope entities:read,sparql:execute \
  --expires-in 365d
```

**Output:**
```
API Key: pi_key_abc123xyz789...
Secret: pi_secret_...

Save this secret in a secure location. You won't be able to retrieve it again.
```

**Via API (requires admin scope):**
```bash
curl -X POST https://api.process-intelligence.research/v1/admin/keys \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "external-researcher",
    "tier": "standard",
    "scopes": ["entities:read", "sparql:execute"],
    "expires_in_days": 365
  }'
```

### Using an API Key

```bash
# In header
curl -H "X-API-Key: pi_key_abc123xyz789..." \
  https://api.process-intelligence.research/v1/entities

# In query parameter (not recommended; use header)
curl "https://api.process-intelligence.research/v1/entities?api_key=pi_key_..."
```

### API Key Structure

```
Prefix (3 chars): pi_key_ (identifies as API key)
Random ID (64 chars): abc123...
Secret Hash (64 chars): sha256(secret)

Full key: pi_key_abc123xyz789[secret_hash]
```

### API Key Rotation

```bash
# 1. Create new key
NEW_KEY=$(pi-api-cli keys create --name "my-integration-v2")

# 2. Update client to use new key
# (test in staging first)

# 3. Revoke old key after grace period
pi-api-cli keys revoke pi_key_old_...

# Grace period: 30 days (configurable)
# During grace period: old key still works with warnings
```

---

## JWT Bearer Tokens

### Obtaining a JWT Token

**Option 1: User Authorization (Interactive)**

```bash
# 1. Authenticate user
curl https://auth.process-intelligence.research/oauth/authorize \
  -d "client_id=my-app&redirect_uri=http://localhost:8080/callback&scope=entities:read"

# 2. User logs in via browser
# 3. Redirected to callback with authorization code

# 4. Exchange code for token
curl -X POST https://auth.process-intelligence.research/oauth/token \
  -d "grant_type=authorization_code&code=$CODE&client_id=my-app&client_secret=$SECRET"
```

**Response:**
```json
{
  "access_token": "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9...",
  "token_type": "Bearer",
  "expires_in": 3600,
  "refresh_token": "rt_abc123...",
  "scope": "entities:read sparql:execute"
}
```

**Option 2: Client Credentials (Service-to-Service)**

```bash
curl -X POST https://auth.process-intelligence.research/oauth/token \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "grant_type=client_credentials&client_id=blue-river&client_secret=$SECRET&scope=entities:read"
```

**Option 3: Refresh Token**

```bash
curl -X POST https://auth.process-intelligence.research/oauth/token \
  -d "grant_type=refresh_token&refresh_token=$REFRESH_TOKEN&client_id=my-app"
```

### Using a JWT Token

```bash
curl -H "Authorization: Bearer $ACCESS_TOKEN" \
  https://api.process-intelligence.research/v1/entities
```

### JWT Token Structure

**Header:**
```json
{
  "alg": "RS256",
  "typ": "JWT",
  "kid": "key-id-123"
}
```

**Payload:**
```json
{
  "iss": "https://auth.process-intelligence.research",
  "sub": "user:12345",
  "aud": "https://api.process-intelligence.research",
  "exp": 1685705400,
  "iat": 1685701800,
  "scope": ["entities:read", "sparql:execute"],
  "tier": "standard"
}
```

**Signature:**
```
RS256(base64(header).base64(payload), private_key)
```

### Validating JWT Tokens

**Via JWKS Endpoint:**

```bash
# Get signing keys
curl https://auth.process-intelligence.research/.well-known/jwks.json
```

**Validation Steps:**
1. Extract JWT from Authorization header
2. Fetch JWKS from issuer
3. Find key matching `kid` in JWT header
4. Verify signature using public key
5. Validate claims:
   - `iss` == expected issuer
   - `aud` == current service
   - `exp` > current time
6. Check scopes against requested operation

---

## OAuth 2.0 Flow

### Authorization Code Flow (User Login)

```
┌─────────────────────────────────────────────────┐
│ User clicks "Login with Process Intelligence"   │
└────────────────┬────────────────────────────────┘
                 ↓
┌─────────────────────────────────────────────────┐
│ Redirect to auth.process-intelligence.research  │
│ /authorize?client_id=...&redirect_uri=...       │
└────────────────┬────────────────────────────────┘
                 ↓
┌─────────────────────────────────────────────────┐
│ User enters credentials                         │
│ (SAML/OIDC bridge to enterprise identity)       │
└────────────────┬────────────────────────────────┘
                 ↓
┌─────────────────────────────────────────────────┐
│ Consent screen (show requested scopes)          │
└────────────────┬────────────────────────────────┘
                 ↓
┌─────────────────────────────────────────────────┐
│ Redirect to callback with authorization code    │
│ http://localhost:8080/callback?code=...         │
└────────────────┬────────────────────────────────┘
                 ↓
┌─────────────────────────────────────────────────┐
│ Exchange code for access token (backend)        │
│ POST /oauth/token                               │
└────────────────┬────────────────────────────────┘
                 ↓
┌─────────────────────────────────────────────────┐
│ Receive access token & refresh token            │
│ Use for API calls                               │
└─────────────────────────────────────────────────┘
```

---

## RBAC & Scopes

### Scope Hierarchy

```
system:admin (full access)
├── entities:read
├── entities:write
├── sparql:execute
├── stream:read
├── receipts:read
├── provenance:read
├── conformance:execute
└── lifecycle:read
```

### Scope Definitions

| Scope | Operations | Tier | Notes |
|-------|-----------|------|-------|
| `entities:read` | GET /entities, GET /entity/{id} | All | Read-only access |
| `entities:write` | POST /entities, PUT /entity/{id} | Premium+ | Create/update (admin approval required) |
| `sparql:execute` | POST /sparql | Standard+ | Execute SPARQL queries (read-only) |
| `stream:read` | GET /stream/ocel | Premium+ | Subscribe to event streams |
| `receipts:read` | GET /receipts, GET /receipt/{id} | Standard+ | Read receipt chain |
| `provenance:read` | GET /provenance | Standard+ | Trace provenance |
| `conformance:execute` | POST /conformance/check | Premium+ | Run conformance checks (uses PM4Py) |
| `lifecycle:read` | GET /lifecycle | Standard+ | View lifecycle definitions |
| `system:admin` | All admin endpoints | Enterprise | Full system access |

### Requesting Scopes

When obtaining OAuth token, specify scopes in query parameter:
```
/authorize?scope=entities:read+sparql:execute+stream:read
```

### Checking Scopes at Runtime

```bash
# Decode JWT and inspect scope claim
curl https://api.process-intelligence.research/v1/me \
  -H "Authorization: Bearer $TOKEN"
```

**Response:**
```json
{
  "user_id": "user:12345",
  "email": "researcher@example.com",
  "scopes": ["entities:read", "sparql:execute"],
  "tier": "standard",
  "token_expires_at": "2026-06-01T16:30:00Z"
}
```

---

## Service-to-Service Auth

### Configuration for Blue River Dam

```yaml
# blue-river-config.yaml
api_auth:
  method: oauth2
  client_id: blue-river-dam
  client_secret: ${BR_CLIENT_SECRET}
  token_endpoint: https://auth.process-intelligence.research/oauth/token
  scopes:
    - entities:read
    - entities:write
    - lifecycle:read
    - conformance:execute
  
  # Token caching
  cache:
    enabled: true
    ttl_seconds: 3300  # 55 min (token expires at 60 min)
```

### Configuration for ggen Validator

```yaml
# ggen-config.yaml
api_auth:
  method: api_key
  api_key: ${GGEN_API_KEY}
  scopes:
    - entities:write
    - receipts:read
    - conformance:execute
```

### Configuration for OTel Weaver

```yaml
# otel-weaver-config.yaml
api_auth:
  method: mtls
  cert_path: /etc/otel-weaver/client.crt
  key_path: /etc/otel-weaver/client.key
  ca_cert_path: /etc/otel-weaver/ca.crt
  scopes:
    - stream:read  # Read OCEL stream
    - entities:write  # Write OTel events as entities
```

---

## Key Rotation

### API Key Rotation Schedule

- **Rotation Period:** Every 90 days (recommended)
- **Grace Period:** 30 days (old key still works)
- **Emergency Rotation:** Immediate (if compromised)

### Automated Rotation Process

```bash
#!/bin/bash
# Kubernetes CronJob: rotate-api-keys

OLD_KEY=$(cat /secrets/pi-api-key)
NEW_KEY=$(pi-api-cli keys create --name "automated-$(date +%Y%m%d)")

# Update Secret
kubectl patch secret pi-api-key -p \
  "{\"data\":{\"api-key\":\"$(echo -n $NEW_KEY | base64 -w0)\"}}"

# Restart pods (rolling update)
kubectl rollout restart deployment/api-consumer -n default

# Revoke old key after grace period
echo "Revoking $OLD_KEY in 30 days" | at now + 30 days
```

### JWT Signing Key Rotation

- **Rotation Period:** Every 1 year
- **Old keys:** Kept in JWKS for 30 days for validation
- **New keys:** Auto-generated, published immediately

---

## Audit & Compliance

### Access Logging

All API calls logged with:
- Request timestamp
- Client identity (user_id, client_id, service_name)
- Endpoint path, method, status code
- Scopes used
- Response time
- IP address

**Log Location:** `/var/log/pi-api/audit.log`  
**Format:** JSON (structured)  
**Retention:** 7 years (GDPR compliance)

**Example Log Entry:**
```json
{
  "timestamp": "2026-06-01T15:30:45.123Z",
  "request_id": "req:abc123",
  "client_type": "user",
  "client_id": "user:researcher@example.com",
  "method": "GET",
  "path": "/v1/sparql",
  "status": 200,
  "scope_used": "sparql:execute",
  "duration_ms": 245,
  "ip_address": "203.0.113.45"
}
```

### Compliance Certifications

- **SOC 2:** Type II audit performed annually
- **ISO 27001:** Information security management
- **GDPR:** Data protection compliance (right to erasure limited by immutable receipt chain)

### Security Incident Response

**Process:**

1. **Detection:** Anomalous authentication activity detected
2. **Investigation:** Audit logs reviewed
3. **Containment:** Affected keys/tokens revoked
4. **Notification:** Affected users contacted within 24 hours
5. **Recovery:** New credentials issued
6. **Post-Mortem:** Root cause analysis within 7 days

**Contact:** security@process-intelligence.research

---

## Troubleshooting

### "Invalid API Key"

```bash
# Verify key is present and not expired
pi-api-cli keys list --filter "name=my-integration"

# Check key has required scopes
curl -H "X-API-Key: $KEY" \
  https://api.process-intelligence.research/v1/admin/keys/me
```

### "Token Expired"

```bash
# Check token expiry
jwt decode $TOKEN

# Refresh token
curl -X POST https://auth.process-intelligence.research/oauth/token \
  -d "grant_type=refresh_token&refresh_token=$REFRESH_TOKEN"
```

### "Insufficient Scopes"

```bash
# Check what scopes you have
curl https://api.process-intelligence.research/v1/me \
  -H "Authorization: Bearer $TOKEN"

# Request additional scopes via OAuth consent screen
# Or contact admin for direct scope grant
```

---

## References

- [OpenID Connect Core](https://openid.net/specs/openid-connect-core-1_0.html)
- [OAuth 2.0 Authorization Framework](https://tools.ietf.org/html/rfc6749)
- [JSON Web Token (JWT)](https://tools.ietf.org/html/rfc7519)
- [JWKS](https://tools.ietf.org/html/rfc7517)

---

**Authority:** Process Intelligence Research Directorate  
**Last Updated:** 2026-06-01  
**Next Review:** 2026-09-01
