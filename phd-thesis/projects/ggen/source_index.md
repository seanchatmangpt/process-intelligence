# Source Index: ggen Thesis Files

All source files read during thesis manufacture for the ggen project.

| File | Description |
|------|-------------|
| `/Users/sac/process-intelligence/ggen/ggen.toml` | Generation rule manifest: wires SPARQL queries to Tera templates to output files; declares template engine config (strict_variables=true, autoescape=true), BLAKE3 receipt format, and in-memory SPARQL endpoint |
| `/Users/sac/process-intelligence/ggen/README.md` | Architecture overview: three-layer pipeline description, SPARQL query documentation, Tera template slide/sheet structure, board admissibility contract four pillars, usage examples |
| `/Users/sac/process-intelligence/ggen/GGEN_MANUFACTURING_SUMMARY.md` | Phase 3 manufacturing summary: 17 artifact inventory (5 rules, 5 templates, 7 audits), authority chain diagram, compliance standards, status MANUFACTURED, PROCESS_INTELLIGENCE_ALIVE_001 pending |
| `/Users/sac/process-intelligence/ggen/audit.json` | Audit record: ggen_version 26.5.21, generated_at 2026-06-01T20:38:53Z, validation_passed: true, empty inputs/pipeline/outputs fields |
| `/Users/sac/process-intelligence/ggen/.ggen/receipts/sync-20260601-175316.json` | Chain-root BLAKE3 receipt: operation de2ad19f, blue_river_dam/src/lib.rs output hash 4e341f5a, previous_receipt_hash null |
| `/Users/sac/process-intelligence/ggen/.ggen/receipts/sync-20260601-230600.json` | Third (latest) BLAKE3 receipt: operation 09084e1e, previous_receipt_hash 057521e5, output_hashes empty |
| `/Users/sac/process-intelligence/ggen/templates/blue-river.tera` | Tera template manufacturing BlueRiverOrchestrator Rust source: LifecycleState enum, MAPE-K method stubs, embedded test suite |
| `/Users/sac/process-intelligence/ggen/templates/witness-marker.tera` | Tera template manufacturing WitnessMarker Rust enum: eight lattice positions VanDerAalst1989 to BlueRiverDam, join-semilattice algebraic test proofs |
| `/Users/sac/process-intelligence/ggen/rules/feature-law.yaml` | Six Cargo feature definitions (default, ts, wasm, component, strict, wasm4pm), tool-smuggling doctrine, ERR_TOOL_SMUGGLING_INTO_COMPAT refusal code, feature unification effects |
| `/Users/sac/process-intelligence/ggen/queries/extract-board-claims.rq` | SPARQL SELECT query: five claim type FILTER, fitness>=0.95 and precision>=0.90 FILTER, log format FILTER (ocel:2.0 or xes:1849-2016), ORDER BY DESC fitness |
