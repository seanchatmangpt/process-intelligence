# Archive Phase: Process Intelligence Lifecycle

## Definition

The archive phase is the terminal lifecycle phase in a process intelligence program. It is the phase in which:
- The final OCEL log is sealed and made immutable
- The final process model is committed with its discovery receipt
- The conformance report is produced against the final model
- All graduation receipts are collected into a single diligence package

The archive is not storage. The archive is the audit trail.

## Archive Requirements

### 1. Machine-Readable Format

The archive must be machine-readable without bespoke tooling:
- OCEL log: OCEL 2.0 JSON or XML format
- Process model: PNML (Petri net) or BPMN 2.0 XML
- Conformance report: structured JSON with fitness, precision, generalization, simplicity scores
- Graduation receipts: typed JSON following the receipt format in `receipts/RECEIPT_REGISTRY.md`

### 2. Verifiable Receipts

Every artifact in the archive must have a corresponding receipt:

```
archive/
  ocel_final.json              → receipt: OCEL_FINAL_RECEIPT
  model_final.pnml             → receipt: MODEL_FINAL_RECEIPT
  conformance_report.json      → receipt: CONFORMANCE_RECEIPT
  graduation_receipts/
    rec_fitness_001.json
    rec_variant_001.json
    rec_sla_001.json
    ...
```

A receipt without a corresponding artifact is incomplete. An artifact without a receipt is not part of the archive.

### 3. OCEL Compliance

The final OCEL log must pass `OcelLog::admit()` without refusal. An OCEL log that fails admission cannot be archived — the archive would be built on non-conforming evidence.

Pre-archive admission check:
```rust
let admitted = Admit::admit(Evidence::<OcelLog, Raw, Ocel20>::raw(final_log))?;
// If this returns Err(Refusal<_, Ocel20>), the log must be fixed before archival
```

### 4. Conformance Report Requirements

The conformance report must include:
- **Fitness** — token replay fitness against the reference model
- **Precision** — how much of the model behavior is observed in the log
- **Generalization** — how well the model generalizes beyond the log
- **Simplicity** — model complexity score
- **Variant inventory** — all process variants with frequency counts
- **Deviation report** — named deviations from the model, with event-level evidence

### 5. Graduation Receipts

Graduation receipts certify that a process intelligence artifact is ready for wasm4pm:
- Each receipt carries a `GraduationReason` (Hard | Soft | Conditional)
- Hard signals: type-safe, fully admitted, conformance-verified, receipted
- Soft signals: partial evidence, gap documentation included
- Conditional: explicit residual list with resolution criteria

## The Archive as M&A Diligence Package

The archive IS the audit trail for M&A diligence.

A diligence buyer receives:
1. The archive directory (machine-readable artifacts)
2. The receipt registry (named evidence for each artifact)
3. The conformance report (independent process truth)
4. The gap registry (honest inventory of what is not yet complete)

A diligence buyer does NOT receive:
- Assertions without receipts
- Conformance scores from unadmitted logs
- Process models without fitness evidence
- Claims that cannot be traced to a named witness

## Archive Phase Entry Criteria

A program may not enter the archive phase unless:
- All lifecycle phases prior to archive have completion receipts
- The OCEL log passes admission without refusal
- At least one conformance check has been run against the admitted log
- The receipt registry is complete (no entries with unknown `produced_by`)

## Archive Phase Exit: Downstream Authorization

A sealed archive authorizes:
- Board-level fitness and variant claims (traceability to archive receipts)
- M&A diligence delivery (archive package is the deliverable)
- wasm4pm graduation (graduation receipts in archive authorize engine integration)
- ALIVE gate certification (archive completion is a criterion for ALIVE_001)

## References

- receipts/RECEIPT_REGISTRY.md — receipt format and registry
- doctrine/RECEIPT_DOCTRINE.md — why receipts are the unit of accountability
- lifecycle/define_archive-state_process_intelligence.md — prior definition (see also)
- wasm4pm-compat `src/graduation.rs` — `GraduationCandidate`, `GraduationReason`
- OCEL 2.0 standard — machine-readable log format specification
