# Integration Point 2: Blue River Dam ↔ Expo/Supabase Framework

**Authority:** Process Intelligence Research Foundry  
**Status:** Integration Specification  
**Date:** 2026-06-01

---

## Purpose

Blue River Dam (autonomic governor) drives governance policy decisions through the MAPE-K loop. Expo/Supabase Framework provides the mobile UX and data persistence layer. This integration establishes the **policy enforcement boundary**: Blue River decides governance rules; Supabase RLS enforces them at the database and API level; Expo displays constraints to users.

---

## Interface Specification

### Blue River as Policy Producer

Blue River must expose governance policy endpoints:

```rust
// blue_river_dam/src/governance.rs

pub struct GovernancePolicy {
    pub policy_id: String,                      // Unique identifier
    pub governance_state: LifecycleState,        // Current MAPE-K state
    pub allowed_roles: Vec<String>,              // Roles permitted in this state
    pub allowed_stages: Vec<String>,             // Stages executable in this state
    pub forbidden_transitions: Vec<(String, String)>, // (from_stage, to_stage)
    pub rate_limits: RateLimitPolicy,            // Events/minute per role
    pub data_access_rules: Vec<DataAccessRule>,  // Which data is readable/writable
    pub timestamp: i64,                          // UNIX timestamp (ms)
    pub expires_at: i64,                         // Policy expiration (ms)
    pub receipt: PolicyReceipt,                  // BLAKE3 hash + Ed25519 signature
}

pub struct RateLimitPolicy {
    pub per_minute: u32,                    // Events per minute
    pub per_hour: u32,                      // Events per hour
    pub burst_capacity: u32,                // Max concurrent requests
}

pub struct DataAccessRule {
    pub resource: String,               // E.g., "process_instances"
    pub action: String,                 // "read" | "write" | "delete"
    pub condition: String,              // ABAC condition (e.g., "process.status='active'")
    pub roles: Vec<String>,             // Roles that can perform this action
}

pub struct PolicyReceipt {
    pub policy_hash: String,            // BLAKE3(policy_id || state || rules || timestamp)
    pub signature: String,              // Ed25519 signature by governor key
    pub signing_key_id: String,         // "blue_river_governor_v1"
    pub generated_at: i64,              // Generation timestamp (ms)
}

impl Governor {
    pub async fn emit_governance_policy(&self) -> Result<GovernancePolicy, GovError> {
        // 1. Capture current MAPE-K state
        let current_state = self.get_lifecycle_state();
        
        // 2. Query type law for allowed actions in this state
        let allowed_roles = self.query_type_law("allowed_roles", &current_state)?;
        let allowed_stages = self.query_type_law("allowed_stages", &current_state)?;
        let forbidden_transitions = self.query_type_law("forbidden_transitions", &current_state)?;
        
        // 3. Query governance rules for rate limits
        let rate_limits = self.get_rate_limits(&current_state)?;
        
        // 4. Query data access rules (ABAC-based)
        let data_access_rules = self.get_data_access_rules(&current_state)?;
        
        // 5. Create policy object
        let policy = GovernancePolicy {
            policy_id: format!("gov_pol_{}", uuid::Uuid::new_v4()),
            governance_state: current_state.clone(),
            allowed_roles,
            allowed_stages,
            forbidden_transitions,
            rate_limits,
            data_access_rules,
            timestamp: now_ms(),
            expires_at: now_ms() + 600_000, // 10 minutes
            receipt: self.sign_policy(...)?,
        };
        
        // 6. Emit policy to Supabase
        self.push_policy_to_supabase(&policy).await?;
        
        Ok(policy)
    }

    pub async fn push_policy_to_supabase(&self, policy: &GovernancePolicy) -> Result<(), GovError> {
        let policy_json = serde_json::to_string(policy)?;
        let response = http_client
            .post("https://supabase.example.com/api/governance/policies")
            .header("Authorization", &format!("Bearer {}", self.jwt_token))
            .header("X-Signature", &policy.receipt.signature)
            .body(&policy_json)
            .send()
            .await?;
        
        if response.status() != 200 {
            return Err(GovError::PolicyPushFailed(response.status().to_string()));
        }
        
        Ok(())
    }
}

// REST endpoint (Blue River → Supabase)
pub async fn emit_governance_policies_endpoint(
    State(governor): State<Governor>,
) -> Response {
    match governor.emit_governance_policy().await {
        Ok(policy) => {
            Json(json!({
                "policy_id": policy.policy_id,
                "status": "published",
                "expires_at": policy.expires_at,
                "receipt": policy.receipt
            })).into_response()
        }
        Err(e) => {
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
```

