# Source Index: sources-wasm4pm

All source files read during thesis chapter manufacture for this project.

| File | Description |
|---|---|
| `/Users/sac/process-intelligence/sources/wasm4pm/Cargo.toml` | Crate manifest: name wasm4pm, version 30.1.2, cdylib+rlib targets, sole dependency wasm4pm-compat (path) |
| `/Users/sac/process-intelligence/sources/wasm4pm/src/lib.rs` | Crate root declaring 17 public modules including conformance, mining, crypto, evidence, graduation, ltl, sandbox, ocel_v2, otel, lifecycle |
| `/Users/sac/process-intelligence/sources/wasm4pm/src/conformance.rs` | 1,229-line Conformance Authority: ConformanceVerdict enum, ConformanceVerdicts aggregate, TokenReplayEngine, AlignmentEngine (A* BinaryHeap), Between01/Metric const generics, RuntimeBetween01 |
| `/Users/sac/process-intelligence/sources/wasm4pm/src/mining/mod.rs` | 847-line Mining Authority (sealed receipt): ProcessModel union, AlphaWitness/InductiveWitness/HeuristicsWitness with Lattice impl, Admitted state enum, four Evidence-returning API functions |
| `/Users/sac/process-intelligence/sources/wasm4pm/src/crypto.rs` | Self-implemented cryptographic primitives: BLAKE3 Merkle-tree chaining, SHA-256, SHA-512, ChaCha20, Twisted-Edwards Curve25519, Ed25519 RFC-8032, verify_jcs_receipt_signature |
| `/Users/sac/process-intelligence/sources/wasm4pm/src/graduation.rs` | GraduateToWasm4pm trait implementation bridging wasm4pm-compat GraduationCandidate into execution engine; four inline tests |
| `/Users/sac/process-intelligence/sources/wasm4pm/src/evidence.rs` | Evidence<T,State,W> carrier type with Blake3Hash, IdentitySignature, epoch; Lattice infrastructure |
| `/Users/sac/process-intelligence/sources/wasm4pm/src/ltl.rs` | LTL/Declare constraint evaluation: DeclareRule with Precedence/Response variants over traces |
| `/Users/sac/process-intelligence/sources/wasm4pm/src/sandbox.rs` | GasMeter (10M cycle budget, checked_add), RecursionGuard (max depth 100), execute_oblivion_protocol (memory shredding) |
| `/Users/sac/process-intelligence/sources/wasm4pm/src/otel.rs` | OtelTrace/OtelSpan OpenTelemetry model augmented with blake3_receipt, witness_id, token_state lifecycle metadata |
| `/Users/sac/process-intelligence/sources/wasm4pm/src/ocel_v2.rs` | ZeroCopyOcelV2 zero-allocation OCEL 2.0 binary parser with magic/version/offset validation |
| `/Users/sac/process-intelligence/sources/wasm4pm/src/lifecycle/mod.rs` | ProcessIntelligence<S> 13-state MAPE-K lifecycle typestate machine with SoundnessProof, SimulationProof, ConformanceProof, RepairReceipt, DecommissioningReceipt |
| `/Users/sac/process-intelligence/sources/wasm4pm/EXECUTION_AUTHORITY_ATLAS.md` | Authoritative audit of ~/wasm4pm/ live codebase: crate inventory (12 crates), algorithm surface, GAP_001 critical finding, provenance surface, comparative positioning vs. PM4Py (20-25x speedup claim) |
| `/Users/sac/process-intelligence/sources/wasm4pm/GAP_ANALYSIS.md` | Ten structural gaps GAP_001-GAP_010 with severity classifications; documents compat consumption absence, string-typed errors, missing algorithms |
| `/Users/sac/process-intelligence/sources/wasm4pm/MINING_RENDER_RECEIPT.md` | Sealed manufacturing receipt (MINING_AUTHORITY_RENDER_001, 2026-06-01T00:00:00Z) for src/mining/mod.rs, SHA-256 08a067d1ee19ea67150c194e9a2db7d86dfd994223b922de7e8606f45fbdf8e5 |
| `/Users/sac/process-intelligence/sources/wasm4pm/research-verdict.md` | Doctoral-level execution authority classification (2026-05-31): four authorities specified, GO verdict issued, five critical success factors, deployment roadmap phases 1-5 |
| `/Users/sac/process-intelligence/sources/wasm4pm/mining-authority-ontology.md` | Evidence type registry: four evidence types all lattice-compliant, mining authority specification triples |
| `/Users/sac/process-intelligence/sources/wasm4pm/OBJECT_CENTRIC_RUNTIME.md` | Object-centric runtime design documentation |
| `/Users/sac/process-intelligence/sources/wasm4pm/tests/integration_tests.rs` | Integration test suite (part of 31-test, 10.60s passing run) |
| `/Users/sac/process-intelligence/sources/wasm4pm/tests/e2e_tests.rs` | End-to-end test suite (part of 31-test passing run) |
| `/Users/sac/process-intelligence/sources/wasm4pm/tests/weaver_integration_tests.rs` | Weaver integration test suite (part of 31-test passing run) |
| `/Users/sac/process-intelligence/phd-thesis/projects/sources-wasm4pm/project_manifest.yaml` | Project manifest: slug, path, detected languages/frameworks/research surfaces, thesis role, key files |
