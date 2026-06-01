#!/usr/bin/env python3
"""
v30.1.1 Adversarial Process Intelligence Research Program
Simulation Models for Trace Laundering and Spoofing Attacks.
Author: AGI Adversarial Process Intelligence Researcher
"""

import json
import hashlib
import hmac
import uuid
import sys
import pandas as pd
from datetime import datetime, timedelta

# Genuine cryptographic authority key (simulating ERP system secret)
GENUINE_SYSTEM_KEY = b"genuine-erp-authority-secret-key-v30.1.1"
FORGED_SYSTEM_KEY = b"forged-unauthorized-adversary-key"

def canonical_serialize(data):
    """Deterministically serializes event payloads to bytes."""
    return json.dumps(data, sort_keys=True, separators=(',', ':')).encode('utf-8')

def compute_signature(key, payload):
    """Computes an HMAC-SHA256 signature for the given payload."""
    return hmac.new(key, canonical_serialize(payload), hashlib.sha256).hexdigest()

def compute_hash_chain_link(payload, prev_hash, signature):
    """Computes the cryptographic hash chain link for an event."""
    hasher = hashlib.sha256()
    hasher.update(canonical_serialize(payload))
    hasher.update(prev_hash.encode('utf-8'))
    hasher.update(signature.encode('utf-8'))
    return hasher.hexdigest()

def generate_conforming_trace(trace_id, activities, base_time_ns, system_key=GENUINE_SYSTEM_KEY):
    """Generates a conforming trace with valid signatures and hash chain links."""
    trace_events = []
    prev_hash = hashlib.sha256(trace_id.encode('utf-8')).hexdigest()
    
    current_time_ns = base_time_ns
    for i, activity in enumerate(activities):
        # Add realistic execution delays
        current_time_ns += int(timedelta(hours=2).total_seconds() * 1e9)
        
        payload = {
            "event_id": str(uuid.uuid4()),
            "activity_name": activity,
            "timestamp_ns": current_time_ns,
            "attributes": {
                "cost": float(100 + i * 50),
                "resource_id": f"agent_{i % 3}"
            }
        }
        
        signature = compute_signature(system_key, payload)
        hash_link = compute_hash_chain_link(payload, prev_hash, signature)
        
        trace_events.append({
            "payload": payload,
            "signature": signature,
            "hash_chain_link": hash_link
        })
        prev_hash = hash_link
        
    return {
        "trace_id": trace_id,
        "events": trace_events
    }

class LaunderingRefusalError(Exception):
    """Custom exception raised when ingestion boundary checks fail."""
    def __init__(self, rule_failed, trace_id, detailed_error):
        self.rule_failed = rule_failed
        self.trace_id = trace_id
        self.detailed_error = detailed_error
        super().__init__(detailed_error)

