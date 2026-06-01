# Admission-Refusal Map

## Boundary Control
This map dictates the state-space where process evidence is either integrated or violently rejected by the system.

### Refusal Signatures
1. **Temporal Anomalies**: Timestamps that violate the local monotonic clock of the WASM runtime.
2. **Type Violations**: Data payloads attempting to bypass strong memory bounds via buffer overflows or type confusion (mitigated by strict WASM linear memory).
3. **Causal Disconnects**: Events referencing non-existent parent `ProcessInstance` IDs.

### Admission Pathways
Only process traces carrying a valid BLAKE3 cryptographic receipt, signed by a Tier-Zero witness, are admitted into the OCEL ontology. The refusal matrix is rigid, operating on a default-deny postulate.
