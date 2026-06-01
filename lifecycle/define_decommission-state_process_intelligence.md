# Lifecycle: Define Decommission-State Process Intelligence

The **Decommissioning Stage** is the final phase of the process lifecycle, governing the safe, compliant, and auditable retirement of process models.

## Autonomic MAPE-K Mapping
* **Loop Role**: **Execute** & **Knowledge**
* **Responsibility**: In the Execute phase, the system revokes execution authorizations and stops event listeners. In the Knowledge phase, the final historical log metadata, performance logs, and residual rules are archived in the knowledge base.
* **Actuation Trigger**: Initiated by an autonomic flag or human operator when a process is replaced by an optimized variant or when the underlying business unit is shut down.

---

## Decommissioning Protocol & Receipt Structure

To prevent "ghost processes" (obsolete models that continue executing and consuming resources), a strict decommissioning protocol is enforced.

### 1. The Retirement Flow
1. **Quarantine State**: Stop accepting new case initiations ($\lambda_{new} = 0$). Allow existing in-flight cases to reach the sink place $o$.
2. **Log Export**: Compile the final execution log $L_{final}$ in OCEL 2.0.
3. **Execution Lock & Oblivion Protocol**: Revoke WASM execution permissions for the Petri Net. The host initiates the **Oblivion Protocol** to completely sanitize the guest space, overwriting the entire WASM linear memory space (`memory.size`) using three sequential passes of cryptographically secure random bytes generated via a ChaCha20 CSPRNG.
4. **Knowledge Harvest**: Extract structural patterns that were highly successful and catalog process debt resolved during the process's lifetime.
5. **Receipt Generation**: Generate the cryptographic decommissioning receipt.

### 2. Cryptographic Decommissioning Receipt
The **Decommissioning Receipt** ($R_d$) is a JSON-LD metadata document signed by the process engine authority:
$$R_d = \text{Ed25519}_{K_{priv}} \left( \text{BLAKE3}(N) \parallel \text{BLAKE3}(L_{final}) \parallel C_{total} \parallel F_{final} \parallel T_{retire} \right)$$
where:
* $\text{BLAKE3}(N)$ is the BLAKE3 hash of the Petri Net structure.
* $\text{BLAKE3}(L_{final})$ is the BLAKE3 hash of the final event log.
* $C_{total}$ is the total number of process cases processed during the model's active lifecycle.
* $F_{final}$ is the final calculated alignment fitness of the log against the model.
* $T_{retire}$ is the retirement timestamp.
* $\text{Ed25519}_{K_{priv}}$ is the Ed25519 signature of the decommissioning authority.

### 3. The Oblivion Protocol & Linear Memory Shredding

To prevent "ghost process" residual data leaks and resist JIT/heap escape exploits from hyper-intelligent or adversarial AGIs, the engine implements the **Oblivion Protocol**.

#### A. The Shredding Sequence
Upon execution frame termination or decommissioning, the host triggers the memory scuttling sequence. 
Let $M$ be the guest WASM linear memory buffer of size $S_{\text{buf}} = \text{ceiling} + 8$ bytes (which contains the double-buffered arena, including aligned padding bytes).

The ChaCha20 generator is initialized with a cryptographically secure 256-bit seed $K$ and a 96-bit zero-initialized nonce $N$:
$$K \in \{0,1\}^{256}, \quad N = 0^{96}$$

For each pass $p \in \{1, 2, 3\}$, a stream of pseudo-random bytes $C_p$ is generated using ChaCha20:
$$C_p = \text{ChaCha20}_K(N, \text{block\_counter})$$

Every byte of $M$ is overwritten sequentially by $C_p$:
$$\forall i \in [0, S_{\text{buf}} - 1], \quad M[i] \leftarrow C_{p}[i]$$

This sequence is executed three times ($p=1, 2, 3$) to guarantee physical and magnetic entropy erasure on the host pages.

