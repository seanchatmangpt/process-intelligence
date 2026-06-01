# PROCESS_INTELLIGENCE_SPR_THESIS.md

**Canonical SPR (Sparse Priming Representation) Thesis**
**Research Program: Process Intelligence**
**Authority:** ~/process-intelligence
**Status:** ALIVE — anchored at PROCESS_INTELLIGENCE_ALIVE_001

---

## Prime Thesis

Process intelligence is NOT process mining. NOT observability. NOT dashboard interpretation. NOT AI summarization.

It is the **full lifecycle manufacturing of lawful process reality**:

> design → simulation → construction → activation → operation → monitoring → repair → optimization → board projection → integration → decommissioning → archive

**Core breakthrough:** Knowledge must actuate, constrain, evidence, repair, project, and retire work. Passive description is not intelligence. Lawful manufacture is.

**Universal business:** Manufacturing board-admissible process reality from public standards, type law, receipts, replay, refusal, residuals, and lifecycle authority.

A process is real when its design, execution, evidence, failure boundaries, repairs, projections, and retirement can be lawfully constructed, receipted, replayed, audited, and relied upon.

---

## Formal Objects

### Symbol Definitions

| Symbol | Name | Definition |
|--------|------|------------|
| **O** | Object | A typed process object (case, artifact, entity) with identity and lifecycle |
| **O\*** | Knowledge Corpus | The full set of public standards, type laws, and research receipts |
| **P** | Process | A lifecycle-governed sequence of transitions over typed objects |
| **L** | Lifecycle | The ordered set of lifecycle stages (see Lifecycle Calculus) |
| **E** | Event | A timestamped, attributed, object-linked occurrence in an event log |
| **T** | Transition | A lawful stage-to-stage move: tau: L_i → L_{i+1} |
| **R** | Receipt | A verifiable, typed, non-forgeable evidence record of a lawful transition |
| **F** | Refusal | A typed rejection carrying a named law as reason — not an error string |
| **X** | Partial | An incomplete outcome: some claims admitted, some refused, with residuals |
| **Q** | Quality Metric | A conformance measure in [0,1]: fitness, precision, generalization, simplicity |
| **M** | Model | A discovered or declared process model (WF-net, BPMN, POWL, Declare, DFG, etc.) |
| **C** | Claim | A board-admissible process assertion backed by evidence and receipts |
| **B** | Board Projection | A set of claims admissible for executive or fiduciary decision-making |
| **D** | Dependency | A lawful cross-process or cross-lifecycle dependency that must be closed before decommission |
| **mu** | Manufacture | mu(O*, T, L) → P_i: the act of constructing a process instance from knowledge, transitions, and lifecycle law |
| **alpha** | Actuation | alpha(K, P, L, T) → tau: knowledge actuating a transition — not describing it |
| **rho** | Evidence Emission | rho(tau_i) = R_i: every lawful transition emits a receipt |
| **pi** | Projection | pi(P_i) → B: the transformation of process evidence into board-admissible claims |
| **kappa** | Gate | kappa(tau) ∈ {ADMIT(R), REFUSE(F), PARTIAL(X)}: no silent success |
| **delta** | Decommission | delta(P) → Retired(P) + Archive(A) + Receipt(R_delta) |

### Core Equations

**Process manufacturing:**
```
R proves P_i = mu(O*, T, L)
```
A process instance is real when a receipt proves it was manufactured from knowledge corpus, transitions, and lifecycle law.

**Board projection:**
```
R, Replay, Audit proves B = pi(P_i)
```
A board projection is admissible when backed by receipts, replayed execution, and audit trail.

**M&A-admissible board projection:**
```
R, Replay, Residuals, Refusals proves B_MA = pi(P_i, Risk, Synergy, Integration, Debt)
```
A merger-and-acquisition-ready projection additionally requires residuals (open work), refusals (named failure boundaries), risk quantification, synergy evidence, integration mapping, and technical debt receipts.

---

## Lifecycle Calculus

```
L = {Design, Simulation, Construction, Activation, Operation,
     Monitoring, Repair, Optimization, BoardProjection,
     Integration, Decommission, Archive}
```

**Lawful lifecycle:**
```
P : L_0 → L_1 → ... → L_n
```

Every transition tau_i must be lawful and emit evidence:
```
rho(tau_i) = R_i
```

**Blue River Dam Rule:**
```
kappa(tau) ∈ {ADMIT(R), REFUSE(F), PARTIAL(X)}
```

No silent success. No uncategorized failure. Every gate outcome is typed, receipted, and replayable. A transition that produces no evidence did not happen in the lawful sense.

**Lifecycle stage semantics:**

