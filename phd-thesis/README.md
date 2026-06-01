# PhD Thesis Corpus Manufacturing Workspace

This directory is the dissertation corpus manufacturing workspace for the process-intelligence research foundry.

## Purpose

This workspace manufactures the full artifact corpus for a PhD dissertation grounded in the Van der Aalst Constitution and the full-lifecycle process intelligence doctrine. It does not generate documents — it manufactures receipted, ledger-backed, proof-gated artifacts through a declared manufacturing pipeline.

## Branch

All work in this directory is manufactured on branch `phd-thesis-corpus-manufacture-001`.

## Directory Structure

| Directory | Purpose |
|---|---|
| `frontmatter/` | Title page, abstract, acknowledgments, table of contents |
| `chapters/` | Dissertation chapter TeX sources |
| `projects/` | Chapter-scoped project artifacts and source evidence |
| `ledgers/` | Receipt chain, claim ledger, workflow receipt |
| `scripts/` | Manufacturing pipeline scripts |
| `build/` | Compiled PDF output and build artifacts |

## Ledgers

- `ledgers/WORKFLOW_RECEIPT.yaml` — Manufacturing workflow provenance record
- `ledgers/DO_NOT_CLAIM_LEDGER.md` — Absolute claim prohibitions enforced across all artifacts

## ALIVE Verdict Criteria

An artifact is ALIVE only when all of the following are true:

1. TeX source files exist in the declared location
2. Source ledger is present and populated
3. Claim ledger prohibitions have been respected
4. PDF compiles without error
5. Compiled PDF hash is recorded in the receipt chain

## Doctrine

This workspace operates under the process-intelligence research foundry doctrine. No claim may be manufactured without source artifact backing. No downstream use of artifacts from this workspace is authorized until the research program issues authorization.
