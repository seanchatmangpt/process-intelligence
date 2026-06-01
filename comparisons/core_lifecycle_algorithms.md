# Core Lifecycle Algorithms

This document provides a formal, mathematically rigorous specification of the four core lifecycle algorithms defined by the process intelligence research program: **Paper-to-Law**, **PM4Py Oracle Mapping**, **Admissible Process Evidence (Admission Gate)**, and **Receipt-Bearing Execution**.

These algorithms govern the compilation, verification, admission, and cryptographic auditing of process models and event telemetry.

---

## 1. Algorithm 1: Paper-to-Law Mapping

### 1.1 Overview
The **Paper-to-Law Mapping** protocol translates academic operational invariants and structural boundaries defined in process mining literature into static, zero-cost compile-time type-safety guarantees and witness markers in the Rust typestate system. These compile-time restrictions are validated using compiler-enforced constraints and trybuild compile-fail UI tests.

### 1.2 Mathematical and Logical Formulation
Let $P$ be a process mining paper or specification. Let $I_P = \{ \lambda_1, \lambda_2, \dots, \lambda_n \}$ be the set of formal structural invariants defined in $P$. The goal is to construct a compile-time type mapping:
$$\mathcal{M}: I_P \to (\mathcal{T}_{\text{Rust}}, \mathcal{W})$$
where:
*   $\mathcal{T}_{\text{Rust}}$ is a set of type definitions and traits in the Rust type system.
*   $\mathcal{W}$ is a set of witness markers implementing the `Witness` trait.

For any structural invariant $\lambda_i \in I_P$, we define a compilation test function:
$$\text{Compile}(\text{Program}) \in \{ \text{Success}, \text{Failure} \}$$
The type-system mapping $\mathcal{M}$ is correct if and only if for any program violating $\lambda_i$:
$$\text{Compile}(\text{Program}_{\text{violating } \lambda_i}) \equiv \text{Failure}$$

### 1.3 Inputs and Outputs
*   **Inputs:**
    *   Academic specification $P$ and its associated structural invariants $I_P$.
    *   Test harness configuration and compilation parameters.
*   **Outputs:**
    *   A witness marker $W \in \mathcal{W}$ implementing the `Witness` trait, declared using the witness system.
    *   A set of compile-fail UI test fixtures (`.rs` files) designed to trigger compiler errors upon invariant violations.
    *   A set of compiler diagnostic snapshot logs (`.stderr` files) confirming type-checking rejection.

### 1.4 Preconditions
*   The structural invariants $I_P$ must be statically verifiable at compile time (e.g., arity constraints, bipartite graph structures, or state machine transitions).
*   Any invariants requiring dynamic runtime graph traversal (e.g., sound marking reachability in large Petri nets) must be flagged and deferred to runtime checkers using the `COVERED_BY_GRADUATION_BOUNDARY` pattern.

### 1.5 Step-by-Step Logic Flow
1.  **Analyze & Filter Invariants:** Scan the paper $P$ to isolate all structural and behavioral invariants $I_P$. Classify each invariant $\lambda_i$ as either statically expressible or runtime-dependent.
2.  **Declare Witness Marker:** Define the unique witness marker using the witness macro system:
    ```rust
    witness_marker!(PaperWitness, "paper_key", WitnessFamily::Paper, "Title of Paper", Some(PublicationYear));
    ```
3.  **Implement Safe Type Structures:** Construct target type definitions. For bipartite graphs (e.g., Petri nets), define places and transitions as separate types and enforce that arcs can only be constructed between places and transitions, never between nodes of the same type:
    ```rust
    pub struct PlaceToTransitionArc {
        pub source: Place,
        pub target: Transition,
    }
    pub struct TransitionToPlaceArc {
        pub source: Transition,
        pub target: Place,
    }
    ```
    By omitting any constructor for place-to-place or transition-to-transition arcs, the compiler statically prevents bipartite violations.
4.  **Formulate Compile-Fail UI Test Fixtures:** Write an integration test file (e.g., `tests/ui/fail_bipartite.rs`) containing code that attempts to violate the invariant:
    ```rust
    fn main() {
        let p1 = Place::new("p1");
        let p2 = Place::new("p2");
        // Attempting to construct place-to-place link directly
        let illegal_arc = PlaceToTransitionArc { source: p1, target: p2 }; // Compile error: type mismatch
    }
    ```
5.  **Compile and Snapshot Validation:** Execute `cargo test --test ui_tests` using the `trybuild` validation framework.
    *   Verify that the compiler rejects the invalid program.
    *   Capture the compiler diagnostic `.stderr` file and save it as `tests/ui/fail_bipartite.stderr` for regression testing.
