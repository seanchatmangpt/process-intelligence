# Source Index: sources-wasm4pm-compat

All source files read during TeX file authorship, with one-line descriptions.

| File | Description |
|------|-------------|
| `/Users/sac/process-intelligence/sources/wasm4pm-compat/research-verdict.md` | Version 30.1.2 conformance audit verdict declaring GRADUATION-READY across 5 audit categories, with admissibility axioms, lattice coverage, and doctoral thesis verdict. |
| `/Users/sac/process-intelligence/sources/wasm4pm-compat/type-law-atlas.md` | Version 30.1.1 inventory of the five type-law domains: Evidence container, witness lattice, admission/refusal law, loss policy, and graduation boundary. |
| `/Users/sac/process-intelligence/sources/wasm4pm-compat/witness-lattices.md` | Algebraic witness structure definitions for the bounded join-semilattice over WitnessMarker variants. |
| `/Users/sac/process-intelligence/sources/wasm4pm-compat/manufactured/witnesses/witness-markers-20260601.rs` | Sealed Rust artifact (7,942 bytes, 187 lines) defining WitnessMarker enum, witness_join, witness_leq, and lattice property tests. |
| `/Users/sac/process-intelligence/sources/wasm4pm-compat/manufactured/manifests/manufacturing-manifest-20260601.yaml` | Manufacturing manifest recording 3 sealed artifacts, 3 issued receipts, 3 SPARQL queries (8+15+36 triples), and 3 Tera template renders. |
| `/Users/sac/process-intelligence/sources/wasm4pm-compat/manufactured/boundaries/boundary-law-20260601.wit` | Sealed WIT artifact (9,156 bytes, 256 lines) defining 12 WebAssembly component boundary crossings. |
| `/Users/sac/process-intelligence/sources/wasm4pm-compat/manufactured/forms/process-forms-20260601.md` | Sealed Markdown artifact (8,794 bytes, 293 lines) covering 5 process form type-law definitions. |
| `/Users/sac/process-intelligence/sources/wasm4pm-compat/compat/src/lib.rs` | Public API entry point declaring #![forbid(unsafe_code)] and re-exporting all manufacturing and graduation types. |
| `/Users/sac/process-intelligence/sources/wasm4pm-compat/compat/src/graduation.rs` | GraduationReason enum (7 signals, 5 hard), GraduationCandidate struct, and GraduateToWasm4pm trait definition. |
| `/Users/sac/process-intelligence/sources/wasm4pm-compat/compat/manufacturing/receipt_ledger.rs` | ArtifactReceipt struct and ReceiptLedger HashMap registry with verify_all(), count_by_witness(), and inline tests. |
| `/Users/sac/process-intelligence/sources/wasm4pm-compat/compat/manufacturing/audit.rs` | ComplianceAuditor, AuditResult, AuditLevel, and AuditFinding definitions with test blocks. |
| `/Users/sac/process-intelligence/sources/wasm4pm-compat/compat/Cargo.toml` | Crate manifest for the compat library. |
| `/Users/sac/process-intelligence/sources/wasm4pm-compat/compat/templates/config.toml` | ALIVE gate declaration: 406 compile-pass + 398 compile-fail fixtures. |
| `/Users/sac/process-intelligence/sources/wasm4pm-compat/DELIVERY_CHECKLIST.md` | 2026-05-31 certification of 19 files totaling 3,100+ lines across templates, engines, API, and documentation. |
| `/Users/sac/process-intelligence/sources/wasm4pm-compat/GRADUATION_BOUNDARY_MAP.md` | Formal graduation boundary conditions: what may enter compat and what must never enter. |
| `/Users/sac/process-intelligence/sources/wasm4pm-compat/ADMISSION_REFUSAL_MAP.md` | Boundary control rule documentation for 4 refusal classes under default-deny semantics. |
| `/Users/sac/process-intelligence/sources/wasm4pm-compat/STRUCTURAL_GAPS.md` | Open defect documentation for 6 structural gaps including WfNet split-brain (High) and zero multi-step pipeline fixtures (Medium). |
| `/Users/sac/process-intelligence/sources/wasm4pm-compat/adversarial-type-law.md` | Adversarial type-law test cases and hostile findings. |
| `/Users/sac/process-intelligence/sources/wasm4pm-compat/quantum-witness-lattice.md` | Extended witness lattice definitions exploring quantum-inspired lattice extensions. |
| `/Users/sac/process-intelligence/ggen/wasm4pm-compat.ttl` | Authoritative RDF ontology in Turtle format; source of truth for all manufactured artifacts via SPARQL + Tera rendering. |
