# Public Standards as the Process Intelligence Gravity Field

**Authority:** process-intelligence  
**Document type:** Strategic positioning  
**Claim:** "Our process intelligence is built on 10+ public standards with zero proprietary lock-in."

---

## The Gravitational Model

Public standards are not a compliance checkbox. They are the gravitational center around which the entire process intelligence capability orbits. Every capability — event log ingestion, process model import, conformance checking, provenance tracing, query answering — is anchored to a named public standard.

This is a deliberate architectural choice with a direct sales consequence: **the customer can always leave**.

---

## The Standards Inventory

| Standard | Body | Domain | What It Grounds |
|---|---|---|---|
| OCEL 2.0 | IEEE (pending) / van der Aalst et al. | Object-centric event logs | Event log ingestion, object-centric process mining |
| XES (IEEE 1849-2023) | IEEE | Classic flat event logs | Historical log compatibility, case-centric process mining |
| BPMN 2.0 | OMG | Process model notation | Process model import/export, visual representation |
| WF-net soundness (van der Aalst 1998) | Academic | Workflow net verification | Machine-verifiable process model correctness |
| Declare / LTL constraints (Pesic & van der Aalst 2006) | Academic | Declarative process modeling | Formal compliance constraint checking |
| PROV-O | W3C | Provenance and data lineage | Process evidence provenance, audit trail |
| SHACL | W3C | Constraint language for RDF | RDF-level process data validation |
| OpenTelemetry (OTel) | CNCF | Distributed system observability | Live system telemetry as process event source |
| OCPQ (Küsters & van der Aalst 2025) | Academic | Object-centric process querying | Complex multi-object process queries |
| POWL (Kourani & van der Aalst 2023) | Academic | Partially ordered workflow language | Non-block-structured process model shapes |
| Process Trees / Inductive Miner (Leemans et al.) | Academic | Block-structured process discovery | Process tree shapes, loop arity law |
| OC-Petri nets (van der Aalst 2019) | Academic | Object-centric Petri nets | Multi-object token semantics, arc inscription law |

That is 12 public standards. Every one is publicly documented, independently implementable, and non-proprietary.

---

## Reverse Lock-In: The Sales Advantage

Proprietary data formats and query languages create lock-in. Customers know this. They resist it. The standard objection to any process intelligence vendor is:

> "What happens to our data if we stop using your tool?"

The answer grounded in public standards is:

> "Your event logs are in OCEL 2.0 or XES — both IEEE-documented open formats. Your process models are in BPMN 2.0 (OMG standard) or WF-net (academic open format). Your provenance traces are in PROV-O (W3C standard). Any tool that speaks these standards can read your data. We are the best at analyzing it, but you are never locked in."

This is **reverse lock-in**: the openness itself is the differentiator. Competitors who use proprietary formats are vulnerable to the lock-in objection. We are not.

---

## The 10+ Standards Claim: How to Use It

The claim "Our process intelligence is built on 10+ public standards with zero proprietary lock-in" is board-admissible because:

1. It is precisely countable (12 standards in the inventory above)
2. Each standard is named and independently verifiable
3. The "zero proprietary lock-in" assertion is structurally grounded — every data format and model type has a public standard definition
4. The wasm4pm-compat crate formalizes this claim in Rust types: every witness marker (`Ocel20`, `Xes1849`, `WfNetSoundnessPaper`, `PowlPaper`) is a named type carrying the standard it certifies

---

## The Gravity Field Diagram

```
                     Public Standards
                          (12+)
                            │
          ┌─────────────────┼─────────────────┐
          │                 │                 │
     Event Logs       Process Models    Conformance
  (OCEL 2.0, XES)  (BPMN, WF-net,     (Alignment,
                    Declare, POWL)     Token Replay)
          │                 │                 │
          └─────────────────┼─────────────────┘
                            │
                   Process Intelligence
                   (wasm4pm engine)
                            │
                    ┌───────┴────────┐
                    │                │
              Provenance        Querying
            (PROV-O, SHACL)    (OCPQ, OTel)
```

The engine sits at the center. The public standards are the gravity field that holds everything in place — and that lets the customer extract their data at any time.

---

## Relationship to wasm4pm-compat

wasm4pm-compat is the type-level formalization of this gravity field. Each public standard maps to one or more witness markers in `src/witness.rs`. Each witness marker is a non-forgeable Rust type. The type system enforces that evidence claiming to be OCEL 2.0-compliant carries the `Ocel20` witness — not a string claim, a type constraint.

This means the gravity field is not just a marketing claim. It is a compile-time invariant.
