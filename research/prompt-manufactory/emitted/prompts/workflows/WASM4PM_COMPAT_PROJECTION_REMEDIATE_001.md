# Workflow Warrant: WASM4PM_COMPAT_PROJECTION_REMEDIATE_001

**Program ID:** WASM4PM_COMPAT_PROJECTION_REMEDIATE_001
**Mission:** Remediate DTO flattening violation: move JSON serialization from compat → wasm4pm engine
**Authority Layer:** Research Program Law
**Workflow URI:** https://pi-research.dev/workflows#REMEDIATE_WORKFLOW
**Warrant Type:** Workflow Permit
**Status:** AUTHORIZED
**Issued:** 2026-06-03
**derivedFrom:** research-program-law.ttl#WASM4PM_COMPAT_PROJECTION_REMEDIATE_001

---

## Program Details

| Property | Value |
|----------|-------|
| **Program ID** | WASM4PM_COMPAT_PROJECTION_REMEDIATE_001 |
| **Mission** | Remediate DTO flattening violation: move JSON serialization from compat → wasm4pm engine |
| **Workflow** | https://pi-research.dev/workflows#REMEDIATE_WORKFLOW |
| **Prompt Class** | REMEDIATE |
| **Scope** | wasm4pm-compat DTO boundary, to_json_string(), receipt_json() |
| **Upstream Gap** | Identified by GGEN_ECOSYSTEM_INTEL_001: DTO flattening boundary violation |

---

## Workflow Phases

| Phase | Label | Mission | Subagent Roles |
|-------|-------|---------|----------------|
| 1 | Phase 1: Remediation | Route failed gates and implement fixes | Remediation Agent (primary) |

### Phase Transitions

This is a single-phase REMEDIATE workflow. The phase gate is:

```
Remediation Phase → COMPLETE
```

### Phase Entry Conditions

| Gate | Entry Condition |
|------|-----------------|
| Remediation can start | DTO flattening boundary violation documented with file:line evidence |
| Remediation is complete | to_json_string() and receipt_json() moved out of compat boundary; re-audit PASS |
| ALIVE can be declared | All 3 blocking issues from GGEN_ECOSYSTEM_INTEL_001 resolved with evidence |

---

## Remediation Target

**Blocking Issue:** DTO Flattening Boundary Violation in wasm4pm-compat

**Location:** wasm4pm-compat crate — `to_json_string()` and `receipt_json()` implementations that serialize DTOs to JSON strings inside the compat boundary.

**Why It Blocks:** The compat layer is a TYPE-LAW layer — it enforces type compatibility, not serialization. JSON serialization is an ENGINE concern (wasm4pm). Having serialization in compat creates a boundary violation where the type-law layer depends on engine serialization behavior.

**Required Remediation:**
1. Move `to_json_string()` and `receipt_json()` implementations OUT of wasm4pm-compat
2. Place them in the wasm4pm engine (the correct serialization layer)
3. Update compat boundary to reference engine-provided serialization
4. Re-run all 183+ compat tests — they must all PASS
5. Emit a new ALIVE receipt for wasm4pm-compat post-remediation

---

## Subagent Role Assignments

| Role | Owned Surface | Forbidden Surface | Output Contract |
|------|--------------|-------------------|-----------------|
| Remediation Agent | wasm4pm-compat/src/, wasm4pm/src/ | other repositories | research/pi-program/intel/remediation-report.md |

---

## Forbidden Paths

- Moving serialization without first documenting the current location with file:line evidence
- Declaring remediation complete before all 183+ tests pass
- Introducing new DTO boundary violations while fixing existing ones
- Emitting ALIVE before the re-audit confirms all 3 blocking issues resolved
- Hand-coding serialization changes without a warrant from this program
- Collapsing `Evidence<T, State, W>` into untyped JSON strings without boundary classification

**Authority:** `forbidden-collapse-law.ttl` — three collapse bans enforced at all lifecycle phases.

---

## Artifact Lifecycle

| Stage | Artifact | Gate | Verdict |
|-------|----------|------|---------|
| Remediation | Boundary violation fix | DTO boundary clean | PASS/FAIL |
| Remediation | Test suite re-run | 183+ tests PASS | PASS/FAIL |
| Remediation | Re-audit report | Blocking issues resolved | PASS/FAIL |
| Checkpoint | ALIVE/PARTIAL verdict | All remediation gates | ALIVE/PARTIAL |

**Lifecycle Rule:** Remediation is complete only when ALL blocking issues are resolved with evidence.
**Fallback Rule:** PARTIAL is the lawful verdict if any blocking issue remains. No forced ALIVE.

---

## Manufacturing Authorization

- **Authority Layer:** Research Program Law
- **Warrant Type:** Workflow Permit
- **Status:** AUTHORIZED
- **Binding Doctrine:** Process Intelligence Lifecycle Law
- **Upstream Receipt:** research-program-law.ttl (instance: WASM4PM_COMPAT_PROJECTION_REMEDIATE_001)
- **Manufacture Method:** Manual warrant from ontology instance (ggen SPARQL execution blocked — see audit.json)

*Warrant manufactured: 2026-06-03 | Authority: Process Intelligence Research Foundry*
