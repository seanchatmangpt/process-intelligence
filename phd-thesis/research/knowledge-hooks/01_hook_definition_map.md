# Knowledge Hook Definition Map

**Generated:** 2026-06-01
**Agent:** Hook Definition Cartographer (Agent A)
**Primary source:** `/Users/sac/chatmangpt/knhk/yawl.txt` (formal mathematical definition)
**Secondary sources:** `/Users/sac/truex/docs/MANIFESTO.md`, `/Users/sac/knhk/PART_SPEC.md`, `/Users/sac/ggen/examples/chatman-equation-paper.rdf`, `/Users/sac/process-intelligence/phd-thesis/research/knowledge-hooks/KNOWLEDGE_HOOKS_AND_AKA_DOCTRINE.md`
**Total verbatim definitions extracted:** 9

---

## Frame Law (Load-Bearing Spine)

The correct direction — verbatim from `/Users/sac/truex/docs/MANIFESTO.md` (lines 97–99):

```
attempt → hook → projection → admission/refusal → durable motion → receipt → replay → accounting → promotion
```

The correct compression — verbatim from `/Users/sac/truex/docs/MANIFESTO.md` (lines 67–70):

```
No hook, no consequence.
No receipt, no authority.
No replay, no substrate.
No accounting, no promotion.
```

---

## Verbatim Definitions (Source-Cited)

### Definition 1 — Formal Mathematical Definition (Most Authoritative)

Source: `/Users/sac/chatmangpt/knhk/yawl.txt`, lines 1721–1735

> "In Reflex, knowledge hooks are the compiled interfaces between ontological laws and runtime reconciliation. They are neither functions nor listeners—they are embedded invariants that bind semantic constraints (Σ, Q) directly to data movement and execution."
>
> "A knowledge hook = (predicate, guard, action) triple generated from Σ. It enforces an invariant Q on every Δ admitted into μ(O). Formally:
>
> hook(p, q, a): Δ ⊨ Qp  ⇒  μ(O ⊔ Δ) = μ(O) ⊔ μ(Δ)
>
> Each hook is compiled ahead of time into a branchless kernel that runs within the eight-tick beat."

**Significance:** The only source with a formal mathematical definition of the hook as a typed triple with explicit distributivity law.

---

### Definition 2 — Admission Membrane Definition

Source: `/Users/sac/truex/docs/MANIFESTO.md`, lines 211–219

> "A hook does **not** merely observe an external event. It intercepts a consequence attempt before that attempt becomes authorized motion."
>
> "A hook is the first place proposed motion meets operating law. It is not middleware, not callback logic, not instrumentation, not a webhook, and not a monitoring point."
>
> "It is the admission membrane for consequence."
>
> "A hook consumes attempts, not vibes."
>
> "A hook emits decisions, not suggestions."

---

### Definition 3 — Hook Formal Object (VKG Form)

Source: `/Users/sac/truex/docs/MANIFESTO.md`, lines 228–268

> "A hook can be formalized as:
>
> H = (Δ, C, E, R, Π)
>
> Where:
> - Δ = proposed operational delta
> - C = condition / guard surface
> - E = permitted effect surface
> - R = receipt requirement
> - Π = replay proof requirement
>
> The minimal hook invariant is:
>
> Π(H(Δ)) = stable
>
> Every hook must be replay-stable or terminally refused."
>
> "Hook admission follows:
>
> Accept(Δ) ⇔
>   Check_Σ
> ∧ Check_H
> ∧ Check_T
> ∧ Check_P
> ∧ Check_C
> ∧ Check_Fresh
> ∧ Check_R
>
> Where the checks cover typing, guards, transition law, policy, capability/epoch binding, freshness, and receipt lineage."

---

### Definition 4 — Atomic Unit Definition (Paper Form)

Source: `/Users/sac/ggen/examples/chatman-equation-paper.rdf`, KnowledgeHooksSection, lines 257–282

