# Process Intelligence Simulation Dashboard

This dashboard is a high-fidelity, interactive simulator built using vanilla HTML5, CSS3, and JavaScript. It provides a visual representation of process mining research, implementing token game replay, LTL-based Declare constraint validation, A* process alignment calculations, EWMA concept drift detection, and SHA-256 cryptographic audit ledger verification.

## Core Component Architectures

Each component is implemented with production-grade completeness, avoiding mocks and placeholders:

1. **LTL Declare Constraint Engine**
   The Declare validator parses declarative LTL-based rules and evaluates live execution traces. It calculates activations, satisfactions, and violating event indices.
   Core Validator Implementation: file:///Users/sac/process-intelligence/experiments/visualizer/declare.js

2. **A* Process Alignment Solver**
   Computes the optimal path aligning observed event logs with the formal process model. It distinguishes synchronous moves (zero cost), log-only moves (cost of 1), and model-only moves (cost of 1) using a priority-queue-driven search.
   Alignment Solver: file:///Users/sac/process-intelligence/experiments/visualizer/alignment.js

3. **EWMA Concept Drift Detector**
   Monitors streaming process execution times to flag latency shifts. It computes the Exponentially Weighted Moving Average (EWMA) with dynamic control limits (UCL/LCL) based on historical baselines.
   Drift Detector: file:///Users/sac/process-intelligence/experiments/visualizer/drift.js

4. **Cryptographic Event Chain Ledger**
   Constructs an immutable ledger of process events using SHA-256 hashing. Each block contains a back-link to the previous hash, preventing silent tampering.
   Cryptographic Ledger: file:///Users/sac/process-intelligence/experiments/visualizer/blockchain.js

5. **Petri Net Token Game Simulator**
   Defines the coordinates, arc directions, and place/transition nodes representing a Loan Approval Process, animating token movements dynamically.
   Petri Net Model: file:///Users/sac/process-intelligence/experiments/visualizer/petrinet.js

6. **User Interface and Layout**
   Built as a responsive single-page dashboard featuring a dark cyberpunk glassmorphism theme.
   Dashboard Markup: file:///Users/sac/process-intelligence/experiments/visualizer/index.html
   Dashboard Styles: file:///Users/sac/process-intelligence/experiments/visualizer/styles.css
   Main Controller: file:///Users/sac/process-intelligence/experiments/visualizer/app.js

## Features & Simulation Scenarios

- **Standard Execution**: Fires transitions sequentially, compliant with all predefined Declare rules.
- **Fast Reject Short-Circuit**: Skips credit check and income verification, violating `Existence(CheckCredit)` and `Precedence(CheckCredit, Reject)`.
- **Out-of-Order Execution**: Forces a credit check before request receipt, triggering violations of `Init(ReceiveRequest)` and `Precedence(ReceiveRequest, CheckCredit)`.
- **Drift & Latency Event**: Compliance remains high, but delays are injected into processing durations. The EWMA statistic will rise, crossing the UCL and triggering a red neon drift warning.
- **Ledger Tampering Attack**: Mutates historical block values to show how the SHA-256 integrity chain breaks, highlighting the tampered segments in red.
