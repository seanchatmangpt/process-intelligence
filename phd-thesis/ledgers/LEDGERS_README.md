# PhD Thesis Ledgers — Complete Index & Analysis

**Generated:** 2026-06-01  
**Repository:** ~/process-intelligence  
**Ledgers Location:** ~/process-intelligence/phd-thesis/ledgers/

This directory contains the authoritative ledgers for the PhD thesis corpus: indexed claims, evidence chains, search analysis, and milestone verdicts.

---

## Ledger Files

### 1. CLAIM_LEDGER.yaml (83.7 KB)
**Type:** Evidence-indexed claim registry  
**Format:** YAML with structured claim definitions  
**Contents:**
- Thesis claims (board-admissible manufacturing claims)
- Evidence citations (paper, experiment, checkpoint references)
- Claim status (ALIVE, PARTIAL, FAILED, WITHDRAWN)
- Dependency graph (which claims block others)

**Query Examples:**
```bash
# Find all ALIVE claims
grep -E "status:\s*ALIVE" CLAIM_LEDGER.yaml | wc -l

# Find claims citing Van der Aalst papers
grep -A5 "Van der Aalst" CLAIM_LEDGER.yaml

# Find PARTIAL claims requiring evidence
grep -E "status:\s*PARTIAL" CLAIM_LEDGER.yaml
```

**Authority:** Immutable once committed. Updates only via new audit entries.

---

### 2. EVIDENCE_LEDGER.yaml (28.8 KB)
**Type:** Evidence authority chain  
**Format:** YAML with source provenance  
**Contents:**
- Paper evidence (classified papers with mapping to claim types)
- Experiment evidence (benchmark fixtures, capability tests)
- Checkpoint evidence (ALIVE verdicts with gate criteria)
- Proof chain (receipt hashes linking claims to evidence)

**Query Examples:**
```bash
# Find all papers classified as Type Law sources
grep "type_law_authority" EVIDENCE_LEDGER.yaml

# Find evidence grounding a specific claim
grep "claim_id: CL_001" EVIDENCE_LEDGER.yaml

# Verify receipt chain integrity
sha256sum evidence-ledger-checkpoint-*.json
```

**Authority:** Authoritative source of truth for claim evidence grounding.

---

### 3. PROJECT_INDEX.yaml (43.5 KB)
**Type:** Corpus project inventory  
**Format:** YAML with project metadata  
**Contents:**
- Project registry (all projects in phd-thesis/projects/)
- TeX file listings (8 files per project for thesis compilation)
- Project-to-claim mappings (which projects ground which claims)
- Compilation dependencies (ordering for PDF generation)

**Query Examples:**
```bash
# List all projects
grep "^  - project_id:" PROJECT_INDEX.yaml

# Find TeX files for a specific project
grep -A8 "project_id: proofs-petri-nets" PROJECT_INDEX.yaml

# Count total TeX files
grep "tex_file:" PROJECT_INDEX.yaml | wc -l
```

**Authority:** Dynamic (updates as projects are added/modified).

---

### 4. SEARCH_ANALYSIS.md (newly created)
**Type:** Definition cluster analysis  
**Format:** Markdown with structured sections  
**Contents:**
- Definition clusters by maturity (Primary Thesis, Mathematical Spec, Operational Law)
- Core operators with usage frequency (alpha, kappa, rho, delta)
- Key equations with formal proofs (Soundness, Fitness, LTL invariants)
- Citation patterns and source authority
- Recurring terms taxonomy (Tier 1, 2, 3 concepts)
- Frame law alignment verification

**Key Findings:**
- ✓ Three definition clusters, all ALIVE maturity
- ✓ 33 doctrine files analyzed
- ✓ 10 core operators with type signatures
- ✓ Zero frame violations detected
- ✓ All frame-law concepts aligned with canonical terminology

**Authority:** Read-only analysis snapshot (regenerated as corpus grows).

---

### 5. DEFINITIONS_BY_CLUSTER.txt (newly created)
**Type:** Quick reference index of definitions  
**Format:** Plain text with structured sections  
**Contents:**
- Cluster 1: Primary Thesis definitions (5-point AKA, MAPE-K, full-lifecycle)
- Cluster 2: Mathematical specifications (kappa, alpha, rho operators; soundness theorems)
- Cluster 3: Operational law (authority hierarchy, elastic/compliance partition)
- Recurring term taxonomy (89 process × 53 receipt × 43 evidence ...)
- Frame law alignment table

**Use Case:** Quick lookup of definition specificity and citation counts.

**Example Query:**
```bash
# Find definition of kappa operator
grep -A15 "\[2.1\] GATE LOGIC" DEFINITIONS_BY_CLUSTER.txt
```

---

### 6. HOOK_AKA_SEARCH_INDEX.md (11.8 KB)
**Type:** Corpus search index (auto-generated)  
**Format:** Markdown with search results  
**Contents:**
- Knowledge Hook core search results (term: "knowledge hook", "hook receipt", etc.)
- Autonomic Knowledge Actuation search hits (8 files citing AKA)
- AutoInstinct/ccog search results (NO HITS in active source)
- CONSTRUCT8 search results (indirect via bounded-8 law)
- MAPE-K search results (6 canonical references)
- Negative findings (Frame Law terms not found as exact phrases)
- TTL ontology summary (6 HookPolicy instances in hook-law.ttl)

