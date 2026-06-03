//! # c8-market
//!
//! Market Planck Cell modeler and relational market state-change detection engine.
//!
//! Relational state changes (rather than raw tick messages) are the foundational unit of
//! observation. Raw tick frames are converted into `MarketPlanckCell` structures and
//! translated into CONSTRUCT8 graph deltas.
//!
//! Price is integer ticks (i64), NOT f64. No float on the hot path.

use c8_core::{Construct8Mask, InstrumentId, NodeId, RelationId, VenueId};
use c8_graph::{Construct8Delta, Construct8DeltaBuilder, Construct8Triple};

// ---------------------------------------------------------------------------
// Side
// ---------------------------------------------------------------------------

/// Order book side.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Side {
    Bid = 0,
    Ask = 1,
}

// ---------------------------------------------------------------------------
// MarketRelationKind
// ---------------------------------------------------------------------------

/// Relational categories that can be detected in the market graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MarketRelationKind {
    LiquidityTopology = 0,
    CapitalPressure = 1,
    RelationBreak = 2,
    WavePhase = 3,
    SettlementConstraint = 4,
    LatencyGeometry = 5,
}

// ---------------------------------------------------------------------------
// ActuationClass
// ---------------------------------------------------------------------------

/// Bounded actuation class for market cells.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ActuationClass {
    Observe = 0,
    Alert = 1,
    Actuate = 2,
    Refuse = 3,
}

// ---------------------------------------------------------------------------
// TickObservation
// ---------------------------------------------------------------------------

/// A raw tick observation. Price is integer ticks, NOT f64.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TickObservation {
    pub price_ticks: i64,
    pub size: u64,
    pub side: Side,
    pub instrument_id: InstrumentId,
    pub venue_id: VenueId,
    pub monotonic_ns: u64,

    // Compatibility fields
    pub price: u64,
    pub volume: u64,
    pub bid_price: u64,
    pub ask_price: u64,
    pub bid_volume: u64,
    pub ask_volume: u64,
    pub timestamp: u64,
}

impl TickObservation {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        instrument_id: u32,
        venue_id: u32,
        price: u64,
        volume: u64,
        bid_price: u64,
        ask_price: u64,
        bid_volume: u64,
        ask_volume: u64,
        timestamp: u64,
    ) -> Self {
        let side = if bid_volume >= ask_volume {
            Side::Bid
        } else {
            Side::Ask
        };
        TickObservation {
            price_ticks: price as i64,
            size: volume,
            side,
            instrument_id: InstrumentId(instrument_id),
            venue_id: VenueId(venue_id),
            monotonic_ns: timestamp,
            price,
            volume,
            bid_price,
            ask_price,
            bid_volume,
            ask_volume,
            timestamp,
        }
    }
}

// ---------------------------------------------------------------------------
// MarketPlanckCell
// ---------------------------------------------------------------------------

/// Smallest actionable relational state-change unit in the market graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MarketPlanckCell {
    pub instrument_id: InstrumentId,
    pub venue_id: VenueId,
    pub relation_kind: MarketRelationKind,
    pub causal_time: u64,
    pub monotonic_time: u64,
    pub pre_state_hash: u64,
    pub post_state_hint: u64,
    pub delta_mask: u8,
    pub confidence_bucket: u8,
    pub actuation_class: ActuationClass,
}

impl MarketPlanckCell {
    /// Construct a MarketPlanckCell from a tick observation and a relation kind.
    pub fn from_tick_relation(
        tick: &TickObservation,
        relation_kind: MarketRelationKind,
        pre_hash: u64,
        causal_time: u64,
    ) -> Self {
        // post_state_hint: mix pre_hash with tick content (no floats)
        let post_state_hint = pre_hash ^ (tick.price_ticks as u64) ^ tick.size ^ tick.monotonic_ns;

        MarketPlanckCell {
            instrument_id: tick.instrument_id,
            venue_id: tick.venue_id,
            relation_kind,
            causal_time,
            monotonic_time: tick.monotonic_ns,
            pre_state_hash: pre_hash,
            post_state_hint,
            delta_mask: Construct8Mask::EMPTY.set(0).0,
            confidence_bucket: 100,
            actuation_class: ActuationClass::Alert,
        }
    }

