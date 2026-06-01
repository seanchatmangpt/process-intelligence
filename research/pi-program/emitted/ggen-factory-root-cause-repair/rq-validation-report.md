# RQ Validation Report — Phase 6

**Date:** 2026-06-01  
**Validator:** rdflib (SPARQL 1.1 Python)  
**Gate Status:** PASS

## Summary

All 63 SPARQL query files validated successfully. 3 files had syntax errors that were fixed in-place. All referenced queries (6 files in active generation rules) pass validation.

| Category | Count |
|---|---|
| Total Files Tested | 63 |
| Passed | 63 |
| Failed | 0 |
| Syntax Errors (Initial) | 3 |
| Syntax Errors (After Fix) | 0 |
| Referenced in ggen.toml | 6 |

## Remediation Applied

### 1. audit-checkpoints-have-receipts-or-explicit-missing.rq

**Error:** SPARQL syntax error — improperly nested FILTER clause in ASK query  
**Root Cause:** Attempted to use boolean grouping with parentheses containing multiple FILTER statements (invalid SPARQL 1.1 syntax)  
**Fix:** Removed parentheses and flattened FILTER conditions using implicit AND semantics  
**Status:** ✓ FIXED

```sparql
# Before (INVALID)
ASK {
    FILTER NOT EXISTS {
        ?checkpoint a pi:Checkpoint .
        ?checkpoint pi:status "SEALED" .
        (
            FILTER NOT EXISTS { ?checkpoint pi:receiptReference ?ref }
            AND
            FILTER NOT EXISTS { ?checkpoint pi:blake3Seal ?seal }
        )
    }
}

# After (VALID)
ASK {
    FILTER NOT EXISTS {
        ?checkpoint a pi:Checkpoint .
        ?checkpoint pi:status "SEALED" .
        FILTER NOT EXISTS { ?checkpoint pi:receiptReference ?ref }
        FILTER NOT EXISTS { ?checkpoint pi:blake3Seal ?seal }
    }
}
```

### 2. audit-every-generation-rule-has-query-template-output.rq

**Error:** SPARQL syntax error — nested FILTER keywords with OR operators (not valid syntax)  
**Root Cause:** Attempted to use multiple FILTER statements chained with OR; SPARQL requires conditions inside a single FILTER expression  
**Fix:** Consolidated into single FILTER with boolean || operators for disjunctive conditions  
**Status:** ✓ FIXED

```sparql
# Before (INVALID)
ASK {
    FILTER NOT EXISTS {
        ?rule a pi:GenerationRule .
        ?rule pi:status "READY" .
        (
            FILTER NOT EXISTS { ?rule pi:query ?q }
            OR
            FILTER NOT EXISTS { ?rule pi:template ?t }
            OR
            FILTER NOT EXISTS { ?rule pi:outputFile ?o }
        )
    }
}

# After (VALID)
ASK {
    FILTER NOT EXISTS {
        ?rule a pi:GenerationRule .
        ?rule pi:status "READY" .
        FILTER (
            NOT EXISTS { ?rule pi:query ?q }
            ||
            NOT EXISTS { ?rule pi:template ?t }
            ||
            NOT EXISTS { ?rule pi:outputFile ?o }
        )
    }
}
```

### 3. select-subagent-prompts.rq

**Error:** SPARQL syntax error — property path expression not properly closed in OPTIONAL  
**Root Cause:** Used abbreviated property path `pm:hasPhase/?phase pm:hasSubagentRole` inside OPTIONAL block, which requires expansion into explicit triple patterns  
**Fix:** Expanded property path notation into separate triple patterns with proper OPTIONAL and BIND sequencing  
**Status:** ✓ FIXED

```sparql
# Before (INVALID)
OPTIONAL {
    ?program pm:hasPhase/?phase pm:hasSubagentRole ?role . BIND (?program AS ?ownerProgram) .
}

# After (VALID)
OPTIONAL {
    ?program pm:hasPhase ?phase .
    ?phase pm:hasSubagentRole ?role .
    BIND (?program AS ?ownerProgram)
}
```

## Query Type Distribution

| Type | Count | Notes |
|---|---|---|
| SELECT | 51 | Primary query type for data retrieval and reports |
| ASK | 12 | Boolean verification and audit queries |
| CONSTRUCT | 0 | Not used in current ruleset |
| DESCRIBE | 0 | Not used in current ruleset |

## Referenced Queries in Active Generation Rules

### PI Program

1. **select-all-checkpoints.rq** (SELECT)
   - Used by: `checkpoint-ledger` generation rule
   - Status: ✓ PASS
   - Purpose: Render ledger of all checkpoints with receipt status

2. **select-all-generation-rules.rq** (SELECT)
   - Used by: `generation-rule-ledger` generation rule
   - Status: ✓ PASS
   - Purpose: Render ledger of all generation rules with execution status

3. **select-invalid-ggen-files.rq** (SELECT)
   - Used by: `invalid-ggen-classification` generation rule
   - Status: ✓ PASS
   - Purpose: Render classification ledger for legacy .ggen files

4. **select-all-rendered-artifacts.rq** (SELECT)
   - Used by: `rendered-artifact-ledger` generation rule
   - Status: ✓ PASS
   - Purpose: Render ledger of all rendered artifacts with source trace

5. **select-warrant-paths.rq** (SELECT)
   - Used by: `warrant-path-proof` generation rule (DISABLED)
   - Status: ✓ PASS
   - Purpose: Prove Prompt Manufactory warrant path through ggen execution

### Prompt Manufactory

6. **select-rendered-prompts.rq** (SELECT)
   - Used by: `structural-placeholder` generation rule
   - Status: ✓ PASS
   - Purpose: Structural placeholder for ggen.toml validation

## Validation Methodology

Each RQ file was:
1. Parsed using rdflib's SPARQL 1.1 query parser
2. Validated for syntactic correctness
3. Classified by query type (SELECT/ASK/CONSTRUCT/DESCRIBE)
4. Cross-referenced against ggen.toml generation rules

### Validator Output

- **Tool:** rdflib (Python RDF library)
- **Standard:** SPARQL 1.1
- **Mode:** Full parse validation
- **Coverage:** All 63 files in PI program and Prompt Manufactory projects

## Missing Query Analysis

All 6 queries referenced in ggen.toml exist and validate:
- ✓ select-all-checkpoints.rq
- ✓ select-all-generation-rules.rq
- ✓ select-invalid-ggen-files.rq
- ✓ select-all-rendered-artifacts.rq
- ✓ select-warrant-paths.rq
- ✓ select-rendered-prompts.rq

**No missing queries to create.**

## Scope Notes

### Out of Scope

- `open-ontologies-program-integration` queries — no files found (out of scope by absence)

### Disabled Queries

- `select-warrant-paths.rq` is referenced by a disabled generation rule (`warrant-path-proof`). The rule is disabled due to template parsing errors (Phase 8 recovery). The query itself validates and passes.

## Gate Conclusion

✓ **PASS** — Phase 6 SPARQL validation gate passes.

- All 63 active RQ files parse correctly
- 3 syntax errors identified and fixed in-place
- All 6 ggen.toml-referenced queries exist and validate
- No missing queries to create
- Ready for Phase 7+ execution and integration testing
