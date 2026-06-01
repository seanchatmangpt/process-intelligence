#!/usr/bin/env python3
import json
import urllib.request
import urllib.error
import time
import sys

TRACES_URL = "http://localhost:4318/v1/traces"
METRICS_URL = "http://localhost:4318/v1/metrics"

def send_payload(url, data):
    payload = json.dumps(data).encode("utf-8")
    req = urllib.request.Request(
        url,
        data=payload,
        headers={"Content-Type": "application/json"}
    )
    try:
        with urllib.request.urlopen(req) as res:
            return res.status, res.read().decode("utf-8")
    except urllib.error.HTTPError as e:
        print(f"HTTP Error {e.code}: {e.read().decode('utf-8')}", file=sys.stderr)
        return e.code, None
    except urllib.error.URLError as e:
        print(f"URL Error: {e.reason}", file=sys.stderr)
        return None, None

def generate_traces():
    # Current timestamp in nanoseconds
    now_ns = int(time.time() * 1e9)
    
    # 1. Span A: Compliant Telemetry Feedstock
    # Contains all required semantic convention attributes.
    # Mixed-case activity name to verify transform/pi processor.
    span_a = {
        "traceId": "4bf92f3577b34da6a3ce929d0e0e4736",
        "spanId": "00f067aa0ba902b7",
        "name": "ApproveInvoiceSpan",
        "kind": "SPAN_KIND_INTERNAL",
        "startTimeUnixNano": str(now_ns - 10000000),
        "endTimeUnixNano": str(now_ns),
        "attributes": [
            {"key": "process.pi.instance_id", "value": {"stringValue": "inst-8874f-99bc2-3312a"}},
            {"key": "process.pi.activity.name", "value": {"stringValue": "APPROVE_INVOICE"}},
            {"key": "process.pi.activity.type", "value": {"stringValue": "task"}},
            {"key": "process.pi.lifecycle", "value": {"stringValue": "complete"}},
            {"key": "process.pi.witness.id", "value": {"stringValue": "auth_governor_alpha"}},
            {"key": "process.pi.witness.hash", "value": {"stringValue": "4a98f1c8b210e309228d4bf09c0fa4db55fe310efc35ad1a89b3f021e85a6111"}},
        ],
        "events": [
            {
                "timeUnixNano": str(now_ns - 5000000),
                "name": "authorization_started",
                "attributes": [
                    {"key": "event_detail", "value": {"stringValue": "checking credentials"}}
                ]
            }
        ]
    }

    # 2. Span B: Missing Instance ID (Filtered Feedstock)
    # Should be dropped by filter/pi processor because process.pi.instance_id is nil.
    span_b = {
        "traceId": "4bf92f3577b34da6a3ce929d0e0e4737",
        "spanId": "00f067aa0ba902b8",
        "name": "ReceivePaymentSpan",
        "kind": "SPAN_KIND_INTERNAL",
        "startTimeUnixNano": str(now_ns - 10000000),
        "endTimeUnixNano": str(now_ns),
        "attributes": [
            {"key": "process.pi.activity.name", "value": {"stringValue": "receive_payment"}},
            {"key": "process.pi.lifecycle", "value": {"stringValue": "complete"}},
            {"key": "process.pi.witness.id", "value": {"stringValue": "payment_gateway"}},
            {"key": "process.pi.witness.hash", "value": {"stringValue": "5a98f1c8b210e309228d4bf09c0fa4db55fe310efc35ad1a89b3f021e85a6222"}},
        ]
    }

    # 3. Span C: Missing Witness Hash (Admission Failure / Refusal Feedstock)
    # Has process.pi.instance_id so it passes the collector's filter, but will fail
    # live-check schema validation because process.pi.witness.hash is missing.
    span_c = {
        "traceId": "4bf92f3577b34da6a3ce929d0e0e4738",
        "spanId": "00f067aa0ba902b9",
        "name": "ValidateAdmissionSpan",
        "kind": "SPAN_KIND_INTERNAL",
        "startTimeUnixNano": str(now_ns - 10000000),
        "endTimeUnixNano": str(now_ns),
        "attributes": [
            {"key": "process.pi.instance_id", "value": {"stringValue": "inst-8874f-99bc2-3312b"}},
            {"key": "process.pi.activity.name", "value": {"stringValue": "validate_admission"}},
            {"key": "process.pi.lifecycle", "value": {"stringValue": "start"}},
            {"key": "process.pi.witness.id", "value": {"stringValue": "auth_governor_beta"}},
            # Missing process.pi.witness.hash on purpose
        ]
    }

    payload = {
        "resourceSpans": [
            {
                "resource": {
                    "attributes": [
                        {"key": "service.name", "value": {"stringValue": "pi-test-app"}}
                    ]
                },
                "scopeSpans": [
                    {
                        "scope": {
                            "name": "pi-harness",
                            "version": "1.0.0"
                        },
                        "spans": [span_a, span_b, span_c]
                    }
                ]
            }
        ]
    }
    return payload

def generate_metrics():
    now_ns = int(time.time() * 1e9)
    metric_a = {
        "name": "pi.feedstock.operation_count",
        "description": "Number of process operations executed",
        "unit": "1",
        "sum": {
            "dataPoints": [
                {
                    "startTimeUnixNano": str(now_ns - 10000000),
                    "timeUnixNano": str(now_ns),
                    "asInt": "42",
                    "attributes": [
                        {"key": "process.pi.instance_id", "value": {"stringValue": "inst-8874f-99bc2-3312a"}}
                    ]
                }
            ],
            "aggregationTemporality": "AGGREGATION_TEMPORALITY_CUMULATIVE",
            "isMonotonic": True
        }
    }
    payload = {
        "resourceMetrics": [
            {
                "resource": {
                    "attributes": [
                        {"key": "service.name", "value": {"stringValue": "pi-test-app"}}
                    ]
                },
                "scopeMetrics": [
                    {
                        "scope": {
                            "name": "pi-harness",
                            "version": "1.0.0"
                        },
                        "metrics": [metric_a]
                    }
                ]
            }
        ]
    }
    return payload

def main():
    print("Generating synthetic traces...")
    traces = generate_traces()
    status_t, body_t = send_payload(TRACES_URL, traces)
    if status_t:
        print(f"Traces sent successfully: HTTP {status_t}")
    else:
        print("Failed to send traces", file=sys.stderr)

    print("Generating synthetic metrics...")
    metrics = generate_metrics()
    status_m, body_m = send_payload(METRICS_URL, metrics)
    if status_m:
        print(f"Metrics sent successfully: HTTP {status_m}")
    else:
        print("Failed to send metrics", file=sys.stderr)

if __name__ == "__main__":
    main()
