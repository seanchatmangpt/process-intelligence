# Workflow Papers Corpus Inventory

**Inventory Date:** 2026-05-31
**Source Directory:** ~/Documents/Papers/workflow
**Total PDFs found:** 20
**Additional sources:** ~/Documents/Papers/AI_LLM (1 PM paper), ~/Documents/Papers/RDF (3 papers, 0 PM-relevant)
**Note:** Corpus is YAWL-heavy (van der Aalst, ter Hofstede era) with OCEL/XES standards, workflow patterns, and process mining tools.

---

## Complete Inventory: ~/Documents/Papers/workflow (20 papers)

| # | Filename | Identified Title | Year | Authors | Domain |
|---|---|---|---|---|---|
| 1 | `Compliance-Aware Predictive Process Monitoring- A Neuro-Symbolic Approach .pdf` | Compliance-Aware Predictive Process Monitoring: A Neuro-Symbolic Approach | 2026 | De Santis, Park, van der Aalst, Zanichelli | Predictive PM / Neuro-symbolic |
| 2 | `HANDSON_PYTHON_FOR_DEVOPS.pdf` | Hands-On Python for DevOps | 2024 | Ankur Roy | DevOps (out of scope) |
| 3 | `Hierarchical Decomposition of Separable Workflow-Nets .pdf` | Hierarchical Decomposition of Separable Workflow-Nets | 2026 | Kourani, Park, van der Aalst | POWL 2.0 / WF-net |
| 4 | `How-Anthropic-teams-use-Claude-Code_v2.pdf` | How Anthropic Teams Use Claude Code | 2024 | Anthropic | Tool report (out of scope) |
| 5 | `Object-Centric Analysis of XES Event Logs- Integrating OCED Modeling with SPARQL Queries .pdf` | Object-Centric Analysis of XES Event Logs: Integrating OCED Modeling with SPARQL Queries | 2025 | Latif, Latif, Rahman | XES / OCED / SPARQL |
| 6 | `OCPQ- Object-Centric Process Querying & Constraints .pdf` | OCPQ: Object-Centric Process Querying & Constraints | 2025 | Küsters, van der Aalst | Object-centric querying |
| 7 | `PM4Py Software Impacts.pdf` | PM4Py: A process mining library for Python (Software Impacts) | 2023 | Berti, van Zelst, Schuster | Process mining library |
| 8 | `PM4Py- A process mining library for Python.pdf` | PM4Py: A process mining library for Python | 2023 | Berti, van Zelst, Schuster | Process mining library (duplicate) |
| 9 | `PMAx- An Agentic Framework for AI-Driven Process Mining .pdf` | PMAx: An Agentic Framework for AI-Driven Process Mining | 2026 | Antonov, Kourani, Berti, Park, van der Aalst | Agentic PM framework |
| 10 | `Process mining for healthcare- Characteristics and challenges .pdf` | Process mining for healthcare: Characteristics and challenges | 2022 | Munoz-Gama, Martin, Fernandez-Llatas et al. | Domain application survey |
| 11 | `Real-Life BPMN - edition 4.pdf` | Real-Life BPMN (4th edition) | 2019 | Freund, Rücker | BPMN notation / practice |
| 12 | `sAirflow- Adopting Serverless in a Legacy Workflow Scheduler .pdf` | sAirflow: Adopting Serverless in a Legacy Workflow Scheduler | 2024 | Mikina, Zuk, Rzadca | HPC/serverless (out of scope) |
| 13 | `Why Automate This? Exploring the Connection between Time Use, Well-being and Robot Automation Across Social Groups.pdf` | Why Automate This? Exploring the Connection between Time Use, Well-being and Robot Automation | 2025 | Ray, Pang, Srivastava et al. | HRI/sociology (out of scope) |
| 14 | `workflow-patterns-the-definitive-guide-9780262029827-9780262329408-0262329409_compress.pdf` | Workflow Patterns: The Definitive Guide | 2016 | Russell, van der Aalst, ter Hofstede | Workflow patterns |
| 15 | `Workflows Community Summit 2024- Future Trends and Challenges in Scientific Workflows  .pdf` | Workflows Community Summit 2024: Future Trends and Challenges in Scientific Workflows | 2024 | Ferreira da Silva et al. (111 authors) | Scientific/HPC workflows (out of scope) |
| 16 | `YAWL - Technical Manual.pdf` | YAWL Technical Manual (Version 5) | 2023 | The YAWL Foundation | YAWL engine manual |
| 17 | `YAWL_An_open_source_Business_Process_Management_Sy.pdf` | YAWL: An open source Business Process Management System from science for science | 2020 | Adams, Hense, ter Hofstede | YAWL BPMS |
| 18 | `YAWL- Yet Another Workflow Language.pdf` | YAWL: Yet Another Workflow Language (Revised version) | 2004 | van der Aalst, ter Hofstede | WF-net soundness / YAWL lang |
| 19 | `YAWLTechnicalManual5.0.pdf` | YAWL Technical Manual Version 5.0 | 2023 | The YAWL Foundation | YAWL manual (duplicate of #16) |
| 20 | `YAWLUserManual5.1.pdf` | YAWL User Manual Version 5.1 | 2024 | The YAWL Foundation | YAWL user manual (out of scope) |

---

## Additional Paper: ~/Documents/Papers/AI_LLM

| # | Filename | Title | Year | Authors | Domain |
|---|---|---|---|---|---|
| 21 | `No AI Without PI! Object-Centric Process Mining as the Enabler for Generative, Predictive, and Prescriptive Artificial Intelligence .pdf` | No AI Without PI! Object-Centric Process Mining as the Enabler for Generative, Predictive, and Prescriptive AI | 2025 | van der Aalst | OCPM / AI boundary |

---

## Corpus Character Notes

### YAWL-heavy cluster
Papers 16, 17, 18, 19, 20 all concern YAWL. Paper 18 (van der Aalst & ter Hofstede 2004) is the foundational language reference. Papers 16/19 are duplicates (YAWL Technical Manual v5). Paper 20 is a user manual. Paper 17 is a system description that overlaps with 16 and 18.

### Process mining tools cluster
Papers 7 and 8 are the same PM4Py paper in two journal editions. Paper 9 (PMAx) builds on PM4Py for agentic orchestration.

### Object-centric cluster
Papers 5, 6, 21 represent the OCEL/OCED/OCPQ lineage that drives the wasm4pm-compat canon.

### Out-of-scope papers (4 confirmed)
Papers 2 (DevOps), 4 (Anthropic tool report), 12 (serverless HPC), 13 (HRI/sociology), 15 (scientific workflows) have no process-mining type-law relevance.

---

## Extended Corpus (tracked in wasm4pm-compat PAPER_COVERAGE_LEDGER.md)

The PAPER_COVERAGE_LEDGER.md tracks 81 papers total (rows #1–#82, one duplicate slot).
The 20 physical PDFs in ~/Documents/Papers/workflow correspond to rows #1–#21 in the ledger.
Rows #22–#82 are canonical PM papers referenced by the ledger but not present as local PDFs.

**Key canonical papers tracked but not local PDFs include:**
- van der Aalst (1998) WF-net soundness [#34]
- Murata (1989) Petri Nets [#33]
- van der Aalst (2013) OCEL 1.0 [#35], OCEL 2.0 spec [#25], XES IEEE 1849-2023 [#26]
- Declare/LTL (Pesic & van der Aalst 2006) [#28]
- POWL (Kourani & van der Aalst) [#43]
- Alpha Miner (2004) [#29], Inductive Miner (2013) [#27]
- Conformance alignments (2011) [#32]
- OC-Petri Nets (2019) [#31]

Total corpus assessed: 81 unique papers.
