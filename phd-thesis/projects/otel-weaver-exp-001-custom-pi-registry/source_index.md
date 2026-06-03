# Source Index: otel-weaver-exp-001-custom-pi-registry

All source files consulted during evidence extraction for this project chapter.

| File | Description |
|------|-------------|
| `/Users/sac/process-intelligence/otel-weaver/experiments/exp-001-custom-pi-weaver-registry/README.md` | Primary experiment documentation; specifies Weaver CLI commands and registry structure |
| `/Users/sac/process-intelligence/otel-weaver/experiments/exp-001-custom-pi-weaver-registry/semconv/process_pi.yaml` | Custom OTel Weaver registry defining 8 span attributes under `process.pi.activity`; `file_format: 1.2.0` |
| `/Users/sac/process-intelligence/otel-weaver/checkpoints/GGEN_OTEL_WEAVER_PI_ALIVE_001.md` | ALIVE checkpoint; status COMPLETE and ACTIVE; SHA-256 sealed by ggen manufacturing authority |
| `/Users/sac/process-intelligence/otel-weaver/checkpoints/GGEN_OTEL_WEAVER_PI_PARTIAL_001.md` | Design gate checkpoint; PASSED status; earliest gate in the sequence |
| `/Users/sac/process-intelligence/otel-weaver/checkpoints/GGEN_OTEL_WEAVER_PI_RUNTIME_001.md` | Runtime integration gate; PASSED; cites 62 passing tests across wasm4pm, wasm4pm-compat, blue_river_dam |
| `/Users/sac/process-intelligence/otel-weaver/doctrine/otel-weaver-is-feedstock.md` | Doctrine: OTel Weaver is feedstock-layer only; does not perform conformance checking |
| `/Users/sac/process-intelligence/otel-weaver/doctrine/telemetry-is-not-process-evidence.md` | Doctrine: telemetry spans are not process evidence without schema-validated structure |
| `/Users/sac/process-intelligence/otel-weaver/doctrine/registry-diff-is-not-process-drift.md` | Doctrine: schema registry changes must not be conflated with process conformance drift |
| `/Users/sac/process-intelligence/otel-weaver/mappings/otel-to-pi-witness-map.yaml` | Crosswalk mapping OTel span fields to process intelligence witness attributes |
| `/Users/sac/process-intelligence/otel-weaver/mappings/otel-signal-to-process-evidence-map.yaml` | Maps OTel signal types (spans, metrics, logs) to process-evidence categories |
| `/Users/sac/process-intelligence/otel-weaver/ggen/manifests/otel-weaver-source.manifest.yaml` | ggen manifest declaring BLAKE3 receipt chain with declared store at receipts/otel-weaver-source-receipt.json |
| `/Users/sac/process-intelligence/otel-weaver/ggen/templates/pi-weaver-registry.yaml.ggen` | ggen template from which process_pi.yaml was manufactured |
| `/Users/sac/process-intelligence/otel-weaver/experiments/exp-002-weaver-diff-to-pi-residual/src/bridge.rs` | Defines `BridgeRx` Rust struct; translates schema-version diffs without producing residual state |
| `/Users/sac/process-intelligence/otel-weaver/experiments/exp-003-live-check-to-refusal/src/validator.rs` | Defines `validate_and_project`, `Admission<T,W>`, `Refusal`, `RefusalCode` enum, `OtelSpanToOcelEventProjection` |
| `/Users/sac/process-intelligence/otel-weaver/README.md` | Top-level OTel Weaver integration suite overview; describes five-experiment structure |
| `/Users/sac/process-intelligence/sources/wasm4pm/tests/weaver_integration_tests.rs` | Integration test file; contains `test_weaver_integration_synthesis` (1 passed per RUNTIME checkpoint) |
