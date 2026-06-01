# Buyer Reliance Requirements

During corporate transactions, the buyer's due diligence team must verify all seller assertions to justify valuations, quantify synergies, and identify operational liabilities. For a buyer to legally rely on process intelligence outputs, the target's process mining data room must meet the stringent requirements defined in this document.

## 1. The Independence and Replication Rule

A buyer cannot rely on a static PDF report or a PowerPoint presentation alone. Buyer reliance requires **independent reproducibility**.

* **Independent Execution**: The buyer's advisors must be able to ingest the target's raw event logs (IEEE XES or OCEL 2.0 format) and execute the exact conformance and performance queries on a neutral, independent runtime engine (such as the `wasm4pm` execution core).
* **Tolerance Boundary**: The conformance metrics (fitness, precision) calculated by the buyer's independent audit must match the seller's claims within an absolute tolerance limit:
  $$\left| f_{\text{buyer}} - f_{\text{seller}} \right| \le 10^{-6}$$
  Any deviation larger than $10^{-6}$ triggers a diligence exception, rejecting the claim's admissibility.

## 2. Virtual Data Room (VDR) Completeness

The seller must populate the VDR with the following technical artifacts under a dedicated `/process-intelligence/` directory:

1. **Source Event Logs**: Complete, unredacted event logs containing all object types, attributes, and transitions.
2. **Process Models**: Mined and verified process models (expressed in BPMN 2.0, Petri Nets, or POWL format) representing the "As-Is" state.
3. **Alignment Metadata**: Verification receipts, conformance scripts, and logs showing optimal alignment calculations as defined in [Adriansyah 2014](file:///Users/sac/process-intelligence/sources/papers/paper-canon.md).
4. **Data Extraction Queries**: The SQL, OCPQ (Object-Centric Process Query), or API scripts used to pull event data from source ERP/CRM systems (e.g., SAP, Salesforce) to verify data lineage.

## 3. Data Cleaning and Preprocessing Transparency

To prevent "process washing" or the selective exclusion of unfavorable operational bottlenecks, the seller must adhere to the following rules:

* **Raw-to-Filtered Mapping**: The seller must provide both the raw event log ($L_{\text{raw}}$) and the filtered/cleaned log ($L_{\text{clean}}$) used for model mining.
* **Delta Verification**: A delta analysis report must detail every event and trace removed during cleaning, along with a formal justification (e.g., system-generated test cases, known network failures).
* **No Unsanctioned Filtering**: Traces with low conformance or high latency cannot be filtered out unless they violate a formal business constraint that is programmatically checked and signed off by the target's Chief Compliance Officer.

## 4. Log Representativeness and Coverage Bounds

To ensure the process intelligence reflects stable, representative operations:
* **Time Horizon**: The event log must cover a continuous, representative period of at least 12 months, capturing seasonal fluctuations and full financial close cycles.
* **Volume Coverage**: The log must capture at least 98% of all completed transactions (by count and monetary value) within the scope of the diligence claim (e.g., Order-to-Cash, Procure-to-Pay).

## 5. Related M&A Validation Documents

* For the seller's requirements to defend their claims, see [Seller Defensibility Requirements](file:///Users/sac/process-intelligence/ma/define_seller_defensibility_requirements.md).
* For the mathematical and cryptographic definition of slide assertions, see [Board-Admissible Claim Requirements](file:///Users/sac/process-intelligence/ma/define_board-admissible_claim_requirements.md).
* For the step-by-step audit path, see [Auditor Evidence Path](file:///Users/sac/process-intelligence/ma/define_auditor_evidence_path.md).
* For linking slides to concrete replay logs, see [Slide-to-Replay Map](file:///Users/sac/process-intelligence/ma/define_slide-to-replay_map.md).