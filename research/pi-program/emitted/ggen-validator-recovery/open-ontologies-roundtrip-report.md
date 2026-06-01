# Open Ontologies Roundtrip Check — Phase 7

**Timestamp:** 2026-06-01T13:15:45.878195
**Classification:** `AVAILABLE`

## Summary

- **TTL File Validation:** VALIDATE_PASSED
- **Graph Loading:** LOAD_SUCCEEDED
- **Smoke Queries:** 4/4 passed
- **Receipt TTL:** RECEIPT_NOT_FOUND

---

## 1. TTL File Validation

**Status:** VALIDATE_PASSED

### Files Analyzed


| File | Status | Triples | Subjects | Predicates | Objects |
|------|--------|---------|----------|-----------|---------|
| `checkpoint-ledger.ttl` | ✓ | 221 | 46 | 35 | 135 |

| File | Status | Triples | Subjects | Predicates | Objects |
|------|--------|---------|----------|-----------|---------|
| `conformance-ledger.ttl` | ✓ | 156 | 34 | 29 | 100 |

| File | Status | Triples | Subjects | Predicates | Objects |
|------|--------|---------|----------|-----------|---------|
| `forbidden-collapse-law.ttl` | ✓ | 202 | 40 | 30 | 128 |

| File | Status | Triples | Subjects | Predicates | Objects |
|------|--------|---------|----------|-----------|---------|
| `graduation-boundary.ttl` | ✓ | 155 | 33 | 27 | 96 |

| File | Status | Triples | Subjects | Predicates | Objects |
|------|--------|---------|----------|-----------|---------|
| `pi-ggen-audit-law.ttl` | ✓ | 327 | 71 | 29 | 236 |

| File | Status | Triples | Subjects | Predicates | Objects |
|------|--------|---------|----------|-----------|---------|
| `pi-ggen-checkpoint-ledger.ttl` | ✓ | 466 | 97 | 91 | 244 |

| File | Status | Triples | Subjects | Predicates | Objects |
|------|--------|---------|----------|-----------|---------|
| `pi-ggen-generation-ledger.ttl` | ✓ | 316 | 52 | 41 | 171 |

| File | Status | Triples | Subjects | Predicates | Objects |
|------|--------|---------|----------|-----------|---------|
| `pi-ggen-invalid-extension-ledger.ttl` | ✓ | 260 | 50 | 28 | 116 |

| File | Status | Triples | Subjects | Predicates | Objects |
|------|--------|---------|----------|-----------|---------|
| `pi-ggen-project-registry.ttl` | ✓ | 349 | 60 | 40 | 239 |

| File | Status | Triples | Subjects | Predicates | Objects |
|------|--------|---------|----------|-----------|---------|
| `pi-ggen-source-ledger.ttl` | ✓ | 513 | 66 | 29 | 212 |

| File | Status | Triples | Subjects | Predicates | Objects |
|------|--------|---------|----------|-----------|---------|
| `pi-ggen-unified-run.ttl` | ✓ | 168 | 25 | 34 | 124 |

| File | Status | Triples | Subjects | Predicates | Objects |
|------|--------|---------|----------|-----------|---------|
| `pi-program.ttl` | ✓ | 176 | 40 | 23 | 117 |

| File | Status | Triples | Subjects | Predicates | Objects |
|------|--------|---------|----------|-----------|---------|
| `project-registry.ttl` | ✓ | 195 | 38 | 24 | 145 |

| File | Status | Triples | Subjects | Predicates | Objects |
|------|--------|---------|----------|-----------|---------|
| `research-artifact-ledger.ttl` | ✓ | 238 | 36 | 23 | 137 |

**File Count:** 14
**Valid Count:** 14

---

## 2. Graph Loading

**Status:** LOAD_SUCCEEDED


**Unified Graph Statistics:**

- **Total Triples:** 3684
- **Unique Subjects:** 661
- **Unique Predicates:** 278
- **Unique Objects:** 1941

---

## 3. SPARQL Smoke Queries

