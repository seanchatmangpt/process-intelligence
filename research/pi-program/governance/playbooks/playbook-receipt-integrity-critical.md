# Playbook: Receipt Chain Integrity Critical Incident

**Authority:** Autonomic Governance Engine
**Trigger:** Policy 001 - Receipt Chain Integrity Violation (KPI_002 < 99%)
**Severity:** CRITICAL (non-negotiable)
**Auto-Execute:** YES (immediate)
**SLA:** 1 minute alert latency, 5 minutes to stabilization

---

## Critical Context

Receipt chain integrity is **non-negotiable**. If cryptographic hashes fail to verify or the chain becomes broken, we cannot trust manufacturing history, artifact provenance, or governance auditability.

**Consequences of inaction:**
- M&A claims become unverifiable
- Board admissibility gates cannot be satisfied
- Process intelligence loses legal standing
- Manufacturing authority is compromised

---

## Immediate Actions (T+0 to T+1 minute)

### Action 1: Alert and Escalation (T+0)
```
ALERT LEVEL: CRITICAL
MESSAGE: "Receipt chain integrity violation detected: {violation_count} invalid receipts"
TARGETS:
  - PagerDuty critical severity
  - Slack @governance-oncall
  - Email: security@process-intelligence.local
EXPECTED RESPONSE: Immediate human attention
```

### Action 2: Suspend All Manufacturing (T+0)
```
ggen.halt_manufacturing_global()
  - Stop all rule execution immediately
  - Preserve all logs and traces
  - Keep manufacturing rules suspended until explicit resume
  - Emit suspension receipt to chain
TIMEOUT: None (suspension is indefinite until cleared)
```

### Action 3: Preserve Evidence (T+0-1 minute)
```
forensics.snapshot_state():
  - Dump full RDF store (governance knowledge graph)
  - Snapshot OTel traces from past 24 hours
  - Export receipt registry
  - Capture filesystem state
  - Location: /Users/sac/process-intelligence/research/pi-program/governance/incidents/{incident_id}/
CRITICAL: Do not modify; this is forensic evidence
```

---

## Diagnosis Phase (T+1 to T+5 minutes)

### Step 1: Classify Violation Type (T+1-2 minutes)

**Run:** `analyze-receipt-corruption.sparql`

**Possible violation types:**

1. **Cryptographic Hash Mismatch** (tampering / corruption)
   - Declared hash ≠ verified hash
   - Severity: CRITICAL
   - Likely cause: bit-flip, malicious tampering, key compromise
   - Action path: "TAMPERING_PROTOCOL"

2. **Chain Break** (missing intermediate receipts)
   - Receipt at position N, then receipt at position N+2 (N+1 missing)
   - Severity: CRITICAL
   - Likely cause: partial write failure, missing emission
   - Action path: "CHAIN_RECOVERY_PROTOCOL"

3. **Timestamp Anomaly** (time inversion / replay attack)
   - Receipt at position N has later timestamp than position N+1
   - Severity: HIGH
   - Likely cause: clock skew, replay attack, out-of-order emission
   - Action path: "TIMESTAMP_VERIFICATION_PROTOCOL"

4. **Authority Signature Failure** (key compromise)
   - Multiple signature verification failures from same authority
   - Severity: CRITICAL
   - Likely cause: private key compromise, authority breach
   - Action path: "KEY_ROTATION_PROTOCOL"

5. **Data Corruption Pattern** (systemic storage issue)
   - Multiple receipts with identical bit positions corrupted
   - Severity: CRITICAL
   - Likely cause: storage subsystem failure, ECC failure
   - Action path: "STORAGE_RECOVERY_PROTOCOL"

---

## Action Paths

### ACTION PATH: TAMPERING_PROTOCOL

**Assumption:** One or more receipts are cryptographically invalid

**Step 1: Isolate Tampered Receipts (T+2)**
```
tampered = []
for receipt in chain:
  if !receipt.crypto.verify():
    tampered.append(receipt)
    
log("Isolated tampered receipts:", len(tampered))
```