**Key Finding:**
All Frame Law concepts present in corpus under canonical terminology:
- "knowledge hook" → pm:HookPolicy / Andon Gate
- "No hook, no consequence" → Normalized as "A transition producing no evidence did not happen"
- "No receipt, no authority" → "Executions without receipts are narration, not closures"

---

### 7. DO_NOT_CLAIM_LEDGER.md (486 bytes)
**Type:** Claim boundary document  
**Format:** Markdown list  
**Contents:**
- Concepts explicitly forbidden from being manufactured as board-admissible claims
- Reasons (insufficient evidence, frame violations, design-phase-only)
- Examples of claims that would violate the boundary

**Purpose:** Prevents accidental claim manufacturing on unsupported concepts.

---

### 8. WORKFLOW_RECEIPT.yaml (476 bytes)
**Type:** Workflow execution receipt  
**Format:** YAML  
**Contents:**
- Workflow ID (current research phase)
- Execution timestamp
- Gate criteria verification status
- Authority signature

**Authority:** Cryptographic proof that workflow executed lawfully.

---

### 9. HOME_ROOT_CENSUS.txt (6.7 KB)
**Type:** Directory structure census  
**Format:** Plain text tree listing  
**Contents:**
- Complete directory hierarchy of ~/process-intelligence
- File count per directory
- Notable files (CLAUDE.md, COVENANT.md, README.md)

**Use Case:** Quick verification that all expected directories exist.

---

## How to Use These Ledgers

### For Claim Verification
```bash
# 1. Find a specific claim
grep "claim_id: CL_042" CLAIM_LEDGER.yaml

# 2. Verify its evidence is ALIVE
grep -A5 "claim_id: CL_042" EVIDENCE_LEDGER.yaml

# 3. Confirm citations in primary sources
grep "CL_042" ../../../sources/papers/paper-to-type-law.md
```

### For Thesis Compilation
```bash
# 1. Check which projects are ready
grep "status: ready" PROJECT_INDEX.yaml

# 2. Find TeX files for a ready project
grep -A8 "status: ready" PROJECT_INDEX.yaml | grep "tex_file:"

# 3. Compile in dependency order (defined in PROJECT_INDEX.yaml)
```

### For Definition Lookup
```bash
# 1. Find a concept by name
grep "kappa(τ)" DEFINITIONS_BY_CLUSTER.txt

# 2. Check its specificity (MAXIMUM, HIGH, MEDIUM)
grep -A10 "\[2.1\] GATE LOGIC" DEFINITIONS_BY_CLUSTER.txt

# 3. Follow citation count to source
grep "citation Count:" DEFINITIONS_BY_CLUSTER.txt | sort -t: -k2 -nr
```

### For Frame Law Compliance
```bash
# 1. Check overall alignment status
tail -20 SEARCH_ANALYSIS.md

# 2. Find which corpus term corresponds to each Frame Law concept
grep "^| Frame Law" SEARCH_ANALYSIS.md -A10

# 3. Verify no frame violations exist
grep "VIOLATION" SEARCH_ANALYSIS.md
```

---

## Ledger Generation Workflow

These ledgers are generated via a **deterministic, gated workflow**:

1. **Source Authority Audit** → Classifies papers, experiments, checkpoints
2. **Claim Extraction** → Identifies board-admissible manufacturing claims
3. **Evidence Mapping** → Links claims to source evidence (paper/experiment/checkpoint)
4. **Status Assignment** → ALIVE (all gates met), PARTIAL (evidence gap), FAILED (invalid)
5. **Immutable Commit** → Checksum ledger files, emit receipt, tag commit

**Immutability Doctrine:**
- Ledger files are **append-only**
- Corrections are via **addendum commits**, never rewrites
- Status downgrade (ALIVE → PARTIAL) requires **audit documentation**
- Verdict deletions are **prohibited** (mark as WITHDRAWN instead)

---

## Verification

To verify ledger integrity:

```bash
# Check file sizes haven't changed unexpectedly
ls -la *.yaml *.md *.txt

# Verify all cited evidence actually exists
grep "evidence:" CLAIM_LEDGER.yaml | cut -d: -f3 | sort -u

# Confirm no circular claim dependencies
grep "blocks:" CLAIM_LEDGER.yaml | sort -u
```

---

## See Also

- **~/process-intelligence/COVENANT.md** — What may and may not be claimed
- **~/process-intelligence/phd-thesis/README.md** — Thesis corpus overview
- **~/process-intelligence/doctrine/** — Immutable process law definitions
- **~/process-intelligence/sources/papers/** — Paper analysis and classification

---

*Last Updated: 2026-06-01 by Corpus Search Agent*  
*Ledgers Directory is canonical source of truth for claim evidence grounding.*