**Overall:** 4/4 queries passed


### count_all_instances

**Status:** ✓ QUERY_PASSED

- **Result Rows:** 1
- **Sample:** [(rdflib.term.Literal('3684', datatype=rdflib.term.URIRef('http://www.w3.org/2001/XMLSchema#integer')),)]

### count_checkpoints

**Status:** ✓ QUERY_PASSED

- **Result Rows:** 1
- **Sample:** [(rdflib.term.Literal('0', datatype=rdflib.term.URIRef('http://www.w3.org/2001/XMLSchema#integer')),)]

### count_artifacts

**Status:** ✓ QUERY_PASSED

- **Result Rows:** 1
- **Sample:** [(rdflib.term.Literal('0', datatype=rdflib.term.URIRef('http://www.w3.org/2001/XMLSchema#integer')),)]

### sample_subjects

**Status:** ✓ QUERY_PASSED

- **Result Rows:** 5
- **Sample:** [(rdflib.term.URIRef('https://process.intelligence/source/PROJ_3_ONTOLOGY_6'), rdflib.term.Literal('10', datatype=rdflib.term.URIRef('http://www.w3.org/2001/XMLSchema#integer'))), (rdflib.term.URIRef('https://process.intelligence/checkpoint/SUBSTRATE_COMPLETE_001'), rdflib.term.Literal('26', datatype=rdflib.term.URIRef('http://www.w3.org/2001/XMLSchema#integer'))), (rdflib.term.URIRef('https://process.intelligence/ggen-unified-run/PHASE_SHACL_VALIDATION'), rdflib.term.Literal('4', datatype=rdflib.term.URIRef('http://www.w3.org/2001/XMLSchema#integer')))]

---

## 4. Receipt TTL Verification

**Status:** RECEIPT_NOT_FOUND


**Candidates Checked:**

- /Users/sac/process-intelligence/research/pi-program/ggen/ontology/receipt.ttl
- /Users/sac/process-intelligence/research/pi-program/ggen/ontology/pi-ggen-receipt.ttl
- /Users/sac/process-intelligence/research/pi-program/ggen/ontology/checkpoint-receipt.ttl

*Note: Receipt TTL not found. This is acceptable if receipts are stored elsewhere.*

---

## Classification

**Phase 7 Verdict:** `AVAILABLE`

### Mapping to Requirements

- `AVAILABLE` — Open Ontologies is operational; all validation and query tests pass
- `NOT_CONFIGURED` — Open Ontologies not available; does not block ALIVE checkpoint
- `VALIDATE_FAILED` — TTL files could not be parsed
- `LOAD_FAILED` — Graph could not be unified
- `QUERY_FAILED` — SPARQL queries failed against loaded graph
- `VERSION_FAILED` — Graph snapshot versioning not available

**Current Status:** `AVAILABLE`

---

## Raw Validation Data

