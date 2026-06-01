# Autonomic Governance KPI Atlas

**Authority:** Process Intelligence Program
**Version:** 1.0
**Date:** 2026-06-01

---

## Manufacturing KPIs (Tier 1: Critical)

### KPI-001: Manufacturing Rule Success Rate
- **Definition:** Percentage of manufacturing operations that complete without error
- **Measurement:** `(successful_rules / total_rules) * 100`
- **Source:** ggen execution logs
- **Threshold (Green):** ≥ 98%
- **Threshold (Yellow):** 95-98%
- **Threshold (Red):** < 95%
- **Alert Latency:** 5 minutes
- **Action:** Route to ggen remediation engine

### KPI-002: Receipt Chain Integrity
- **Definition:** Percentage of receipts that cryptographically verify and form complete chains
- **Measurement:** `(valid_receipts / total_receipts) * 100`
- **Source:** receipt registry, Blue River Dam
- **Threshold (Green):** 100%
- **Threshold (Yellow):** 99-100%
- **Threshold (Red):** < 99%
- **Alert Latency:** 1 minute (cryptographic integrity is non-negotiable)
- **Action:** CRITICAL: Suspend manufacturing, audit receipt authority

### KPI-003: Van der Aalst Fitness Drift
- **Definition:** Process conformance fitness score vs. baseline
- **Measurement:** `fitness_current - fitness_baseline`
- **Source:** pm4py conformance checking
- **Threshold (Green):** ≥ -0.02 (drift acceptable if minimal)
- **Threshold (Yellow):** -0.05 to -0.02
- **Threshold (Red):** < -0.05
- **Alert Latency:** 30 minutes (allows for natural variance)
- **Action:** Route to lifecycle analyzer; flag for doctrine review

### KPI-004: Manufacturing Pipeline Latency
- **Definition:** P95 time from manufacturing trigger to receipt emission
- **Measurement:** 95th percentile of end-to-end latency
- **Source:** OTel instrumentation
- **Threshold (Green):** < 500ms
- **Threshold (Yellow):** 500ms - 1000ms
- **Threshold (Red):** > 1000ms
- **Alert Latency:** 10 minutes
- **Action:** Route to performance analyzer; check for bottlenecks

### KPI-005: Rule Authority Consistency
- **Definition:** Percentage of manufacturing rules that match declared doctrine
- **Measurement:** `(conforming_rules / total_rules) * 100`
- **Source:** doctrine scanner vs. active rules
- **Threshold (Green):** 100%
- **Threshold (Yellow):** 98-100%
- **Threshold (Red):** < 98%
- **Alert Latency:** 15 minutes
- **Action:** Route to ggen for rule sync; audit doctrine

---

## Data Flow KPIs (Tier 1: Critical)

### KPI-006: Cross-Project Data Flow Health
- **Definition:** Percentage of inter-project data transfers that complete without error
- **Measurement:** `(successful_transfers / total_transfers) * 100`
- **Source:** integration logs
- **Threshold (Green):** ≥ 99%
- **Threshold (Yellow):** 95-99%
- **Threshold (Red):** < 95%
- **Alert Latency:** 5 minutes
- **Action:** Route to integration analyzer; check schemas

### KPI-007: API Uptime
- **Definition:** Percentage of API availability window
- **Measurement:** `(available_seconds / total_seconds) * 100`
- **Source:** API health checks
- **Threshold (Green):** ≥ 99.9%
- **Threshold (Yellow):** 99-99.9%
- **Threshold (Red):** < 99%
- **Alert Latency:** 1 minute
- **Action:** Route to infrastructure agent; trigger failover if applicable

### KPI-008: API Latency (P95)
- **Definition:** 95th percentile API response time
- **Measurement:** P95 latency in milliseconds
- **Source:** OTel instrumentation
- **Threshold (Green):** < 100ms
- **Threshold (Yellow):** 100-200ms
- **Threshold (Red):** > 200ms
- **Alert Latency:** 5 minutes
- **Action:** Route to performance analyzer

### KPI-009: Schema Validation Pass Rate
- **Definition:** Percentage of incoming messages that pass schema validation
- **Measurement:** `(valid_messages / total_messages) * 100`
- **Source:** integration middleware
- **Threshold (Green):** ≥ 99.5%
- **Threshold (Yellow):** 98-99.5%
- **Threshold (Red):** < 98%
- **Alert Latency:** 5 minutes
- **Action:** Route to schema governance; audit incoming data source

---

## Lifecycle KPIs (Tier 2: Important)

### KPI-010: Lifecycle State Transition Validity
- **Definition:** Percentage of state transitions that comply with lifecycle rules
- **Measurement:** `(valid_transitions / total_transitions) * 100`
- **Source:** lifecycle state machine logs
- **Threshold (Green):** 100%
- **Threshold (Yellow):** 99-100%
- **Threshold (Red):** < 99%
- **Alert Latency:** 15 minutes
- **Action:** Route to lifecycle auditor; flag invalid transitions

