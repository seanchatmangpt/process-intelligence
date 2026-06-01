# Auditor Evidence Path

The Auditor Evidence Path (AEP) defines the formal protocol that financial, compliance, and operational auditors must execute to verify process-related assertions made during due diligence. It ensures a transparent, tamper-proof, and reproducible chain of custody from raw database transactions to executive board slides.

## 1. The 5-Step Auditing Protocol

Auditors must follow this exact step-by-step verification pathway for every contested or high-impact process claim:

```
┌─────────────────────┐      1. Extract Slide UUID      ┌──────────────────────┐
│  M&A Slide Claim    │ ──────────────────────────────> │ Verification Receipt │
└─────────────────────┘                                 └──────────┬───────────┘
                                                                   │
                                                                   │ 2. Trace Lineage
                                                                   ▼
┌─────────────────────┐    3. BLAKE3 & Ed25519 Check    ┌──────────────────────┐
│  Source Database    │ <────────────────────────────── │   XES / OCEL Log     │
└─────────────────────┘                                 └──────────┬───────────┘
                                                                   │
                                                                   │ 4. Replay Conformance
                                                                   ▼
                                                        ┌──────────────────────┐
                                                        │  Petri Net Replay    │
                                                        │  (Optimal Alignment) │
                                                        └──────────────────────┘
```

### Step 1: Reference Extraction
* **Action**: Identify the slide's UUID and extract the corresponding cryptographic verification receipt from the VDR (e.g., `receipt_<slide_id>.json`).
* **Output**: The schema-compliant JSON receipt defining the target log hash, process model hash, query definition, and expected conformance metrics.

### Step 2: Source Lineage Audit (PROV-O Provenance)
* **Action**: Verify the data extraction lineage. Trace the event log generation script (SQL/OCPQ) back to the source ERP/CRM database transaction logs.
* **Output**: Verification that event timestamps match database commit timestamps within a synchronization tolerance ($\Delta t \le 1$ second).

### Step 3: Cryptographic Integrity Verification (BLAKE3 & Ed25519)
* **Action**: Re-calculate the BLAKE3 tree hash of the target event log $L_{\text{local}}$ and verify the Ed25519 signature of the verification receipt.
* **Math Check**: Verify that the file hash and the verification receipt signature match:
  $$\operatorname{BLAKE3}(L_{\text{local}}) == \text{target\_log\_hash}$$
  $$\operatorname{Ed25519-Verify}(\operatorname{PK}_{\text{validator}}, B_{\text{receipt}}, \text{validator\_signature}) == \text{True}$$
* **Output**: Tamper-proof validation of the event log data and non-repudiation of the verification receipt.

