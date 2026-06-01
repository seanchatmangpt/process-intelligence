# OCEL 2.0 Order-to-Cash Lifecycle Sample

**Analyst:** Dr. OCEL Specialist (AGI)
**Date:** 2026-05-31

---

## Process: Order-to-Cash

The order-to-cash (O2C) process is the canonical multi-object OCEL benchmark because it involves four co-evolving object types with complex cross-object causality.

### Object Types

| Type | Description | Example Instances |
|---|---|---|
| `Order` | A customer purchase order | O-001, O-002 |
| `Customer` | The party placing orders | C-001 |
| `Invoice` | A billing document derived from an order | I-001, I-002 |
| `Payment` | A financial settlement against an invoice | P-001 |

---

## Sample OCEL 2.0 Log

### Objects

```
OcelObject { id: "O-001", object_type: "Order",    attributes: { "value": 1200.00 } }
OcelObject { id: "O-002", object_type: "Order",    attributes: { "value": 450.00  } }
OcelObject { id: "C-001", object_type: "Customer", attributes: { "name": "Acme Corp", "tier": "Gold" } }
OcelObject { id: "I-001", object_type: "Invoice",  attributes: { "amount": 1200.00 } }
OcelObject { id: "I-002", object_type: "Invoice",  attributes: { "amount": 450.00  } }
OcelObject { id: "P-001", object_type: "Payment",  attributes: { "amount": 1200.00 } }
```

### Events with E2O Links

```
E-001: activity="order_created",    ts=2026-05-01T09:00Z
  E2O: E-001 → O-001 (qualifier: "created_order")
  E2O: E-001 → C-001 (qualifier: "placing_customer")

E-002: activity="item_added",       ts=2026-05-01T09:05Z
  E2O: E-002 → O-001 (qualifier: "target_order")

E-003: activity="item_added",       ts=2026-05-01T09:07Z
  E2O: E-003 → O-001 (qualifier: "target_order")

E-004: activity="order_confirmed",  ts=2026-05-01T10:00Z
  E2O: E-004 → O-001 (qualifier: "confirmed_order")
  E2O: E-004 → C-001 (qualifier: "confirming_customer")

E-005: activity="order_created",    ts=2026-05-02T11:00Z
  E2O: E-005 → O-002 (qualifier: "created_order")
  E2O: E-005 → C-001 (qualifier: "placing_customer")

E-006: activity="invoice_sent",     ts=2026-05-03T08:00Z
  E2O: E-006 → I-001 (qualifier: "sent_invoice")
  E2O: E-006 → O-001 (qualifier: "invoiced_order")
  E2O: E-006 → C-001 (qualifier: "billed_customer")

E-007: activity="order_confirmed",  ts=2026-05-03T09:00Z
  E2O: E-007 → O-002 (qualifier: "confirmed_order")
  E2O: E-007 → C-001 (qualifier: "confirming_customer")

E-008: activity="invoice_sent",     ts=2026-05-04T10:00Z
  E2O: E-008 → I-002 (qualifier: "sent_invoice")
  E2O: E-008 → O-002 (qualifier: "invoiced_order")
  E2O: E-008 → C-001 (qualifier: "billed_customer")

E-009: activity="payment_received", ts=2026-05-20T14:00Z
  E2O: E-009 → P-001 (qualifier: "received_payment")
  E2O: E-009 → I-001 (qualifier: "settled_invoice")
  E2O: E-009 → C-001 (qualifier: "paying_customer")

E-010: activity="order_fulfilled",  ts=2026-05-21T09:00Z
  E2O: E-010 → O-001 (qualifier: "fulfilled_order")
```

### O2O Links

```
O2O: O-001 → C-001 (qualifier: "placed_by")
O2O: O-002 → C-001 (qualifier: "placed_by")
O2O: I-001 → O-001 (qualifier: "invoices_order")
O2O: I-002 → O-002 (qualifier: "invoices_order")
O2O: P-001 → I-001 (qualifier: "settles_invoice")
```

### ObjectAttributeChanges

