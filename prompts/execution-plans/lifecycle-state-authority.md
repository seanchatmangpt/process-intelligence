# Lifecycle State Authority Execution Blueprint (v30.1.1)

## 1. Context & Objective
This execution blueprint details the exact mechanism for enforcing the Lifecycle State Authority (LSA) within the `wasm4pm` execution engine. The LSA provides hard boundary enforcement, ensuring queries and simulations align with the ontological model's current state.

## 2. Permitted Lifecycle States

### 2.1. DESIGN (`state::Design`)
*   **Context**: Iterative modeling phase using `ggen`.
*   **Allowed**: Structural validation and topology queries (`ModelQuery::Topology`).
*   **Blocked**: Temporal and deterministic simulations.
*   **Enforcement**: Engine yields `LifecycleViolation::SimulationBlockedInDesign`.

### 2.2. OPERATION (`state::Operation`)
*   **Context**: Deployed, stable runtime.
*   **Allowed**: OTel telemetry trace emission, deterministic process queries, and unconstrained simulation.
*   **Blocked**: Governor-level structural mutation.
*   **Enforcement**: All alterations yield `LifecycleViolation::GovernorInterventionBlocked`.

### 2.3. DECOMMISSION (`state::Decommission`)
*   **Context**: Archived ontological ledger.
*   **Allowed**: Auditor validation, BLAKE3 receipt verification, read-only historical queries (`ModelQuery::Historical`).
*   **Blocked**: All mutations, temporal simulations.
*   **Enforcement**: Yields `LifecycleViolation::ArchivedModelImmutable`.

## 3. Execution Pipeline Integration
The execution engine employs the LSA verifier as an immutable middleware guard before granting Wasm linear memory access.

```rust
pub fn enforce_lifecycle_state(
    operation: &OperationRequest,
    lsa_context: &LifecycleAuthority,
) -> Result<(), LifecycleError> {
    let current_state = lsa_context.resolve_current_state()?;
    
    match (current_state, operation.kind()) {
        (LifecycleState::Design, OperationKind::Simulate) => {
            Err(LifecycleError::SimulationBlockedInDesign)
        }
        (LifecycleState::Operation, OperationKind::MutateStructure) => {
            Err(LifecycleError::GovernorInterventionBlocked)
        }
        (LifecycleState::Decommission, OperationKind::Mutate | OperationKind::Simulate) => {
            Err(LifecycleError::ArchivedModelImmutable)
        }
        _ => Ok(()), // Cleared for execution
    }
}
```

## 4. Diagnostics & Traceability (OTel/BLAKE3)
Violations of the LSA emit standardized OTel spans. In the `Operation` and `Decommission` states, all authorized actions must yield cryptographic BLAKE3 receipts to ensure unforgeable evidence of compliance across transitions.