| Stage | Invariant |
|-------|-----------|
| Design | Declared model exists; no execution has occurred |
| Simulation | Synthetic traces generated; conformance pre-checked |
| Construction | Type law enforced; ALIVE gate must pass |
| Activation | First lawful execution receipt emitted |
| Operation | Continuous lawful execution; conformance monitored |
| Monitoring | Drift detection; anomaly refusals emitted |
| Repair | Residuals identified; targeted fixes applied; re-receipted |
| Optimization | Quality metrics improved; receipts re-confirmed |
| BoardProjection | Executive claims manufactured; audit trail complete |
| Integration | Cross-process dependencies receipted and closed |
| Decommission | All dependencies closed or refused; archive receipt emitted |
| Archive | Immutable; receipts preserved; claims revocable on re-examination |

---

## Autonomic Knowledge Actuation

**Old paradigm:**
```
K describes P
```
Knowledge sits in documents. Humans read it. Humans act. Knowledge is passive.

**New paradigm:**
```
alpha(K, P, L, T) → tau
```
Knowledge directly actuates transitions. No human interpretation step required for the lawful path.

**Validity law:**
```
tau is valid only when:
  tau = alpha(K, P, L, T)
  AND kappa(tau) ∈ {ADMIT(R), REFUSE(F), PARTIAL(X)}
```

A transition that was not actuated by knowledge and did not emit a gate outcome is not a lawful transition. It is noise.

**Consequence:** Every refusal is a knowledge actuation. The system refuses lawfully rather than silently accepting unlawful input. Accumulated refusals are strategic capital — they prove the system has functioning knowledge boundaries.

---

## Public Standards as Feedstock

```
S = {
  OCEL,          -- Object-Centric Event Log 2.0
  XES,           -- eXtensible Event Stream (IEEE 1849)
  BPMN,          -- Business Process Model and Notation
  Petri,         -- Petri net formalism
  WF-net,        -- Workflow net (soundness paper)
  POWL,          -- Partially-Ordered Workflow Language
  Declare,       -- Declarative process constraints
  ProcessTree,   -- Hierarchical process decomposition
  DFG,           -- Directly-Follows Graph
  OCPQ,          -- Object-Centric Process Querying
  OTel/Weaver,   -- OpenTelemetry semantic conventions
  PROV-O,        -- W3C Provenance Ontology
  SHACL,         -- W3C Shapes Constraint Language
  DCTERMS,       -- Dublin Core Metadata Terms
  SKOS,          -- Simple Knowledge Organization System
  ODRL,          -- Open Digital Rights Language
  papers         -- van der Aalst canon, PM4Py research, OCEL spec papers
}
```

**Value gradient:**
```
dValue/dStandards > 0
```

Every new public standard is a new manufacturing surface, not a commoditization threat. Competitors who treat standards as constraints are weakened by new standards. Process intelligence is strengthened by them.

**Manufacturing surface expansion:**
Each standard in S adds:
- A new import/export contract surface
- A new conformance check axis
- A new refusal reason namespace
- A new board projection claim type
- A new ALIVE gate fixture set

---

## Reverse Porter Five Algebra

**Porter's Five Forces (classical):**
```
beta  = buyer power
sigma = supplier power
upsilon = substitution threat
epsilon = new entrant threat
gamma = rivalry intensity
```

**Classical Porter:**
```
Force↑ → Margin↓
```

**Reverse Porter:**
```
Force↑ → Demand for validation authority↑
```

**The inversion mechanism:** When buyers have power, they demand proof. When substitutes exist, differentiation requires receipts. When new entrants arrive, incumbents must demonstrate replay. When rivalry intensifies, the winner is the one whose process claims survive audit.

**The Challenger Question:**

> "If you claim to use public standards, can you manufacture an M&A-ready PowerPoint where every process claim is validated, traceable, replayable, and tied to public standards?"

This question cannot be answered by description. It can only be answered by manufacturing. Every force that increases competitive pressure increases the value of being able to answer YES with receipts.

**Competitive moat from refusal capital:**
```
RefusalCapital = Σ verified_refusals_with_receipts
```

Competitors can copy positive vocabulary. They cannot copy accumulated refusal capital. A system that has refused 10,000 unlawful transitions lawfully has demonstrated 10,000 functioning knowledge boundaries. That is the moat.

---

## Authority Table

| Repository / System | Authority |
|--------------------|-----------|
| `process-intelligence` | Research authority — doctrine, SPR thesis, formal objects, algorithm definitions |
| `wasm4pm-compat` | Process-evidence type foundry — nightly Rust, type law, ALIVE gate, trybuild fixtures |
| `wasm4pm` | Execution authority — runtime manufacture, receipted execution, replay engine |
| `ggen` | Manufacturing and projection machinery — commit manufacturing, lifecycle orchestration |
| `Blue River Dam` | Full-lifecycle authority layer — gate enforcement, kappa operator, decommission authority |
| `PM4Py` | Comparative oracle — discovery, conformance, replay as ground truth for validation |
| `M&A PowerPoint` | Highest-value executive projection surface — the ultimate board-admissible output |

