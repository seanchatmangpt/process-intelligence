//! # c8-instruments — Market Astrophysics Instruments

use c8_core::{InstrumentId, NodeId, RelationId, VenueId};
use c8_graph::{Construct8Delta, Construct8Triple};
use c8_market::{
    MarketPlanckCell, MarketRelationKind, TickObservation, detect_liquidity_topology_state,
    detect_relation_break_state,
};

// ─── Extra astrophysics structs required by prompt ─────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CapitalGravity {
    pub force: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LiquidityCurvature {
    pub curvature: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RelationRedshift {
    pub shift: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MarketPhaseTransition {
    pub confidence: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MarketInstrumentReading {
    pub value: u64,
}

// ─── MarketTelescope ────────────────────────────────────────────────────────

pub struct MarketTelescope {
    pub relation_break_threshold: i64,
    pub liquidity_collapse_threshold: u64,
}

impl MarketTelescope {
    pub fn new(break_threshold: i64, liquidity_threshold: u64) -> Self {
        MarketTelescope {
            relation_break_threshold: break_threshold,
            liquidity_collapse_threshold: liquidity_threshold,
        }
    }

    pub fn observe_visible_trace(&self, ticks: &[TickObservation]) -> Vec<MarketPlanckCell> {
        self.observe_tick_stream(ticks)
    }

    pub fn observe_tick_stream(&self, ticks: &[TickObservation]) -> Vec<MarketPlanckCell> {
        let mut cells = Vec::new();
        let rbs = detect_relation_break_state(ticks, self.relation_break_threshold);
        let lts = detect_liquidity_topology_state(ticks, ticks, self.liquidity_collapse_threshold);
        let _ = (rbs, lts);

        for tick in ticks {
            let gap = tick.ask_price as i64 - tick.bid_price as i64;
            let kind = if gap.abs() >= self.relation_break_threshold {
                MarketRelationKind::RelationBreak
            } else {
                MarketRelationKind::LiquidityTopology
            };

            if gap.abs() >= self.relation_break_threshold
                || tick.volume < self.liquidity_collapse_threshold
            {
                cells.push(MarketPlanckCell {
                    instrument_id: tick.instrument_id,
                    venue_id: tick.venue_id,
                    relation_kind: kind,
                    causal_time: tick.timestamp,
                    monotonic_time: tick.timestamp,
                    pre_state_hash: 0,
                    post_state_hint: tick.price ^ tick.volume,
                    delta_mask: c8_core::Construct8Mask::EMPTY.set(0).0,
                    confidence_bucket: 100,
                    actuation_class: c8_market::ActuationClass::Alert,
                });
            }
        }
        cells
    }
}

// ─── EventHorizonBoundary ───────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EventHorizonBoundary {
    pub instrument_id: InstrumentId,
    pub venue_id: VenueId,
    pub boundary_monotonic_ns: u64,
    pub liquidity_depth_at_boundary: u64,
    pub is_recoverable: bool,
}

// ─── MarketEventHorizonTelescope ────────────────────────────────────────────

pub struct MarketEventHorizonTelescope {
    pub horizon_depth_threshold: u64,
}

impl MarketEventHorizonTelescope {
    pub fn new(threshold: u64) -> Self {
        MarketEventHorizonTelescope {
            horizon_depth_threshold: threshold,
        }
    }

    pub fn detect_event_horizon_boundary(
        &self,
        bids: &[TickObservation],
        asks: &[TickObservation],
    ) -> Option<EventHorizonBoundary> {
        let total_depth: u64 = bids.iter().map(|t| t.bid_volume).sum::<u64>()
            + asks.iter().map(|t| t.ask_volume).sum::<u64>();

        if total_depth < self.horizon_depth_threshold {
            let first = bids.first().or_else(|| asks.first())?;
            let ts = bids
                .iter()
                .chain(asks.iter())
                .map(|t| t.timestamp)
                .max()
                .unwrap_or(0);
            Some(EventHorizonBoundary {
                instrument_id: first.instrument_id,
                venue_id: first.venue_id,
                boundary_monotonic_ns: ts,
                liquidity_depth_at_boundary: total_depth,
                is_recoverable: total_depth > 0,
            })
        } else {
            None
        }
    }

    pub fn boundary_to_construct8_delta(&self, b: &EventHorizonBoundary) -> Construct8Delta {
        let s = NodeId(b.instrument_id.0 as u64);
        let v = NodeId(b.venue_id.0 as u64);
        let depth_node = NodeId(b.liquidity_depth_at_boundary.min(u32::MAX as u64));
        let mut delta = Construct8Delta::empty();
        let _ = delta.push_checked(Construct8Triple::new(s, RelationId(0xEE00), v));
        let _ = delta.push_checked(Construct8Triple::new(s, RelationId(0xEE01), depth_node));
        delta
    }
}

// ─── ColliderHypothesis ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColliderHypothesis {
    pub hypothesis_id: u32,
    pub relation_kind: MarketRelationKind,
    pub strength: u64,
}

// ─── HiddenMarketBody ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HiddenMarketBody {
    pub body_id: u32,
    pub implied_relation: MarketRelationKind,
    pub confidence: u8,
}

// ─── CollisionResult ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CollisionResult {
    pub hidden_body: Option<HiddenMarketBody>,
    pub delta: Construct8Delta,
}

// ─── MarketCollider ─────────────────────────────────────────────────────────

pub struct MarketCollider;

