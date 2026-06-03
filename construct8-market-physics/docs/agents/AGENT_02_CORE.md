# AGENT 2 — Rust Workspace and Type-Law Founder Report

## Mission
Create the Rust workspace structure and implement the core type law (`c8-core`) required for CONSTRUCT8.

## Files Inspected
- `crates/c8-core/Cargo.toml`
- `crates/c8-core/src/lib.rs`

## Files Created/Updated
- `crates/c8-core/Cargo.toml`
- `crates/c8-core/src/lib.rs`
- `docs/agents/AGENT_02_CORE.md`

## Implementation Decisions
- Declared the core typed identifiers (`C8Id`, `NodeId`, `RelationId`, `VenueId`, `InstrumentId`, `ActorClassId`, `GraphSlot`).
- Enforced the CONSTRUCT8 size invariant via `Construct8Len`, which validates that length does not exceed 8.
- Set up bitmask support (`Construct8Mask`) using non-branching bitwise operators.
- Created error types (`C8Error`) and finite hot-path outcomes (`HotPathVerdict`).
- Added module documentation explaining why dynamic logic-chaos is disallowed on the hot path.

## Tests Added
- `test_construct8_len_bounds` (accepts 0..=8, rejects 9)
- `test_need9_refusal` (Need9 returned when length boundary violated)
- `test_hot_path_verdict_finite` (ensures HotPathVerdict has no open string path)

## Benchmarks Added
- None in this stage.

## Risks
- Bitwise masks must be handled correctly in downstream graph and index engines.

## Verdict
**ALIVE** — Workspace created, core type law implemented and unit tested.
