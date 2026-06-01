#!/usr/bin/env python3
"""
validate_ocpq_refinement.py

Programmatically verifies the OCPQ query variable refinement relation:
    p <=_L c <=> dom(p) subseteq dom(c) and forall x in dom(p), p(x) = c(x)

Under the v30.1.1 ultimate standard.
"""

import sys


def refines(parent: dict, child: dict) -> bool:
    """
    Determines whether child is a refinement of parent under the relation p <=_L c.
    
    Relation definition:
      dom(p) is a subset of dom(c) AND
      for all x in dom(p), p(x) = c(x)
    """
    if not isinstance(parent, dict) or not isinstance(child, dict):
        raise TypeError("Both parent and child must be dict instances representing variable domains.")
        
    for var, val in parent.items():
        # Check if the variable is in child's domain
        if var not in child:
            return False
        # Check if the values match
        if child[var] != val:
            return False
            
    return True


def test_ocpq_refinement():
    print("==================================================")
    print("Testing OCPQ Query Variable Refinement Relation")
    print("==================================================")

    # 1. Reflexivity (p <=_L p)
    p1 = {"order_id": "po_4001", "status": "COMPLIANT"}
    print(f"Testing reflexivity with p = {p1}")
    res_reflexive = refines(p1, p1)
    print(f"  Result: {res_reflexive}")
    assert res_reflexive is True, "Reflexivity failed: parent should refine itself."

    # 2. Proper extension (child domain is a strict superset of parent, values match)
    p2 = {"order_id": "po_4001"}
    c2 = {"order_id": "po_4001", "part_id": "part_a", "shipment_id": "ship_99"}
    print(f"Testing proper extension:\n  parent = {p2}\n  child  = {c2}")
    res_extension = refines(p2, c2)
    print(f"  Result: {res_extension}")
    assert res_extension is True, "Proper extension failed: child with extra variables should refine parent."

    # 3. Conflicting values (overlapping variables have different values)
    p3 = {"order_id": "po_4001", "status": "COMPLIANT"}
    c3 = {"order_id": "po_4001", "status": "VIOLATION", "part_id": "part_a"}
    print(f"Testing conflicting values:\n  parent = {p3}\n  child  = {c3}")
    res_conflict = refines(p3, c3)
    print(f"  Result: {res_conflict}")
    assert res_conflict is False, "Conflicting values failed: child with different values should not refine parent."

    # 4. Missing domain variables (child is missing a variable from parent)
    p4 = {"order_id": "po_4001", "status": "COMPLIANT"}
    c4 = {"order_id": "po_4001"}
    print(f"Testing missing domain variables:\n  parent = {p4}\n  child  = {c4}")
    res_missing = refines(p4, c4)
    print(f"  Result: {res_missing}")
    assert res_missing is False, "Missing domain variables failed: child missing parent variable should not refine parent."

    # Extra robustness check: empty parent (should always be refined by any child)
    p_empty = {}
    c_any = {"x": 42}
    print(f"Testing empty parent:\n  parent = {p_empty}\n  child  = {c_any}")
    res_empty_parent = refines(p_empty, c_any)
    print(f"  Result: {res_empty_parent}")
    assert res_empty_parent is True, "Empty parent check failed."

    print("\nSUCCESS: All OCPQ refinement relation assertions passed!")
    print("==================================================")


if __name__ == "__main__":
    try:
        test_ocpq_refinement()
        sys.exit(0)
    except AssertionError as e:
        print(f"FAILURE: Assertion error: {e}", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"FAILURE: Unexpected error: {e}", file=sys.stderr)
        sys.exit(1)
