# Project: process-intelligence-v30.1.1-alignment

## Architecture
- The research corpus represents the authority layer for studying process-evidence type law.
- The structure consists of doctrine definitions, public standard mapping, academic paper mappings, lifecycle transitions, M&A diligence claim projections, and sample/fixture validations.
- Interaction flows from theoretical papers/standards to execution structures, verified via cryptographic receipts.

## Milestones
| # | Name | Scope | Dependencies | Status |
|---|------|-------|-------------|--------|
| 1 | M1: Adversarial Audit | Scan doctrine, sources, standards, lifecycle, ma, and experiments for stubs and weak math | none | DONE |
| 2 | M2: M&A Diligence Rigor | Update slide-to-receipt maps, cost minimization, and signature verification with JSON schemas and math in `ma/define_slide-to-receipt_map.md`, `ma/define_auditor_evidence_path.md`, `ma/define_slide-to-replay_map.md`, and `ma/slide-to-receipt-map.md` | M1 | IN_PROGRESS |
| 3 | M3: Petri Net & OCPQ Rigor | Formalize Petri Net place/transition dynamics, stochastic nets, and OCPQ query bindings in `standards/petri_net_placement.md`, `standards/petri-net.md`, and `standards/ocpq_placement.md` using LaTeX | M1 | IN_PROGRESS |
| 4 | M4: Fixture & Link Upgrades | Review and optimize validation scripts/fixtures in `experiments/` and `audits/`, fix broken links in `standards/reverse-lock-in.md` and checkpoints | M2, M3 | PLANNED |
| 5 | M5: Final Report & Integration | Resolve `INCOMPLETE` markers in `sources/wasm4pm-compat/`, compile final audit report at `audits/adversarial_audit_v30.1.1.md`, and run Forensic Audit | M4 | PLANNED |

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
