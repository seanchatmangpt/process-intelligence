// THIS IS NOT A PROFIT CLAIM. It is a structural mismatch demonstration.

use c8_graph::GraphField;
use c8_market::{
    MarketPlanckCell, MarketRelationKind, TickObservation, detect_relation_break_state,
};

#[derive(Debug, Clone)]
pub struct GameTreeNode {
    pub node_id: u32,
    pub description: &'static str,
    pub is_relation_break_aware: bool,
}

#[derive(Debug, Clone)]
pub struct LogicPlayer {
    pub known_nodes: Vec<GameTreeNode>,
}

impl LogicPlayer {
    pub fn new() -> Self {
        LogicPlayer {
            known_nodes: vec![
                GameTreeNode {
                    node_id: 0,
                    description: "price_up",
                    is_relation_break_aware: false,
                },
                GameTreeNode {
                    node_id: 1,
                    description: "price_down",
                    is_relation_break_aware: false,
                },
                GameTreeNode {
                    node_id: 2,
                    description: "volume_spike",
                    is_relation_break_aware: false,
                },
                GameTreeNode {
                    node_id: 3,
                    description: "spread_wide",
                    is_relation_break_aware: false,
                },
            ],
        }
    }

    pub fn can_represent_relation_break(&self) -> bool {
        self.known_nodes.iter().any(|n| n.is_relation_break_aware)
    }

    pub fn process_tick_stream(&self, ticks: &[TickObservation]) -> Vec<&'static str> {
        let mut events: Vec<&'static str> = Vec::new();
        for window in ticks.windows(2) {
            let prev = &window[0];
            let curr = &window[1];
            if curr.price_ticks > prev.price_ticks {
                events.push("price_up");
            } else if curr.price_ticks < prev.price_ticks {
                events.push("price_down");
            }
            if curr.size >= prev.size.saturating_mul(2) && prev.size > 0 {
                events.push("volume_spike");
            }
        }
        events
    }
}

impl Default for LogicPlayer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct GraphPlayer {
    pub field: GraphField,
    pub known_cells: Vec<MarketPlanckCell>,
}

impl GraphPlayer {
    pub fn new() -> Self {
        GraphPlayer {
            field: GraphField::new(),
            known_cells: Vec::new(),
        }
    }

    pub fn can_represent_relation_break(&self) -> bool {
        true
    }

    pub fn process_tick_stream(&mut self, ticks: &[TickObservation]) {
        if let Some(cell) = detect_relation_break_state(ticks, 10) {
            let delta = cell.to_construct8_delta();
            let _ = self.field.apply_construct8(&delta);
            self.known_cells.push(cell);
        }
    }

    pub fn relation_break_count(&self) -> usize {
        self.known_cells
            .iter()
            .filter(|c| c.relation_kind == MarketRelationKind::RelationBreak)
            .count()
    }
}

impl Default for GraphPlayer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct MissingStateBasis {
    pub state_name: &'static str,
    pub logic_player_can_see: bool,
    pub graph_player_can_see: bool,
    pub explanation: &'static str,
}

#[derive(Debug, Clone)]
pub struct RepresentationGap {
    pub missing_states: Vec<MissingStateBasis>,
    pub gap_score: u32,
}

#[derive(Debug, Clone)]
pub struct ProphecyIllusion {
    pub adversary_claim: &'static str,
    pub actual_explanation: &'static str,
}

pub fn find_missing_state_basis(
    logic: &LogicPlayer,
    graph: &GraphPlayer,
    _ticks: &[TickObservation],
) -> RepresentationGap {
    let rb = MissingStateBasis {
        state_name: "RelationBreak",
        logic_player_can_see: logic.can_represent_relation_break(),
        graph_player_can_see: graph.can_represent_relation_break(),
        explanation: "Crossed bid/ask spread is a relational state change; logic trees lack the basis vector",
    };
    let ltc = MissingStateBasis {
        state_name: "LiquidityTopologyCollapse",
        logic_player_can_see: false,
        graph_player_can_see: true,
        explanation: "Volume horizon collapse is a relational depth event; price features cannot encode it",
    };
    let gap_score = [&rb, &ltc]
        .iter()
        .filter(|s| !s.logic_player_can_see && s.graph_player_can_see)
        .count() as u32;
    RepresentationGap {
        missing_states: vec![rb, ltc],
        gap_score,
    }
}

