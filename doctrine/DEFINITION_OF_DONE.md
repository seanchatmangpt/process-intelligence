# Definition of Done

**Status:** ACTIVE — append-only doctrine  
**Authority:** process-intelligence research foundry  
**Initial implementation:** 2026-08-13  
**Supersedes:** no prior sealed artifact

## Definition

**DONE is scoped closure, not activity completion.**

For a declared scope `S`, let `R(S)` be its required predicates and `F(S)` its falsifiers. Then:

\[
Done(S) \iff \bigwedge_{r \in R(S)} r \land \neg \bigvee_{f \in F(S)} f.
\]

A scope is not DONE merely because work stopped, files exist, tests ran once, or a human says it is complete. DONE requires the declared predicates to be evidenced against an exact subject and every declared falsifier to be false.

## Relationship to ALIVE

- **ALIVE** means a named capability was actually executed and demonstrated against an exact subject.
- **DONE** means all required ALIVE capabilities, receipts, replay obligations, gap obligations, and release predicates for a declared scope are closed.
- Therefore `DONE ⇒ required ALIVE predicates`, but an individual ALIVE capability does not imply repository or release DONE.

## Definitions of Done

### Artifact DONE

An artifact is DONE when it has identity, source binding, reversibility, and a verification path. Anonymous, unbound, or unverifiable outputs are not done.

### Evidence DONE

Evidence is DONE when it names a witness, emits a receipt, is replayable, and exposes named refusal. Activity without witness and hash-only "receipts" are not done.

### Gate DONE

A gate is DONE when it is named, binds the exact subject, executes deterministically, fails closed, and emits a receipt artifact. Synthetic-subject execution or relabelled failure falsifies DONE.

### Repository DONE

The repository is DONE when required capability gates are ALIVE, there are zero unmitigated gaps, tests are green, and receipts bind the exact head. For this repository the mandatory capability gates are `PROCESS_INTELLIGENCE_ALIVE_002` and `PROCESS_INTELLIGENCE_DFCM_ALIVE_001`.

### Release-Candidate DONE

A release candidate is DONE when Repository DONE holds, the PR is mergeable, all required checks are green, and no unreceipted external-state transition is required to justify the claim.

Merge itself is a separate state transition. After merge, push-to-main workflows re-execute the gates against the merge subject.

## Falsifier

Any declared falsifier is sufficient to make the corresponding scope `NOT_DONE`. Failure is not averaged away. There is no percentage-complete path to DONE.

## Machine Contract

`dod/process-intelligence.toml` is the canonical machine-readable contract.  
`tools/verify_done.py` validates the structural contract and emits a deterministic receipt.  
`.github/workflows/definition-of-done.yml` composes the structural verifier with the existing ALIVE_002 and DFCM executors.

The workflow executes each underlying capability rather than trusting historical checkpoint prose.

## Receipt Law

A DoD receipt binds:

- exact Git HEAD and tree;
- DoD manifest digest;
- all declared scopes;
- required artifact digests;
- falsifier state;
- underlying ALIVE_002 execution;
- underlying DFCM execution and deterministic replay.

A DoD receipt is evidence for the exact subject only. It does not transfer standing to a later commit without re-execution.

## Release Law

A pull-request head may reach `RELEASE_CANDIDATE_DONE`.  
The merge commit must then re-run all push-to-main gates before `MAIN_DONE` can be claimed.

This preserves the distinction between a proven candidate and the actual integrated subject.
