#!/usr/bin/env bash
# Regenerate all cross-project receipts
cd /Users/sac/process-intelligence/construct8-market-physics && cargo test --workspace 2>&1 | tail -5
bash /Users/sac/process-intelligence/cross-project-coordinate-alpha/scripts/validate_cross_project.sh 2>&1
echo "receipts emitted to: /Users/sac/process-intelligence/cross-project-coordinate-alpha/receipts/"
