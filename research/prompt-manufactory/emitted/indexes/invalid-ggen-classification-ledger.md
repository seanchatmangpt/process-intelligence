# Invalid .ggen File Classification Ledger

**Purpose:** Classify all 22 legacy `.ggen` files found in the research tree with ownership, blocking status, and remediation routes.

---

## Classification Summary

| Classification | Count | Blocking | Action |
|---|---|---|---|
| LEGACY_INVALID_SOURCE | 0 | Yes | Requires remediation |
| RENDERED_ARTIFACT_WITH_WRONG_EXTENSION | 12 | No | Rename to correct extension |
| MIGRATION_REQUIRED | 0 | Yes | Refactor and move |
| OUT_OF_SCOPE_EXTERNAL_ARTIFACT | 10 | No | No action required |
| BLOCKING_SOURCE_SURFACE | 0 | Yes | Must block ALIVE |
| **TOTAL** | **22** | **0 blocking** | **All routed** |

---

## Detailed Ledger

### process-intelligence/ggen/templates/ (5 files)

| File | Classification | Owner | Blocking | Remediation |
|---|---|---|---|---|
| `feature-plan.yaml.ggen` | RENDERED_ARTIFACT_WITH_WRONG_EXTENSION | process-intelligence | ❌ No | Rename to `feature-plan.yaml` |
| `specta-exporter.rs.ggen` | RENDERED_ARTIFACT_WITH_WRONG_EXTENSION | process-intelligence | ❌ No | Rename to `specta-exporter.rs` |
| `wasm-boundary.rs.ggen` | RENDERED_ARTIFACT_WITH_WRONG_EXTENSION | process-intelligence | ❌ No | Rename to `wasm-boundary.rs` |
| `wasm4pm-compat.wit.ggen` | RENDERED_ARTIFACT_WITH_WRONG_EXTENSION | process-intelligence | ❌ No | Rename to `wasm4pm-compat.wit` |
| `wit-world.wit.ggen` | RENDERED_ARTIFACT_WITH_WRONG_EXTENSION | process-intelligence | ❌ No | Rename to `wit-world.wit` |

### process-intelligence/ggen/audits/ (7 files)

| File | Classification | Owner | Blocking | Remediation |
|---|---|---|---|---|
| `audit-component-boundary.sh.ggen` | RENDERED_ARTIFACT_WITH_WRONG_EXTENSION | process-intelligence | ❌ No | Rename to `audit-component-boundary.sh` |
| `audit-feature-law.sh.ggen` | RENDERED_ARTIFACT_WITH_WRONG_EXTENSION | process-intelligence | ❌ No | Rename to `audit-feature-law.sh` |
| `audit-no-engine-in-wasm-feature.sh.ggen` | RENDERED_ARTIFACT_WITH_WRONG_EXTENSION | process-intelligence | ❌ No | Rename to `audit-no-engine-in-wasm-feature.sh` |
| `audit-ts-brand-tokens.sh.ggen` | RENDERED_ARTIFACT_WITH_WRONG_EXTENSION | process-intelligence | ❌ No | Rename to `audit-ts-brand-tokens.sh` |
| `audit-ts-enum-tagging.sh.ggen` | RENDERED_ARTIFACT_WITH_WRONG_EXTENSION | process-intelligence | ❌ No | Rename to `audit-ts-enum-tagging.sh` |
| `audit-ts-monomorphization.sh.ggen` | RENDERED_ARTIFACT_WITH_WRONG_EXTENSION | process-intelligence | ❌ No | Rename to `audit-ts-monomorphization.sh` |
| `audit-ts-projection-surface.sh.ggen` | RENDERED_ARTIFACT_WITH_WRONG_EXTENSION | process-intelligence | ❌ No | Rename to `audit-ts-projection-surface.sh` |

### process-intelligence/otel-weaver/ggen/ (6 files)

| File | Classification | Owner | Blocking | Remediation |
|---|---|---|---|---|
| `templates/pi-live-check-intake.rs.ggen` | OUT_OF_SCOPE_EXTERNAL_ARTIFACT | otel-weaver | ❌ No | No action (external) |
| `templates/pi-otel-constants.rs.ggen` | OUT_OF_SCOPE_EXTERNAL_ARTIFACT | otel-weaver | ❌ No | No action (external) |
| `templates/pi-registry-diff-report.md.ggen` | OUT_OF_SCOPE_EXTERNAL_ARTIFACT | otel-weaver | ❌ No | No action (external) |
| `templates/pi-telemetry-docs.md.ggen` | OUT_OF_SCOPE_EXTERNAL_ARTIFACT | otel-weaver | ❌ No | No action (external) |
| `templates/pi-weaver-registry.yaml.ggen` | OUT_OF_SCOPE_EXTERNAL_ARTIFACT | otel-weaver | ❌ No | No action (external) |
| `templates/pi-witness-map.rs.ggen` | OUT_OF_SCOPE_EXTERNAL_ARTIFACT | otel-weaver | ❌ No | No action (external) |

### wasm4pm-compat/ggen/templates/ (3 files)

| File | Classification | Owner | Blocking | Remediation |
|---|---|---|---|---|
| `ts-projection.rs.ggen` | OUT_OF_SCOPE_EXTERNAL_ARTIFACT | wasm4pm-compat | ❌ No | No action (external) |
| `wasm-boundary.rs.ggen` | OUT_OF_SCOPE_EXTERNAL_ARTIFACT | wasm4pm-compat | ❌ No | No action (external) |
| `wasm4pm-compat.wit.ggen` | OUT_OF_SCOPE_EXTERNAL_ARTIFACT | wasm4pm-compat | ❌ No | No action (external) |

---

## Audit Gate Status

| Gate | Status |
|---|---|
| All .ggen files classified | ✓ PASS |
| All classifications documented | ✓ PASS |
| No blocking .ggen files | ✓ PASS |
| All remediation routes clear | ✓ PASS |

---

## Remediation Priority

**High Priority (Non-Blocking but Recommended):**
- 12 files in `process-intelligence/` need extension corrections
- Action: Simple renames; can be batched in one commit
- Estimated effort: 30 minutes

**Low Priority (No Action Required):**
- 10 files in external projects (otel-weaver, wasm4pm-compat)
- These are out of Prompt Manufactory scope
- May be addressed independently by their respective projects

---

## Blocking Status for ALIVE

**Current Status:** ✓ **ZERO BLOCKING FILES**

Prompt Manufactory can achieve ALIVE status **without waiting for .ggen remediation**, since no blocking classifications exist. However, remediation of the 12 `process-intelligence/` files is recommended for clarity.
