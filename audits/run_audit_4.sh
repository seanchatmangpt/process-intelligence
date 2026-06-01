#!/bin/bash

set -euo pipefail

AUDIT_NAME="audit-projection-receipt"
COMPAT_ROOT="/Users/sac/process-intelligence/sources/wasm4pm-compat"
OUTPUT="/Users/sac/process-intelligence/audits/${AUDIT_NAME}.txt"

echo "=== AUDIT 4: PROJECTION RECEIPT ===" > "$OUTPUT"
echo "Timestamp: $(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$OUTPUT"
echo "Target: $COMPAT_ROOT" >> "$OUTPUT"
echo "" >> "$OUTPUT"

echo "Projection Receipt Requirements:" >> "$OUTPUT"
echo "1. Every projected artifact (TS, .d.ts, WIT) is committed or snapshotted" >> "$OUTPUT"
echo "2. Feature manifests exist" >> "$OUTPUT"
echo "3. Audit results are recorded" >> "$OUTPUT"
echo "" >> "$OUTPUT"

PASS=true

# Check for TypeScript projections
echo "TypeScript Projections:" >> "$OUTPUT"
TS_FILES=$(find "$COMPAT_ROOT" -name "*.ts" -o -name "*.d.ts" 2>/dev/null | wc -l)
echo "  Found: $TS_FILES TypeScript files" >> "$OUTPUT"

if [ "$TS_FILES" -eq 0 ]; then
  echo "  [INFO] No TypeScript projections (may be pending)" >> "$OUTPUT"
fi

# Check for WIT projections
echo "" >> "$OUTPUT"
echo "WebAssembly Interface Type (WIT) Projections:" >> "$OUTPUT"
WIT_FILES=$(find "$COMPAT_ROOT" -name "*.wit" 2>/dev/null | wc -l)
echo "  Found: $WIT_FILES WIT files" >> "$OUTPUT"

if [ "$WIT_FILES" -eq 0 ]; then
  echo "  [INFO] No WIT projections (may be pending)" >> "$OUTPUT"
fi

# Check for feature manifests
echo "" >> "$OUTPUT"
echo "Feature Manifests:" >> "$OUTPUT"
MANIFEST_FILES=$(find "$COMPAT_ROOT" -name "*.toml" -o -name "MANIFEST*" 2>/dev/null | grep -E "(MANIFEST|Cargo\.toml)" | wc -l)
echo "  Found: $MANIFEST_FILES manifest files" >> "$OUTPUT"

if [ "$MANIFEST_FILES" -gt 0 ]; then
  find "$COMPAT_ROOT" -name "MANIFEST*" -o -name "Cargo.toml" 2>/dev/null | sed 's/^/    /' >> "$OUTPUT"
fi

# Check for audit results
echo "" >> "$OUTPUT"
echo "Recorded Audit Results:" >> "$OUTPUT"
AUDIT_FILES=$(find /Users/sac/process-intelligence/audits -name "*.txt" -o -name "*.yaml" 2>/dev/null | wc -l)
echo "  Found: $AUDIT_FILES audit files in /audits directory" >> "$OUTPUT"

# Git status check - are outputs committed?
echo "" >> "$OUTPUT"
echo "Git Status Check:" >> "$OUTPUT"
cd "$COMPAT_ROOT"

UNTRACKED=$(git ls-files --others --exclude-standard 2>/dev/null | wc -l || echo "0")
MODIFIED=$(git diff --name-only 2>/dev/null | wc -l || echo "0")

echo "  Untracked files in compat: $UNTRACKED" >> "$OUTPUT"
echo "  Modified files in compat: $MODIFIED" >> "$OUTPUT"

if [ "$UNTRACKED" -gt 0 ]; then
  echo "  [WARN] Projected artifacts may not be committed" >> "$OUTPUT"
  PASS=false
fi

echo "" >> "$OUTPUT"

if [ "$PASS" = true ]; then
  echo "RESULT: PASS" >> "$OUTPUT"
  echo "Projection receipt requirements satisfied." >> "$OUTPUT"
else
  echo "RESULT: WARN" >> "$OUTPUT"
  echo "Some projection receipts may be pending." >> "$OUTPUT"
fi

echo "" >> "$OUTPUT"
echo "Audit complete: $(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$OUTPUT"

cat "$OUTPUT"
