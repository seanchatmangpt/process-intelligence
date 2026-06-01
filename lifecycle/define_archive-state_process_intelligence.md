# Lifecycle: Define Archive-State Process Intelligence

The **Archive State** governs the long-term, read-only retention of historical process logs, structural models, and decommissioning receipts.

## Autonomic MAPE-K Mapping
* **Loop Role**: **Knowledge** (long-term memory)
* **Responsibility**: In the Knowledge phase, retired or historical execution data is compressed, structured, and stored for retrospective audit, compliance verification, and future baseline comparison.
* **Actuation Trigger**: Activated immediately following the successful completion of the **Decommissioning Stage** and the verification of the decommissioning receipt.

---

## Archival Formats and Compression Schemas

To minimize storage overhead while maintaining total auditability, the process engine uses standardized schemas:

### 1. OCEL 2.0 SQLite / Parquet Schema
Object-Centric Event Logs (OCEL 2.0) are archived using dual formats:
* **OCEL SQLite**: For active, indexed query access. It uses a relational database containing event tables, object tables, and relationship tables (e.g. event-to-object links).
* **OCEL Parquet**: For long-term cold storage. Events and objects are exported into columnar Parquet files, optimized for highly efficient compression and fast analytics queries using engines like DuckDB.

### 2. XES Compression Policy
Flat event streams are stored in compressed GZipped-XES (`.xes.gz`) formats, reducing storage size by up to 90% while adhering to the IEEE XES standard.

### 3. Archive Query Protocol
Auditors query the archive using standard **OCPQ (Object-Centric Process Query)** or SQL. An archive-state process record remains discoverable and query-compatible:
```sql
SELECT event_id, event_activity, object_id 
FROM ocel_events 
JOIN ocel_event_object ON ocel_events.event_id = ocel_event_object.event_id
WHERE ocel_events.event_timestamp BETWEEN :start AND :end;
```

---

## M&A Diligence Claims
In M&A, the Archive State represents the **Audit Trail Defensibility**.
* **Buyer Reliance**: The buyer relies on the archive to verify historical operating metrics (e.g. 5-year compliance track record) and to prove to regulators that all data retention laws are satisfied.
* **Slide-to-Receipt Map**: Slides asserting "We have a fully compliant 7-year audit history for all customer billing transactions" must map to Archive State storage records containing signed XES/OCEL files and their associated verification keys.

---

## Related Documents
* See the [Decommissioning Stage](file:///Users/sac/process-intelligence/lifecycle/define_decommission-state_process_intelligence.md) for pre-archiving steps.
* See the [Final Receipt State](file:///Users/sac/process-intelligence/lifecycle/define_final_receipt_state.md) for the audit format.
* Back to [Lifecycle README](file:///Users/sac/process-intelligence/lifecycle/docs-law__lifecycle_readme.md).