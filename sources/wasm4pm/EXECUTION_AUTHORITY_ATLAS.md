# EXECUTION AUTHORITY ATLAS — wasm4pm

**Agent:** E3 — wasm4pm Execution Authority
**Date:** 2026-05-31
**Status:** RESEARCH PASS COMPLETE

---

## Critical Finding: GAP_001 — Zero wasm4pm-compat Consumption

`grep -r "wasm4pm.compat\|wasm4pm_compat" ~/wasm4pm/ --include="*.toml" --include="*.rs"` returns nothing.

wasm4pm does NOT import wasm4pm-compat. The typed type-law surfaces defined in wasm4pm-compat (`Evidence<T,State,W>`, `Admission`, `Refusal`, `LossReport`, named law reasons) are entirely absent from the execution engine. The graduation bridge in `wasm4pm-compat/src/graduation.rs` (`GraduateToWasm4pm` trait, `GraduationCandidate`) is declared but no wasm4pm crate implements it. This is GAP_001: the covenant is stated but the seam is unconnected.

---

## 1. Crate Inventory

| Crate | Path | Role |
|---|---|---|
| `wasm4pm-types` | `crates/wasm4pm-types/` | Canonical data types — EventLog, OCEL, PetriNet, DFG, DeclareModel, ConformanceResult, ProvenanceChain |
| `wasm4pm-algos` | `crates/wasm4pm-algos/` | All algorithm implementations — discovery, conformance, streaming, truex receipt verification |
| `wasm4pm-cli` | `crates/wasm4pm-cli/` | CLI entry point |
| `wasm4pm-types/dense_kernel` | `crates/wasm4pm-types/src/dense_kernel.rs` | Packed key table, FNV-1a index, DenseIndex |
| `wasm4pm-cognition` | `crates/wasm4pm-cognition/` | (not yet explored) |
| `wasm4pm-macros` | `crates/wasm4pm-macros/` | Proc macros |
| `wasm4pm-utils` | `crates/wasm4pm-utils/` | Shared utilities |
| `pm-core` | `crates/pm-core/` | Zero-cost no_std process-mining types — paper-grounded, ZERO algorithm implementations |
| `ocel-core` | `crates/ocel-core/` | OCEL 2.0 canonical types (flatten, intake, validate) |
| `ocpq` | `crates/ocpq/` | OCPQ runtime — Küsters & van der Aalst 2025 arXiv:2506.11541 |
| `miniml-core` | `crates/miniml-core/` | ML mini-core |
| `prolog8` | `crates/prolog8/` | Prolog reasoning engine |

---

## 2. Algorithm Surface (wasm4pm-algos)

### 2.1 Discovery Algorithms

| Algorithm | File | Input | Output | Coverage |
|---|---|---|---|---|
| DFG Discovery | `dfg.rs` | `&EventLog`, `activity_key: &str` | `Result<DFG>` | Tested |
| Alpha+ Miner | `alpha.rs` | `&EventLog`, `activity_key: &str` | `Result<PetriNet>` | Tested |
| Heuristic Miner | `heuristic.rs` | `&EventLog`, `activity_key: &str` | `Result<DFG>` | Tested |
| Streaming DFG | `streaming.rs` | `&EventLog`, `activity_key: &str` | `Result<DFG>` | Tested |
| Columnar Encoding | `columnar.rs` | `&EventLog`, `activity_key: &str` | `ColumnarEdgeCounts` | Shared helper |

**MISSING:** Inductive Miner, POWL Discovery (despite `Powl8Op` enum in types), Log Skeleton discovery, Object-Centric DFG.

### 2.2 Conformance Algorithms

| Algorithm | File | Input | Output | Coverage |
|---|---|---|---|---|
| Token Replay (DFG) | `conformance.rs` | `&EventLog`, `&DFG`, `activity_key` | `Result<ConformanceResult>` | Tested |
| Prefix Conformance | `prefix_conformance/mod.rs` | `OrderingLaw`, streaming events | `PrefixVerdict` (ALIVE/DEAD/TERMINAL) | Tested |
| TrueX Receipt Verify | `truex/verify.rs` | JSON envelope | `VerificationResult` | Tested |