### Step 4: Model Soundness and Liveness Audit
* **Action**: Load the process model (Petri net Workflow Net) and verify structural soundness.
* **Math Check**: Formally verify soundness (van der Aalst 1998):
  1. **Option to Complete**: $\forall M \in [M_0\rangle, \quad [o] \in [M\rangle$
  2. **Liveness**: $\forall M \in [M_0\rangle, \forall t \in T, \exists M', M'' \in [P\rangle : M \xrightarrow{*} M' \xrightarrow{t} M''$
  3. **Boundedness**: $\exists k \in \mathbb{N}^+ : \forall M \in [M_0\rangle, \forall p \in P, \quad M(p) \le k$
* **Output**: A certified sound process model.

### Step 5: Conformance Replay and Alignment Audit
* **Action**: Execute the optimal alignment conformance algorithm (Adriansyah 2014) on the verified log and model using the parameters specified in the receipt.
* **Math Check**: Re-calculate fitness ($f$) and precision ($p$) using alignment-driven state space analysis, verifying that the difference lies within the replication tolerance:
  $$\left| f_{\text{audited}} - f_{\text{claimed}} \right| < 10^{-6} \quad \text{and} \quad \left| p_{\text{audited}} - p_{\text{claimed}} \right| < 10^{-6}$$
* **Output**: The final audit verdict.

## 2. Cryptographic Specifications

To protect log archives and validation receipts against retrospective modification, the verification protocol employs a combination of high-speed tree-based hashing (**BLAKE3**) and elliptic curve signatures (**Ed25519**).

### 2.1 BLAKE3 Hashing & Tree Structure Specifications
BLAKE3 is an agile, tree-structured cryptographic hash function designed to prevent post-hoc trace manipulation, insertion, deletion, or reordering. Rather than processing the event log $L$ as a flat byte stream, BLAKE3 constructs a binary Merkle tree over the data chunks.

1. **Chunk Partitioning**: The input log $L$ (represented as a sequence of bytes) is partitioned into $n$ contiguous chunks:
   $$L = C_0 \mathbin{\Vert} C_1 \mathbin{\Vert} \dots \mathbin{\Vert} C_{n-1}$$
   where each chunk $C_i$ has a maximum size of 1024 bytes.

2. **Compression Function**: The core compression function $F$ updates the 256-bit chaining value using a 512-bit message block $m$, a block counter $t$ (tracking the chunk's block offset), block length $d$, and a 32-bit bitfield of flags $f$:
   $$h_i = F(h_{i-1}, m, t, d, f)$$
   Where:
   - **`CHUNK_START`** flag (bit 0) is set on the first 64-byte block of a chunk.
   - **`CHUNK_END`** flag (bit 1) is set on the last 64-byte block of a chunk.
   - **`PARENT`** flag (bit 2) is set when compressing parent nodes.
   - **`ROOT`** flag (bit 3) is set on the final compression block to output the root hash.

3. **Leaf Node Derivation**: For each chunk $C_i$, the leaf hash is computed by compressing the 64-byte blocks of $C_i$ sequentially. For a single-block chunk, the leaf hash is:
   $$H_{\text{leaf}, i} = F(\text{IV}, C_i, i, \text{len}(C_i), \text{CHUNK\_START} \mid \text{CHUNK\_END})$$

4. **Parent Node Compression**: Adjacent leaf/parent node hashes are paired and compressed to form the parent node in the binary tree:
   $$H_{\text{parent}}(H_{\text{left}}, H_{\text{right}}) = F(\text{IV}, H_{\text{left}} \mathbin{\Vert} H_{\text{right}}, 0, 64, \text{PARENT})$$
   This process is repeated hierarchically until a single root hash $H_{\text{root}}$ is obtained.

5. **Root Digest Production**: The final parent compression sets the `ROOT` flag:
   $$H_{\text{root}} = F(\text{IV}, H_{\text{left}} \mathbin{\Vert} H_{\text{right}}, 0, 64, \text{PARENT} \mid \text{ROOT})$$

6. **Log Protection Against Retrospective Modification**:
   - **Native Merkle Tree**: Because BLAKE3 is natively a Merkle tree, any alteration (even a single bit representing an event timestamp or resource) propagates up the tree, invalidating $H_{\text{root}}$.
   - **Event-Level Chaining**: Within the event log, individual events are chained chronologically using BLAKE3:
     $$\mathcal{H}(e_j) = \operatorname{BLAKE3}(e_j \mathbin{\Vert} \mathcal{H}(e_{j-1}) \mathbin{\Vert} \operatorname{Sig}_{\text{system}}(e_j))$$
     where $\mathcal{H}(e_0) = \operatorname{BLAKE3}(\sigma_{\text{id}})$.
   - **Incremental Audit Paths**: Auditors can verify a subset of traces or event chunks without hashing the entire file. Given the sibling hashes (audit path) $\Pi_i$ from leaf $C_i$ to the root, verification of $C_i$'s integrity takes $O(\log n)$ time, ensuring that historical segments of the log cannot be silently modified without disrupting the entire tree.

### 2.2 Ed25519 Signature Validation Procedures
To guarantee that the receipt was produced by the certified `wasm4pm` execution core and has not been forged or tampered with, auditors must execute the following Ed25519 signature verification procedure:

1. **Deterministic Receipt Serialization**:
   - Remove the `validator_signature` field from the verification receipt JSON object to yield the unsigned receipt $R_{\text{unsigned}}$.
   - Serialize $R_{\text{unsigned}}$ using the **JSON Canonicalization Scheme (JCS - RFC 8785)** to yield a deterministic, system-independent byte sequence:
     $$B_{\text{receipt}} = \operatorname{JCS}(R_{\text{unsigned}})$$

2. **Key and Signature Parsing**:
   - Parse the 32-byte validator public key $\operatorname{PK}_{\text{validator}}$ and check that it is a valid compressed point representing a curve coordinate on the Edwards curve:
     $$-x^2 + y^2 = 1 - \frac{121665}{121666} x^2 y^2 \pmod p$$
     over the field $\mathbb{F}_p$ where $p = 2^{255} - 19$. If it represents a low-order point or is invalidly encoded, reject the signature.
   - Parse the 64-byte signature and split it into components $R$ (first 32 bytes, representing a curve point) and $S$ (last 32 bytes, representing a scalar).
   - Check that the scalar $S$ is in the range:
     $$0 \le S < L$$
     where $L = 2^{252} + 277454108928092425263413932207934334793$ is the prime order of the base point $B$. If $S \ge L$, the signature is invalid (this prevents signature malleability attacks).

3. **Scalar Hash Verification**:
   - Compute the message digest $M$ as the BLAKE3 hash of the canonical serialized receipt:
     $$M = \operatorname{BLAKE3}(B_{\text{receipt}})$$
   - Compute the verification scalar $k$ using SHA-512 over the concatenated signature components and message digest:
     $$k = \operatorname{SHA-512}(R \mathbin{\Vert} \operatorname{PK}_{\text{validator}} \mathbin{\Vert} M) \pmod L$$

4. **Curve Equation Check**:
   - Verify the verification relation:
     $$[S]B = R + [k]\operatorname{PK}_{\text{validator}}$$
     To avoid issues with the cofactor of the curve, clear the cofactor 8:
     $$[8][S]B = [8]R + [8][k]\operatorname{PK}_{\text{validator}}$$
   - If the equation holds, the receipt signature is certified as valid and authentic; otherwise, verification fails.

## 3. Auditing Toolchain Requirements

To ensure objectivity, all audit steps must be executed using open-source, standards-compliant tooling (e.g., the `wasm4pm` execution core or standard `pm4py` libraries) rather than proprietary, closed-source vendor packages.

## 4. Related M&A Validation Documents

* For the receipt structure, see [Slide-to-Receipt Map](file:///Users/sac/process-intelligence/ma/define_slide-to-receipt_map.md).
* For the mathematical definition of alignment, see [Slide-to-Replay Map](file:///Users/sac/process-intelligence/ma/define_slide-to-replay_map.md).
* For the buyer reliance standards, see [Buyer Reliance Requirements](file:///Users/sac/process-intelligence/ma/define_buyer_reliance_requirements.md).
* For the board admissibility rules, see [Board-Admissible Claim Requirements](file:///Users/sac/process-intelligence/ma/define_board-admissible_claim_requirements.md).