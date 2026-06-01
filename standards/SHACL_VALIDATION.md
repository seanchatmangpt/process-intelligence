# SHACL Validation — Shapes Constraint Language for Process Models

**Authority:** process-intelligence  
**Source Standard:** W3C SHACL (Shapes Constraint Language), W3C Recommendation 2017  
**Related:** PROV-O (W3C), RDF/OWL, wasm4pm-compat structural law

---

## What SHACL Is

SHACL (Shapes Constraint Language) is a W3C standard for validating RDF graphs against a set of conditions called shapes. A SHACL shape defines what properties a node in an RDF graph must or must not have. Constraint violations are typed reports, not free-text errors.

SHACL operates as a constraint-first system: you declare what is lawful, then validate data against that declaration. Violations are named and structured. This is the same operating principle as wasm4pm-compat's type law.

---

## Why SHACL Belongs in Process Intelligence Authority

Process intelligence data — event logs, process models, provenance traces, conformance results — is increasingly represented as RDF:

- OCEL event logs can be lifted to RDF via OCED ontology
- Process models (Petri nets, BPMN) can be exported as RDF/OWL
- Provenance traces are natively PROV-O (W3C RDF standard)
- Process querying results can be materialized as RDF graphs

SHACL provides the validation layer over this RDF representation. When process intelligence data is stored or exchanged as RDF, SHACL is the authority for declaring what a lawful event log, process model, or provenance trace looks like in that representation.

---

## SHACL and wasm4pm-compat: Constraint-First Parallel

Both SHACL and wasm4pm-compat are constraint-first systems. The correspondence is direct:

| wasm4pm-compat concept | SHACL concept |
|---|---|
| Named refusal reason (e.g., `DanglingEventObjectLink`) | Named SHACL violation with `sh:resultMessage` |
| Compile-fail fixture (law rejection at compile time) | SHACL shape validation (law rejection at data ingestion) |
| `Admit::admit()` as the only lawful path to `Admitted` | SHACL `sh:targetClass` — only nodes matching the shape pass |
| `LossPolicy::RefuseLoss` | `sh:minCount 1` with no default — absent required properties cause violation |
| Witness marker (e.g., `Ocel20`, `Xes1849`) | SHACL namespace prefix binding the shape to a named standard |

wasm4pm-compat enforces laws at compile time (Rust type system). SHACL enforces laws at data ingestion time (RDF graph validation). They are complementary, not overlapping.

---

## SHACL Validates PROV-O Provenance Graphs

Process intelligence produces provenance traces (where did this result come from? what log was replayed? which model was used?). These traces are expressed in PROV-O (W3C Provenance Ontology), which is itself an RDF standard.

SHACL shapes over PROV-O graphs answer questions such as:

- Did every `prov:Activity` have a `prov:startedAtTime` and `prov:endedAtTime`?
- Did every `prov:Entity` (result artifact) have a `prov:wasGeneratedBy` link to an activity?
- Did every conformance check result have a named `prov:wasAttributedTo` model and log?

Without SHACL, these constraints are narrative requirements. With SHACL, they are machine-verifiable shapes over the provenance graph.

---

## SHACL Shape Example: Lawful OCEL Event as RDF

```turtle
ex:OcelEventShape a sh:NodeShape ;
    sh:targetClass ocel:Event ;
    sh:property [
        sh:path ocel:hasTimestamp ;
        sh:minCount 1 ;
        sh:maxCount 1 ;
        sh:datatype xsd:dateTime ;
    ] ;
    sh:property [
        sh:path ocel:hasActivity ;
        sh:minCount 1 ;
        sh:nodeKind sh:IRI ;
    ] ;
    sh:property [
        sh:path ocel:hasObjectLink ;
        sh:minCount 1 ;
        sh:message "An OCEL event must link to at least one object. Isolated events are structurally invalid." ;
    ] .
```

This SHACL shape encodes the same law as wasm4pm-compat's `ocel_e2o_missing_link` compile-fail fixture — an event without an object link is structurally invalid.

---

## Relationship to Other Standards

- **PROV-O** — SHACL validates PROV-O provenance graphs; together they provide machine-verifiable data lineage
- **OCEL 2.0** — OCEL data lifted to RDF is validated by SHACL shapes derived from the OCEL 2.0 spec
- **XES** — XES logs serialized as RDF (e.g., via OCED/RDF lifting) can be validated by SHACL shapes encoding XES extension declaration laws
- **wasm4pm-compat** — type law at compile time (Rust); SHACL at data ingestion time (RDF); both are constraint-first

---

## Board Claim Contribution

> "Our process models and provenance traces can be validated against W3C-standard constraint shapes."

SHACL makes process intelligence validation machine-verifiable and interoperable with the W3C Semantic Web stack. Any tool that speaks SHACL can validate our process data. This is zero proprietary lock-in.
