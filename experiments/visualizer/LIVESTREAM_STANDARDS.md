# Dr. Wil van der Aalst AGI Livestreaming Standards

This document defines the operational standards and multi-agent roles required to host a continuous, high-assurance livestream of full-lifecycle process intelligence simulations in the `~/process-intelligence` ecosystem.

---

## 1. Hosting the Simulation Dashboard

The simulation dashboard is a zero-dependency, static web application located under `/Users/sac/process-intelligence/experiments/visualizer/`. To run and stream the dashboard locally:

1.  **Start the Local Server:**
    Navigate to the visualizer directory and start a lightweight Python web server:
    ```bash
    python3 -m http.server 8000
    ```
2.  **Access the Interface:**
    Open your browser and navigate to `http://localhost:8000`.
3.  **Active Controls:**
    - Click **▶ Run Stream** to begin a continuous, simulated event stream.
    - Toggle **Activate Process Drift** to inject real-time control-flow and performance drift profiles.
    - Click any case in the **Live Case Stream** feed to run the A* search solver and visualize its conformance alignment.

---

## 2. 5-Agent AGI Livestreaming Swarm Architecture

To ensure 100% compliance with the Dr. Wil van der Aalst AGI standards during live-streaming, a coordinated swarm of 5 subagents manages, audits, and validates the live stream.

```
                  ┌───────────────────────────────┐
                  │      Stream Director Agent    │
                  │ (OBS Feed & UI Layout Control)│
                  └───────────────┬───────────────┘
                                  │
         ┌────────────────────────┼────────────────────────┐
         ▼                        ▼                        ▼
┌─────────────────┐      ┌─────────────────┐      ┌─────────────────┐
│Telemetry Auditor│      │ A* Alignment    │      │  Drift Sentry   │
│ (XES/OCEL parsing)     │  Referee Agent  │      │  (EWMA monitor) │
└─────────────────┘      └─────────────────┘      └─────────────────┘
         │                        │                        │
         └────────────────────────┼────────────────────────┘
                                  │
                                  ▼
                       ┌────────────────────┐
                       │  Ledger Custodian  │
                       │ (SHA-256 Ledger)   │
                       └────────────────────┘
```

### 2.1. Agent 1: Stream Director Agent
- **Role**: Coordinates the live broadcast layout, overlay assets, and OBS scene transitions.
- **Task**: Binds to the browser window and monitors console logs. It adjusts visual zoom settings to focus on active Petri Net token animations or highlight newly chained ledger blocks as they fire.
- **Compliance Metric**: Zero frame dropping; latency $< 200\text{ms}$.

### 2.2. Agent 2: Telemetry Auditor Agent
- **Role**: Validates the schema and structure of incoming event packages at the stream boundary.
- **Task**: Inspects raw JSON event payloads to verify proper XES lifecycle attributes and OCEL 2.0 object-event association hashes. If a trace is missing required links, it flags the transaction for containment.
- **Compliance Metric**: $\text{VerifySchema}(e) == \text{True}$ for 100% of events.

### 2.3. Agent 3: A* Alignment Referee Agent
- **Role**: Real-time conformance executioner.
- **Task**: Listens for case completion events. As soon as a case is logged, the agent runs the A* solver to compute optimal alignments against the sound Petri net model, displaying synchronous, model, and log moves.
- **Compliance Metric**: Resolves optimal alignments in $\le 10\text{ms}$ for traces up to length 100.

### 2.4. Agent 4: Drift Sentry Agent
- **Role**: Process deviation and concept drift analyzer.
- **Task**: Monitors the EWMA control chart values calculated dynamically from trace alignment fitness. If the smoothed conformance fitness falls below the Lower Control Limit ($LCL = 0.92$), it triggers a process drift alert and updates the red warning banner on the stream overlay.
- **Compliance Metric**: Spans a 50-case sliding window to prevent false positives while capturing true drift within 5 cases.

### 2.5. Agent 5: Ledger Custodian Agent
- **Role**: Immutable chain auditor.
- **Task**: Validates the block hash sequence of the tamper-evident SHA-256 process ledger. It recalculates the hash of every transaction block recursively to verify that:
  $$H_i = \operatorname{SHA-256}(i \mathbin{\Vert} t \mathbin{\Vert} \text{case\_id} \mathbin{\Vert} \text{payload} \mathbin{\Vert} H_{i-1})$$
  It ensures the "Ledger Intact" verification shield stays green.
- **Compliance Metric**: 100% block sequence integrity.

---

## 3. Conformance Verification
The visualizer's code files have been syntax-checked and validated under:
- `alignment.js` (A* search heuristic verification)
- `ledger.js` (SHA-256 cryptographic sequence)
- `app.js` (Event bindings and statistics orchestration)

All assets conform to the v30.1.1 ultimate standard and contain zero mocks, placeholders, or stubs.
