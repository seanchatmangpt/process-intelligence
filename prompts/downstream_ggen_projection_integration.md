# Downstream Integration Directive: ggen Generative Projections

This document establishes the architecture and execution directives for the `ggen` generative projection engine. `ggen` is responsible for manufacturing audit-ready outputs, PowerPoint presentations, and buyer-reliance diligence documents directly from formal process mining execution evidence.

## 1. Slide-to-Receipt Mapping Architecture
Every downstream presentation, deck, or text report produced by `ggen` must map each qualitative or quantitative claim to a concrete, cryptographic execution receipt.
- **Assertion Anchors**: For any slide claiming operational metrics (e.g., "92% trace compliance on Order-to-Cash process" or "EBITDA leak of $1.2M due to unauthorized approval loop"), `ggen` must inject a metadata block (JSON/XML) containing:
  1. The hash of the raw event log ($H_{log}$).
  2. The hash of the process model ($H_{model}$).
  3. The alignment matrix hash ($H_{alignment}$).
  4. The fitness calculation witness ($f \in [0, 1]$).
  5. The cryptographic signature of the wasm4pm execution core that computed the metric.
- **Slide-to-Receipt Registry**: `ggen` must compile a master index matching slide identifiers (e.g., Slide 4, Bullet 2) to their corresponding execution receipts.

## 2. Diligence and Buyer Reliance Evidence
- **Defensibility Audits**: The generated reports must be mathematically defensible in M&A diligence. If a buyer audits a claim, they must be able to run `wasm4pm` with the target log and model, and reproduce the exact metric printed on the slide.
- **Process Debt and Risk Taxonomy**: Map identified process deviations to financial risk categories:
  - *Process Debt*: Cumulative cost of workaround paths (e.g., redundant approvals).
  - *Process Risk*: Probability of regulatory violations (e.g., four-eyes principle bypass).
  - *Process Residual*: Gaps that cannot be automated due to system limitations.

## 3. Projection Target Formats and Equations
Downstream generation tools must calculate the following metrics:
- **Synergy Projection Value**:
  $$V_{\text{synergy}} = \sum_{i \in \text{parallelizable}} \text{latency}_i \times \text{frequency}_i \times (1 - \text{dependency\_coeff}_i)$$
- **Process Debt Value**:
  $$V_{\text{debt}} = \sum_{j \in \text{non\_conforming}} \text{rework\_cost}_j \times \text{recurrence}_j \times (1 - \text{insurance\_coverage}_j)$$
- **Control-Flow Compliance**:
  $$\operatorname{Compliance} = \frac{|L_{\text{conforming}}|}{|L_{\text{total}}|}$$

Target formats include:
- **OpenXML PowerPoint Generation**: Generate `.pptx` presentations with embedded slide-to-receipt metadata.
- **PDF Diligence Reports**: Generate print-ready PDF reports with QR codes or cryptographic hyperlinks pointing to the verification registry.
- **Structured JSON Manifests**: Compile all project variables, logs, models, and alignment receipts into a single ZIP archive for buyer ingestion.

## 4. Downstream Integration and Traceability
All implementation details must align with:
- [ggen_projection_sample.md](file:///Users/sac/process-intelligence/experiments/ggen_projection_sample.md)
- [ma-ready-powerpoint.md](file:///Users/sac/process-intelligence/doctrine/ma-ready-powerpoint.md)