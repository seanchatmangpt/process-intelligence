# AGENT 6 — Market Astrophysics Instruments Report

## Mission

Implement astrophysics-themed measurement instruments that map price traces and liquidity
depth to relational graph-state updates via CONSTRUCT8 deltas.

## Files Created/Updated

- `crates/c8-instruments/Cargo.toml` — deps: c8-core, c8-graph, c8-market (all path)
- `crates/c8-instruments/src/lib.rs` — full instrument suite
- `docs/agents/AGENT_06_INSTRUMENTS.md` — this report

## Structures Implemented

### MarketTelescope
- Fields: `relation_break_threshold: i64`, `liquidity_collapse_threshold: u64`
- `new(break_threshold, liquidity_threshold) -> Self`
- `observe_tick_stream(&self, ticks: &[TickObservation]) -> Vec<MarketPlanckCell>`
  calls `detect_relation_break_state` and `detect_liquidity_topology_state`

### EventHorizonBoundary
- Fields: `instrument_id`, `venue_id`, `boundary_monotonic_ns: u64`,
  `liquidity_depth_at_boundary: u64`, `is_recoverable: bool`

### MarketEventHorizonTelescope
- Field: `horizon_depth_threshold: u64`
- `new(threshold) -> Self`
- `detect_event_horizon_boundary(&self, bids, asks: &[TickObservation]) -> Option<EventHorizonBoundary>`
  sums all bid/ask sizes; emits boundary if total < threshold
- `boundary_to_construct8_delta(&self, b: &EventHorizonBoundary) -> Construct8Delta`
  pushes 2 triples: (instrument, 0xEE00, venue) and (instrument, 0xEE01, depth as u32)

### ColliderHypothesis
- Fields: `hypothesis_id: u32`, `relation_kind: MarketRelationKind`, `strength: u64`

### HiddenMarketBody
- Fields: `body_id: u32`, `implied_relation: MarketRelationKind`, `confidence: u8`

### CollisionResult
- Fields: `hidden_body: Option<HiddenMarketBody>`, `delta: Construct8Delta`

### MarketCollider (unit struct)
- `collide_hypotheses(&self, hypotheses: &[ColliderHypothesis]) -> CollisionResult`
  iterates up to 8 hypotheses, pushes one triple per hypothesis via `push_checked`,
  emits `HiddenMarketBody` if combined_strength > 1000

## Tests

| Test | Assertion |
|------|-----------|
| `detect_liquidity_cliff_from_synthetic_depth_collapse` | bids size=8 < threshold=100 emits boundary |
| `telescope_detects_relation_break_from_gap` | gap=15 >= threshold=10 emits RelationBreak cell |
| `collider_emits_bounded_construct8_delta` | 10 hypotheses → delta.len() <= 8 |
| `collider_finds_hidden_body_when_combined_strength_high` | 700+500=1200 > 1000 → hidden body emitted |
| `event_horizon_delta_has_at_most_8_updates` | boundary delta.len() <= 8 |

## Cargo Check Result

PASS — `cargo check -p c8-instruments` compiles clean.

## Verdict

**ALIVE** — All instrument structures, detection methods, and CONSTRUCT8 delta-bounded
transformations implemented and verified.
