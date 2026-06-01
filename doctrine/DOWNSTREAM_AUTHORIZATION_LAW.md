# DOWNSTREAM_AUTHORIZATION_LAW.md

**Authorization Law for Downstream Repository Refactor Mandates**
**Authority:** ~/process-intelligence
**Status:** ACTIVE — PROCESS_INTELLIGENCE_ALIVE_001 sealed

---

## The Law

> **No downstream refactor mandate may be issued until ~/process-intelligence ALIVE gate is sealed.**

This is not a preference. It is a manufacturing precondition. Issuing refactor mandates to downstream repositories before the research authority has sealed its ALIVE gate produces unconstrained downstream drift — refactors that cannot be receipted, replayed, or audited against a stable doctrine surface.

---

## Current Seal Status

| Checkpoint | Status | Commits | Criteria |
|------------|--------|---------|----------|
| PROCESS_INTELLIGENCE_ALIVE_001 | **SEALED** | 588 | 12/12 met |

ALIVE_001 is sealed. All authorized downstream workflows listed below are now available.

---

## Authorization Gate Logic

```
DownstreamRefactorAuthorized(repo, workflow) iff
  ALIVE_001.sealed = true
  AND workflow ∈ AuthorizedWorkflows
  AND repo ∈ workflow.targets
```

A downstream refactor that is not in AuthorizedWorkflows is not authorized, even with ALIVE_001 sealed. New workflows require a new doctrine entry or an explicit extension of this law.

---

## Authorized Downstream Workflows

The following workflows are now authorized. Each is receipted against ALIVE_001.

### 1. wasm4pm Refactor

**Target:** wasm4pm execution authority repository
**Prompt:** `prompts/DOWNSTREAM_WASM4PM_REFACTOR.md`
**Scope:** Align wasm4pm execution surfaces with process-intelligence type law doctrine
**Gate:** wasm4pm-compat ALIVE gate must pass after refactor
**Receipt requirement:** Commit receipts from wasm4pm repository referencing ALIVE_001

### 2. wasm4pm-compat Gap Close

**Target:** wasm4pm-compat process-evidence type foundry
**Prompt:** `prompts/DOWNSTREAM_COMPAT_GAP_CLOSE.md`
**Scope:** Close all gaps between wasm4pm-compat type surfaces and process-intelligence formal objects
**Gate:** wasm4pm-compat ALIVE gate must pass; all trybuild fixtures must be receipted
**Receipt requirement:** Trybuild compile-fail and compile-pass fixtures for all doctrine algorithms

### 3. ggen Projection Target Expansion

**Target:** ggen manufacturing and projection machinery
**Scope:** Expand ggen projection targets to cover all twelve lifecycle stages and all board projection claim types defined in the SPR thesis
**Gate:** Projection outputs must be traceable to receipted process evidence
**Receipt requirement:** Commit receipts demonstrating lifecycle stage coverage

### 4. Blue River Dam Lifecycle Authority Implementation

**Target:** Blue River Dam full-lifecycle authority layer
**Scope:** Implement the kappa operator: kappa(tau) ∈ {ADMIT(R), REFUSE(F), PARTIAL(X)} for all twelve lifecycle stages
**Gate:** Blue River Dam operating equation must be demonstrable: kappa(rho(alpha(mu(O*)))) → ALIVE | PARTIAL | REFUSED
**Receipt requirement:** Gate receipts for each lifecycle stage transition

### 5. M&A Deck Manufacturing

**Target:** M&A PowerPoint manufacturing surface (highest-value executive projection)
**Scope:** Manufacture a full M&A-ready deck where every slide claim satisfies Algorithm 5 validity law
**Gate:** Every B_i must satisfy: Evidence_i AND TypeLaw_i AND Receipt_i AND Replay_i AND Standard_i AND Lifecycle_i
**Receipt requirement:** Deck manufacturing receipt referencing all supporting process evidence

### 6. PM4Py Benchmark Comparison

**Target:** PM4Py comparative oracle integration
**Scope:** For each PM4Py capability c, produce Omega(c) mapping per Algorithm 2
**Gate:** BlueRiverReady(c) must hold for all covered capabilities
**Receipt requirement:** Oracle mapping receipts for each PM4Py capability

---

## Workflow Preconditions

Before initiating any authorized workflow:

1. Verify ALIVE_001 seal: `cat ~/process-intelligence/checkpoints/PROCESS_INTELLIGENCE_ALIVE_001.md`
2. Confirm the workflow prompt exists at its declared path
3. Read the SPR thesis: `~/process-intelligence/doctrine/PROCESS_INTELLIGENCE_SPR_THESIS.md`
4. Confirm the downstream repository is at a known-good commit (no uncommitted residuals)

---

## Adding New Authorized Workflows

To add a new downstream workflow:

1. Write the workflow prompt to `prompts/DOWNSTREAM_<NAME>.md`
2. Add an entry to the Authorized Downstream Workflows section of this document
3. Commit with message: `doctrine: authorize downstream workflow <NAME>`
4. The new workflow is authorized from that commit forward

---

## Violation Consequences

A refactor issued without authorization produces:

- Downstream commits that cannot be receipted against doctrine
- Type law changes without ALIVE gate coverage
- Board projection claims without traceable evidence
- Refusal reasons that do not match named doctrine laws

These are manufacturing defects, not discrepancies. They must be repaired, not accepted.

---

*Sealed against: PROCESS_INTELLIGENCE_ALIVE_001 (588 commits, 12/12 criteria).*
*See: doctrine/PROCESS_INTELLIGENCE_SPR_THESIS.md for formal object definitions.*
