# Invalid GGen Classification Ledger

**Process Intelligence Program — Audit Report**
**Generated:** 2026-06-01 15:09:47

---

## Overview

This ledger documents legacy `.ggen` files requiring migration or containing classification law violations.

**Total Files Found:** 23

---

## 1. ggen/audits/audit-component-boundary.sh.ggen

- **ID:** `https://process.intelligence/invalid-ggen/FILE_001`
- **Classification:** `MIGRATION_REQUIRED`
- **Purpose:** Script validates WIT world segregation, forbids unsafe code, checks crate dependency isolation
- **Remediation Route:** Execute via Tera engine → output `.sh` file

---
## 2. ggen/audits/audit-feature-law.sh.ggen

- **ID:** `https://process.intelligence/invalid-ggen/FILE_002`
- **Classification:** `MIGRATION_REQUIRED`
- **Purpose:** Validates Cargo feature constraints (no execution tool smuggling)
- **Remediation Route:** Execute via Tera engine → output `.sh` file

---
## 3. ggen/audits/audit-no-engine-in-wasm-feature.sh.ggen

- **ID:** `https://process.intelligence/invalid-ggen/FILE_003`
- **Classification:** `MIGRATION_REQUIRED`
- **Purpose:** Enforces confinement of discovery/conformance/replay to wasm4pm (not compat)
- **Remediation Route:** N/A

---
## 4. ggen/audits/audit-ts-brand-tokens.sh.ggen

- **ID:** `https://process.intelligence/invalid-ggen/FILE_004`
- **Classification:** `MIGRATION_REQUIRED`
- **Purpose:** Validates TypeScript board claim DTOs carry witness brand tokens
- **Remediation Route:** N/A

---
## 5. ggen/audits/audit-ts-enum-tagging.sh.ggen

- **ID:** `https://process.intelligence/invalid-ggen/FILE_005`
- **Classification:** `MIGRATION_REQUIRED`
- **Purpose:** Verifies all TypeScript enums tagged with brand tokens
- **Remediation Route:** N/A

---
## 6. ggen/audits/audit-ts-monomorphization.sh.ggen

- **ID:** `https://process.intelligence/invalid-ggen/FILE_006`
- **Classification:** `MIGRATION_REQUIRED`
- **Purpose:** Validates Evidence<T, State, W> monomorphization completeness
- **Remediation Route:** N/A

---
## 7. ggen/audits/audit-ts-projection-surface.sh.ggen

- **ID:** `https://process.intelligence/invalid-ggen/FILE_007`
- **Classification:** `MIGRATION_REQUIRED`
- **Purpose:** Ensures TypeScript surface is projection-safe (no internal types leakage)
- **Remediation Route:** N/A

---
## 8. ggen/templates/feature-plan.yaml.ggen

- **ID:** `https://process.intelligence/invalid-ggen/FILE_012`
- **Classification:** `MIGRATION_REQUIRED`
- **Purpose:** Feature activation plan: adds derives and dependencies only, zero execution tools
- **Remediation Route:** N/A

---
## 9. ggen/templates/specta-exporter.rs.ggen

- **ID:** `https://process.intelligence/invalid-ggen/FILE_011`
- **Classification:** `MIGRATION_REQUIRED`
- **Purpose:** Rust + Specta bindings for automatic TypeScript export of board claim types
- **Remediation Route:** N/A

---
## 10. ggen/templates/wasm-boundary.rs.ggen

- **ID:** `https://process.intelligence/invalid-ggen/FILE_010`
- **Classification:** `MIGRATION_REQUIRED`
- **Purpose:** WASM DTO projections, lifecycle state enums, board claim safe projections for FFI
- **Remediation Route:** N/A

---
## 11. ggen/templates/wasm4pm-compat.wit.ggen

- **ID:** `https://process.intelligence/invalid-ggen/FILE_009`
- **Classification:** `MIGRATION_REQUIRED`
- **Purpose:** Evidence type definitions, admission gate (11 named law checks), refusal gate with named refusal reasons
- **Remediation Route:** N/A

---
## 12. ggen/templates/wit-world.wit.ggen

