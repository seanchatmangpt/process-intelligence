# Legacy .ggen File Classification Ledger
**Generated:** 2026-06-01T13:40:00.469211
**Total Files:** 23

## Classification Summary

- **RENDERED_ARTIFACT_CONSTANTS:** 1
- **RENDERED_ARTIFACT_DOCUMENTATION:** 2
- **RENDERED_ARTIFACT_MAPPING:** 1
- **RENDERED_ARTIFACT_PROOF_LEDGER:** 12
- **RENDERED_ARTIFACT_REGISTRY:** 1
- **STATIC_ARTIFACT_WITH_WRONG_EXTENSION:** 1
- **TEMPLATE_PLACEHOLDER_INCOMPLETE:** 1
- **TEMPLATE_SCHEMA_AUTHORITY:** 2
- **TEMPLATE_TYPE_SURFACE_BOUNDARY:** 2

## Blocking Status

**Files Blocking Gate 4:** 0

## Detailed Classification Ledger

| Path | Owner | Classification | Remediation | Blocking | Source Type |
|------|-------|-----------------|-------------|----------|-------------|
| `ggen/audits/audit-component-boundary.sh.ggen` | process-intelligence (ggen) | RENDERED_ARTIFACT_PROOF_LEDGER | MOVE_TO_CHECKPOINTS_WITH_PROOF_RECEIPT | 🟢 NO | RENDERED_ARTIFACT |
| `ggen/audits/audit-feature-law.sh.ggen` | process-intelligence (ggen) | RENDERED_ARTIFACT_PROOF_LEDGER | MOVE_TO_CHECKPOINTS_WITH_PROOF_RECEIPT | 🟢 NO | RENDERED_ARTIFACT |
| `ggen/audits/audit-no-engine-in-wasm-feature.sh.ggen` | process-intelligence (ggen) | RENDERED_ARTIFACT_PROOF_LEDGER | MOVE_TO_CHECKPOINTS_WITH_PROOF_RECEIPT | 🟢 NO | RENDERED_ARTIFACT |
| `ggen/audits/audit-ts-brand-tokens.sh.ggen` | process-intelligence (ggen) | RENDERED_ARTIFACT_PROOF_LEDGER | MOVE_TO_CHECKPOINTS_WITH_PROOF_RECEIPT | 🟢 NO | RENDERED_ARTIFACT |
| `ggen/audits/audit-ts-enum-tagging.sh.ggen` | process-intelligence (ggen) | RENDERED_ARTIFACT_PROOF_LEDGER | MOVE_TO_CHECKPOINTS_WITH_PROOF_RECEIPT | 🟢 NO | RENDERED_ARTIFACT |
| `ggen/audits/audit-ts-monomorphization.sh.ggen` | process-intelligence (ggen) | RENDERED_ARTIFACT_PROOF_LEDGER | MOVE_TO_CHECKPOINTS_WITH_PROOF_RECEIPT | 🟢 NO | RENDERED_ARTIFACT |
| `ggen/audits/audit-ts-projection-surface.sh.ggen` | process-intelligence (ggen) | RENDERED_ARTIFACT_PROOF_LEDGER | MOVE_TO_CHECKPOINTS_WITH_PROOF_RECEIPT | 🟢 NO | RENDERED_ARTIFACT |
| `ggen/templates/feature-plan.yaml.ggen` | process-intelligence (ggen) | TEMPLATE_PLACEHOLDER_INCOMPLETE | COMPLETE_TEMPLATE_VARIABLES | 🟢 NO | TEMPLATE_SOURCE |
| `ggen/templates/specta-exporter.rs.ggen` | process-intelligence (ggen) | TEMPLATE_TYPE_SURFACE_BOUNDARY | PUBLISH_AS_CROSS_PROJECT_ARTIFACT | 🟢 NO | TEMPLATE_SOURCE |
| `ggen/templates/wasm-boundary.rs.ggen` | process-intelligence (ggen) | TEMPLATE_TYPE_SURFACE_BOUNDARY | PUBLISH_AS_CROSS_PROJECT_ARTIFACT | 🟢 NO | TEMPLATE_SOURCE |
| `ggen/templates/wasm4pm-compat.wit.ggen` | process-intelligence (ggen) | TEMPLATE_SCHEMA_AUTHORITY | PUBLISH_TO_COMPONENT_REGISTRY | 🟢 NO | TEMPLATE_SOURCE |
| `ggen/templates/wit-world.wit.ggen` | process-intelligence (ggen) | TEMPLATE_SCHEMA_AUTHORITY | PUBLISH_TO_COMPONENT_REGISTRY | 🟢 NO | TEMPLATE_SOURCE |
| `otel-weaver/ggen/audits/audit-live-check-findings-routed....` | otel-weaver | RENDERED_ARTIFACT_PROOF_LEDGER | MOVE_TO_CHECKPOINTS_WITH_PROOF_RECEIPT | 🟢 NO | RENDERED_ARTIFACT |
| `otel-weaver/ggen/audits/audit-no-telemetry-equals-process...` | otel-weaver | RENDERED_ARTIFACT_PROOF_LEDGER | MOVE_TO_CHECKPOINTS_WITH_PROOF_RECEIPT | 🟢 NO | RENDERED_ARTIFACT |
| `otel-weaver/ggen/audits/audit-registry-diff-routed.sh.ggen` | otel-weaver | RENDERED_ARTIFACT_PROOF_LEDGER | MOVE_TO_CHECKPOINTS_WITH_PROOF_RECEIPT | 🟢 NO | RENDERED_ARTIFACT |
| `otel-weaver/ggen/audits/audit-schema-url-present.sh.ggen` | otel-weaver | RENDERED_ARTIFACT_PROOF_LEDGER | MOVE_TO_CHECKPOINTS_WITH_PROOF_RECEIPT | 🟢 NO | RENDERED_ARTIFACT |
| `otel-weaver/ggen/audits/audit-weaver-finding-not-receipt....` | otel-weaver | RENDERED_ARTIFACT_PROOF_LEDGER | MOVE_TO_CHECKPOINTS_WITH_PROOF_RECEIPT | 🟢 NO | RENDERED_ARTIFACT |
| `otel-weaver/ggen/templates/pi-live-check-intake.rs.ggen` | otel-weaver | STATIC_ARTIFACT_WITH_WRONG_EXTENSION | RENAME_TO_CORRECT_EXTENSION | 🟢 NO | RENDERED_ARTIFACT |
| `otel-weaver/ggen/templates/pi-otel-constants.rs.ggen` | otel-weaver | RENDERED_ARTIFACT_CONSTANTS | MOVE_TO_EMITTED_WITH_RECEIPT | 🟢 NO | RENDERED_ARTIFACT |
| `otel-weaver/ggen/templates/pi-registry-diff-report.md.ggen` | otel-weaver | RENDERED_ARTIFACT_DOCUMENTATION | MOVE_TO_EMITTED_WITH_RECEIPT | 🟢 NO | RENDERED_ARTIFACT |
| `otel-weaver/ggen/templates/pi-telemetry-docs.md.ggen` | otel-weaver | RENDERED_ARTIFACT_DOCUMENTATION | MOVE_TO_EMITTED_WITH_RECEIPT | 🟢 NO | RENDERED_ARTIFACT |
| `otel-weaver/ggen/templates/pi-weaver-registry.yaml.ggen` | otel-weaver | RENDERED_ARTIFACT_REGISTRY | MOVE_TO_EMITTED_WITH_RECEIPT | 🟢 NO | RENDERED_ARTIFACT |
| `otel-weaver/ggen/templates/pi-witness-map.rs.ggen` | otel-weaver | RENDERED_ARTIFACT_MAPPING | MOVE_TO_EMITTED_WITH_RECEIPT | 🟢 NO | RENDERED_ARTIFACT |

