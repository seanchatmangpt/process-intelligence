# Receipt Doctrine for Process Intelligence Accountability

## What a Receipt Is Not

A receipt is not a hash. A hash proves integrity of bytes. A receipt proves something richer: that a named law was applied, by a named witness, to a typed result, within a lawful object lifecycle.

## What a Receipt Is

A receipt is a typed, witnessed, bound evidence artifact.

```
Receipt<T, W>
  T = the result type (what was produced)
  W = the witness marker (which law produced it)
```

The witness `W` is not a string. It is a zero-sized type that names a specific paper, standard, or law (e.g., `Ocel20`, `Xes1849`, `WfNetSoundnessPaper`). Two receipts with different witnesses are different types at compile time — they cannot be confused or substituted.

## What Receipts Enable

1. **Independent verification** — Any auditor can re-run the admission path and produce the same receipt type. The type is the claim; the claim is checkable.
2. **Chain-of-custody** — Each lifecycle transition leaves a receipt. The chain of receipts is the audit trail.
3. **Board claim traceability** — "Our process fitness is 0.95" becomes: which `Receipt<ConformanceResult, WfNetSoundnessPaper>` backs that claim? If none exists, the claim is inadmissible.
4. **M&A diligence** — Due diligence buyers receive receipts, not assertions. A receipt registry is a machine-verifiable diligence package.

## What Receipts Prevent

- **"Trust me" claims** — A claim without a receipt is not a claim; it is an opinion.
- **Silent loss** — Every lossy projection emits a `LossReport`. The receipt carries the report or the receipt does not exist.
- **Unnamed projection** — `ProjectionName` is a `&'static str` newtype. Projections without names cannot produce receipts.
- **Raw laundering** — `wasm4pm` consumes `Admitted` evidence. A raw `EventLog` cannot enter the conformance path. No receipt can be produced from unadmitted input.

## The Receipt Is the Unit of Process Intelligence Accountability

Every board-level claim in a process intelligence program must be traceable to a receipt. If the receipt does not exist, the claim does not exist.

This is not a QA policy. It is a type law: the receipt is the type; the type is the proof; the proof is the accountability surface.

## Canonical Receipt Format

```
name: <RECEIPT_NAME>
produced_by: <operator or module>
witness: <W — the named law>
result_type: <T>
criteria_met: [list of criteria satisfied]
gaps_remaining: [list of open items]
timestamp_ns: <nanosecond-precision timestamp>
```

## References

- van der Aalst, W.M.P. — Object-Centric Process Mining (OCEL 2.0)
- wasm4pm-compat `src/receipt.rs` — canonical `Receipt<T, W>` implementation
- wasm4pm-compat `src/witness.rs` — witness marker registry
- wasm4pm-compat `src/loss.rs` — `LossReport` covenant
