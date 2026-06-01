use std::fs::{self, File};
use std::io::Write;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct SchemaGroup {
    id: String,
    #[serde(rename = "type")]
    group_type: String,
    brief: String,
    attributes: Vec<SchemaAttribute>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SchemaAttribute {
    id: String,
    #[serde(rename = "type")]
    attr_type: String,
    brief: String,
    requirement_level: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResolvedSchema {
    groups: Vec<SchemaGroup>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Simulate reading resolved_schema.json
    let json_data = r#"{
        "groups": [
            {
                "id": "process.pi.activity",
                "type": "span",
                "brief": "Represents a discrete execution activity within a process instance.",
                "attributes": [
                    {
                        "id": "process.pi.instance_id",
                        "type": "string",
                        "brief": "The unique identifier of the process execution instance.",
                        "requirement_level": "required"
                    },
                    {
                        "id": "process.pi.witness.id",
                        "type": "string",
                        "brief": "The identifier of the witness node.",
                        "requirement_level": "required"
                    },
                    {
                        "id": "process.pi.witness.hash",
                        "type": "string",
                        "brief": "BLAKE3 cryptographic hash sealing this specific transition trace.",
                        "requirement_level": "required"
                    }
                ]
            }
        ]
    }"#;

    let schema: ResolvedSchema = serde_json::from_str(json_data)?;
    let mut out_content = String::new();

    // Generate Header
    out_content.push_str("//! Generated from OTel Weaver resolved registry. DO NOT EDIT HAND-CODED.\n");
    out_content.push_str("//! Generated at: 2026-06-01T10:10:51-07:00\n\n");
    out_content.push_str("use serde::{Serialize, Deserialize};\n\n");

    // Generate Lattice Trait Definition
    out_content.push_str(r#"/// The process-evidence lattice trait.
pub trait Lattice: Eq + PartialOrd + Serialize {
    fn bottom() -> Self;
    fn top() -> Self;
    fn join(&self, other: &Self) -> Self;
}
"#);

    // Parse resolved groups to build witness structures
    for group in schema.groups {
        if group.id == "process.pi.activity" {
            out_content.push_str("\n/// Witness representing the validation seal of process.pi.activity\n");
            out_content.push_str("#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]\n");
            out_content.push_str("pub struct ActivityWitness {\n");
            
            for attr in &group.attributes {
                let rust_name = attr.id.replace("process.pi.", "").replace(".", "_");
                let rust_type = match attr.attr_type.as_str() {
                    "string" => "String",
                    "int" => "i64",
                    _ => "String",
                };
                out_content.push_str(&format!("    /// {}\n", attr.brief));
                out_content.push_str(&format!("    #[serde(rename = \"{}\")]\n", attr.id));
                out_content.push_str(&format!("    pub {}: {},\n", rust_name, rust_type));
            }
            out_content.push_str("}\n\n");

            // Implement Lattice for ActivityWitness
            out_content.push_str(r#"impl PartialOrd for ActivityWitness {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            Some(std::cmp::Ordering::Equal)
        } else if self.witness_hash.is_empty() && !other.witness_hash.is_empty() {
            Some(std::cmp::Ordering::Less)
        } else if !self.witness_hash.is_empty() && other.witness_hash.is_empty() {
            Some(std::cmp::Ordering::Greater)
        } else {
            None // Incomparable distinct execution paths
        }
    }
}

impl Lattice for ActivityWitness {
    fn bottom() -> Self {
        Self {
            instance_id: String::new(),
            witness_id: String::new(),
            witness_hash: String::new(),
        }
    }

    fn top() -> Self {
        Self {
            instance_id: "TOP".to_string(),
            witness_id: "TOP".to_string(),
            witness_hash: "f".repeat(64),
        }
    }

    fn join(&self, other: &Self) -> Self {
        if self == other {
            return self.clone();
        }
        if self.witness_hash.is_empty() {
            return other.clone();
        }
        if other.witness_hash.is_empty() {
            return self.clone();
        }
        // Conflict resolution: return top to represent contradiction
        Self::top()
    }
}
"#);
        }
    }

    // Ensure output parent directory exists
    let out_path = "/Users/sac/process-intelligence/otel-weaver/experiments/exp-004-registry-to-wasm4pm-compat-witness/src/generated_witnesses.rs";
    if let Some(parent) = std::path::Path::new(out_path).parent() {
        fs::create_dir_all(parent)?;
    }
    
    // Write file
    let mut file = File::create(out_path)?;
    file.write_all(out_content.as_bytes())?;
    println!("Successfully generated witnesses.");
    Ok(())
}
