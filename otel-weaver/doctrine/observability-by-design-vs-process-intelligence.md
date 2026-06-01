# observability-by-design-vs-process-intelligence.md

**Authority:** `/Users/sac/process-intelligence/otel-weaver/doctrine`  
**Status:** ALIVE — anchored at OBSERVABILITY_VS_PROCESS_INTELLIGENCE_001  

---

## The Dual-Core Paradigm

To design high-assurance systems, we must recognize two separate disciplines: system observability and process intelligence. 

While they share a dependency on system telemetry, their objectives, methods, and formal foundations are distinct.

> **Observability-by-design ensures the software is readable; Process Intelligence ensures the business is lawful.**

- **Observability-by-Design** is an engineering practice. It embeds structured logging, metric counters, and trace context propagation directly into code to make the internal state of software inferable from its outputs.
- **Process Intelligence** is a governance practice. It uses formal process models, execution replay, and mathematical conformance checking to discover, audit, and actuate the lifecycle of business processes.

---

## Observability-by-Design: The Engineering View

Observability-by-design is concerned with the operational health of software components. It utilizes the OpenTelemetry standard and Weaver conventions to ensure consistency.

- **Primary Questions**: 
  - *What is the p99 latency of the payment service?*
  - *Are memory leak trends visible in the container metrics?*
  - *Where is the bottleneck in the microservice dependency graph?*
- **Core Entities**: Traces, Spans, Metrics, Logs, Resources, Baggage.
- **Formulas/Metrics**: Apdex score, CPU utilization, throughput (req/sec), network packet loss.
- **Goal**: Maintain system availability, reliability, and performance.

---

## Process Intelligence: The Outcome View

Process intelligence is concerned with the execution of business processes. It views software merely as one of several execution mediums.

- **Primary Questions**:
  - *Does our actual order fulfillment flow conform to our documented compliance model?*
  - *Are inventory adjustments always preceded by a signed audit log?*
  - *What is the financial cost and bottleneck impact of manual interventions in our automated supply chain?*
- **Core Entities**: Event logs (OCEL/XES), Petri Nets, BPMN models, Process Trees, Receipts, Witness markers.
- **Formulas/Metrics**: Fitness ($f$), Precision ($p$), Generalization ($g$), Conformance rate, cycle time.
- **Goal**: Guarantee compliance, auditability, operational efficiency, and legal consequence.

---

## Comparison Matrix

| Property | Observability-by-Design | Process Intelligence |
| :--- | :--- | :--- |
| **User Base** | DevOps, Site Reliability Engineers (SREs), Systems Architects | Compliance Officers, Operations Leaders, Auditors, Board of Directors |
| **Object of Study** | Microservices, databases, networks, and infrastructure | Business activities, physical resources, actors, and legal entities |
| **Standard Ingestion** | OTel (JSON/Protobuf over OTLP) | OCEL 2.0, XES, BPMN, POWL |
| **Mathematical Basis** | Queueing theory, statistical time-series analysis | Automata theory, Petri nets, temporal logic, algebraic replay |
| **System Boundary** | Software boundaries (API gateways, execution threads) | Enterprise boundaries (multi-system flows, manual processes, partner APIs) |
| **Output Type** | Diagnostic dashboard alerts, traces, log streams | Replay-backed receipts, conformance reports, exception logs |

---

## Why System Observability Cannot Solve Process Governance

A system can be perfectly observable but completely out of control from a process standpoint. 

For example, a modern e-commerce platform using microservices might have excellent observability: every API call generates a clean trace span, latency is low, and CPU usage is stable. However, due to a bug in the discount service logic, users are able to apply multiple promotion codes and purchase items for \$0. 

To the observability system, everything is green: spans are returning HTTP 200, latency is 50ms, and Weaver reports no schema validation errors. 

To the process intelligence system, this is a major failure: the order execution trace violates the normative process model, which requires that total discounts must not exceed 50% of the cart value without manual manager approval. The process intelligence engine detects this deviation immediately via token replay, raises a conformance exception, and blocks the order settlement.

Observability-by-design provides the high-quality telemetry feedstock, but only process intelligence can enforce the rules that ensure business compliance and accountability.

---

## References

- [doctrine/PROCESS_INTELLIGENCE_IS_NOT.md](file:///Users/sac/process-intelligence/doctrine/PROCESS_INTELLIGENCE_IS_NOT.md)
- [doctrine/PROCESS_INTELLIGENCE_DEFINED.md](file:///Users/sac/process-intelligence/doctrine/PROCESS_INTELLIGENCE_DEFINED.md)
- [standards/OTEL_WEAVER.md](file:///Users/sac/process-intelligence/standards/OTEL_WEAVER.md)
- [doctrine/otel-weaver-is-feedstock.md](file:///Users/sac/process-intelligence/otel-weaver/doctrine/otel-weaver-is-feedstock.md)
