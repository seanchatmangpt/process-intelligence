# Source Index: otel-weaver-exp-003-live-check-to-refusal

All source files read during thesis writing for this project, with one-line descriptions.

| File | Description |
|------|-------------|
| `/Users/sac/process-intelligence/otel-weaver/experiments/exp-003-live-check-to-refusal/src/validator.rs` | Complete Rust implementation of the boundary gate: all struct/enum definitions, `validate_and_project`, `build_refusal`, and two inline unit tests |
| `/Users/sac/process-intelligence/otel-weaver/experiments/exp-003-live-check-to-refusal/Cargo.toml` | Crate manifest declaring `serde`, `serde_json`, `blake3 1.3`, and the `wasm4pm-compat` path dependency |
| `/Users/sac/process-intelligence/otel-weaver/experiments/exp-003-live-check-to-refusal/README.md` | Experiment README documenting the formal gate function signature, the `Refusal` domain model JSON structure, the operational pipeline, and the `refusals.json` persistence specification |
| `/Users/sac/process-intelligence/otel-weaver/mappings/live-check-finding-to-refusal-map.yaml` | Authoritative YAML mapping six named laws (EmptyTrace, MissingCaseId, InvalidTimestampOrder, DanglingEventObjectLink, ProjectionNameRequired, SchemaVersionMismatch) to refusal verdicts and remediation guidance |
| `/Users/sac/process-intelligence/otel-weaver/mappings/otel-to-pi-witness-map.yaml` | Witness mapping rules translating OTel resource attributes to `pi.feedstock.witness.id` values, with a conformance isolation rule |
| `/Users/sac/process-intelligence/otel-weaver/doctrine/weaver-finding-is-not-receipt.md` | Doctrine establishing the nominal category boundary between Weaver structural findings and process receipts, including the five-row comparison table |
| `/Users/sac/process-intelligence/otel-weaver/doctrine/otel-weaver-is-feedstock.md` | Doctrine formalizing the Feedstock Theorem and the three reasons OTel Weaver is feedstock (passive observation, lossy projection, no execution authority) |
| `/Users/sac/process-intelligence/otel-weaver/checkpoints/GGEN_OTEL_WEAVER_PI_ALIVE_001.md` | Checkpoint certifying all five OTel Weaver experiments created and verified; SHA-256 seal `e6bc59b8...` |
| `/Users/sac/process-intelligence/otel-weaver/checkpoints/GGEN_OTEL_WEAVER_PI_RUNTIME_001.md` | Checkpoint certifying runtime integration phase with test output (`test_weaver_integration_synthesis ... ok`, 62 total tests passing); SHA-256 seal `86ef45b3...` |
| `/Users/sac/process-intelligence/sources/wasm4pm/tests/weaver_integration_tests.rs` | Integration test exercising all five refusal/admission cases with genuine `wasm4pm::crypto::Blake3` digests, including `BridgeRx` schema version translation |
