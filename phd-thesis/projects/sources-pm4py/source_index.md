# Source Index — sources-pm4py

All source files read and used in manufacturing the 8 TeX files for this project.

## Primary Oracle Documents

| File | Description |
|------|-------------|
| `/Users/sac/process-intelligence/sources/pm4py/CAPABILITY_ATLAS.md` | Complete enumeration of ~120 PM4Py capabilities in 10 categories (I/O, Discovery, Conformance, Performance, Filtering, Variants, Visualization, Organizational Mining, Prediction/ML, Object-Centric) with WASM-bridge status per entry |
| `/Users/sac/process-intelligence/sources/pm4py/ORACLE_MAP.md` | Cross-reference of every pm4wasm.d.ts WASM function against its PM4Py Python counterpart, input/output types, and required wasm4pm type surface across 6 function categories |
| `/Users/sac/process-intelligence/sources/pm4py/ALGORITHM_CROSSWALK.md` | Formal gap count for 37 algorithms: 2 COVERED, 16 PARTIAL, 19 MISSING across Process Discovery, Conformance, Variant Analysis, and Object-Centric categories |
| `/Users/sac/process-intelligence/sources/pm4py/NOT_TO_COPY.md` | Ten architectural anti-patterns from PM4Py that must never be replicated in wasm4pm, with named law rationale for each refusal |

## Matrix and Analysis Documents

| File | Description |
|------|-------------|
| `/Users/sac/process-intelligence/sources/pm4py/divergence-matrix.md` | v2.0 (2026-05-31) — must-match vs. safe-divergence classification for every PM4Py capability category, with Evidence binding types and operational checklist |
| `/Users/sac/process-intelligence/sources/pm4py/POWL_CHOICEGRAPH_ANALYSIS.md` | Analysis of Kourani-Park-van der Aalst arXiv:2505.07052 (2025), tracing DecisionGraph type gap to src/powl.rs with proposed Rust type sketches and ChoiceGraphPaper2025 witness enum |
| `/Users/sac/process-intelligence/sources/pm4py/conformance-authority-map.md` | v30.1.1 — formal mathematical definitions of all 7 conformance techniques (TBR, alignment, footprints, DECLARE, log skeleton, temporal profile, OCEL), failure modes, scalability profiles, and multi-metric validation workflow |
| `/Users/sac/process-intelligence/sources/pm4py/oracle-vulnerability.md` | Adversarial red-team assessment of PM4Py as comparative oracle: DataFrame mutability, absence of typestate enforcement, latency/memory overhead |
| `/Users/sac/process-intelligence/sources/pm4py/process-form-crosswalk.md` | Mapping of PM4Py structural representations (Petri net, DFG, OCEL) to process-intelligence foundry counter-forms |

## Receipt and Checkpoint Documents

| File | Description |
|------|-------------|
| `/Users/sac/process-intelligence/receipts/rec_ebitda_rework_001.json` | EBITDA rework reduction receipt: fitness 0.982, precision 0.945, throughput 4.12 days, BLAKE3 log/model hashes, ed25519 validator signature |
| `/Users/sac/process-intelligence/receipts/RECEIPT_REGISTRY.md` | Registry of 7 canonical research program receipts; PM4PY_ORACLE_RECEIPT entry certifies oracle corpus completeness |
| `/Users/sac/process-intelligence/checkpoints/ALIVE_GATE_ASSESSMENT.md` | ALIVE gate assessment: all 12/12 criteria MET; criterion 4 (sources/pm4py >= 4 files, actual 14) confirms corpus ALIVE status |
| `/Users/sac/process-intelligence/checkpoints/PROCESS_INTELLIGENCE_ALIVE_001.md` | Sealed ALIVE checkpoint with downstream authorisations |

## Additional Source Files Referenced (Not Fully Read)

| File | Description |
|------|-------------|
| `/Users/sac/process-intelligence/sources/pm4py/mining-authority-map.md` | Mining authority map for PM4Py discovery algorithms |
| `/Users/sac/process-intelligence/sources/pm4py/oracle-map.md` | Lower-case variant of oracle map |
| `/Users/sac/process-intelligence/sources/pm4py/algorithm-crosswalk.md` | Lower-case variant of algorithm crosswalk |
| `/Users/sac/process-intelligence/receipts/rec_residual_standard_005.json` | Residual standard receipt (ed25519 signed) |
| `/Users/sac/process-intelligence/receipts/rec_risk_compliance_004.json` | Risk compliance receipt (ed25519 signed) |
| `/Users/sac/process-intelligence/receipts/rec_risk_sla_003.json` | Risk SLA receipt (ed25519 signed) |
| `/Users/sac/process-intelligence/receipts/rec_wc_ar_002.json` | Working capital accounts receivable receipt (ed25519 signed) |