## Ledger by Owner Project

### otel-weaver (11 files)

- `otel-weaver/ggen/audits/audit-live-check-findings-routed.sh.ggen`
  - Classification: RENDERED_ARTIFACT_PROOF_LEDGER
  - Remediation: MOVE_TO_CHECKPOINTS_WITH_PROOF_RECEIPT
  - Blocking: 🟢
- `otel-weaver/ggen/audits/audit-no-telemetry-equals-process.sh.ggen`
  - Classification: RENDERED_ARTIFACT_PROOF_LEDGER
  - Remediation: MOVE_TO_CHECKPOINTS_WITH_PROOF_RECEIPT
  - Blocking: 🟢
- `otel-weaver/ggen/audits/audit-registry-diff-routed.sh.ggen`
  - Classification: RENDERED_ARTIFACT_PROOF_LEDGER
  - Remediation: MOVE_TO_CHECKPOINTS_WITH_PROOF_RECEIPT
  - Blocking: 🟢
- `otel-weaver/ggen/audits/audit-schema-url-present.sh.ggen`
  - Classification: RENDERED_ARTIFACT_PROOF_LEDGER
  - Remediation: MOVE_TO_CHECKPOINTS_WITH_PROOF_RECEIPT
  - Blocking: 🟢
- `otel-weaver/ggen/audits/audit-weaver-finding-not-receipt.sh.ggen`
  - Classification: RENDERED_ARTIFACT_PROOF_LEDGER
  - Remediation: MOVE_TO_CHECKPOINTS_WITH_PROOF_RECEIPT
  - Blocking: 🟢