### Supabase RLS as Policy Enforcer

Supabase PostgreSQL enforces governance policies at the database layer:

```sql
-- supabase/rls_policies/governance_enforcement.sql

-- Table to store Blue River governance policies
CREATE TABLE IF NOT EXISTS public.governance_policies (
    policy_id TEXT PRIMARY KEY,
    governance_state TEXT NOT NULL,      -- "Monitor", "Analyze", "Plan", "Execute", "Knowledge"
    allowed_roles TEXT[] NOT NULL,       -- ["operator", "auditor"]
    allowed_stages TEXT[] NOT NULL,      -- ["OrderReceived", "PaymentProcessing", ...]
    forbidden_transitions JSONB NOT NULL,  -- [{"from": "State1", "to": "State2"}, ...]
    rate_limit_per_minute INT NOT NULL,
    rate_limit_per_hour INT NOT NULL,
    rate_limit_burst INT NOT NULL,
    data_access_rules JSONB NOT NULL,    -- [{"resource": "...", "action": "...", "condition": "...", "roles": [...]}]
    signature TEXT NOT NULL,              -- Ed25519 signature by blue_river_governor_key
    signing_key_id TEXT NOT NULL,        -- "blue_river_governor_v1"
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    expires_at TIMESTAMP NOT NULL,
    
    CONSTRAINT governance_policies_signature_check CHECK (
        octet_length(signature) = 128  -- Ed25519 signature is 64 bytes hex
    )
);

-- Table to track active governance state per session
CREATE TABLE IF NOT EXISTS public.governance_context (
    session_id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES auth.users(id),
    active_policy_id TEXT NOT NULL REFERENCES public.governance_policies(policy_id),
    governance_state TEXT NOT NULL,
    assigned_at TIMESTAMP NOT NULL DEFAULT now(),
    expires_at TIMESTAMP NOT NULL
);

-- RLS Policy: Users can only see processes allowed by governance state
CREATE POLICY enforce_governance_on_process_read
    ON public.process_instances
    FOR SELECT
    USING (
        -- User must have active governance context
        EXISTS (
            SELECT 1 FROM public.governance_context gc
            WHERE gc.user_id = auth.uid()
            AND gc.active_policy_id IN (
                SELECT policy_id FROM public.governance_policies gp
                WHERE gp.allowed_stages @> ARRAY[process_instances.current_stage]
                AND gp.expires_at > now()
            )
        )
    );

-- RLS Policy: Users cannot transition to forbidden states
CREATE POLICY enforce_forbidden_transitions
    ON public.process_instances
    FOR UPDATE
    USING (
        NOT EXISTS (
            SELECT 1 FROM public.governance_policies gp,
                   public.governance_context gc
            WHERE gc.user_id = auth.uid()
            AND gc.active_policy_id = gp.policy_id
            AND (process_instances.current_stage, process_instances.next_stage)::text IN (
                SELECT jsonb_array_elements(gp.forbidden_transitions)::text
            )
        )
    );

-- RLS Policy: Enforce rate limits
CREATE POLICY enforce_rate_limits
    ON public.process_events
    FOR INSERT
    WITH CHECK (
        (
            SELECT COUNT(*) FROM public.process_events pe
            WHERE pe.user_id = auth.uid()
            AND pe.created_at > now() - interval '1 minute'
        ) < (
            SELECT gp.rate_limit_per_minute
            FROM public.governance_context gc
            JOIN public.governance_policies gp ON gc.active_policy_id = gp.policy_id
            WHERE gc.user_id = auth.uid()
            AND gp.expires_at > now()
        )
    );

-- Trigger: Verify Blue River signature on policy updates
CREATE OR REPLACE FUNCTION verify_governance_policy_signature()
RETURNS TRIGGER AS $$
BEGIN
    -- Call external function to verify Ed25519 signature
    IF NOT verify_ed25519_signature(
        NEW.policy_id || NEW.governance_state || NEW.allowed_roles::text || ...,
        NEW.signature,
        'blue_river_governor_v1'
    ) THEN
        RAISE EXCEPTION 'Invalid governance policy signature';
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER governance_policy_signature_trigger
BEFORE INSERT OR UPDATE ON public.governance_policies
FOR EACH ROW
EXECUTE FUNCTION verify_governance_policy_signature();
```

