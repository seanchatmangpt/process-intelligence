//! Manufacturing traits
//!
//! Core traits for renderable modules and receipted artifacts

use std::path::PathBuf;

/// A module that can be rendered from a template
pub trait RenderableModule: Sized {
    /// Module name
    fn module_name(&self) -> &str;

    /// Template source path
    fn template_source(&self) -> &PathBuf;

    /// Render module to Rust source code
    fn render(&self) -> Result<String, String>;

    /// Verify the rendered module compiles
    fn verify_compilation(&self) -> Result<bool, String>;
}

/// An artifact that carries a receipt (provenance + witness)
pub trait Receiptable {
    /// Content hash (blake3)
    fn content_hash(&self) -> &str;

    /// Witness marker at graduation
    fn witness(&self) -> &str;

    /// Verify receipt integrity
    fn verify_receipt(&self) -> bool;

    /// Export receipt as JSON
    fn receipt_json(&self) -> String;
}

/// Combined trait for modules that are both renderable and receiptable
pub trait ManufacturedModule: RenderableModule + Receiptable {
    /// Lifecycle state at manufacturing
    fn lifecycle_state(&self) -> &str {
        "Sealed"
    }

    /// Check if manufacturing is complete (rendered, compiled, audited)
    fn manufacturing_complete(&self) -> bool;
}
