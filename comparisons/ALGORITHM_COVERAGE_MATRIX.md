# Algorithm Coverage Matrix: PM4Py vs wasm4pm vs wasm4pm-compat

## Legend

- **Full** — algorithm fully implemented and runnable
- **Shape** — structural types only (no execution); graduation to engine required
- **Partial** — algorithm present but incomplete or limited
- **No** — not present

## Coverage Matrix

| Algorithm | PM4Py | wasm4pm | wasm4pm-compat | Gap |
|---|:---:|:---:|:---:|---|
| **Alpha Miner** | Full | Full (alpha.rs) | Shape (petri.rs) | wasm4pm-compat carries PetriNet shape; discovery stays in wasm4pm |
| **Inductive Miner** | Full | No | Shape (process_tree.rs, powl.rs) | wasm4pm has no Inductive Miner; PM4Py lead |
| **Heuristics Miner** | Full | Full (heuristic.rs, returns DFG) | Shape (dfg.rs) | wasm4pm-compat DFG shape travels to wasm4pm for heuristic pass |
| **POWL Discovery** | Full (pm4py 2.7+) | No | Shape (powl.rs) | POWL discovery algorithm absent from wasm4pm; PM4Py only |
| **Token Replay** | Full | Full (conformance.rs, DFG-based) | Shape (conformance.rs) | wasm4pm uses DFG not full Petri net for token replay |
| **Alignment Conformance** | Full (A* sync product) | No | Shape (conformance.rs SyncMove etc.) | Alignment engine absent from wasm4pm; PM4Py only |
| **Declare Checking** | Full (LTL, automaton) | No | Shape (declare.rs) | Declare evaluation absent from wasm4pm; PM4Py only |
| **Log Skeleton** | Full | No | No | Only PM4Py has log skeleton mining |
| **OC-DFG** | Full (ocel_dfg) | No | Shape (dfg.rs, ocel.rs) | wasm4pm has no OC-DFG algorithm; compat has structure |
| **OCPQ** | No | No | Shape (ocpq.rs) | OCPQ is a wasm4pm-compat invention; no engine anywhere yet |
| **Variant Analysis** | Full | Partial (streaming.rs) | Shape (eventlog.rs) | wasm4pm has streaming variant tracking; no full variant miner |

## Notes by Algorithm

### Alpha Miner
- PM4Py: full Alpha+ with implicit place handling.
- wasm4pm: `discover_alpha` in `alpha.rs` computes directly-follows relations,
  causality, and parallel pairs, returns `PetriNet`. Single-pass, O(n + m²).
- wasm4pm-compat: `src/petri.rs` carries `WfNetConst<SOUNDNESS>` — the shape
  of a WF-net with a non-forgeable soundness witness. Not a miner.

### Inductive Miner
- PM4Py: multiple variants (IM, IMf, IMd). Returns a process tree directly.
- wasm4pm: absent. The DFG discovery (`dfg.rs`) is a precursor, but the
  inductive split logic is not implemented.
- wasm4pm-compat: `process_tree.rs` carries `TypedLoopNode<ARITY>` and
  `ProcessTree` shapes. Ready to receive Inductive Miner output but no
  discovery algorithm exists in this stack.

### Heuristics Miner
- PM4Py: dependency graph, loop detection, long-distance dependencies.
- wasm4pm: `discover_heuristic` returns a `DFG`. Single-pass, O(n + m).
  No loop detection or long-distance dependency handling.
- wasm4pm-compat: `dfg.rs` carries `Dfg`, `DfgEdge`, `DfgWeight` shapes.

### POWL Discovery
- PM4Py 2.7+: full POWL discovery (SHS-Miner). Returns partially ordered
  workflow structures that exceed block-structured process trees.
- wasm4pm: no POWL discovery algorithm.
- wasm4pm-compat: `powl.rs` is the richest POWL shape vocabulary in the stack —
  `PowlNode`, `OrderEdge`, `Powl`, `TreeProjectable`, `ExceedsProcessTree`
  witness markers. Structure awaiting an engine.

### Token Replay
- PM4Py: full Petri net replay against `(net, im, fm)` triple.
- wasm4pm: DFG-based replay only. Fitness formula: Rozinat & van der Aalst.
  No Petri net replay.
- wasm4pm-compat: `ConformanceVerdict`, `Deviation`, move markers. No replay.

### Alignment Conformance
- PM4Py: A* optimal alignment over synchronous product net. Costs configurable.
- wasm4pm: absent.
- wasm4pm-compat: `SyncMove`, `LogOnlyMove`, `ModelOnlyMove` — the three
  move types in an alignment are modeled as first-class types. No A*.

### Declare Checking
- PM4Py: LTL-based checker, automaton-based checker.
- wasm4pm: absent.
- wasm4pm-compat: `declare.rs` has `DeclareTemplate`, `DeclareConstraint`,
  `DeclareScope` with OC-Declare extension. No evaluator.

### OCPQ
- PM4Py: no OCPQ.
- wasm4pm: no OCPQ.
- wasm4pm-compat: `ocpq.rs` is original — `ObjectScope`, `Predicate` tree,
  `OcpqQuery`. No evaluation engine exists anywhere in the stack.

## Summary of Coverage Gaps

1. **Inductive Miner** — critical for block-structured WF-net / process tree
   output. Absent from wasm4pm. wasm4pm-compat ready to receive.
2. **Alignment conformance** — PM4Py exclusive. Optimal alignment requires A*
   over sync product. wasm4pm-compat has the shape, no engine.
3. **POWL discovery** — PM4Py 2.7+ exclusive. wasm4pm-compat has the richest
   POWL shape vocabulary in the stack.
4. **Declare checking** — PM4Py exclusive. wasm4pm-compat has the constraint
   shape vocabulary.
5. **OC-DFG** — wasm4pm-compat has the component shapes. No wasm4pm algorithm.
6. **OCPQ evaluation** — entirely novel to this stack. No engine anywhere.
