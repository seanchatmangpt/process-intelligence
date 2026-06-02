//! # collider_demo
//!
//! Collides liquidity topology and capital pressure hypotheses, and emits hidden market body candidate.

use c8_instruments::{ColliderHypothesis, MarketCollider};
use c8_market::MarketRelationKind;

fn main() {
    println!("=== CONSTRUCT8 Market Collider Demo ===");

    let collider = MarketCollider;

    // 1. Set up hypotheses
    // Hypothesis 1: Liquidity topology curvature shows potential hidden accumulation
    let h1 = ColliderHypothesis {
        hypothesis_id: 101,
        relation_kind: MarketRelationKind::LiquidityTopology,
        strength: 600,
    };
    // Hypothesis 2: Capital pressure shows buying volume imbalance
    let h2 = ColliderHypothesis {
        hypothesis_id: 102,
        relation_kind: MarketRelationKind::CapitalPressure,
        strength: 550,
    };

    println!("Collider Hypotheses:");
    println!(
        "  Hypothesis 1: ID = {}, Kind = {:?}, Strength = {}",
        h1.hypothesis_id, h1.relation_kind, h1.strength
    );
    println!(
        "  Hypothesis 2: ID = {}, Kind = {:?}, Strength = {}",
        h2.hypothesis_id, h2.relation_kind, h2.strength
    );

    // 2. Perform collision (total strength = 600 + 550 = 1150 > 1000 threshold)
    println!("Colliding hypotheses...");
    let result = collider.collide_hypotheses(&[h1, h2]);

    // 3. Emit hidden market body candidate
    if let Some(body) = result.hidden_body {
        println!("Successfully detected Hidden Market Body candidate:");
        println!("  Body ID:          {}", body.body_id);
        println!("  Implied Relation: {:?}", body.implied_relation);
        println!("  Confidence:       {}%", body.confidence);

        // Print emitted Construct8 delta
        println!("Emitted Construct8Delta of length {}:", result.delta.len());
        for (i, triple) in result
            .delta
            .as_fixed_slots()
            .iter()
            .take(result.delta.len())
            .enumerate()
        {
            println!("  Slot {}: {:?}", i, triple);
        }
    } else {
        panic!("Failed to detect hidden market body -- collision strength insufficient!");
    }
}
