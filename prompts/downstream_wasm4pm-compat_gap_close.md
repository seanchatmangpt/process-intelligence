# Downstream Gap Closing Directive: wasm4pm-compat Type-Law Foundry

This document defines the requirements to close compatibility and type-law gaps within `wasm4pm-compat`. The compatibility library must enforce strict algebraic and behavioral laws on all process assets before they are admitted to downstream execution.

## 1. The Evidence Lattice: Evidence<T, State, Witness>
To ensure formal proof of execution, all process artifacts (logs, models, alignments) must be wrapped in an `Evidence` container:
- **T**: The target process asset type (e.g., `Ocel2Log`, `PetriNet`).
- **State**: The dynamic state of verification (e.g., `Parsed`, `ValidatedSound`, `Replayed`).
- **Witness**: A cryptographic or structural proof of correctness (e.g., an algebraic signature, a reachability tree certificate, or an alignment cost matrix).
- **Lattice Order**: The set of verification states $S = \{\text{Parsed}, \text{ValidatedSound}, \text{Replayed}\}$ forms a join-semilattice $(S, \sqsubseteq)$ with the partial order:
  $$\text{Parsed} \sqsubseteq \text{ValidatedSound} \sqsubseteq \text{Replayed}$$
  A state transition is only valid if it moves monotonically upward in the information lattice.
  For any states $a, b \in S$, there exists a unique least upper bound (join) $a \sqcup b$ satisfying:
  - Associativity: $a \sqcup (b \sqcup c) = (a \sqcup b) \sqcup c$
  - Commutativity: $a \sqcup b = b \sqcup a$
  - Idempotency: $a \sqcup a = a$

### 1.1 Receipt-Shaped Object Graduation Specification
An `Evidence` instance graduates to a terminal "receipt-shaped" status and is serialized as an immutable, audited execution receipt when:
1. **Terminal Marking State**: The process state reaches a terminal marking (e.g., a token exists in the sink place $o$, or the process tree completes execution).
2. **Configurable Fitness Threshold**: The calculated replay fitness $f(\sigma, N)$ meets or exceeds the configurable fitness threshold $\theta_{\text{fit}}$ specified for the context:
   - **Board Admissibility**: $\theta_{\text{fit}} \geq 0.95$ (requires validator and Board-member signatures for lower values, but is strictly capped at $0.85$ absolute minimum).
   - **Audit Admissibility**: $\theta_{\text{fit}} \geq 0.85$.
3. **Role-Based Signature Registry**: The receipt hash is signed by a valid entity matching the `Auditor` or `Validator` role. The compat layer must verify signatures against a role key registry:
   - Registry roles: `Auditor`, `Runner`, `Board`, `Validator`.
   - Implement Ed25519 signature verification against public keys corresponding to these registered roles.
4. **JCS Canonicalization**: Prior to signature generation or verification, the unsigned receipt JSON payload must be serialized according to the **JSON Canonicalization Scheme (JCS - RFC 8785)**:
   $$B_{\text{receipt}} = \operatorname{JCS}(R_{\text{unsigned}})$$
   The signature is then validated against $B_{\text{receipt}}$ using the registered role public key:
   $$\operatorname{Ed25519-Verify}(\operatorname{PK}_{\text{role}}, B_{\text{receipt}}, \text{signature}) == \text{True}$$
5. **Receipt Schema Conformance**: The graduated object must conform to the `ProcessIntelligenceVerificationReceipt` schema defined in [define_slide-to-receipt_map.md](file:///Users/sac/process-intelligence/ma/define_slide-to-receipt_map.md), which contains:
   - `slide_id`: Presentation slide UUIDv4.
   - `slide_title` & `assertion_text`: Metadata and claim details.
   - `target_log_hash`: SHA-256 hash of the XES/OCEL event log.
   - `process_model_hash`: SHA-256 hash of the process model.
   - `query_definition`: Engine identifier (`wasm4pm`), URI, and parameters.
   - `verification_results`: Calculated `fitness`, `precision`, and `throughput_days` metrics.
   - `validator_signature`: Cryptographic signature of the execution engine.

## 2. Admission and Refusal Laws
- **Strict Schema Admission**: Implement strict parsers that refuse any log failing schema validation (e.g., XML schema for XES, JSON-schema/SQLite schema for OCEL 2.0).
- **Raw-Laundering Refusal**: Prevent unverified raw logs from bypassing type boundaries. All raw inputs must go through an admission pipeline that verifies:
  1. No duplicate event IDs exist.
  2. All event timestamps are monotonically ordered within trace scopes.
  3. All event-to-object and object-to-object relations in OCEL logs point to existing entities.
- **Refusal Report**: If a log is rejected, the engine must generate a structured `RefusalReport` documenting the exact violation location and the rule violated.

## 3. LossPolicy and LossReport (Semantic Loss Auditing)
When converting process logs and models between different public standards, the compat layer must generate a `LossReport` based on a predefined `LossPolicy`:
- **OCEL to XES Conversion**:
  - Objects are flattened into trace attributes.
  - Multi-perspective relationships are lost.
  - The `LossReport` must quantify the cardinality of lost relations and attribute pruning.
- **BPMN to Petri Net Conversion**:
  - Non-local choice semantics or OR-joins that cannot be expressed in standard Petri Nets must be explicitly reported as semantic loss.
- **Process Tree to DFG Conversion**:
  - Concurrency information and loop structures are flattened. The DFG must report the loss of block-structured hierarchy.

## 4. Structural Law Validation
Implement strict validation logic for:
- **XES**: Standard extensions (Concept, Lifecycle, Time, Organizational).
- **OCEL 2.0**: Object-centric tables, lifecycle attributes, and type safety constraints.
- **Petri Net**: Workflow net soundness ($WF$-net constraints).
- **POWL**: Proper block-structure constraints.
- **Declare**: Valid LTL formula templates (e.g., response, precedence, coexistence).

## 5. Downstream Integration and Traceability
All implementation details must align with:
- [type-law-atlas.md](file:///Users/sac/process-intelligence/sources/wasm4pm-compat/type-law-atlas.md)
- [pm4py_vs_compat_type_boundary_matrix.md](file:///Users/sac/process-intelligence/experiments/pm4py_vs_compat_type_boundary_matrix.md)