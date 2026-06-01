# Paper-to-M&A Claim Mapping Samples

**Version:** 1.0  
**Status:** EXPERIMENT  
**Last Updated:** 2026-05-31  
**Purpose:** Show how paper algorithms → board-admissible M&A claims → slide receipts

---

## Overview

This document demonstrates:
1. **Paper Algorithm** → Process Mining Metric
2. **Process Metric** → M&A Claim Type
3. **M&A Claim** → Board Slide with Receipt Evidence
4. **Claim Quantification** → Financial/Risk Impact

All mappings are backed by formal paper citations and wasm4pm proof gates.

---

## Sample 1: Fitness ≥ 0.95 → Buyer Reliance Claim

### Paper Foundation
**[PC-001] PM4Py: Alignment-based Conformance**

- Adriansyah et al. (2014) alignment-based fitness metric
- Board-admissibility threshold: **f ≥ 0.95** (< 5% process drift)
- Evidence artifact: TokenReplayReceipt with per-trace fitness

### Type-Law
```
Fitness : [0, 1]
Board Admissible IFF Fitness ≥ 0.95
Receipt Proof: TokenReplayReceipt { aggregate_fitness, per_trace_results, signature }
```

### Paper-to-M&A Mapping

| Field | Source | Value | Authority |
|-------|--------|-------|-----------|
| **Paper Metric** | PC-001 PM4Py | Token Replay Fitness | Berti et al. (2023) |
| **Calculation** | Math Foundation | f(L, N) = 1 - Σ cost(σ) / Σ worst_cost(σ) | Adriansyah (2014) |
| **Board Claim Type** | Diligence Taxonomy | "Buyer Reliance" | PC-001 conformance proof |
| **Claim Statement** | M&A Deck | "Process model certified by conformance proof (fitness ≥ 0.95)" | Token Replay Receipt |
| **Financial Impact** | Valuation Model | Reduces audit cost by $500K/year | Risk haircut averted |
| **Verification** | Proof Gate | Re-run conformance query; verify fitness | wasm4pm signature |

### Slide Deck Representation

**Slide Title:** "Process Conformance: Model-to-Reality Match"

```
┌─────────────────────────────────────────────────────────────┐
│ BUYER RELIANCE CLAIM: Process Executed to Model             │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  Fitness Score:  95.8% ✓ (Above board threshold of 95%)    │
│                                                              │
│  What It Means:                                              │
│  • 95.8% of observed process events match declared model     │
│  • Only 4.2% variance attributable to known exceptions       │
│  • System-enforced controls captured in model                │
│                                                              │
│  Evidence:                                                   │
│  • Event log: 24,567 cases, 487,234 events                  │
│  • Model: 47 activities, 92 control-flow rules               │
│  • Conformance: Optimal alignment (Adriansyah 2014)          │
│  • Receipt Hash: abc123def456xyz...                         │
│                                                              │
│  Board Impact:                                               │
│  → Audit risk MITIGATED (scope reduced 40%)                 │
│  → Control environment VALIDATED (SOX 404 relief)           │
│  → Integration cost REDUCED (known process shapes)           │
│                                                              │
│  Signed by: wasm4pm execution core v4.5.2                   │
│  Timestamp: 2026-05-31T12:00:00Z                            │
│  Claim ID: fitness-claim-2026-05-31-001                     │
└─────────────────────────────────────────────────────────────┘
```

### Financial Valuation Formula

```
Audit Cost Reduction = 
  Base_Audit_Cost × (1 - Fitness_Delta) × Risk_Multiplier
  = $2,000,000 × (1 - (1 - 0.958)) × 0.5
  = $2,000,000 × 0.042 × 0.5
  = $42,000 audit scope reduction (conservative)
  
Transaction Impact:
  Buyer saves $42K in audit fees (direct)
  + $500K in post-close control validation (indirect)
  + $1M in integration planning (risk reduction)
  = ~$1.5M valuation uplift to buyer
```

### Receipt Evidence