**Authority law:** Each system operates within its authority. No system claims authority beyond its boundary. Cross-system claims require receipted integration evidence.

---

## The Six Algorithms

### Algorithm 1: Paper-to-Law Manufacturing

Given a research paper p, produce a full manufacturing surface:

```
Phi(p) = {
  O_p,            -- formal objects extracted from paper
  A_p,            -- algorithms extracted from paper
  I_p,            -- invariants stated or implied
  Y_p,            -- type law surfaces (Rust types, const bounds)
  H_p,            -- ALIVE gate fixtures (compile-fail + compile-pass)
  Fail_p,         -- failure modes and refusal reasons
  Compat_p,       -- wasm4pm-compat module coverage
  Wasm_p,         -- wasm4pm execution coverage
  PM4Py_p,        -- PM4Py oracle mapping
  Fixtures_p,     -- trybuild fixture set
  Receipts_p,     -- receipt types for this paper's transitions
  Replay_p,       -- replay surfaces
  Board_p,        -- board projection claims enabled by this paper
  Lifecycle_p,    -- lifecycle stages this paper governs
  Decommission_p  -- decommission conditions for this paper's objects
}
```

**Coverage law:**
```
Covered(p) iff Compat_p OR Wasm_p OR ExplicitGraduationBoundary_p
```

A paper that is neither covered by type law, execution authority, nor an explicit graduation boundary is an uncovered surface — a gap in the manufacturing floor.

### Algorithm 2: PM4Py Oracle Mapping

Given a PM4Py capability c, produce a complete mapping:

```
Omega(c) = {
  Input,            -- accepted data structures
  Output,           -- produced artifacts
  Standard,         -- governing public standard
  Algorithm,        -- algorithm family (discovery, conformance, replay, etc.)
  Assumption,       -- implicit assumptions
  CompatType,       -- wasm4pm-compat type that admits this input
  WasmExecution,    -- wasm4pm execution surface
  Refusal,          -- named refusal reasons for unlawful inputs
  Receipt,          -- receipt types emitted on lawful execution
  Replay,           -- replay mechanism
  Lifecycle,        -- lifecycle stages involved
  BoardClaim        -- board-admissible claims this capability supports
}
```

**Blue River readiness:**
```
BlueRiverReady(c) iff
  PM4Py(c)          -- PM4Py can execute it
  AND TypeLaw(c)    -- wasm4pm-compat enforces type law
  AND ExecAuthority(c) -- wasm4pm has execution authority
  AND Receipt(c)    -- receipts are emitted
  AND Replay(c)     -- replay is possible
  AND Lifecycle(c)  -- lifecycle stage is governed
```

### Algorithm 3: Admissible Process Evidence

The admission gate is the only lawful path from raw input to process execution:

```
Admit(x, W, T) → Evidence<T, Admitted, W>
              | Refusal(F)
              | Partial(X)
```

**Laws:**
- No silent coercion. Raw input that fails admission must produce Refusal(F), not a degraded Admitted.
- Refusal reason F must name a specific structural law, not a string or generic error.
- Partial(X) carries residuals — the specific claims that could not be admitted and why.
- The Witness W is non-forgeable. Evidence<T, Admitted, Ocel20> cannot be confused with Evidence<T, Admitted, Xes1849> at the type level.

### Algorithm 4: Receipt-Bearing Execution

Given admitted evidence, execute and produce receipts:

```
Execute(Evidence<T, Admitted, W>, a, theta) → V + R + Gamma + X
```

Where:
- V = verified result
- R = receipt (non-forgeable, typed, replayable)
- Gamma = graduated evidence (ready for next lifecycle stage)
- X = residuals (open claims, partial completions)

**Receipt laws:**
```
V => R          -- every verified result produces a receipt
R => Gamma      -- every receipt produces graduated evidence
Gamma => Evidence -- graduated evidence is typed Evidence
```

No execution without receipts. No receipt without verification. No graduation without receipts.

### Algorithm 5: M&A Deck Manufacturing

The highest-value executive projection surface:

```
Deck = pi({B_i})
```

Validity law for each board claim B_i:
```
B_i valid iff
  Evidence_i        -- backed by admitted evidence
  AND TypeLaw_i     -- type law enforced at manufacture time
  AND Receipt_i     -- receipt exists
  AND Replay_i      -- replay is possible
  AND Standard_i    -- tied to a public standard
  AND Lifecycle_i   -- lifecycle stage is receipted
```

**Invariant:** No unsupported slide claims. Every slide claim in the M&A deck must be traceable to a B_i that satisfies all six conditions. A claim without evidence is not a process intelligence claim — it is a consulting opinion.

