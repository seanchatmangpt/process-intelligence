# Synthetic Fixture Ledger

## Purpose

All market data in this codebase is synthetic. No real market data was used.
These fixtures exist to demonstrate structural properties, not to optimize trading strategies.

Every tick sequence, depth value, and hypothesis strength in this codebase was constructed
to exercise a specific structural condition in the CONSTRUCT8 state-representation engine.
None of these fixtures represent observations of, or advice about, any real financial instrument.

---

## Scenario Inventory

### Scenario 1: 15-tick gap triggers RelationBreak (c8-market unit test)

- Source: `crates/c8-market/src/lib.rs` — test `relation_break_detected_on_large_gap`
- Input: Two ticks — price_ticks=100, size=50, Side::Bid; then price_ticks=115, size=50, Side::Bid
- Expected output: `detect_relation_break_state` returns `Some(MarketPlanckCell { relation_kind: RelationBreak, ... })`
- Structural property tested: RelationBreak — the absolute price-tick gap between consecutive observations exceeds the configured threshold
- Threshold values: gap_threshold=10; gap=15 satisfies gap >= threshold
- Is this a trading signal? NO — it is a structural state representation

---

### Scenario 2: 1-tick gap does not trigger RelationBreak (c8-market unit test)

- Source: `crates/c8-market/src/lib.rs` — test `relation_break_not_detected_on_small_gap`
- Input: Two ticks — price_ticks=100, size=50, Side::Bid; then price_ticks=101, size=50, Side::Bid
- Expected output: `detect_relation_break_state` returns `None`
- Structural property tested: Negative boundary — gap=1 is strictly below threshold=10; no state is emitted
- Threshold values: gap_threshold=10; gap=1 fails gap >= threshold
- Is this a trading signal? NO — it is a structural state representation

---

### Scenario 3: 5-tick wave with 2+ reversals triggers WavePhase (c8-market unit test)

- Source: `crates/c8-market/src/lib.rs` — test `wave_phase_state_is_graph_state_not_mysticism`
- Input: Five ticks at price_ticks=[100, 110, 105, 115, 108], each size=10, Side::Bid
- Expected output: `detect_wave_phase_state` returns `Some(MarketPlanckCell { relation_kind: WavePhase, ... })`
- Structural property tested: WavePhase — direction-reversal count in 3-tick windows reaches >= 2; the cell encodes a graph-state node, not a chart pattern label
- Threshold values: reversal_count >= 2; windows containing local peak (b>a && b>c) or trough (b<a && b<c) are counted
- Is this a trading signal? NO — it is a structural state representation

---

### Scenario 4: Single tick constructs a Planck cell with pre_hash not equal to price_ticks (c8-market unit test)

- Source: `crates/c8-market/src/lib.rs` — test `tick_alone_is_not_planck_cell`
- Input: Single tick — price_ticks=100, size=50, Side::Bid; caller supplies pre_hash=0
- Expected output: `cell.pre_state_hash != tick.price_ticks as u64` (i.e., 0 != 100)
- Structural property tested: The pre_state_hash of a MarketPlanckCell is the caller-supplied transition hash, not the raw price value; a tick alone does not constitute a causal transition
- Threshold values: N/A
- Is this a trading signal? NO — it is a structural state representation

---

### Scenario 5: Planck cell emits Construct8Delta with at most 8 triples (c8-market unit test)

- Source: `crates/c8-market/src/lib.rs` — test `planck_cell_emits_construct8_delta_with_max_8_triples`
- Input: Single tick — price_ticks=200, size=100, Side::Ask; relation_kind=LiquidityTopology; pre_hash=42
- Expected output: `delta.len() <= 8`
- Structural property tested: CONSTRUCT8 8-lane bound — a single MarketPlanckCell emits exactly 3 triples, always within the fixed [T;8] array bound
- Threshold values: max_triples=8
- Is this a trading signal? NO — it is a structural state representation

---

### Scenario 6: Depth collapse detects EventHorizonBoundary (c8-instruments unit test)

- Source: `crates/c8-instruments/src/lib.rs` — test `detect_liquidity_cliff_from_synthetic_depth_collapse`
- Input: Single bid tick — bid_volume=8, ask_volume=8; empty asks slice; horizon_depth_threshold=100
- Expected output: `boundary.is_some()` and `boundary.liquidity_depth_at_boundary < 100`
- Structural property tested: LiquidityTopology collapse — total book depth (8) is below horizon threshold (100); the EventHorizonTelescope emits an EventHorizonBoundary node
- Threshold values: horizon_depth_threshold=100; total_depth=8+0=8 < 100
- Is this a trading signal? NO — it is a structural state representation

---

### Scenario 7: Spread gap >= threshold emits RelationBreak cell from MarketTelescope (c8-instruments unit test)