6.  **Record Graduation Boundaries:** Document the coverage boundaries in the paper coverage registry.

### 1.6 Error Handling & Exceptional Flows
*   **Statically Unexpressible Invariants:** If a structural constraint cannot be modeled in Rust's type system, it must be marked as `COVERED_BY_GRADUATION_BOUNDARY`. The system generates a runtime verification boundary inside the WebAssembly module, leaving the compile-time type mapping purely as a placeholder witness marker.
*   **Compilation Slip-Through:** If the compile-fail UI test compiles successfully, the type law is broken. The test suite halts with an exit code of `1`. The developer must refactor the types to enforce sealed traits or private fields.

---

## 2. Algorithm 2: PM4Py Oracle Mapping

### 2.1 Overview
The **PM4Py Oracle Mapping** protocol maps legacy, dynamically typed Python/Pandas-based process mining algorithms and structures to high-assurance WebAssembly execution boundaries and zero-cost static type schemas, enforcing strict memory boundaries and deterministic cross-platform execution.

### 2.2 Mathematical and Logical Formulation
Let $A_{\text{pm4py}}$ be a legacy process mining algorithm. Let $E$ be an event log. PM4Py models are dynamically typed objects with non-deterministic execution times and floating-point errors.
We define the mapping:
$$\mathcal{O}: (A_{\text{pm4py}}, E_{\text{DataFrame}}) \to (A_{\text{wasm4pm}}, E_{\text{Arena}})$$
such that for any execution, the outputs are identical across CPU architectures:
$$\text{Execute}(A_{\text{wasm4pm}}, E_{\text{Arena}}) \equiv \text{DeterministicOutput}$$
For Heuristics Miner, the dependency calculation uses deterministic tie-breaking over activity hashes:
$$\text{dep}(a, b) = \frac{|a \to b| - |b \to a|}{|a \to b| + |b \to a| + 1}$$
If $\text{dep}(a, b) = \text{dep}(c, d)$, tie-breaking uses:
$$\text{SortOrder}(a, b) < \text{SortOrder}(c, d) \iff \text{BLAKE3}(a) < \text{BLAKE3}(c)$$

### 2.3 Inputs and Outputs
*   **Inputs:**
    *   Event data stream $E$ (XES XML or OCEL 2.0 binary).
    *   PM4Py legacy entrypoint reference and execution parameters (e.g. noise thresholds).
*   **Outputs:**
    *   An attested, sandboxed WebAssembly execution candidate mapping (`OcelQuery` or `WfNetConst`).
    *   An Ed25519-signed JSON execution receipt containing performance metrics and structural proofs.

### 2.4 Preconditions
*   The event log activity alphabet size $|A|$ must satisfy $|A| \le 1,000$ to prevent stack/heap exhaustion during dense dependency matrix construction in WASM linear memory.
*   The input log must be parsed directly into contiguous guest memory arenas.

### 2.5 Step-by-Step Logic Flow
1.  **Ingest Stream to Arena:** Receive the raw event log binary stream. Parse the events sequentially and load them into a flat, contiguous memory arena in guest linear memory, eliminating dynamic pointer-chasing and garbage collection.
2.  **Determine Algorithm Entrypoint:** Map the PM4Py API function to its WebAssembly counterpart:
    *   `discover_petri_net_alpha` $\rightarrow$ Validated via post-hoc soundness audits in `wasm4pm`.
    *   `discover_heuristics_net` $\rightarrow$ `wasm4pm::discover_heuristics_miner`.
    *   `discover_petri_net_inductive` $\rightarrow$ `wasm4pm::discover_inductive_miner`.
3.  **Execute Zero-Copy Projections:** For recursive mining algorithms (e.g., Inductive Miner), instead of cloning or slicing sub-logs, project sub-DFG cuts as indexed bitmask views over the original contiguous memory arena.
4.  **Deterministic Calculations:** Calculate the relations (e.g., dependency scores) using fixed-point arithmetic or strict sorting rules. Resolve ties lexicographically by hashing activity names:
    $$\text{Hash}(a) = \text{BLAKE3}(a)$$
5.  **Emit Cryptographic Receipt:** CANON-encode the output structure according to RFC 8785, hash it, sign it with the validator's Ed25519 private key, and output the final receipt.

