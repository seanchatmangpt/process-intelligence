#!/usr/bin/env python3
"""
Verification Script: Weighted Log Conformance Fitness & OCPQ Query Refinement
Governed by Dr. Wil van der Aalst AGI standards under v30.1.1.
"""

import sys
from typing import Dict, List, Any

# =============================================================================
# 1. Weighted Log Fitness Equation Verification
# =============================================================================

def calculate_log_fitness(traces: List[Dict[str, Any]]) -> float:
    """
    Computes the log fitness f(L, N) based on token game metrics:
    f(L, N) = 1 - (sum_{sigma} L(sigma)*m(sigma) / sum_{sigma} L(sigma)*c(sigma))
                - (sum_{sigma} L(sigma)*r(sigma) / sum_{sigma} L(sigma)*p(sigma))
    """
    total_weighted_missing = 0.0
    total_weighted_consumed = 0.0
    total_weighted_remaining = 0.0
    total_weighted_produced = 0.0

    for i, trace in enumerate(traces):
        freq = trace.get("frequency", 1)
        m = trace.get("missing", 0)
        c = trace.get("consumed", 0)
        r = trace.get("remaining", 0)
        p = trace.get("produced", 0)

        # Defensive boundaries
        assert freq >= 0, f"Trace {i}: Frequency must be non-negative, found {freq}"
        assert m >= 0, f"Trace {i}: Missing tokens must be non-negative, found {m}"
        assert c >= 0, f"Trace {i}: Consumed tokens must be non-negative, found {c}"
        assert r >= 0, f"Trace {i}: Remaining tokens must be non-negative, found {r}"
        assert p >= 0, f"Trace {i}: Produced tokens must be non-negative, found {p}"

        if c == 0 and m > 0:
            raise ValueError(f"Trace {i}: Consumed tokens cannot be zero if missing tokens exist ({m})")
        if p == 0 and r > 0:
            raise ValueError(f"Trace {i}: Produced tokens cannot be zero if remaining tokens exist ({r})")

        total_weighted_missing += freq * m
        total_weighted_consumed += freq * c
        total_weighted_remaining += freq * r
        total_weighted_produced += freq * p

    # Safety checks for total denominators
    if total_weighted_consumed == 0:
        raise ZeroDivisionError("Total weighted consumed tokens across the log is zero.")
    if total_weighted_produced == 0:
        raise ZeroDivisionError("Total weighted produced tokens across the log is zero.")

    missing_term = total_weighted_missing / total_weighted_consumed
    remaining_term = total_weighted_remaining / total_weighted_produced
    fitness = 1.0 - missing_term - remaining_term

    # Conformance fitness must mathematically stay within [0, 1] for sound nets.
    # We enforce defensive clamping/check boundaries.
    assert -1e-9 <= fitness <= 1.0 + 1e-9, f"Computed fitness {fitness} is outside mathematical [0, 1] range"
    return max(0.0, min(1.0, fitness))


# =============================================================================
# 2. OCPQ Query Variable Refinement Relation Verification
# =============================================================================

def is_refinement(p: Dict[str, Any], c: Dict[str, Any]) -> bool:
    """
    Evaluates the refinement relation:
    p <=_L c <=> dom(p) subseteq dom(c) AND forall x in dom(p), p(x) == c(x)
    """
    # Check domain subset inclusion: dom(p) must be a subset of dom(c)
    p_domain = set(p.keys())
    c_domain = set(c.keys())
    
    if not p_domain.issubset(c_domain):
        return False
        
    # Check value correspondence for all variables in dom(p)
    for x in p_domain:
        if p[x] != c[x]:
            return False
            
    return True


# =============================================================================
# 3. Test Invariants & Assertion Run
# =============================================================================

