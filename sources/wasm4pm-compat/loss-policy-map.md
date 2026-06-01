# Loss Policy Map

## Acceptable Degradation in Process Traces
In adversarial or high-latency environments, perfect trace capture is physically impossible. The Loss Policy Map defines the thermodynamic limits of process evidence.

### Permissible Loss
- **Metadata Attrition**: Non-critical process attributes may be pruned if the WASM memory approaches 90% saturation.
- **Trace Decimation**: In flood-state events, rapid repetitive state transitions can be probabilistically sampled rather than fully logged.

### Absolute Unacceptable Loss (Terminal States)
- Loss of the causal link between `START` and `END` states.
- Corruption of the cryptographic signature of the event payload.
If an absolute loss occurs, the `wasm4pm-compat` layer must initiate a self-halt, refusing to emit corrupted OCEL data.
