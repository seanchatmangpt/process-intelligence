# Project: Process Intelligence Research Foundry

## Architecture
The Process Intelligence Research Foundry acts as the authority layer for studying process-evidence type law and executing process reality mapping under the v30.1.1 ultimate standard.
- The foundry consists of a type-law engine defining `Evidence<T, State, Witness>`, information lattices, and Ed25519 signature checks.
- It includes a deterministic Petri Net execution engine and an A* alignment solver.
- It implements Declare LTL verification checks.
- Sandbox memory boundaries are secured via gas metering, recursion guards, and ChaCha20 memory sanitization.
- Chronological event logs are chained as an event ledger using SHA-256 blocks.
- PowerPoint/JSON receipts project verified board claims.

## Milestones
| # | Name | Scope | Dependencies | Status |
|---|------|-------|-------------|--------|
| 1 | M1: Type-Law Engine | Implement Evidence, Information Lattices, Ed25519, autonomic lifecycle state machine | none | PLANNED |
| 2 | M2: Petri & LTL Engine | Petri Net token-game replay, P-invariants, soundness checks, Declare LTL checks | M1 | PLANNED |
| 3 | M3: A* Solver & Sandbox | A* alignment solver, sandbox (gas, memory, ChaCha20), ledger hashing | M2 | PLANNED |
| 4 | M4: ggen Projection | PowerPoint/JSON receipt projection, code generators | M3 | PLANNED |
| 5 | M5: E2E Integration | Verify 100% passing tests and run Forensic Auditor | M4 | PLANNED |

## Interface Contracts
### Evidence ↔ Execution Authority
- `Evidence<T, State, Witness>` must bind event payload, execution state, and cryptographic witness.
- `Witness` join/meet operations verify monotonic state transitions.
- Authority signatures must be Ed25519 compliant.

### Petri Net ↔ A* Solver
- The Petri Net token game provides transition firing matrices.
- The A* solver uses transition firing rules to search for optimal alignments minimizing synchronous/model-only/log-only move costs.

## Code Layout
- `sources/wasm4pm/src/evidence.rs` - Type-Law structures, lattices, signatures, and lifecycle.
- `sources/wasm4pm/src/petri.rs` - Petri Net definition, token game, P-invariants, soundness checks.
- `sources/wasm4pm/src/ltl.rs` - Declare LTL parser and verification checks.
- `sources/wasm4pm/src/alignment.rs` - A* alignment solver.
- `sources/wasm4pm/src/ledger.rs` - Event ledger block chaining.
- `sources/wasm4pm/src/sandbox.rs` - Sandbox constraints (gas, recursion, ChaCha20 memory shredding).
