# Lifecycle: Define Activation-State Process Intelligence

The **Activation State** governs the transition of a process model from a validated simulation to a live, system-enforced execution state.

## Autonomic MAPE-K Mapping
* **Loop Role**: **Execute** & **Plan**
* **Responsibility**: In the Plan phase, the execution hooks and data bindings are mapped. In the Execute phase, the process engine activates listeners, mounts message queues, and enforces the model structure on real transactions.
* **Actuation Trigger**: Transitioning to Activation requires passing the **ALIVE Checkpoint** and **Gate 2: Behavioral Bounds**.

---

## The Activation Protocol

To activate a process model, the engine executes three primary steps:

### 1. Bytecode Compilation (WASM compilation)
To ensure high performance and strict type safety, the validated Petri Net or POWL model is compiled into a WebAssembly (WASM) kernel using the `wasm4pm` execution core. This compiler generates:
* **Transition Firing Table**: A lookup table mapping transition enablement rules.
* **Token State Vectors**: In-memory vectors representing current markings.
* **Callback Bindings**: Outbound hooks triggered when transitions fire.

### 2. Live System Bindings (Message Queues)
Transitions are bound to enterprise middleware endpoints:
* **Trigger Bindings**: Connect transitions to incoming event streams (e.g., Apache Kafka topics, RabbitMQ queues, HTTP webhooks).
* **Execution Guard Checks**: Before admitting an incoming event, the WASM kernel checks if the matching transition is enabled. If enabled, it fires the transition and updates the token vector. If disabled, it routes the message to the **Repair Stage** queue.

### 3. State Vector Initialization
The runtime registers the initial marking vector $M_{init} = [i]$. A cryptographic activation receipt is written to the process ledger, signing the deployed WASM hash and binding config.

---

## M&A Diligence Claims
In M&A, the Activation State represents the **Go-Live Validation** of integration synergies.
* **Buyer Reliance**: The buyer relies on activation records to verify that the newly designed, synergy-yielding processes are actually running in production and are not merely "shelfware" slides.
* **Slide-to-Receipt Map**: PowerPoint assertions claiming "The unified procurement workflow was activated across both entities" must link to the Activation State receipt showing the signed WASM kernel hash, the Kafka topic mappings, and the initialization timestamp.

---

## Related Documents
* See the [Construction State](file:///Users/sac/process-intelligence/lifecycle/define_construction-state_process_intelligence.md) for Petri Net compilation.
* See the [Operation State](file:///Users/sac/process-intelligence/lifecycle/define_operation-state_process_intelligence.md) for run-time streaming.
* Back to [Lifecycle README](file:///Users/sac/process-intelligence/lifecycle/docs-law__lifecycle_readme.md).