# Rendering Authority: Complete Rendering Layer Implementation

## Overview

The rendering layer transforms high-level specifications into type-law-compliant code. It is the manufacturing authority responsible for graduating raw specifications into sealed, audited, receipted artifacts.

All generated code carries witness markers and must pass compliance audits before sealing with BLAKE3 receipts.

## Architecture

### 1. RenderEngine (Core Orchestrator)

**Purpose:** Template variable substitution with conditional support

**Features:**
- Jinja2-style `{{ variable }}` substitution
- Conditional blocks: `{{ #if condition }}...{{ /if }}`
- Escape hatch: all undefined variables raise `RenderError::UndefinedVariable`

**Usage:**
```rust
let mut context = TemplateContext::new();
context.set("module_name", "AlphaMiner");
context.set("witness", "VanDerAalst2016");

let engine = RenderEngine::new().with_context(context);
let rendered = engine.render_with_conditionals(template)?;
```

**Key Methods:**
- `render(&str) -> Result<String>` — Simple variable substitution
- `render_with_conditionals(&str) -> Result<String>` — With conditional support

### 2. RustGenerator

**Purpose:** Renders Rust modules with witness markers and lifecycle enforcement

**Features:**
- Template-based code generation (`.rs.j2` → `.rs`)
- Witness marker injection (spec.witness → context)
- Zero-unsafe-code enforcement
- Type law constraint injection
- Evidence carrier binding

**Usage:**
```rust
let spec = ModuleSpec {
    name: "AlphaMiner".to_string(),
    witness: "VanDerAalst2016".to_string(),
    evidence_carriers: vec!["Event", "Place"],
    lifecycle_states: vec!["Raw", "Admitted"],
    api_functions: vec![],
    features: vec![],
};

let gen = RustGenerator::new(templates_dir, output_dir);
let module = gen.generate_with_witness(&spec, &mut context)?;

// Enforce compliance
gen.check_unsafe_free(&module.content)?;
gen.check_witness_markers(&module.content)?;
```

**Enforcements:**
- `check_unsafe_free(&str) -> Result<(), RenderError>` — No `unsafe` blocks allowed
- `check_witness_markers(&str) -> Result<(), RenderError>` — Must reference witness

### 3. TomlGenerator

**Purpose:** Generates Cargo.toml and module configuration files

**Features:**
- Cargo.toml generation with dependencies
- Optional dependencies and feature flags
- Module metadata (authors, documentation)
- Module configuration TOML with witness and lifecycle metadata

**Usage:**
```rust
let mut builder = CargoTomlBuilder::new("wasm4pm-alpha", "1.0.0");
builder.add_author("wasm4pm witness: VanDerAalst2016");
builder.add_dependency(Dependency::new("serde", "1.0"));
builder.add_feature("witness", vec!["blake3"]);

let gen = TomlGenerator::new(output_dir);
let toml = gen.generate_cargo_toml(&builder)?;
```

**Configuration:**
```toml
[module]
name = "AlphaMiner"

[witness]
marker = "VanDerAalst2016"

[lifecycle]
state = "Raw"
state = "Admitted"

[graduation]
boundary = "compilation && audit_pass"
```

### 4. ReceiptLedger

**Purpose:** Track every rendered artifact with provenance and witness proof

**Features:**
- BLAKE3 content hash storage
- Witness marker recording
- Lifecycle state tracking
- Compilation and audit status
- Context snapshot (template variables at render time)
- Ledger-level verification and reporting

**Usage:**
```rust
let mut ledger = ReceiptLedger::new();

let receipt = ArtifactReceipt::new(
    "blake3_hash".to_string(),
    "VanDerAalst2016".to_string(),
    "templates/mining/alpha_miner.rs.j2".to_string(),
)
.with_compilation(true)
.with_audit(true);

ledger.record("AlphaMiner", receipt);

// Verify all receipts
assert!(ledger.verify_all());

// Export accountability report
println!("{}", ledger.export_report());
```

**Receipt Structure:**
```rust
pub struct ArtifactReceipt {
    pub content_hash: String,           // BLAKE3(content)
    pub witness: String,                // Witness marker
    pub lifecycle_state: String,        // "Sealed"
    pub timestamp: u64,                 // Unix timestamp
    pub template_source: String,        // Path to .rs.j2
    pub context_snapshot: HashMap<String, String>, // Variables at render time
    pub compiled: bool,                 // Passed rustc
    pub audit_passed: bool,             // Passed compliance audit
}
```

### 5. ComplianceAuditor

**Purpose:** Validate generated code meets type law constraints

**Features:**
- License header enforcement (wasm4pm graduation bridge)
- Witness marker presence checking
- Evidence type usage pattern validation
- WfNetConst soundness witnessing
- Zero-unsafe-code enforcement
- Template syntax verification
- Structured audit findings with severity levels

