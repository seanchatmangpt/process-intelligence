//! # adversary_gap_demo
//!
//! Runs LogicPlayer and GraphPlayer on same stream, shows state basis visible
//! to GraphPlayer but missing from LogicPlayer, prints "representation gap",
//! not profit claim.

use c8_adversary::{
    explain_prophecy_illusion, find_missing_state_basis, score_representation_gap, GraphPlayer,
    LogicPlayer,
};
use c8_market::TickObservation;

fn main() {
    println!("=== CONSTRUCT8 Adversary Gap Demo ===");

    // 1. Set up logic and graph players
    let logic_player = LogicPlayer::new();
    let mut graph_player = GraphPlayer::new();

    println!("Initialized players:");
    println!(
        "  LogicPlayer (has {} logic nodes)",
        logic_player.known_nodes.len()
    );
    println!("  GraphPlayer (has clean graph field)");

    // 2. Prepare same synthetic market tick stream
    let ticks = vec![
        TickObservation::new(1, 10, 100, 50, 99, 101, 10, 10, 1_000_000),
        TickObservation::new(1, 10, 115, 60, 114, 116, 10, 10, 1_001_000), // relation break (gap = 15 >= 10)
        TickObservation::new(1, 10, 112, 50, 111, 113, 10, 10, 1_002_000),
        TickObservation::new(1, 10, 113, 100, 112, 114, 10, 10, 1_003_000), // volume spike
    ];

    println!("Running tick stream through LogicPlayer and GraphPlayer...");

    // 3. Players process stream
    let logic_events = logic_player.process_tick_stream(&ticks);
    graph_player.process_tick_stream(&ticks);

    println!("LogicPlayer observed events: {:?}", logic_events);
    println!(
        "GraphPlayer registered {} relation break cells.",
        graph_player.relation_break_count()
    );

    // 4. Compute representation gap
    let gap = find_missing_state_basis(&logic_player, &graph_player, &ticks);
    let gap_score = score_representation_gap(&gap);

    println!("\n--- REPRESENTATION GAP DETECTED ---");
    println!("Gap Score: {}", gap_score);
    for state in &gap.missing_states {
        println!("  State: {}", state.state_name);
        println!(
            "    LogicPlayer can represent: {}",
            state.logic_player_can_see
        );
        println!(
            "    GraphPlayer can represent: {}",
            state.graph_player_can_see
        );
        println!("    Explanation:               {}", state.explanation);
    }

    // 5. Explain prophecy illusion
    let illusion = explain_prophecy_illusion();
    println!("\n--- ADVERSARY OBSERVATION SUMMARY ---");
    println!("Adversary Claim:     \"{}\"", illusion.adversary_claim);
    println!("Actual Explanation:  \"{}\"", illusion.actual_explanation);

    println!("\nConclusion: The representation gap enables structural coordinate-system alpha, not predictive magic.");
}