**Step 2: Find Last Known Good Receipt (T+3)**
```
last_good = None
for i in range(len(chain) - 1, -1, -1):
  if chain[i].crypto.verify():
    last_good = chain[i]
    break
    
if last_good is None:
  escalate("TOTAL_LOSS: No valid receipts found. Manual recovery required.")
else:
  log("Last known good receipt:", last_good.id)
```

**Step 3: Audit Blue River Dam Authority (T+4)**
```
blue_river.audit():
  - Verify Blue River's private key (not compromised)
  - Check Blue River's hashing algorithm (not corrupted)
  - Review recent Blue River code changes
  - Check Blue River's audit logs
REPORT: "Blue River Authority Status: [OK | COMPROMISED | UNKNOWN]"
```

**Step 4: Decision (T+5)**
```
if blue_river_status == "OK":
  escalate("Storage corruption suspected. Audit storage subsystem.")
  action_path = "STORAGE_RECOVERY_PROTOCOL"
  
elif blue_river_status == "COMPROMISED":
  escalate("Blue River authority compromised. Initiate key rotation.")
  action_path = "KEY_ROTATION_PROTOCOL"
  
else:  # UNKNOWN
  escalate("Unable to determine cause. Manual investigation required.")
  action_path = "ESCALATION_ONLY"
```

---

### ACTION PATH: CHAIN_RECOVERY_PROTOCOL

**Assumption:** Some receipts are missing from the chain

**Step 1: Identify Missing Positions (T+2)**
```
gaps = []
for i in range(1, len(chain)):
  if chain[i].position != chain[i-1].position + 1:
    gap_start = chain[i-1].position + 1
    gap_end = chain[i].position - 1
    gaps.append((gap_start, gap_end))
    
log("Found gaps:", gaps)
```

**Step 2: Search for Missing Receipts (T+3)**
```
for (gap_start, gap_end) in gaps:
  missing = []
  for pos in range(gap_start, gap_end + 1):
    receipt = blue_river.lookup_receipt_by_position(pos)
    if receipt is None:
      missing.append(pos)
    else:
      # Found orphaned receipt; integrate it
      chain.insert_receipt(receipt)
      
log("Unrecoverable missing positions:", missing)
```

**Step 3: Determine Recovery Strategy (T+4)**
```
if len(missing) == 0:
  # All missing receipts recovered; chain is complete
  log("Chain recovered successfully")
  action = "VERIFY_AND_RESUME"
  
elif len(missing) < 5:
  # Small gap; may be able to reissue
  escalate("Small gap in receipt chain. Blue River can reissue receipts.")
  action_path = "BLUE_RIVER_REISSUE_PROTOCOL"
  
else:
  # Large gap; requires human decision
  escalate("Large gap in receipt chain ({} missing). Manual recovery required.")
  action = "ESCALATION_ONLY"
```

---

### ACTION PATH: KEY_ROTATION_PROTOCOL

**Assumption:** Private key is compromised

**Step 1: Revoke Current Key (T+2 - IMMEDIATE)**
```
blue_river.revoke_key(current_key_id)
  - Emit revocation to all dependent systems
  - Mark all receipts signed with this key as "SIGNED_WITH_REVOKED_KEY"
  - Timeout: None (revocation is immediate)
```

**Step 2: Generate New Key (T+3)**
```
new_key = blue_river.generate_new_key()
blue_river.activate_key(new_key)
log("New key activated:", new_key.id)
```

**Step 3: Resign Affected Receipts (T+4-10)**
```
for receipt in chain:
  if receipt.signing_key_id == revoked_key_id:
    new_receipt = blue_river.resign_receipt(receipt, new_key)
    chain[receipt.position] = new_receipt
    
log("Resignature complete")
```

**Step 4: Verify New Chain (T+10)**
```
if chain.verify_integrity() == true:
  action = "VERIFY_AND_RESUME"
else:
  escalate("Chain verification still fails after key rotation.")
  action = "ESCALATION_ONLY"
```

---

### ACTION PATH: STORAGE_RECOVERY_PROTOCOL

**Assumption:** Storage subsystem has a fault (e.g., ECC failure, bit-flip)