### KPI-011: Artifact Completeness Score
- **Definition:** Average completeness of manufactured artifacts
- **Measurement:** `(complete_artifacts / total_artifacts) * 100`
- **Source:** artifact scanner
- **Threshold (Green):** ≥ 98%
- **Threshold (Yellow):** 95-98%
- **Threshold (Red):** < 95%
- **Alert Latency:** 30 minutes
- **Action:** Route to artifact quality analyzer

### KPI-012: Proof Gate Pass Rate
- **Definition:** Percentage of proof gates that accept artifacts
- **Measurement:** `(passing_gates / total_gates) * 100`
- **Source:** proof gate logs
- **Threshold (Green):** ≥ 97%
- **Threshold (Yellow):** 94-97%
- **Threshold (Red):** < 94%
- **Alert Latency:** 20 minutes
- **Action:** Route to gate auditor; review gate criteria

---

## Governance KPIs (Tier 2: Important)

### KPI-013: Doctrine Completeness
- **Definition:** Percentage of claims in doctrine that cite evidence sources
- **Measurement:** `(sourced_claims / total_claims) * 100`
- **Source:** doctrine scanner
- **Threshold (Green):** 100%
- **Threshold (Yellow):** 95-100%
- **Threshold (Red):** < 95%
- **Alert Latency:** Daily (batch)
- **Action:** Route to doctrine auditor; flag unsourced claims

### KPI-014: Gap Resolution Velocity
- **Definition:** Average days to resolve documented gaps
- **Measurement:** `total_gap_days / resolved_gaps`
- **Source:** gap tracker
- **Threshold (Green):** < 30 days
- **Threshold (Yellow):** 30-60 days
- **Threshold (Red):** > 60 days
- **Alert Latency:** Weekly
- **Action:** Route to program coordinator; escalate aging gaps

### KPI-015: Checkpoint Validity
- **Definition:** Percentage of checkpoints with all gate criteria satisfied
- **Measurement:** `(valid_checkpoints / total_checkpoints) * 100`
- **Source:** checkpoint validator
- **Threshold (Green):** 100%
- **Threshold (Yellow):** 95-100%
- **Threshold (Red):** < 95%
- **Alert Latency:** Upon checkpoint issuance
- **Action:** CRITICAL: Reject checkpoint; route to checkpoint auditor

---

## Event Log Quality KPIs (Tier 3: Observability)

### KPI-016: Event Completeness
- **Definition:** Percentage of events with all required attributes
- **Measurement:** `(complete_events / total_events) * 100`
- **Source:** OTel log validator
- **Threshold (Green):** ≥ 99%
- **Threshold (Yellow):** 95-99%
- **Threshold (Red):** < 95%
- **Alert Latency:** 30 minutes
- **Action:** Route to instrumentation auditor

### KPI-017: Event Log Conformance
- **Definition:** Percentage of event logs that conform to OCEL 2.0 spec
- **Measurement:** `(conforming_logs / total_logs) * 100`
- **Source:** OCEL validator
- **Threshold (Green):** 100%
- **Threshold (Yellow):** 98-100%
- **Threshold (Red):** < 98%
- **Alert Latency:** 30 minutes
- **Action:** Route to OCEL auditor

---

## Summary

| KPI # | Name | Tier | Alert Latency | Owner |
|-------|------|------|---------------|-------|
| 001 | Manufacturing Rule Success Rate | 1 | 5m | ggen engine |
| 002 | Receipt Chain Integrity | 1 | 1m | Blue River Dam |
| 003 | Van der Aalst Fitness Drift | 1 | 30m | Lifecycle Analyzer |
| 004 | Manufacturing Pipeline Latency | 1 | 10m | Performance Analyzer |
| 005 | Rule Authority Consistency | 1 | 15m | ggen engine |
| 006 | Cross-Project Data Flow Health | 1 | 5m | Integration Analyzer |
| 007 | API Uptime | 1 | 1m | Infrastructure |
| 008 | API Latency (P95) | 1 | 5m | Performance Analyzer |
| 009 | Schema Validation Pass Rate | 1 | 5m | Schema Governance |
| 010 | Lifecycle State Transition Validity | 2 | 15m | Lifecycle Auditor |
| 011 | Artifact Completeness Score | 2 | 30m | Artifact Analyzer |
| 012 | Proof Gate Pass Rate | 2 | 20m | Gate Auditor |
| 013 | Doctrine Completeness | 2 | Daily | Doctrine Auditor |
| 014 | Gap Resolution Velocity | 2 | Weekly | Program Coordinator |
| 015 | Checkpoint Validity | 2 | Immediate | Checkpoint Auditor |
| 016 | Event Completeness | 3 | 30m | Instrumentation Auditor |
| 017 | Event Log Conformance | 3 | 30m | OCEL Auditor |

---

## Escalation Policy

**Tier 1 (Critical):** Automatic escalation → immediate human review + automated remediation attempt
**Tier 2 (Important):** Batched review → daily analysis → routing to appropriate handler
**Tier 3 (Observability):** Batch collection → weekly synthesis → continuous improvement

---

## See Also

- `autonomic-config.yaml` — KPI threshold and alert configuration
- `monitors/` — SPARQL queries that derive KPI values
- `analyzers/` — Root cause analysis rules
- `policy-rules.ttl` — Governance policy in RDF