- **ID:** `https://process.intelligence/invalid-ggen/FILE_008`
- **Classification:** `MIGRATION_REQUIRED`
- **Purpose:** Defines two WIT worlds: compat-world (structure-only) and engine-world (execution) with strict boundary segregation
- **Remediation Route:** N/A

---
## 13. otel-weaver/ggen/audits/audit-live-check-findings-routed.sh.ggen

- **ID:** `https://process.intelligence/invalid-ggen/FILE_013`
- **Classification:** `MIGRATION_REQUIRED`
- **Purpose:** Verifies live check intake verdicts route only to court consequence (not process mining)
- **Remediation Route:** N/A

---
## 14. otel-weaver/ggen/audits/audit-no-telemetry-equals-process.sh.ggen

- **ID:** `https://process.intelligence/invalid-ggen/FILE_014`
- **Classification:** `MIGRATION_REQUIRED`
- **Purpose:** Rejects if telemetry feedstock conflated with process evidence
- **Remediation Route:** N/A

---
## 15. otel-weaver/ggen/audits/audit-registry-diff-routed.sh.ggen

- **ID:** `https://process.intelligence/invalid-ggen/FILE_015`
- **Classification:** `MIGRATION_REQUIRED`
- **Purpose:** Verifies weaver schema diffs route to observability channel (not receipt channel)
- **Remediation Route:** N/A

---
## 16. otel-weaver/ggen/audits/audit-schema-url-present.sh.ggen

- **ID:** `https://process.intelligence/invalid-ggen/FILE_016`
- **Classification:** `MIGRATION_REQUIRED`
- **Purpose:** Verifies all schema diffs include OpenTelemetry schema URL
- **Remediation Route:** N/A

---
## 17. otel-weaver/ggen/audits/audit-weaver-finding-not-receipt.sh.ggen

- **ID:** `https://process.intelligence/invalid-ggen/FILE_017`
- **Classification:** `MIGRATION_REQUIRED`
- **Purpose:** Rejects if weaver schema diff conflated with process receipt (weaver ≠ process drift)
- **Remediation Route:** N/A

---
## 18. otel-weaver/ggen/templates/pi-live-check-intake.rs.ggen

- **ID:** `https://process.intelligence/invalid-ggen/FILE_018`
- **Classification:** `MIGRATION_REQUIRED`
- **Purpose:** Rust struct + handlers for telemetry intake socket with route-to-court logic
- **Remediation Route:** N/A

---
## 19. otel-weaver/ggen/templates/pi-otel-constants.rs.ggen

- **ID:** `https://process.intelligence/invalid-ggen/FILE_019`
- **Classification:** `MIGRATION_REQUIRED`
- **Purpose:** OTel attribute key → value mappings (constant definitions)
- **Remediation Route:** N/A

---
## 20. otel-weaver/ggen/templates/pi-registry-diff-report.md.ggen

- **ID:** `https://process.intelligence/invalid-ggen/FILE_022`
- **Classification:** `MIGRATION_REQUIRED`
- **Purpose:** Human-readable OTel schema change report
- **Remediation Route:** N/A

---
## 21. otel-weaver/ggen/templates/pi-telemetry-docs.md.ggen

- **ID:** `https://process.intelligence/invalid-ggen/FILE_021`
- **Classification:** `MIGRATION_REQUIRED`
- **Purpose:** Human-readable schema documentation with feedstock/court nominal categories
- **Remediation Route:** N/A

---
## 22. otel-weaver/ggen/templates/pi-weaver-registry.yaml.ggen

- **ID:** `https://process.intelligence/invalid-ggen/FILE_023`
- **Classification:** `MIGRATION_REQUIRED`
- **Purpose:** OTel schema version catalog (YAML registry)
- **Remediation Route:** N/A

---
## 23. otel-weaver/ggen/templates/pi-witness-map.rs.ggen

- **ID:** `https://process.intelligence/invalid-ggen/FILE_020`
- **Classification:** `MIGRATION_REQUIRED`
- **Purpose:** Witness marker → OTel schema version mapping
- **Remediation Route:** N/A

---


**Generated at:** 2026-06-01T15:09:47Z