### Expo as UI Enforcer

Expo/React Native app enforces policies at the UI level:

```typescript
// expo/src/hooks/useGovernanceContext.ts

export function useGovernanceContext() {
    const [policy, setPolicy] = useState<GovernancePolicy | null>(null);
    const [loading, setLoading] = useState(true);
    
    useEffect(() => {
        // 1. Fetch active governance policy from Supabase on app start
        const fetchPolicy = async () => {
            const { data, error } = await supabase
                .from('governance_context')
                .select('active_policy_id')
                .eq('user_id', authUser.id)
                .eq('expires_at', `gt:${new Date().toISOString()}`)
                .single();
            
            if (error) {
                console.error('Failed to fetch governance context:', error);
                setLoading(false);
                return;
            }
            
            // 2. Fetch the actual policy
            const policyId = data.active_policy_id;
            const { data: policyData, error: policyError } = await supabase
                .from('governance_policies')
                .select('*')
                .eq('policy_id', policyId)
                .single();
            
            if (policyError) {
                console.error('Failed to fetch policy:', policyError);
            } else {
                setPolicy(policyData);
            }
            
            setLoading(false);
        };
        
        fetchPolicy();
        
        // 3. Subscribe to policy updates (Supabase Realtime)
        const subscription = supabase
            .on('postgres_changes',
                { event: 'UPDATE', schema: 'public', table: 'governance_context' },
                (payload) => {
                    if (payload.new.user_id === authUser.id) {
                        fetchPolicy(); // Refetch on policy change
                    }
                }
            )
            .subscribe();
        
        return () => {
            subscription.unsubscribe();
        };
    }, []);
    
    return { policy, loading };
}

// expo/src/components/ProcessStageSelector.tsx

export function ProcessStageSelector() {
    const { policy, loading } = useGovernanceContext();
    const [currentStage, setCurrentStage] = useState<string>('OrderReceived');
    const [availableStages, setAvailableStages] = useState<string[]>([]);
    
    useEffect(() => {
        if (!policy) return;
        
        // 1. Get allowed stages from policy
        const allowed = policy.allowed_stages;
        
        // 2. Filter out forbidden transitions
        const valid = allowed.filter(stage => {
            const transition = [currentStage, stage];
            return !policy.forbidden_transitions.some(
                forbidden => 
                    forbidden.from === transition[0] && 
                    forbidden.to === transition[1]
            );
        });
        
        setAvailableStages(valid);
    }, [policy, currentStage]);
    
    const handleStageChange = async (newStage: string) => {
        // 3. Verify transition is allowed before submitting
        if (!availableStages.includes(newStage)) {
            Alert.alert(
                'Transition Not Allowed',
                `Cannot transition from ${currentStage} to ${newStage} due to governance policy.`
            );
            return;
        }
        
        // 4. Submit to Supabase (RLS will enforce again)
        const { error } = await supabase
            .from('process_instances')
            .update({ current_stage: newStage })
            .eq('process_id', currentProcessId)
            .select()
            .single();
        
        if (error) {
            // RLS rejection
            Alert.alert('Update Rejected', `Policy enforcement: ${error.message}`);
        } else {
            setCurrentStage(newStage);
        }
    };
    
    return (
        <View>
            <Text>Current Stage: {currentStage}</Text>
            <ScrollView>
                {availableStages.map(stage => (
                    <TouchableOpacity
                        key={stage}
                        onPress={() => handleStageChange(stage)}
                        style={styles.stageButton}
                    >
                        <Text>{stage}</Text>
                    </TouchableOpacity>
                ))}
            </ScrollView>
        </View>
    );
}
```

