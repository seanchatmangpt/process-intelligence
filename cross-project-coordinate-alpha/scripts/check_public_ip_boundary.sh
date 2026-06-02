#!/usr/bin/env bash
# Check public docs for private terms
echo "=== PUBLIC IP BOUNDARY CHECK ==="
PRIVATE_TERMS="actuation.path|capital.deployment|operational.trading|order.submit|private.key|wallet.address"
for doc in /Users/sac/process-intelligence/cross-project-coordinate-alpha/*.md /Users/sac/process-intelligence/construct8-market-physics/docs/*.md; do
  if [ -f "$doc" ]; then
    hits=$(grep -i -E "$PRIVATE_TERMS" "$doc" 2>/dev/null | head -3)
    if [ -n "$hits" ]; then echo "REVIEW: $doc"; echo "$hits";
    else echo "CLEAN: $doc"; fi
  fi
done