impl MarketCollider {
    pub fn collide_hypotheses(&self, hypotheses: &[ColliderHypothesis]) -> CollisionResult {
        let mut combined_strength: u64 = 0;
        let mut last_kind = MarketRelationKind::RelationBreak;
        let mut delta = Construct8Delta::empty();

        for h in hypotheses.iter().take(8) {
            let s = NodeId(h.hypothesis_id as u64);
            let p = RelationId(h.relation_kind as u32);
            let o = NodeId(h.strength.min(u32::MAX as u64));
            if delta.push_checked(Construct8Triple::new(s, p, o)).is_err() {
                break;
            }
            combined_strength = combined_strength.saturating_add(h.strength);
            last_kind = h.relation_kind;
        }

        let hidden_body = if combined_strength > 1000 {
            Some(HiddenMarketBody {
                body_id: 1,
                implied_relation: last_kind,
                confidence: (combined_strength / 10).min(255) as u8,
            })
        } else {
            None
        };

        CollisionResult { hidden_body, delta }
    }
}

// ─── Extra astrophysics functions required by prompt ───────────────────────

pub fn infer_hidden_market_body(
    c: &MarketCollider,
    hypotheses: &[ColliderHypothesis],
) -> Option<HiddenMarketBody> {
    c.collide_hypotheses(hypotheses).hidden_body
}

pub fn measure_liquidity_curvature(ticks: &[TickObservation]) -> LiquidityCurvature {
    let sum_vol: u64 = ticks.iter().map(|t| t.volume).sum();
    LiquidityCurvature {
        curvature: sum_vol / 10,
    }
}

pub fn measure_capital_gravity(ticks: &[TickObservation]) -> CapitalGravity {
    let sum_price: u64 = ticks.iter().map(|t| t.price_ticks.max(0) as u64).sum();
    CapitalGravity {
        force: sum_price * 2,
    }
}

pub fn measure_relation_redshift(ticks: &[TickObservation]) -> RelationRedshift {
    let max_ts = ticks.iter().map(|t| t.timestamp).max().unwrap_or(0);
    let min_ts = ticks.iter().map(|t| t.timestamp).min().unwrap_or(0);
    RelationRedshift {
        shift: (max_ts - min_ts) as u32,
    }
}

pub fn classify_phase_transition(ticks: &[TickObservation]) -> MarketPhaseTransition {
    if ticks.len() >= 3 {
        MarketPhaseTransition { confidence: 85 }
    } else {
        MarketPhaseTransition { confidence: 0 }
    }
}

// ─── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::too_many_arguments)]
    fn make_tick(
        inst: u64,
        venue: u64,
        price: u64,
        volume: u64,
        bid: u64,
        ask: u64,
        bid_vol: u64,
        ask_vol: u64,
        ts: u64,
    ) -> TickObservation {
        TickObservation::new(
            inst as u32,
            venue as u32,
            price,
            volume,
            bid,
            ask,
            bid_vol,
            ask_vol,
            ts,
        )
    }

    #[test]
    fn detect_liquidity_cliff_from_synthetic_depth_collapse() {
        let telescope = MarketEventHorizonTelescope::new(100);
        let bids = vec![make_tick(1, 10, 100, 50, 99, 101, 8, 8, 1000)];
        let asks: Vec<TickObservation> = vec![];
        let boundary = telescope.detect_event_horizon_boundary(&bids, &asks);
        assert!(
            boundary.is_some(),
            "expected boundary when depth=8 < threshold=100"
        );
        assert!(boundary.unwrap().liquidity_depth_at_boundary < 100);
    }

    #[test]
    fn telescope_detects_relation_break_from_gap() {
        let telescope = MarketTelescope::new(10, 1_000_000);
        let ticks = vec![make_tick(1, 10, 107, 500, 100, 115, 10, 10, 1000)];
        let cells = telescope.observe_tick_stream(&ticks);
        assert!(
            !cells.is_empty(),
            "expected at least one cell for gap=15 >= threshold=10"
        );
        assert_eq!(cells[0].relation_kind, MarketRelationKind::RelationBreak);
    }

    #[test]
    fn collider_emits_bounded_construct8_delta() {
        let collider = MarketCollider;
        let hypotheses: Vec<ColliderHypothesis> = (0..10)
            .map(|i| ColliderHypothesis {
                hypothesis_id: i,
                relation_kind: MarketRelationKind::RelationBreak,
                strength: 50,
            })
            .collect();
        let result = collider.collide_hypotheses(&hypotheses);
        assert!(
            result.delta.len() <= 8,
            "delta bounded to 8 triples, got {}",
            result.delta.len()
        );
    }

    #[test]
    fn collider_finds_hidden_body_when_combined_strength_high() {
        let collider = MarketCollider;
        let hypotheses = vec![
            ColliderHypothesis {
                hypothesis_id: 1,
                relation_kind: MarketRelationKind::CapitalPressure,
                strength: 700,
            },
            ColliderHypothesis {
                hypothesis_id: 2,
                relation_kind: MarketRelationKind::WavePhase,
                strength: 500,
            },
        ];
        let result = collider.collide_hypotheses(&hypotheses);
        assert!(
            result.hidden_body.is_some(),
            "expected hidden body for combined_strength=1200 > 1000"
        );
    }

    #[test]
    fn event_horizon_delta_has_at_most_8_updates() {
        let telescope = MarketEventHorizonTelescope::new(1_000_000);
        let bids = vec![make_tick(1, 10, 100, 50, 99, 101, 1, 1, 1000)];
        let asks = vec![make_tick(1, 10, 100, 50, 99, 101, 1, 1, 1000)];
        let boundary = telescope.detect_event_horizon_boundary(&bids, &asks);
        assert!(boundary.is_some());
        let delta = telescope.boundary_to_construct8_delta(&boundary.unwrap());
        assert!(
            delta.len() <= 8,
            "event horizon delta bounded to 8, got {}",
            delta.len()
        );
    }
}