```json
{
  "receipt_id": "fitness-claim-2026-05-31-001",
  "claim_type": "Buyer Reliance: Process Conformance",
  "metric": "Token Replay Fitness",
  "value": 0.958,
  "threshold_required": 0.95,
  "board_admissible": true,
  "event_log_id": "order-to-cash-jan-2026.xes",
  "case_count": 24567,
  "event_count": 487234,
  "model": {
    "activities": 47,
    "control_flows": 92,
    "soundness_proven": true
  },
  "per_activity_fitness": {
    "Create Order": 1.0,
    "Validate": 0.98,
    "Assign": 0.95,
    "Ship": 0.99,
    "Invoice": 0.97,
    "Reconcile": 0.92
  },
  "variant_analysis": {
    "total_variants": 342,
    "model_supports": 340,
    "unsupported_variance": 2,
    "rare_variants": ["emergency_delivery", "credit_hold_override"]
  },
  "conformance_proof": {
    "algorithm": "Optimal Alignment (Adriansyah 2014)",
    "alignment_cost_per_case": 0.042,
    "move_on_log_count": 2047,
    "move_on_model_count": 156,
    "execution_time_seconds": 487
  },
  "signature": "sig_wasm4pm_2026_05_31_12_00_00",
  "timestamp": "2026-05-31T12:00:00Z"
}
```

---

## Sample 2: Precision ≥ 0.90 → Operational Debt Mitigation

### Paper Foundation
**[PC-001] PM4Py: Precision Metric**

- Precision measures overfitting: low precision = model allows behavior not in log
- Board-admissibility threshold: **p ≥ 0.90** (< 10% unobserved paths)
- Evidence artifact: PrecisionReceipt with reachable markings analysis

### Type-Law
```
Precision : [0, 1]
Board Admissible IFF Precision ≥ 0.90
Receipt Proof: PrecisionReceipt { 
  enabled_transitions_per_marking,
  observed_transitions_per_marking,
  reachable_markings_visited,
  signature 
}
```

### Paper-to-M&A Mapping

| Field | Source | Value | Authority |
|-------|--------|-------|-----------|
| **Paper Metric** | PC-001 PM4Py | Alignment-based Precision | Berti et al. (2023) |
| **Calculation** | Math Foundation | p(L, N) = 1 - Σ unobserved / Σ enabled | Adriansyah (2014) |
| **Board Claim Type** | Operational Debt Taxonomy | "Process Spaghetti Debt" | Trace entropy H(L) |
| **Claim Statement** | M&A Deck | "Precision ≥ 0.90: Model correctly bounds process behavior" | Precision Receipt |
| **Financial Impact** | Debt Valuation | Each 10% precision loss = $200K automation cost | Rework activity overhead |
| **Verification** | Proof Gate | Replay model on control-flow graph; count enabled vs observed | wasm4pm alignment |

### Slide Deck Representation

**Slide Title:** "Process Model Precision: Automation Readiness"

```
┌─────────────────────────────────────────────────────────────┐
│ OPERATIONAL DEBT MITIGATION: Process Boundaries Defined     │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  Precision Score:  91.2% ✓ (Above board threshold of 90%)  │
│                                                              │
│  What It Means:                                              │
│  • Model allows only 91.2% known process paths               │
│  • Just 8.8% unobserved process behavior in model            │
│  • Automation can proceed with high confidence               │
│                                                              │
│  Evidence:                                                   │
│  • Reachable markings visited: 1,247                         │
│  • Enabled transitions per marking: 3.2 (avg)               │
│  • Observed in log: 2.91 (avg)                              │
│  • Unobserved branches: 0.29 per marking (low)              │
│                                                              │
│  Operational Debt Impact:                                    │
│  → Spaghetti Risk: H(L) = 1.8 (acceptable, <3.0)            │
│  → Automation Ready: Low hidden path risk                    │
│  → Custom Logic Needed: Minimal (only 8.8% unobserved)      │
│                                                              │
│  Savings Projection:                                         │
│  → Automation feasible: $400K labor reduction               │
│  → Exception handling: $85K for 8.8% unobserved paths       │
│  → Net automation benefit: +$315K                            │
│                                                              │
│  Signed by: wasm4pm execution core v4.5.2                   │
│  Timestamp: 2026-05-31T12:05:00Z                            │
│  Claim ID: precision-claim-2026-05-31-001                   │
└─────────────────────────────────────────────────────────────┘
```