- Source: `crates/c8-instruments/src/lib.rs` — test `telescope_detects_relation_break_from_gap`
- Input: Single tick — price=107, volume=500, bid=100, ask=115; break_threshold=10, liquidity_threshold=1_000_000
- Expected output: At least one MarketPlanckCell with relation_kind=RelationBreak
- Structural property tested: MarketTelescope classifies a tick as RelationBreak when |ask_price - bid_price| >= relation_break_threshold; spread=15 >= threshold=10
- Threshold values: break_threshold=10; spread=15
- Is this a trading signal? NO — it is a structural state representation

---

### Scenario 8: 10 weak hypotheses do not produce hidden body (c8-instruments unit test)

- Source: `crates/c8-instruments/src/lib.rs` — test `collider_emits_bounded_construct8_delta`
- Input: 10 ColliderHypotheses each with strength=50, relation_kind=RelationBreak
- Expected output: `result.delta.len() <= 8`; `result.hidden_body` may be None (combined_strength=500 <= 1000)
- Structural property tested: CONSTRUCT8 bound enforcement — MarketCollider processes at most 8 hypotheses (lanes), emitting at most 8 triples; combined_strength threshold is not reached with 8 x 50 = 400
- Threshold values: max_lanes=8; combined_strength_threshold=1000; observed combined_strength=400
- Is this a trading signal? NO — it is a structural state representation

---

### Scenario 9: Two strong hypotheses produce HiddenMarketBody (c8-instruments unit test)

- Source: `crates/c8-instruments/src/lib.rs` — test `collider_finds_hidden_body_when_combined_strength_high`
- Input: Hypothesis 1 — kind=CapitalPressure, strength=700; Hypothesis 2 — kind=WavePhase, strength=500
- Expected output: `result.hidden_body.is_some()` (combined_strength=1200 > 1000)
- Structural property tested: MarketCollider hidden-body detection — when combined hypothesis strength exceeds 1000, the collider emits a HiddenMarketBody candidate node encoding an implied relational structure not visible in either hypothesis alone
- Threshold values: combined_strength_threshold=1000; observed combined_strength=1200
- Is this a trading signal? NO — it is a structural state representation

---

### Scenario 10: EventHorizon delta is bounded to 8 triples (c8-instruments unit test)

- Source: `crates/c8-instruments/src/lib.rs` — test `event_horizon_delta_has_at_most_8_updates`
- Input: Single bid tick bid_volume=1, ask_volume=1; horizon_depth_threshold=1_000_000 (forces boundary detection)
- Expected output: `boundary.is_some()` and `delta.len() <= 8`
- Structural property tested: CONSTRUCT8 8-lane bound on EventHorizonTelescope output — boundary_to_construct8_delta emits exactly 2 triples encoding instrument, venue, and depth node
- Threshold values: horizon_depth_threshold=1_000_000; total_depth=2 < 1_000_000
- Is this a trading signal? NO — it is a structural state representation

---

### Scenario 11: LogicPlayer cannot represent RelationBreak (c8-adversary unit test)

- Source: `crates/c8-adversary/src/lib.rs` — test `logic_tree_lacks_relation_break_node`
- Input: LogicPlayer initialized with its default 4 known nodes (price_up, price_down, volume_spike, spread_wide); none have is_relation_break_aware=true
- Expected output: `logic.can_represent_relation_break() == false`
- Structural property tested: Representation gap — the logic game tree has no basis vector for RelationBreak; the property is structural, not runtime
- Threshold values: N/A
- Is this a trading signal? NO — it is a structural state representation

---

### Scenario 12: GraphPlayer can represent RelationBreak (c8-adversary unit test)

- Source: `crates/c8-adversary/src/lib.rs` — test `graph_tree_contains_relation_break_capability`
- Input: GraphPlayer initialized with a fresh GraphField
- Expected output: `graph.can_represent_relation_break() == true`
- Structural property tested: Graph-state player's representation basis includes RelationBreak unconditionally by construction
- Threshold values: N/A
- Is this a trading signal? NO — it is a structural state representation

---

### Scenario 13: Same stream yields missing state basis (c8-adversary unit test)

- Source: `crates/c8-adversary/src/lib.rs` — test `same_market_stream_yields_missing_state_basis`
- Input: 4-tick stream — prices=[100, 115, 112, 113], sizes=[50, 60, 50, 100] (gap=15 at tick 2 triggers RelationBreak)
- Expected output: `gap.gap_score > 0`; RelationBreak entry has logic_player_can_see=false and graph_player_can_see=true
- Structural property tested: Representation gap scoring — on identical input streams, the GraphPlayer encodes a state the LogicPlayer cannot name; gap_score counts structurally missing basis vectors
- Threshold values: gap_threshold=10 (internal to GraphPlayer.process_tick_stream)
- Is this a trading signal? NO — it is a structural state representation

