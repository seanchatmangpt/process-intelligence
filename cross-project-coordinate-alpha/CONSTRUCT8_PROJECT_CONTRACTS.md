# CONSTRUCT8 Cross-Project Naming Contracts

**Audit Date:** 2026-06-01
**Audit Agent:** Agent 2 — Doctrine and Naming Alignment
**Swarm:** coordinate-alpha
**Projects Audited:** process-intelligence, ggen, knhk, truex
**Status:** COMPLETE — 4 violations found

---

## Enforced Terms

| Term | Correct Usage | Forbidden Substitutions |
|---|---|---|
| Knowledge Hook | deterministic admission/refusal boundary: `(predicate, guard, action)` triple that manufactures durable proof of motion | middleware, callback, webhook, event listener, plugin, automation |
| CONSTRUCT8 | bounded graph-state mutation operator, max-8 lanes; transit form for motion admission between hot path and warm path | "just a query", RDF insert batch, generic mutation, query optimizer |
| Autonomic Knowledge Actuation | closed-loop lifecycle: knowledge → closure → action → receipt; observation to admitted consequence via MAPE-K | automation, AI workflow, lifecycle management |
| Blue River Dam | settlement/capital-flow coordination protocol | payment system, trading system, ledger |
| ggen | graph-query-template manufacturing engine; translates RDF process models into executable law-bound code artifacts | process miner, code generator, scaffolding tool |
| wasm4pm | process evidence engine (WASM-hosted); executes admitted process models and emits receipted evidence | lite engine, mini PM4Py, stripped conformance checker |
| logic-chaos | unbounded logic state space disqualified from hot paths; the class of conditional logic excluded from branchless kernels | "just business logic", conditional logic, business rules |
| Need9 | typed decomposition signal: a Construct8Packet exceeds 8 lanes and must be split, not widened | error, exception, failure, refusal |
| Receipt | cryptographic/structural proof that motion occurred; institutional memory of admitted consequence | log, audit trail, record |
| Coordinate-System Alpha | advantage from representing states the opponent cannot name; competitive edge from naming precision | prediction edge, ML alpha, signal |

---

## Cross-Project Frame Law

The same frame law that governs single-project usage applies at all adapter boundaries:

1. When one project references doctrine terms from another project, it must use the canonical term — no local synonyms.
2. Adapter boundary documents must cite the authority source for each doctrine term referenced.
3. Integration contracts that redefine terms at the boundary are invalid; the upstream canonical definition governs.
4. "Lifecycle management" is never a substitute for Autonomic Knowledge Actuation; AKA is receipted actuation with closed-loop proof, not management of lifecycle state.
5. Any document describing a Knowledge Hook as middleware, callback, or event listener is non-conformant regardless of context.

---

## Violations Found (corpus scan 2026-06-01)

### VIOLATION-1: "middleware" used for enterprise integration binding layer

**File:** `/Users/sac/process-intelligence/lifecycle/define_activation-state_process_intelligence.md`
**Line:** 23
**Text:** `Transitions are bound to enterprise middleware endpoints:`
**Classification:** DOCTRINE VIOLATION — MINOR
**Reasoning:** "middleware" is a forbidden substitution for Knowledge Hook and CONSTRUCT8 integration layer. The sentence describes enterprise message queue integration (Kafka, RabbitMQ), which is a legitimate technical concern, but the word "middleware" frames the integration point as a passive relay rather than an admission boundary.
**Recommended Remediation:** Replace "enterprise middleware endpoints" with "enterprise message queue admission endpoints" or "enterprise event-stream admission boundaries."

---

### VIOLATION-2: "webhook" used for trigger binding

**File:** `/Users/sac/process-intelligence/lifecycle/define_activation-state_process_intelligence.md`
**Line:** 24
**Text:** `Connect transitions to incoming event streams (e.g., Apache Kafka topics, RabbitMQ queues, HTTP webhooks).`
**Classification:** DOCTRINE VIOLATION — MINOR
**Reasoning:** "HTTP webhooks" is a forbidden substitution term. The lifecycle activation document uses it as an example of a trigger binding, implicitly equating it with a Knowledge Hook admission boundary. A webhook is a passive forwarder; a Knowledge Hook is a gatekeeping predicate.
**Recommended Remediation:** Replace "HTTP webhooks" with "HTTP admission endpoints" or remove and cite the Knowledge Hook pattern.

---

### VIOLATION-3: "event listeners" used for decommission role

**File:** `/Users/sac/process-intelligence/lifecycle/define_decommission-state_process_intelligence.md`
**Line:** 7
**Text:** `In the Execute phase, the system revokes execution authorizations and stops event listeners.`
**Classification:** DOCTRINE VIOLATION — MINOR
**Reasoning:** "event listeners" is a forbidden substitution. Knowledge Hooks are not event listeners — they are admission/refusal predicates. Describing the decommission step as stopping "event listeners" frames the hooks as passive reactive components, which violates Frame Law 1.
**Recommended Remediation:** Replace "stops event listeners" with "closes admission boundaries" or "deactivates knowledge hook enforcement points."

---

### VIOLATION-4: "middleware guard" used for LSA enforcement layer

