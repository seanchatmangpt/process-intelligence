#!/usr/bin/env bash
# Check for live trading dependencies across all project crates
echo "=== NO-LIVE-TRADING CHECK ==="
TERMS="broker|exchange.api|alpaca|interactive.broker|binance|coinbase|kraken|fix.protocol|order.submit|wallet|private.key|custodian|websocket.*live|live.*feed"
STATUS=PASS
for d in /Users/sac/process-intelligence/construct8-market-physics /Users/sac/truex /Users/sac/pcp /Users/sac/knhk /Users/sac/ggen; do
  if [ -d "$d/src" ] || [ -d "$d/crates" ]; then
    hits=$(grep -r -i -E "$TERMS" "$d" --include="*.rs" --include="*.toml" 2>/dev/null | grep -v "^.*#\|//\|docs\|tests" | head -5)
    if [ -n "$hits" ]; then echo "SOURCE_RISK: $d"; echo "$hits"; STATUS=WARN;
    else echo "CLEAN: $d"; fi
  fi
done
echo "Result: $STATUS"
