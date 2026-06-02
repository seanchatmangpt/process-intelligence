# Source Index: research-pi-program

All source files read during thesis chapter manufacture for this project.
Paths are relative to `/Users/sac/process-intelligence/`.

| File | Description |
|---|---|
| `research/pi-program/manufacturing/MANUFACTURING_SUMMARY.md` | Phase 2 manufacturing receipt ledger; 123 artifacts across 6 sub-projects; 100% BLAKE3 coverage claim; gate verification for receipt completeness, hash algorithm, and source rule documentation. |
| `research/pi-program/audits/audit-results.yaml` | 12-gate Van der Aalst conformance audit; 10 PASS, 2 FAIL (audit_005 DTO flattening CRITICAL, audit_012 remediation routing); overall verdict PARTIAL. |
| `research/pi-program/governance/AUTONOMIC_ACTIVATION_LOG_20260601.yaml` | MAPE-K autonomic activation log; 5 feedback loop tests all PASSED (avg confidence 0.96); 4/4 deployment gates PASS; 6 drift detection rules; 5 Andon gates; Blue River Dam registration. |
| `research/pi-program/checkpoints/PI_GGEN_UNIFIED_RUN_PARTIAL_001.md` | Unified ggen manufacturing run checkpoint; 4/14 gates PASS; 5 critical blockers (A-E); 12-hour remediation plan; zero pipelines executed end-to-end. |
| `research/pi-program/ggen/ontology/pi-program.ttl` | Top-level program-role OWL ontology; defines pi:ProgramRole and 31 subclasses including PROGRAM, ENGINE, COMPATIBILITY_LAYER, MANUFACTURING_CELL, PROOF_CELL; first 80 lines read. |
| `research/pi-program/manufacturing/RECEIPT_CHAIN_VERIFICATION_20260601.yaml` | Independent chain verifier; 28 receipts audited; 8 chain breaks; 10 hash mismatches (template hashes vs actual BLAKE3); gate criteria not met. |
