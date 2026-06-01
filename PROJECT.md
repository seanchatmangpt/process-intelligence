# Project: Process Intelligence Research Foundry

## Architecture
The Process Intelligence Research Foundry acts as the authority layer for studying process-evidence type law and executing process reality mapping under the v30.1.1 ultimate standard.
- The foundry consists of a type-law engine defining `Evidence<T, State, Witness>`, information lattices, typestate transitions, and cryptographic witnesses.
- It includes a deterministic Petri Net execution engine, a structural soundness and 1-boundedness reachability/coverability solver.
- It implements Declare LTL verification checks.
- Sandbox memory boundaries are secured via FFI boundary safety checks, gas metering, recursion guards, and ChaCha20 memory sanitization.
- Chronological event logs are chained as an event ledger using SHA-256 blocks.
- PowerPoint/JSON receipts project verified board claims.

## Milestones
| # | Name | Scope | Status |
|---|------|-------|--------|
| 1 | Milestone 1: Exploration & Gap Analysis | Gap analysis, environment preparation, initial structural design | DONE |
| 2 | Milestone 2: Type-Law and Execution Core Implementation | Evidence lattice, typestate transitions, Petri Net solver, FFI safety checks | IN_PROGRESS |
| 3 | Milestone 3: Board-Admissible M&A Projections and Autonomic Actuation | Alignment solvers, autonomic actuation policies, projection mapping | PLANNED |
| 4 | Milestone 4: Test Fixtures, Integration Tests & Threat Simulation | Edge cases, differential testing, simulated threat vectors | PLANNED |
| 5 | Milestone 5: Verification & Forensic Audit | End-to-end certification, audit trail generation, code freeze | PLANNED |

## Interface Contracts
### Evidence ↔ Execution Authority
- `Evidence<T, State, Witness>` must bind event payload, execution state, and cryptographic witness.
- `Witness` join/meet operations verify monotonic state transitions.
- Typestates generic over `Parsed`, `ValidatedSound`, and `Replayed` enforce strict lifecycle invariants.

### Petri Net ↔ Execution Core
- The Petri Net token game provides transition firing matrices.
- Structural soundness verifies unique source and sink places.
- Reachability and coverability solvers construct graphs to check for deadlocks, proper completion, and 1-boundedness.

### FFI Boundary Safety
- FFI memory checks enforce safety bounds on raw pointers, preventing overflows, alignment mismatch, overlap, and unauthorized global memory access.

## Code Layout
- `sources/wasm4pm/src/evidence.rs` - Type-Law structures, lattices, signatures, and typestate transitions.
- `sources/wasm4pm/src/petri.rs` - Petri Net definition, token game, coverability/reachability graph, structural soundness solver.
- `sources/wasm4pm/src/safety.rs` - FFI memory sandboxing boundary safety checks and pointer validation.
- `sources/wasm4pm/src/ffi.rs` - Foreign function interfaces and integration with safety checkers.
- `sources/wasm4pm/src/lib.rs` - Library registration and exports.