---

## Policy Sync Cycle

Blue River emits governance policies to Supabase on a **10-minute heartbeat** cycle:

```
┌─────────────────────────────────────────────────────────────────┐
│              Blue River MAPE-K Cycle (Every 10 min)             │
└─────────────────────────────────────────────────────────────────┘

Time 14:00:00 - Monitor Phase
  ├─ Collect all ZOEapp events (last 10 minutes)
  ├─ Load OCEL event log from OTel Weaver
  └─ Create Evidence<ProcessEvent> entries

Time 14:00:10 - Analyze Phase
  ├─ Run conformance check
  ├─ Calculate fitness score
  ├─ Detect drift (if fitness < 0.95)
  ├─ Assess risk (fraud, compliance violations)
  └─ Generate Analysis artifact

Time 14:00:20 - Plan Phase
  ├─ If no drift detected:
  │   ├─ Keep current governance state
  │   ├─ Maintain current policy rules
  │   └─ Rate limits unchanged
  │
  ├─ If drift detected (fitness < 0.95):
  │   ├─ Propose corrections
  │   ├─ Restrict allowed_stages (e.g., disable high-risk stages)
  │   ├─ Increase rate_limit_burst constraints
  │   └─ Flag operators with high confidence
  │
  └─ Generate Plan artifact with risk assessment

Time 14:00:30 - Execute Phase
  ├─ Create GovernancePolicy object
  ├─ Sign policy with Ed25519 key
  ├─ POST policy to Supabase
  │   POST https://supabase.example.com/api/governance/policies
  │   Body: { policy_id, governance_state, allowed_roles, ... }
  │   Header: X-Signature: [ed25519_sig]
  │
  ├─ Receive response: { policy_id, status: "published" }
  └─ Emit OTel Span: governance.policy_published

Time 14:00:40 - Knowledge Phase
  ├─ Store policy ID + receipt in Blue River registry
  ├─ Store conformance_fitness + analysis results
  ├─ Update knowledge model (lessons learned)
  └─ Emit OTel Span: mape_k.cycle_complete

        ↓
┌─────────────────────────────────────────────────────────────────┐
│            Supabase RLS Policy Update                           │
└─────────────────────────────────────────────────────────────────┘

Receive policy:
  ├─ Verify Ed25519 signature against blue_river_governor_key
  ├─ Check policy expiration (must be in future)
  ├─ Insert into governance_policies table
  ├─ Run RLS policy trigger:
  │   verify_governance_policy_signature() → OK
  │
  └─ Update governance_context for all active sessions
      UPDATE governance_context SET active_policy_id = ?

        ↓
┌─────────────────────────────────────────────────────────────────┐
│            Expo App Receives Policy Update (Realtime)           │
└─────────────────────────────────────────────────────────────────┘

Supabase broadcasts:
  {
    "event": "UPDATE",
    "table": "governance_context",
    "payload": {
      "new": {
        "user_id": "user_123",
        "active_policy_id": "gov_pol_xyz789",
        "governance_state": "Monitor",
        "expires_at": "2026-06-01T14:10:00Z"
      }
    }
  }

Expo app:
  ├─ Receives realtime broadcast
  ├─ Calls useGovernanceContext() hook
  ├─ Fetches updated policy from Supabase
  ├─ Re-renders ProcessStageSelector component
  ├─ Updates available stages based on new policy
  └─ Notifies user if their allowed actions changed

Time 14:10:00 - Next MAPE-K Cycle
  ├─ Blue River repeats (conformance check, plan, execute, ...)
  └─ [Cycle continues indefinitely]
```

