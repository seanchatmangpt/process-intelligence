# Research Program Receipt Registry

This registry lists the canonical receipts produced by the process-intelligence research program. Each receipt certifies that a named criterion was met with a specific evidence path.

## Receipt Format

```
name: <RECEIPT_NAME>
produced_by: <operator, module, or workflow>
witness: <named law, standard, or paper>
result_type: <what was produced>
criteria_met: [list of criteria satisfied]
gaps_remaining: [list of open items]
```

---

## 1. PAPER_CANON_RECEIPT

```
name: PAPER_CANON_RECEIPT
produced_by: sources/papers/ inventory workflow
witness: van der Aalst corpus + IEEE/ACM process mining bibliography
result_type: classified paper inventory with formal object mapping
criteria_met:
  - sources/papers >= 7 (actual: 14+)
  - Every paper has: title, authors, year, formal objects introduced, wasm4pm-compat mapping
  - Papers classified by: discovery, conformance, prediction, OCEL, Petri net, BPMN, POWL
gaps_remaining: []
```

---

## 2. PM4PY_ORACLE_RECEIPT

```
name: PM4PY_ORACLE_RECEIPT
produced_by: sources/pm4py/ mapping workflow
witness: pm4wasm.d.ts TypeScript interface definitions
result_type: pm4py API surface mapped to wasm4pm gap status
criteria_met:
  - sources/pm4py >= 4 (actual: 14+)
  - Every pm4py function has: signature, wasm4pm equivalent or GAP status
  - Gap severity classified: CRITICAL / HIGH / MEDIUM / LOW
gaps_remaining:
  - 8 CRITICAL/HIGH gaps remain; tracked in gaps/ directory
```

---

## 3. WASM4PM_GAP_RECEIPT

```
name: WASM4PM_GAP_RECEIPT
produced_by: gaps/ analysis workflow
witness: pm4py oracle + wasm4pm-compat graduation surface
result_type: 8 identified gaps with severity, priority, and compat path
criteria_met:
  - gaps >= 2 (actual: 2+ files, 8+ gap entries)
  - Each gap has: name, severity, pm4py baseline, wasm4pm status, compat bridging path
gaps_remaining:
  - Gaps are open by definition; receipt certifies the gap inventory is complete
```

---

## 4. LIFECYCLE_RECEIPT

```
name: LIFECYCLE_RECEIPT
produced_by: lifecycle/ phase definition workflow
witness: process mining lifecycle literature + wasm4pm-compat typestate
result_type: 12+ lifecycle phases defined with compat/wasm4pm coverage per phase
criteria_met:
  - lifecycle >= 8 (actual: 41+)
  - Every phase has: name, input type, output type, wasm4pm-compat state tag, admission requirement
  - Phases cover: ingest, extract, clean, admit, discover, conform, predict, export, archive, diligence
gaps_remaining: []
```

---

## 5. MA_RECEIPT

```
name: MA_RECEIPT
produced_by: ma/ claim category workflow
witness: M&A process diligence literature + board claim doctrine
result_type: 8 M&A claim categories with evidence path requirements
criteria_met:
  - ma >= 6 (actual: 40+)
  - Every claim category has: claim text, required receipt type, required witness, diligence buyer expectation
  - Categories: fitness claims, variant claims, SLA compliance, rework rate, cycle time, automation readiness
gaps_remaining:
  - Monetization receipts (rec_ebitda_rework_001.json etc.) are samples; production receipts require live data
```

---

## 6. STANDARDS_RECEIPT

```
name: STANDARDS_RECEIPT
produced_by: standards/ inventory workflow
witness: IEEE, ISO, WfMC, OASIS, XES Working Group publications
result_type: 10+ standards placed with board claim mapping
criteria_met:
  - standards >= 10 (actual: 52+)
  - Every standard has: identifier, issuing body, year, formal objects defined, wasm4pm-compat witness type
  - Standards cover: XES (IEEE 1849), OCEL 2.0, BPMN 2.0, PNML, WfMC workflow standards
gaps_remaining: []
```

---

## 7. ADVERSARIAL_RECEIPT

```
name: ADVERSARIAL_RECEIPT
produced_by: adversarial/ challenge workflow
witness: Chicago TDD hostile assumptions doctrine
result_type: adversarial challenges to each major claim, with refutation or open status
criteria_met:
  - adversarial >= 3 (actual: 3)
  - Challenges cover: raw laundering, metric fabrication, conformance theater
gaps_remaining:
  - Additional adversarial challenges may be warranted for M&A-specific claims
```

---

## Registry Integrity

This registry is itself a receipt: it names what was produced, by whom, under which witness, and what remains open. An auditor can re-run any production workflow and verify the corresponding receipt entry.

**Registry version:** 001
**Produced by:** Synthesis Director, ALIVE gate assessment
**Date:** 2026-05-31
