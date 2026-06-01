# Source Index: otel-weaver-exp-004-registry-to-witness

All source files read during thesis chapter manufacture for this project.

## Primary Experiment Files

| File | Description |
|------|-------------|
| `/Users/sac/process-intelligence/otel-weaver/experiments/exp-004-registry-to-wasm4pm-compat-witness/README.md` | Full architectural pipeline documentation, ASCII pipeline diagram, generator source listing, and verification instructions for EXP-004 |
| `/Users/sac/process-intelligence/otel-weaver/experiments/exp-004-registry-to-wasm4pm-compat-witness/codegen/generator.rs` | Executable Rust generator: parses resolved schema JSON, filters on process.pi.activity group, materializes ActivityWitness and Lattice implementations to generated_witnesses.rs |
| `/Users/sac/process-intelligence/otel-weaver/experiments/exp-004-registry-to-wasm4pm-compat-witness/src/generated_witnesses.rs` | Materialized Rust output artifact: Lattice trait, ActivityWitness struct, PartialOrd impl, and Lattice impl; stamped with generation timestamp 2026-06-01T10:10:51-07:00 |

## Checkpoints

| File | Description |
|------|-------------|
| `/Users/sac/process-intelligence/otel-weaver/checkpoints/GGEN_OTEL_WEAVER_PI_ALIVE_001.md` | Parent suite checkpoint declaring COMPLETE & ACTIVE status for the five-experiment otel-weaver integration suite; sealed SHA-256 e6bc59b8d210e309228d4bf09c0fa4db55fe310efc35ad1a89b3f021e85a67f1 |
| `/Users/sac/process-intelligence/otel-weaver/checkpoints/GGEN_OTEL_WEAVER_PI_RUNTIME_001.md` | Runtime integration checkpoint recording 62 tests passing and test_weaver_integration_synthesis passing; sealed SHA-256 86ef45b3c63ae38007701169bcf0abbda2694a43160f3f8d3695c6255b02cb44 |
| `/Users/sac/process-intelligence/otel-weaver/checkpoints/GGEN_OTEL_WEAVER_PI_PARTIAL_001.md` | Design-gate checkpoint recording initial scaffold and categorical boundary invariant definitions; sealed SHA-256 d4b844c48811199b21d2746af484456d9b2d8643c154988cfcf35fe310efc35a |

## Doctrine and Mappings

| File | Description |
|------|-------------|
| `/Users/sac/process-intelligence/otel-weaver/doctrine/otel-weaver-is-feedstock.md` | Feedstock Theorem doctrine: formal statement that telemetry schema validation does not equal process compliance; defines the Ingestion Covenant (Admit, Loss Reports, Witness Seal) |
| `/Users/sac/process-intelligence/otel-weaver/mappings/otel-to-pi-witness-map.yaml` | Operational mapping rules from OTel resource attributes to process-intelligence feedstock keys; three witness mapping rules and three witness registry entries; conformance isolation rule |

## Registry and Test Files

| File | Description |
|------|-------------|
| `/Users/sac/process-intelligence/otel-weaver/experiments/exp-001-custom-pi-weaver-registry/semconv/process_pi.yaml` | Source OTel Weaver convention registry defining the process.pi.activity group with eight attributes including process.pi.witness.hash; OTel file format 1.2.0, schema URL 1.25.0 |
| `/Users/sac/process-intelligence/sources/wasm4pm/tests/weaver_integration_tests.rs` | Integration test exercising full feedstock-to-court pipeline: BridgeRx schema translation, validate_and_project boundary, five admission and refusal cases, BLAKE3 digest verification |

## Project Manifest

| File | Description |
|------|-------------|
| `/Users/sac/process-intelligence/phd-thesis/projects/otel-weaver-exp-004-registry-to-witness/project_manifest.yaml` | Thesis project manifest: slug, absolute path, detected languages (Rust), detected frameworks (otel-weaver, wasm4pm-compat, ggen), research surfaces, thesis role, receipt_present: false |
