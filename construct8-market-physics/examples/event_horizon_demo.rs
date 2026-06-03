//! # event_horizon_demo
//!
//! Creates synthetic liquidity/depth collapse, detects market event horizon boundary,
//! and emits graph-state updates.

use c8_graph::GraphField;
use c8_instruments::MarketEventHorizonTelescope;
use c8_market::TickObservation;

fn main() {
    println!("=== CONSTRUCT8 Market Event Horizon Demo ===");

    // 1. Create synthetic ticks representing normal and collapsing liquidity
    // Threshold is 100 total depth.
    let threshold = 100;
    let telescope = MarketEventHorizonTelescope::new(threshold);

    println!(
        "Event Horizon Telescope created with threshold = {}.",
        threshold
    );

    // Normal state: total bid/ask volume is 120 >= 100
    let normal_bids = vec![TickObservation::new(
        1, 10, 100, 60, 99, 101, 60, 0, 1_000_000,
    )];
    let normal_asks = vec![TickObservation::new(
        1, 10, 100, 60, 99, 101, 0, 60, 1_000_000,
    )];

    println!("Checking normal state (bids volume = 60, asks volume = 60)...");
    let normal_boundary = telescope.detect_event_horizon_boundary(&normal_bids, &normal_asks);
    assert!(
        normal_boundary.is_none(),
        "Normal liquidity should not cross horizon"
    );
    println!("  OK: No boundary detected.");

    // Collapsed state: total bid/ask volume is 8 < 100
    let collapsed_bids = vec![TickObservation::new(
        1, 10, 100, 4, 99, 101, 4, 0, 1_002_000,
    )];
    let collapsed_asks = vec![TickObservation::new(
        1, 10, 100, 4, 99, 101, 0, 4, 1_002_000,
    )];

    println!("Checking collapsed state (bids volume = 4, asks volume = 4)...");
    if let Some(boundary) =
        telescope.detect_event_horizon_boundary(&collapsed_bids, &collapsed_asks)
    {
        println!(
            "Successfully detected Event Horizon Boundary: {:?}",
            boundary
        );

        // 2. Emit graph-state updates (Construct8Delta)
        let delta = telescope.boundary_to_construct8_delta(&boundary);
        println!("Emitted Construct8Delta of length {}:", delta.len());
        for (i, triple) in delta.as_fixed_slots().iter().take(delta.len()).enumerate() {
            println!("  Slot {}: {:?}", i, triple);
        }

        // 3. Apply updates to GraphField
        let mut field = GraphField::new();
        let initial_hash = field.state_hash();
        println!("Initial GraphField state hash: {:#X}", initial_hash);

        let _ = field.apply_construct8(&delta);
        let final_hash = field.state_hash();
        println!("Final GraphField state hash:   {:#X}", final_hash);
        assert_ne!(
            initial_hash, final_hash,
            "State hash must change after event horizon boundary is applied"
        );
    } else {
        panic!("Failed to detect event horizon boundary on collapsed liquidity!");
    }
}
