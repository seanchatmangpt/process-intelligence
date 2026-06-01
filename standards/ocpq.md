# OCPQ v30.1.1: Multi-Object Path Query Semantics

## Overview
Object-Centric Process Query (OCPQ) is the unified query interface for N-dimensional event logs. In the v30.1.1 protocol, OCPQ provides the query language to audit complex, multi-object transaction flows without flattening them into sequential logs, preserving multi-perspective topology.

## Lifecycle Actuation Mapping
OCPQ powers the **Autonomic Query Gate**. During runtime, the lifecycle actuation layer executes OCPQ path queries to detect deviations or latency anomalies. If an OCPQ query detects that the time delta between an order placement and its shipping exceeds a compliance threshold, it signals the actuation loop to reroute supply chain resources.

## M&A Claim Verification
OCPQ is the primary auditing tool for M&A due diligence. Slide claims asserting transaction efficiency or compliance are translated into formal OCPQ queries. Diligence teams run these queries against the target's OCEL graphs, and the cryptographic results are registered as verified receipts in the Slide-to-Receipt map, validating claims with zero trust.
