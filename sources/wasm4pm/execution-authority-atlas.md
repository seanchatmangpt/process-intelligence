# Execution Authority Atlas

This atlas defines the formal mapping of WebAssembly (WASM) FFI boundaries, execution authorities, standards compliance pipelines, and risk-identification policies for the `wasm4pm` process intelligence engine.

## Execution Atlases: FFI and Boundary Mappings

* **Execution Atlas 1: FFI Memory Layout and Arena Allocator**
  - Defines the zero-copy buffer sharing model. The engine manages linear memory in 64KB pages, exposing raw offset pointers to the host runtime to write serialized IEEE XES or OCEL 2.0 binary chunks.
* **Execution Atlas 2: Type Boundary Marshalling and Error Gates**
  - Maps native Rust result types (e.g., `Result<T, E>`) to flat 64-bit integer values returned across the FFI, encoding offset and length, or specialized error codes for panics and validations.
* **Execution Atlas 3: Graph Traversal and Index Mapping**
  - Establishes how object-centric graphs are represented in linear memory using contiguous vectors of structures, avoiding pointer indirection to optimize cache locality inside the WASM sandbox.
* **Execution Atlas 4: Replay Token Registry**
  - Specifies the registry structure tracking produced, consumed, missing, and remaining tokens for concurrent execution paths.
* **Execution Atlas 5: Verification Receipt Signatures**
  - Defines the Ed25519 signature scheme and public key registry embedded within the WASM binary to generate non-forgeable execution receipts.

---

## Core Execution Authority Maps

* **Step: map mining authority**
  - Defines the requirements for process discovery and model synthesis inside the WASM engine.
  - Complete specifications are defined in [mining-authority-map.md](file:///Users/sac/process-intelligence/sources/wasm4pm/mining-authority-map.md).
  - Key executions: Alpha Miner footprints, Heuristics Miner dependency graphs, and Inductive Miner block cuts.

* **Step: map discovery authority**
  - Defines how raw logs are synthesized into process representations (BPMN, Petri Nets, DFGs).
  - Handles the translation of discovered structures into host-consumable serialized layouts.

* **Step: map conformance authority**
  - Defines the rules for replaying trace logs against process models to calculate fitness and alignments.
  - Complete specifications are defined in [conformance-authority-map.md](file:///Users/sac/process-intelligence/sources/wasm4pm/conformance-authority-map.md).
  - Key executions: Token game replay bookkeeping and optimal alignments via A* state space search.

* **Step: map replay authority**
  - Details the step-by-step firing rules of the Petri Net engine during trace simulation, tracking place markings and enabling transitions.

* **Step: map query authority**
  - Defines the execution of object-centric process querying (OCPQ) and temporal graph matching.
  - Complete specifications are defined in [query-authority-map.md](file:///Users/sac/process-intelligence/sources/wasm4pm/query-authority-map.md).
  - Key executions: Event-to-object and object-to-object temporal queries, and slide-to-receipt cryptographic mapping.

* **Step: map optimization authority**
  - Specifies the execution of bottleneck analysis and resource optimization queries. Calculates queuing delays and identifies path delays using temporal event data.

* **Step: map simulation authority**
  - Defines Monte Carlo simulation capabilities. The WASM engine generates synthetic logs by executing stochastic token game walks based on transition probabilities.

* **Step: map visualization preparation authority**
  - Computes coordinates and layout parameters (e.g., node positioning for DFGs and Petri Nets) inside WASM, returning pre-calculated graphics data to prevent rendering latency on the host.

---

## Runtime and Interface Authorities

* **Step: map WASM runtime authority**
  - Defines the runtime isolation environment. The engine runs under strict WASI constraints, restricting file system access, network calls, and arbitrary sys-calls.

* **Step: map CLI authority**
  - Specifies the command-line interface entry points for command execution, allowing direct ingestion of logs and generation of audit receipts.

* **Step: map receipt validation authority**
  - Details how downstream validation engines parse and verify cryptographic query receipts, verifying the Ed25519 signature against the engine's public key.

* **Step: map lifecycle state authority**
  - Enforces the transition states of process assets through the lifecycle: Parsed -> ValidatedSound -> Replayed -> Archived.

---

## Structural and Standards Mappings

* **Step: map object-centric runtime spine**
  - Specifies the core data layout in WASM linear memory representing the multi-perspective relations of OCEL 2.0 without data flattening.

* **Step: map OCEL runtime obligations**
  - Enforces schema constraints for OCEL 2.0 JSON and XML inputs, verifying type safety for object attributes.

* **Step: map XES runtime obligations**
  - Validates lifecycle events and standard extensions (Concept, Time, Org) for IEEE XES inputs.

* **Step: map BPMN runtime obligations**
  - Standardizes the compilation of BPMN gateways (AND/XOR/OR) into equivalent Petri Net models.

* **Step: map Petri runtime obligations**
  - Verifies workflow net structure and coverability properties (liveness, boundedness).

* **Step: map POWL runtime obligations**
  - Manages Partial Order Workflow Language structures, verifying block soundness.

* **Step: map Declare runtime obligations**
  - Evaluates Linear Temporal Logic (LTL) formulas representing Declare rules (e.g., Response, Precedence).

* **Step: map process tree runtime obligations**
  - Validates hierarchical process trees, ensuring sound block structures.

* **Step: map DFG runtime obligations**
  - Calculates transition matrices and frequency counts for directly-follows graphs.

* **Step: map OCPQ runtime obligations**
  - Enforces resource boundaries and step limits on object-centric process queries.

---

## Risk and Gap Identification Policies

* **Step: identify duplicated compat law**
  - Identifies overlapping validation checks between the pre-flight `wasm4pm-compat` type layer and the core `wasm4pm` execution engine, optimizing out redundant parsing.

* **Step: identify missing execution law**
  - Identifies processes that cannot be executed in the current WASM runtime (e.g., non-local OR-joins) and maps fallback handlers.

* **Step: identify raw-laundering risks**
  - Detects paths where raw logs might bypass the `wasm4pm-compat` validation layer, enforcing compile-time type-safety constraints.

* **Step: identify replay gaps**
  - Addresses state-space explosion during A* search by dynamically switching to beam search when trace length or transition counts exceed safe thresholds.

* **Step: identify decommissioning gaps**
  - Establishes rules for model archiving and final receipt generation when decommissioning process assets.

---

## Strategic and Thesis Alignment

* **Step: write execution-authority thesis**
  - Articulates why WASM-based process mining is the ultimate authority for board-level audits. By decoupling the execution engine from volatile Python/Pandas environments, it guarantees reproducibility, security, and cryptographic auditability.

* **Step: audit: wasm4pm research completeness**
  - Verification check confirming that all maps (mining, conformance, query) are fully defined and linked, and all stubs have been eliminated.

* **Step: checkpoint: wasm4pm research pass complete**
  - Final verification that the WASM execution authority atlas is structurally complete.