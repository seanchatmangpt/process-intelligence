# Source Index — audits project

All source files read during manufacture of the 8 TeX files for this project.

| File | Description |
|------|-------------|
| `/Users/sac/process-intelligence/audits/README_AUDITS.md` | Top-level guide to the Van der Aalst conformance auditing program; defines the 5-audit structure, violation summary, and remediation roadmap |
| `/Users/sac/process-intelligence/audits/VAN_DER_AALST_AUDIT_SUMMARY.txt` | Quick-reference audit execution summary with all violation details and reproducibility instructions |
| `/Users/sac/process-intelligence/audits/CONFORMANCE_REPORT.md` | Human-readable executive conformance report; documents 4/5 PASS / 1 FAIL verdict and graduation path |
| `/Users/sac/process-intelligence/audits/AUDIT_LOG.yaml` | Machine-readable consolidated audit results in structured YAML; canonical source of record for violations and GraduationReason mappings |
| `/Users/sac/process-intelligence/audits/forensic_audit_verdict.md` | Clean-room forensic audit confirming 84 passing tests, zero stubs/mocks/hardcoded outputs, and pure-Rust cryptographic implementation |
| `/Users/sac/process-intelligence/audits/petri_net_soundness_audit.md` | Mathematical verification of WF-net soundness (option-to-complete, proper completion, no dead transitions) against van der Aalst 1997/1998/2011 and Murata 1989 |
| `/Users/sac/process-intelligence/audits/AUDIT_PAPER_COVERAGE.md` | Paper coverage audit of 81 papers across six verdict categories; identifies inflated COVERED_BY_TYPE claims |
| `/Users/sac/process-intelligence/audits/AUDIT_TYPE_LAW_QUALITY.md` | Type-law quality audit of 199 compile-fail fixtures; documents E0308/E0277/E0599 distribution and zero-E0425 finding |
| `/Users/sac/process-intelligence/audits/AUDIT_FIXTURE_QUALITY.md` | Companion fixture quality audit confirming 97.7% valid law-proving error code coverage |
| `/Users/sac/process-intelligence/audits/alignment_referee_audit.md` | A* alignment referee certification: admissibility proof for h(n)=|L(n)-M(n)|, empirical 74-microsecond latency, state-space safety limits |
| `/Users/sac/process-intelligence/audits/drift_sentry_audit.md` | EWMA concept drift detection audit: LCL=0.92 mathematical derivation, 19 empirical test cases, 4-trace detection latency |
| `/Users/sac/process-intelligence/audits/telemetry_auditor_audit.md` | Telemetry auditor report with conflicting XES/OCEL compliance verdict (0% OCEL 2.0, 2/5 XES) |
| `/Users/sac/process-intelligence/audits/ledger_custodian_audit.md` | SHA-256 ledger integrity audit; documents test_sha256_ledger_compliance Rust integration test and genesis block hash |
| `/Users/sac/process-intelligence/audits/stream_director_audit.md` | Stream director compliance audit declaring PASS on telemetry surface after DOM remediation |
| `/Users/sac/process-intelligence/audits/adversarial_audit_v30.1.1.md` | ALIVE_001 adversarial certification: 567 commits, 12 corpus directories at threshold, BLAKE3+Ed25519 signature |
| `/Users/sac/process-intelligence/audits/audit-execution-boundaries.md` | Execution boundary audit mapping compat vs. wasm4pm capability surfaces |
| `/Users/sac/process-intelligence/audits/audit-type-law-coverage.md` | Type-law coverage audit across the projection surface |
| `/Users/sac/process-intelligence/audits/neuro-symbolic-verification.md` | Neuro-symbolic verification pipeline: GNN heuristic + SMT/Z3 symbolic verification against paradoxical RDF topologies; references Chatman Equation |
| `/Users/sac/process-intelligence/audits/rebranding_ontology_audit.md` | Ontology rebranding audit identifying ZOEapp-to-PCP TTL renames required (unresolved) |
| `/Users/sac/process-intelligence/audits/EXECUTION_MANIFEST.txt` | Execution manifest listing all audit artifacts and their roles |
| `/Users/sac/process-intelligence/audits/audit-lifecycle-completeness.md` | Lifecycle completeness audit: 5 residual gaps in wasm4pm execution boundary mapping across Design and Simulation stages |
| `/Users/sac/process-intelligence/audits/audit-board-claim-support.md` | Board claim support audit confirming traceability for all 9 claim types across 4 strategic domains (EBITDA, Working Capital, GRC, Integration Velocity) |
| `/Users/sac/process-intelligence/audits/audit-no-dto-flattening.txt` | Receipt file for Audit 1 — current status: FAIL (DTO_001 violation) |
| `/Users/sac/process-intelligence/audits/audit-no-tool-smuggling.txt` | Receipt file for Audit 2 — current status: PASS |
| `/Users/sac/process-intelligence/audits/audit-feature-isolation.txt` | Receipt file for Audit 3 — current status: PASS |
| `/Users/sac/process-intelligence/audits/audit-projection-receipt.txt` | Receipt file for Audit 4 — current status: PASS |
| `/Users/sac/process-intelligence/audits/audit-graduation-boundary.txt` | Receipt file for Audit 5 — current status: PASS |
