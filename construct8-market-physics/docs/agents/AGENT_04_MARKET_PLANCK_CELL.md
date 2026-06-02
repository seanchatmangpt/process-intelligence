# Agent 04 — Market Planck Cell Modeler

## Crate

`crates/c8-market` — Market Planck Cell modeler and relational market state-change detection engine.

## Status

cargo check: PASS

## Design Decisions

### Price as Integer Ticks (i64)

`TickObservation.price_ticks` is `i64`, not `f64`. No floating-point on the hot path. Negative ticks
represent inverted or synthetic instruments.

### Side Enum

`Side::Bid = 0`, `Side::Ask = 1` — `#[repr(u8)]`, zero-cost layout.

### MarketRelationKind

Six relational categories, `#[repr(u8)]`:

| Variant | Value | Meaning |
|---|---|---|
| LiquidityTopology | 0 | Order book depth collapse |
| CapitalPressure | 1 | Buy/sell size imbalance |
| RelationBreak | 2 | Price gap breach |
| WavePhase | 3 | Directional reversal pattern |
| SettlementConstraint | 4 | Clearing/settlement lock |
| LatencyGeometry | 5 | Latency topology anomaly |

### ActuationClass

Four bounded classes, `#[repr(u8)]`: Observe, Alert, Actuate, Refuse.

### MarketPlanckCell

The smallest actionable relational state-change unit. Constructed via
`from_tick_relation(tick, relation_kind, pre_hash, causal_time)`. Never holds floats.

`to_construct8_delta()` emits exactly 3 triples:
1. `instrument --[relation]--> venue`
2. `causal_time_node --[relation]--> instrument`
3. `pre_state_hash_node --[relation]--> post_state_hint_node`

Always <= 8 triples — the CONSTRUCT8 lane bound is never approached.

### Detection Functions

| Function | Logic |
|---|---|
| `detect_relation_break_state` | abs(ticks[n] - ticks[n-1]) >= threshold |
| `detect_liquidity_topology_state` | total_bid_size + total_ask_size < collapse_threshold |
| `detect_capital_pressure_state` | buy_size * 100 / sell_size >= imbalance_ratio |
| `detect_wave_phase_state` | count windows(3) with local peak/trough reversals >= 2 |

## Tests

| Test | Assertion |
|---|---|
| `tick_alone_is_not_planck_cell` | cell.pre_state_hash != tick.price_ticks as u64 |
| `planck_cell_emits_construct8_delta_with_max_8_triples` | delta.len() <= 8 |
| `relation_break_detected_on_large_gap` | gap=15, threshold=10 -> Some |
| `relation_break_not_detected_on_small_gap` | gap=1, threshold=10 -> None |
| `wave_phase_state_is_graph_state_not_mysticism` | reversals >= 2 -> Some(WavePhase) |

## Dependencies

- `c8-core` (path) — InstrumentId, VenueId, NodeId, RelationId, Construct8Mask
- `c8-graph` (path) — Construct8Delta, Construct8DeltaBuilder, Construct8Triple
