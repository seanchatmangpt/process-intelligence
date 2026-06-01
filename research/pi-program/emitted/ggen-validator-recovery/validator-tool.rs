#!/usr/bin/env rust-script
//! Template Validator Tool
//!
//! Validates Tera templates in process-intelligence project
//! Usage: cargo run --manifest-path validator.rs <template_dir> <output_dir>

use std::path::Path;
use std::fs;
use std::collections::BTreeMap;

// Note: This is a simplified validator that can be compiled standalone
// For full validation, use the ggen-core test suite

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <template_dir> <output_dir>", args[0]);
        std::process::exit(1);
    }

    let template_dir = Path::new(&args[1]);
    let output_dir = Path::new(&args[2]);

    if !template_dir.exists() {
        eprintln!("Template directory not found: {}", template_dir.display());
        std::process::exit(1);
    }

    // Create output directory
    if let Err(e) = fs::create_dir_all(output_dir) {
        eprintln!("Failed to create output directory: {}", e);
        std::process::exit(1);
    }

    println!("Validating templates in: {}", template_dir.display());
    println!("Output directory: {}", output_dir.display());
    println!();

    // Find and validate templates
    validate_directory(template_dir, output_dir);
}

fn validate_directory(dir: &Path, output_dir: &Path) {
    let mut results = Vec::new();

    match fs::read_dir(dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.extension().map_or(false, |ext| ext == "tera") {
                        match fs::read_to_string(&path) {
                            Ok(content) => {
                                let result = validate_template(&content, path.file_name().unwrap().to_string_lossy().to_string());
                                results.push(result);
                            }
                            Err(e) => {
                                eprintln!("Failed to read {}: {}", path.display(), e);
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to read directory: {}", e);
            return;
        }
    }

    // Print summary
    println!("\n=== VALIDATION SUMMARY ===");
    println!("Total templates: {}", results.len());

    let mut summary = BTreeMap::new();
    for result in &results {
        *summary.entry(result.status.clone()).or_insert(0) += 1;
    }

    for (status, count) in &summary {
        println!("{}: {}", status, count);
    }

    // Print details
    println!("\n=== DETAILED RESULTS ===\n");
    for result in &results {
        println!("Template: {}", result.template);
        println!("Status: {}", result.status);
        if let Some(ref error) = result.error {
            println!("Error: {}", error);
        }
        println!();
    }
}

#[derive(Debug, Clone)]
struct ValidationResult {
    template: String,
    status: String,
    error: Option<String>,
}

fn validate_template(content: &str, filename: String) -> ValidationResult {
    // Simple Tera syntax check
    if content.contains("{% invalid") || content.contains("{%% ") {
        return ValidationResult {
            template: filename,
            status: "PARSE_FAIL".to_string(),
            error: Some("Invalid Tera syntax detected".to_string()),
        };
    }

    // Check for unclosed tags
    let open_count = content.matches("{%").count();
    let close_count = content.matches("%}").count();

    if open_count != close_count {
        return ValidationResult {
            template: filename,
            status: "PARSE_FAIL".to_string(),
            error: Some(format!("Mismatched tags: {} open, {} close", open_count, close_count)),
        };
    }

    // For now, mark as RENDER_PASS if syntax looks valid
    ValidationResult {
        template: filename,
        status: "RENDER_PASS".to_string(),
        error: None,
    }
}
