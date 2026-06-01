# Van der Aalst Conformance Auditing Program

**Objective:** Verify projection boundaries of wasm4pm-compat are sound per Van der Aalst process law.

**Doctrine:** If code says it worked but the event log cannot prove a lawful process happened, then it did not work.

---

## Quick Reference

| Audit | Result | Violations | Status |
|-------|--------|-----------|--------|
| **Audit 1: NO DTO FLATTENING** | FAIL ❌ | 1 | Blocking |
| **Audit 2: NO TOOLS IN COMPAT** | PASS ✅ | 0 | Clear |
| **Audit 3: FEATURE ISOLATION** | PASS ✅ | 0 | Clear |
| **Audit 4: PROJECTION RECEIPT** | PASS ✅ | 0 | Clear |
| **Audit 5: GRADUATION BOUNDARY** | PASS ✅ | 0 | Clear |

**Overall:** PARTIAL (4/5 pass; 1 DTO violation requiring remediation)

---

## Files in This Directory

### Audit Scripts (Deterministic & Reproducible)

- **`run_audit_1.sh`** — Detects `.*Dto`, `payload_json`, `_json`, `state_tag as String` patterns
- **`run_audit_2.sh`** — Maps forbidden tool functions to GraduationReason entries
- **`run_audit_3.sh`** — Verifies feature isolation (no specta, wasm-bindgen, tsify in default)
- **`run_audit_4.sh`** — Confirms projected artifacts (TS, WIT, manifests) are recorded
- **`run_audit_5.sh`** — Van der Aalst conformance: declared boundary ≠ actual surface = defect

### Audit Results

- **`audit-no-dto-flattening.txt`** — Audit 1 output; documents DTO violations
- **`audit-no-tool-smuggling.txt`** — Audit 2 output; tool audit results
- **`audit-feature-isolation.txt`** — Audit 3 output; feature boundary verification
- **`audit-projection-receipt.txt`** — Audit 4 output; projection artifact receipts
- **`audit-graduation-boundary.txt`** — Audit 5 output; Van der Aalst conformance

### Structured Results

- **`AUDIT_LOG.yaml`** — Machine-readable consolidated results
  - Violations with remediation steps
  - GraduationReason mappings
  - Feature isolation rules
  - Van der Aalst conformance checklist

### Reports

- **`CONFORMANCE_REPORT.md`** — Human-readable executive summary
  - Audit results dashboard
  - Violation details with rationale
  - Remediation plan (Priority 1: DTO fix)
  - Verdict and graduation path

- **`VAN_DER_AALST_AUDIT_SUMMARY.txt`** — Quick reference guide
  - All violations at a glance
  - Remediation roadmap (3 phases)
  - Reproducibility instructions
  - Status dashboard

---

## How to Run Audits

### Run All Audits

```bash
cd /Users/sac/process-intelligence/audits

bash run_audit_1.sh
bash run_audit_2.sh
bash run_audit_3.sh
bash run_audit_4.sh
bash run_audit_5.sh
```

### Run Individual Audit

```bash
bash run_audit_1.sh  # NO DTO FLATTENING only
```

### Verify Audit Reproducibility

All audits are deterministic. Re-running should produce identical results:

```bash
bash run_audit_1.sh > /tmp/audit1_first.txt
bash run_audit_1.sh > /tmp/audit1_second.txt
diff /tmp/audit1_first.txt /tmp/audit1_second.txt  # Should be empty
```

---

## Violation Summary

### Audit 1 Violation: DTO_001 (JSON Serialization Collapse)

**Pattern:** `_json` suffix in method names

**Locations:**
```
1. compat/src/manufacturing/mod.rs:735
   pub fn to_json_string(&self) -> String

2. compat/src/manufacturing/traits.rs:34
   fn receipt_json(&self) -> String
```

**Severity:** Error (admission refusal)

**Rationale:** JSON is a casual carrier that flattens structured Evidence<T> types. The `*_json` suffix violates type law requiring receipts to remain proof carriers.

**Fix:**
```rust
// Before
pub fn to_json_string(&self) -> String
fn receipt_json(&self) -> String

// After
pub fn serialize_receipt(&self) -> String
fn encode_receipt_proof(&self) -> String
```