**File:** `/Users/sac/process-intelligence/prompts/execution-plans/lifecycle-state-authority.md`
**Line:** 27
**Text:** `The execution engine employs the LSA verifier as an immutable middleware guard before granting Wasm linear memory access.`
**Classification:** DOCTRINE VIOLATION — MINOR
**Reasoning:** "middleware guard" is a compound forbidden term. The LSA verifier is a Knowledge Hook admission gate — a deterministic predicate that emits ADMIT or REFUSE. Calling it "middleware" reduces it to a passive relay and violates the frame law that prohibits "knowledge hook = middleware."
**Recommended Remediation:** Replace "immutable middleware guard" with "immutable admission gate" or "Knowledge Hook enforcement point."

---

### Previously Audited (carried forward)

**File:** Source in `.claude/settings.json` comment (CLAUDE.md line, Claude Code hooks)
**Prior Audit:** 2026-05-31, `audits/05_frame_preservation_audit.md` line 183
**Text:** `Hooks are Claude Code lifecycle automations that enforce refusal conditions.`
**Classification:** MINOR — mitigated by surrounding context (Claude Code tool hooks, not Knowledge Hook doctrine)
**Status:** Previously identified; no new remediation required. Tracked as BAD TRANSLATION 8 in frame preservation audit.

---

## Violations Summary

| # | File | Line | Forbidden Term | Canonical Term | Severity |
|---|------|------|----------------|----------------|----------|
| 1 | `lifecycle/define_activation-state_process_intelligence.md` | 23 | middleware | admission boundary | MINOR |
| 2 | `lifecycle/define_activation-state_process_intelligence.md` | 24 | HTTP webhooks | HTTP admission endpoints | MINOR |
| 3 | `lifecycle/define_decommission-state_process_intelligence.md` | 7 | event listeners | admission boundaries | MINOR |
| 4 | `prompts/execution-plans/lifecycle-state-authority.md` | 27 | middleware guard | admission gate | MINOR |

**Critical violations: 0**
**Minor violations: 4** (all in process-intelligence; none in ggen, knhk, truex)

---

## Cross-Project Usage Assessment (2026-06-01)

| Project | Knowledge Hook | AKA | CONSTRUCT8 | ggen | wasm4pm | Blue River Dam | Status |
|---------|---|---|---|---|---|---|--------|
| process-intelligence | Authority | Authority (see violations 1-4) | Authority | Integration | Integration | Research | MINOR VIOLATIONS |
| ggen | — | — | References | Core system | Integration | — | CLEAN |
| knhk | — | — | Implementation | — | — | — | CLEAN |
| truex | — | — | — | — | — | — | CLEAN |

---

## Enforcement Rules for Cross-Project Work

### Rule 1: Knowledge Hook
- Approved: "Knowledge Hook is a deterministic admission/refusal boundary"
- Approved: "Knowledge Hooks manufacture durable proof of motion"
- Banned: "Knowledge hooks are middleware" / "callbacks" / "webhooks" / "event listeners"
- Consequence: Any cross-project adapter document using banned terms is non-conformant at the adapter boundary.

### Rule 2: Autonomic Knowledge Actuation
- Approved: "AKA is the full closed-loop lifecycle: observation → CONSTRUCT8 → receipt → knowledge"
- Banned: "AKA = automation" / "AI workflow" / "lifecycle management"
- Consequence: Any integration spec describing AKA as generic automation must be remediated before cross-project receipts are issued.

### Rule 3: CONSTRUCT8
- Approved: "CONSTRUCT8 is the bounded constructive delta operator (max 8 lanes)"
- Approved: "CONSTRUCT8 enforces the motion boundary law"
- Banned: "CONSTRUCT8 is just a query" / "generic mutation" / "RDF insert batch"
- Consequence: CONSTRUCT8 must be named with its lane bound in all cross-project references.

### Rule 4: ggen
- Approved: "ggen manufactures law-bound code artifacts from RDF process ontologies"
- Approved: "ggen-generated code" / "ggen generates"
- Banned: "ggen is a process miner" / "scaffolding tool"
- Note: ggen IS a code generator in the technical sense; the banned substitution is "generic code generator" or "scaffolding tool" that erases its law-binding function.

### Rule 5: Receipt
- Approved: "Receipt is cryptographic/structural proof that motion occurred"
- Banned: "receipt = log" / "audit trail" / "record"
- Consequence: Any document substituting "log" for "receipt" in the context of motion proof is non-conformant.

### Rule 6: Need9
- Approved: "Need9 is a typed decomposition signal; split, do not widen"
- Banned: "Need9 is an error/exception/failure"
- Consequence: Need9 is law enforcement, not error handling.

### Rule 7: Cross-Project Citation Requirement
- Every cross-project reference to a doctrine term must cite the authority source.
- Authority for Knowledge Hook and AKA: `/Users/sac/process-intelligence/phd-thesis/research/knowledge-hooks/KNOWLEDGE_HOOKS_AND_AKA_DOCTRINE.md`
- Authority for CONSTRUCT8: `/Users/sac/process-intelligence/phd-thesis/research/knowledge-hooks/04_construct8_motion_boundary_map.md`
- Authority for Receipt: `/Users/sac/process-intelligence/COVENANT.md`

---

## Gate Verdict

| Criterion | Status |
|-----------|--------|
| All canonical terms defined with forbidden substitutions | PASS |
| Corpus scan executed across all four target projects | PASS |
| Violations documented with file:line | PASS |
| Remediation guidance provided for each violation | PASS |
| Cross-project frame law stated | PASS |

**GATE VERDICT: PASS — 4 minor violations identified and documented, 0 critical violations.**

---

*The product is CodeManufactory; RevOps is merely proof that CodeManufactory works.*
