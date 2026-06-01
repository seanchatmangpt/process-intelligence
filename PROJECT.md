# Project: process-intelligence-v30.1.1-alignment

## Architecture
- The research corpus represents the authority layer for studying process-evidence type law.
- The structure consists of doctrine definitions, public standard mapping, academic paper mappings, lifecycle transitions, M&A diligence claim projections, and sample/fixture validations.
- Interaction flows from theoretical papers/standards to execution structures, verified via cryptographic receipts.

## Milestones
| # | Name | Scope | Dependencies | Status |
|---|------|-------|-------------|--------|
| 1 | M1: Adversarial Audit | Scan doctrine, sources, standards, lifecycle, ma, and experiments for stubs and weak math | none | IN_PROGRESS |
| 2 | M2: M&A Diligence Rigor | Update slide-to-receipt maps and diligence slide definitions with JSON schemas and verification math | M1 | PLANNED |
| 3 | M3: Petri Net & OCPQ Rigor | Formalize Petri Net place/transition dynamics and OCPQ bindings in standards/ and lifecycle/ using LaTeX | M1 | PLANNED |
| 4 | M4: Fixture Upgrades | Review and optimize validation scripts/fixtures in experiments/ and audits/ | M2, M3 | PLANNED |
| 5 | M5: Final Report & Integration | Align all files to v30.1.1, compile the final audit report, and run Forensic Audit | M4 | PLANNED |

## Interface Contracts
### slide-to-receipt ↔ verification math
- Diligence claims must correspond to a schema defining inputs, output cryptographic hash, and signers.
- Verification math must specify SHA-256/BLAKE3 hash checks and Ed25519 signature checks.

### Petri Net place/transition ↔ OCPQ query
- Petri Net dynamics must be specified in terms of place-transition flow matrices and marking transitions.
- OCPQ query bindings must specify patterns/rules to select and project events into the Petri Net.

## Code Layout
- `doctrine/` - Process law foundations
- `ma/` - Diligence claims, slide-to-receipt maps, and taxonomies
- `standards/` - Place of Petri Net and OCPQ rules, public schemas
- `sources/` - Research mappings (papers, pm4py, wasm4pm, wasm4pm-compat)
- `lifecycle/` - Transition laws and MAPE-K configurations
- `experiments/` & `audits/` - Validation fixtures and correctness tests
