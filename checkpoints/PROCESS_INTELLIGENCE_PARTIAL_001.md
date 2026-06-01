# Process Intelligence Status: PARTIAL_001

This checkpoint represents the initial bootstrapping and design phase of the process-intelligence repository. In this state, the foundry structure is established, but execution prompts and checkpoints are in draft or initial formats.

## 1. Bootstrapping State
- **Foundry Initialized**: The central repository structure is created, defining directories for doctrine, lifecycle rules, sources, audits, and experiments.
- **Architectural Mapping**: The roles of the downstream products are defined in [README.md](file:///Users/sac/process-intelligence/README.md):
  - `wasm4pm` (Execution Core)
  - `wasm4pm-compat` (Type-Law Foundry)
  - `ggen` (Manufacturing)

## 2. Transition Plan to ALIVE_001
To transition from `PARTIAL_001` to `ALIVE_001`, the following steps must be completed:
1. **Expand Downstream Prompts**: Replace all initial template prompts in the prompts directory with concrete, actionable instructions containing mathematical formulas, memory constraints, and type obligations.
2. **Close Type-Law Gaps**: Define the `Evidence<T, State, Witness>` information lattice and conversion loss reporting rules in `wasm4pm-compat` instructions.
3. **Specify Slide-to-Receipt Mapping**: Provide rules in `ggen` instructions linking PowerPoint and PDF diligence claims to cryptographic process mining receipts.
4. **Establish the Paper Canon Alignment**: Connect academic process mining papers (van der Aalst, Adriansyah, Leemans, Ghahfarokhi) to executable test fixture directives.
5. **Verify Link Integrity**: Remove all incomplete elements and backticks from markdown links, replacing them with absolute URLs of the format `file:///Users/sac/process-intelligence/...`.

## 3. Transition Verdict
- **Status**: **TRANSITIONED**. The transition steps have been completed. All downstream prompts are fully expanded and actionable. The repository is ready for evaluation against the final status criteria in [PROCESS_INTELLIGENCE_ALIVE_001.md](file:///Users/sac/process-intelligence/checkpoints/PROCESS_INTELLIGENCE_ALIVE_001.md).

---

## Section 8: The ALIVE Gate (v30.1.1 Spec)

A codebase state $\mathcal{R}$ satisfies the ALIVE certification level if and only if:
$$\text{ALIVE}_{004}(\mathcal{R}) \iff \bigwedge_{i=1}^{10} G_i(\mathcal{R})$$
where:
* $G_1(\mathcal{R})$: Paper Coverage ($\geq 80$ paper families ledgered).
* $G_2(\mathcal{R})$: No Missing Type Law.
* $G_3(\mathcal{R})$: Compile-Pass Fixtures ($\geq 200$).
* $G_4(\mathcal{R})$: Compile-Fail Fixtures ($\geq 160$).
* $G_5(\mathcal{R})$: Stderr Parity ($|\text{compile\_fail}| = |\text{.stderr}|$).
* $G_6(\mathcal{R})$: Audit Script Coverage ($\geq 20$).
* $G_7(\mathcal{R})$: Master Audit Gate (exit 0).
* $G_8(\mathcal{R})$: Cargo Tests (exit 0).
* $G_9(\mathcal{R})$: Clippy Warnings (exit 0).
* $G_{10}(\mathcal{R})$: Format check (exit 0).