### Financial Valuation Formula

```
Automation Feasibility = Precision_Score × Completeness_Factor

Automation Cost = 
  Base_Automation_Cost × (1 - Precision_Score) × Rework_Multiplier
  = $500,000 × (1 - 0.912) × 1.0
  = $500,000 × 0.088
  = $44,000 exception-handling budget

Net Synergy (Automation) =
  Labor_Savings × Feasibility - Exception_Handling_Cost
  = $400,000 × 0.912 - $44,000
  = $364,800 - $44,000
  = $320,800 net automation synergy

Operational Debt Reduction:
  Debt = Unobserved_Paths × Complexity × Rework_Cost
  = 0.088 × 2.1 × $150,000
  = $27,720 debt item (low risk)
```

### Receipt Evidence

```json
{
  "receipt_id": "precision-claim-2026-05-31-001",
  "claim_type": "Operational Debt: Process Model Precision",
  "metric": "Alignment-based Precision",
  "value": 0.912,
  "threshold_required": 0.90,
  "board_admissible": true,
  "event_log_id": "order-to-cash-jan-2026.xes",
  "reachable_markings_count": 1247,
  "avg_enabled_per_marking": 3.2,
  "avg_observed_per_marking": 2.91,
  "unobserved_paths_estimate": 0.29,
  "precision_formula": "1 - Σ(unobserved) / Σ(enabled)",
  "process_spaghetti": {
    "trace_entropy": 1.8,
    "entropy_threshold": 3.0,
    "spaghetti_risk": "LOW",
    "variant_count": 342,
    "rare_variant_percent": 2.1
  },
  "automation_readiness": {
    "ready_for_rpa": true,
    "exception_handling_needed": true,
    "exception_percent": 8.8,
    "custom_logic_blocks": 4
  },
  "signature": "sig_wasm4pm_2026_05_31_12_05_00",
  "timestamp": "2026-05-31T12:05:00Z"
}
```

---

## Sample 3: OCPQ Constraint Satisfaction ≥ 0.95 → Compliance Claim

### Paper Foundation
**[PC-005] OCPQ: Constraint-based Process Querying**

- Declarative constraints over event logs (DECLARE-style)
- Violations quantified; satisfaction score [0,1]
- Board-admissibility threshold: **satisfaction ≥ 0.95** (< 5% violations)

### Type-Law
```
ConstraintSatisfactionScore : [0, 1]
Board Admissible IFF Score ≥ 0.95
Receipt Proof: OCPQReceipt { 
  constraints_checked,
  violation_count,
  satisfied_cases,
  aggregate_score,
  signature 
}
```

### Paper-to-M&A Mapping

| Field | Source | Value | Authority |
|-------|--------|-------|-----------|
| **Paper Metric** | PC-005 OCPQ | Constraint Satisfaction | DECLARE patterns |
| **Constraint Types** | Temporal Rules | Precedence, Response, ChainPrecedence | Process compliance rules |
| **Board Claim Type** | Diligence Claim Taxonomy | "Policy Compliance Verification" | Control environment |
| **Claim Statement** | M&A Deck | "95.2% of cases comply with declared controls; 73 violations documented" | OCPQ Receipt |
| **Financial Impact** | Control Debt Valuation | Each 1% violation = $50K control remediation | Compliance remediation cost |
| **Verification** | Proof Gate | Re-evaluate OCPQ constraints; compare violation sets | wasm4pm constraint engine |

### Slide Deck Representation

**Slide Title:** "Control Compliance: Process Rules Enforcement"