```
O-001, ts=2026-05-01T10:00Z, status: "open" → "confirmed"
O-001, ts=2026-05-21T09:00Z, status: "confirmed" → "fulfilled"
I-001, ts=2026-05-20T14:00Z, status: "pending" → "paid"
```

---

## wasm4pm-compat Representation

### Constructing the OcelLog

```rust
use wasm4pm_compat::ocel::{OcelLog, OcelEvent, OcelObject, EventObjectLink, ObjectObjectLink, ObjectChange};

let order_001 = OcelObject::new("O-001".into(), "Order".into());
let customer_001 = OcelObject::new("C-001".into(), "Customer".into());
let invoice_001 = OcelObject::new("I-001".into(), "Invoice".into());
let payment_001 = OcelObject::new("P-001".into(), "Payment".into());

let e001 = OcelEvent::new("E-001".into(), "order_created".into())
    .at_ns(1746090000_000_000_000u64);

let e001_to_o001 = EventObjectLink::new("E-001".into(), "O-001".into())
    .qualified("created_order".into());
let e001_to_c001 = EventObjectLink::new("E-001".into(), "C-001".into())
    .qualified("placing_customer".into());

let o001_to_c001 = ObjectObjectLink::new("O-001".into(), "C-001".into())
    .qualified("placed_by".into());

let log = OcelLog::new()
    .with_object(order_001)
    .with_object(customer_001)
    .with_object(invoice_001)
    .with_object(payment_001)
    .with_event(e001)
    .with_e2o_link(e001_to_o001)
    .with_e2o_link(e001_to_c001)
    .with_o2o_link(o001_to_c001);
```

### Admission

```rust
use wasm4pm_compat::admission::Admit;
use wasm4pm_compat::witness::Ocel20;

let admitted = Admit::<OcelLog, Ocel20>::admit(log)
    .expect("structurally valid OCEL 2.0 log");
// Type: Admission<OcelLog, Ocel20>
// The Ocel20 witness names the IEEE OCEL 2.0 standard.
```

---

## What wasm4pm Would Need to Discover the Process

### Step 1: OC-DFG Discovery (per object type)

```
wasm4pm::discovery::oc_dfg(admitted) -> OcDfg<Ocel20>
```

Expected per-type DFGs:
- **Order DFG:** `order_created → item_added → order_confirmed → invoice_sent → order_fulfilled`
- **Customer DFG:** `order_created → order_confirmed → invoice_sent → payment_received`
- **Invoice DFG:** `invoice_sent` (then `payment_received` via P-001 O2O link)

### Step 2: Per-Type Process Discovery (Inductive Miner required)

```
wasm4pm::discovery::inductive_miner(order_dfg) -> ProcessTree<InductiveMinerPaper>
```

Expected Order process tree:
```
→(order_created, *(item_added, τ), order_confirmed, invoice_sent, order_fulfilled)
```

### Step 3: Cross-Object Conformance

```
wasm4pm::conformance::oc_alignment(admitted, models) -> OcAlignmentResult<Ocel20>
```

Multi-type conformance: each object follows its type-specific model. Cross-object causality (O-001 confirmed before I-001 sent) checked via O2O/E2O link tracing.

### Step 4: Receipt Generation

```
wasm4pm::receipt::generate(oc_alignment_result) -> Receipt<OcAlignmentResult, Ocel20>
```

---

## Cross-Object Causality Observations

1. **C-001 participates in all 10 events** — invisible to per-case flat mining; detectable only via E2O links.
2. **I-001 invoice lineage** — causally downstream of E-004 (order_confirmed for O-001); detectable via O2O `invoices_order` link.
3. **P-001 settles I-001 but not I-002** — partial settlement tracking requires O2O `settles_invoice`; flat log loses this distinction.
4. **O-002 lifecycle is incomplete** — no `order_fulfilled` event for O-002; object-centric discovery flags this as an incomplete lifecycle variant.

These four observations are the primary argument for OCEL 2.0 over XES for order-to-cash process intelligence.
