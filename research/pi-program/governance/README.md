# Autonomic Governance Engine

**Date:** 2026-06-01
**Authority:** Process Intelligence Program
**Status:** OPERATIONAL

The Autonomic Governance Engine is a self-governing MAPE-K feedback loop that continuously monitors process intelligence ecosystem health, automatically detects problems, analyzes root causes, plans remediation, executes fixes, and learns from incidents.

---

## What Is This?

A **fully autonomous system** that:
1. **Monitors** 17 critical KPIs every 1-10 minutes
2. **Analyzes** root causes using SPARQL rules + temporal correlation
3. **Plans** remediation steps based on policies and playbooks
4. **Executes** automatic fixes with staged verification and rollback
5. **Learns** from incidents by recording them in knowledge graph

**Goal:** Manufacturing is always healthy. When it's not, the system fixes it automatically.

---

## Directory Structure

```
governance/
├── README.md                                  # This file
├── GOVERNANCE_GUIDE.md                        # Complete operational guide
├── OPERATIONAL_CHECKLIST.md                   # Pre-launch and post-launch tasks
├── kpi-atlas.md                               # All 17 KPI definitions
├── policy-rules.ttl                           # Governance policies (RDF)
├── autonomic-config.yaml                      # MAPE-K loop configuration
├── autonomic-loop-executor.py                 # Main MAPE-K orchestrator
├── test-autonomic-loop.py                     # Simulation demo (manufacturing cascade)
│
├── monitors/                                  # KPI collection queries (SPARQL)
│   ├── monitor-manufacturing-rules.rq         # KPI-001: rule success rate
│   ├── monitor-receipt-chain.rq               # KPI-002: receipt integrity
│   ├── monitor-fitness-drift.rq               # KPI-003: conformance fitness
│   └── monitor-pipeline-latency.rq            # KPI-004: P95 latency
│
├── analyzers/                                 # Root cause analysis (SPARQL)
│   ├── analyze-rule-failures.sparql           # Detect cascades, resource issues
│   └── analyze-receipt-corruption.sparql      # Detect tampering, key compromise
│
├── playbooks/                                 # Remediation procedures
│   ├── playbook-manufacturing-failure-cascade.md
│   ├── playbook-receipt-integrity-critical.md
│   └── (more playbooks for other failure types)
│
├── logs/                                      # Runtime logs
│   └── autonomic-governance.log
│
├── audit-trail.log                            # All governance decisions
├── decisions/                                 # Decision artifacts (JSON)
└── receipts/                                  # Incident receipts
    └── remediations/                          # Remediation receipts
```

---

## Quick Start

### 1. Prerequisites
- Python 3.8+
- RDF store (rdf4j) running at `http://localhost:8080/rdf4j-server`
- OpenTelemetry collector at `http://localhost:4317`
- External handlers reachable (ggen, Blue River Dam, infrastructure APIs)

### 2. Start the Loop

```bash
cd /Users/sac/process-intelligence/research/pi-program/governance

# Run a few test cycles
python3 autonomic-loop-executor.py

# Or run continuously
nohup python3 autonomic-loop-executor.py > logs/autonomic-governance.log 2>&1 &
```

### 3. Monitor Loop Health

```bash
# Watch logs
tail -f logs/autonomic-governance.log

# Check recent decisions
ls -lt decisions/ | head -5

# View incident receipts
ls -lt receipts/remediations/ | head -5
```

### 4. Test with Simulated Incident

```bash
# Run the demonstration
python3 test-autonomic-loop.py

# Expected output:
# ✓ Cascade detected within alert latency
# ✓ Root cause correctly identified
# ✓ Remediation executed successfully
# ✓ KPIs recovered
# ✓ Receipt issued
# AUTONOMIC GOVERNANCE ENGINE: OPERATIONAL
```

---

## Key Features

