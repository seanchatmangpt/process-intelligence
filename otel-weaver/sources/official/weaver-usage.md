# OpenTelemetry Weaver CLI Usage Guide

The `weaver` command-line utility provides the tools necessary to parse, validate, diff, and generate telemetry schemas. This document details CLI workflows, subcommands, arguments, and exit codes.

---

## 1. Global Flags and Options

These options apply across all `weaver` commands:

*   `--debug`: Enables debug log output. Pass twice (`--debug --debug`) to display trace-level logs (useful for diagnosing parser or resolver errors).
*   `--quiet`: Suppresses logging and diagnostic output except for fatal errors.
*   `--future`: Opts in to the latest (upcoming) semantic convention registry validation rules. This is recommended when checking new custom registries or building against the main branch of `open-telemetry/semantic-conventions`.
*   `-h, --help`: Prints usage help.
*   `-V, --version`: Prints the version number (`weaver 0.22.1`).

---

## 2. Subcommand Reference

### 2.1. `weaver registry check`
Validates a semantic convention registry for syntax, integrity, and policy compliance.

*   **Syntax**:
    ```bash
    weaver registry check [OPTIONS]
    ```
*   **Key Options**:
    *   `-r, --registry <PATH>`: Path to local folder or Git repository of the registry (default: official semantic conventions Git repository).
    *   `-p, --policy <PATH>`: Directory or file path containing custom Rego validation rules (`.rego`).
    *   `--skip-policies`: Bypasses policy checks.
    *   `--v2`: Outputs version 2 of the telemetry schema during internal resolution.
    *   `--diagnostic-format <FORMAT>`: Renders diagnostic output as `ansi`, `json`, or `gh_workflow_command`.
*   **Exit Codes**:
    *   `0`: The registry is completely valid and passes all policy validations.
    *   `1`: Critical parser error, validation rules failed, or syntax mismatch.

### 2.2. `weaver registry generate`
Generates artifacts (code, configuration, markdown) from semantic convention files.

*   **Syntax**:
    ```bash
    weaver registry generate [OPTIONS] [TARGET] [OUTPUT]
    ```
*   **Arguments**:
    *   `[TARGET]`: The code generation target (e.g. `rust`, `go`, `markdown`).
    *   `[OUTPUT]`: Destination folder for generated output (default: `./output`).
*   **Key Options**:
    *   `-t, --templates <PATH>`: Root directory containing template definitions and `weaver.yaml` (default: `./templates`).
    *   `-c, --config <FILE>`: Overriding list of `weaver.yaml` files.
    *   `-D, --param <KEY=VALUE>`: Overriding parameter passed to the template.
    *   `-p, --policy <PATH>`: Policy rules checked prior to code generation.
*   **Exit Codes**:
    *   `0`: Code generation completed successfully.
    *   `1`: Invalid configuration, template rendering engine error, or filesystem write failure.

### 2.3. `weaver registry diff`
Generates a structured differences report comparing two versioned registries.

*   **Syntax**:
    ```bash
    weaver registry diff [OPTIONS] --baseline-registry <BASELINE_REGISTRY>
    ```
*   **Key Options**:
    *   `-r, --registry <PATH>`: The current version of your registry.
    *   `--baseline-registry <PATH>`: Path to the previous version of your registry (required).
    *   `--format <FORMAT>`: Output report format: `ansi`, `json`, or `markdown` (default: `ansi`).
*   **Exit Codes**:
    *   `0`: Comparison completed and report outputted.
    *   `1`: Target or baseline registry files failed to load.

> [!WARNING]
> **Schema Diff vs. Process Drift**:
> `weaver registry diff` measures static discrepancies between two schema definitions (contracts). This represents schema evolution (e.g., deleted metric fields, renamed spans). It does NOT reflect **process drift**, which refers to variations in system runtime telemetry behavior (e.g., changes in frequency, latencies, or paths) compared to a process model.

### 2.4. `weaver registry live-check`
Performs real-time telemetry feedstock checks by matching active OTLP streams or files against a registry schema.

*   **Syntax**:
    ```bash
    weaver registry live-check [OPTIONS]
    ```
*   **Key Options**:
    *   `--input-source <SOURCE>`: Telemetry source: `otlp` (default), `stdin`, or file path.
    *   `--otlp-grpc-port <PORT>`: Port to run the OTLP gRPC listener (default: `4317`).
    *   `--inactivity-timeout <SECONDS>`: Auto-shutdown threshold if no telemetry arrives (default: `10`).
    *   `-o, --output <PATH>`: Save report. Use `none` to output nothing, or `http` to respond to admin stop command.
    *   `--emit-otlp-logs`: Sends policy violations as OTLP logs downstream.
*   **Exit Codes**:
    *   `0`: Clean execution, no critical mismatches found.
    *   `1`: Server failed to bind, or major schema violations detected.

---

## 3. Recommended Workflow Patterns

### Workflow A: Registry Validation in CI/CD
To ensure any modifications to your telemetry contract are valid before merging, add this step in your pipeline:
```bash
# Checks the local registry under ./telemetry-registry/ against policies in ./policies/
weaver registry check \
  --registry ./telemetry-registry \
  --policy ./policies \
  --future
```

### Workflow B: Code and Documentation Generation
To regenerate Go telemetry constants and markdown documentation after schema changes:
```bash
# Generate Go structures into ./pkg/telemetry/
weaver registry generate \
  --registry ./telemetry-registry \
  --templates ./templates/go \
  go ./pkg/telemetry/

# Generate markdown documentation into ./docs/
weaver registry generate \
  --registry ./telemetry-registry \
  --templates ./templates/markdown \
  markdown ./docs/
```

### Workflow C: Schema Evolution Audit (Diff)
Before deploying, compare your schema branch with the production main branch to identify breaking changes:
```bash
weaver registry diff \
  --registry ./telemetry-registry \
  --baseline-registry https://github.com/my-org/telemetry-registry.git@main \
  --format markdown \
  -o ./schema-diff.md
```

### Workflow D: Sandbox Telemetry Verification (Live-Check)
To verify that your microservices emit valid telemetry feedstock during integration testing:
```bash
# Run live-check as an OTLP server, listening for telemetry feedstock
weaver registry live-check \
  --registry ./telemetry-registry \
  --otlp-grpc-port 4317 \
  --inactivity-timeout 30 \
  --output ./live-check-report.yaml
```
If the live check detects missing attributes or incorrect types, it alerts developers before the invalid feedstock propagates to downstream storage, preventing corrupted inputs to the process court.
