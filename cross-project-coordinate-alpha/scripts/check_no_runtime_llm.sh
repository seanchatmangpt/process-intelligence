#!/usr/bin/env bash
# Check for runtime LLM dependencies in src crates
echo "=== NO-RUNTIME-LLM CHECK ==="
TERMS="openai|anthropic|claude.*api|llm|chatcompletion|messages.create|api.key"
STATUS=PASS
for d in /Users/sac/process-intelligence/construct8-market-physics/crates /Users/sac/truex /Users/sac/ggen /Users/sac/wasm4pm /Users/sac/wasm4pm-compat; do
  if [ -d "$d" ]; then
    hits=$(grep -r -i -E "$TERMS" "$d" --include="*.rs" 2>/dev/null | grep -v "//\|#\[doc\]" | head -5)
    if [ -n "$hits" ]; then echo "SOURCE_RISK: $d"; echo "$hits"; STATUS=WARN;
    else echo "CLEAN: $d"; fi
  fi
done
echo "Result: $STATUS"