### 2.6 Error Handling & Exceptional Flows
*   **Memory Boundary Trap:** If the size of the event log or intermediate structures exceeds the pre-allocated linear memory arena (4GB limit), the execution halts with error code `0xFA01` and returns a refusal report.
*   **Instruction Cycle Budget Exhaustion:** If the execution of recursive cuts or alignments exceeds the JIT-metered cycle limits (e.g., 5,000,000 cycles for Inductive Miner), the runtime traps, terminates the execution, and returns a partial refusal witness containing the last valid progress indicator.

---

## 3. Algorithm 3: Admissible Process Evidence (Admission Gate)

### 3.1 Overview
The **Admission Gate** is the structural filter that validates parsed raw process data structures against a target witness's invariants, ensuring that only sound, safe, and compliant models and logs are admitted into the type-law runtime.

### 3.2 Mathematical and Logical Formulation
Let $v$ be a value of type $T$ in a `Parsed` state. Let $W$ be a witness marker with a set of structural laws $\Lambda_W = \{ \lambda_1, \lambda_2, \dots, \lambda_k \}$.
The Admission Gate defines a decision function:
$$\text{Admit}(v, W) = \begin{cases}
    \text{Ok}(\text{Admission}\langle T, W \rangle(v)) & \text{if } \forall \lambda_i \in \Lambda_W, \text{Check}(v, \lambda_i) \equiv \text{True} \\
    \text{Err}(\text{Refusal}\langle R, W \rangle) & \text{otherwise}
