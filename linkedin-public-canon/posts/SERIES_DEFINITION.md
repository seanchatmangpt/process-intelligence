---
artifact: SERIES_DEFINITION
project: linkedin-public-canon
date: 2026-06-02
status: ACTIVE
gate: Fish Gate (primary)
---

# Process Intelligence Public Canon — Series Definition

## Series Name

**Process Intelligence Public Canon**

## Tagline

> No claim without a receipt. No receipt without a proof gate.

## Mission

A weekly LinkedIn series grounding public claims about process intelligence in upstream ALIVE
receipts from the process-intelligence research foundry. Each post cites a specific receipt
identifier and checkpoint date. No post is published without an upstream ALIVE artifact.

This is the city gate — where process intelligence findings pass into public witness.

---

## Publishing Cadence

- **Frequency:** Weekly (every Monday)
- **Target:** 1 post per week minimum; up to 3 per week during active checkpoint periods
- **Series minimum for ALIVE verdict:** 3 published posts

---

## Topic Framework

Topics are drawn from ALIVE-gated doctrine files in /Users/sac/process-intelligence/doctrine/.
Each topic maps to a gate and a doctrine source. No topic may be published until its upstream
doctrine is ALIVE-gated.

### Topic Track 1: Representational Separability (CONSTRUCT8)

| Post | Topic | Upstream Receipt | Gate |
|------|-------|-----------------|------|
| POST_001 | CONSTRUCT8 defense sentence — representational separability | C8_MARKET_PHYSICS_ALIVE_002 | Fish Gate |
| POST_002 | What is a Rust witness? Branchless proof without prediction claims | C8_MARKET_PHYSICS_ALIVE_002 | Fish Gate |
| POST_003 | Market Planck cells, event horizons, and hidden bodies as first-class state | C8_MARKET_PHYSICS_ALIVE_002 | Fish Gate |

### Topic Track 2: Process Intelligence Doctrine (Blue River Dam)

| Post | Topic | Upstream Receipt | Gate |
|------|-------|-----------------|------|
| POST_004 | Activity is not evidence — the Blue River Dam distinction | doctrine/BLUE_RIVER_DAM.md | Fish Gate |
| POST_005 | Conformance checking is law enforcement, not diagnostics | doctrine/CONFORMANCE_AS_LAW.md | Water Gate |
| POST_006 | Object-centric vs. trace-centric — why XES is insufficient for multi-object processes | doctrine/OBJECT_CENTRIC_SUPREMACY.md | Water Gate |

### Topic Track 3: Full-Lifecycle Process Intelligence

| Post | Topic | Upstream Receipt | Gate |
|------|-------|-----------------|------|
| POST_007 | What full-lifecycle process intelligence means — and what it is not | doctrine/PROCESS_INTELLIGENCE_DEFINED.md | Fish Gate |
| POST_008 | Five maturity levels: from recording activity to adjudicating process truth | doctrine/PROCESS_INTELLIGENCE_DEFINED.md | Water Gate |
| POST_009 | The upstream dam principle: whoever controls admissible process truth controls downstream AI | doctrine/BLUE_RIVER_DAM.md | Sheep Gate |

---

## Content Rules (Connection Rules Mapping)

1. **Connection Rule 1 (Fish Gate):** Every post must be grounded in an ALIVE-gated upstream
   artifact. No speculative claims. No unreceipted assertions.

2. **Connection Rule 2 (Water Gate):** Teaching posts must be grounded in doctrine files.
   Cite the doctrine source by name (e.g., "doctrine/CONFORMANCE_AS_LAW.md").

3. **Connection Rule 3 (Sheep Gate):** The manifesto declares what is at stake — the category
   claim for full-lifecycle process intelligence — not how the engine works. Posts must not
   expose internal receipt formats, gate implementation details, or private repo paths.

4. **Connection Rule 4 (Fish Gate entry):** Every post must be discoverable — use consistent
   series name "Process Intelligence Public Canon" and hashtags for series continuity.

---

## Quality Gates per Post

Before a post draft is moved to status: PUBLISHED, it must satisfy:

- [ ] Upstream receipt identifier is cited
- [ ] Checkpoint date is cited
- [ ] No private repo internals exposed (no file paths like `/Users/sac/...`)
- [ ] Series name "Process Intelligence Public Canon" appears in post
- [ ] LinkedIn URL recorded in PUBLICATION_REGISTRY.yaml after publication

---

## Series Registry

Posts are tracked in PUBLICATION_REGISTRY.yaml under `publications[]`.
Each entry must include:
- `post_id` (e.g., POST_001)
- `title`
- `linkedin_url` (PENDING until published)
- `upstream_receipt`
- `checkpoint_identifier`
- `publication_date`
- `gate_satisfied`
