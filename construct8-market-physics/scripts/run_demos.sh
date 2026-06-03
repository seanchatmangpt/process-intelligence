#!/usr/bin/env bash
set -e
cd /Users/sac/process-intelligence/construct8-market-physics
cargo run --example market_planck_demo
cargo run --example event_horizon_demo
cargo run --example collider_demo
cargo run --example adversary_gap_demo
