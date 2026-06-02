#!/usr/bin/env bash
# C8_MARKET_PHYSICS_ALIVE_002 — One-command defense packet regeneration
# Run from workspace root to regenerate all receipts and verify ALIVE status
set -e
WS="/Users/sac/process-intelligence/construct8-market-physics"
echo "=== C8 MARKET PHYSICS DEFENSE PACKET ==="
echo "Step 1: Format check"
cargo fmt --all --check
echo "Step 2: Clippy"
cargo clippy --workspace --all-targets
echo "Step 3: Tests"
cargo test --workspace
echo "Step 4: Examples"
cargo run --example market_planck_demo
cargo run --example event_horizon_demo
cargo run --example collider_demo
cargo run --example adversary_gap_demo
echo "Step 5: Benchmarks"
cargo bench -p c8-bench 2>&1 | tail -20 || echo "PARTIAL_BENCH"
echo "=== DEFENSE PACKET COMPLETE ==="
