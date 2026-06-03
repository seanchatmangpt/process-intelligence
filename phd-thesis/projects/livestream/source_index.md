# Source Index: livestream

All source files read during manufacture of the 8 TeX files for this project chapter.

| File | Description |
|------|-------------|
| `/Users/sac/process-intelligence/livestream/aalst_broadcaster.js` | Node.js module that transforms git log output into OCEL 2.0 event stream (JSONL to stdout) |
| `/Users/sac/process-intelligence/livestream/master_ocel_stream.js` | Node.js polling loop that emits new replay JSON files from /Users/sac/zoeapp/replays every 1 second |
| `/Users/sac/process-intelligence/livestream/replay_aggregator.js` | Node.js one-shot aggregator that consolidates OCEL 2.0 events from all replay JSON files into master_conversation.ocel |
| `/Users/sac/process-intelligence/livestream/agent_conversation_log.js` | Node.js parser for Gemini JSONL chatlogs; emits typed process events (Admissibility Guard, toolCall, Event); contains syntax error on line 18 |
| `/Users/sac/process-intelligence/livestream/conformance_audit.md` | Reports Fitness=0.9880, Precision=1.0000 against Blue River Dam thesis model; notes collision handling by aggregator |
| `/Users/sac/process-intelligence/AALST_LIVESTREAM_MANIFEST.md` | v30.1.1 manifest declaring AALST_LIVESTREAM_ACTIVE verdict; describes four livestream infrastructure components |
| `/Users/sac/process-intelligence/AALST_CERTIFIED_ALIVE.md` | SHA-256 attestation (ea15dda...) certifying the AGI conversation satisfies v30.1.1 real-time transparency standards |
| `/Users/sac/process-intelligence/experiments/visualizer/autonomic.js` | Pure-JS SHA-256 implementation, AutonomicController (MAPE-K tick), EWMACalculator (alpha=0.15, LCL=0.920), AuditLedger (SHA-256 chain) |
| `/Users/sac/process-intelligence/checkpoints/LIVESTREAM_STATUS_001.md` | Petri net token table showing [Adversarial Audit] and [Gating] states as pending |
| `/Users/sac/process-intelligence/checkpoints/PROCESS_INTELLIGENCE_ALIVE_001.md` | Phase 11 final checkpoint; 12/12 ALIVE gate criteria declared met; authorizes ggen and BLAKE3 receipt emission |
| `/Users/sac/process-intelligence/checkpoints/PROCESS_INTELLIGENCE_ADVERSARIAL_V30.1.1_OMEGA.md` | 20-agent adversarial swarm final declaration; foundry declared definitively ALIVE; V30.1.1 locked |
| `/Users/sac/process-intelligence/checkpoints/PETRI_NET_SNAPSHOT.md` | Base64-encoded Petri net snapshot; final sound marking with single token at [Decommissioning], no deadlocks |
| `/Users/sac/process-intelligence/receipts/RECEIPT_REGISTRY.md` | Registry of 7 research receipts (paper canon, pm4py oracle, wasm4pm gap, lifecycle, M&A, standards, adversarial); version 001, 2026-05-31 |
| `/Users/sac/process-intelligence/audits/stream_director_audit.md` | v30.1.2 PASS audit; 60.0 FPS, 6-15ms latency, 0% frame drop; DOM binding remediation declared complete |
| `/Users/sac/process-intelligence/audits/telemetry_auditor_audit.md` | v30.1.2 FAIL audit; DOM binding mismatch halts execution; XES 2/5; OCEL 2.0 0% compliance |
| `/Users/sac/process-intelligence/phd-thesis/projects/livestream/project_manifest.yaml` | Project manifest declaring slug, path, description, detected languages, and likely thesis role |
