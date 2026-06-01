# Dr. Wil van der Aalst AGI Innovation — Gap Closure Workflow

This document outlines the formal workflow, dependency structure, and execution plan designed to close all structural, quality, and architectural gaps in the `process-intelligence` ecosystem.

---

## 1. Governance Principles (AGI Innovation Standards)

Under the **V30.1.1 Epoch**, all process mining operations are governed by three constitutional directives:
1. **Mathematical Soundness (Wil van der Aalst 1998):** Every Petri net must be sound, bounded, and live. The option-to-complete property must be compile-time verifiable.
2. **Non-Forgeability (Blue River Dam Doctrine):** No lifecycle state transition (e.g., `Raw → Admitted → Receipted`) can be forged. Every transition requires a cryptographic proof witness.
3. **Exhaustive Telemetry (OpenTelemetry + OCEL 2.0):** No event or object may be silently flattened or dropped without a signed `LossReport` specifying a named `LossPolicy`.

---

## 2. Gap Closure Priority Matrix

Remediations are sequenced by dependency and impact (derived from the [gaps/GAP_PRIORITY_MATRIX.md](file:///Users/sac/process-intelligence/gaps/GAP_PRIORITY_MATRIX.md)):

```
GAP_007 (Independent - WfNet split-brain) ---> Resolved
    │
    ▼
GAP_001 (Critical - Compat-to-Wasm Bridge)
    │
    ├─► GAP_002 (Critical - Named Validation Refusals in wasm4pm)
    ├─► GAP_003 (Major - Inductive Miner & Soundness Proofs)
    ├─► GAP_004 (Major - A* Alignment Conformance)
    ├─► GAP_005 (Major - OCPQ Query Execution Engine)
    ├─► GAP_006 (Minor - POWL Process Discovery)
    └─► GAP_008 (Minor - E0425 Fixture Quality) ---> In-progress
```

---

## 3. Workflow Execution Steps

### Phase 1: Securing the Compatibility Core (P1 Gaps)

#### Step 1: Resolve WfNet Soundness Forgeability (GAP_007)
- **Problem**: `WfNet::attest_witnessed()` allowed callers to transition a Workflow net to a `SoundnessWitnessed` state without verifying a soundness proof, creating a loophole.
- **Remediation**:
  - Changed `attest_witnessed()` from `pub` to `pub(crate)` in `wasm4pm-compat/src/petri.rs` to hide it from external users.
  - Added the `#[allow(dead_code)]` attribute to silence unused method warnings inside the crate.
  - Replaced the deprecated compile-fail test with `wfnet_attest_witnessed_private.rs` to verify E0624 (private method) compiler errors.

#### Step 2: Implement the Wasm4pm-to-Compat Bridge (GAP_001)
- **Plan**:
  - Add `wasm4pm-compat = { path = "../wasm4pm-compat", features = ["wasm4pm"] }` to `wasm4pm`'s `Cargo.toml`.
  - Refactor wasm4pm algorithm functions to consume `Evidence<EventLog, Admitted, W>` rather than raw, unvalidated logs.
  - Implement `GraduateToWasm4pm` in `wasm4pm::graduation` to bridge the type systems.

#### Step 3: Enforce Named Refusal Laws (GAP_002)
- **Plan**:
  - Deprecate `Error::ValidationError(String)` in `wasm4pm-types`.
  - Introduce `Refusal<R, W>` where `R` is a typed process-mining law (e.g., `DanglingEventObjectLink`).

---

### Phase 2: Advancing Mining Heuristics (P2 & P3 Gaps)

#### Step 4: Inductive Miner Integration (GAP_003)
- **Plan**: Scaffolds `InductiveMiner` returning `TypedProcessTree` structures from the compat library, enforcing block-structured soundness.

#### Step 5: A* Conformance Alignment (GAP_004)
- **Plan**: Replaces the token game replay approximation in wasm4pm with the optimal A* search algorithm, yielding exact fitness metrics bounded by `Between01`.

#### Step 6: OCPQ Relational Query Evaluator (GAP_005)
- **Plan**: Implements the object-centric pattern query (`OcpqEvaluator`) engine executing query shapes against admitted `Ocel20` logs.

---

## 4. SWARM Verification Guard

The 5-agent AGI swarm continuously monitors execution:
- **Drift Sentry**: Ensures EWMA metrics stay within control bounds ($LCL = 0.92$).
- **Telemetry Auditor**: Confirms event streams adhere strictly to OCEL 2.0 schemas.
- **Ledger Custodian**: Audits tamper-evident event block hashes.
- **Alignment Referee**: Measures performance bounds of A* solver ($<10\text{ms}$).
- **Stream Director**: Renders live telemetry and dynamic Petri net components.