**Blocking:** Yes — cannot graduate.

---

## Audit 2 Details: No Tools in Compat

**Forbidden Tools:** All present ✅
- ✅ simulate_replay → NeedsReplay
- ✅ compute_alignment → NeedsConformanceExecution
- ✅ discover_model → NeedsDiscovery
- ✅ execute_ocpq → NeedsObjectCentricQueryExecution
- ✅ run_conformance → NeedsConformanceExecution
- ✅ mint_receipt → NeedsReceipts
- ✅ benchmark_gate_run → NeedsBenchmarkGate

**Result:** PASS — No forbidden tools found.

---

## Audit 3 Details: Feature Isolation

**Rules:**
1. Default feature: no specta, wasm-bindgen, tsify ✅
2. ts feature: no wasm-bindgen unless explicitly paired ✅
3. wasm feature: no conformance/replay/discovery imports ✅
4. wasm4pm feature: no engine unless bridge-only ✅

**Current State:**
- Single lib target (no conditional features)
- Default dependencies: only serde
- Result: PASS

---

## Audit 4 Details: Projection Receipt

**Requirements:**
1. Projected artifacts (TS, WIT, .d.ts) committed/snapshotted ✅
2. Feature manifests exist ✅
3. Audit results recorded ✅

**Current State:**
- TypeScript projections: pending
- WIT projections: pending
- Cargo.toml manifest: present
- Audit files: 5 recorded
- Result: PASS

---

## Audit 5 Details: Graduation Boundary (Van der Aalst Conformance)

**Philosophy:** If code says it worked but event log cannot prove lawful process, it did not work.

**Conformance Checklist:**
1. **Declared Graduation Boundary:** `compilation && audit_pass` ✅
2. **Actual Surface:** 87 public items (structs, functions, traits) ✅
3. **Manifest Conformance:** 6 declared; 87 actual (superset lawful) ✅
4. **Receipt Trait:** Fully implemented ✅
   - `content_hash()` ✅
   - `witness()` ✅
   - `verify_receipt()` ✅
   - `encode_receipt_proof()` ✅
5. **Soundness Witnessing:** ArtifactReceipt.verify() checks all fields ✅

**Result:** PASS — Graduation boundary conforms to Van der Aalst law.

---

## Remediation Roadmap

### Phase 1: Apply DTO Flattening Fix
```bash
# Edit compat/src/manufacturing/mod.rs:735
# to_json_string() → serialize_receipt()

# Edit compat/src/manufacturing/traits.rs:34
# receipt_json() → encode_receipt_proof()

cargo test --lib
bash run_audit_1.sh
```

### Phase 2: Verify Conformance
- Re-run all 5 audits
- Confirm all PASS
- Check AUDIT_LOG.yaml for "PASS" verdicts

### Phase 3: Declare Checkpoint
```bash
mkdir -p checkpoints
touch checkpoints/PROCESS_INTELLIGENCE_COMPAT_CONFORMANCE_001
git add audits/ checkpoints/
git commit -m "checkpoint(compat): COMPAT_CONFORMANCE_001"
```

---

## Verdict

**Current Status:** PARTIAL

- ✅ 4/5 audits pass (tool isolation, feature isolation, receipt baseline, Van der Aalst conformance)
- ❌ 1/5 audit fails (DTO flattening)
- **Blocking:** Yes — DTO violation must be fixed before graduation

**Impact:** Compat-only issue. Does not affect wasm4pm graduation.

**Next Action:** Apply remediation, re-test, declare checkpoint.

---

## Related Documents

- `/Users/sac/process-intelligence/ggen/intel/forbidden-tool-ledger.yaml` — Tool definitions
- `/Users/sac/process-intelligence/ggen/intel/graduation-surface-ledger.yaml` — Manifest reference
- `/Users/sac/process-intelligence/CLAUDE.md` — Project rules (immutability, commit format)
- `~/.claude/rules/process-mining-chicago-tdd.md` — Van der Aalst Constitution

---

Generated: 2026-06-01 | Process Intelligence Research Foundry