**Step 1: Identify Storage Issue (T+2-3)**
```
if corruption_pattern.common_bit_positions:
  # Systematic bit pattern suggests ECC failure
  log("Bit pattern:", corruption_pattern.common_bit_positions)
  escalate("Storage ECC failure detected. Contact infrastructure team.")
else:
  # Random corruption suggests bit-flip or drive failure
  escalate("Random bit corruption detected. Evaluate storage hardware.")
```

**Step 2: Restore from Backup (T+5)**
```
backup_chain = restore_from_backup(timestamp="24h_ago")
if backup_chain.verify_integrity() == true:
  log("Backup integrity verified; using backup chain")
  chain = backup_chain
  action = "VERIFY_AND_RESUME"
else:
  escalate("Backup chain is also corrupted. Check backup infrastructure.")
  action = "ESCALATION_ONLY"
```

---

### ACTION PATH: VERIFY_AND_RESUME

**Step 1: Final Chain Verification (T+5-10)**
```
verification_report = chain.verify_full_integrity():
  - All hashes compute correctly
  - All signatures verify with current keys
  - All positions are consecutive
  - All timestamps are monotonically increasing
  - All cross-references are valid
  
if verification_report.all_pass == true:
  log("Chain integrity verified ✓")
else:
  escalate("Chain verification failed. Details:", verification_report.failures)
  action = "ESCALATION_ONLY"
  return
```

**Step 2: Resume Manufacturing (T+10)**
```
ggen.resume_manufacturing_gradual():
  - Resume 1 rule at a time
  - Monitor for new failures
  - If any failure: suspend immediately
  - Complete resume takes 5-10 minutes
  
log("Manufacturing resumed")
```

**Step 3: Emit Incident Receipt (T+12)**
```
receipt = blue_river.emit_incident_receipt():
  - incident_type: "receipt_chain_integrity_violation"
  - violation_type: (from diagnosis)
  - root_cause: (from diagnosis)
  - duration_minutes: T
  - resolution: "SUCCESSFUL" | "ESCALATED"
  - corrupted_receipt_ids: [...]
  - recovery_strategy: (which protocol was used)
  - timestamp: now()
  
governance.record_incident(receipt)
```

---

### ACTION PATH: ESCALATION_ONLY

**When automatic protocols cannot resolve:**

1. **Emit Critical Incident Ticket**
   - Platform: Jira / internal ticket system
   - Priority: P0 (highest)
   - Assignee: Governance Board + Security Team
   - SLA: 4 hours to response

2. **Stop All Manufacturing**
   - Remain in suspended state
   - Do not resume automatically
   - Await manual human decision

3. **Convene Incident Review**
   - Forensic analysis of all evidence
   - Determine root cause
   - Plan manual recovery
   - Execute recovery steps

---

## Post-Remediation Verification

### Verification Checklist

- [ ] Chain integrity KPI >= 100% (all receipts cryptographically valid)
- [ ] Manufacturing rules resume successfully
- [ ] Manufacturing rule success rate >= 98% (no fallout from incident)
- [ ] No receipt anomalies in past 1 hour
- [ ] Blue River authority passes health check
- [ ] Incident receipt issued and recorded
- [ ] Governance knowledge graph updated with incident data

### Doctrine Review

**Did this incident reveal a gap in doctrine?**

- If YES: Create gap document (gaps/GAP_XXX_receipt_chain_integrity_protocol.md)
- If YES: Update COVENANT.md or doctrine files as needed

---

## Testing This Playbook

**Chaos Simulation:**

```yaml
chaos_test:
  name: "receipt_chain_integrity_critical_test"
  inject_failure:
    type: "hash_corruption"
    target: "random_receipt"
    timestamp: "now"
  inject_failure:
    type: "signature_failure"
    count: 3
    author: "blue_river_authority"
  expected_behavior:
    - Alert within 1 minute
    - Manufacturing suspended within 1 minute
    - Root cause identified correctly
    - Appropriate remediation protocol selected
    - Chain integrity restored
    - Manufacturing resumes without errors
```

---

## See Also

- `policy-rules.ttl` — Policy 001 definition
- `kpi-atlas.md` — KPI-002 definition
- `analyze-receipt-corruption.sparql` — Violation analysis rules
- `audit-trail.log` — All governance decisions logged here
