# wasm4pm-compat Manufacturing Layer

ggen manufacturing machinery embedded in wasm4pm-compat as a substrate framework.

## Quick Navigation

### 📋 Documentation
- **[MANUFACTURING_SETUP.md](../MANUFACTURING_SETUP.md)** — Setup overview and integration checklist
- **[TEMPLATE_ARCHITECTURE_SUMMARY.md](../TEMPLATE_ARCHITECTURE_SUMMARY.md)** — Comprehensive design guide
- **[config.toml](templates/config.toml)** — Witness markers, lifecycle states, graduation boundaries

### 🏗️ Architecture

```
compat/
├── templates/                    # 11 Jinja2-style .rs.j2 templates
│   ├── mining/                   # 3 process discovery algorithms
│   │   ├── alpha_miner.rs.j2
│   │   ├── inductive_miner.rs.j2
│   │   └── heuristics_miner.rs.j2
│   ├── conformance/              # 2 conformance checking engines
│   │   ├── token_replay.rs.j2
│   │   └── alignment.rs.j2
│   ├── replay/                   # 2 execution/simulation engines
│   │   ├── executor.rs.j2
│   │   └── step_simulator.rs.j2
│   ├── lifecycle/                # 2 state/actuation modules
│   │   ├── state_machine.rs.j2
│   │   └── actuation.rs.j2
│   ├── fixtures/                 # 2 test fixture templates
│   │   ├── compile_pass_fixture.rs.j2
│   │   └── compile_fail_fixture.rs.j2
│   └── config.toml               # Manufacturing configuration
│
├── manufacturing/                # 5 rendering engines
│   ├── rendering_engine.rs       # Jinja2-style {{ var }} substitution
│   ├── rust_generator.rs         # .rs.j2 → .rs module rendering
│   ├── toml_generator.rs         # Cargo.toml + config.toml generation
│   ├── receipt_ledger.rs         # Artifact provenance tracking (blake3)
│   └── audit.rs                  # Compliance validation
│
└── src/
    └── manufacturing/
        ├── mod.rs                # Public API exports
        └── traits.rs             # RenderableModule, Receiptable, ManufacturedModule
```

## Key Concepts

### Templates (Jinja2-style)
Each `.rs.j2` file is a Rust module stub with `{{ variable }}` placeholders:
```rust
//! Generated from: templates/mining/alpha_miner.rs.j2
pub struct {{ module_name }} {
    // ...
}
```

Rendering replaces `{{ module_name }}` with context values like "AlphaMiner".

### Manufacturing Engines

| Engine | Purpose |
|--------|---------|
| **RenderEngine** | Substitutes `{{ var }}` with context values |
| **RustGenerator** | Reads `.rs.j2`, renders, writes `.rs` to disk |
| **TomlGenerator** | Generates Cargo.toml and config.toml |
| **ReceiptLedger** | Records artifact provenance (blake3 hash + witness) |
| **ComplianceAuditor** | Validates generated code meets type law |

### Receipt (Non-Forgeable Artifact Record)
```json
{
  "content_hash": "abc123...",         // blake3 hash
  "witness": "VanDerAalst1989",        // Witness marker
  "lifecycle_state": "Sealed",
  "timestamp": 1717177200,
  "template_source": "mining/alpha_miner.rs.j2",
  "compiled": true,                    // Verified via rustc
  "audit_passed": true                 // All compliance rules passed
}
```

### Witness Markers (8 defined)
- `VanDerAalst1989` — Petri net soundness
- `VanDerAalst1998` — WF-net conformance
- `VanDerAalst2016` — Object-centric process mining
- `Murata1989` — Petri net theory
- `Weijters2011` — Heuristics miner algorithm
- `Leemans2013` — Inductive mining algorithm
- `Adriansyah2011` — Process-log alignment
- `BlueRiverDam` — Condition cell law

### Audit Rules (6 enforced)
1. License header required
2. Witness reference required
3. Evidence boundary enforcement
4. Soundness non-forgeability
5. ConditionCell<N> law (N <= 8)
6. Between01<NUM, DEN> law (0 <= NUM/DEN <= 1)

## Usage Example

### Step 1: Create Template Context
```rust
use manufacturing::TemplateContext;

let mut context = TemplateContext::new();
context.set("module_name", "AlphaMiner");
context.set("test_case", "discover from ocel");
```

