# GRADUATION BOUNDARY MAP — wasm4pm-compat

**Source:** /Users/sac/wasm4pm-compat/src/graduation.rs
**Covenant:** "Compat carries the evidence. wasm4pm adjudicates it."

---

## The Graduation Surface

`graduation.rs` is the **only** public bridge between the structure-only compat layer and the wasm4pm execution engine. It implements nothing of wasm4pm — it lets a compat value declare itself a graduation candidate and name why it must leave.

The module has three public items:
1. `GraduationReason` enum — the named trigger signs
2. `GraduationCandidate` struct — the typed, reviewable case
3. `GraduateToWasm4pm` trait — the seam a host implements

---

## GraduationReason Variants

`#[non_exhaustive]` — future trigger signs can be added without breaking existing match arms.

### `NeedsDiscovery`
- **Tag:** `needs_discovery`
- **Hard signal:** YES
- **What it names:** A process model must be *discovered* from a log — an algorithmic job (Inductive Miner, Alpha Miner, Heuristics Miner, etc.).
- **What wasm4pm must provide:** A discovery engine that takes an event log and returns a process model (Petri net, process tree, POWL, DFG, etc.).
- **Why compat cannot do it:** Discovery requires log traversal, causal inference, and algorithm execution. Structure-only cannot run an algorithm.
- **Typical trigger:** Host has an admitted OCEL/XES log and needs a model for it.

### `NeedsConformanceExecution`
- **Tag:** `needs_conformance_execution`
- **Hard signal:** YES
- **What it names:** A conformance result must be *computed* (token replay, alignment computation), not merely claimed.
- **What wasm4pm must provide:** A conformance checking engine producing `ConformanceVerdict` shapes (Fitness, Precision, Deviations) by running token replay or alignment algorithms.
- **Why compat cannot do it:** `conformance.rs` only carries verdict *shapes* — bounded newtypes for scores, deviation containers. It never derives these values.
- **Typical trigger:** Host has an admitted log + an admitted model and needs to know fitness/precision.

### `NeedsReplay`
- **Tag:** `needs_replay`
- **Hard signal:** YES
- **What it names:** A log must be *replayed* against a model — the token-game execution.
- **What wasm4pm must provide:** A token-replay engine that fires transitions, tracks markings, and records moves.
- **Why compat cannot do it:** `petri.rs` carries net shapes and soundness claims; it never fires tokens. `WfNet::validate()` checks structural shape only.
- **Typical trigger:** Host needs a trace-level replay to identify deviating paths.

### `NeedsReceipts`
- **Tag:** `needs_receipts`
- **Hard signal:** NO (soft signal)
- **What it names:** Provenance receipts must be *minted and chained*, not merely shaped.
- **What wasm4pm must provide:** A receipt minting and chaining engine (Blake3 or equivalent hash chain) that produces `Receipted` evidence with verifiable provenance.
- **Why compat cannot do it:** `receipt.rs` carries receipt shapes. Minting (hashing, chaining) is an engine function.
- **Typical trigger:** Host needs tamper-evident evidence chains for audit or compliance purposes.

### `NeedsBenchmarkGate`
- **Tag:** `needs_benchmark_gate`
- **Hard signal:** NO (soft signal)
- **What it names:** A benchmark gate must be *run* to admit a result — performance or quality thresholds must be verified against real execution.
- **What wasm4pm must provide:** A benchmark harness that runs a target algorithm/pipeline and gates on measured performance (time, memory, quality metrics).
- **Why compat cannot do it:** Quality metric shapes (`Metric<KIND, NUM, DEN>`) can carry claimed scores but cannot verify them against actual runs.
- **Typical trigger:** Host claims a fitness of 0.95 and needs the engine to verify it.

### `NeedsObjectCentricQueryExecution`
- **Tag:** `needs_object_centric_query_execution`
- **Hard signal:** YES
- **What it names:** An OCPQ query must be *executed* against an OC-PM model, not merely declared.
- **What wasm4pm must provide:** An OCPQ query execution engine that evaluates queries against an object-centric log or model.
- **Why compat cannot do it:** `ocpq.rs` carries query shapes (`OcpqQuery`, `OcpqPredicate`). No evaluation engine exists in compat.
- **Typical trigger:** Host has a declared OCPQ query and an admitted OCEL log and needs query results.

### `RebuildingProcessMiningLocally`
- **Tag:** `rebuilding_process_mining_locally`
- **Hard signal:** YES (the strongest signal)
- **What it names:** The host has started reimplementing process mining functionality locally — the strongest sign it should adopt the engine instead.
- **What wasm4pm must provide:** Everything: discovery, conformance, replay, query execution, receipt minting.
- **Why compat cannot do it:** This is the diagnostic that the host has gone past the compat mandate entirely.
- **Typical trigger:** Host has written local token-replay logic, a local fitness calculator, or a local DFG miner rather than graduating.

---

## Hard Signal vs. Soft Signal

```
is_hard_signal() == true:
  NeedsDiscovery
  NeedsConformanceExecution
  NeedsReplay
  NeedsObjectCentricQueryExecution
  RebuildingProcessMiningLocally

is_hard_signal() == false:
  NeedsReceipts
  NeedsBenchmarkGate
```

Hard signals mean the host is already executing or re-implementing process mining — it is past the compat layer's mandate. Soft signals mean the host has reached an edge case that wasm4pm should handle but could conceivably defer.

---

## GraduationCandidate

```rust
pub struct GraduationCandidate {
    pub reason: GraduationReason,
    pub subject: String,      // "p2p OCEL log", "discovered Petri net"
    pub evidence_ref: String, // opaque reference, e.g. "blake3:deadbeef"
}
```

`is_grounded()` returns true when both `evidence_ref` and `subject` are non-empty. The engine intake should reject ungrounded candidates — a candidate without a reference is not reviewable.

---

## GraduateToWasm4pm Trait

```rust
pub trait GraduateToWasm4pm {
    fn candidate(&self) -> GraduationCandidate;
}
```

This is the seam a host (or the engine itself) implements. The trait:
- Has no dependency on wasm4pm (no import of the engine crate)
- Produces a candidate, never crosses the boundary
- Is structure-only: implementing it makes the boundary explicit; it does not cross it

---

## Connection to the Research Program

This surface is the direct connection between:
1. The wasm4pm-compat research program (paper-derived type law surfaces)
2. The future wasm4pm refactor (the execution engine)

Every `GraduationReason` variant names a capability the research program has identified as belonging to the engine layer, not the compat layer. The full variant set defines the engine's minimum required surface:

| Engine capability required | Graduation reason |
|---|---|
| Process discovery algorithms | `NeedsDiscovery` |
| Conformance checking (replay/alignment) | `NeedsConformanceExecution` |
| Token game replay | `NeedsReplay` |
| Provenance chain minting | `NeedsReceipts` |
| Performance/quality benchmark gates | `NeedsBenchmarkGate` |
| OCPQ query execution | `NeedsObjectCentricQueryExecution` |
| Full process mining suite | `RebuildingProcessMiningLocally` |

The graduation boundary is the precise interface specification between wasm4pm-compat (structural evidence law) and wasm4pm (process mining execution engine).