class IngestionBoundary:
    """Zero-trust ingestion boundary that validates input logs."""
    
    def __init__(self, system_key=GENUINE_SYSTEM_KEY):
        self.system_key = system_key
        
    def validate_log(self, log_input, evaluation_id="eval_default"):
        # 1. Ingestion Boundary: Reject Unhashed Pandas Dataframe
        if isinstance(log_input, pd.DataFrame):
            # A raw pandas DataFrame is mutable and has no cryptographic envelope
            refusal_response = {
                "evaluation_id": evaluation_id,
                "analyzed_log_hash_sha256": "0000000000000000000000000000000000000000000000000000000000000000",
                "verdict": "REJECTED",
                "detection_rules": {
                    "check_cryptographic_chain": False,
                    "check_impossible_velocity": False,
                    "check_timestamp_monotonicity": False,
                    "check_signature_authenticity": False,
                    "reject_unhashed_dataframe": True
                },
                "refusal_reason": {
                    "rule_failed": "reject_unhashed_dataframe",
                    "trace_id": "N/A - Direct DataFrame Ingestion",
                    "detailed_error": "Rejected unhashed pandas DataFrame at ingestion boundary. Logs must be immutable, pre-hashed, and signed."
                }
            }
            return refusal_response

        # Compute analyzed log hash (SHA-256 of canonical JSON serialized events data)
        try:
            canonical_log = json.dumps(log_input, sort_keys=True, separators=(',', ':')).encode('utf-8')
            log_hash = hashlib.sha256(canonical_log).hexdigest()
        except Exception as e:
            log_hash = "malformed_json_hash"

        # Initialize detection rules audit state
        detection_rules = {
            "check_cryptographic_chain": True,
            "check_impossible_velocity": True,
            "check_timestamp_monotonicity": True,
            "check_signature_authenticity": True,
            "reject_unhashed_dataframe": True
        }

        try:
            # Validate structure
            if not isinstance(log_input, dict) or "traces" not in log_input:
                raise LaunderingRefusalError(
                    "check_cryptographic_chain", 
                    "N/A", 
                    "Log envelope is malformed or missing 'traces' container."
                )

            for trace in log_input["traces"]:
                trace_id = trace.get("trace_id", "unknown_trace")
                events = trace.get("events", [])
                
                prev_hash = hashlib.sha256(trace_id.encode('utf-8')).hexdigest()
                prev_timestamp = None
                
                for idx, event in enumerate(events):
                    payload = event.get("payload")
                    signature = event.get("signature")
                    hash_chain_link = event.get("hash_chain_link")
                    
                    if not payload or not signature or not hash_chain_link:
                        raise LaunderingRefusalError(
                            "check_cryptographic_chain",
                            trace_id,
                            f"Event at index {idx} is missing cryptographic payload, signature, or hash chain link."
                        )
                        
                    # A. Check Signature Authenticity (Anti-Spoofing Check)
                    expected_signature = compute_signature(self.system_key, payload)
                    if not hmac.compare_digest(signature, expected_signature):
                        raise LaunderingRefusalError(
                            "check_signature_authenticity",
                            trace_id,
                            f"Event '{payload.get('activity_name')}' at index {idx} contains a forged or invalid signature."
                        )
                        
                    # B. Check Cryptographic Chain Link (Anti-Laundering Check)
                    expected_hash_link = compute_hash_chain_link(payload, prev_hash, signature)
                    if hash_chain_link != expected_hash_link:
                        raise LaunderingRefusalError(
                            "check_cryptographic_chain",
                            trace_id,
                            f"Event '{payload.get('activity_name')}' at index {idx} has an invalid transition state hash (chain broken)."
                        )
                        
                    # C. Check Timestamp Monotonicity (Temporal Monotonicity Check)
                    current_timestamp = payload.get("timestamp_ns")
                    if prev_timestamp is not None and current_timestamp < prev_timestamp:
                        raise LaunderingRefusalError(
                            "check_timestamp_monotonicity",
                            trace_id,
                            f"Event '{payload.get('activity_name')}' at index {idx} violates temporal monotonicity (timestamp goes backwards)."
                        )
                        
                    # D. Check Impossible Velocity (Operational Bound Check)
                    if prev_timestamp is not None:
                        time_delta_ns = current_timestamp - prev_timestamp
                        # Minimum duration between events: 1 second (1e9 nanoseconds)
                        if time_delta_ns < 1e9:
                            raise LaunderingRefusalError(
                                "check_impossible_velocity",
                                trace_id,
                                f"Event '{payload.get('activity_name')}' at index {idx} has impossible velocity: {time_delta_ns / 1e9} seconds from previous event."
                            )
                            
                    prev_hash = hash_chain_link
                    prev_timestamp = current_timestamp
                    
            # All traces passed
            return {
                "evaluation_id": evaluation_id,
                "analyzed_log_hash_sha256": log_hash,
                "verdict": "ACCEPTED",
                "detection_rules": detection_rules
            }
            
        except LaunderingRefusalError as ex:
            return {
                "evaluation_id": evaluation_id,
                "analyzed_log_hash_sha256": log_hash,
                "verdict": "REJECTED",
                "detection_rules": detection_rules,
                "refusal_reason": {
                    "rule_failed": ex.rule_failed,
                    "trace_id": ex.trace_id,
                    "detailed_error": ex.detailed_error
                }
            }

