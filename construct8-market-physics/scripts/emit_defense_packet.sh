#!/bin/bash
set -e

cargo run --example adversary_gap_demo > DEFENSE_PACKET.md 2>&1
echo "
Automated Verification:
" >> DEFENSE_PACKET.md
cargo test --workspace >> DEFENSE_PACKET.md 2>&1
