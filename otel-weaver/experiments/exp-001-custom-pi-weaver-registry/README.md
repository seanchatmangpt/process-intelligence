# Experiment EXP-001: Custom Process Intelligence OpenTelemetry Semantic Conventions Registry

This experiment defines and registers custom OpenTelemetry (OTel) semantic conventions for Process Intelligence (PI) using OTel Weaver. The goal is to establish a rigorous, standardized format for process execution telemetry (feedstock) before it is passed to the process mining engine (court) for conformance checking.

## 1. Ontological Foundations

Nominal categories must never collapse:
* **Telemetry is feedstock:** It represents the raw, descriptive signals emitted by the execution layer (traces, spans, events, and metrics). It lacks process consequence.
* **Process consequence is court:** It is the verification layer (`wasm4pm` / `wasm4pm-compat`) where execution is judged against process models, LTL rules, and type-bound invariants.
* **Weaver diffs are not process drift:** Weaver diffs track schema migrations at design-time. Process drift tracks real-world runtime behavior departing from process models.

This experiment implements the schema definitions for the telemetry feedstock.

---

## 2. Process Intelligence Semantic Conventions Schema

Below is the complete OTel semantic convention definition (`process_pi.yaml`) compiled by Weaver to generate process-aware telemetry instrumentation libraries.

```yaml
# file:///Users/sac/process-intelligence/otel-weaver/experiments/exp-001-custom-pi-weaver-registry/semconv/process_pi.yaml
file_format: 1.2.0
schema_url: https://opentelemetry.io/schemas/1.25.0
groups:
  - id: process.pi.activity
    type: span
    brief: "Represents a discrete execution activity within a process instance."
    note: "This span must be emitted for every transition or execution task in the process topology."
    attributes:
      - id: process.pi.instance_id
        type: string
        brief: "The unique identifier of the process execution instance (case ID)."
        examples: ["inst-8874f-99bc2-3312a"]
        requirement_level: required

      - id: process.pi.activity.name
        type: string
        brief: "The name of the process activity corresponding to a Petri net transition or BPMN task."
        examples: ["approve_invoice", "receive_payment", "validate_admission"]
        requirement_level: required

      - id: process.pi.activity.type
        type: string
        brief: "The architectural category of the activity."
        examples: ["task", "subprocess", "gate", "message_event"]
        requirement_level: recommended

      - id: process.pi.lifecycle
        type: string
        brief: "The transactional lifecycle phase of the activity."
        examples: ["schedule", "start", "suspend", "resume", "complete", "abort"]
        requirement_level: required

      - id: process.pi.token.state_before
        type: string
        brief: "JSON-serialized map of Petri net place markings before the activity transition fires."
        examples: ["{\"p_start\": 1, \"p_approved\": 0}"]
        requirement_level: recommended

      - id: process.pi.token.state_after
        type: string
        brief: "JSON-serialized map of Petri net place markings after the activity transition fires."
        examples: ["{\"p_start\": 0, \"p_approved\": 1}"]
        requirement_level: recommended

      - id: process.pi.witness.id
        type: string
        brief: "The identifier of the witness node or agent asserting the transition."
        examples: ["heuristics_miner_v3", "auth_governor_alpha"]
        requirement_level: required

      - id: process.pi.witness.hash
        type: string
        brief: "BLAKE3 cryptographic hash sealing this specific transition trace."
        examples: ["4a98f1c8b210e309228d4bf09c0fa4db55fe310efc35ad1a89b3f021e85a6111"]
        requirement_level: required
```

---

## 3. Resolving the Registry with OTel Weaver

To resolve these semantic conventions, map them into a coherent registry, and check for conflicts against the core OpenTelemetry semantic conventions, we invoke the `weaver` CLI.

### Step 1: Initialize the Registry
```bash
weaver registry init --registry-dir /Users/sac/process-intelligence/otel-weaver/registry
```

### Step 2: Add Custom Semantic Conventions
We link our custom schema to the registry:
```bash
cp /Users/sac/process-intelligence/otel-weaver/experiments/exp-001-custom-pi-weaver-registry/semconv/process_pi.yaml \
   /Users/sac/process-intelligence/otel-weaver/registry/semconv/process_pi.yaml
```

### Step 3: Compile and Resolve the Schema
Using `weaver` to compile, resolve, and generate the target telemetry metadata:
```bash
weaver registry resolve \
  --registry /Users/sac/process-intelligence/otel-weaver/registry \
  --output /Users/sac/process-intelligence/otel-weaver/registry/resolved_schema.json
```

This output is a fully resolved, flat semantic conventions schema, resolving any inheritance or common attribute references.

---

## 4. Verification Check

To verify that the custom semantic conventions compile without violations, run the following verification:

```bash
weaver registry validate \
  --registry /Users/sac/process-intelligence/otel-weaver/registry
```

**Validation Assertion Matrix:**
* **Structure:** The resolved schema contains the `process.pi.instance_id` and `process.pi.witness.hash` fields.
* **Requirement Enforcements:** Any span with `process.pi.activity` MUST contain a valid BLAKE3 hash matching `process.pi.witness.hash` and a non-empty `process.pi.instance_id`.
* **Zero Collapses:** The schema attributes map strictly to the telemetry domain, containing no process conformance calculations (consequences).

---

## 5. Artifact Reference Links

* [Custom YAML SemConv](file:///Users/sac/process-intelligence/otel-weaver/experiments/exp-001-custom-pi-weaver-registry/semconv/process_pi.yaml)
* [Parent Experiment Directory](file:///Users/sac/process-intelligence/otel-weaver/experiments/)
* [Checkpoints Registry](file:///Users/sac/process-intelligence/checkpoints/)