**MISSING:** Alignment-based conformance (A* shortest path in synchronous product), ETConformance precision (escaping-edges — defined in pm-core but not implemented in algos), Log Skeleton conformance.

### 2.3 Type Models

#### wasm4pm-types types

| Type | File | Paper Grounding |
|---|---|---|
| `EventLog`, `Trace`, `Event` | `event_log.rs` | XES IEEE 1849-2016 |
| `DFG`, `DFGNode`, `DFGEdge` | `models.rs` | van der Aalst 2016 §3 |
| `PetriNet`, `Place`, `Transition`, `Arc` | `models.rs` | Murata 1989 |
| `DeclareModel`, `DeclareConstraint` | `models.rs` | van der Aalst et al. 2009 |
| `OCEL`, `OCELEvent`, `OCELObject` | `ocel.rs` | re-exports from ocel-core |
| `ConformanceResult`, `TokenReplayResult` | `conformance.rs` | Rozinat & van der Aalst |
| `ProvenanceChain` | `provenance.rs` | BLAKE3 hash chain |
| `Powl8Op` | `powl8_op.rs` | POWL operators |
| `ChoiceGraph` | `choice_graph.rs` | Kourani et al. arXiv:2505.07052 |
| `FlatIncidenceMatrix` | `models.rs` | Linear algebra for replay |

#### pm-core types (no_std, ZERO algorithms)

| Type | Paper Grounding |
|---|---|
| `ProcessTree`, `ProcessOperator` | Leemans et al. 2013 ICATPN |
| `AlignmentMove`, `Alignment`, `AlignmentCost` | Adriansyah 2014 PhD TU/e |
| `LogSkeleton` | Verbeek 2021 STTT |
| `EtcPrecisionResult` | Munoz-Gama & Carmona 2010 |
| `OcelEvent`, `OcelObject`, `ObjectCentricEventLog` | OCEL 2.0 standard 2023 |
| `HeuristicsNet` | Weijters & van der Aalst 2003 |
| `PerformanceSpectrum` | Denisov et al. 2018 |

---

## 3. Error Surface (GAP_002)

`wasm4pm_types::Error` is string-typed:

```rust
pub enum Error {
    ValidationError(String),
    ParseError(String),
    ExecutionError(String),
    HashError(String),
    ProvenanceError(String),
    BudgetExceeded(String),
    StateError(String),
    NotFound(String),
    SerializationError(String),
    Unknown(String),
}
```

Every variant carries a `String` payload. There are no named structural laws, no reason types, no typed refusals. This is the direct inverse of wasm4pm-compat's `Refusal<R, W>` where R must be a specific named law.

---

## 4. Provenance Surface

`ProvenanceChain` in `wasm4pm-types/src/provenance.rs` contains BLAKE3 hashes for `input_hash`, `config_hash`, `plan_hash`, `output_hash`, `combined_hash`, `algorithm_id`, `algorithm_version`, `backend_id`, `kernel_version`, `wasm_build_hash`. This is a receipt structure, but it is not generated by algorithms — no discovery or conformance function returns a `ProvenanceChain`. The chain must be assembled manually by callers.

---

## 5. GAP_001 — The Critical Finding

The wasm4pm-compat graduation bridge (`/Users/sac/wasm4pm-compat/src/graduation.rs`) defines `GraduateToWasm4pm` as the seam where compat evidence hands off to the execution engine. The `GraduationReason` variants name the exact capabilities the engine must provide:

- `NeedsDiscovery`
- `NeedsConformanceExecution`
- `NeedsReplay`
- `NeedsReceipts`
- `NeedsBenchmarkGate`
- `NeedsObjectCentricQueryExecution`
- `RebuildingProcessMiningLocally`

None of these are connected to wasm4pm. wasm4pm does not depend on wasm4pm-compat, does not implement `GraduateToWasm4pm`, and does not consume `Evidence<T, State, W>` types. The type-law covenant is entirely one-sided.
