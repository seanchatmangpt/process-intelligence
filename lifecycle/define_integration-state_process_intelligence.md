# Lifecycle: Define Integration-State Process Intelligence

The **Integration State** governs the merging, alignment, and joint validation of newly acquired or designed processes into the broader corporate process ecosystem.

## Autonomic MAPE-K Mapping
* **Loop Role**: **Plan** & **Knowledge**
* **Responsibility**: In the Plan phase, cross-process interfaces, shared object schemas, and synchronization points are mapped. In the Knowledge phase, the combined global process ontology is verified for joint soundness.
* **Actuation Trigger**: Initiated during post-merger integration or when introducing new auxiliary workflows to an active core process.

---

## Inter-Process Integration & Joint Soundness

When merging two process models $N_1 = (P_1, T_1, F_1)$ and $N_2 = (P_2, T_2, F_2)$, the resulting combined net $N_{joint}$ is not guaranteed to be sound, even if $N_1$ and $N_2$ are sound individually. Integration requires rigorous joint soundness checking.

### 1. Synchronization Points
Processes interface through three main methods:
* **Place-based Merging**: A place $p \in P_1$ is merged with a place $p' \in P_2$ (e.g., a shared queue place).
* **Transition-based Merging (Synchronous Handshake)**: A transition $t \in T_1$ is merged with $t' \in T_2$, requiring both processes to fire the event simultaneously.
* **Message-based Arcs**: Directed arcs connect transitions in $N_1$ to places in $N_2$ (representing asynchronous notifications).

### 2. Joint Soundness Verification
The integration engine constructs the combined Petri Net $N_{joint} = (P_1 \cup P_2, T_1 \cup T_2, F_{joint})$ and checks for:
* **Structural Deadlocks**: Check if synchronizations restrict token flow such that the sink place of either process becomes unreachable.
* **Boundedness Violations**: Ensure that asynchronous messages do not accumulate indefinitely, violating 1-boundedness.
* **Verification Protocol**: Construct the reachability graph of $N_{joint}$ with the joint initial marking $i_{joint} = [i_1, i_2]$. If $[o_1, o_2]$ is reachable and there are no dead markings, joint soundness is certified.

---

## Standards Alignment

* **OCEL 2.0 Shared Objects**: Integration leverages OCEL 2.0 to define shared object types (e.g., a single `Customer` object interacting with both the Billing process and the Support process).
* **Enterprise BPMN Collaboration**: BPMN Collaboration diagrams (with pools and message flows) are compiled into the joint Petri Net representation to run the integration-state conformance checks.

---

## M&A Diligence Claims
In M&A, the Integration State represents the **Post-Merger Synergy Plan**.
* **Buyer Reliance**: The buyer relies on this mapping to confirm that the target's IT systems can integrate with the buyer's without causing cascading transaction failures or deadlock delays.
* **Slide-to-Receipt Map**: Slides claiming "Integration of procurement systems will be completed within 90 days with zero operational disruption" must resolve to an Integration State receipt containing the joint soundness proof of the merged billing and procurement Petri Net.

---

## Related Documents
* See the [Acquisition State](file:///Users/sac/process-intelligence/lifecycle/define_acquisition-state_process_intelligence.md) for pre-integration details.
* See the [Operation State](file:///Users/sac/process-intelligence/lifecycle/define_operation-state_process_intelligence.md) for live running models.
* Back to [Lifecycle README](file:///Users/sac/process-intelligence/lifecycle/docs-law__lifecycle_readme.md).