    /// Emit a Construct8Delta from this cell. Always <= 8 triples (exactly 3).
    pub fn to_construct8_delta(&self) -> Construct8Delta {
        let s = NodeId(self.instrument_id.0 as u64);
        let p = RelationId(self.relation_kind as u32);
        let o = NodeId(self.venue_id.0 as u64);

        // Triple 1: instrument --[relation]--> venue
        let t1 = Construct8Triple::new(s, p, o);
        // Triple 2: causal time node --[relation]--> instrument
        let t2 = Construct8Triple::new(
            NodeId(self.causal_time),
            RelationId(self.relation_kind as u32),
            s,
        );
        // Triple 3: pre_state_hash node --[relation]--> post_state_hint node
        let t3 = Construct8Triple::new(
            NodeId(self.pre_state_hash),
            RelationId(self.relation_kind as u32),
            NodeId(self.post_state_hint),
        );

        // Build with exactly 3 triples, always within the 8-triple bound.
        let builder = Construct8DeltaBuilder::new()
            .push(t1)
            .expect("first triple always fits")
            .push(t2)
            .expect("second triple always fits")
            .push(t3)
            .expect("third triple always fits");

        builder.build()
    }
}

// ---------------------------------------------------------------------------
// Detection functions
// ---------------------------------------------------------------------------

/// Detect a relational break: abs(ticks[n].price_ticks - ticks[n-1].price_ticks) >= threshold.
pub fn detect_relation_break_state(
    ticks: &[TickObservation],
    threshold: i64,
) -> Option<MarketPlanckCell> {
    if ticks.len() < 2 {
        return None;
    }
    for i in 1..ticks.len() {
        let gap = (ticks[i].price_ticks - ticks[i - 1].price_ticks).abs();
        if gap >= threshold {
            let tick = &ticks[i];
            let pre_hash = ticks[i - 1].price_ticks as u64 ^ ticks[i - 1].size;
            return Some(MarketPlanckCell::from_tick_relation(
                tick,
                MarketRelationKind::RelationBreak,
                pre_hash,
                tick.monotonic_ns,
            ));
        }
    }
    None
}

/// Detect liquidity topology collapse: total size (bids + asks) < collapse_threshold.
pub fn detect_liquidity_topology_state(
    bids: &[TickObservation],
    asks: &[TickObservation],
    collapse_threshold: u64,
) -> Option<MarketPlanckCell> {
    let total_bid: u64 = bids.iter().map(|t| t.size).sum();
    let total_ask: u64 = asks.iter().map(|t| t.size).sum();
    let total = total_bid + total_ask;

    if total < collapse_threshold {
        let anchor = bids.first().or_else(|| asks.first())?;
        let pre_hash = total_bid ^ total_ask;
        Some(MarketPlanckCell::from_tick_relation(
            anchor,
            MarketRelationKind::LiquidityTopology,
            pre_hash,
            anchor.monotonic_ns,
        ))
    } else {
        None
    }
}

/// Detect capital pressure imbalance: buy_size * 100 / sell_size >= imbalance_ratio.
pub fn detect_capital_pressure_state(
    ticks: &[TickObservation],
    imbalance_ratio: u64,
) -> Option<MarketPlanckCell> {
    let buy_size: u64 = ticks
        .iter()
        .filter(|t| t.side == Side::Bid)
        .map(|t| t.size)
        .sum();
    let sell_size: u64 = ticks
        .iter()
        .filter(|t| t.side == Side::Ask)
        .map(|t| t.size)
        .sum();

    if sell_size == 0 {
        return None;
    }

    if buy_size * 100 / sell_size >= imbalance_ratio {
        let anchor = ticks.first()?;
        let pre_hash = buy_size ^ sell_size;
        Some(MarketPlanckCell::from_tick_relation(
            anchor,
            MarketRelationKind::CapitalPressure,
            pre_hash,
            anchor.monotonic_ns,
        ))
    } else {
        None
    }
}

