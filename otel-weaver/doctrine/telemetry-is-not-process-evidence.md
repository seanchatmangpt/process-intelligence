# telemetry-is-not-process-evidence.md

**Authority:** `/Users/sac/process-intelligence/otel-weaver/doctrine`  
**Status:** ALIVE — anchored at TELEMETRY_NOT_EVIDENCE_001  

---

## The Evidence Boundary

A system that emits telemetry is not a system that emits process evidence. We enforce a strict nominal distinction:

> **Telemetry is feedstock; process consequence is court.**

Telemetry is the observer's report of a system's internal actions, recorded as logs, spans, or metrics. Evidence is a typed, witnessed, and immutable receipt showing that a specific action was executed in compliance with a named law. Telemetry is a signal; evidence is a fact.

In a corporate transaction, an audit, or a regulatory review, a telemetry span is inadmissible on its own. Telemetry can be modified, dropped, or injected by infrastructure operators without invalidating the business state. Evidence, however, is bound by cryptographic chain-of-custody and type law, meaning it cannot be altered or fabricated without invalidating the entire process.

---

## Structural Differences

To prevent category collapse, the physical and logical structures of telemetry and process evidence must remain separate:

| Feature | Telemetry (OTel Spans/Logs) | Process Evidence (Foundry Receipts) |
| :--- | :--- | :--- |
| **Perspective** | Observer-centric (what the software saw) | Actor-centric (what the business process did) |
| **Mutability** | Ephemeral, subject to retention policies and filtering | Immutable, cryptographically sealed, and archived |
| **Identity** | Trace IDs, Span IDs (random or sequential context) | Object-centric identities (OCEL 2.0 identifiers) |
| **Accountability** | System host, service name, runtime ID | Named operator, active law, signed witness |
| **Verification** | Syntactic validation (Weaver schemas) | Algebraic replay, Petri net conformance checks |

---

## The Concept of Board-Admissibility

A board-admissible claim requires process evidence, not telemetry. 

Consider a due diligence scenario:
- **Telemetry-Backed Claim**: *"Our distributed tracing infrastructure shows that the order processing service had a 99% success rate with an average span latency of 240ms."*  
  **Critique**: This is a system measurement. It does not prove that the orders were legally authorized, that inventory was actually allocated, or that the billing sequences conformed to standard accounting principles. It only proves the software was fast and did not throw uncaught exceptions.
- **Evidence-Backed Claim**: *"We executed 45,000 order-to-cash transactions under the compliance standards of standard `Ocel20` and delegation law `AuthLaw_2026_03`. Every transaction is backed by a `Receipt<OrderProcessed, AuthLaw_2026_03>`, verified via replay against our Petri net model `OrderPipelineNet`."*  
  **Critique**: This is a process fact. The claim is auditable, repeatable, and carries legal accountability.

---

## The Evidence Chain

The conversion of telemetry feedstock into process evidence follows a strict manufacturing path:

```
[ Raw OTel Spans ]
        │  (Ingestion Boundary)
        ▼
[ Loss-Accounted OCEL Events ] (Sealed with LossReport)
        │  (Replay & Validation)
        ▼
[ Conformance Verification ] (Evaluated against Petri Net / POWL)
        │  (Witness Signing)
        ▼
[ Process Evidence Receipt ] (Cryptographic Proof)
```

At no point in this chain is telemetry allowed to bypass validation and masquerade as process evidence. A system that stores raw OTel spans on a ledger is not a process intelligence system; it is a telemetry database. The value of process intelligence lies in the manufacturing authority that transforms feedstock into court-ready evidence.

---

## References

- [doctrine/otel-weaver-is-feedstock.md](file:///Users/sac/process-intelligence/otel-weaver/doctrine/otel-weaver-is-feedstock.md)
- [doctrine/weaver-finding-is-not-receipt.md](file:///Users/sac/process-intelligence/otel-weaver/doctrine/weaver-finding-is-not-receipt.md)
- [doctrine/RECEIPT_DOCTRINE.md](file:///Users/sac/process-intelligence/doctrine/RECEIPT_DOCTRINE.md)
- [standards/otel_weaver_projection_placement.md](file:///Users/sac/process-intelligence/standards/otel_weaver_projection_placement.md)