def run_simulation_tests():
    print("=== INITIALIZING V30.1.1 ADVERSARIAL PROCESS SIMULATION MATRIX ===")
    boundary = IngestionBoundary(GENUINE_SYSTEM_KEY)
    base_time = 1780292000000000000
    activities = ["Create_Order", "Approve_Order", "Ship_Goods", "Invoice_Customer", "Receive_Payment"]
    
    # ----------------------------------------------------
    # Scenario 0: Conforming Log Generation & Ingestion
    # ----------------------------------------------------
    print("\n--- Running Scenario 0: Conforming Log Ingestion ---")
    trace1 = generate_conforming_trace("trace_001_conforming", activities, base_time)
    trace2 = generate_conforming_trace("trace_002_conforming", activities, base_time + 100000)
    conforming_log = {"traces": [trace1, trace2]}
    
    res0 = boundary.validate_log(conforming_log, "eval_conforming_001")
    print(f"Verdict: {res0['verdict']}")
    print(f"Log Hash: {res0['analyzed_log_hash_sha256']}")
    assert res0["verdict"] == "ACCEPTED"
    
    # ----------------------------------------------------
    # Scenario 1: Log Laundering via Timestamp Modification (SLA Manipulation)
    # ----------------------------------------------------
    print("\n--- Running Scenario 1: Laundering via Timestamp Shifting ---")
    laundered_trace = generate_conforming_trace("trace_99011_laundering_candidate", activities, base_time)
    # Adversary alters timestamp of 'Invoice_Customer' (index 3) to mask delay,
    # without updating signatures/chains.
    laundered_trace["events"][3]["payload"]["timestamp_ns"] -= int(timedelta(hours=1).total_seconds() * 1e9)
    laundered_log = {"traces": [laundered_trace]}
    
    res1 = boundary.validate_log(laundered_log, "eval_laundering_shift")
    print(f"Verdict: {res1['verdict']}")
    print(f"Failed Rule: {res1['refusal_reason']['rule_failed']}")
    print(f"Detailed Error: {res1['refusal_reason']['detailed_error']}")
    assert res1["verdict"] == "REJECTED"
    assert res1["refusal_reason"]["rule_failed"] == "check_signature_authenticity" # caught because payload changed!

    # ----------------------------------------------------
    # Scenario 2: Log Laundering via Event Deletion
    # ----------------------------------------------------
    print("\n--- Running Scenario 2: Laundering via Event Deletion ---")
    deleted_trace = generate_conforming_trace("trace_99011_laundering_candidate", activities, base_time)
    # Adversary deletes intermediate activity "Ship_Goods" (index 2) to hide transit duration
    deleted_trace["events"].pop(2)
    deleted_log = {"traces": [deleted_trace]}
    
    res2 = boundary.validate_log(deleted_log, "eval_laundering_deletion")
    print(f"Verdict: {res2['verdict']}")
    print(f"Failed Rule: {res2['refusal_reason']['rule_failed']}")
    print(f"Detailed Error: {res2['refusal_reason']['detailed_error']}")
    assert res2["verdict"] == "REJECTED"
    assert res2["refusal_reason"]["rule_failed"] == "check_cryptographic_chain"

    # ----------------------------------------------------
    # Scenario 3: Signature Forgery Attack
    # ----------------------------------------------------
    print("\n--- Running Scenario 3: Signature Forgery Attempt ---")
    # Adversary alters event payload and regenerates ALL signatures and chain links
    # using a FORGED key, trying to spoof the verification layer.
    forged_trace = generate_conforming_trace(
        "trace_99011_laundering_candidate", 
        ["Create_Order", "Approve_Order", "Audit_Invoice", "Ship_Goods", "Invoice_Customer", "Receive_Payment"], 
        base_time,
        system_key=FORGED_SYSTEM_KEY
    )
    forged_log = {"traces": [forged_trace]}
    
    res3 = boundary.validate_log(forged_log, "eval_forgery_attack")
    print(f"Verdict: {res3['verdict']}")
    print(f"Failed Rule: {res3['refusal_reason']['rule_failed']}")
    print(f"Detailed Error: {res3['refusal_reason']['detailed_error']}")
    assert res3["verdict"] == "REJECTED"
    assert res3["refusal_reason"]["rule_failed"] == "check_signature_authenticity"

    # ----------------------------------------------------
    # Scenario 4: Ingestion Boundary: Reject Raw Pandas DataFrame
    # ----------------------------------------------------
    print("\n--- Running Scenario 4: Unhashed DataFrame Ingestion Rejection ---")
    # Adversary attempts to pass a raw pandas DataFrame representing event logs
    events_df = pd.DataFrame([
        {"trace_id": "t1", "activity": "Create", "time": base_time},
        {"trace_id": "t1", "activity": "Approve", "time": base_time + 1000}
    ])
    
    res4 = boundary.validate_log(events_df, "eval_raw_dataframe")
    print(f"Verdict: {res4['verdict']}")
    print(f"Failed Rule: {res4['refusal_reason']['rule_failed']}")
    print(f"Detailed Error: {res4['refusal_reason']['detailed_error']}")
    assert res4["verdict"] == "REJECTED"
    assert res4["refusal_reason"]["rule_failed"] == "reject_unhashed_dataframe"
    
    # ----------------------------------------------------
    # Scenario 5: Impossible Velocity Attack
    # ----------------------------------------------------
    print("\n--- Running Scenario 5: Impossible Velocity Detection ---")
    velocity_trace = generate_conforming_trace("trace_99011_laundering_candidate", activities, base_time)
    # Manually change the timestamp difference between index 0 and index 1 to be 100 nanoseconds (impossible velocity)
    # We must recalculate signature and chain links with GENUINE key to isolate velocity check.
    payload0 = velocity_trace["events"][0]["payload"]
    payload1 = velocity_trace["events"][1]["payload"]
    payload1["timestamp_ns"] = payload0["timestamp_ns"] + 100 # 100 ns difference
    
    # Recalculate signature and links
    sig1 = compute_signature(GENUINE_SYSTEM_KEY, payload1)
    velocity_trace["events"][1]["signature"] = sig1
    velocity_trace["events"][1]["payload"] = payload1
    
    # Recalculate chain link
    prev_hash = velocity_trace["events"][0]["hash_chain_link"]
    h1 = compute_hash_chain_link(payload1, prev_hash, sig1)
    velocity_trace["events"][1]["hash_chain_link"] = h1
    
    # Update downstream chain links to preserve chain integrity
    prev_hash = h1
    for idx in range(2, len(velocity_trace["events"])):
        payload_idx = velocity_trace["events"][idx]["payload"]
        sig_idx = velocity_trace["events"][idx]["signature"]
        h_idx = compute_hash_chain_link(payload_idx, prev_hash, sig_idx)
        velocity_trace["events"][idx]["hash_chain_link"] = h_idx
        prev_hash = h_idx
        
    velocity_log = {"traces": [velocity_trace]}
    res5 = boundary.validate_log(velocity_log, "eval_velocity_check")
    print(f"Verdict: {res5['verdict']}")
    print(f"Failed Rule: {res5['refusal_reason']['rule_failed']}")
    print(f"Detailed Error: {res5['refusal_reason']['detailed_error']}")
    assert res5["verdict"] == "REJECTED"
    assert res5["refusal_reason"]["rule_failed"] == "check_impossible_velocity"
    
    print("\n=== ALL ADVERSARIAL SIMULATION SCENARIOS SUCCESSFULLY VERIFIED ===")

if __name__ == "__main__":
    run_simulation_tests()
