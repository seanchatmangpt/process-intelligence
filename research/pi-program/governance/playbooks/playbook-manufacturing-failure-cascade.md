# Playbook: Manufacturing Failure Cascade Remediation

**Authority:** Autonomic Governance Engine
**Trigger:** Policy 002 - High Rule Failure Rate (success_rate < 95%)
**Severity:** CRITICAL
**Auto-Execute:** YES (Tier 1)
**SLA:** 15 minutes to remediation attempt

---

## Problem Description

Manufacturing rules fail in cascades: one rule's failure triggers downstream dependencies to fail, creating exponential failure amplification. This playbook detects the cascade pattern and remediates the root cause.

---

## Detection Signals

1. **Multiple rule failures at different times**
   - Rule A fails at T=0s
   - Rules B, C, D fail at T=5s (within root cause window)
   - Rules E, F, G fail at T=10s
   - Pattern indicates rule A outputs feed rules B, C, D, etc.

2. **Confidence: HIGH** when:
   - Temporal ordering is deterministic
   - Data flow graph confirms dependency
   - Failure messages match "missing input" or "invalid schema"

---

## Diagnosis Steps

### Step 1: Identify Root Cause Rule (0-2 minutes)

**Action:** Run `analyze-rule-failures.sparql` → detect `CascadingFailure` pattern

**Expected Output:**
```
root_rule: ggen:rule_12345
affected_rules: [ggen:rule_6789, ggen:rule_2345, ...]
confidence: 0.92
remediation: "Fix rule_12345 output schema"
```

**What we're looking for:**
- Single rule whose failure precedes all others
- No other rule generates the same output
- High temporal correlation

### Step 2: Analyze Root Cause (1-5 minutes)

**Action:** Fetch rule definition and recent execution trace

**Query:**
```sparql
SELECT ?rule ?definition ?last_error_log
WHERE {
  ggen:rule_12345 ggen:definition ?definition .
  ggen:rule_12345 ggen:last_error_log ?last_error_log .
}
```

**What to look for:**
- Schema mismatch between rule output and rule's contracts
- Resource exhaustion during execution
- Recent code changes that broke compatibility
- Invalid input data patterns

### Step 3: Classify Failure Type (1-3 minutes)

**Decision Tree:**

```
Is failure rate > 90% and consistent?
├─ YES: Resource exhaustion or architectural issue
│  └─ Action: Scale resources (see Playbook: Resource Scaling)
│
├─ NO: Pattern in error messages?
│  ├─ "schema validation failed": Data quality issue
│  │  └─ Action: Audit input data, update schemas (Playbook: Schema Fix)
│  │
│  ├─ "timeout": Latency/performance issue
│  │  └─ Action: Optimize rule, parallelize (Playbook: Performance Fix)
│  │
│  ├─ "null pointer / panic": Code defect
│  │  └─ Action: Review recent commits, revert if necessary
│  │
│  └─ "rule not found" or "permission denied": Configuration error
│     └─ Action: Fix ggen configuration (Playbook: Config Fix)
```

---

## Remediation Execution

### Phase 1: Immediate Stabilization (0-5 minutes)

**1A. Emit Governance Alert**
```
Severity: CRITICAL
Message: "Manufacturing rule cascade detected: root_rule=rule_12345, affected=7 downstream rules"
Target: PagerDuty + Slack #governance-alerts
SLA: immediate human awareness
```

**1B. Suspend Affected Rules**
```
ggen.suspend_rules([
  "rule_12345",  # Root cause
  "rule_6789",   # Dependent 1
  "rule_2345",   # Dependent 2
  ...
])
```
- Stops cascade amplification
- Preserves logs for analysis
- Timeout: 1 minute max

**Expected outcome:** No more rule failures from this cascade

### Phase 2: Root Cause Repair (5-15 minutes)

**2A. Automatic Fixes (if high confidence)**

Option A1: **Schema Mismatch Fix**
```
If failure reason = "schema validation failed":
  1. Query rule_12345 output schema
  2. Query downstream input schemas
  3. Run schema reconciliation:
     ggen.reconcile_schemas(
       rule_12345,
       downstream_rules=[rule_6789, rule_2345, ...],
       strategy="align_to_producer"  # Trust rule_12345's declared output
     )
  4. Emit governance receipt documenting schema change
```

