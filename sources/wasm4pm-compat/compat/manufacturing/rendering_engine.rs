//! Jinja2-style template rendering engine
//!
//! Renders .rs.j2 templates into compiled Rust modules with witness tracking

use std::collections::HashMap;
use std::fmt;

/// Template variable binding
#[derive(Debug, Clone)]
pub struct TemplateContext {
    variables: HashMap<String, String>,
}

impl TemplateContext {
    /// Create a new template context
    pub fn new() -> Self {
        TemplateContext {
            variables: HashMap::new(),
        }
    }

    /// Bind a template variable
    pub fn set(&mut self, key: &str, value: &str) {
        self.variables.insert(key.to_string(), value.to_string());
    }

    /// Get a template variable
    pub fn get(&self, key: &str) -> Option<&str> {
        self.variables.get(key).map(|s| s.as_str())
    }

    /// List all bound variables
    pub fn all(&self) -> &HashMap<String, String> {
        &self.variables
    }
}

impl Default for TemplateContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Rendering error
#[derive(Debug, Clone)]
pub enum RenderError {
    /// Variable not found in context
    UndefinedVariable(String),
    /// Invalid template syntax
    InvalidSyntax(String),
    /// Rendering failed
    RenderFailed(String),
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RenderError::UndefinedVariable(var) => write!(f, "Undefined variable: {}", var),
            RenderError::InvalidSyntax(msg) => write!(f, "Invalid syntax: {}", msg),
            RenderError::RenderFailed(msg) => write!(f, "Render failed: {}", msg),
        }
    }
}

/// Template rendering engine
pub struct RenderEngine {
    context: TemplateContext,
}

impl RenderEngine {
    /// Create a new rendering engine
    pub fn new() -> Self {
        RenderEngine {
            context: TemplateContext::new(),
        }
    }

    /// Set template context
    pub fn with_context(mut self, context: TemplateContext) -> Self {
        self.context = context;
        self
    }

    /// Render a template string
    /// Replaces {{ var }} with context values
    pub fn render(&self, template: &str) -> Result<String, RenderError> {
        let mut output = template.to_string();

        // Find all {{ variable }} patterns and replace
        let pattern = regex::Regex::new(r"\{\{\s*(\w+)\s*\}\}").map_err(|e| {
            RenderError::RenderFailed(format!("Regex error: {}", e))
        })?;

        for capture in pattern.captures_iter(template) {
            if let Some(var_match) = capture.get(1) {
                let var_name = var_match.as_str();
                let value = self
                    .context
                    .get(var_name)
                    .ok_or_else(|| RenderError::UndefinedVariable(var_name.to_string()))?;

                let full_match = capture.get(0).unwrap().as_str();
                output = output.replace(full_match, value);
            }
        }

        Ok(output)
    }

    /// Render with conditional blocks {{ #if condition }}...{{ /if }}
    pub fn render_with_conditionals(&self, template: &str) -> Result<String, RenderError> {
        // Simple conditional rendering
        // {{ #if variable }} ... {{ /if }}

        let mut output = template.to_string();

        // First render variables
        output = self.render(&output)?;

        // Then process conditionals
        let if_pattern =
            regex::Regex::new(r"\{\{#if\s+(\w+)\s*\}\}(.*?)\{\{/if\}\}")
                .map_err(|e| RenderError::RenderFailed(format!("Regex error: {}", e)))?;

        for capture in if_pattern.captures_iter(template) {
            if let (Some(cond), Some(block)) = (capture.get(1), capture.get(2)) {
                let condition = cond.as_str();
                let block_content = block.as_str();

                // Check if condition variable is truthy (non-empty)
                let should_include = self
                    .context
                    .get(condition)
                    .map(|v| !v.is_empty())
                    .unwrap_or(false);

                let replacement = if should_include {
                    block_content.to_string()
                } else {
                    String::new()
                };

                let full_match = capture.get(0).unwrap().as_str();
                output = output.replace(full_match, &replacement);
            }
        }

        Ok(output)
    }
}

impl Default for RenderEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_variable_rendering() {
        let mut context = TemplateContext::new();
        context.set("name", "AlphaMiner");
        context.set("version", "1.0");

        let engine = RenderEngine::new().with_context(context);
        let template = "Module: {{ name }} v{{ version }}";
        let result = engine.render(template).expect("render should succeed");

        assert_eq!(result, "Module: AlphaMiner v1.0");
    }

    #[test]
    fn test_undefined_variable_error() {
        let engine = RenderEngine::new();
        let template = "Module: {{ undefined }}";
        let result = engine.render(template);

        assert!(matches!(result, Err(RenderError::UndefinedVariable(_))));
    }
}