```
┌─────────────────────────────────────────────────────────────┐
│ BUYER RELIANCE: Process Constraints Enforced (Controls)     │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  Compliance Score:  95.2% ✓ (Above board threshold of 95%)  │
│                                                              │
│  Constraints Validated:  12 critical rules checked          │
│                                                              │
│  Constraint Matrix:                                          │
│  • Invoice before Payment:        96.1% (73 violations)      │
│  • Approval before Payment:       98.3% (29 violations)      │
│  • SoD: Create ≠ Approve:         100% (0 violations)        │
│  • Spending Limit Enforced:       94.5% (143 violations)     │
│  • Reconciliation within 30d:     92.1% (183 violations)     │
│  [+ 7 more constraints]                                      │
│                                                              │
│  Violation Details:                                          │
│  • Total violations: 731 cases (of 24,567)                  │
│  • Repeated violators: 88 cases (serial offenders)          │
│  • One-time exceptions: 643 cases (isolated issues)         │
│                                                              │
│  Risk Assessment:                                            │
│  → Regulatory risk: MEDIUM (SOX 404 control gaps)           │
│  → Fraud risk: LOW (SoD controls working)                   │
│  → Financial misstatement risk: MEDIUM                       │
│                                                              │
│  Remediation Cost:                                           │
│  → For 5.8% noncompliance: ~$290K remediation              │
│  → Buyer haircut: $145K-290K valuation adjustment           │
│                                                              │
│  Signed by: wasm4pm execution core v4.5.2                   │
│  Timestamp: 2026-05-31T12:10:00Z                            │
│  Claim ID: compliance-claim-2026-05-31-001                  │
└─────────────────────────────────────────────────────────────┘
```

### Financial Valuation Formula

```
Control Remediation Cost = 
  Violation_Rate × Total_Cases × Cost_Per_Violation × Risk_Multiplier
  = 0.058 × 24,567 × $25 × 1.5
  = 1,425 cases × $25 × 1.5
  = ~$53,450 remediation cost (labor + process redesign)

Post-Close Compliance Risk:
  Risk_Value = Remaining_Violations × Probability × Financial_Impact
  = 73 × 0.02 (2% restatement probability) × $1M materiality
  = $1.46M breach risk (priced into deal)

Buyer Adjustment:
  Valuation Haircut = min(Remediation_Cost × 2, Risk_Value × 0.5)
  = min($106,900, $730,000)
  = $106,900 SPA adjustment (cap on escrow)
```

### Receipt Evidence

```json
{
  "receipt_id": "compliance-claim-2026-05-31-001",
  "claim_type": "Buyer Reliance: Compliance Control Verification",
  "metric": "OCPQ Constraint Satisfaction",
  "aggregate_score": 0.952,
  "threshold_required": 0.95,
  "board_admissible": true,
  "event_log_id": "order-to-cash-jan-2026.xes",
  "total_cases": 24567,
  "constraints_checked": 12,
  "constraints_satisfied": 10,
  "violation_summary": {
    "total_violations": 731,
    "violation_rate": 0.0297,
    "critical_violations": 73,
    "non_critical_violations": 658
  },
  "constraint_results": [
    {
      "constraint_id": "Invoice_Before_Payment",
      "constraint_type": "Precedence",
      "satisfied_cases": 23551,
      "violated_cases": 1016,
      "satisfaction_score": 0.959,
      "violation_details": ["case_12345", "case_12346"]
    },
    {
      "constraint_id": "Approval_Before_Payment",
      "constraint_type": "Precedence",
      "satisfied_cases": 23936,
      "violated_cases": 631,
      "satisfaction_score": 0.974,
      "violation_details": []
    },
    {
      "constraint_id": "SoD_Create_Not_Approve",
      "constraint_type": "Exclusion",
      "satisfied_cases": 24567,
      "violated_cases": 0,
      "satisfaction_score": 1.0,
      "violation_details": []
    }
  ],
  "risk_assessment": {
    "regulatory_risk_level": "MEDIUM",
    "fraud_risk_level": "LOW",
    "financial_misstatement_risk": "MEDIUM",
    "estimated_remediation_cost": 53450,
    "breach_probability": 0.02,
    "material_breach_amount": 1000000
  },
  "signature": "sig_wasm4pm_2026_05_31_12_10_00",
  "timestamp": "2026-05-31T12:10:00Z"
}
```

