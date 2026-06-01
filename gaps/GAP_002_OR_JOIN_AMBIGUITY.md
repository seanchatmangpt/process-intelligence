# GAP_002: BPMN OR-Join Gateway Ambiguity

**Severity**: MEDIUM  
**Status**: RESOLVED (v30.1.1 Smart-Completion Policy Enforced)  
**Remediation Target**: wasm4pm-compat Core Engine  

---

## 1. Gap Description

The BPMN 2.0 specification is notoriously ambiguous regarding the precise synchronization logic of inclusive gateways (OR-Join). Without a formal policy, different runtimes behave non-deterministically when tokens are inside loops or parallel branch pathways.

---

## 2. Remediation Strategy (Smart-Completion)

We resolve this gap under the v30.1.1 standard by enforcing a **Smart-Completion** policy. At runtime:
- The execution engine constructs a dynamic Reachability Matrix for all active tokens.
- An OR-Join is authorized to fire if and only if no active token has a structural path to any of the waiting incoming arcs of the OR-Join.

$$\forall t \in Tokens_{Active}, \forall a \in Arcs_{Incoming}, \quad t \not\to a$$

This resolves the non-determinism, ensuring reproducible conformance alignments.
