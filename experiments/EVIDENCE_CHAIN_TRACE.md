# Evidence Chain Trace — From Raw Log to Board Claim

**Analyst:** Dr. OCEL Specialist (AGI)
**Date:** 2026-05-31

**Board claim under analysis:**
> "Invoice processing conforms 94.2% to process model."

---

## Full Evidence Chain

### Step 1: Raw OCEL Log

**Input:** Raw OCEL 2.0 JSON/XML file (from enterprise system, e.g., SAP, Oracle)

```
Source: ERP export, format=OCEL 2.0 JSON
Content: ~50,000 invoice processing events, 12,000 invoice objects, 8,000 order objects
Status: Raw — no structural validation, no admission gate
Type (wasm4pm-compat): Evidence<OcelLog, Raw, Ocel20>  // constructed via Evidence::raw(ocel_log)
```

The `Raw` state tag means the log has been received but not validated. Any claim made against a `Raw` log is unattested.

---

### Step 2: OcelLog::parse()

**Operation:** Structural parsing — deserialize JSON/XML into typed `OcelLog` struct

```rust
let raw_log: Evidence<OcelLog, Raw, Ocel20> = Evidence::raw(parsed_json);

// parse() validates:
// - All event_ids are unique
// - All object_ids are unique
// - All E2O links reference known event_ids and object_ids
// - All O2O links reference known object_ids
// - All timestamps are well-formed

let parsed: Evidence<OcelLog, Parsed, Ocel20> = raw_log.into_parsed()?;
```

**Failure path:** If any structural constraint is violated, this returns `Err(Refusal<MalformedOcelStructure, Ocel20>)`. The reason type `MalformedOcelStructure` is a specific named law — not a generic `InvalidInput`.

**State after:** `Evidence<OcelLog, Parsed, Ocel20>` — structurally well-formed but not yet admitted as law-conforming.

---

### Step 3: OcelLog::admit() via Admit<OcelLog, Ocel20>

**Operation:** Law-conformance admission — verify the log satisfies OCEL 2.0 standard requirements

```rust
use wasm4pm_compat::admission::Admit;
use wasm4pm_compat::witness::Ocel20;

let admitted: Admission<OcelLog, Ocel20> = Admit::<OcelLog, Ocel20>::admit(parsed.into_inner())?;
```

**Admission checks (OCEL 2.0 law):**
- Every `EventObjectLink` event_id references an event in the log (`DanglingEventObjectLink` law)
- Every `EventObjectLink` object_id references an object in the log (`DanglingEventObjectLink` law)
- No two events share the same `event_id` (`DuplicateEventId` law)
- Object types are non-empty strings (`EmptyObjectType` law)
- Activity names are non-empty strings (`EmptyActivityName` law)
- `ObjectAttributeChange` timestamps are >= the object's creation event timestamp (`TemporalObjectChangeLaw`)

**Failure paths (specific named laws, not generic errors):**
- `Err(Refusal<DanglingEventObjectLink, Ocel20>)` — broken E2O reference
- `Err(Refusal<DuplicateEventId, Ocel20>)` — duplicate event IDs
- `Err(Refusal<TemporalObjectChangeLaw, Ocel20>)` — impossible temporal order

**State after:** `Admission<OcelLog, Ocel20>` — the log is certified as law-conforming OCEL 2.0. The `Ocel20` witness names the standard. This is the root of the evidence chain.

---

### Step 4: wasm4pm Token Replay (current state — GAP_001)

**Intended operation:** Replay admitted log against a process model to produce fitness

```rust
// INTENDED (correct):
wasm4pm::conformance::token_replay(admitted: Admission<OcelLog, Ocel20>, model: WfNetConst<Sound>)
    -> Metric<FITNESS, NUM, DEN>
```

**Current state in wasm4pm — GAP_001:**

```rust
// ACTUAL (defective — accepts raw EventLog, not Admitted<OcelLog, Ocel20>):
wasm4pm::conformance::token_replay(log: EventLog, model: PetriNet) -> f64
```

