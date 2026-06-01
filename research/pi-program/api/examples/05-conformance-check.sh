#!/bin/bash
# Example 5: Conformance Checking via PM4Py Integration
# Purpose: Submit event log for conformance analysis
# Authority: Process Intelligence Research Directorate

set -e

API_BASE="https://api.process-intelligence.research/v1"
API_KEY="${PI_API_KEY}"

if [ -z "$API_KEY" ]; then
  echo "Error: PI_API_KEY environment variable not set"
  exit 1
fi

echo "=== Conformance Checking (PM4Py Integration) ==="
echo

# Create sample OCEL event log
echo "1. Creating sample OCEL event log..."

cat > /tmp/event-log.json << 'EOF'
{
  "ocel:global-event": {
    "ocel:timestamp": "timestamp"
  },
  "ocel:events": [
    {
      "ocel:event-id": "event:1",
      "ocel:activity": "rule_started",
      "ocel:timestamp": "2026-06-01T15:00:00Z",
      "ocel:object-refs": [
        { "ocel:object-id": "rule:gen-001", "ocel:object-type": "GenerationRule" },
        { "ocel:object-id": "engine:v1", "ocel:object-type": "ManufacturingEngine" }
      ]
    },
    {
      "ocel:event-id": "event:2",
      "ocel:activity": "artifact_created",
      "ocel:timestamp": "2026-06-01T15:00:30Z",
      "ocel:object-refs": [
        { "ocel:object-id": "artifact:abc123", "ocel:object-type": "Artifact" },
        { "ocel:object-id": "rule:gen-001", "ocel:object-type": "GenerationRule" }
      ]
    },
    {
      "ocel:event-id": "event:3",
      "ocel:activity": "receipt_signed",
      "ocel:timestamp": "2026-06-01T15:00:45Z",
      "ocel:object-refs": [
        { "ocel:object-id": "receipt:def456", "ocel:object-type": "Receipt" },
        { "ocel:object-id": "artifact:abc123", "ocel:object-type": "Artifact" }
      ]
    }
  ],
  "ocel:objects": [
    {
      "ocel:object-id": "engine:v1",
      "ocel:object-type": "ManufacturingEngine",
      "ocel:attributes": {
        "status": "active",
        "version": "1.0.0"
      }
    },
    {
      "ocel:object-id": "rule:gen-001",
      "ocel:object-type": "GenerationRule",
      "ocel:attributes": {
        "name": "Generate Validation",
        "enabled": true
      }
    },
    {
      "ocel:object-id": "artifact:abc123",
      "ocel:object-type": "Artifact",
      "ocel:attributes": {
        "name": "validation-rule.json",
        "status": "completed"
      }
    },
    {
      "ocel:object-id": "receipt:def456",
      "ocel:object-type": "Receipt",
      "ocel:attributes": {
        "hash": "blake3:...",
        "verified": true
      }
    }
  ]
}
EOF

echo "  Created sample OCEL log with 3 events"
echo

# Submit for conformance checking
echo "2. Submitting event log for conformance checking..."
echo "   This will measure:"
echo "   - Fitness (token-based replay)"
echo "   - Precision (model specificity)"
echo "   - Simplicity (Occam's razor)"
echo "   - Generalization (model flexibility)"
echo

RESPONSE=$(curl -s -X POST "$API_BASE/conformance/check" \
  -H "X-API-Key: $API_KEY" \
  -H "Content-Type: application/json" \
  -d @/tmp/event-log.json)

echo "Conformance Results:"
echo "===================="
echo

# Extract and display metrics
RESULT=$(echo "$RESPONSE" | jq '.')

FITNESS=$(echo "$RESULT" | jq '.fitness')
PRECISION=$(echo "$RESULT" | jq '.precision')
SIMPLICITY=$(echo "$RESULT" | jq '.simplicity')
GENERALIZATION=$(echo "$RESULT" | jq '.generalization')
STATUS=$(echo "$RESULT" | jq -r '.status')
RECEIPT=$(echo "$RESULT" | jq -r '.receipt_id')

echo "Fitness:           $FITNESS"
echo "Precision:         $PRECISION"
echo "Simplicity:        $SIMPLICITY"
echo "Generalization:    $GENERALIZATION"
echo "Overall Status:    $STATUS"
echo
echo "Receipt ID: $RECEIPT"
echo "Timestamp:  $(echo "$RESULT" | jq -r '.timestamp')"
echo

# Verify receipt
if [ ! -z "$RECEIPT" ] && [ "$RECEIPT" != "null" ]; then
  echo "3. Verifying conformance receipt..."
  RECEIPT_CHECK=$(curl -s "$API_BASE/receipt/$RECEIPT" \
    -H "X-API-Key: $API_KEY")

  echo "Receipt verified: $(echo "$RECEIPT_CHECK" | jq -r '.verified')"
  echo "Chain parent: $(echo "$RECEIPT_CHECK" | jq -r '.chain_parent')"
fi

echo
echo "Thresholds:"
echo "==========="
echo "Fitness >= 0.95:           $([ $(echo "$FITNESS >= 0.95" | bc) -eq 1 ] && echo "✓ PASS" || echo "✗ FAIL")"
echo "Precision >= 0.90:         $([ $(echo "$PRECISION >= 0.90" | bc) -eq 1 ] && echo "✓ PASS" || echo "✗ FAIL")"
echo "Simplicity >= 0.80:        $([ $(echo "$SIMPLICITY >= 0.80" | bc) -eq 1 ] && echo "✓ PASS" || echo "✗ FAIL")"
echo "Generalization >= 0.85:    $([ $(echo "$GENERALIZATION >= 0.85" | bc) -eq 1 ] && echo "✓ PASS" || echo "✗ FAIL")"
echo

# Clean up
rm -f /tmp/event-log.json

echo "✓ Conformance checking example complete"
