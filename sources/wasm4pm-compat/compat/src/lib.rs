#![forbid(unsafe_code)]
//! wasm4pm-compat: Type law manufacturing for process intelligence


pub mod manufacturing;
pub mod graduation;

pub use graduation::{GraduateToWasm4pm, GraduationCandidate, GraduationReason};

pub use manufacturing::{
    RenderEngine, RenderError, TemplateContext, ModuleSpec, RenderContext,
    RustGenerator, GeneratedModule, TomlGenerator, Dependency, CargoTomlBuilder,
    ReceiptLedger, ArtifactReceipt, ComplianceAuditor, AuditResult, AuditLevel, AuditFinding,
    render_rust_module, render_toml, render_slide_deck, render_governance,
};