def run_tests():
    print("--- INITIATING PROCESS INTELLIGENCE EQUATION VERIFICATION GATE ---")
    
    # Test Suite 1: Log Conformance Fitness
    print("\n[Test Suite 1] Verifying Log Conformance Fitness...")
    
    # Case 1.1: Perfect conformance
    perfect_log = [
        {"frequency": 10, "missing": 0, "consumed": 50, "remaining": 0, "produced": 50},
        {"frequency": 5, "missing": 0, "consumed": 30, "remaining": 0, "produced": 30}
    ]
    f_perfect = calculate_log_fitness(perfect_log)
    print(f"  - Case 1.1 (Perfect Conformance): Fitness = {f_perfect:.5f}")
    assert f_perfect == 1.0, "Expected perfect fitness of 1.0"

    # Case 1.2: Partial conformance
    partial_log = [
        {"frequency": 10, "missing": 0, "consumed": 50, "remaining": 0, "produced": 50},
        {"frequency": 5, "missing": 2, "consumed": 30, "remaining": 1, "produced": 29}
    ]
    f_partial = calculate_log_fitness(partial_log)
    print(f"  - Case 1.2 (Partial Conformance): Fitness = {f_partial:.5f}")
    # Math calculation:
    # missing_term = (10*0 + 5*2) / (10*50 + 5*30) = 10 / 650 = 0.0153846
    # remaining_term = (10*0 + 5*1) / (10*50 + 5*29) = 5 / 645 = 0.0077519
    # expected = 1.0 - 0.0153846 - 0.0077519 = 0.9768635
    assert abs(f_partial - 0.9768635) < 1e-6, f"Expected ~0.97686, got {f_partial}"

    # Case 1.3: Low conformance bounds
    low_conformance_log = [
        {"frequency": 1, "missing": 20, "consumed": 30, "remaining": 10, "produced": 30}
    ]
    f_low = calculate_log_fitness(low_conformance_log)
    print(f"  - Case 1.3 (Low Conformance): Fitness = {f_low:.5f}")
    # missing_term = 20 / 30 = 0.66667
    # remaining_term = 10 / 30 = 0.33333
    # expected = 1 - 0.66667 - 0.33333 = 0.0
    assert abs(f_low - 0.0) < 1e-6, f"Expected 0.0, got {f_low}"

    # Case 1.4: Defensively handling negative frequencies
    print("  - Case 1.4 (Defensive Negative Frequency Checking):")
    try:
        calculate_log_fitness([{"frequency": -1, "missing": 0, "consumed": 10, "remaining": 0, "produced": 10}])
        print("    ERROR: Failed to trap negative frequency")
        sys.exit(1)
    except AssertionError as e:
        print(f"    PASSED: Successfully trapped negative frequency: '{e}'")

    # Case 1.5: Defensively handling missing-to-consumed mismatch
    print("  - Case 1.5 (Defensive Consumed Underflow Checking):")
    try:
        calculate_log_fitness([{"frequency": 1, "missing": 5, "consumed": 0, "remaining": 0, "produced": 10}])
        print("    ERROR: Failed to trap zero consumed with non-zero missing tokens")
        sys.exit(1)
    except ValueError as e:
        print(f"    PASSED: Successfully trapped mismatch: '{e}'")


    # Test Suite 2: OCPQ Variable Refinement
    print("\n[Test Suite 2] Verifying OCPQ Query Refinement Relation...")

    # Case 2.1: Simple valid refinement
    p1 = {"e1": "event_101", "o1": "object_202"}
    c1 = {"e1": "event_101", "o1": "object_202", "o2": "object_303"}
    ref1 = is_refinement(p1, c1)
    print(f"  - Case 2.1 (Valid Refinement): Result = {ref1}")
    assert ref1 is True, "Expected True for valid domain subset and matching mappings"

    # Case 2.2: Value mismatch
    p2 = {"e1": "event_101", "o1": "object_202"}
    c2 = {"e1": "event_101", "o1": "object_999", "o2": "object_303"} # o1 has different value
    ref2 = is_refinement(p2, c2)
    print(f"  - Case 2.2 (Value Mismatch): Result = {ref2}")
    assert ref2 is False, "Expected False due to value mismatch on o1"

    # Case 2.3: Domain mismatch (parent has variable missing in child)
    p3 = {"e1": "event_101", "o1": "object_202", "z1": "extra"}
    c3 = {"e1": "event_101", "o1": "object_202"}
    ref3 = is_refinement(p3, c3)
    print(f"  - Case 2.3 (Domain Mismatch): Result = {ref3}")
    assert ref3 is False, "Expected False because dom(p) is not a subset of dom(c)"

    # Case 2.4: Empty parent binding
    p4 = {}
    c4 = {"e1": "event_101"}
    ref4 = is_refinement(p4, c4)
    print(f"  - Case 2.4 (Empty Parent Binding): Result = {ref4}")
    assert ref4 is True, "Expected True since empty set is subset of any set"

    print("\n--- ALL VALIDATION CHECKS COMPLETED WITH 100% CORRECTNESS ---")

if __name__ == "__main__":
    run_tests()