**Audit Levels:**
- `AuditLevel::Error` — Critical law violation (blocks graduation)
- `AuditLevel::Warning` — Minor issue (informational)
- `AuditLevel::Info` — Observation (non-blocking)

**Usage:**
```rust
let audit = ComplianceAuditor::audit_module("AlphaMiner", &content, &receipt);

if !audit.is_passed() {
    for finding in audit.findings {
        eprintln!("{}: {} at {}", finding.level, finding.code, finding.location);
    }
}

// Export detailed audit report
println!("{}", audit.export_report());
```

**Checks Performed:**

| Check | Level | Enforces |
|-------|-------|----------|
| License header | Error | `License: Executable only under wasm4pm graduation bridge` |
| Witness reference | Warning | Module mentions witness marker |
| Evidence construction | Error | Use `Evidence::raw()` not direct construction |
| Admitted boundary | Warning | Admitted state via `into_admitted()` |
| WfNet witnessing | Error | `SoundnessState::Witnessed` via `witness_soundness()` |
| Unsafe code | Error | Zero unsafe blocks |
| Template markers | Error | Balanced `{{ }}` in templates |

### 6. PowerPointGenerator (Stub)

**Purpose:** Render slide decks with charts and receipt references

**Features (Planned):**
- Slide template rendering from specifications
- Chart/diagram generation from process models
- Receipt chain visualization
- Board-admissibility callouts

**Usage:**
```rust
let deck = PowerPointGenerator::render_slide_deck(&claims, &evidence)?;
```

### 7. RQGenerator (Stub)

**Purpose:** Render governance rules as Research Questions

**Features (Planned):**
- Research questions as rendered artifacts
- Governance rules as RQ output
- Compliance checks as RQ queries

**Usage:**
```rust
let program = RQGenerator::render_governance(&rules)?;
```

## Public API

### Module-Level Functions

```rust
pub fn render_rust_module(spec: &ModuleSpec, context: &RenderContext)
    -> Result<GeneratedModule, RenderError>
```
Render a complete Rust module from specification with all constraints.

```rust
pub fn render_toml(spec: &ModuleSpec, context: &RenderContext)
    -> Result<String, RenderError>
```
Generate Cargo.toml configuration matching the module specification.

```rust
pub fn render_slide_deck(claims: &[String], evidence: &ReceiptLedger)
    -> Result<String, RenderError>
```
Render PowerPoint slide deck with receipt references.

```rust
pub fn render_governance(rules: &[String])
    -> Result<String, RenderError>
```
Render governance rules as RQ program.

### Traits

**RenderableModule**
```rust
pub trait RenderableModule: Sized {
    fn module_name(&self) -> &str;
    fn template_source(&self) -> &PathBuf;
    fn render(&self) -> Result<String, String>;
    fn verify_compilation(&self) -> Result<bool, String>;
}
```

**Receiptable**
```rust
pub trait Receiptable {
    fn content_hash(&self) -> &str;
    fn witness(&self) -> &str;
    fn verify_receipt(&self) -> bool;
    fn receipt_json(&self) -> String;
}
```

**ManufacturedModule**
```rust
pub trait ManufacturedModule: RenderableModule + Receiptable {
    fn lifecycle_state(&self) -> &str {
        "Sealed"
    }
    fn manufacturing_complete(&self) -> bool;
}
```

## Data Structures

### ModuleSpec
High-level specification (YAML/JSON/Rust enum):
```rust
pub struct ModuleSpec {
    pub name: String,                       // Module name
    pub witness: String,                    // Witness marker
    pub evidence_carriers: Vec<String>,     // Types holding Evidence<T>
    pub lifecycle_states: Vec<String>,      // Raw, Admitted, etc.
    pub api_functions: Vec<FunctionSpec>,   // Public API
    pub features: Vec<String>,              // Feature flags
}
```

### RenderContext
Rendering configuration with law enforcement:
```rust
pub struct RenderContext {
    pub output_dir: PathBuf,
    pub template_dir: PathBuf,
    pub inject_witnesses: bool,     // Default: true
    pub enforce_zero_unsafe: bool,  // Default: true
    pub audit_required: bool,       // Default: true
}
```

### GeneratedModule
Output metadata:
```rust
pub struct GeneratedModule {
    pub name: String,
    pub source_template: PathBuf,
    pub output_path: PathBuf,
    pub content: String,
    pub compiled: bool,
    pub compilation_errors: Vec<String>,
}
```

## Error Handling

All rendering operations return `Result<T, RenderError>`:

```rust
pub enum RenderError {
    UndefinedVariable(String),      // Variable not in context
    InvalidSyntax(String),          // Malformed template
    RenderFailed(String),           // Rendering failure
    IoError(String),                // File I/O failure
    ComplianceViolation(String),    // Type law violation
}
```

## Workflow Example: Simple Module Rendering

```rust
// 1. Define specification
let spec = ModuleSpec {
    name: "AlphaMiner".to_string(),
    witness: "VanDerAalst2016".to_string(),
    evidence_carriers: vec!["Event".to_string()],
    lifecycle_states: vec!["Raw".to_string(), "Admitted".to_string()],
    api_functions: vec![],
    features: vec![],
};

// 2. Create render context
let context = RenderContext::new(
    PathBuf::from("./templates"),
    PathBuf::from("./output"),
);

// 3. Render Rust module
let module = render_rust_module(&spec, &context)?;

// 4. Run compliance audit
let audit = ComplianceAuditor::audit_module(
    &spec.name,
    &module.content,
    &ArtifactReceipt::new(...),
);

if !audit.is_passed() {
    return Err(RenderError::ComplianceViolation(
        format!("Audit failed: {:?}", audit.findings),
    ));
}

// 5. Seal with receipt
let mut ledger = ReceiptLedger::new();
let receipt = ArtifactReceipt::new(
    blake3_hash(&module.content),
    spec.witness.clone(),
    "templates/alpha_miner.rs.j2".to_string(),
)
.with_compilation(true)
.with_audit(true);

ledger.record(&spec.name, receipt);

// 6. Generate Cargo.toml
let toml = render_toml(&spec, &context)?;

// 7. Export accountability report
println!("{}", ledger.export_report());
```

## Workflow Example: Complex Module with Conditionals

```rust
// Template with feature gates
let template = r#"
pub struct {{ module_name }} {
    witness: String,
}

{{#if enable_audit}}
impl {{ module_name }} {
    pub fn audit(&self) -> bool {
        true
    }
}
{{/if}}

pub fn get_witness() -> &'static str {
    "{{ witness }}"
}
"#;

// Render with feature flag
let mut context = TemplateContext::new();
context.set("module_name", "AlphaMiner");
context.set("witness", "VanDerAalst2016");
context.set("enable_audit", "true");

let engine = RenderEngine::new().with_context(context);
let rendered = engine.render_with_conditionals(template)?;
```

## Test Coverage

The implementation includes 23 comprehensive tests:

**Unit Tests (13):**
- Variable substitution
- Conditional rendering
- Error handling
- Cargo.toml generation
- Receipt creation and ledger operations
- Template auditing
- Unsafe code detection
- Witness marker enforcement

**Integration Tests (10):**
- Simple module rendering workflow
- Complex module with conditionals
- Receipt ledger compliance tracking
- Full audit pipeline (compliant code)
- Cargo.toml with features
- Template context variable binding
- Multiline conditional rendering
- Module configuration serialization
- Audit finding accumulation
- Witness marker injection

All tests pass: **23/23 ✓**

## Files

**Source:**
- `/Users/sac/process-intelligence/sources/wasm4pm-compat/compat/src/manufacturing/mod.rs` — Complete rendering engine (1800+ lines)
- `/Users/sac/process-intelligence/sources/wasm4pm-compat/compat/src/manufacturing/traits.rs` — Traits (RenderableModule, Receiptable, ManufacturedModule)
- `/Users/sac/process-intelligence/sources/wasm4pm-compat/compat/src/lib.rs` — Library root

**Validation:**
- `cargo test --lib` — All 23 tests pass
- `cargo doc --lib --no-deps` — Documentation generated (0 warnings)
- `cargo check --lib` — No errors or warnings

## Invariants

1. **Type Law Injection**: Every generated module has witness markers injected
2. **Zero Unsafe Code**: All generated code passes `check_unsafe_free()`
3. **Witness Attestation**: Every receipt carries witness marker at graduation
4. **Compliance Gates**: Audit failures block artifact sealing
5. **Immutable Receipts**: Once sealed with BLAKE3, receipts are tamper-evident
6. **Causality Chain**: Spec → Render → Test → Receipt forms unbroken chain

## Graduation Boundary

An artifact graduates from Manufacturing to Graduated when ALL of:
1. ✓ Rendered successfully (content generated)
2. ✓ Zero unsafe code (enforced)
3. ✓ Witness markers present (enforced)
4. ✓ Compilation successful (receipt.compiled = true)
5. ✓ Compliance audit passes (receipt.audit_passed = true)
6. ✓ BLAKE3 receipt sealed in ledger (receipt.verify() = true)

## Doctrine

> The product is CodeManufactory; the rendering layer is its manufacturing authority. Every artifact must prove its provenance through witness markers and sealed receipts before graduation.
