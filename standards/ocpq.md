# OCPQ v30.1.1: Multi-Object Path Query Semantics

## Overview
Object-Centric Process Query (OCPQ) is the unified query interface for N-dimensional event logs. In the v30.1.1 protocol, OCPQ provides the query language to audit complex, multi-object transaction flows without flattening them into sequential logs, preserving multi-perspective topology.

## Mathematical Formalization of OCPQ Query Mappings
An Object-Centric Event Log (OCEL) can be formally represented as a directed property graph $G = (V, E, \Sigma, \mathcal{T}, \mathcal{A})$ where:
*   $V = E_{\text{events}} \cup O_{\text{objects}}$ is the set of vertices partitioned into events and objects.
*   $E \subseteq (E_{\text{events}} \times O_{\text{objects}}) \cup (O_{\text{objects}} \times O_{\text{objects}})$ is the set of directed edges representing event-to-object links and object-to-object relationships.
*   $\Sigma$ is the set of attribute keys, event types, and object classes.
*   $\mathcal{T}: V \cup E \rightarrow \Sigma$ is a type-labeling function.
*   $\mathcal{A}: (V \cup E) \times \Sigma \rightarrow \text{Val}$ maps vertices and edges to their attribute values.

An OCPQ query is defined by a query pattern graph $Q = (V_Q, E_Q, \mathcal{T}_Q, \mathcal{C}_Q)$ where:
*   $V_Q = V_E \cup V_O$ is the set of query event and object variables.
*   $E_Q \subseteq V_E \times V_O$ represents the required event-to-object bindings.
*   $\mathcal{T}_Q: V_Q \cup E_Q \rightarrow \Sigma$ defines the required types or classes.
*   $\mathcal{C}_Q$ represents the set of temporal and attribute constraints.

A query mapping (or match) of the query $Q$ over the OCEL directed property graph $G$ is defined as finding an injective subgraph isomorphism $g: Q \rightarrow G$. Specifically, the mapping function $g: V_Q \rightarrow V$ must satisfy:
1.  **Injectivity**: $\forall u, v \in V_Q, \ u \neq v \implies g(u) \neq g(v)$.
2.  **Edge Preservation**: $\forall (u, v) \in E_Q, \ (g(u), g(v)) \in E$.
3.  **Type Compatibility**: $\forall v \in V_Q, \ \mathcal{T}(g(v)) = \mathcal{T}_Q(v)$, and $\forall e \in E_Q, \ \mathcal{T}(g(e)) = \mathcal{T}_Q(e)$.
4.  **Constraint Satisfaction**: The mapped vertices $g(V_Q)$ and edges $g(E_Q)$ satisfy all constraints in $\mathcal{C}_Q$, such as:
    $$\forall (e_a, e_b, \text{before}, \Delta) \in \mathcal{C}_Q, \quad \mathcal{A}(g(e_b), \text{timestamp}) - \mathcal{A}(g(e_a), \text{timestamp}) \ge \Delta$$

## Lifecycle Actuation Mapping
OCPQ powers the **Autonomic Query Gate**. During runtime, the lifecycle actuation layer executes OCPQ path queries to detect deviations or latency anomalies. If an OCPQ query detects that the time delta between an order placement and its shipping exceeds a compliance threshold, it signals the actuation loop to reroute supply chain resources.

## M&A Claim Verification
OCPQ is the primary auditing tool for M&A due diligence. Slide claims asserting transaction efficiency or compliance are translated into formal OCPQ queries. Diligence teams run these queries against the target's OCEL graphs, and the cryptographic results are registered as verified receipts in the Slide-to-Receipt map, validating claims with zero trust.
