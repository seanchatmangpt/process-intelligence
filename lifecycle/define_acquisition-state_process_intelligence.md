# Lifecycle: Define Acquisition-State Process Intelligence

The **Acquisition State** governs the ingestion, evaluation, and baselining of processes belonging to an acquisition target during M&A due diligence.

## Autonomic MAPE-K Mapping
* **Loop Role**: **Knowledge** (initial discovery and baselining)
* **Responsibility**: Map raw, unstandardized transaction records from the target company's systems into a structured event log to construct the "as-is" process baseline.
* **Actuation Trigger**: Initiated upon signing the Letter of Intent (LOI) to start operational due diligence.

---

## Ingestion and Discovery Protocol

During an acquisition, the target's process flows are often poorly documented. The acquisition-state process intelligence engine reconstructs these flows:

### 1. Ingestion Pipeline
1. **Raw Log Extraction**: Extract database transactional logs (e.g. ERP, CRM tables, audit trails) containing at least:
   * **Case ID**: A unique identifier for the transaction instance (e.g. `order_id`).
   * **Activity**: The name of the event (e.g. `approve_invoice`).
   * **Timestamp**: Millisecond-precision start and end times.
2. **XES/OCEL Conversion**: Schema-map the database tables to XES or OCEL format.
3. **Heuristics Discovery**: Execute a discovery algorithm (e.g., Heuristics Miner or Inductive Miner with high noise filtering) to generate the initial "as-is" Petri Net $N_{as\_is}$.

### 2. Baseline Conformance and Risk Audit
The ingested process is evaluated to establish the **Valuation Baseline**:
* **Baseline Fitness**: Calculate $f(L_{target}, N_{as\_is})$. If the target's own procedures do not align with their actual logs, it indicates low operational control.
* **Baseline Process Debt**: Quantify the target's process debt $D_p$. High process debt represents immediate integration costs.
* **Compliance Risk Analysis**: Run trace alignment checking against regulatory reference models to flag potential compliance violations.

---

## M&A Due Diligence Application

In M&A, the Acquisition-State defines the **Operational Discount** or **Risk Premium**:
* **Buyer Reliance**: The buyer's valuation model relies on these baseline numbers to verify the seller's claims about processing speed and compliance.
* **Slide-to-Receipt Map**: PowerPoint slides claiming "The target operates a highly standardized manufacturing pipeline with 99% compliance" must resolve to an Acquisition-State receipt containing the heuristics discovery log, the exact DFG, and the alignment compliance report of the target's systems.

---

## Related Documents
* See the [Integration Stage](file:///Users/sac/process-intelligence/lifecycle/define_integration-state_process_intelligence.md) for post-acquisition merging.
* See the [False-Claim Taxonomy](file:///Users/sac/process-intelligence/lifecycle/define_false-claim_taxonomy.md) to audit seller claims.
* Back to [Lifecycle README](file:///Users/sac/process-intelligence/lifecycle/docs-law__lifecycle_readme.md).