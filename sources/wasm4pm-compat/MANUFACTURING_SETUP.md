# Manufacturing Setup — ggen Machinery Embedded in wasm4pm-compat

**Status:** Template Architecture Complete  
**Date:** 2026-05-31  
**Substrate:** wasm4pm-compat as framework (no separate factory repo)

---

## Directory Structure Created

```
wasm4pm-compat/
├── compat/
│   ├── templates/                          # Jinja2-style .rs.j2 templates
│   │   ├── mining/
│   │   │   ├── alpha_miner.rs.j2
│   │   │   ├── inductive_miner.rs.j2
│   │   │   └── heuristics_miner.rs.j2
│   │   ├── conformance/
│   │   │   ├── token_replay.rs.j2
│   │   │   └── alignment.rs.j2
│   │   ├── replay/
│   │   │   ├── executor.rs.j2
│   │   │   └── step_simulator.rs.j2
│   │   ├── lifecycle/
│   │   │   ├── state_machine.rs.j2
│   │   │   └── actuation.rs.j2
│   │   ├── fixtures/
│   │   │   ├── compile_pass_fixture.rs.j2
│   │   │   └── compile_fail_fixture.rs.j2
│   │   └── config.toml                     # Witness markers, lifecycle states
│   │
│   ├── manufacturing/                      # Rendering engines
│   │   ├── rendering_engine.rs             # Jinja2-style template renderer
│   │   ├── rust_generator.rs               # Renders .rs modules
│   │   ├── toml_generator.rs               # Renders Cargo.toml
│   │   ├── receipt_ledger.rs               # Artifact provenance tracking (blake3)
│   │   └── audit.rs                        # Compliance validation
│   │
│   └── src/
│       └── manufacturing/
│           ├── mod.rs                      # Public API
│           └── traits.rs                   # RenderableModule, Receiptable
```

---

## Components Delivered

### 1. **Template Layer** (`.rs.j2` templates)

**Mining Algorithm Templates:**
- `alpha_miner.rs.j2` — Discovers Petri nets from OCEL event logs (van der Aalst 2004)
- `inductive_miner.rs.j2` — Discovers process trees with ARITY law enforcement (Leemans 2013)
- `heuristics_miner.rs.j2` — Discovers causal nets with Between01 witness (Weijters 2011)

**Conformance Checking Templates:**
- `token_replay.rs.j2` — Token replay conformance (van der Aalst 1999)
- `alignment.rs.j2` — Process-log alignment (Adriansyah 2011)

**Execution & Simulation Templates:**
- `executor.rs.j2` — Process model executor (transitions, markings)
- `step_simulator.rs.j2` — Interactive step-by-step simulator

**Lifecycle Management Templates:**
- `state_machine.rs.j2` — Object lifecycle state machines
- `actuation.rs.j2` — Autonomic knowledge actuation policies

**Test Fixture Templates:**
- `compile_pass_fixture.rs.j2` — Proves legal use cases compile
- `compile_fail_fixture.rs.j2` — Proves illegal use cases do NOT compile (ui_test)

### 2. **Manufacturing Engines**

**RenderEngine** (Jinja2-style template processor)
- Substitutes `{{ variable }}` markers with context values
- Supports conditional blocks `{{ #if var }}...{{ /if }}`
- No external dependencies; simple string replacement

**RustGenerator**
- Reads `.rs.j2` templates
- Renders with TemplateContext
- Writes `.rs` modules to disk
- Verifies compilation (placeholder for rustc integration)

**TomlGenerator**
- Builds Cargo.toml configurations
- Configures dependencies, features, dev-dependencies
- Generates module config.toml with witness markers

**ReceiptLedger**
- Records artifact provenance: content_hash (blake3), witness, timestamp
- Tracks context snapshots (template variables used)
- Records compilation and audit status
- Exports ledger as text report or JSONL

**ComplianceAuditor**
- Validates generated code meets type law:
  - License header required
  - Witness markers enforced
  - Evidence types used correctly (lifecycle boundaries)
  - WfNetConst soundness non-forgeability
  - ConditionCell<N> law (N <= 8)
  - Between01<NUM, DEN> law (0 <= NUM/DEN <= 1)
- Reports findings as AuditResult (Error/Warning/Info)

### 3. **Public API** (`compat/src/manufacturing/`)

**RenderableModule** trait
- `module_name()` → module identifier
- `template_source()` → template path
- `render()` → Rust source code (or error)
- `verify_compilation()` → bool

