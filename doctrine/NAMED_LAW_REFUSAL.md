# Named Law Refusal Doctrine

## The Problem with Generic Errors

```rust
Err("invalid input")
```

This is unfalsifiable. It is not actionable. An auditor cannot determine:
- Which structural law was violated
- Which object was non-conforming
- Which paper or standard defines the violation
- Whether the error is a defect in the data or a defect in the code

Generic error strings are not process evidence. They are noise.

## The Named Law Alternative

```rust
Err(Refusal<DanglingEventObjectLink, Ocel20>)
```

This is specific. It is auditable. It is named. An auditor can determine:
- `DanglingEventObjectLink` — the exact structural law violated: an event references an object that does not exist in the log
- `Ocel20` — the witness: OCEL 2.0 standard defines this constraint
- The `Refusal` type is the defect; the defect has a named reason type; the reason type is checkable at compile time

## The Van der Aalst Principle

> "A conformance problem is a defect, not a discrepancy."
> — van der Aalst, Object-Centric Process Mining

Applied to admission: a refusal is a named defect, not a generic failure. The defect must be specific enough to be assigned, tracked, and resolved.

## The Forbidden Pattern

```rust
// FORBIDDEN: unfalsifiable, not actionable
Err(Refusal::<InvalidInput, _>::new("something went wrong"))

// FORBIDDEN: string-typed catch-all
fn admit(&self) -> Result<_, String>
```

Every refusal must carry a **specific named reason type** as its first type parameter. `InvalidInput` or string-typed catch-alls are defects in the admission code, not in the data.

## Named Law Inventory

The following named laws are in current use across the wasm4pm-compat type system:

| Named Law | Standard/Paper | What It Names |
|---|---|---|
| `DanglingEventObjectLink` | OCEL 2.0 | Event references non-existent object |
| `MissingFinalMarking` | Petri net soundness | Workflow net has no reachable final marking |
| `EmptyTrace` | XES 1849 | Trace contains no events |
| `MissingCaseId` | XES 1849 | Event log trace has no case identifier |
| `InvalidTimestampOrder` | OCEL 2.0 | Events within an object have non-monotone timestamps |
| `UnsoundWfNet` | van der Aalst soundness criterion | Workflow net fails soundness proof |
| `ProjectionNameRequired` | wasm4pm-compat loss covenant | Projection attempted without a `ProjectionName` |

## The Compile-Time Guarantee

Because refusal reason types are type parameters, the compiler enforces the law:
- You cannot construct a `Refusal<DanglingEventObjectLink, Ocel20>` unless `DanglingEventObjectLink` is a named type
- You cannot catch a refusal without naming what you are catching
- The test suite can enumerate all named laws and verify each has a corresponding compile-fail fixture

## References

- wasm4pm-compat `src/admission.rs` — `Refusal<R, W>` type definition
- wasm4pm-compat `tests/ui/compile_fail/` — compile-fail fixtures for each named law
- OCEL 2.0 standard — structural constraints for object-centric event logs
- van der Aalst — Process Mining: Data Science in Action (soundness criterion)
