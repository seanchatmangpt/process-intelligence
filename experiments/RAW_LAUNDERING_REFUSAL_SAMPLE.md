# Raw Laundering Refusal vs Compat-Law Admission: Sample

## The Raw Laundering Scenario

```python
# wasm4pm usage WITHOUT compat-law admission (laundering path)
raw_log = EventLog.from_csv("process_data.csv")  # malformed events present
result = wasm4pm.conformance(raw_log, reference_model)
print(result.fitness)  # → 0.95
```

**What happened:** `raw_log` contains malformed events — specifically, events that reference object IDs that do not exist in the log. These dangling links inflate apparent event density, which skews the token-replay alignment. The 0.95 fitness figure is computed from garbage input.

**The board claim:** "Our process fitness is 0.95 as validated by wasm4pm."

**The audit finding:** No admission gate was applied. No named law was checked. The 0.95 is not evidence. It is a number derived from unadmitted input. The claim is inadmissible.

## Why This Is "Raw Laundering"

Raw laundering is the pattern where unadmitted, potentially non-conforming data is passed directly to a conformance engine, laundering it into a board-visible metric. The metric looks authoritative. The data was never checked.

This is analogous to financial laundering: unverified funds enter a clean-looking system and emerge as verified output.

## The Refused Alternative

```rust
// wasm4pm-compat admission path
let raw = Evidence::<OcelLog, Raw, Ocel20>::raw(ocel_log);
let admitted = Admit::admit(raw)?;
// If the log contains DanglingEventObjectLink, admit() returns:
// Err(Refusal<DanglingEventObjectLink, Ocel20>)
```

**What happened:** `OcelLog::admit()` checks every event-object link against the object registry. The first dangling link produces `Err(Refusal<DanglingEventObjectLink, Ocel20>)`. The admission fails with a named law, not a generic error.

**The conformance check never runs.** Because `wasm4pm` is typed to consume `Admission<OcelLog, Ocel20>`, not a raw `OcelLog`, the conformance call is not reachable without a successful admission. This is enforced at compile time.

**The board claim that was not made:** none. The refusal is evidence that the input is non-conforming. The next step is: fix the dangling links, re-admit, then run conformance.

## This Is Correct Behavior

The 0.95 that was never computed is more valuable than the 0.95 that was. The refusal is actionable:
- `DanglingEventObjectLink` names the defect
- `Ocel20` names the standard that defines it
- The event and object IDs involved are available from the `Refusal` payload
- A data engineer can fix the source export and re-run

## Why wasm4pm Must Consume Admitted Evidence

If `wasm4pm.conformance()` accepted raw `EventLog`:
1. Every caller must independently implement admission logic — inconsistently
2. Conformance results are not comparable across callers (different admission criteria)
3. Board claims cannot be traced to a specific admission gate
4. Raw laundering becomes the default path, not an exception

When `wasm4pm.conformance()` is typed to `Admission<OcelLog, Ocel20>`:
1. Admission is mandatory, not optional
2. The witness `Ocel20` is part of the conformance result type — results are comparable
3. Every conformance result is traceable to the same named law
4. Raw laundering is a compile error

## References

- wasm4pm-compat `src/admission.rs` — `Admit::admit()` contract
- wasm4pm-compat `src/evidence.rs` — `Evidence<T, Raw, W>` → `Admission<T, W>` lifecycle
- doctrine/RECEIPT_DOCTRINE.md — why receipts prevent raw laundering
- doctrine/NAMED_LAW_REFUSAL.md — why `DanglingEventObjectLink` is the right error type
