# c8-core

Foundation Module for Construct8 Market Physics.

## Why Logic-Chaos Is Disqualified from c8-core

**Logic-chaos** refers to uncontrolled string-based decisions, open-ended enum variants,
and silent failures that escape the type system. It is incompatible with c8-core because:

1. **Unbounded decisioning**: Strings, open-ended variants, and catch-all patterns
   allow unknown verdicts to propagate silently. We require **bounded enums** where
   every variant is named and auditable.

2. **Refusal signals are first-class**: When logic cannot proceed (e.g., Construct8Len
   of 9 or higher), c8-core rejects at the type level using `Err(NeedNine)`, not with
   panic or silent default values. The refusal **is the signal**.

3. **Zero-cost enforcement**: Type tags (`C8Id`, `NodeId`, etc.) carry no runtime data.
   Adding string-based routing or dynamic dispatch would violate the zero-cost promise.

4. **Evidence lineage**: Every decision (`HotPathVerdict`) must trace to a lawful origin.
   Open-ended logic breaks the audit trail.

Logic-chaos is therefore **forbidden**. All verdicts are bounded enums. All refusals
carry typed reasons. All constructors prove their invariants at the type level.
