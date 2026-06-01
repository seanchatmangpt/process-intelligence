# no-dashboard-truth.md

**Authority:** `/Users/sac/process-intelligence/otel-weaver/doctrine`  
**Status:** ALIVE — anchored at NO_DASHBOARD_TRUTH_001  

---

## The Illusion of Visual Agreement

Organizations frequently make the mistake of using dashboards as their primary records of compliance and performance. We reject this practice:

> **A dashboard is a projection without a receipt. Ground truth is an auditable, replayable evidence chain.**

Dashboards aggregate data to create visual representations—charts, heatmaps, and KPIs. They are designed for human consumption and quick interpretation. However, they lack the structural authority to prove anything. 

In due diligence, auditing, or legal reviews, a dashboard printout is merely an assertion. It does not carry proof of its own derivation.

---

## Dashboards as Lossy Projections

A dashboard is the final, most lossy layer of the process intelligence stack. 

To display a metric like "95% Conformance Rate" on a dashboard, the system must project thousands of high-fidelity, object-centric event logs through multiple aggregation, filtering, and rendering steps. 

```
[ Real Execution Logs ]
        │  (Lossy Filter)
        ▼
[ Aggregated Metric Streams ]
        │  (Visualization Engine)
        ▼
[ Dashboard Chart ] ◄── This is NOT ground truth.
```

In this pipeline, critical details are lost:
- **Causality**: Handovers, concurrency, and temporal order of events are flattened into static averages.
- **Traceability**: Individual outliers and non-conforming paths are hidden inside statistical aggregates.
- **Accountability**: There are no signatures, witness markers, or names of active laws attached to a bar chart.

A dashboard cannot be replayed. It cannot be mathematically verified. It is an interpretation, not evidence.

---

## What Ground Truth Is: The Replay Test

Ground truth in process intelligence is defined by the **Replay Test**:

> *Can an independent third-party auditor take the raw execution logs, apply the named laws, re-run the conformance checker, and produce the identical receipted state without relying on your system's visualizations?*

If yes, the system has ground truth. If no, the system only has descriptions.

To satisfy the Replay Test, every claim must be backed by a registry of receipts. If a slide or a dashboard claims a conformance rate of $0.95$, there must exist a specific `Receipt<ConformanceSummary, W>` where the witness `W` is a mathematically verifiable conformance algorithm (e.g., `WfNetSoundness` or `OcelReplayEngine`), pointing to the exact hashes of the raw logs.

---

## The Dashboard Challenger Question

To test if a system is built on process truth or visual illusion, ask the Challenger Question:

> *"If we select a single data point on your dashboard representing a compliance failure from six months ago, can your system automatically produce the cryptographic receipt, the witness signature, the exact state of the Petri net at the moment of failure, and the raw trace logs to prove it occurred, or does it require database archaeology?"*

If the system relies on dashboard visualization, the response will be manual query building and log hunting. If the system is built on process intelligence, the response is an immediate, machine-verifiable evidence path.

---

## References

- [doctrine/RECEIPT_DOCTRINE.md](file:///Users/sac/process-intelligence/doctrine/RECEIPT_DOCTRINE.md)
- [doctrine/PROCESS_INTELLIGENCE_IS_NOT.md](file:///Users/sac/process-intelligence/doctrine/PROCESS_INTELLIGENCE_IS_NOT.md)
- [ma/define_board-admissible_claim_requirements.md](file:///Users/sac/process-intelligence/ma/define_board-admissible_claim_requirements.md)
