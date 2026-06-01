# Downstream Gap Closing Directive: wasm4pm-compat Type-Law Foundry

This document defines the requirements to close compatibility and type-law gaps within `wasm4pm-compat`. The compatibility library must enforce strict algebraic and behavioral laws on all process assets before they are admitted to downstream execution.

## 1. The Evidence Lattice: Evidence<T, State, Witness>
To ensure formal proof of execution, all process artifacts (logs, models, alignments) must be wrapped in an `Evidence` container:
- **T**: The target process asset type (e.g., `Ocel2Log`, `PetriNet`).
- **State**: The dynamic state of verification (e.g., `Parsed`, `ValidatedSound`, `Replayed`).
- **Witness**: A cryptographic or structural proof of correctness (e.g., an algebraic signature, a reachability tree certificate, or an alignment cost matrix).
- **Lattice Order**: Define a partial order $\sqsubseteq$ over `Evidence` states such that:
  $$\text{Parsed} \sqsubseteq \text{ValidatedSound} \sqsubseteq \text{Replayed}$$
  A state transition is only valid if it moves monotonically upward in the information lattice.

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