---

## Sample 4: Soundness ≥ 95% Verification → Seller Defensibility

### Paper Foundation
**[PC-002] YAWL: Workflow Net Soundness**

- Soundness: proper termination + no deadlock + no livelock
- van der Aalst (1998) formal verification
- Board-admissibility: soundness **proven offline; bundled with model**

### Type-Law
```
Soundness : {Verified(hash), Assumed(reason)}
Board Admissible IFF Verified (not Assumed)
Receipt Proof: SoundnessReceipt { 
  net_structure,
  proof_artifact_hash,
  verification_method,
  signature 
}
```

### Paper-to-M&A Mapping

| Field | Source | Value | Authority |
|-------|--------|-------|-----------|
| **Paper Metric** | PC-002 YAWL | WF-net Soundness | van der Aalst (1998) |
| **Verification Method** | Formal Methods | Option-to-complete + no-deadlock + bounded | Petri net analysis |
| **Board Claim Type** | Seller Defensibility | "No control-flow defects; process termination guaranteed" | Soundness proof |
| **Claim Statement** | M&A Deck | "Soundness verified: process always terminates lawfully" | SoundnessReceipt |
| **Financial Impact** | Liability Reduction | $500K+ litigation risk avoided if process is sound | Undisclosed process bugs |
| **Verification** | Proof Gate | Assert soundness_proof matches bundled artifact | wasm4pm signature validation |

### Slide Deck Representation

**Slide Title:** "Process Structural Integrity: Soundness Proven"

```
┌─────────────────────────────────────────────────────────────┐
│ SELLER DEFENSIBILITY: Process Model is Mathematically Sound │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  Soundness Status: ✓ VERIFIED                               │
│                                                              │
│  What "Sound" Means:                                         │
│  • Every case initiated will eventually complete            │
│  • No deadlocks (processes stuck indefinitely)              │
│  • No livelocks (infinite loops with no progress)           │
│  • Proper final marking reached (control ends cleanly)      │
│                                                              │
│  Verification Method:                                       │
│  • Formal proof: State-space reachability analysis           │
│  • Coverage: All 92 control-flow paths verified             │
│  • Proof artifact: 512-bit SHA hash (immutable)             │
│  • Verification timestamp: 2026-04-15 (pre-deal)            │
│                                                              │
│  Key Properties Proven:                                      │
│  • Option-to-complete: 24,567/24,567 cases (100%)           │
│  • No dead transitions: All 47 activities reachable         │
│  • Bounded net: Max tokens per place: 3 (safe)             │
│  • Liveness: All tasks fire in ≥1 execution path            │
│                                                              │
│  Seller Protection:                                          │
│  → No undisclosed process bugs hidden in model              │
│  → No control-flow defects that could expose buyer          │
│  → Litigation risk from failed process termination: ZERO    │
│  → Buyer cannot claim process was "broken" post-close       │
│                                                              │
│  Proof Hash: abc123def456xyz789...                          │
│  Authority: van der Aalst Soundness Framework               │
│                                                              │
│  Signed by: wasm4pm execution core v4.5.2                   │
│  Timestamp: 2026-05-31T12:15:00Z                            │
│  Claim ID: soundness-claim-2026-05-31-001                   │
└─────────────────────────────────────────────────────────────┘
```

### Financial Valuation Formula

```
Seller Risk Mitigation = 
  Soundness_Risk_Factor × Undisclosed_Liability

Where:
  Soundness_Risk_Factor = 
    Probability(process_terminates_illegally) × Materiality
    = (1 - Soundness_Proof_Validity) × Estimated_Failure_Cost
    = 0.0 × $500K (if soundness proven)
    = $0 risk exposure (if verified)

Buyer's Perspective:
  If NOT proven sound:
    Post-Close Integration Risk = $500K-$2M
    (undefined behavior, cases stuck, lawsuits from customers)
  
  If proven sound:
    Post-Close Integration Risk = $0
    (seller has eliminated control-flow liability)
```

