#!/bin/bash

set -euo pipefail

AUDIT_NAME="audit-no-dto-flattening"
COMPAT_SRC="/Users/sac/process-intelligence/sources/wasm4pm-compat/compat/src"
OUTPUT="/Users/sac/process-intelligence/audits/${AUDIT_NAME}.txt"

# Forbidden patterns
FORBIDDEN_PATTERNS=(
  ".*Dto" 
  "payload_json"
  "_json"
  "state_tag as String"
)

ALLOWED_CARRIERS=(
  "EvidenceProjection"
  "WitnessKey"
  "EvidenceState"
  "RefusalReason"
)

echo "=== AUDIT 1: NO DTO FLATTENING ===" > "$OUTPUT"
echo "Timestamp: $(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$OUTPUT"
echo "Target: $COMPAT_SRC" >> "$OUTPUT"
echo "" >> "$OUTPUT"

VIOLATIONS=0
PASS=true

# Search for forbidden DTO patterns
for pattern in "${FORBIDDEN_PATTERNS[@]}"; do
  matches=$(grep -r "$pattern" "$COMPAT_SRC" 2>/dev/null || true)
  if [ -n "$matches" ]; then
    echo "VIOLATION: Found pattern '$pattern'" >> "$OUTPUT"
    echo "$matches" | sed 's/^/  /' >> "$OUTPUT"
    echo "" >> "$OUTPUT"
    VIOLATIONS=$((VIOLATIONS + 1))
    PASS=false
  fi
done

# Check for allowed carriers (no violations if these appear correctly)
echo "Allowed carriers found:" >> "$OUTPUT"
for carrier in "${ALLOWED_CARRIERS[@]}"; do
  count=$(grep -r "$carrier" "$COMPAT_SRC" 2>/dev/null | wc -l || echo "0")
  if [ "$count" -gt 0 ]; then
    echo "  $carrier: $count occurrences (allowed)" >> "$OUTPUT"
  fi
done
echo "" >> "$OUTPUT"

if [ "$PASS" = true ]; then
  echo "RESULT: PASS" >> "$OUTPUT"
  echo "No forbidden DTO patterns detected." >> "$OUTPUT"
else
  echo "RESULT: FAIL" >> "$OUTPUT"
  echo "Found $VIOLATIONS violation(s)." >> "$OUTPUT"
fi

echo "" >> "$OUTPUT"
echo "Audit complete: $(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$OUTPUT"

cat "$OUTPUT"