\end{cases}$$
For Petri net boundedness, the check function verifies:
$$\forall M \in [M_0\rangle, \forall p \in P, \quad M(p) \le 1 \quad (\text{1-boundedness})$$

### 3.3 Inputs and Outputs
*   **Inputs:**
    *   A raw parsed value $v : T$ representing a process model, log, or query.
    *   A target witness marker $W$ representing the specific academic paper or standard.
*   **Outputs:**
    *   `Ok(Admission<T, W>)` wrapping the validated value.
    *   `Err(Refusal<R, W>)` containing a strongly typed refusal proof indicating which law was violated.

### 3.4 Preconditions
*   The raw value $v$ must have been parsed successfully and reside in a `Parsed` state.
*   The witness marker $W$ must expose its collection of structural laws $\Lambda_W$.

### 3.5 Step-by-Step Logic Flow
1.  **Retrieve Structural Laws:** Query the witness marker $W$ to retrieve the set of active laws $\Lambda_W = \{ \lambda_1, \lambda_2, \dots, \lambda_k \}$.
2.  **Iterative Verification:**
    For each structural law $\lambda_i \in \Lambda_W$:
    *   Run the verification check `Check(v, \lambda_i)`.
    *   *Example (Petri Net 1-Boundedness):* Trace the state-space marking graph from $M_0$. If at any marking $M$, any place $p$ has $M(p) > 1$, return `false`.
    *   *Example (Workflow Net Structure):* Verify that there is exactly one source place $i$, one sink place $o$, and every node lies on a path from $i$ to $o$.
3.  **Halt on Violation:** If any check fails, immediately stop the loop. Construct a `Refusal<R, W>` object where the parameter $R$ identifies the exact law $\lambda_i$ that was violated, along with diagnostic details. Return `Err(Refusal<R, W>)`.
4.  **Issue Admission:** If all checks succeed, wrap the value $v$ in a type-safe `Admission<T, W>` container. Return `Ok(Admission<T, W>(v))`.

### 3.6 Error Handling & Exceptional Flows
*   **Liveness Verification Timeout:** If reachability analysis or liveness checking enters a complex state space that threatens to exceed execution time limits, the checker aborts with a `LivenessUnverifiable` error code, refusing admission.
*   **Malformed Type Representation:** If the structure of $v$ lacks required properties (e.g., a Petri Net missing its initial marking definition), the gate throws a `StructuralViolation` error, bypassing the standard checks.

---

## 4. Algorithm 4: Receipt-Bearing Execution

### 4.1 Overview
The **Receipt-Bearing Execution** protocol audits the history of execution transitions, commits, and actions to guarantee every change is signed, verified, and mapped to a valid law class.

### 4.2 Mathematical and Logical Formulation
Let $\mathcal{R}$ be a git repository or transactional log. Let the transition range be $[c_{\text{start}}, c_{\text{end}}]$. Let $K$ be the set of authorized law classes:
$$K = \{ \text{type-law}, \text{fixture-fail}, \text{fixture-pass}, \text{paper-ledger}, \text{audit} \}$$
For each action or commit $c$, let $M(c)$ be the message payload. The audit function returns:
$$\text{Audit}(\mathcal{R}, c_{\text{start}}, c_{\text{end}}) = (\text{valid}, \text{violations})$$
where a commit $c$ is a violation if:
$$\text{LawClass}(M(c)) \notin K \quad \lor \quad \text{SignatureVerify}(\text{JCS}(c_{\text{receipt}}), \text{PK}_{\text{validator}}) \equiv \text{False}$$
The Ed25519 signature verification checks:
$$[8][S]B = [8]R + [8][k]\text{PK}_{\text{validator}}$$
where:
$$k = \operatorname{SHA-512}(R \mathbin{\Vert} \text{PK}_{\text{validator}} \mathbin{\Vert} B_{\text{receipt}}) \pmod L$$

### 4.3 Inputs and Outputs
*   **Inputs:**
    *   A Git repository history or transaction log $\mathcal{R}$.
    *   The target transition range $[c_{\text{start}}, c_{\text{end}}]$.
    *   The set of authorized law classes $K$.
*   **Outputs:**
    *   A Boolean status flag `valid` indicating whether the execution/commit range is compliant.
    *   A list of `violations` detailing the non-compliant transitions or commits.

### 4.4 Preconditions
*   The validator has read-access to the repository or transaction log.
*   The public keys of the authorized auditors are pre-loaded into the validator state.

### 4.5 Step-by-Step Logic Flow
1.  **Ingest History:** Retrieve all commits or transitions in the range $[c_{\text{start}}, c_{\text{end}}]$. Initialize `violations = []`.
2.  **Examine Action Metadata:**
    For each commit $c$ in the range:
    a. Extract the commit message or metadata payload $M = \text{Message}(c)$.
    b. Parse the payload to verify the presence of a `Law:` annotation tag.
       *   If the tag `Law:` is absent, append $c$ to `violations` and proceed to the next iteration.
    c. Extract the associated law class $C_{\text{law}} = \text{ExtractLawClass}(M)$.
    d. Validate if $C_{\text{law}} \in K$ (belongs to the set of authorized law classes).
       *   If $C_{\text{law}}$ is not in $K$, append $c$ to `violations` and proceed to the next iteration.
3.  **Cryptographic Receipt Audit:**
    If $c$ contains a cryptographic receipt (for runtime transitions):
    a. Extract the JCS-canonicalized receipt bytes $B_{\text{receipt}} = \text{JCS}(c_{\text{receipt}})$.
    b. Recompute the BLAKE3 hash of the receipt payload:
       $$H_{\text{computed}} = \text{BLAKE3}(B_{\text{receipt}})$$
    c. Verify that $H_{\text{computed}}$ matches the hash embedded in the receipt signature header.
    d. Verify the Ed25519 signature of the receipt using the auditor's public key $\text{PK}_{\text{validator}}$:
       *   Check that the signature scalars are within bounds.
       *   Verify the signature equation using cofactor-8 clearing:
         $$[8][S]B = [8]R + [8][k]\text{PK}_{\text{validator}}$$
         where $S$ and $R$ are components of the signature, $B$ is the generator, and:
         $$k = \operatorname{SHA-512}(R \mathbin{\Vert} \text{PK}_{\text{validator}} \mathbin{\Vert} B_{\text{receipt}}) \pmod L$$
       *   If signature verification fails, append $c$ to `violations`.
4.  **Evaluate Compliance Status:**
    *   If `violations` is not empty, set `valid = false`.
    *   Else, set `valid = true`.
5.  **Output Results:** Return `(valid, violations)`.

### 4.6 Error Handling & Exceptional Flows
*   **Non-Standard Message Formats:** Any commit message that is empty or corrupt is treated as an immediate violation.
*   **Cryptographic Signature Mismatch:** If a signature contains invalid parameters or curve points not lying on the Ed25519 Curve25519, the validation framework throws a `MalformedSignature` trap and registers it as a compliance violation.

---

## 5. Verification Asset Links

To verify the implementation of these algorithms across the repository, consult the following assets:

*   For architectural comparisons of discovery algorithms, see the [Algorithmic Comparative Analysis](file:///Users/sac/process-intelligence/comparisons/algorithm_comparison.md).
*   For type boundary specifications, see the [PM4Py Oracle Mapping](file:///Users/sac/process-intelligence/sources/pm4py/oracle-map.md).
*   To examine the active transition signing and decommissioning rules, see the [Decommission State Specification](file:///Users/sac/process-intelligence/lifecycle/define_decommission-state_process_intelligence.md).
*   For details on the cryptographic receipt structure and validator settings, see the [Slide-to-Receipt Map](file:///Users/sac/process-intelligence/ma/slide-to-receipt-map.md).