### Receipt Evidence

```json
{
  "receipt_id": "soundness-claim-2026-05-31-001",
  "claim_type": "Seller Defensibility: WF-net Soundness",
  "metric": "Formal Soundness Proof (van der Aalst 1998)",
  "soundness_status": "VERIFIED",
  "proof_requirement": "MANDATORY",
  "board_admissible": true,
  "net_structure": {
    "places": 47,
    "transitions": 92,
    "arcs": 187,
    "source_place": "p_init",
    "sink_place": "p_final",
    "max_tokens_per_place": 3
  },
  "soundness_properties": {
    "option_to_complete": {
      "property": "Every reachable marking can reach final marking",
      "verified": true,
      "cases_verified": 24567,
      "cases_succeeded": 24567
    },
    "proper_completion": {
      "property": "Final marking has exactly 1 token in sink, 0 elsewhere",
      "verified": true,
      "cases_verified": 24567,
      "cases_proper": 24567
    },
    "no_dead_transitions": {
      "property": "Every transition fires in at least 1 path",
      "verified": true,
      "dead_transitions": 0,
      "total_transitions": 92
    },
    "boundedness": {
      "property": "No unbounded token accumulation",
      "verified": true,
      "max_tokens_reached": 3,
      "memory_safe": true
    }
  },
  "verification_details": {
    "method": "State-space reachability analysis",
    "reachable_markings": 1247,
    "state_space_explored": 100,
    "proof_execution_time_seconds": 287,
    "verification_tool": "TAPAAL (Timed Arc Petri Nets)",
    "tool_version": "3.9.1"
  },
  "proof_artifact": {
    "hash_algorithm": "SHA-512",
    "hash_value": "abc123def456xyz789...",
    "signature_date": "2026-04-15T09:30:00Z",
    "expiry_date": null,
    "proof_is_immutable": true
  },
  "seller_defensibility": {
    "undisclosed_process_bugs": 0,
    "control_flow_defects": 0,
    "litigation_risk_from_soundness": "$0",
    "seller_liability_eliminated": true
  },
  "signature": "sig_wasm4pm_2026_05_31_12_15_00",
  "timestamp": "2026-05-31T12:15:00Z"
}
```

---

## Summary: Paper ➔ M&A Claim ➔ Slide ➔ Receipt Mapping

| Paper | Metric | M&A Claim | Slide Title | Board Threshold | Risk Impact |
|-------|--------|-----------|-------------|-----------------|------------|
| PC-001 | Fitness | Buyer Reliance | Process Conformance | ≥ 0.95 | Audit scope: $42K saving |
| PC-001 | Precision | Operational Debt | Model Precision | ≥ 0.90 | Automation feasible: +$320K |
| PC-005 | Constraint Satisfaction | Policy Compliance | Control Compliance | ≥ 0.95 | Control risk: $53K remediation |
| PC-002 | Soundness | Seller Defensibility | Structural Integrity | Verified | Litigation risk: $500K avoided |

Each claim:
- Backed by peer-reviewed paper
- Quantified by formal algorithm
- Verified by wasm4pm proof gate
- Signed cryptographically
- Board-presentable (< 5% materiality)

---

## Status: COMPLETE

**M&A Claim Samples Provided:** 4 (Conformance, Precision, Compliance, Soundness)  
**Paper Citations:** 3 (PC-001, PC-002, PC-005)  
**Financial Impact Quantified:** ✓ (all four samples)  
**Board Slide Mockups:** ✓ (executive-ready language)  
**Receipt Evidence Format:** ✓ (JSON + cryptographic signatures)  
**Valuation Formulas:** ✓ (transaction-impact pricing)  

Ready for M&A diligence integration and deal-closing board presentation.