### Tier 1: Critical Policies (Automatic, Immediate)
| Policy | KPI | Trigger | Action | SLA |
|--------|-----|---------|--------|-----|
| Receipt Integrity | KPI-002 | < 99% | **SUSPEND_MANUFACTURING**, audit encryption | 1m |
| Rule Failure Cascade | KPI-001 | < 95% success | Analyze cascade, fix root cause | 5m |
| API Uptime | KPI-007 | < 99.9% uptime | Failover or restart | 1m |

### Tier 2: Important Policies (Manual, Routed)
| Policy | KPI | Trigger | Routing | SLA |
|--------|-----|---------|---------|-----|
| Fitness Drift | KPI-003 | drift < -0.05 | lifecycle_analyzer | 30m |
| Proof Gate Pass Rate | KPI-012 | < 94% pass | gate_auditor | 20m |
| Doctrine Completeness | KPI-013 | < 95% sourced | doctrine_auditor | daily |

### Tier 3: Observability Policies (Batch Analysis)
- Event log completeness
- OCEL conformance
- Continuous improvement

---

## The MAPE-K Cycle

```
MONITOR (T+1m)
  └─> Query 17 KPIs
  └─> Identify breaches
  └─> Alert if needed

ANALYZE (T+3m)
  └─> Root cause analysis (SPARQL rules)
  └─> Pattern detection (temporal, dependency)
  └─> Confidence scoring

PLAN (T+5m)
  └─> Match KPI → Policy
  └─> Select playbook
  └─> Generate stages with timeouts
  └─> Estimate duration, set SLA

EXECUTE (T+10m)
  └─> Dispatch to handler (ggen, Blue River, infrastructure)
  └─> Run stages sequentially
  └─> Verify between stages
  └─> Rollback on failure
  └─> Emit receipt

KNOWLEDGE (T+23m)
  └─> Record incident
  └─> Update pattern library
  └─> Detect doctrine gaps
  └─> Assess effectiveness
  └─> Provide feedback to MONITOR
```

**Total cycle time:** ~23 minutes for remediation, continuous monitoring

---

## Real-World Example: Manufacturing Failure Cascade

**Scenario:** Manufacturing rule outputs schema X, but downstream rules expect X'. Cascade failure affects 4+ rules.

**What happens:**

1. **MONITOR** (T+1m): KPI-001 drops to 89.3% (threshold: 95%) → RED alert
2. **ANALYZE** (T+3m): Root cause detected as `ggen:rule_12345` with 92% confidence (temporal ordering, data flow match)
3. **PLAN** (T+5m): Policy 002 triggered → Plan 4-stage schema reconciliation
4. **EXECUTE** (T+10m):
   - Stage 1: Analyze failures (120s)
   - Stage 2: Classify failure type (60s)
   - Stage 3: Reconcile schemas (300s)
   - Stage 4: Verify success_rate >= 98% (300s)
5. **KNOWLEDGE** (T+23m):
   - Incident recorded
   - Effectiveness: 98% (KPI recovered from 89.3% → 98.9%)
   - Gap detected: schema validation not in doctrine → GAP_002 created

**Result:** Manufacturing back to health in 23 minutes, no human intervention.

---

## Governance Authority Chain

```
Process Intelligence Program (Authority)
    ↓
Autonomic Governance Engine (Executor)
    ├─ MONITOR Layer (Data Collection)
    ├─ ANALYZE Layer (Diagnosis)
    ├─ PLAN Layer (Policy Application)
    ├─ EXECUTE Layer (Remediation)
    └─ KNOWLEDGE Layer (Learning)
    
    Human Escalation Path:
    Automatic Remediation → SRE On-Call → Governance Board → Program Authority
```

**Non-negotiables:**
1. Receipt chain integrity (KPI-002) = 100% or manufacturing is SUSPENDED
2. Manufacturing rule success (KPI-001) ≥ 98% continuously
3. All remediation actions emit receipts to Blue River Dam (immutable audit trail)
4. No policy changes without governance board approval
5. All doctrine claims must cite evidence (paper, experiment, checkpoint)

---

## Integration Points

