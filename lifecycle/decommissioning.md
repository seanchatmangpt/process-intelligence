# DECOMMISSIONING — Process Intelligence Lifecycle Phase

## Lawful decommissioning sequence

Decommissioning is not deletion. It is a lawful termination that produces a complete, auditable
record of what the process was, how it performed, and why it was retired:

1. **Freeze the stream:** No new operational events are admitted after the decommissioning
   decision. The `EventStream` is closed — no further `append` calls.
2. **Final conformance check:** Compute fitness, precision, generalization, and simplicity on
   the complete historical log. This is the last conformance verdict the process will receive.
3. **Final alignment score:** Run alignment-based conformance (cost-optimal alignment) on the
   full log to produce a definitive alignment score. Token-based fitness is fast but approximate;
   the final score uses alignment for precision.
4. **Variant archive:** Enumerate all distinct trace variants, their frequencies, and their
   conformance status. This is the behavioral fingerprint of the process over its lifetime.
5. **Archive record:** Produce a typed archive artifact containing: the reference process model,
   the complete event log (admitted, receipted), all historical conformance reports, all repair
   receipts, the final alignment score, and the decommissioning reason.

## The archive record as provenance artifact

The archive record is the decommissioning receipt. It is:
- **Typed:** An instance of a typed structure, not a PDF or a prose document.
- **Admitted:** Passed through the standard admission path with a decommissioning witness.
- **Receipted:** A `Receipt` artifact that proves this archive was produced at this time.
- **Replayable:** The archived event log can be replayed against the archived model at any
  future time. A buyer, auditor, or regulator can reproduce the final conformance score.

The archive record is not a summary. It is a complete provenance chain.

## A decommissioned process leaves receipts that prove what happened.

This is the decommissioning covenant. A process that cannot produce this record at decommissioning
time was not managed under process intelligence discipline. The absence of a decommissioning
receipt is itself a finding — it means the process operated without provenance.

## Board claim

> We maintain complete process provenance through decommissioning.

This claim is admissible only if:
1. Every decommissioned process has an archive record that is typed, admitted, and receipted.
2. The archive record contains the full event log, model, conformance history, repair history,
   and final alignment score.
3. The archive record can be replayed: a conformance checker run against the archived log and
   model produces the same final score.

Without replayable archive records, "complete process provenance" is a governance narrative.
With replayable archive records, it is an auditor-admissible, M&A-ready claim.

## wasm4pm-compat surfaces relevant to decommissioning

| Type | Module | Decommissioning role |
|---|---|---|
| `Receipt` | `src/receipt.rs` | Typed proof of decommissioning record production |
| `Evidence<T, Receipted, W>` | `src/evidence.rs` | Final receipted state of archived evidence |
| `Admission<T, W>` | `src/admission.rs` | Admission path for the archive record itself |
| `LossReport<From, To, Items>` | `src/loss.rs` | Required if log is summarized during archival |
| `ExportBoundaryConst<HAS_WITNESS, HAS_ROUND_TRIP>` | `src/strict.rs` | Export boundary for archive handoff |

## wasm4pm obligations for full decommissioning phase

- Stream freeze: typed operation to close an `EventStream` permanently.
- Final alignment computation: cost-optimal alignment on the complete historical log.
- Archive record production: typed artifact assembly from all lifecycle evidence.
- Decommissioning receipt: a `Receipt` that seals the archive record.
- Replay verification: demonstrate that the archive is replayable before sealing.