---

## Data Contracts

### GovernancePolicy JSON Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "GovernancePolicy",
  "description": "Policy issued by Blue River to Supabase",
  "type": "object",
  "required": [
    "policy_id",
    "governance_state",
    "allowed_roles",
    "allowed_stages",
    "forbidden_transitions",
    "rate_limit_per_minute",
    "rate_limit_per_hour",
    "rate_limit_burst",
    "signature",
    "signing_key_id",
    "created_at",
    "expires_at"
  ],
  "properties": {
    "policy_id": {
      "type": "string",
      "pattern": "^gov_pol_[a-f0-9]{8}$"
    },
    "governance_state": {
      "type": "string",
      "enum": ["Monitor", "Analyze", "Plan", "Execute", "Knowledge"]
    },
    "allowed_roles": {
      "type": "array",
      "items": { "type": "string" },
      "examples": [["operator", "auditor"]]
    },
    "allowed_stages": {
      "type": "array",
      "items": { "type": "string" },
      "examples": [["OrderReceived", "PaymentProcessing", "Fulfillment"]]
    },
    "forbidden_transitions": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["from", "to"],
        "properties": {
          "from": { "type": "string" },
          "to": { "type": "string" }
        }
      },
      "examples": [[{"from": "OrderReceived", "to": "Fulfillment"}]]
    },
    "rate_limit_per_minute": {
      "type": "integer",
      "minimum": 1,
      "maximum": 10000
    },
    "rate_limit_per_hour": {
      "type": "integer",
      "minimum": 1
    },
    "rate_limit_burst": {
      "type": "integer",
      "minimum": 1,
      "description": "Max concurrent requests allowed"
    },
    "data_access_rules": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["resource", "action", "condition", "roles"],
        "properties": {
          "resource": { "type": "string" },
          "action": { "enum": ["read", "write", "delete"] },
          "condition": { "type": "string" },
          "roles": { "type": "array", "items": { "type": "string" } }
        }
      }
    },
    "signature": {
      "type": "string",
      "pattern": "^[a-f0-9]{128}$",
      "description": "Ed25519 signature (64 bytes hex)"
    },
    "signing_key_id": {
      "type": "string",
      "pattern": "^blue_river_governor_v[0-9]+$"
    },
    "created_at": {
      "type": "string",
      "format": "date-time"
    },
    "expires_at": {
      "type": "string",
      "format": "date-time"
    }
  }
}
```

---

## Error Handling

| Scenario | Detection | Recovery |
|----------|-----------|----------|
| Supabase unreachable (timeout > 5s) | HTTP timeout | Retry with exponential backoff; keep local policy in Blue River |
| Policy signature invalid | Supabase trigger fails | Reject policy, log security alert, notify Blue River |
| Policy expired | expires_at < now() | Supabase RLS rejects, Expo app refetches |
| RLS conflict with app logic | Expo update rejected by RLS | Log incident, display error to user, escalate to compliance |
| User session expired during policy push | Token invalid | Fail gracefully, require re-authentication |
| Network partition between Expo and Supabase | Realtime subscription drops | Expo polls governance_context every 10s (fallback) |

---

## Acceptance Criteria

- [ ] Blue River can generate GovernancePolicy with valid Ed25519 signatures
- [ ] Supabase RLS policies enforce all forbidden transitions correctly
- [ ] Supabase RLS rate limits block requests exceeding policy limits
- [ ] Expo app receives governance policy updates within 2s via Realtime
- [ ] Expo UI disables forbidden transitions before user attempts them
- [ ] Policy expiration causes automatic RLS rejection of updates
- [ ] Signature verification failures are logged and escalated
- [ ] Network failures trigger exponential backoff without data loss
- [ ] JSON schema validation passes for all GovernancePolicy instances

---

**Authority:** Process Intelligence Research Foundry  
**Status:** Integration Specification (Ready for Implementation)