### Inputs (What the engine observes)
- **OTel traces** from manufacturing rules (ggen)
- **Receipt ledger** from Blue River Dam
- **Process models** and event logs (pm4py)
- **Infrastructure metrics** (Prometheus)
- **API health** checks

### Outputs (What the engine affects)
- **ggen rule execution** (pause/resume/update)
- **Blue River governance updates** (receipt chain)
- **Infrastructure actions** (failover, scaling)
- **Data governance** (schema fixes)
- **Doctrine updates** (evidence-based)

### Knowledge Graph
- **Storage:** RDF store (rdf4j)
- **Format:** Turtle (.ttl)
- **Contents:** Incident history, patterns, policies, doctrine

---

## Failure Modes and Recovery

### If MONITOR fails (can't collect KPIs)
→ RDF store down or network issue
→ Action: Verify RDF store connectivity, restart if needed
→ Loop halts until monitoring restored

### If ANALYZE fails (root cause analysis errors)
→ SPARQL queries return wrong confidence
→ Action: Review analyzer rules, check evidence quality
→ Escalate to SRE for manual investigation

### If PLAN fails (can't create remediation plan)
→ No policy matches KPI or policy is malformed
→ Action: Add policy or verify policy-rules.ttl syntax
→ Escalate to governance board

### If EXECUTE fails (remediation doesn't work)
→ Handler returns error or stages fail
→ Action: Attempt rollback, escalate to appropriate owner
→ Emit FAILED receipt with details

### If KNOWLEDGE fails (can't record incident)
→ RDF store write fails or receipt can't be issued
→ Action: **CRITICAL** — investigate immediately
→ May indicate Blue River Dam infrastructure issue

---

## Documentation Map

| Document | Purpose | Audience |
|----------|---------|----------|
| `README.md` (this file) | Architecture overview | Everyone |
| `GOVERNANCE_GUIDE.md` | Complete operational guide | Operators, SRE |
| `OPERATIONAL_CHECKLIST.md` | Launch readiness, post-launch tasks | Operators |
| `kpi-atlas.md` | KPI definitions, thresholds, alert latencies | Metrics team |
| `policy-rules.ttl` | Governance policies, routing, escalation | Policy team |
| `autonomic-config.yaml` | MAPE-K loop configuration | Operators, engineers |
| `playbooks/` | Remediation procedures | SRE, engineers |

---

## Testing

### Unit: Individual components
```bash
python3 -m pytest tests/  # Run unit tests (add tests/ dir)
```

### Integration: MAPE-K cycle
```bash
python3 test-autonomic-loop.py  # Simulates manufacturing cascade
```

### System: In production
```bash
# Enable chaos testing in autonomic-config.yaml
testing:
  chaos_testing:
    enabled: true
    inject_rule_failures: true
    inject_receipt_corruption: true
```

---

## Future Enhancements

1. **Predictive Analysis** — Forecast KPI breaches before they happen
2. **ML Anomaly Detection** — Learn normal vs. anomalous patterns
3. **Self-Tuning Policies** — Automatically adjust thresholds
4. **Multi-Agent Coordination** — Coordinate across ecosystem
5. **Hypothesis-Driven Learning** — Test remediations, measure effectiveness

---

## Authority and Approval

| Role | Name | Date | Status |
|------|------|------|--------|
| Implementation | Autonomic Governance Engine | 2026-06-01 | OPERATIONAL |
| Authority | Process Intelligence Program | 2026-06-01 | APPROVED |
| Doctrine Compliance | Van der Aalst Constitution | 2026-06-01 | COMPLIANT |
| Gate Verdict | PASS | 2026-06-01 | ✓ |

---

## Support

For operational issues, questions, or escalations:

1. **Check:** `GOVERNANCE_GUIDE.md` (troubleshooting section)
2. **Search:** `audit-trail.log` for similar past incidents
3. **Escalate:** `governance@process-intelligence.local`
4. **Emergency:** Page on-call SRE if manufacturing is affected

---

Last Updated: 2026-06-01
Authority: Process Intelligence Program
Status: OPERATIONAL

