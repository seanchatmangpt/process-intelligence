# Downstream Audit Mesh Expansion Directive

This document defines the requirements to expand and reinforce the verification mesh of the process intelligence repository. The goal is to build an automated, self-auditing suite of test fixtures that validates every process claim, capability, and type law.

## 1. Automated Capability Verification
- **Traceability Matrix Tests**: Every capability listed in the reference matrices (such as the PM4Py comparison and execution capability matrices) must have at least one corresponding integration test.
- **Verification Assertions**: Tests must verify correctness of Petri net execution (e.g., that token replays on a sound network correctly yield fitness = 1.0, and that known mismatching traces yield the exact expected fitness and missing/remaining token counts).
- **Execution Speed Benchmarks**: Implement continuous integration benchmarks that track token replay throughput (traces/second) under WASM runtimes to detect performance regressions.

## 2. Type-Law Boundary Audits
- **Refusal Test Fixtures**: Design tests that feed intentionally broken event logs (e.g., logs with circular object references in OCEL, invalid lifecycles in XES, or duplicate event IDs) and verify that the `wasm4pm-compat` layer rejects them with a structured `RefusalReport`.
- **Loss Report Verifiers**: Design tests that convert standard files (e.g., converting OCEL to XES and back) and assert that the generated `LossReport` matches expected loss metrics (such as the exact number of discarded object attributes).

## 3. Dynamic Audit Scanning
- **Mesh Scanner**: Build an automated runner that scans all Markdown files in the `doctrine/`, `lifecycle/`, and `experiments/` folders, ensuring that all cross-references to files exist and that all cited code assets are compiled and tested.
- **Reporting**: The scanner must output a structured `MeshAuditReport` highlighting any orphaned documentation or untested execution boundaries.

## 4. Downstream Integration and Traceability
All implementation details must align with:
- [audit-lifecycle-completeness.md](file:///Users/sac/process-intelligence/audits/audit-lifecycle-completeness.md)
- [audit__experiment_completeness.md](file:///Users/sac/process-intelligence/experiments/audit__experiment_completeness.md)