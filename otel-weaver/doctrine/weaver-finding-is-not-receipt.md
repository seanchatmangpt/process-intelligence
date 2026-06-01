# weaver-finding-is-not-receipt.md

**Authority:** `/Users/sac/process-intelligence/otel-weaver/doctrine`  
**Status:** ALIVE — anchored at WEAVER_FINDING_NOT_RECEIPT_001  

---

## Nominal Category Distinction

We enforce a strict boundary between static/runtime structural findings and cryptographic process receipts:

> **A Weaver finding proves structural compatibility; a Process Receipt proves execution validity and named law compliance.**

Confusing these two categories is a structural defect. A system that conflates schema matching with transaction verification is vulnerable to silent process failure.

---

## Weaver Findings: Structural Compatibility

An **OTel Weaver Finding** is the output of a schema alignment check. It answers system-level questions:
- *Does the span named `purchase_order` conform to the `process.activity` schema convention?*
- *Are the required attributes `order.id` and `customer.id` present and typed correctly?*
- *Does the payload match the target specification defined in the semantic conventions registry?*

A Weaver finding is a predicate check. Its output is binary: `Compatible` or `Incompatible` (with warning logs for missing optional attributes). It validates that the telemetry collector has received readable structured data. It has no knowledge of execution history, temporal dependencies, or business rules.

---

## Process Receipts: Operational Validity

A **Process Receipt** is a typed, witnessed, and bound evidence artifact. In the `wasm4pm-compat` framework, it is represented as:

```rust
pub struct Receipt<T, W> {
    pub result: T,
    pub witness: std::marker::PhantomData<W>,
    pub signature: Signature,
    pub timestamp_ns: u64,
}
```

A Process Receipt answers business-level and legal-level questions:
- *Was the `purchase_order` issued by an authorized user matching the delegation rules of `AuthLaw`?*
- *Did the `payment` transition occur only after the order was verified and approved by the inventory manager?*
- *Is this execution path valid within the Petri net `PurchaseOrderNet`?*

The witness `W` is a zero-sized type that names the specific standard, paper, or business law applied to the execution. The receipt is a permanent, non-repudiable record of process reality.

---

## The Category Contrast

The following matrix highlights the functional differences:

| Attribute | Weaver Finding | Process Receipt |
| :--- | :--- | :--- |
| **Type Level** | Dynamic structure validation (YAML/Protobuf schema check) | Compile-time rust type checking (`Receipt<T, W>`) |
| **Verification Scope** | Individual span attributes | Full process execution trace (history, causality, and concurrency) |
| **Audit Value** | Proves telemetry instrumentation is functional | Proves operational and legal compliance |
| **Storage** | Ephemeral logs (discarded or archived with system telemetry) | Ledger-bound state (part of the company's permanent evidence chain) |
| **Witness Marker** | None (software-generated diagnostic log) | Cryptographic signature of a named law / execution engine |

---

## The Hazard of Conflation

If a process intelligence engine fails to distinguish between a Weaver finding and a Process Receipt, it creates a security and compliance gap. 

For instance, an adversary could generate structurally perfect spans (`customer.id` is valid, `payment.amount` is present) and inject them into the telemetry stream to simulate completed transactions. A validation checker running OTel Weaver would report a finding of **100% compliance** because the schema is satisfied. 

However, a conformance checking engine running `wasm4pm` will reject the spans because they lack preceding events, causal linkages, and cryptographic signatures from authorized actor systems. No `Receipt` will be generated, and the injection attempt will be caught.

---

## References

- [doctrine/RECEIPT_DOCTRINE.md](file:///Users/sac/process-intelligence/doctrine/RECEIPT_DOCTRINE.md)
- [doctrine/otel-weaver-is-feedstock.md](file:///Users/sac/process-intelligence/otel-weaver/doctrine/otel-weaver-is-feedstock.md)
- [standards/otel_weaver_projection_placement.md](file:///Users/sac/process-intelligence/standards/otel_weaver_projection_placement.md)
