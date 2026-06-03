# Source Index — experiments-core

All source files read during thesis chapter manufacture for this project.

| File | Description |
|---|---|
| `/Users/sac/process-intelligence/experiments/checkpoint__experiments_complete.md` | Checkpoint declaring VERIFIED & COMPLETE for equation-verification and 18/18 experiment coverage gates |
| `/Users/sac/process-intelligence/experiments/EVIDENCE_CHAIN_TRACE.md` | 7-step evidence chain from Raw OCEL to Board Claim; documents GAP_001 (broken Ocel20 witness thread) |
| `/Users/sac/process-intelligence/experiments/verify_equations.py` | Unified gate: 5 fitness cases + 4 OCPQ cases; reports 100% correctness on pass |
| `/Users/sac/process-intelligence/experiments/validate_log_fitness.py` | 5 Petri net token replay assertions; exact floating-point fitness results |
| `/Users/sac/process-intelligence/experiments/validate_ocpq_refinement.py` | 4 OCPQ refinement relation assertions (reflexivity, extension, conflict, missing domain) |
| `/Users/sac/process-intelligence/experiments/audit__experiment_completeness.md` | Audit recording 18/18 experiments PASSED with concrete JSON schemas, zero stubs |
| `/Users/sac/process-intelligence/experiments/pm4py_vs_wasm4pm_capability_matrix.md` | Capability matrix comparing wasm4pm and PM4Py across 8 algorithm dimensions |
| `/Users/sac/process-intelligence/experiments/pm4py_vs_compat_type_boundary_matrix.md` | Type-boundary comparison between wasm4pm-compat and PM4Py type systems |
| `/Users/sac/process-intelligence/experiments/petri_conformance_sample.md` | Petri net token replay sample with concrete JSON schema |
| `/Users/sac/process-intelligence/experiments/replay_receipt_sample.md` | CryptographicReplayReceipt schema + instance (fitness=0.982, BLAKE3 hashes, Ed25519 signature) |
| `/Users/sac/process-intelligence/experiments/ocel_lifecycle_sample.md` | OCEL 2.0 lifecycle sample with concrete JSON event/object schema |
| `/Users/sac/process-intelligence/experiments/RAW_LAUNDERING_REFUSAL_SAMPLE.md` | Refusal<DanglingEventObjectLink, Ocel20> schema demonstrating raw laundering prevention |
| `/Users/sac/process-intelligence/experiments/otel_trace_integration_sample.md` | OTel-BLAKE3 receipt chain integration with span-level receipt instances |
| `/Users/sac/process-intelligence/experiments/paper-to-m&a-claim_mapping_sample.md` | Mapping from academic paper findings to board-admissible M&A synergy claims |
| `/Users/sac/process-intelligence/experiments/paper-fixture-design/spec.md` | Specification for adversarial GAN-based fixture manufacturing (not yet implemented) |
| `/Users/sac/process-intelligence/experiments/pm4py-comparison/matrix.md` | Performance benchmark: DFG 37.5x, Alpha Miner 34.2x, Token Replay 39.3x speedup |
| `/Users/sac/process-intelligence/experiments/pm4py-comparison/zero-knowledge-benchmarks.md` | ZKP benchmark: wasm4pm 2^18 cycles vs PM4Py 2^30+ timeout; 220KB STARK |
| `/Users/sac/process-intelligence/experiments/wasm4pm-compat-evaluation/report.md` | wasm4pm-compat type-boundary evaluation report |
| `/Users/sac/process-intelligence/experiments/visualizer/README.md` | Browser-based process mining dashboard overview (A*, EWMA, SHA-256 audit chain, LTL, Petri animation) |
| `/Users/sac/process-intelligence/experiments/visualizer/bindings.d.ts` | TypeScript type projections of Rust typestate across WASM boundary (EvidenceTs, AdmissionTs, etc.) |
| `/Users/sac/process-intelligence/experiments/visualizer/visualizer-validation.ts` | TypeScript type-level validation of 10 WASM boundary types |
| `/Users/sac/process-intelligence/experiments/visualizer/alignment.js` | A* alignment solver (sync moves cost 0, log/model-only moves cost 1, BFS heuristic) |
| `/Users/sac/process-intelligence/experiments/visualizer/blockchain.js` | SHA-256 cryptographic audit chain with genesis block and back-linked hash per event |
| `/Users/sac/process-intelligence/experiments/visualizer/drift-detector.js` | EWMA drift detector (lambda=0.20, L=3.00 sigma, mu0=5.0s baseline, UCL/LCL limits) |