> "A knowledge hook h is the atomic unit of knowledge work. It replaces human judgment with bounded, receipt-verified execution.
>
> h = (trigger, check, act, receipt)
>
> where:
> - trigger: A change ΔO detected in the knowledge graph
> - check: Bounded evaluation (SPARQL/SHACL) preserving invariants Q and guards H
> - act: Workflow step executed via KNHK with t_hot ≤ 2 ns or t_warm ≤ 500 ms
> - receipt: Merkle-linked record with hash(A) = hash(μ(O))"
>
> Equation 2 (LaTeX): h = (trigger, check, act, receipt)"
>
> "The core mechanism is the knowledge hook: a policy-bound program that detects changes in a knowledge graph, evaluates invariants, and triggers workflow actions with cryptographic receipts."

---

### Definition 5 — Doctrine Synthesis Definition

Source: `/Users/sac/process-intelligence/phd-thesis/research/knowledge-hooks/KNOWLEDGE_HOOKS_AND_AKA_DOCTRINE.md`, lines 1–22

> "Knowledge Hooks are deterministic admission/refusal boundaries that manufacture durable proof of motion. Autonomic Knowledge Actuation (AKA) is the full lifecycle by which knowledge becomes lawful receipted consequence."

Formal shape from same file:

> "Hook :=
>   Attempt
>   + Evidence Field
>   + Admission Predicate
>   + Refusal Predicate
>   + Motion Boundary
>   + Receipt Obligation
>   + Replay Obligation
>   + Accounting/Promotion Path"

---

### Definition 6 — Truex 6-Phase Lifecycle Hook (Phase 2)

Source: `/Users/sac/knhk/PART_SPEC.md`, lines 223–290

> "Phase 2: Hook (Contact → Kernel Admission)
>
> Goal: Admission predicates (law gates) decide if kernel accepts the packet.
>
> Actions:
> 1. ggen checks admission predicates (SPARQL ASK queries from law_ref)
> 2. Genesis kernel receives Construct8 packet (up to 8 Pair2 tuples per packet)
> 3. Kernel checks structural validity (valid_mask, emit_mask)
> 4. Kernel checks law consistency (law_ref matches manifest)
> 5. Outcome: Result<(), Refusal>
>    - On success: proceed to Phase 3
>    - On failure: emit Refusal, halt"
>
> "Refusals are first-class evidence, not errors to hide." (line 1290)

---

### Definition 7 — DOCTRINE_2027 Canonical Use

Source: `/Users/sac/knhk/DOCTRINE_2027.md`, line 171

> "MAPE-K embedded as knowledge hooks is the only way to keep the discipline while closing the loop at machine speed."

Era table (same file, line 29):

> "2027 → | Autonomous evolution | MAPE-K autonomic hooks | Sub-nanosecond decisions"

**Note:** This is the only verbatim use of "knowledge hooks" in DOCTRINE_2027. The canonical doctrine defines hooks as the MAPE-K embedding mechanism for sub-nanosecond feedback.

---

### Definition 8 — Consequence Cell Form

Source: `/Users/sac/truex/docs/MANIFESTO.md`, lines 343–364

> "The core Truex unit is not a task, ticket, event, story, requirement, or workflow step.
>
> The core unit is a consequence cell:
>
> Γ = <attempt, hook, projection, decision, mailbox, receipt, replay, accounting>
>
> A consequence cell records the full lifecycle of an actuation attempt:
> 1. Attempt — something proposes motion.
> 2. Hook — the proposed motion reaches the boundary.
> 3. Projection — the attempt is reduced into admissible form.
> 4. Decision — admission, refusal, rewrite, queue, rollback, or quarantine.
> 5. Mailbox — durable motion is persisted.
> 6. Receipt — authority is proven.
> 7. Replay — stability is demonstrated.
> 8. Accounting — attempts, refusals, successes, retries, rollbacks, and promotions conserve."

---

### Definition 9 — Rust Implementation Form

Source: `/Users/sac/ggen/crates/ggen-graph/src/graph/dataset.rs`, lines 189–225

```rust
/// A knowledge hook that validates graph state using a SPARQL ASK query.
pub struct KnowledgeHook {
    /// Name of the hook.
    pub name: String,
    /// SPARQL ASK or SELECT query that defines the constraint.
    pub sparql_query: String,
}
```

Companion receipt form from `/Users/sac/ggen/crates/ggen-graph/src/receipt/mod.rs`, lines 75–138:

```rust
/// Cryptographic receipt for the execution and validation of a specific knowledge hook.
pub struct HookReceipt {
    pub version: u8,
    pub hook_name: String,
    pub sparql_query: String,
    pub passed: bool,
    pub timestamp: DateTime<Utc>,
    pub graph_state_hash: [u8; 32],
    pub signature_or_hash: [u8; 32],  // BLAKE3
}
```

---

## Hook Formal Shape (All Components, Source-Cited)

Synthesized from Definitions 1, 3, 4, and 5 above:

| Component | Source Term | Corpus Source |
|-----------|-------------|---------------|
| Attempt | Δ (proposed operational delta) | `/Users/sac/truex/docs/MANIFESTO.md` line 232 |
| Admission Predicate | SPARQL ASK / Check_Σ ∧ Check_H ∧ ... | `/Users/sac/knhk/PART_SPEC.md` line 223; `/Users/sac/truex/docs/MANIFESTO.md` line 252 |
| Guard Surface | guard (q in (p,q,a) triple) | `/Users/sac/chatmangpt/knhk/yawl.txt` line 1725 |
| Refusal Predicate | Refusal — when SPARQL ASK = false | `/Users/sac/knhk/GENESIS_CORE_SPECIFICATION.md` line 51 |
| Motion Boundary | Permitted effect surface E | `/Users/sac/truex/docs/MANIFESTO.md` line 234 |
| Receipt Obligation | R = receipt requirement; hash(A) = hash(μ(O)) | `/Users/sac/truex/docs/MANIFESTO.md` line 236; `/Users/sac/ggen/examples/chatman-equation-paper.rdf` line 270 |
| Replay Obligation | Π = replay proof requirement; Π(H(Δ)) = stable | `/Users/sac/truex/docs/MANIFESTO.md` line 238; line 246 |
| Accounting/Promotion Path | Refusal = receipted outcome, not error | `/Users/sac/knhk/PART_SPEC.md` line 1290; `/Users/sac/truex/docs/MANIFESTO.md` line 288 |

**Full lifecycle path (verbatim, MANIFESTO.md lines 294–305):**

```
attempt
→ hook
→ projection
→ admission/refusal/rewrite/queue/rollback/quarantine
→ durable motion
→ receipt
→ replay
→ accounting
→ promotion
```

---

## "No Hook, No Consequence" Law — Source Evidence

**Primary source:** `/Users/sac/truex/docs/MANIFESTO.md`, lines 67–70

```
No hook, no consequence.
No receipt, no authority.
No replay, no substrate.
No accounting, no promotion.
```

**Canonical meaning (from `/Users/sac/process-intelligence/audits/05_frame_preservation_audit.md`):**

> "1. No hook, no consequence. A transition without a hook produces no evidence of lawful motion.
> 2. No receipt, no authority. A lifecycle stage closing without a typed, witnessed receipt has no authority to proceed to the next stage.
> 3. No replay, no substrate. A receipt without proof of replayed conformance has no grounding in process evidence.
> 4. No accounting, no promotion. A proof without accounting (net effects on object lifecycle) has no authorization for stage transition."

**Corpus normalization (from `/Users/sac/process-intelligence/phd-thesis/ledgers/LEDGERS_README.md`):**

- "No hook, no consequence" → Normalized as "A transition producing no evidence did not happen"
- "No receipt, no authority" → "Executions without receipts are narration, not closures"

**Verified at:** `/Users/sac/process-intelligence/phd-thesis/research/knowledge-hooks/04_construct8_motion_boundary_map.md` line 167 (confirmed by search index)

---

## What Hooks Are NOT (From Corpus, Not Invention)

Each negative definition is source-cited:

