#!/bin/bash

set -euo pipefail

AUDIT_NAME="audit-no-tool-smuggling"
COMPAT_SRC="/Users/sac/process-intelligence/sources/wasm4pm-compat/compat/src"
OUTPUT="/Users/sac/process-intelligence/audits/${AUDIT_NAME}.txt"

# Forbidden tool functions - these are graduation signals
FORBIDDEN_TOOLS=(
  "simulate_replay"
  "compute_alignment"
  "discover_model"
  "execute_ocpq"
  "run_conformance"
  "mint_receipt"
  "benchmark_gate_run"
)

# Graduation reasons (from forbidden-tool-ledger.yaml)
declare -A GRADUATION_REASONS=(
  ["simulate_replay"]="NeedsReplay"
  ["compute_alignment"]="NeedsConformanceExecution"
  ["discover_model"]="NeedsDiscovery"
  ["execute_ocpq"]="NeedsObjectCentricQueryExecution"
  ["run_conformance"]="NeedsConformanceExecution"
  ["mint_receipt"]="NeedsReceipts"
  ["benchmark_gate_run"]="NeedsBenchmarkGate"
)

echo "=== AUDIT 2: NO TOOLS IN COMPAT ===" > "$OUTPUT"
echo "Timestamp: $(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$OUTPUT"
echo "Target: $COMPAT_SRC" >> "$OUTPUT"
echo "Principle: Graduation signals must not be smuggled into compat" >> "$OUTPUT"
echo "" >> "$OUTPUT"

VIOLATIONS=0
PASS=true

echo "Forbidden Tools & Graduation Mapping:" >> "$OUTPUT"
echo "" >> "$OUTPUT"

for tool in "${FORBIDDEN_TOOLS[@]}"; do
  matches=$(grep -r "$tool" "$COMPAT_SRC" 2>/dev/null || true)
  reason="${GRADUATION_REASONS[$tool]}"
  
  if [ -n "$matches" ]; then
    echo "VIOLATION: Found tool '$tool'" >> "$OUTPUT"
    echo "  Graduation Reason: $reason" >> "$OUTPUT"
    echo "  Occurrences:" >> "$OUTPUT"
    echo "$matches" | sed 's/^/    /' >> "$OUTPUT"
    echo "" >> "$OUTPUT"
    VIOLATIONS=$((VIOLATIONS + 1))
    PASS=false
  else
    echo "[OK] $tool (GraduationReason: $reason)" >> "$OUTPUT"
  fi
done

echo "" >> "$OUTPUT"

if [ "$PASS" = true ]; then
  echo "RESULT: PASS" >> "$OUTPUT"
  echo "No forbidden tool functions detected." >> "$OUTPUT"
else
  echo "RESULT: FAIL" >> "$OUTPUT"
  echo "Found $VIOLATIONS tool violation(s) that must graduate to wasm4pm." >> "$OUTPUT"
fi

echo "" >> "$OUTPUT"
echo "Audit complete: $(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$OUTPUT"

cat "$OUTPUT"
