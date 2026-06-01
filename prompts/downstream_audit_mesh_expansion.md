# Downstream Audit Mesh Expansion Directive

This document defines the requirements to expand and reinforce the verification mesh of the process intelligence repository. The goal is to build an automated, self-auditing suite of test fixtures that validates every process claim, capability, and type law.

## 1. Automated Capability Verification
- **Traceability Matrix Tests**: Every capability listed in the reference matrices (such as the PM4Py comparison and execution capability matrices) must have at least one corresponding integration test.
- **Verification Assertions**: Tests must verify correctness of Petri net execution (e.g., that token replays on a sound network correctly yield fitness = 1.0, and that known mismatching traces yield the exact expected fitness and missing/remaining token counts).
- **Execution Speed Benchmarks**: Implement continuous integration benchmarks that track token replay throughput (traces/second) under WASM runtimes to detect performance regressions.

## 2. Type-Law Boundary Audits
- **Refusal Test Fixtures**: Design tests that feed intentionally broken event logs (e.g., logs with circular object references in OCEL, invalid lifecycles in XES, or duplicate event IDs) and verify that the `wasm4pm-compat` layer rejects them with a structured `RefusalReport`.
- **Loss Report Verifiers**: Design tests that convert standard files (e.g., converting OCEL to XES and back) and assert that the generated `LossReport` matches expected loss metrics (such as the exact number of discarded object attributes).
- **Axiom 2 Non-Monotonicity Rejection Tests**: Construct tests that attempt non-monotonic transitions (e.g., double-firing transitions to backtrack the witness, or joining inconsistent witness states) and assert that the wasm4pm core halts and yields `RefusalReport::NonMonotonicWitnessTransition`.
- **Receipt Signature & JCS Validation Tests**: Build tests that pass invalidly signed receipts, receipts signed by unauthorized roles (e.g. runner signature instead of auditor for graduation), or non-JCS-canonical payloads, and verify that the verification engine flags signature failures.

## 3. Lifecycle Gate and Autonomic Boundary Verifiers
- **Six Quality Gates Tests**: Implement tests that verify transition admissibility at each quality gate defined in the Blue River Dam Gate Map:
  - Gate 1: Check sound and non-sound Petri Nets.
  - Gate 2: Check safety and deadlock-freedom.
  - Gate 3: Check fitness admissibility boundary ($\theta_{\text{fit}} \ge 0.95$). Verify that if fitness is between $0.85$ and $0.95$, a valid Ed25519 board signature override is accepted, but an invalid or missing signature leads to rejection. Verify that any trace with fitness $< 0.85$ is rejected regardless of signatures.
  - Gate 4: Check that repaired nets preserve structural soundness.
  - Gate 5: Check Inductive Miner outputs and process debt reduction.
  - Gate 6: Check decommissioning state receipt emission, archival registration, and secure memory scrubbing.
- **Memory Scrubbing Verification Tests**: Write tests that verify that after decommissioning, the linear memory buffer containing event or marking data is actively zeroed out (all bytes are `0x00`).
- **Subnet Boundary Enforcement Tests**: Test that any attempt to auto-actuate transitions outside the Elastic Subnet ($T_{\text{elastic}}$) and within the Compliance Subnet ($T_{\text{compliance}}$) triggers a compilation or runtime error.

## 4. Dynamic Audit Scanning
- **Mesh Scanner**: Build an automated runner that scans all Markdown files in the `doctrine/`, `lifecycle/`, and `experiments/` folders, ensuring that all cross-references to files exist and that all cited code assets are compiled and tested.
- **Reporting**: The scanner must output a structured `MeshAuditReport` highlighting any orphaned documentation or untested execution boundaries.

## 5. Downstream Integration and Traceability
All implementation details must align with:
- [audit-lifecycle-completeness.md](file:///Users/sac/process-intelligence/audits/audit-lifecycle-completeness.md)
- [audit__experiment_completeness.md](file:///Users/sac/process-intelligence/experiments/audit__experiment_completeness.md)