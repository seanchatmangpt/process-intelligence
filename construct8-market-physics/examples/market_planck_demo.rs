//! # market_planck_demo
//!
//! Creates synthetic ticks, turns relation changes into MarketPlanckCells,
//! emits Construct8Delta, applies delta to GraphField, prints state hash and receipt.

use c8_graph::GraphField;
use c8_market::{detect_relation_break_state, TickObservation};
use c8_receipts::C8Receipt;

fn main() {
    println!("=== CONSTRUCT8 Market Planck Cell Demo ===");

    // 1. Create synthetic ticks
    let ticks = vec![
        TickObservation::new(1, 10, 100, 50, 99, 101, 10, 10, 1_000_000),
        TickObservation::new(1, 10, 115, 50, 114, 116, 10, 10, 1_001_000), // relation break (gap = 15 >= 10)
    ];
    println!("Created {} synthetic ticks.", ticks.len());

    // 2. Turn relation changes into MarketPlanckCells
    let threshold = 10;
    println!(
        "Scanning for relation break with threshold = {}...",
        threshold
    );
    if let Some(cell) = detect_relation_break_state(&ticks, threshold) {
        println!("Successfully detected MarketPlanckCell: {:?}", cell);

        // 3. Emit Construct8Delta
        let delta = cell.to_construct8_delta();
        println!("Emitted Construct8Delta of length {}:", delta.len());
        for (i, triple) in delta.as_fixed_slots().iter().take(delta.len()).enumerate() {
            println!("  Slot {}: {:?}", i, triple);
        }

        // 4. Apply delta to GraphField
        let mut field = GraphField::new();
        let initial_hash = field.state_hash();
        println!("Initial GraphField state hash: {:#X}", initial_hash);

        let apply_result = field.apply_construct8(&delta);
        println!("Apply result: {:?}", apply_result);

        let final_hash = field.state_hash();
        println!("Final GraphField state hash:   {:#X}", final_hash);
        assert_ne!(
            initial_hash, final_hash,
            "State hash must change after apply"
        );

        // 5. Print receipt
        let receipt = C8Receipt::new(initial_hash, &delta, final_hash, cell.causal_time);
        println!("C8Receipt generated successfully:");
        println!("  Pre State Hash:  {:#X}", receipt.pre_state_hash);
        println!("  Post State Hash: {:#X}", receipt.post_state_hash);
        println!("  Causal Time:     {}", receipt.causal_time);
        println!("  C8Receipt Hash:  {:?}", receipt.receipt_hash);
    } else {
        panic!("Failed to detect relation break from synthetic ticks!");
    }
}