### Step 2: Render Template
```rust
use manufacturing::RenderEngine;

let engine = RenderEngine::new().with_context(context);
let template = fs::read_to_string("templates/mining/alpha_miner.rs.j2")?;
let rendered = engine.render(&template)?;
```

### Step 3: Verify & Audit
```rust
use manufacturing::{ComplianceAuditor, ArtifactReceipt};

let audit = ComplianceAuditor::audit_module("alpha_miner", &rendered, &receipt);
assert!(audit.is_passed(), "Audit must pass before graduation");
```

### Step 4: Seal Receipt
```rust
use manufacturing::ReceiptLedger;

let hash = blake3::hash(rendered.as_bytes()).to_hex().to_string();
let receipt = ArtifactReceipt::new(
    hash,
    "VanDerAalst1989".to_string(),
    "templates/mining/alpha_miner.rs.j2".to_string(),
)
.with_compilation(true)
.with_audit(audit.is_passed());

ledger.record("alpha_miner", receipt);
```

## Public API

### Traits

**RenderableModule**
- `module_name() → &str`
- `template_source() → &PathBuf`
- `render() → Result<String, String>`
- `verify_compilation() → Result<bool, String>`

**Receiptable**
- `content_hash() → &str`
- `witness() → &str`
- `verify_receipt() → bool`
- `receipt_json() → String`

**ManufacturedModule** (combines both)
- `lifecycle_state() → &str`
- `manufacturing_complete() → bool`

### Engines

- `RenderEngine::new()` — Create renderer
- `RustGenerator::new(template_dir, output_dir)` — Create Rust code generator
- `TomlGenerator::new(output_dir)` — Create TOML generator
- `ReceiptLedger::new()` — Create artifact ledger
- `ComplianceAuditor::audit_module()` — Validate generated code
- `ComplianceAuditor::audit_template()` — Validate template syntax

## First Render Test (Next Phase)

To prove manufacturing works:

1. Load `templates/mining/alpha_miner.rs.j2`
2. Render with context (module_name="AlphaMiner", etc.)
3. Verify compilation (rustc check)
4. Audit compliance (license, witness, evidence boundaries)
5. Seal receipt in ledger (blake3 hash + timestamp)
6. Export as FIRST_RENDER_SUCCESS.md

Expected: rendered module compiles + audit passes + receipt recorded.

## Integration Checklist

- [ ] Add `regex = "1.10"` to Cargo.toml
- [ ] Add `blake3 = "1.5"` to Cargo.toml (optional)
- [ ] Update `src/lib.rs` to export manufacturing module
- [ ] Move engine modules to `src/manufacturing/`
- [ ] Run first render test
- [ ] Document success in FIRST_RENDER_SUCCESS.md

## Design Philosophy

1. **Type Law First** — Every generated module validated against type law
2. **Receipt Integrity** — Provenance hashed; witness + hash are immutable
3. **Non-Forgeable Witnesses** — Witness paired with content; tampering invalidates
4. **Compliance as Evidence** — Audit findings are first-class artifacts (Error/Warning/Info)
5. **Zero External Rendering** — No Tera/Handlebars; simple regex-based substitution

## Files at a Glance

| File | Lines | Purpose |
|------|-------|---------|
| `templates/mining/*.rs.j2` | 75-87 | Process discovery algorithm templates |
| `templates/conformance/*.rs.j2` | 68-69 | Conformance checking templates |
| `templates/replay/*.rs.j2` | 61-89 | Execution & simulation templates |
| `templates/lifecycle/*.rs.j2` | 92-119 | Lifecycle & actuation templates |
| `templates/fixtures/*.rs.j2` | 35-74 | Test fixture templates |
| `templates/config.toml` | 123 | Manufacturing configuration |
| `manufacturing/rendering_engine.rs` | 180 | Jinja2 renderer |
| `manufacturing/rust_generator.rs` | 155 | Rust code generator |
| `manufacturing/toml_generator.rs` | 218 | TOML configuration generator |
| `manufacturing/receipt_ledger.rs` | 279 | Artifact provenance ledger |
| `manufacturing/audit.rs` | 310 | Compliance auditor |
| `src/manufacturing/mod.rs` | 28 | Public API exports |
| `src/manufacturing/traits.rs` | 32 | Manufacturing traits |

**Total:** 19 files, ~1,300 LOC (templates + engines + API)

## See Also

- **wasm4pm** — Execution engine (graduation destination)
- **wasm4pm-compat** — Type law kernel (wasm4pm-compat/src/)
- **sources/papers/** — Academic papers referenced in witnesses
