# Witness Lattice Architecture

## The Observer's Grid
The witness lattice defines the cryptographic observation points within the `wasm4pm` execution context.

### Node Distribution
- **Zero-Tier Witnesses**: Embedded directly into the WASM runtime hooks. They observe without modifying the stack.
- **First-Tier Witnesses**: The host environment interceptors. They validate the OCEL schema format against the typestate transition.
- **Adversarial Interceptors**: Deployed to inject chaos testing vectors, ensuring the lattice does not collapse under AGI-driven fuzzing.

### Lattice Entanglement
When an event $E$ occurs, witnesses $[W_1, W_2, ... W_n]$ must reach a BFT consensus on the process trace before it is committed to the immutable OCEL log. Gaps in the lattice indicate potential forgeability zones.
