// Ablation test: if LogicPlayer gains relation_break awareness, gap collapses.
//
// This is a READ-ONLY structural demonstration. It does not modify src/ code.
// It simulates the ablation by constructing a LogicPlayer with an injected
// relation_break_aware node, then verifies that the RepresentationGap for
// RelationBreak disappears — proving the gap is caused *by* the missing basis,
// not by any other factor.

use c8_adversary::{GameTreeNode, GraphPlayer, LogicPlayer, find_missing_state_basis};

fn make_tick(price_ticks: i64, size: u64) -> c8_market::TickObservation {
    c8_market::TickObservation::new(
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

/// Baseline: without ablation, the gap for RelationBreak exists.
/// LogicPlayer::new() has no relation_break_aware nodes.
#[test]
fn baseline_without_ablation_gap_exists() {
    let logic = LogicPlayer::new();
    let graph = GraphPlayer::new();
    let ticks = vec![make_tick(100, 10), make_tick(115, 10)];
    let gap = find_missing_state_basis(&logic, &graph, &ticks);

    // Without ablation: gap_score > 0 (at minimum RelationBreak is missing)
    assert!(
        gap.gap_score > 0,
        "Without ablation: gap must exist (gap_score={})",
        gap.gap_score
    );

    // RelationBreak specifically: LogicPlayer cannot see it
    let rb = gap
        .missing_states
        .iter()
        .find(|s| s.state_name == "RelationBreak")
        .expect("RelationBreak must be in missing_states");

    assert!(
        !rb.logic_player_can_see,
        "Without ablation: LogicPlayer must NOT see RelationBreak"
    );
    assert!(
        rb.graph_player_can_see,
        "GraphPlayer must always see RelationBreak"
    );
}

/// Ablation: after injecting a relation_break_aware node into LogicPlayer,
/// the gap for RelationBreak collapses — logic_player_can_see becomes true.
#[test]
fn ablation_logic_player_gains_awareness_gap_collapses() {
    // Simulate ablation: give LogicPlayer a relation_break_aware node
    let mut logic = LogicPlayer::new();
    logic.known_nodes.push(GameTreeNode {
        node_id: 99,
        description: "relation_break_ablated",
        is_relation_break_aware: true,
    });

    // Verify the ablation took effect
    assert!(
        logic.can_represent_relation_break(),
        "After ablation: LogicPlayer must report can_represent_relation_break() == true"
    );

    let graph = GraphPlayer::new();
    let ticks = vec![make_tick(100, 10), make_tick(115, 10)];
    let gap = find_missing_state_basis(&logic, &graph, &ticks);

    // After ablation: RelationBreak is no longer a missing state for LogicPlayer
    let rb = gap
        .missing_states
        .iter()
        .find(|s| s.state_name == "RelationBreak")
        .expect("RelationBreak must still appear in missing_states list");

    assert!(
        rb.logic_player_can_see,
        "After ablation: LogicPlayer must see RelationBreak (logic_player_can_see={})",
        rb.logic_player_can_see
    );

    // The gap score must be reduced compared to baseline (RelationBreak no longer penalized)
    // LiquidityTopologyCollapse remains hardcoded as logic_player_can_see=false,
    // so gap_score drops from 2 → 1 (not to 0), but the RelationBreak contribution is gone.
    assert!(
        rb.logic_player_can_see || !rb.graph_player_can_see,
        "After ablation: RelationBreak must NOT count as a missing-state gap"
    );
}

/// Gap score delta: confirms ablation reduces gap by exactly 1 (RelationBreak contribution).
#[test]
fn ablation_reduces_gap_score_by_one() {
    let ticks = vec![make_tick(100, 10), make_tick(115, 10)];

    // Baseline gap
    let baseline_gap = find_missing_state_basis(&LogicPlayer::new(), &GraphPlayer::new(), &ticks);
    let baseline_score = baseline_gap.gap_score;

    // Ablated gap
    let mut ablated_logic = LogicPlayer::new();
    ablated_logic.known_nodes.push(GameTreeNode {
        node_id: 99,
        description: "relation_break_ablated",
        is_relation_break_aware: true,
    });
    let ablated_gap = find_missing_state_basis(&ablated_logic, &GraphPlayer::new(), &ticks);
    let ablated_score = ablated_gap.gap_score;

    assert_eq!(
        baseline_score,
        ablated_score + 1,
        "Ablation must reduce gap_score by exactly 1 (baseline={}, ablated={})",
        baseline_score,
        ablated_score
    );
}
