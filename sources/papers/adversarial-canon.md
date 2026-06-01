# Adversarial Process Mining: Canon v30.1.1

## Abstract
This document evaluates the `workflow-ledger` paradigm against canonical process mining algorithms in the presence of adversarial environments. We demonstrate that traditional algorithms structurally fail when subjected to Sybil attacks and non-deterministic reality receipts. 

## The Workflow Ledger and Canonical Shortcomings
Traditional process mining assumes benign event logs where identifiers consistently map to ground-truth reality. The `workflow-ledger` introduces the concept of cryptographically anchored reality receipts, exposing the fragility of canonical models.
	### Sybil Attacks on Event Correlation
Canonical algorithms rely on temporal proximity and case IDs for correlation. In an adversarial setting, a Sybil attacker can forge thousands of interleaved case IDs and events, causing state-space explosion and invalidly discovering cyclic or overly permissive process models (Spaghetti models). The lack of BLAKE3 cryptographic receipts means there is no provenance to filter malicious events.

### Non-Deterministic Reality Receipts
Process execution in reality is subject to non-deterministic faults, latency, and out-of-band state changes. Traditional models expect deterministic sequential or concurrent interleavings. When `workflow-ledger` emits non-deterministic reality receipts, canonical discovery algorithms (like Alpha, Heuristics Miner, or Inductive Miner) misinterpret these as noise or novel deviations, leading to incorrect fitness and precision metrics. The ledger enforces typestate progression, invalidating standard replay semantics.
	## Conclusion
Adversarial process mining requires shifting from trusting event logs to verifying reality receipts. The `workflow-ledger` necessitates a new class of Byzantine-fault-tolerant discovery algorithms.