Option A2: **Resource Scaling Fix**
```
If failure reason matches resource exhaustion pattern:
  1. Identify bottleneck resource (CPU/memory/network)
  2. Calculate required scaling factor:
     new_capacity = current_capacity * (p99_latency / p50_latency) * 1.25
  3. Scale infrastructure:
     infrastructure.scale_resources(
       rule_12345,
       cpu_increase_percent=25,
       memory_increase_percent=25
     )
  4. Emit governance receipt documenting infrastructure change
```

**2B. Manual Investigation (if low confidence or high risk)**
```
If confidence < 0.75 OR fix has wide blast radius:
  1. Route to ggen remediation engineer
  2. Engineer reviews failure trace + code
  3. Engineer chooses remediation (fix code, revert commit, etc.)
  4. Changes enter normal CI/CD pipeline
  5. Suspended rules resume once fix deployed
  
  Timeout: 30 minutes → escalate to human review
```

### Phase 3: Verification (10-15 minutes)

**3A. Resume Suspended Rules**
```
ggen.resume_rules([rule_12345, rule_6789, rule_2345, ...])
```

**3B. Monitor Success**
```
FOR next 5 minutes:
  success_rate = count(successful_executions) / count(total_executions)
  IF success_rate >= 0.98:
    → Remediation SUCCESS; close alert
  IF success_rate >= 0.90 and < 0.98:
    → Remediation PARTIAL; continue monitoring, engage SRE
  IF success_rate < 0.90:
    → Remediation FAILED; rollback
```

**3C. Rollback on Failure**
```
If verification fails:
  1. Suspend rules again
  2. Revert changes:
     ggen.revert_to_last_known_good(rule_12345)
  3. Escalate to SRE team for manual investigation
  4. Emit FAILURE receipt with rollback details
```

---

## Post-Remediation

### Knowledge Update
```
governance.record_incident(
  incident_type: "manufacturing_cascade",
  root_cause: "rule_12345 schema mismatch",
  failure_count: 7,
  duration_minutes: 12,
  remediation_type: "automatic_schema_fix",
  remediation_effectiveness: 0.98,
  timestamp: now()
)
```

### Pattern Learning
```
IF this cascade pattern occurs again:
  # Increase confidence threshold for auto-fix next time
  pattern_detector.update_confidence(
    pattern: "cascading_failure_with_schema_mismatch",
    new_confidence: 0.95
  )
```

### Doctrine Review
```
Did this reveal a gap in doctrine?
  → If YES: Create gap document (gaps/GAP_XXX_mfg_cascade_pattern.md)
  → If YES: Update doctrine if needed
```

---

## Escalation

**If automatic remediation fails:**
1. T+15m: Alert escalates to SRE team (if not already engaged)
2. T+30m: Alert escalates to governance board
3. T+60m: CRITICAL incident declared; manufacturing suspended; manual remediation only

---

## Success Criteria

- [x] Cascade detected within alert latency (5 minutes)
- [x] Root cause correctly identified (confidence >= 0.75)
- [x] Remediation executed (automatic or manual)
- [x] Success rate restored to >= 98% within 15 minutes
- [x] Receipt issued documenting incident and fix
- [x] No cascading failures from this root cause in next 24 hours

---

## Testing

**Chaos Simulation (to test this playbook):**

```yaml
chaos_test:
  name: "manufacturing_cascade_simulation"
  inject_failure:
    rule: "ggen:rule_12345"
    failure_mode: "schema_mismatch"
    probability: 1.0
    duration: "PT5M"
  expected_behavior:
    - Detection latency < 5 minutes
    - Root cause identified correctly
    - Cascading failures prevented (downstream rules suspended)
    - Automatic remediation succeeds
    - Success rate >= 0.98 within 15 minutes
```

---

## See Also

- `policy-rules.ttl` — Policy 002 definition
- `analyze-rule-failures.sparql` — Root cause analysis rules
- `playbook-schema-fix.md` — Schema mismatch remediation
- `playbook-resource-scaling.md` — Resource scaling remediation