**Why this is GAP_001:**
1. Input is `EventLog` (flat XES-style), not `Admission<OcelLog, Ocel20>` — the OCEL 2.0 structure is discarded
2. Model is untyped `PetriNet`, not `WfNetConst<Sound>` — soundness is not enforced
3. Output is `f64`, not `Metric<FITNESS, NUM, DEN>` — the `Between01` bound is not enforced at the type level
4. No witness threading — the result carries no `Ocel20` marker
5. **Consequence:** The receipt chain is broken. A fitness value produced this way cannot be attested to the OCEL 2.0 standard.

**The gap in detail:**
- Step 3 produces `Admission<OcelLog, Ocel20>` — an attested, typed artifact
- Step 4 (current wasm4pm) flattens this to `EventLog` — the `Ocel20` witness is lost
- All subsequent results are computed on an unattested proxy, not on the admitted OCEL log
- The board claim "conforms to our process model" cannot be traced to the OCEL 2.0 standard

---

### Step 5 (Correct Path): Metric<FITNESS, 942, 1000>

**What should happen after GAP_001 is fixed:**

```rust
// After correct token replay or alignment:
let fitness: Metric<FITNESS, 942, 1000> = wasm4pm::conformance::alignment(
    admitted,    // Admission<OcelLog, Ocel20>
    model,       // WfNetConst<Sound>
)?;

// Type law enforced:
// - FITNESS const param names the metric kind
// - 942/1000 are NUM/DEN — Between01<942, 1000> is satisfied (942/1000 ∈ [0,1])
// - If NUM > DEN, this FAILS TO COMPILE — impossible fitness values are type errors
```

**Note:** Alignment-based conformance is preferred over token replay for board claims because:
- Alignment makes all deviations explicit (log moves and model moves)
- Token replay can produce false positives
- "94.2% conformance" as a board claim requires alignment evidence, not token replay evidence

---

### Step 6: Receipt

**Operation:** Stamp the conformance result with the evidence chain

```rust
let receipt: Receipt<Metric<FITNESS, 942, 1000>, Ocel20> = Receipt::from_metric(
    fitness,
    admitted.witness(),  // Ocel20 — the standard under which the log was admitted
    model_id,            // identifier of the normative process model
    timestamp,           // when the conformance check was run
);
```

**The receipt certifies:**
- The fitness value (94.2%)
- The log was admitted under OCEL 2.0 (`Ocel20` witness)
- The model was sound (`WfNetConst<Sound>` input)
- The conformance algorithm used (alignment vs. token replay)
- The timestamp of computation

---

### Step 7: Board Claim

The board claim is now attested:

```
"Invoice processing conforms 94.2% to process model."

Evidence chain:
  Raw OCEL 2.0 export
  → Evidence<OcelLog, Raw, Ocel20>
  → Evidence<OcelLog, Parsed, Ocel20>       [structural parse]
  → Admission<OcelLog, Ocel20>              [law admission, Ocel20 witness]
  → alignment against WfNetConst<Sound>     [A*-based alignment]
  → Metric<FITNESS, 942, 1000>              [Between01 type law: 942/1000 ∈ [0,1]]
  → Receipt<Metric<FITNESS,942,1000>, Ocel20> [Ocel20 witness threaded to receipt]
  → Board claim: 94.2% conformance, IEEE OCEL 2.0 attested
```

---

## GAP_001 Summary

| Aspect | Current (defective) | Required (correct) |
|---|---|---|
| wasm4pm input type | `EventLog` (flat) | `Admission<OcelLog, Ocel20>` |
| Model input type | `PetriNet` (unsound allowed) | `WfNetConst<Sound>` |
| Output type | `f64` (unbounded) | `Metric<FITNESS, NUM, DEN>` with `Between01` |
| Witness threading | None | `Ocel20` carried to receipt |
| Receipt | None | `Receipt<Metric<...>, Ocel20>` |
| Board claim attestation | Unattested | Attested to OCEL 2.0 |

**Fixing GAP_001** requires modifying wasm4pm's conformance API to accept `Admission<OcelLog, Ocel20>` as input and thread the `Ocel20` witness through to the receipt. This is a breaking API change in wasm4pm, but it is required for any board claim to be formally attested.
