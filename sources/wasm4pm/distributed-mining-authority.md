# wasm4pm Research Map: Distributed Mining Authority (v30.1.1)

## 1. Abstract
This document outlines the v30.1.1 architecture for the Distributed Mining Authority within the `wasm4pm` ecosystem. It details the mechanisms by which process mining operations are decentralized, ensuring integrity and resilience through Byzantine Fault Tolerant (BFT) consensus protocols operating among a network of zero-trust autonomic agents.

## 2. Architectural Overview
The Distributed Mining Authority eliminates the centralized bottleneck of traditional process mining engines. In `wasm4pm`, the mining capability is embedded within WebAssembly modules deployed directly onto edge nodes and participant systems. These nodes act as zero-trust autonomic agents.

### 2.1 Zero-Trust Autonomic Agents
Each agent in the `wasm4pm` network operates under a zero-trust model. Agents do not implicitly trust event logs or partial process models shared by peers. Every incoming state transition is cryptographically verified against the locally held distributed ledger of process events.

## 3. Decentralized Consensus Mechanism
To maintain a globally consistent view of the discovered process model and the underlying event logs, `wasm4pm` employs a tailored BFT consensus algorithm.

### 3.1 Byzantine Fault Tolerance (BFT) in Process Mining
Given that agents may crash, experience network partitions, or act maliciously (e.g., attempting to inject fabricated event logs to alter the discovered process model), the consensus protocol must tolerate up to `f` Byzantine nodes in a network of `n = 3f + 1` nodes.

### 3.2 Consensus Phases
1. **Event Proposal:** An agent observes a local process event, packages it into a cryptographically signed block, and proposes it to the network.
2. **Pre-Prepare & Prepare:** The network nodes validate the event's structural integrity, timestamp, and signature. Agents exchange `PREPARE` messages.
3. **Commit:** Once an agent receives `2f` matching `PREPARE` messages, it broadcasts a `COMMIT`.
4. **Integration:** Upon collecting `2f + 1` `COMMIT` messages, the event is finalized into the local ledger.

### 3.3 Continuous Process Discovery
As events are committed via BFT, each agent runs its local `wasm4pm` discovery algorithms (e.g., Alpha Miner, Heuristics Miner) to update the emergent process model. Because all non-faulty agents share identical, verified event ledgers, the resulting process models converge deterministically across the network.

## 4. Security & Cryptographic Receipts
Every state transition within the process model generates a BLAKE3 cryptographically verifiable receipt. These receipts ensure that the sequence of events and the resulting model state can be independently audited without re-running the entire consensus process. OTel (OpenTelemetry) spans are heavily integrated to track the lineage of every BFT message and discovery operation.

## 5. Future Research Vectors (v30.2.+)
- Asynchronous BFT (aBFT) optimizations for high-latency edge environments.
- Zero-Knowledge Proofs (zk-SNARKs) for privacy-preserving process mining, allowing agents to prove the validity of a sequence of events without revealing the underlying sensitive data.