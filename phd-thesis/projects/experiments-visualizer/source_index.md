# Source Index: experiments-visualizer

All source files read and referenced in the thesis chapter for this project.

| File | Description |
|---|---|
| `/Users/sac/process-intelligence/experiments/visualizer/README.md` | Project overview and architecture summary for the zero-dependency browser dashboard |
| `/Users/sac/process-intelligence/experiments/visualizer/LIVESTREAM_STANDARDS.md` | v30.1.1 conformance specification: zero mocks/stubs, 5-agent AGI swarm architecture, SHA-256 hash formula |
| `/Users/sac/process-intelligence/experiments/visualizer/package.json` | NPM manifest confirming TypeScript as the sole devDependency |
| `/Users/sac/process-intelligence/experiments/visualizer/tsconfig.json` | TypeScript compiler configuration for visualizer-validation.ts |
| `/Users/sac/process-intelligence/experiments/visualizer/bindings.d.ts` | WASM Boundary Law declarations: EvidenceTs, EvidenceState lifecycle, WitnessKey, ReceiptShapeTs, GraduationCandidateTs, LossPolicyTs, brand tags |
| `/Users/sac/process-intelligence/experiments/visualizer/visualizer-validation.ts` | Compile-time structural validation exercising all 10 type projections with BLAKE3 hash and fitness=0.982 |
| `/Users/sac/process-intelligence/experiments/visualizer/app.js` | Top-level application coordinator wiring subsystem initialization and tick cycle |
| `/Users/sac/process-intelligence/experiments/visualizer/alignment.js` | Alignment computation layer implementing fitness formula 1 - cost/(log_length + minModelCost) |
| `/Users/sac/process-intelligence/experiments/visualizer/astar.js` | Priority-queue A* alignment solver with synchronous/model-only/log-only move types and admissible heuristic |
| `/Users/sac/process-intelligence/experiments/visualizer/autonomic.js` | MAPE-K AutonomicController: tick(metrics, net), rate limiting, S-component hot-swap, SHA-256 ledger events |
| `/Users/sac/process-intelligence/experiments/visualizer/blockchain.js` | CryptographicAuditChain: genesis block, sequential SHA-256 chaining, tamperBlock(), verifyChain() |
| `/Users/sac/process-intelligence/experiments/visualizer/declare.js` | DECLARE LTL template engine: 10 constraint templates, 3 domain rules for supply-chain process |
| `/Users/sac/process-intelligence/experiments/visualizer/drift.js` | Drift visualization and coordination layer complementing drift-detector.js |
| `/Users/sac/process-intelligence/experiments/visualizer/drift-detector.js` | EWMA drift detector: Jaccard DFG-profile distance, lambda=0.20, L=3.0, UCL/LCL control limits, calibrate() API |
| `/Users/sac/process-intelligence/experiments/visualizer/ledger.js` | AuditLedger: higher-level ledger API over CryptographicAuditChain |
| `/Users/sac/process-intelligence/experiments/visualizer/petrinet.js` | PetriNetModel: places, transitions, arcs, markings, fire(), getEnabledTransitions() |
| `/Users/sac/process-intelligence/experiments/visualizer/token-game.js` | TokenGameSimulator: Canvas 2D multi-case token flight animation with t4 Approve-Bypass route |
| `/Users/sac/process-intelligence/experiments/visualizer/dashboard.js` | Rendering coordinator reading metrics from subsystems and audit ledger to populate dashboard panels |
| `/Users/sac/process-intelligence/experiments/visualizer/index.html` | Single-page application entry point loading all 13 JS modules |
| `/Users/sac/process-intelligence/checkpoints/PROCESS_INTELLIGENCE_ALIVE_001.md` | GRADUATION VERIFIED verdict for the broader process intelligence program covering token game fitness formula |
| `/Users/sac/process-intelligence/checkpoints/LIVESTREAM_STATUS_001.md` | Livestream demonstration standards checkpoint for the visualizer dashboard |
| `/Users/sac/process-intelligence/experiments/checkpoint__experiments_complete.md` | Confirmation that all 16 experiment fixtures were verified with no deferred TODOs |
| `/Users/sac/process-intelligence/experiments/replay_receipt_sample.md` | Canonical replay receipt fixture: fitness=0.982, BLAKE3 process hash, supply-chain WF-net |
| `/Users/sac/process-intelligence/receipts/rec_ebitda_rework_001.json` | EBITDA rework process receipt in the broader domain receipt chain |
| `/Users/sac/process-intelligence/receipts/rec_risk_sla_003.json` | Risk SLA process receipt confirming ReceiptShapeTs application to financial domains |
| `/Users/sac/process-intelligence/receipts/rec_risk_compliance_004.json` | Risk compliance process receipt |
| `/Users/sac/process-intelligence/receipts/rec_residual_standard_005.json` | Residual standard process receipt |
| `/Users/sac/process-intelligence/receipts/rec_wc_ar_002.json` | Working capital / accounts receivable process receipt |
| `/Users/sac/process-intelligence/.ggen/receipts/latest.json` | Most recent ggen manufacturing run receipt for the process intelligence program |
