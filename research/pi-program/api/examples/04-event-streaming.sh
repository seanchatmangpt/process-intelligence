#!/bin/bash
# Example 4: OCEL Event Stream Subscription
# Purpose: Subscribe to real-time object-centric events
# Authority: Process Intelligence Research Directorate

API_BASE="https://api.process-intelligence.research/v1"
API_KEY="${PI_API_KEY}"
TIMEOUT_SECONDS="${STREAM_TIMEOUT:-10}"  # Default 10 seconds for demo

if [ -z "$API_KEY" ]; then
  echo "Error: PI_API_KEY environment variable not set"
  exit 1
fi

echo "=== OCEL Event Stream Subscription ==="
echo

# Subscribe to OCEL events since now
echo "1. Subscribing to OCEL events (will run for $TIMEOUT_SECONDS seconds)..."
echo "   Events: All types"
echo "   Objects: All types"
echo

timeout $TIMEOUT_SECONDS curl -s -N "$API_BASE/stream/ocel" \
  -H "X-API-Key: $API_KEY" \
  -H "Accept: text/event-stream" | \
while IFS= read -r line; do
  if [[ $line == "data: "* ]]; then
    # Extract JSON after "data: "
    json_data="${line#data: }"

    # Extract key fields
    activity=$(echo "$json_data" | jq -r '.["ocel:activity"]' 2>/dev/null)
    timestamp=$(echo "$json_data" | jq -r '.["ocel:timestamp"]' 2>/dev/null)
    event_id=$(echo "$json_data" | jq -r '.["ocel:event-id"]' 2>/dev/null)

    if [ ! -z "$activity" ] && [ "$activity" != "null" ]; then
      echo "  [$(echo $timestamp | cut -d'T' -f2)] $activity (ID: $event_id)"
    fi
  fi
done

echo
echo "Stream subscription ended"
echo

# Alternative: Subscribe to specific object type
echo "2. Subscribing to Artifact events only..."
echo

timeout 5 curl -s -N "$API_BASE/stream/ocel?object-type=Artifact" \
  -H "X-API-Key: $API_KEY" \
  -H "Accept: text/event-stream" | \
while IFS= read -r line; do
  if [[ $line == "data: "* ]]; then
    json_data="${line#data: }"
    activity=$(echo "$json_data" | jq -r '.["ocel:activity"]' 2>/dev/null)

    if [ ! -z "$activity" ] && [ "$activity" != "null" ]; then
      echo "  ← $activity"
    fi
  fi
done

echo
echo "✓ Event stream examples complete"
