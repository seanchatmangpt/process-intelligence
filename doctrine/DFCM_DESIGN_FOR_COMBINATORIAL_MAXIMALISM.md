# Design for Combinatorial Maximalism (DFCM)

**Status:** ACTIVE — append-only doctrine
**Authority:** process-intelligence research foundry
**Initial implementation:** 2026-08-13
**Execution boundary:** CONSTRUCT_ONLY

## Source Grounding

DFCM operationalizes already-sealed repository laws rather than replacing them:

1. `doctrine/ENUMERATION_COVENANT.md` — name and enumerate before construction.
2. `doctrine/DOWNSTREAM_AUTHORIZATION_LAW.md` — research authority must speak before downstream external-state change.
3. `doctrine/EVIDENCE_CHAIN.md` — lifecycle transitions are one-way and admission cannot be skipped.
4. `doctrine/RECEIPT_DOCTRINE.md` — hashes prove byte integrity; receipts bind law, witness, result, and lifecycle.
5. `checkpoints/PROCESS_INTELLIGENCE_ALIVE_002.md` — executable content-quality authority for this repository.

DFCM is therefore a construction calculus over admitted design space. It is not an external-state execution engine.

## Definition

Let a design problem expose finite axes

\[
A = \{A_1, A_2, \dots, A_n\}
\]

with a raw Cartesian construction space

\[
C = A_1 \times A_2 \times \cdots \times A_n.
\]

Let `K` be the set of named constraints and exclusions, and let `D(c)` identify a candidate outside the construct-only boundary. DFCM admits the reversible construction space

\[
C^* = \{c \in C \mid K(c) \land \neg D(c)\}.
\]

A deterministic score orders admitted candidates using declared ordinal metadata for reversibility, evidence strength, coverage, and construction cost:

\[
S(c) = w_r R(c) + w_e E(c) + w_g G(c) - w_c Cost(c).
\]

The score ranks constructions. It does not authorize external-state changes.

## Preserve

Sealed checkpoints, doctrine, receipts, and prior falsifiers retain standing. DFCM extends them by new artifacts and addenda. It must not rewrite prior evidence merely to make a new gate pass.

## Fence

DFCM separates three operations:

- **SELECT** — choose a bounded portfolio from already admitted candidates.
- **CONSTRUCT** — enumerate, constrain, score, project, and receipt reversible candidate artifacts.
- **DO** — change external or machine state.

`tools/dfcm.py` implements SELECT and CONSTRUCT only. A candidate outside the configured construct-only effect boundary is emitted as `REFUSED_DO_PATH`.

No code path in the DFCM compiler performs downstream repository mutation, deployment, publication, API-side effects, or other external-state change.

## Calculus

The canonical process-intelligence manifest is `dfcm/process-intelligence.toml`.

It currently explores six axes:

1. phase — preserve, fence, calculus, exclusions, falsifier, extension, operationalization
2. evidence — doctrine, standards, papers, experiments, gaps, receipts
3. representation — markdown, JSON, TOML, RDF, OCEL
4. verification — static, semantic, executable, replay
5. projection — none, checkpoint, downstream prompt, board claim
6. effect boundary — construct only, external state

The manifest is data. New axes and constraints extend the search space without changing the compiler.

## Exclusions

The canonical manifest enforces these hard boundaries:

- operationalization requires executable or replay verification;
- board claims require replay verification and receipt evidence;
- RDF and OCEL constructions require semantic-or-stronger verification;
- preservation and exclusion analysis cannot directly manufacture a board claim;
- every `external_state` candidate is refused by the compiler regardless of score.

Constraint refusal is evidence. It is not silently removed from the denominator.

## Falsifier

DFCM standing is falsified if any of the following occurs:

1. an external-state candidate appears in the admitted construction set;
2. identical manifest bytes produce a different selected ordering;
3. the compiler accepts a manifest whose mode is not `CONSTRUCT_ONLY`;
4. a board-claim construction is admitted without receipt evidence and replay verification;
5. operationalization is admitted with only static or semantic verification;
6. the emitted receipt is not bound to the manifest digest and, when available, the exact Git HEAD/tree;
7. the admitted construction count falls below the manifest's declared minimum.

A falsified DFCM gate is `REFUSED` or `PARTIAL_ALIVE`; it may not be narrated as ALIVE.

## Extension

DFCM is intentionally combinatorial. Extension SHOULD normally occur by adding manifest values, ratings, constraints, exclusions, or downstream projections rather than adding imperative selection branches to the compiler.

The preferred extension order is:

`Preserve → Fence → Calculus → Exclusions → Falsifier → Extension → Operationalization`.

This ordering keeps reversible graph-domain exploration ahead of external-state execution.

## Operationalization

Canonical execution:

```bash
python3 tools/dfcm.py plan dfcm/process-intelligence.toml \
  --root . \
  --output receipts/PROCESS_INTELLIGENCE_DFCM_RUNTIME.json \
  --check
```

The compiler emits:

- manifest SHA-256;
- exact Git HEAD/tree when available;
- raw Cartesian size;
- admitted construction count;
- constraint refusal count;
- typed boundary-refusal count;
- deterministic selected portfolio;
- candidate digests and scores;
- falsifier state;
- canonical receipt SHA-256.

GitHub Actions executes the compiler twice against the exact PR head and byte-compares the receipts. Deterministic replay is therefore part of the gate, not an assertion in prose.

## Authorization Boundary

Passing DFCM authorizes **construction-space exploration and receipt emission only**. It does not authorize downstream external-state changes. Any downstream action remains subject to `DOWNSTREAM_AUTHORIZATION_LAW.md` and the target repository's own admission, execution, and receipt gates.
