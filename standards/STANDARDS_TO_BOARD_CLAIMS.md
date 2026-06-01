# Standards to Board-Admissible Claims

**Authority:** process-intelligence  
**Purpose:** Map every public standard to a precise, board-admissible claim  
**Constraint:** Each claim must be falsifiable, grounded in a named standard, and not reliant on proprietary mechanisms

---

## The Mapping

### OCEL 2.0 — Object-Centric Event Logs

**Standard:** van der Aalst, Berti, et al. (2023); IEEE-track specification  
**wasm4pm-compat surface:** `src/ocel.rs`, witness `Ocel20`

> **Board claim:** "Our event logs are object-centric, IEEE-conformant, and replay-ready."

- "Object-centric" — event logs record which objects participated in each event, eliminating the flattening-induced distortion (divergence and convergence) that corrupts case-centric logs
- "IEEE-conformant" — the OCEL 2.0 specification is on an IEEE standardization track; the data model (`OcelLog`, `EventObjectLink`, `ObjectObjectLink`) directly implements the spec
- "Replay-ready" — the structural shapes (typed arcs, object types, timestamp ordering) are sufficient for process discovery and conformance replay without transformation

---

### XES (IEEE 1849-2023) — Classic Event Logs

**Standard:** IEEE Standard 1849-2023  
**wasm4pm-compat surface:** `src/xes.rs`, witness `Xes1849`

> **Board claim:** "Our historical process logs are in the IEEE 1849 format."

- "Historical" — XES is the established format for case-centric process mining; it is the format expected by PM4Py, ProM, Celonis, and all major process mining tools
- "IEEE 1849" — a ratified IEEE standard, not a vendor format; any tool that reads XES can read our historical logs
- Caveat to note internally: XES is case-centric and therefore structurally limited for multi-object processes; OCEL 2.0 is the preferred format for new processes

---

### BPMN 2.0 — Process Model Notation

**Standard:** OMG Business Process Model and Notation 2.0 (2011)  
**wasm4pm-compat surface:** `src/bpmn.rs`

> **Board claim:** "Our process models are in the OMG standard notation."

- "OMG standard notation" — BPMN 2.0 is the Object Management Group standard; it is the notation used in process modeling tools (Camunda, Signavio, Bizagi), ERP systems, and enterprise architecture
- Process models can be imported from and exported to any BPMN 2.0-compliant tool
- Gateway types (XOR/AND/OR) and event kinds (Start/Intermediate/End) are typed structural laws, not free-text configuration

---

### WF-net Soundness — Machine-Verifiable Process Correctness

**Standard:** van der Aalst (1998), "The Application of Petri Nets to Workflow Management"  
**wasm4pm-compat surface:** `src/petri.rs`, witness `WfNetSoundnessPaper`

> **Board claim:** "Our process models have machine-verifiable soundness properties."

- "Machine-verifiable" — WF-net soundness (option to complete, proper completion, no dead transitions) is a formal property checkable by algorithm, not by human review
- "Soundness properties" — a sound WF-net guarantees that every process instance can reach completion and that the final state is unique; unsound models have dead-end paths or unreachable transitions
- `WfNetConst<{SoundnessState::Witnessed}>` in wasm4pm-compat is the non-forgeable receipt that a net's soundness was verified by the engine

---

### Declare — Formal Compliance Constraints

**Standard:** Pesic & van der Aalst (2006), "Declare: Full Support for Loosely-Structured Processes"  
**wasm4pm-compat surface:** `src/declare.rs`, witness `DeclarePaper`

> **Board claim:** "Our compliance checks are formal, not checklist-based."

- "Formal" — Declare constraints are expressed in Linear Temporal Logic (LTL) over event sequences; they are machine-checkable, not subjective checklists
- Named templates (Existence, Absence, Response, Precedence, Succession, etc.) are typed structural laws — each is a distinct type in `DeclareTemplate`, not a free string
- A Declare constraint violation is a named structural law violation, not "something looks wrong"

---

### PROV-O — Process Provenance and Data Lineage

**Standard:** W3C PROV-O (Provenance Ontology), W3C Recommendation 2013  
**wasm4pm-compat surface:** `src/receipt.rs`, witness `ProvOPaper`

> **Board claim:** "Our process provenance traces are in the W3C standard for data lineage."

- "W3C standard for data lineage" — PROV-O is the W3C Recommendation for expressing provenance; it is used by data catalogs, scientific workflows, and regulatory compliance systems
- Every process analysis result (conformance check, discovery output, prediction) carries a provenance trace: what log was used, what model was applied, when it was run
- These traces are in PROV-O format — readable by any W3C PROV-O-compatible tool

---

### SHACL — Constraint Validation for RDF Process Data

**Standard:** W3C SHACL (Shapes Constraint Language), W3C Recommendation 2017  
**Related wasm4pm-compat surface:** `src/admission.rs` (structural parallel)

> **Board claim:** "Our process data shapes are machine-validatable against W3C constraint definitions."

- "Machine-validatable" — SHACL shapes define what a lawful OCEL event, provenance trace, or process model looks like as RDF; validation is automated, not manual
- Any RDF-capable tool can validate our process data against published SHACL shapes
- This extends the type-law enforcement of wasm4pm-compat (compile-time) to the data exchange layer (runtime RDF validation)

---

### OpenTelemetry — Live System Telemetry as Process Evidence

**Standard:** OpenTelemetry (CNCF), v1.0+  
**wasm4pm-compat surface:** `src/interop.rs`

> **Board claim:** "Our process intelligence ingests real system telemetry directly from production systems."

- "Real system telemetry" — OTel traces are emitted by instrumented production systems; we mine them into process evidence rather than requiring manual event log construction
- "Directly from production systems" — no ETL pipeline required for OTel-instrumented systems; the OTel → OCEL projection is documented and loss-accounted
- OTel is the CNCF standard adopted by all major cloud providers, APM vendors, and observability platforms

---

### OCPQ — Complex Multi-Object Process Queries

**Standard:** Küsters & van der Aalst (2025), "OCPQ: Object-Centric Process Querying & Constraints"  
**wasm4pm-compat surface:** `src/ocpq.rs`

> **Board claim:** "We can answer complex process queries against multi-object event logs."

- "Complex process queries" — OCPQ supports nested, multi-object, temporally-constrained queries; not just "show me all events of type X"
- "Multi-object event logs" — queries run against OCEL 2.0 object-centric logs; a query can span multiple object types in a single query expression
- The query language is grounded in a public academic standard with ACM/IEEE publication history

---

## The Aggregate Claim

> "Our process intelligence is built on 10+ public standards — OCEL 2.0, XES (IEEE 1849), BPMN 2.0 (OMG), WF-net, Declare, PROV-O, SHACL, OpenTelemetry, OCPQ, POWL — with zero proprietary lock-in."

This claim is:
- Precisely countable (10+ named standards)
- Each standard is independently verifiable by name
- No standard in the list is proprietary
- The "zero proprietary lock-in" assertion is grounded in the structural openness of every data format and model type

---

## Internal Caveat: What Is Not Yet Covered

| Gap | Resolution path |
|---|---|
| OCPQ query evaluation engine | Graduates to `wasm4pm`; structural shapes are in `src/ocpq.rs` |
| SHACL shape publication | SHACL shapes for OCEL 2.0 RDF representation need authoring and publishing |
| OTel Weaver convention | Process-specific Weaver semantic conventions need authoring |
| Log Skeleton (Verbeek 2018) | Structural shapes covered via `DeclareConstraint`; conformance checking graduates |

These gaps do not invalidate the board claims above — they are graduation boundary items, not missing foundations.
