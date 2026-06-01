//! Rust module generator
//!
//! Generates .rs modules from .rs.j2 templates with compilation verification

use super::rendering_engine::{RenderEngine, RenderError, TemplateContext};
use std::fs;
use std::path::{Path, PathBuf};

/// Generated Rust module metadata
#[derive(Debug, Clone)]
pub struct GeneratedModule {
    pub name: String,
    pub source_template: PathBuf,
    pub output_path: PathBuf,
    pub content: String,
    pub compiled: bool,
    pub compilation_errors: Vec<String>,
}

/// Rust module generator
pub struct RustGenerator {
    template_dir: PathBuf,
    output_dir: PathBuf,
}

impl RustGenerator {
    /// Create a new Rust generator
    pub fn new(template_dir: PathBuf, output_dir: PathBuf) -> Self {
        RustGenerator {
            template_dir,
            output_dir,
        }
    }

    /// Generate a module from a template
    pub fn generate(
        &self,
        template_name: &str,
        context: TemplateContext,
    ) -> Result<GeneratedModule, RenderError> {
        let template_path = self.template_dir.join(format!("{}.rs.j2", template_name));

        let template_content = fs::read_to_string(&template_path)
            .map_err(|e| RenderError::RenderFailed(format!("Failed to read template: {}", e)))?;

        let engine = RenderEngine::new().with_context(context);
        let rendered_content = engine.render(&template_content)?;

        let output_path = self.output_dir.join(format!("{}.rs", template_name));

        let module = GeneratedModule {
            name: template_name.to_string(),
            source_template: template_path,
            output_path,
            content: rendered_content,
            compiled: false,
            compilation_errors: vec![],
        };

        Ok(module)
    }

    /// Generate and write module to disk
    pub fn generate_and_write(
        &self,
        template_name: &str,
        context: TemplateContext,
    ) -> Result<PathBuf, RenderError> {
        let module = self.generate(template_name, context)?;

        fs::create_dir_all(&self.output_dir)
            .map_err(|e| RenderError::RenderFailed(format!("Failed to create output dir: {}", e)))?;

        fs::write(&module.output_path, &module.content)
            .map_err(|e| RenderError::RenderFailed(format!("Failed to write module: {}", e)))?;

        Ok(module.output_path)
    }

    /// Verify generated module compiles
    pub fn verify_compilation(&self, module_path: &Path) -> Result<bool, String> {
        let temp_output = std::env::temp_dir().join(format!(
            "wasm4pm_compat_rust_check_{}",
            module_path.file_stem().and_then(|s| s.to_str()).unwrap_or("temp")
        ));
        let status = std::process::Command::new("rustc")
            .arg("--crate-type=lib")
            .arg("--edition=2021")
            .arg("--check")
            .arg("-o")
            .arg(temp_output)
            .arg(module_path)
            .status()
            .map_err(|e| format!("Failed to execute rustc: {}", e))?;
        Ok(status.success())
    }

    /// Generate all modules in a category
    pub fn generate_category(
        &self,
        category: &str,
        templates: Vec<(&str, TemplateContext)>,
    ) -> Result<Vec<GeneratedModule>, RenderError> {
        let mut modules = Vec::new();

        let category_dir = self.template_dir.join(category);

        for (template_name, context) in templates {
            let full_template_path = format!("{}/{}", category, template_name);

            // Verify template exists
            let template_file = category_dir.join(format!("{}.rs.j2", template_name));
            if !template_file.exists() {
                return Err(RenderError::RenderFailed(format!(
                    "Template not found: {}",
                    template_file.display()
                )));
            }

            let module = self.generate(&full_template_path, context)?;
            modules.push(module);
        }

        Ok(modules)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_generator_creation() {
        let gen = RustGenerator::new(
            PathBuf::from("/tmp/templates"),
            PathBuf::from("/tmp/output"),
        );

        assert_eq!(gen.template_dir, PathBuf::from("/tmp/templates"));
        assert_eq!(gen.output_dir, PathBuf::from("/tmp/output"));
    }
}
