//! TOML configuration generator
//!
//! Generates Cargo.toml configurations for manufactured modules

use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::PathBuf;

/// Cargo.toml dependency specification
#[derive(Debug, Clone)]
pub struct Dependency {
    pub name: String,
    pub version: String,
    pub optional: bool,
    pub features: Vec<String>,
}

impl Dependency {
    pub fn new(name: &str, version: &str) -> Self {
        Dependency {
            name: name.to_string(),
            version: version.to_string(),
            optional: false,
            features: vec![],
        }
    }

    pub fn with_features(mut self, features: Vec<&str>) -> Self {
        self.features = features.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn optional(mut self, optional: bool) -> Self {
        self.optional = optional;
        self
    }
}

/// Cargo.toml configuration builder
#[derive(Debug, Clone)]
pub struct CargoTomlBuilder {
    name: String,
    version: String,
    edition: String,
    dependencies: HashMap<String, Dependency>,
    dev_dependencies: HashMap<String, Dependency>,
    features: HashMap<String, Vec<String>>,
}

impl CargoTomlBuilder {
    /// Create a new Cargo.toml builder
    pub fn new(name: &str, version: &str) -> Self {
        CargoTomlBuilder {
            name: name.to_string(),
            version: version.to_string(),
            edition: "2021".to_string(),
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            features: HashMap::new(),
        }
    }

    /// Add a dependency
    pub fn add_dependency(&mut self, dep: Dependency) -> &mut Self {
        self.dependencies.insert(dep.name.clone(), dep);
        self
    }

    /// Add a dev dependency
    pub fn add_dev_dependency(&mut self, dep: Dependency) -> &mut Self {
        self.dev_dependencies.insert(dep.name.clone(), dep);
        self
    }

    /// Add a feature
    pub fn add_feature(&mut self, name: &str, deps: Vec<&str>) -> &mut Self {
        self.features.insert(
            name.to_string(),
            deps.iter().map(|s| s.to_string()).collect(),
        );
        self
    }

    /// Build TOML content
    pub fn build(&self) -> String {
        let mut toml = format!(
            "[package]\nname = \"{}\"\nversion = \"{}\"\nedition = \"{}\"\n",
            self.name, self.version, self.edition
        );

        if !self.dependencies.is_empty() {
            toml.push_str("\n[dependencies]\n");
            for dep in self.dependencies.values() {
                toml.push_str(&format!("{} = \"{}\"\n", dep.name, dep.version));
                if !dep.features.is_empty() {
                    toml.push_str(&format!(
                        "# features: {}\n",
                        dep.features.join(", ")
                    ));
                }
            }
        }

        if !self.dev_dependencies.is_empty() {
            toml.push_str("\n[dev-dependencies]\n");
            for dep in self.dev_dependencies.values() {
                toml.push_str(&format!("{} = \"{}\"\n", dep.name, dep.version));
            }
        }

        if !self.features.is_empty() {
            toml.push_str("\n[features]\n");
            for (name, deps) in self.features.iter() {
                toml.push_str(&format!("{} = [{}]\n", name, deps.join(", ")));
            }
        }

        toml
    }
}

/// TOML generator for module configurations
pub struct TomlGenerator {
    output_dir: PathBuf,
}

impl TomlGenerator {
    /// Create a new TOML generator
    pub fn new(output_dir: PathBuf) -> Self {
        TomlGenerator { output_dir }
    }

    /// Generate a Cargo.toml configuration
    pub fn generate_cargo_toml(&self, builder: &CargoTomlBuilder) -> Result<String, String> {
        let content = builder.build();

        let output_path = self.output_dir.join("Cargo.toml");

        fs::create_dir_all(&self.output_dir)
            .map_err(|e| format!("Failed to create output directory: {}", e))?;

        fs::write(&output_path, &content)
            .map_err(|e| format!("Failed to write Cargo.toml: {}", e))?;

        Ok(content)
    }

    /// Generate module config.toml for witness markers
    pub fn generate_config_toml(&self, config: ModuleConfig) -> Result<String, String> {
        let content = config.to_toml();

        let output_path = self.output_dir.join("config.toml");

        fs::write(&output_path, &content)
            .map_err(|e| format!("Failed to write config.toml: {}", e))?;

        Ok(content)
    }
}

/// Module configuration metadata
#[derive(Debug, Clone)]
pub struct ModuleConfig {
    pub name: String,
    pub witness_markers: Vec<String>,
    pub lifecycle_states: Vec<String>,
    pub graduation_boundary: String,
}

impl ModuleConfig {
    /// Convert to TOML string
    pub fn to_toml(&self) -> String {
        let mut toml = format!("[module]\nname = \"{}\"\n", self.name);

        toml.push_str("\n[witness]\n");
        for marker in &self.witness_markers {
            toml.push_str(&format!("marker = \"{}\"\n", marker));
        }

        toml.push_str("\n[lifecycle]\n");
        for state in &self.lifecycle_states {
            toml.push_str(&format!("state = \"{}\"\n", state));
        }

        toml.push_str(&format!("\n[graduation]\nboundary = \"{}\"\n", self.graduation_boundary));

        toml
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cargo_toml_builder() {
        let mut builder = CargoTomlBuilder::new("test-module", "1.0.0");
        builder.add_dependency(Dependency::new("serde", "1.0"));

        let toml = builder.build();
        assert!(toml.contains("name = \"test-module\""));
        assert!(toml.contains("serde = \"1.0\""));
    }
}
