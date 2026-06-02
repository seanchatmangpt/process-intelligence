#!/usr/bin/env bash
set -e
echo "=== CROSS-PROJECT VALIDATION ==="
bash /Users/sac/process-intelligence/cross-project-coordinate-alpha/scripts/census.sh
bash /Users/sac/process-intelligence/cross-project-coordinate-alpha/scripts/check_no_live_trading.sh
bash /Users/sac/process-intelligence/cross-project-coordinate-alpha/scripts/check_no_runtime_llm.sh
bash /Users/sac/process-intelligence/cross-project-coordinate-alpha/scripts/check_public_ip_boundary.sh
echo "=== VALIDATION COMPLETE ==="
