# ADMISSION AND REFUSAL LAW MAP — wasm4pm-compat

**Source:** /Users/sac/wasm4pm-compat/src/admission.rs
**Design principle:** Both outcomes are first-class, strongly-typed values. Neither is an exception.

---

## The Boundary Verdict Surface

`admission.rs` encodes the only sanctioned `Raw → Admitted` path in the entire crate. The module provides three items:

1. `Admission<T, W>` — the positive verdict: the value crossed the named boundary.
2. `Refusal<R, W>` — the negative verdict: the value was declined for a specific named reason.
3. `Admit` trait — the verdict contract. The only authorized issuer of `Admission` and `Refusal`.

---

## Admission<T, W>

```rust
pub struct Admission<T, W> {
    pub value: T,
    witness: PhantomData<W>, // zero-sized, private
}
```

### What it enforces

Holding an `Admission<T, W>` is type-level proof that an `Admit` impl accepted the value against witness `W`. The `witness` field is private — callers cannot construct `Admission` freely; they must go through `Admit::admit()`.

### The only path to Admitted evidence

```rust
impl<T, W> Admission<T, W> {
    pub fn into_evidence(self) -> Evidence<T, crate::state::Admitted, W>
}
```

`into_evidence()` is the **only** bridge from a verdict to `Admitted` `Evidence`. There is no public free conversion `Raw → Admitted` anywhere else in the crate. This is enforced by the compile-fail fixture `admission_raw_state_not_admitted.rs` (E0308).

### What Admission does NOT do

- It does not verify semantic correctness against a live engine.
- It does not run discovery, conformance, or replay.
- "Admitted" means "a named structural law was satisfied at this boundary" — not "an execution engine has verified this value."

---

## Refusal<R, W>

```rust
pub struct Refusal<R, W> {
    pub reason: R,
    witness: PhantomData<W>, // zero-sized, private
}
```

### What it enforces

`R` must be a **named law** — a specific type naming the exact reason the value was declined. The forbidden pattern is using a bare string or a catch-all `InvalidInput` type as `R`.

Examples of valid `R` types (named laws):
- `DanglingEventObjectLink` — an OCEL event references an object that does not exist in the log
- `MissingFinalMarking` — a WF-net has no declared final marking
- `MissingInitialMarking` — a WF-net has no token in the initial marking
- `UnsoundWfNet` — a WF-net has been structurally determined to be unsound

### Why named reasons beat String-typed errors

A `Refusal<String, Ocel20>` is auditable by humans but not by machines. A `Refusal<DanglingEventObjectLink, Ocel20>` allows:
- Pattern matching on `reason` to route the refusal to the right remediation handler
- Compile-time proof that all refusal reasons are handled (exhaustive match)
- A documented, stable law name that other parts of the codebase and documentation can reference
- Cross-referencing to the specific paper or standard that defines the law (via the Witness type `W`)

The `Refusal<R, W>` design directly implements the Van der Aalst Constitution doctrine: "if the event log cannot prove a lawful process happened, then it did not happen." The refusal records *which law was broken* in a machine-readable form.

### Display

```rust
impl<R: Display, W> Display for Refusal<R, W> {
    fn fmt(..) { write!(f, "Refusal: {}", self.reason) }
}
```

The witness tag `W` is zero-sized and has no displayable value.

### Debug

Both `Admission<T, W>` and `Refusal<R, W>` provide manual `Debug` implementations that do not require `W: Debug`. This enables `Result::expect` and `Result::expect_err` in test code without requiring the witness marker to implement `Debug`.

---

## The Admit Trait

```rust
pub trait Admit {
    type Raw;
    type Admitted;
    type Reason;
    type Witness;
    
    fn admit(
        evidence: Evidence<Self::Raw, Raw, Self::Witness>
    ) -> Result<Admission<Self::Admitted, Self::Witness>, Refusal<Self::Reason, Self::Witness>>;
}
```

### The contract

- `admit()` takes `Raw` evidence of `Self::Raw` against `Self::Witness`.
- Returns either `Admission<Self::Admitted, W>` or `Refusal<Self::Reason, W>`.
- The `Self::Reason` type must be a named law (an enum variant, not a string).
- An `Admit` impl encodes *which named structural law* gates the boundary — it does not run an engine.

### Boundaries requiring real verification

When a boundary needs real semantic verification (token replay, soundness checking, alignment computation), the `Admit` impl should return a `GraduationCandidate` instead of attempting local verification. The impl graduates to `wasm4pm` rather than re-implementing engine logic.

---

## Named Refusal Reasons in the Codebase

The following named reason types are used or documented across `petri.rs`, `formats.rs`, `conformance.rs`, and the module-level documentation:

### Petri / WF-net boundary
- `PetriRefusal::MissingInitialMarking` — WF-net has no token in initial marking
- `PetriRefusal::MissingFinalMarking` — WF-net has no declared final marking
- `PetriRefusal::DanglingArc` — arc references an undeclared node
- `PetriRefusal::DuplicateNodeId` — two nodes with the same id

### OCEL 2.0 boundary
- `DanglingEventObjectLink` — event references non-existent object (documented in admission.rs module header and diagnostic.rs)
- Object-type namespace violations (via `OcelObjectType` witness)
- Event-type (activity) violations (via `OcelEventType` witness)

### Format export boundary
- `XesExportRefusal` — named reasons for refusing XES export
- `OcelExportRefusal` — named reasons for refusing OCEL export
- `LossyFormatExport` — requires a named loss report; no silent loss

### Conformance boundary
- `ConformanceRefusal` — named reasons why a conformance verdict cannot be admitted (e.g., missing model, empty log)

### Strict boundary (`strict.rs`)
- `StrictViolation::MissingLossPolicy` — export declared without a loss policy
- `StrictViolation::MissingRefusalPath` — no explicit refusal path for boundary errors
- `StrictViolation::HiddenProcessMiningGrowth` — process mining logic added locally rather than graduating

---

## What Refusal<R, W> Enforces That String-Typed Errors Cannot

| Property | `Refusal<R, W>` | `Result<T, String>` |
|---|---|---|
| Exhaustive match coverage | Yes (Rust exhaustiveness check on `R`) | No |
| Machine-readable reason | Yes | No (human parse only) |
| Witness carries authority | Yes (W names the paper/standard) | No |
| Stable, refactorable law name | Yes (type, not string literal) | No |
| Cross-reference to formal definition | Yes (via Witness::TITLE, YEAR) | No |
| Prevents catch-all handling | Yes (each variant requires handling) | No |
| Zero-cost (no allocation) | Yes (R is typically an enum, no heap) | No (String always allocates) |

---

## ALIVE Gate Coverage for Admission/Refusal

Compile-fail fixtures proving the boundary:
- `admission_raw_state_not_admitted.rs` — `Raw` evidence cannot be passed where `Admitted` is required (E0308)
- `admission_refusal_as_admission.rs` — `Refusal` cannot be used as `Admission` (E0308)
- `refusal_without_named_law.rs` — documents the expectation that refusals carry named laws

Compile-pass fixtures documenting the lawful path:
- `admission_admit_trait_surface.rs` — `Admit` trait implementation surface
- `admission_new_and_into_evidence.rs` — `Admission::new()` + `into_evidence()` chain
- `admission_refusal_named_law.rs` — `Refusal` with a named law reason
- `admission_refusal_new_into_reason.rs` — `Refusal::new()` + `into_reason()`