### Algorithm 6: Decommissioning

The terminal lifecycle transition:

```
delta(P) → Retired(P) + Archive(A) + Receipt(R_delta)
```

**Preconditions:**
```
delta(P) is lawful only when:
  forall D_i in Dependencies(P):
    kappa(D_i) ∈ {CLOSED, REFUSED}
  AND forall C_i in Claims(P):
    kappa(C_i) ∈ {ARCHIVED, REVOKED}
```

No decommission without closing all dependencies. No decommission without archiving or revoking all claims. The decommission receipt R_delta is the proof that the process was lawfully retired, not abandoned.

---

## Negative Evidence and Refusal Capital

**Refusal capital definition:**
```
RefusalCapital = Σ_i verified_refusal_i where kappa(verified_refusal_i) = REFUSE(F_i)
```

**Strategic properties of refusal capital:**

1. **Non-copyable.** Competitors can clone positive vocabulary, terminology, and interface surface. They cannot clone verified refusals — each refusal requires the system to have actually encountered, processed, and rejected a specific unlawful input.

2. **Cumulative.** Refusal capital compounds. A system that has refused 100 times has 100 functioning knowledge boundaries. A system that has refused 10,000 times has demonstrated boundary enforcement at scale.

3. **Auditable.** Every refusal carries a named law as reason. Auditors can inspect the full refusal history and verify that each refusal was lawful — not a false rejection of valid input.

4. **Differentiating.** In due diligence, the question is not "what can your system do?" but "what does your system refuse to do, and why?" A system that refuses unlawfully structured input with named laws is more trustworthy than one that silently degrades.

**Refusal as strategic asset:** The accumulation of verified, receipt-bearing refusals is the deepest competitive moat in process intelligence. It cannot be manufactured in retrospect. It must be earned through lawful execution at scale.

---

## Combinatorial Maximalism

**Manufacturing surface:**
```
Surface = |S| x |L| x |A| x |F| x |R| x |Q| x |B| x |D|
```

Where:
- |S| = number of public standards covered
- |L| = number of lifecycle stages
- |A| = number of algorithms
- |F| = number of named refusal reasons
- |R| = number of receipt types
- |Q| = number of quality metrics
- |B| = number of board claim types
- |D| = number of dependency types

Each dimension multiplies the others. Adding one standard does not add one capability — it adds |L| x |A| x |F| x |R| x |Q| x |B| x |D| new manufacturing cells.

**Takt calculation:**
```
C = n x d x t
```

Where:
- C = total commits (manufacturing receipts)
- n = number of parallel manufacturing lines
- d = days in sprint
- t = commits per line per day

For C = 5000, d = 30, n = 10:
```
t = 5000 / (30 x 10) = 16.7 commits/cell/day
```

This is a factory design problem, not a motivation problem. The question is not "can we manufacture 5000 commits?" but "how many manufacturing lines do we need to hit takt?"

---

## Blue River Dam Operating Equation

The Blue River Dam is the full-lifecycle authority layer — the composition of all four operators:

```
BlueRiverDam = kappa ∘ rho ∘ alpha ∘ mu
```

Operating equation:
```
kappa(rho(alpha(mu(O*)))) → ALIVE | PARTIAL | REFUSED
```

**Operator chain semantics:**

1. `mu(O*)` — manufacture a process instance from the knowledge corpus
2. `alpha(mu(O*))` — actuate knowledge against the manufactured instance
3. `rho(alpha(mu(O*)))` — emit evidence from the actuation
4. `kappa(rho(alpha(mu(O*))))` — gate the evidence: ALIVE, PARTIAL, or REFUSED

**ALIVE:** All transitions lawful, all receipts valid, all claims supported.
**PARTIAL:** Some transitions lawful, residuals identified, open claims receipted.
**REFUSED:** Transition is unlawful, named law provided, no silent degradation.

The Blue River Dam never produces an ungated outcome. There is no fourth option.

---

## Final Kernel

Process intelligence is full-lifecycle process authority. Process mining is one subset. Observability is one projection. The real unit is the lifecycle-governed process claim.

> "A process is real when its design, execution, evidence, failure boundaries, repairs, projections, and retirement can be lawfully constructed, receipted, replayed, audited, and relied upon."

This is not a definition of software. It is not a definition of consulting. It is the definition of a manufactured fact about a business process — a fact that can survive due diligence, board scrutiny, regulatory audit, and M&A examination because every claim in it is backed by a receipt.

The product is process intelligence. All implementations — wasm4pm-compat, wasm4pm, ggen, Blue River Dam — are manufacturing surfaces for this one product.

---

*Generated: PROCESS_INTELLIGENCE_ALIVE_001 sealed (588 commits, 12/12 criteria). See checkpoints/PROCESS_INTELLIGENCE_ALIVE_001.md.*
