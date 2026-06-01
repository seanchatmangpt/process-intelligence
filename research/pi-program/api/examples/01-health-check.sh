#!/bin/bash
# Example 1: Health Check
# Purpose: Verify ecosystem health status and deployment gates
# Authority: Process Intelligence Research Directorate

set -e

API_BASE="https://api.process-intelligence.research/v1"
API_KEY="${PI_API_KEY}"

if [ -z "$API_KEY" ]; then
  echo "Error: PI_API_KEY environment variable not set"
  echo "Usage: export PI_API_KEY=pi_key_... && $0"
  exit 1
fi

echo "=== Process Intelligence API — Health Check ==="
echo

# Check ecosystem health
echo "1. Checking ecosystem health..."
HEALTH=$(curl -s "$API_BASE/health" \
  -H "X-API-Key: $API_KEY")

echo "Health Status:"
echo "$HEALTH" | jq '.status'

echo
echo "Systems:"
echo "$HEALTH" | jq '.systems'

echo
echo "Deployment Gates (30 total):"
GATES=$(echo "$HEALTH" | jq '.gates')
PASS_COUNT=$(echo "$GATES" | jq '[.[] | select(.status=="PASS")] | length')
FAIL_COUNT=$(echo "$GATES" | jq '[.[] | select(.status=="FAIL")] | length')
BLOCKED_COUNT=$(echo "$GATES" | jq '[.[] | select(.status=="BLOCKED")] | length')

echo "  PASS: $PASS_COUNT"
echo "  FAIL: $FAIL_COUNT"
echo "  BLOCKED: $BLOCKED_COUNT"

if [ "$FAIL_COUNT" -gt 0 ]; then
  echo
  echo "Failed Gates:"
  echo "$GATES" | jq '.[] | select(.status=="FAIL") | {gate_id, name, status, owner}'
fi

# Check Van der Aalst conformance metrics
echo
echo "2. Checking Van der Aalst conformance metrics..."
METRICS=$(curl -s "$API_BASE/metrics" \
  -H "X-API-Key: $API_KEY")

echo "$METRICS" | jq '{
  timestamp: .timestamp,
  fitness: .fitness,
  precision: .precision,
  simplicity: .simplicity,
  generalization: .generalization,
  overall_status: .overall_status
}'

echo
echo "✓ Health check complete"