#### B. Mathematical Proof of 100% Coverage
We prove that no part of the linear memory (including unused alignment offsets) escapes the shredding sequence.

Let the base address of the allocated buffer be $A_{\text{base}}$ and the requested memory ceiling be $C$.
1. The buffer is resized to $S_{\text{buf}} = C + 8$ bytes, meaning indices $0$ to $C + 7$ are valid addressable offsets.
2. The aligned allocator start address is:
   $$A_{\text{aligned}} = (A_{\text{base}} + 7) \land \sim 7$$
   yielding an alignment offset:
   $$\Delta_{\text{align}} = A_{\text{aligned}} - A_{\text{base}} \in [0, 7]$$
3. The active heap memory used for permanent and transient allocations is partitioned inside:
   $$\text{ActiveHeap} = [A_{\text{aligned}}, A_{\text{aligned}} + C]$$
   Since $A_{\text{aligned}} + C = A_{\text{base}} + \Delta_{\text{align}} + C$, and $\Delta_{\text{align}} \le 7$, the maximum address accessed is:
   $$\max(\text{ActiveHeap}) = A_{\text{base}} + \Delta_{\text{align}} + C - 1 \le A_{\text{base}} + C + 6$$
   which is strictly less than the allocated buffer boundary $A_{\text{base}} + C + 8$.
4. The shredding function iterates over the entire range:
   $$\text{Offset} \in [0, S_{\text{buf}})$$
   writing to $M[\text{Offset}]$, which maps to physical addresses:
   $$\text{Addr} \in [A_{\text{base}}, A_{\text{base}} + C + 8)$$
5. Because $\text{ActiveHeap} \subset [A_{\text{base}}, A_{\text{base}} + C + 8)$, we have:
   $$\text{ActiveHeap} \subset \text{Shredded Memory Space}$$
   This guarantees that both the active allocations (permanent and transient) and the alignment padding bytes ($[A_{\text{base}}, A_{\text{aligned}})$) are completely overwritten by the CSPRNG stream.

#### C. Programmatic Verification
Programmatic validation is implemented in the `wasm4pm` engine test suite at [sandbox.rs:L98-142](file:///Users/sac/process-intelligence/sources/wasm4pm/src/sandbox.rs#L98-142). The test initializes a 1024-byte arena, fills the entire raw buffer (1032 bytes) with a sentinel value `0xAA`, runs the Oblivion Protocol, and asserts:
1. **100% Modification**: The buffer does not contain its initial state.
2. **Keystream Identity**: Every single byte of the buffer matches the exact expected output of the 3-pass ChaCha20 CSPRNG stream.

---

## Standards Alignment

* **OCEL 2.0 Archive Standard**: The final event log is exported to an OCEL 2.0 SQLite database, ensuring that all object and event relations are preserved for historical audit without data loss.
* **POWL Retrospective**: The final POWL tree structure is saved to the corporate process model library, marked as `DECOMMISSIONED`.

---

## M&A Due Diligence Claims
In M&A, decommissioning proves **Risk Mitigation** and **Legacy Asset Retirement**.
* **Buyer Reliance**: The buyer relies on decommissioning receipts to verify that obsolete and risky legacy applications have been completely deactivated, eliminating software licensing and maintenance liabilities.
* **Slide-to-Receipt Map**: Slides stating "We successfully retired the legacy CRM workflows, saving $2M in annual maintenance" must link directly to the cryptographic decommissioning receipt showing the active lock date and zero active cases.

---

## Related Documents
* See the [Optimization Stage](file:///Users/sac/process-intelligence/lifecycle/define_optimization-state_process_intelligence.md) for pre-decommissioning.
* See the [Archive State](file:///Users/sac/process-intelligence/lifecycle/define_archive-state_process_intelligence.md) for long-term storage details.
* Back to [Lifecycle README](file:///Users/sac/process-intelligence/lifecycle/docs-law__lifecycle_readme.md).