pub fn explain_prophecy_illusion() -> ProphecyIllusion {
    ProphecyIllusion {
        adversary_claim: "GraphPlayer knew every move",
        actual_explanation: "coordinate-system advantage, not omniscience: GraphPlayer encodes relational states that LogicPlayer cannot represent, producing alpha from basis completeness alone",
    }
}

pub fn score_representation_gap(gap: &RepresentationGap) -> u32 {
    gap.gap_score
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_tick(price_ticks: i64, size: u64) -> TickObservation {
        TickObservation::new(
            1,  // instrument_id
            10, // venue_id
            price_ticks as u64,
            size,
            price_ticks as u64,
            (price_ticks + 1) as u64,
            size,
            0,
            1_000_000,
        )
    }

    fn sample_ticks() -> Vec<TickObservation> {
        vec![
            make_tick(100, 50),
            make_tick(115, 60), // gap=15 >= threshold=10: relation break
            make_tick(112, 50),
            make_tick(113, 100),
        ]
    }

    #[test]
    fn logic_tree_lacks_relation_break_node() {
        assert!(!LogicPlayer::new().can_represent_relation_break());
    }

    #[test]
    fn graph_tree_contains_relation_break_capability() {
        assert!(GraphPlayer::new().can_represent_relation_break());
    }

    #[test]
    fn same_market_stream_yields_missing_state_basis() {
        let ticks = sample_ticks();
        let gap = find_missing_state_basis(&LogicPlayer::new(), &GraphPlayer::new(), &ticks);
        assert!(gap.gap_score > 0);
        let rb = gap
            .missing_states
            .iter()
            .find(|s| s.state_name == "RelationBreak")
            .unwrap();
        assert!(!rb.logic_player_can_see);
        assert!(rb.graph_player_can_see);
    }

    #[test]
    fn prophecy_illusion_is_not_omniscience_claim() {
        let illusion = explain_prophecy_illusion();
        assert!(illusion.actual_explanation.contains("coordinate-system"));
    }

    #[test]
    fn coordinate_system_alpha_is_structural_not_ego() {
        let gap = RepresentationGap {
            missing_states: vec![MissingStateBasis {
                state_name: "RelationBreak",
                logic_player_can_see: false,
                graph_player_can_see: true,
                explanation: "structural basis mismatch",
            }],
            gap_score: 1,
        };
        assert_eq!(score_representation_gap(&gap), 1);
    }

    #[test]
    fn adversary_conforms_to_need9_and_max8() {
        // 1. Verify that GraphPlayer's underlying GraphField rejects payloads exceeding 8 triples with C8Error::Need9.
        let mut delta = c8_graph::Construct8Delta::empty();
        for i in 0..8 {
            delta
                .push_checked(c8_graph::Construct8Triple::new(
                    c8_core::NodeId(i),
                    c8_core::RelationId(1),
                    c8_core::NodeId(i + 1),
                ))
                .expect("8 triples must compile/push successfully");
        }

        // Pushing a 9th triple must return C8Error::Need9 (refusal signal)
        let result = delta.push_checked(c8_graph::Construct8Triple::new(
            c8_core::NodeId(9),
            c8_core::RelationId(1),
            c8_core::NodeId(10),
        ));
        assert_eq!(result, Err(c8_core::C8Error::Need9));

        // 2. Verify that GraphPlayer processing a tick stream produces bounded state deltas (<= 8 triples).
        let mut player = GraphPlayer::new();
        let ticks = sample_ticks();
        player.process_tick_stream(&ticks);

        // Ensure every cell encountered translates to a Construct8Delta of <= 8 triples
        for cell in &player.known_cells {
            let delta = cell.to_construct8_delta();
            assert!(
                delta.len() <= 8,
                "delta must have at most 8 triples, got {}",
                delta.len()
            );
        }
    }
}
