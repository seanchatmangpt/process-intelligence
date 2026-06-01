# Mining Execution Authority Boundary (v30.1.1)

## 1. Architectural Mandate

The Mining Execution Engine within `wasm4pm` operates under a strict Authority Boundary. It is constitutionally incapable of processing raw, unverified data. All inputs must cross the Admissibility Boundary before they can be considered for mining.

## 2. Typestate Enforcement: The `Admitted` Witness

The engine must universally refuse `Evidence<T, Unverified>` or any raw byte streams.
The sole acceptable input type for the execution loop is `Evidence<T, Admitted, W>`, where `W` represents a cryptographic witness of admission.

```rust
// The core typestate required by the mining authority
pub struct Evidence<T, S: EvidenceState, W: Witness> {
    payload: T,
    state: PhantomData<S>,
    witness: W,
}

// Mining function signature enforcing the boundary
pub fn execute_mining_cycle<T, W>(
    evidence: Evidence<T, Admitted, W>,
    context: &ExecutionCtx
) -> Result<MiningOutcome, AuthorityError> 
where W: Witness + Verified {
    // Execution logic
}
```

## 3. Mandatory Emission Artifacts

Upon completion of a mining cycle, the Authority Boundary mandates the deterministic generation of three artifacts:

1.  **Verdicts**: The deterministic result of the execution, typed and validated against the ontology.
2.  **Receipts**: A BLAKE3-hashed cryptographic proof of the execution trace, binding the `Evidence<T, Admitted, W>` to the `Verdict`.
3.  **ReplayBundles**: A complete, self-contained package containing the WASM module, the exact memory state delta, and the admitted evidence, guaranteeing bit-for-bit reproducible replays.

## 4. Boundary Violation Consequences

Any attempt to bypass the Admissibility Boundary (e.g., injecting raw evidence into the execution queue) must result in an immediate, unrecoverable `AuthorityPanic`. This panic must emit an OTel trace categorized as `SECURITY_BOUNDARY_VIOLATION` before process termination.