---

### Scenario 14: Coordinate-system alpha is structural, not ego (c8-adversary unit test)

- Source: `crates/c8-adversary/src/lib.rs` — test `coordinate_system_alpha_is_structural_not_ego`
- Input: Manually constructed RepresentationGap with gap_score=1 and one MissingStateBasis (RelationBreak; logic=false, graph=true)
- Expected output: `score_representation_gap(&gap) == 1`
- Structural property tested: The score function is a pure count of states visible to graph but not to logic; it produces a numeric structural observation, not a performance claim
- Threshold values: N/A
- Is this a trading signal? NO — it is a structural state representation

---

### Scenario 15: Prophecy illusion is not omniscience (c8-adversary unit test)

- Source: `crates/c8-adversary/src/lib.rs` — test `prophecy_illusion_is_not_omniscience_claim`
- Input: explain_prophecy_illusion() called with no arguments
- Expected output: actual_explanation contains the substring "coordinate-system"
- Structural property tested: The system explicitly disavows omniscience claims; any advantage is attributed to coordinate-system basis completeness, not prediction
- Threshold values: N/A
- Is this a trading signal? NO — it is a structural state representation

---

### Scenario 16: market_planck_demo — 2-tick gap demo (example)

- Source: `examples/market_planck_demo.rs`
- Input: Two ticks — (instrument=1, venue=10, price=100, volume=50, bid=99, ask=101, bid_vol=10, ask_vol=10, ts=1_000_000); then (price=115, bid=114, ask=116, ts=1_001_000); gap=15 >= threshold=10
- Expected output: MarketPlanckCell detected; Construct8Delta of length 3 applied to GraphField; state hash changes; C8Receipt emitted
- Structural property tested: Full path from synthetic tick stream through RelationBreak detection to graph state transition to receipt
- Threshold values: gap_threshold=10; gap=15
- Is this a trading signal? NO — it is a structural state representation

---

### Scenario 17: adversary_gap_demo — 4-tick adversary comparison (example)

- Source: `examples/adversary_gap_demo.rs`
- Input: 4-tick stream — (price=100, vol=50, ts=1_000_000); (price=115, vol=60, ts=1_001_000, gap=15); (price=112, vol=50, ts=1_002_000); (price=113, vol=100, ts=1_003_000, volume spike 100 >= 60*2 is false but near boundary)
- Expected output: LogicPlayer emits ["price_up", "price_down", "price_up"]; GraphPlayer registers 1 relation break cell; representation gap score > 0
- Structural property tested: Side-by-side representational capacity comparison between a logic game tree and a graph-state player on identical input
- Threshold values: gap_threshold=10 (GraphPlayer internal)
- Is this a trading signal? NO — it is a structural state representation

---

### Scenario 18: event_horizon_demo — normal vs. collapsed liquidity (example)

- Source: `examples/event_horizon_demo.rs`
- Input (normal): bid tick (volume=60, bid_vol=60); ask tick (volume=60, ask_vol=60); total_depth=120 >= threshold=100
- Input (collapsed): bid tick (volume=4, bid_vol=4); ask tick (volume=4, ask_vol=4); total_depth=8 < threshold=100
- Expected output: normal_boundary=None; collapsed boundary=Some(EventHorizonBoundary); Construct8Delta applied to GraphField; state hash changes
- Structural property tested: EventHorizonTelescope boundary detection across normal and collapsed liquidity depth states
- Threshold values: horizon_depth_threshold=100; normal_depth=120; collapsed_depth=8
- Is this a trading signal? NO — it is a structural state representation

---

### Scenario 19: collider_demo — two hypotheses collide to detect hidden structure (example)

- Source: `examples/collider_demo.rs`
- Input: Hypothesis 1 — id=101, kind=LiquidityTopology, strength=600; Hypothesis 2 — id=102, kind=CapitalPressure, strength=550
- Expected output: combined_strength=1150 > 1000; HiddenMarketBody emitted with implied_relation=CapitalPressure; Construct8Delta of length 2 printed
- Structural property tested: MarketCollider hidden-body detection — two hypotheses with sufficient combined strength produce a new graph node encoding an implied relation not present in either input
- Threshold values: combined_strength_threshold=1000; observed=1150
- Is this a trading signal? NO — it is a structural state representation

---

## Summary

Total synthetic scenarios documented: **19**

All values (prices, volumes, depths, hypothesis strengths) are chosen to exercise specific
structural thresholds. They do not represent any real instrument, market session, or
observed data. No fixtures in this codebase have been calibrated against live data,
backtested against historical data, or validated for profitability in any market.