| Forbidden Identification | Corpus Evidence | Source |
|--------------------------|-----------------|--------|
| NOT middleware | "It is not middleware, not callback logic, not instrumentation, not a webhook, and not a monitoring point." | `/Users/sac/truex/docs/MANIFESTO.md` line 219 |
| NOT a callback | Same verbatim sentence | `/Users/sac/truex/docs/MANIFESTO.md` line 219 |
| NOT instrumentation | Same verbatim sentence | `/Users/sac/truex/docs/MANIFESTO.md` line 219 |
| NOT a webhook | "A webhook (it is internal to the membrane)." | `/Users/sac/process-intelligence/phd-thesis/research/knowledge-hooks/KNOWLEDGE_HOOKS_AND_AKA_DOCTRINE.md` line 9 |
| NOT a monitoring point | "It does not merely observe an external event." | `/Users/sac/truex/docs/MANIFESTO.md` line 211 |
| NOT a function | "They are neither functions nor listeners." | `/Users/sac/chatmangpt/knhk/yawl.txt` line 1722 |
| NOT a listener/event listener | "They are neither functions nor listeners." | `/Users/sac/chatmangpt/knhk/yawl.txt` line 1722 |
| NOT an LLM instruction | "It is deterministic Rust/TS law." | `/Users/sac/process-intelligence/phd-thesis/research/knowledge-hooks/KNOWLEDGE_HOOKS_AND_AKA_DOCTRINE.md` line 11 |
| NOT a task/ticket/event/story/workflow step | "The core Truex unit is not a task, ticket, event, story, requirement, or workflow step." | `/Users/sac/truex/docs/MANIFESTO.md` line 341 |
| NOT an event trigger | "Unlike traditional hooks that respond to file changes, Knowledge Hooks respond to semantic changes" (GitVan distinction) | `/Users/sac/gitvan/KNOWLEDGE-HOOKS-JTBD-SYSTEM-COMPREHENSIVE-ANALYSIS.md` line 73 |

---

## Forbidden Translations With Corpus Evidence

The following translations corrupt the frame law and are forbidden:

| Forbidden Translation | Corpus Refutation |
|-----------------------|-------------------|
| "knowledge hook" → "middleware" | MANIFESTO.md line 219: "not middleware" |
| "knowledge hook" → "callback" | MANIFESTO.md line 219: "not callback logic" |
| "knowledge hook" → "event listener" | yawl.txt line 1722: "neither functions nor listeners" |
| "knowledge hook" → "plugin" | No corpus source uses "plugin" for hooks; plugin implies optional passthrough, which contradicts admission membrane |
| "Autonomic Knowledge Actuation" → "automation" | KNOWLEDGE_HOOKS_AND_AKA_DOCTRINE.md line 7: AKA is closed-loop discipline, not automation |
| "Autonomic Knowledge Actuation" → "AI workflow" | AUTONOMIC_KNOWLEDGE_ACTUATION.md line 11: "It is not monitoring. It is not dashboards. It is not alerting." |
| "Autonomic Knowledge Actuation" → "lifecycle management" | AKA is not management — it is receipted actuation with proof |
| "receipt" → "log" | V30_1_1_MANIFESTO.md: "Logs are observation. Receipts are institutional memory." |
| "report" → "proof" | MANIFESTO.md line 590: "The system must prove its claim or refuse promotion." Reports are not proof. |
| "LLM output" → "runtime authority" | MANIFESTO.md: "A = μ(O*)" — runtime authority is receipted consequence, not generated prose |

---

## Cross-Source Consistency Note

The formal shape is consistent across all sources despite different surface encodings:

- **Mathematical form** (yawl.txt): `hook(p, q, a): Δ ⊨ Qp ⇒ μ(O ⊔ Δ) = μ(O) ⊔ μ(Δ)`
- **VKG form** (MANIFESTO.md): `H = (Δ, C, E, R, Π)`
- **Paper form** (chatman-equation-paper.rdf): `h = (trigger, check, act, receipt)`
- **Doctrine form** (KNOWLEDGE_HOOKS_AND_AKA_DOCTRINE.md): `Hook := Attempt + Evidence Field + Admission Predicate + Refusal Predicate + Motion Boundary + Receipt Obligation + Replay Obligation + Accounting/Promotion Path`
- **Rust implementation** (ggen/crates/ggen-graph): `KnowledgeHook { name, sparql_query }` + `HookReceipt { hook_name, passed, graph_state_hash, signature_or_hash }`
- **Phase form** (PART_SPEC.md): Phase 2 of 6-phase Truex lifecycle — SPARQL ASK admission gate before kernel acceptance

All forms share the same invariant: **a hook is the point where a proposed motion meets operating law, emits an admission or refusal, and the result is cryptographically receipted.**