```json
{
  "ttl_validation": {
  "status": "VALIDATE_PASSED",
  "file_count": 14,
  "valid_count": 14,
  "files": {
    "checkpoint-ledger.ttl": {
      "status": "VALID",
      "triples": 221,
      "subjects": 46,
      "predicates": 35,
      "objects": 135
    },
    "conformance-ledger.ttl": {
      "status": "VALID",
      "triples": 156,
      "subjects": 34,
      "predicates": 29,
      "objects": 100
    },
    "forbidden-collapse-law.ttl": {
      "status": "VALID",
      "triples": 202,
      "subjects": 40,
      "predicates": 30,
      "objects": 128
    },
    "graduation-boundary.ttl": {
      "status": "VALID",
      "triples": 155,
      "subjects": 33,
      "predicates": 27,
      "objects": 96
    },
    "pi-ggen-audit-law.ttl": {
      "status": "VALID",
      "triples": 327,
      "subjects": 71,
      "predicates": 29,
      "objects": 236
    },
    "pi-ggen-checkpoint-ledger.ttl": {
      "status": "VALID",
      "triples": 466,
      "subjects": 97,
      "predicates": 91,
      "objects": 244
    },
    "pi-ggen-generation-ledger.ttl": {
      "status": "VALID",
      "triples": 316,
      "subjects": 52,
      "predicates": 41,
      "objects": 171
    },
    "pi-ggen-invalid-extension-ledger.ttl": {
      "status": "VALID",
      "triples": 260,
      "subjects": 50,
      "predicates": 28,
      "objects": 116
    },
    "pi-ggen-project-registry.ttl": {
      "status": "VALID",
      "triples": 349,
      "subjects": 60,
      "predicates": 40,
      "objects": 239
    },
    "pi-ggen-source-ledger.ttl": {
      "status": "VALID",
      "triples": 513,
      "subjects": 66,
      "predicates": 29,
      "objects": 212
    },
    "pi-ggen-unified-run.ttl": {
      "status": "VALID",
      "triples": 168,
      "subjects": 25,
      "predicates": 34,
      "objects": 124
    },
    "pi-program.ttl": {
      "status": "VALID",
      "triples": 176,
      "subjects": 40,
      "predicates": 23,
      "objects": 117
    },
    "project-registry.ttl": {
      "status": "VALID",
      "triples": 195,
      "subjects": 38,
      "predicates": 24,
      "objects": 145
    },
    "research-artifact-ledger.ttl": {
      "status": "VALID",
      "triples": 238,
      "subjects": 36,
      "predicates": 23,
      "objects": 137
    }
  }
},
  "graph_load": {
  "status": "LOAD_SUCCEEDED",
  "total_triples": 3684,
  "total_subjects": 661,
  "total_predicates": 278,
  "total_objects": 1941
},
  "smoke_queries": {
  "count_all_instances": {
    "status": "QUERY_PASSED",
    "row_count": 1,
    "results_sample": "[(rdflib.term.Literal('3684', datatype=rdflib.term.URIRef('http://www.w3.org/2001/XMLSchema#integer')),)]"
  },
  "count_checkpoints": {
    "status": "QUERY_PASSED",
    "row_count": 1,
    "results_sample": "[(rdflib.term.Literal('0', datatype=rdflib.term.URIRef('http://www.w3.org/2001/XMLSchema#integer')),)]"
  },
  "count_artifacts": {
    "status": "QUERY_PASSED",
    "row_count": 1,
    "results_sample": "[(rdflib.term.Literal('0', datatype=rdflib.term.URIRef('http://www.w3.org/2001/XMLSchema#integer')),)]"
  },
  "sample_subjects": {
    "status": "QUERY_PASSED",
    "row_count": 5,
    "results_sample": "[(rdflib.term.URIRef('https://process.intelligence/source/PROJ_3_ONTOLOGY_6'), rdflib.term.Literal('10', datatype=rdflib.term.URIRef('http://www.w3.org/2001/XMLSchema#integer'))), (rdflib.term.URIRef('https://process.intelligence/checkpoint/SUBSTRATE_COMPLETE_001'), rdflib.term.Literal('26', datatype=rdflib.term.URIRef('http://www.w3.org/2001/XMLSchema#integer'))), (rdflib.term.URIRef('https://process.intelligence/ggen-unified-run/PHASE_SHACL_VALIDATION'), rdflib.term.Literal('4', datatype=rdflib.term.URIRef('http://www.w3.org/2001/XMLSchema#integer')))]"
  }
},
  "receipt_check": {
  "status": "RECEIPT_NOT_FOUND",
  "candidates_checked": [
    "/Users/sac/process-intelligence/research/pi-program/ggen/ontology/receipt.ttl",
    "/Users/sac/process-intelligence/research/pi-program/ggen/ontology/pi-ggen-receipt.ttl",
    "/Users/sac/process-intelligence/research/pi-program/ggen/ontology/checkpoint-receipt.ttl"
  ]
}
}
```

---

**Generated:** 2026-06-01T13:15:45.878195
**Tool:** rdflib 2.0.9