- `otel-weaver/ggen/templates/pi-live-check-intake.rs.ggen`
  - Classification: STATIC_ARTIFACT_WITH_WRONG_EXTENSION
  - Remediation: RENAME_TO_CORRECT_EXTENSION
  - Blocking: 🟢
- `otel-weaver/ggen/templates/pi-otel-constants.rs.ggen`
  - Classification: RENDERED_ARTIFACT_CONSTANTS
  - Remediation: MOVE_TO_EMITTED_WITH_RECEIPT
  - Blocking: 🟢
- `otel-weaver/ggen/templates/pi-registry-diff-report.md.ggen`
  - Classification: RENDERED_ARTIFACT_DOCUMENTATION
  - Remediation: MOVE_TO_EMITTED_WITH_RECEIPT
  - Blocking: 🟢
- `otel-weaver/ggen/templates/pi-telemetry-docs.md.ggen`
  - Classification: RENDERED_ARTIFACT_DOCUMENTATION
  - Remediation: MOVE_TO_EMITTED_WITH_RECEIPT
  - Blocking: 🟢
- `otel-weaver/ggen/templates/pi-weaver-registry.yaml.ggen`
  - Classification: RENDERED_ARTIFACT_REGISTRY
  - Remediation: MOVE_TO_EMITTED_WITH_RECEIPT
  - Blocking: 🟢
- `otel-weaver/ggen/templates/pi-witness-map.rs.ggen`
  - Classification: RENDERED_ARTIFACT_MAPPING
  - Remediation: MOVE_TO_EMITTED_WITH_RECEIPT
  - Blocking: 🟢

### process-intelligence (ggen) (12 files)

- `ggen/audits/audit-component-boundary.sh.ggen`
  - Classification: RENDERED_ARTIFACT_PROOF_LEDGER
  - Remediation: MOVE_TO_CHECKPOINTS_WITH_PROOF_RECEIPT
  - Blocking: 🟢
- `ggen/audits/audit-feature-law.sh.ggen`
  - Classification: RENDERED_ARTIFACT_PROOF_LEDGER
  - Remediation: MOVE_TO_CHECKPOINTS_WITH_PROOF_RECEIPT
  - Blocking: 🟢
- `ggen/audits/audit-no-engine-in-wasm-feature.sh.ggen`
  - Classification: RENDERED_ARTIFACT_PROOF_LEDGER
  - Remediation: MOVE_TO_CHECKPOINTS_WITH_PROOF_RECEIPT
  - Blocking: 🟢
- `ggen/audits/audit-ts-brand-tokens.sh.ggen`
  - Classification: RENDERED_ARTIFACT_PROOF_LEDGER
  - Remediation: MOVE_TO_CHECKPOINTS_WITH_PROOF_RECEIPT
  - Blocking: 🟢
- `ggen/audits/audit-ts-enum-tagging.sh.ggen`
  - Classification: RENDERED_ARTIFACT_PROOF_LEDGER
  - Remediation: MOVE_TO_CHECKPOINTS_WITH_PROOF_RECEIPT
  - Blocking: 🟢