**Receiptable** trait
- `content_hash()` → blake3 hash
- `witness()` → witness marker
- `verify_receipt()` → bool
- `receipt_json()` → JSON serialization

**ManufacturedModule** combined trait
- Implements both RenderableModule and Receiptable
- `lifecycle_state()` → current state (Sealed, etc.)
- `manufacturing_complete()` → bool (rendered, compiled, audited)

### 4. **Configuration** (`compat/templates/config.toml`)

Defines:
- **Witness markers**: VanDerAalst1989, VanDerAalst1998, VanDerAalst2016, Murata1989, Weijters2011, Leemans2013, Adriansyah2011, BlueRiverDam
- **Lifecycle states**: Created, Rendered, Compiled, Audited, Sealed, Witnessed, Graduated
- **Graduation boundaries**: 406 compile-pass + 398 compile-fail fixtures; 9 base modules; 9 manufacturing modules
- **Audit rules**: License required, witness references, evidence boundary enforcement, soundness non-forgeability
- **Template categories**: mining, conformance, replay, lifecycle, fixtures
- **First render target**: alpha_miner.rs from mining/alpha_miner.rs.j2 with VanDerAalst1989 witness

---

## First Render Test (NEXT PHASE)

**Target:** Prove manufacturing layer works before rendering wasm4pm

**Steps:**
1. Load `mining/alpha_miner.rs.j2` template
2. Render with TemplateContext (fill module name, test case, etc.)
3. Verify rendered code compiles (rustc check)
4. Validate compliance (AuditResult: license, witness, evidence boundaries)
5. Seal rendered artifact with receipt (blake3 hash, witness marker, timestamp)
6. Record receipt in ledger
7. Export ledger as report + FIRST_RENDER_SUCCESS.md

**Success Criteria:**
- ✓ Rendered module compiles without errors
- ✓ Audit passes: license + witness + law enforcement
- ✓ Receipt sealed in ledger
- ✓ FIRST_RENDER_SUCCESS.md documents the entire pipeline
- ✓ Ledger export shows: 1 artifact, witness=VanDerAalst1989, compiled=true, audit_passed=true

---

## Integration Points

### To Cargo.toml

Required dependencies for manufacturing machinery:
```toml
[dependencies]
regex = "1.10"        # For template variable matching
serde = "1.0"         # For receipt serialization (optional)
serde_json = "1.0"    # For JSON output (optional)
blake3 = "1.5"        # For content hashing (optional; can use sha256)
```

### To src/lib.rs

Export manufacturing API:
```rust
pub mod manufacturing;

pub use manufacturing::{
    RenderEngine, RustGenerator, TomlGenerator,
    ReceiptLedger, ComplianceAuditor,
    RenderableModule, Receiptable,
};
```

---

## Design Philosophy

1. **Type Law First**: Every generated module is checked for law compliance before graduation
2. **Receipt Integrity**: Provenance is hashed and ledger-tracked; no forged artifacts
3. **Non-Forgeable Witnesses**: Witness markers are paired with content hashes; witness-only claims are rejected
4. **Compliance as Evidence**: Audit findings are first-class artifacts (Error/Warning/Info)
5. **Zero External Rendering**: No Tera or Handlebars; simple Jinja2-style `{{ var }}` substitution

---

## What Comes Next

After first render success:

1. **Mining Module Rendering**: Render all 3 mining templates (alpha, inductive, heuristics)
2. **Conformance Module Rendering**: Render both conformance templates
3. **Replay & Lifecycle Modules**: Render executor, simulator, state_machine, actuation
4. **Fixture Generation**: Generate compile-pass and compile-fail test fixtures
5. **wasm4pm Graduation Bridge**: Seal manufacturing output; graduate to wasm4pm execution engine

---

## Evidence Trail

This document serves as the MANUFACTURING_SETUP receipt.

- **Date**: 2026-05-31
- **Status**: Template Architecture & Machinery Complete
- **Templates Created**: 11 (.rs.j2 files)
- **Manufacturing Engines**: 5 (RenderEngine, RustGenerator, TomlGenerator, ReceiptLedger, ComplianceAuditor)
- **Public API Traits**: 3 (RenderableModule, Receiptable, ManufacturedModule)
- **Configuration**: compat/templates/config.toml (witness markers, lifecycle, graduation boundaries)
- **Next**: First Render Test (mining/alpha_miner.rs.j2 → compiled module + receipt)