/// Detect wave phase: count direction reversals in windows of 3.
/// Returns a cell if there are >= 2 reversals.
pub fn detect_wave_phase_state(ticks: &[TickObservation]) -> Option<MarketPlanckCell> {
    if ticks.len() < 3 {
        return None;
    }

    let mut reversals: usize = 0;

    for window in ticks.windows(3) {
        let a = window[0].price_ticks;
        let b = window[1].price_ticks;
        let c = window[2].price_ticks;
        // A reversal: local peak (b > a and b > c) or local trough (b < a and b < c)
        let is_reversal = (b > a && b > c) || (b < a && b < c);
        if is_reversal {
            reversals += 1;
        }
    }

    if reversals >= 2 {
        let anchor = ticks.first()?;
        let pre_hash = reversals as u64 ^ anchor.price_ticks as u64;
        Some(MarketPlanckCell::from_tick_relation(
            anchor,
            MarketRelationKind::WavePhase,
            pre_hash,
            anchor.monotonic_ns,
        ))
    } else {
        None
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_tick(price_ticks: i64, size: u64, side: Side) -> TickObservation {
        let bid = if side == Side::Bid {
            price_ticks as u64
        } else {
            (price_ticks - 1) as u64
        };
        let ask = if side == Side::Ask {
            price_ticks as u64
        } else {
            (price_ticks + 1) as u64
        };
        TickObservation::new(
            1,  // instrument_id
            10, // venue_id
            price_ticks as u64,
            size,
            bid,
            ask,
            if side == Side::Bid { size } else { 0 },
            if side == Side::Ask { size } else { 0 },
            1_000_000,
        )
    }

    /// A tick alone is not a Planck cell: pre_state_hash != tick.price_ticks as u64.
    #[test]
    fn tick_alone_is_not_planck_cell() {
        let tick = make_tick(100, 50, Side::Bid);
        let pre_hash = 0u64; // initial pre-hash is not derived from price_ticks
        let cell = MarketPlanckCell::from_tick_relation(
            &tick,
            MarketRelationKind::RelationBreak,
            pre_hash,
            tick.monotonic_ns,
        );
        // The cell pre_state_hash is the caller-supplied pre_hash (0), NOT price_ticks (100)
        assert_ne!(cell.pre_state_hash, tick.price_ticks as u64);
    }

    /// A Planck cell emits a Construct8Delta with at most 8 triples.
    #[test]
    fn planck_cell_emits_construct8_delta_with_max_8_triples() {
        let tick = make_tick(200, 100, Side::Ask);
        let cell = MarketPlanckCell::from_tick_relation(
            &tick,
            MarketRelationKind::LiquidityTopology,
            42,
            tick.monotonic_ns,
        );
        let delta = cell.to_construct8_delta();
        assert!(
            delta.len() <= 8,
            "delta must have <= 8 triples, got {}",
            delta.len()
        );
    }

    /// A large gap (15 >= threshold 10) triggers relation break detection.
    #[test]
    fn relation_break_detected_on_large_gap() {
        let ticks = vec![
            make_tick(100, 50, Side::Bid),
            make_tick(115, 50, Side::Bid), // gap = 15
        ];
        let cell = detect_relation_break_state(&ticks, 10);
        assert!(
            cell.is_some(),
            "expected relation break cell for gap=15, threshold=10"
        );
    }

    /// A small gap (1 < threshold 10) does not trigger relation break detection.
    #[test]
    fn relation_break_not_detected_on_small_gap() {
        let ticks = vec![
            make_tick(100, 50, Side::Bid),
            make_tick(101, 50, Side::Bid), // gap = 1
        ];
        let cell = detect_relation_break_state(&ticks, 10);
        assert!(cell.is_none(), "expected no cell for gap=1, threshold=10");
    }

    /// Wave phase detection is graph-state (direction reversals), not mysticism.
    #[test]
    fn wave_phase_state_is_graph_state_not_mysticism() {
        // Sequence: 100, 110, 105, 115, 108 -- multiple local reversals
        let ticks = vec![
            make_tick(100, 10, Side::Bid),
            make_tick(110, 10, Side::Bid),
            make_tick(105, 10, Side::Bid),
            make_tick(115, 10, Side::Bid),
            make_tick(108, 10, Side::Bid),
        ];
        let cell = detect_wave_phase_state(&ticks);
        assert!(cell.is_some(), "expected wave phase cell -- reversals >= 2");
        // Verify the cell encodes WavePhase relation kind (graph state, not free text)
        assert_eq!(cell.unwrap().relation_kind, MarketRelationKind::WavePhase);
    }
}