- `ggen/audits/audit-ts-monomorphization.sh.ggen`
  - Classification: RENDERED_ARTIFACT_PROOF_LEDGER
  - Remediation: MOVE_TO_CHECKPOINTS_WITH_PROOF_RECEIPT
  - Blocking: 🟢
- `ggen/audits/audit-ts-projection-surface.sh.ggen`
  - Classification: RENDERED_ARTIFACT_PROOF_LEDGER
  - Remediation: MOVE_TO_CHECKPOINTS_WITH_PROOF_RECEIPT
  - Blocking: 🟢
- `ggen/templates/feature-plan.yaml.ggen`
  - Classification: TEMPLATE_PLACEHOLDER_INCOMPLETE
  - Remediation: COMPLETE_TEMPLATE_VARIABLES
  - Blocking: 🟢
- `ggen/templates/specta-exporter.rs.ggen`
  - Classification: TEMPLATE_TYPE_SURFACE_BOUNDARY
  - Remediation: PUBLISH_AS_CROSS_PROJECT_ARTIFACT
  - Blocking: 🟢
- `ggen/templates/wasm-boundary.rs.ggen`
  - Classification: TEMPLATE_TYPE_SURFACE_BOUNDARY
  - Remediation: PUBLISH_AS_CROSS_PROJECT_ARTIFACT
  - Blocking: 🟢
- `ggen/templates/wasm4pm-compat.wit.ggen`
  - Classification: TEMPLATE_SCHEMA_AUTHORITY
  - Remediation: PUBLISH_TO_COMPONENT_REGISTRY
  - Blocking: 🟢
- `ggen/templates/wit-world.wit.ggen`
  - Classification: TEMPLATE_SCHEMA_AUTHORITY
  - Remediation: PUBLISH_TO_COMPONENT_REGISTRY
  - Blocking: 🟢

## Gate 4 Criteria Verification

### Criterion 1: Every discovered .ggen file has classification, owner, remediation route, blocking status
**Result:** ✓ PASS — All 22 files classified

### Criterion 2: Zero unclassified .ggen files
**Result:** ✓ PASS

## Remediation Actions

### COMPLETE_TEMPLATE_VARIABLES (1 files)
- ggen/templates/feature-plan.yaml.ggen

### MOVE_TO_CHECKPOINTS_WITH_PROOF_RECEIPT (12 files)
- ggen/audits/audit-component-boundary.sh.ggen
- ggen/audits/audit-feature-law.sh.ggen
- ggen/audits/audit-no-engine-in-wasm-feature.sh.ggen
- ggen/audits/audit-ts-brand-tokens.sh.ggen
- ggen/audits/audit-ts-enum-tagging.sh.ggen
- ggen/audits/audit-ts-monomorphization.sh.ggen
- ggen/audits/audit-ts-projection-surface.sh.ggen
- otel-weaver/ggen/audits/audit-live-check-findings-routed.sh.ggen
- otel-weaver/ggen/audits/audit-no-telemetry-equals-process.sh.ggen
- otel-weaver/ggen/audits/audit-registry-diff-routed.sh.ggen
- otel-weaver/ggen/audits/audit-schema-url-present.sh.ggen
- otel-weaver/ggen/audits/audit-weaver-finding-not-receipt.sh.ggen

### MOVE_TO_EMITTED_WITH_RECEIPT (5 files)
- otel-weaver/ggen/templates/pi-otel-constants.rs.ggen
- otel-weaver/ggen/templates/pi-registry-diff-report.md.ggen
- otel-weaver/ggen/templates/pi-telemetry-docs.md.ggen
- otel-weaver/ggen/templates/pi-weaver-registry.yaml.ggen
- otel-weaver/ggen/templates/pi-witness-map.rs.ggen

### PUBLISH_AS_CROSS_PROJECT_ARTIFACT (2 files)
- ggen/templates/specta-exporter.rs.ggen
- ggen/templates/wasm-boundary.rs.ggen

### PUBLISH_TO_COMPONENT_REGISTRY (2 files)
- ggen/templates/wasm4pm-compat.wit.ggen
- ggen/templates/wit-world.wit.ggen

### RENAME_TO_CORRECT_EXTENSION (1 files)
- otel-weaver/ggen/templates/pi-live-check-intake.rs.ggen
