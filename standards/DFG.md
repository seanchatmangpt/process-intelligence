# DFG — Directly-Follows Graph

**Authority:** van der Aalst 2011 (Process Mining book); DFG-based discovery (Leemans 2018)
**Witness key:** `dfg` — `WitnessFamily::Paper`

---

## Overview

A **Directly-Follows Graph (DFG)** is the simplest process model derived from an event
log. It is a weighted directed graph where:

- **Nodes** are activities (distinct event types in the log).
- **Edges** record that activity `B` directly followed activity `A` at least once in the
  log, with a **weight** equal to the frequency of that directly-follows relation.

DFGs are the foundation of the simplest family of process discovery algorithms and the
basis for filtering, conformance screening, and performance overlays.

---

## Formal Definition

Given a trace `σ = ⟨e₁, e₂, ..., eₙ⟩`, activity `b` directly follows activity `a` in
`σ` if there exists `i` such that `act(eᵢ) = a` and `act(eᵢ₊₁) = b`. The DFG aggregates
these relations across all traces, counting occurrences.

A DFG G = (A, E, w) where:
- **A** — a finite set of activity nodes.
- **E ⊆ A × A** — a set of directed edges.
- **w: E → ℕ** — a weight function giving directly-follows frequency.

Start and end activity frequencies (entry/exit counts) may be recorded separately.

---

## DFG vs. More Expressive Models

| Property | DFG | Process Tree | POWL | Petri net |
|---|---|---|---|---|
| Concurrency detection | No | Yes | Yes | Yes |
| Loop detection | No | Yes | Yes | Yes |
| Sound by construction | Not guaranteed | Yes (IM output) | Partial | Requires analysis |
| Discovery complexity | O(n) trace scan | O(n log n) | O(n²) | Polynomial/EXPSPACE |
| Use case | Quick exploration, performance overlay | Discovery with guarantees | Partial-order discovery | Formal analysis |

DFGs are the fastest to compute and the most common first step in exploratory process
mining. Their limitation is **underfitting**: a DFG cannot distinguish concurrency from
non-determinism, and it may permit traces that never appeared in the log.

---

## wasm4pm-compat Implementation (dfg.rs)

`src/dfg.rs` models the DFG **structural shape** — the graph value, not the discovery
algorithm:

- `DfgActivityId` — a `&'static str` newtype for compile-time activity naming.
- `DfgNode` — an activity node with a `String` name and optional entry/exit frequency.
- `DfgEdge` — a directed edge from source node ID to target node ID with a `DfgWeight`.
- `DfgWeight` — a non-negative frequency count (u64).
- `Dfg` — the container: `Vec<DfgNode>` and `Vec<DfgEdge>`.
- `Dfg::validate()` — checks graph shape: edges reference declared nodes, weights are
  non-negative, graph is non-empty.
- `DfgRefusal::DiscoveryRequired` — the named law refusing to treat an empty DFG as if
  it had been discovered. A DFG must be populated before it can be validated.

### The DiscoveryRequired Boundary

Asking a DFG to behave as if it had been discovered when it is empty is refused as
`DfgRefusal::DiscoveryRequired`. This boundary makes it unmistakable that DFG
**discovery** (computing edges from a log) is an engine responsibility — not a structural
operation that belongs in this crate.

---

## What wasm4pm Must Provide

| Capability | Graduates to |
|---|---|
| DFG discovery from EventLog or OcelLog | `wasm4pm` |
| OC-DFG discovery (per object type) | `wasm4pm` |
| DFG filtering (by frequency threshold, performance metric) | `wasm4pm` |
| DFG-based conformance screening | `wasm4pm` |
| DFG performance overlay (duration, waiting time) | `wasm4pm` |
| DFG → process tree / WF-net conversion | `wasm4pm` |

---

## Board Placement

The DFG is the universal entry point for process exploration: every process mining
platform (Celonis, Disco, pm4py) shows a DFG as the first visualization. By defining the
DFG structural shape in wasm4pm-compat with a named `DiscoveryRequired` boundary, the
compat layer makes explicit that DFG values only travel from the `wasm4pm` discovery
engine forward — never fabricated in compatibility glue code.
