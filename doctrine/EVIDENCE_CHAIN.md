# Evidence Chain: From Raw Events to Board Claims

> Activity is not evidence. Evidence requires witness.

---

## The One-Way Door

The central invariant of wasm4pm-compat is a typed, one-way lifecycle enforced by the type
system. The lifecycle is not a convention or a runtime flag — it is expressed in Rust types
such that violating the lifecycle is a compile error, not a runtime exception.

```
Raw ──parse──▶ Parsed ──admit──▶ Admitted ──▶ {Projected | Exportable | Receipted}
  │                                  ▲
  └────────────── refuse ────────────┴──▶ Refused  (terminal; carries a named law)
```

`Evidence<T, State, W>` is the universal carrier. `State` and `W` are zero-sized `PhantomData`
tags. `Evidence<T, Raw, W>` and `Evidence<T, Admitted, W>` are **different types** — the
Rust type system prevents any path from Raw to Admitted except through `Admit::admit()`.

---

## `Evidence<T, State, W>` — the carrier

Located in `src/evidence.rs`.

- `T` — the payload: an `OcelLog`, a `XesLog`, a `WfNetConst<SOUNDNESS>`, any process
  evidence shape.
- `State` — one of: `Raw`, `Parsed`, `Admitted`, `Refused`, `Projected`, `Exportable`,
  `Receipted`. These are empty enums in `src/state.rs`. They are type-level tags, not
  runtime values. They occupy zero bytes.
- `W` — a witness marker from `src/witness.rs` (e.g., `Ocel20`, `Xes1849`,
  `WfNetSoundnessPaper`, `PowlPaper`). `W` names the external authority that grounds the
  evidence. `Admission<T, Ocel20>` and `Admission<T, Xes1849>` are different types — you
  cannot accidentally use an XES-admitted log where an OCEL-admitted log is required.

### Constructors and transitions

- `Evidence::raw(v)` — the only free constructor; produces `Evidence<T, Raw, W>`.
- `into_parsed()` — infallible builder method; `Raw → Parsed`.
- `Admit::admit()` — the **only** sanctioned `Parsed → Admitted` path. Returns
  `Result<Admission<T, W>, Refusal<R, W>>`. The `Admitted` constructor is `pub(crate)`.
- `into_projected()` — `Admitted → Projected`; requires `Project` trait implementation.
- `into_exportable()` — `Admitted → Exportable`; requires format covenant compliance.
- `into_receipted()` — `Admitted → Receipted`; emits a `Receipt`.

There is no `Evidence::admitted(v)`. There is no `Evidence::force_admit(v)`. The only way
to produce an `Evidence<T, Admitted, W>` is to pass through `Admit::admit()`.

---

## Board Claims Trace to Event Log Evidence

A board claim is admissible if and only if it traces through the evidence chain to a specific
admitted event log and a receipted conformance result.

### The chain for a conformance claim:

```
Raw event records
  │
  └─ Evidence::raw(ocel_log) : Evidence<OcelLog, Raw, Ocel20>
       │
       └─ into_parsed() : Evidence<OcelLog, Parsed, Ocel20>
            │
            └─ Admit::admit() → Ok(Admission<OcelLog, Ocel20>)   or Err(Refusal<R, Ocel20>)
                 │
                 └─ wasm4pm: execute token replay → FitnessConst value
                      │
                      └─ into_receipted() : Evidence<Metric<FITNESS>, Receipted, Ocel20>
                           │
                           └─ Board claim: "Fitness = [NUM/DEN] under Ocel20 witness"
```

Every node in this chain is typed. Every witness is named. Every refusal carries a specific
law. No step can be skipped — the type system makes it a compile error to skip `Admit::admit()`
and jump directly to `Admitted`.

---

## What Happens When the Chain Breaks

### 1. Raw Laundering

**Definition:** Consuming `Evidence<T, Raw, W>` as if it were admitted, without passing
through `Admit::admit()`.

**Consequence:** The evidence has no named witness. There is no refusal surface. Any
structural defect in the raw data passes silently into downstream processing. The board claim
that emerges has no traceable receipt.

**Detection:** The type system prevents this in wasm4pm-compat — you cannot call `into_projected()`
on `Evidence<T, Raw, W>`. Any attempt is a compile error. Raw laundering can only happen
if the type law is bypassed (which requires `unsafe`, which is `#![forbid(unsafe_code)]` in
this crate).

### 2. Hidden Loss

**Definition:** A lossy transformation (e.g., OCEL → XES flattening) that does not emit a
`LossReport` and does not declare a `LossPolicy`.

**Consequence:** Object-type information, object-to-object links, and multi-instance event
multiplicity are silently discarded. Divergence and convergence defects are introduced into
the evidence without being named.

**Detection:** The `formats` feature enforces the `LossyFormatExport` trait, which requires
a non-optional loss report. Any `impl LossyFormatExport` that does not produce a `LossReport`
fails to compile.

**Unlawful path (defect):** Implementing a direct `From<OcelLog> for XesLog` without
`LossPolicy` + `LossReport`. This is exactly what the format covenant prohibits:
```
NO: OcelLog → XesLog directly
YES: OcelLog → admitted compat structure → XesLog via LossyFormatExport with LossReport
```

### 3. Unsigned Projection

**Definition:** A projection that names no `ProjectionName`, declares no `LossPolicy`, and
emits no `LossReport`, but still produces a target structure.

**Consequence:** The projection is indistinguishable from a full-fidelity transformation.
Downstream consumers cannot tell whether information was lost.

**Detection:** `LossPolicy::RefuseLoss` will refuse any projection that encounters loss
without explicit allowance. `LossPolicy::AllowNamedProjection` requires a `ProjectionName`.
`LossPolicy::AllowLossWithReport` requires a `LossReport`. There is no unnamed, reportless
projection path in the type system.

---

## The Full Chain: Raw to Receipt

```
1. Raw data arrives from external system (OCEL JSON, XES XML, BPMN 2.0, PNML)
2. Evidence::raw(data) → Evidence<T, Raw, W>            [type: Raw]
3. Format parse → Evidence<T, Parsed, W>                 [type: Parsed]
4. Admit::admit() → Admission<T, W>                      [type: Admitted]
   (or Refusal<NamedLaw, W> if structural law violated)
5. wasm4pm executes algorithm over Admission<T, W>
   → Metric<FITNESS, NUM, DEN> with Between01<NUM,DEN> bound
   → Metric<PRECISION, NUM, DEN>
   → AlignmentResult shapes
6. Evidence::into_receipted() → Evidence<Metric, Receipted, W>  [type: Receipted]
7. Receipt emitted → board claim possible
```

Every step names its law. Every refusal names its violated law. Every loss names its policy.
Every projection names its name. Every receipt names its witness.

**A claim that skips any step is narration. A claim that completes every step is process intelligence.**
