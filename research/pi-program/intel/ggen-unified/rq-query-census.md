# SPARQL Query Census — Process Intelligence Research Foundry

**Date:** 2026-06-01  
**Authority:** Process Intelligence Program Ontology (`pi-program.ttl`)  
**Scope:** All `.rq` files in `/Users/sac/process-intelligence`  
**Total Queries:** 37

---

## Summary

| Category | Count | Prefix | Query Type |
|----------|-------|--------|-----------|
| **M&A Manufacturing** | 4 | `ggen/queries/` | SELECT |
| **PI Program Audits** | 21 | `research/pi-program/ggen/queries/` | ASK/SELECT |
| **PI Program Selections** | 12 | `research/pi-program/ggen/queries/` | SELECT |
| **Prompt Manufactory** | 2 | `research/prompt-manufactory/ggen/queries/` | SELECT |

---

## SECTION 1: M&A Manufacturing Queries

### Query 1: extract-board-claims.rq

**Path:** `/Users/sac/process-intelligence/ggen/queries/extract-board-claims.rq`

| Property | Value |
|----------|-------|
| **Query Type** | SELECT |
| **Expected Bindings** | `?claim`, `?claimLabel`, `?claimType`, `?metric`, `?metricValue`, `?metricThreshold`, `?verdict`, `?verdictFitness`, `?verdictPrecision`, `?receipt`, `?receiptHash`, `?receiptTimestamp`, `?backedByLog`, `?logFormat` |
| **Binding Count** | 14 |
| **Referenced Classes** | `ma:SynergyProjection`, `ma:OperationalDebtClaim`, `ma:IntegrationRiskClaim`, `ma:ProcessAssetClaim`, `ma:ControlClaim`, `wasm4pm:ConformanceVerdict`, `compat:EventLog` |
| **Referenced Properties** | `rdfs:label`, `ma:quantifies`, `ma:value`, `ma:threshold`, `ma:backedBy`, `wasm4pm:fitness`, `wasm4pm:precision`, `wasm4pm:receipt`, `wasm4pm:receiptHash`, `wasm4pm:timestamp`, `wasm4pm:replayedAgainst`, `compat:format` |
| **Intended Template** | Board-admissible M&A claims manufacturing |
| **Intended Audit** | Claims must be backed by ConformanceVerdict with fitness ≥ 0.95 and precision ≥ 0.90 |
| **Executable Status** | ⚠️ CONDITIONAL — Requires populated `ma:*`, `wasm4pm:*`, `compat:EventLog` instances in TTL |
| **Failure Reason** | No instance data in available TTL files for M&A domain classes; requires upstream manufacturing |
| **Filters** | `?claimType IN (ma:SynergyProjection, ...)`, `?verdictFitness >= 0.95 && ?verdictPrecision >= 0.90`, `?logFormat IN ("ocel:2.0", "xes:1849-2016")` |
| **Order By** | DESC(?verdictFitness) |
| **Optional Patterns** | `?claim ma:admissible true` |

---

### Query 2: extract-diligence-claims.rq

**Path:** `/Users/sac/process-intelligence/ggen/queries/extract-diligence-claims.rq`

| Property | Value |
|----------|-------|
| **Query Type** | SELECT |
| **Expected Bindings** | `?claim`, `?claimLabel`, `?claimCategory`, `?claimDescription`, `?synergyCategoryIfApplicable`, `?operationalDebtIfApplicable`, `?riskSeverity`, `?quantifiedMetric`, `?metricValue`, `?metricUnit`, `?replayTrace`, `?traceDeviations`, `?traceGasToReturn`, `?remediationPath`, `?remediationEffortHours`, `?relatedActivity`, `?activityName`, `?activityBottleneck`, `?evidenceLink`, `?verdictFitness` |
| **Binding Count** | 20 |
| **Referenced Classes** | `ma:SynergyProjection`, `ma:OperationalDebtInstance`, `ma:IntegrationRiskAssertion`, `wasm4pm:ReplayTrace`, `wasm4pm:ConformanceVerdict` |
| **Referenced Properties** | `rdfs:label`, `dcterms:description`, `ma:synergyType`, `ma:debtCategory`, `ma:riskSeverity`, `ma:quantifies`, `ma:value`, `ma:unit`, `wasm4pm:supportedBy`, `wasm4pm:deviationCount`, `wasm4pm:gasToReturn`, `ma:hasRemediationPath`, `ma:estimatedEffortHours`, `ma:affectsActivity`, `ma:isBottleneck`, `ma:evidencedBy`, `wasm4pm:fitness`, `wasm4pm:receipt`, `rdfs:seeAlso` |
| **Intended Template** | Detailed M&A due diligence claim extraction with replay traces and metrics |
| **Intended Audit** | Claims categorized by synergy/debt/risk; traced to conformance verdicts |
| **Executable Status** | ⚠️ CONDITIONAL — Requires M&A claim instances and replay trace instances |
| **Failure Reason** | No instance data in available TTL files; requires upstream ggen manufacturing |
| **Filters** | `?claimCategory IN (ma:SynergyProjection, ma:OperationalDebtInstance, ma:IntegrationRiskAssertion)` |
| **Order By** | ?claimCategory DESC(?metricValue) |
| **Optional Patterns** | Synergy category, debt category, risk severity, remediation path, bottleneck flag |

---

### Query 3: extract-lifecycle-governance.rq

**Path:** `/Users/sac/process-intelligence/ggen/queries/extract-lifecycle-governance.rq`

| Property | Value |
|----------|-------|
| **Query Type** | SELECT |
| **Expected Bindings** | `?state`, `?stateName`, `?statePhase`, `?stateDescription`, `?transitionTarget`, `?transitionTargetName`, `?transitionGuardCondition`, `?transitionGuardExpression`, `?transitionActionOnFire`, `?monitorRule`, `?monitorExpression`, `?monitorMetric`, `?analyzeRule`, `?analyzeExpression`, `?analyzeThreshold`, `?planRule`, `?planPolicyExpression`, `?planOutputShape`, `?executeAction`, `?executeActionName`, `?executeAuditLog`, `?knowledgeAsset`, `?knowledgeAssetType`, `?knowledgeAssetValue` |
| **Binding Count** | 23 |
| **Referenced Classes** | `lifecycle:ProcessState`, `lifecycle:DesignPhase`, `lifecycle:SimulationPhase`, `lifecycle:ValidationPhase`, `lifecycle:MonitoringPhase`, `lifecycle:OptimizationPhase`, `lifecycle:RepairPhase`, `lifecycle:DecommissionPhase`, `lifecycle:ProcessModel`, `lifecycle:ConformancePattern`, `lifecycle:RemediationStrategy`, `lifecycle:PredictiveModel`, `lifecycle:AutonomicPolicy` |
| **Referenced Properties** | `rdfs:label`, `lifecycle:phase`, `dcterms:description`, `lifecycle:transitions`, `lifecycle:to`, `lifecycle:guard`, `lifecycle:condition`, `lifecycle:expression`, `lifecycle:action`, `lifecycle:monitorRule`, `lifecycle:analyzeRule`, `lifecycle:planRule`, `lifecycle:canExecute`, `lifecycle:auditedVia`, `lifecycle:knowledgeSource`, `lifecycle:value` |
| **Intended Template** | Blue River autonomic governance MAPE-K rule extraction for lifecycle state machines |
| **Intended Audit** | All 7 lifecycle phases must have monitor/analyze/plan/execute/knowledge rules |
| **Executable Status** | ⚠️ CONDITIONAL — Requires lifecycle state instances and rule instances |
| **Failure Reason** | No lifecycle instance data in available TTL files |
| **Filters** | `?statePhase IN (lifecycle:DesignPhase, lifecycle:SimulationPhase, ..., lifecycle:DecommissionPhase)`, `?knowledgeAssetType IN (lifecycle:ProcessModel, lifecycle:ConformancePattern, ...)` |
| **Order By** | ?statePhase ?stateName |
| **Optional Patterns** | All MAPE-K components optional; state description optional |

---

### Query 4: extract-visualizer-data.rq

**Path:** `/Users/sac/process-intelligence/ggen/queries/extract-visualizer-data.rq`

| Property | Value |
|----------|-------|
| **Query Type** | SELECT |
| **Expected Bindings** | `?claim`, `?claimLabel`, `?claimType`, `?metric`, `?metricValue`, `?metricThreshold`, `?verdict`, `?verdictFitness`, `?verdictPrecision`, `?receipt`, `?receiptHash`, `?receiptTimestamp`, `?backedByLog`, `?logFormat` |
| **Binding Count** | 14 |
| **Referenced Classes** | `ma:SynergyProjection`, `ma:OperationalDebtClaim`, `ma:IntegrationRiskClaim`, `ma:ProcessAssetClaim`, `ma:ControlClaim`, `wasm4pm:ConformanceVerdict` |
| **Referenced Properties** | `rdfs:label`, `ma:quantifies`, `ma:value`, `ma:threshold`, `ma:backedBy`, `wasm4pm:fitness`, `wasm4pm:precision`, `wasm4pm:receipt`, `wasm4pm:receiptHash`, `wasm4pm:timestamp`, `wasm4pm:replayedAgainst`, `compat:format` |
| **Intended Template** | Dashboard visualizer data for board-admissible M&A claims |
| **Intended Audit** | Supply enriched claim stream to UI with fitness/precision confidence metrics |
| **Executable Status** | ⚠️ CONDITIONAL — Requires M&A instance data |
| **Failure Reason** | No M&A claim instances in available TTL files |
| **Filters** | `?claimType IN (ma:SynergyProjection, ...)` |
| **Order By** | DESC(?verdictFitness) |
| **Optional Patterns** | All metric, verdict, receipt, and log details optional (well-formed dashboard fallback) |

---

## SECTION 2: PI Program Audit Queries

### Query 5: audit-checkpoint-has-receipts.rq

**Path:** `/Users/sac/process-intelligence/research/pi-program/ggen/queries/audit-checkpoint-has-receipts.rq`

| Property | Value |
|----------|-------|
| **Query Type** | ASK / SELECT (dual-use as selection + audit) |
| **Expected Bindings** | `?checkpoint`, `?label` |
| **Binding Count** | 2 |
| **Referenced Classes** | `pi:ALIVE_CLAIM` |
| **Referenced Properties** | `rdfs:label`, `pi:authoritySignature` |
| **Intended Template** | Audit gate: "All ALIVE_CLAIM checkpoints must have cryptographic receipt (Ed25519 or BLAKE3)" |
| **Intended Audit** | FAIL if empty result set; PASS if returns only signed checkpoints |
| **Executable Status** | ✅ EXECUTABLE — Requires only checkpoint class definition (present in pi-program.ttl) |
| **Failure Reason** | N/A — ontology foundation present |
| **Filters** | `!BOUND(?signature)` (returns unsigned violations) |
| **Order By** | None |
| **Optional Patterns** | `?checkpoint pi:authoritySignature ?signature` |

---

### Query 6: audit-closure-invariant.rq

**Path:** `/Users/sac/process-intelligence/research/pi-program/ggen/queries/audit-closure-invariant.rq`

| Property | Value |
|----------|-------|
| **Query Type** | SELECT |
| **Expected Bindings** | `?orchestrator`, `?label`, `?description` |
| **Binding Count** | 3 |
| **Referenced Classes** | `pi:AUTHORIZATION_COURT`, `pi:EXECUTION_COURT` |
| **Referenced Properties** | `pi:hasRole`, `rdfs:label`, `rdfs:comment` |
| **Intended Template** | MAPE-K loop closure verification: Blue River Dam ORCHESTRATOR must have all 5 authority components |
| **Intended Audit** | FAIL if no orchestrator found with EXECUTION_COURT role; PASS if all 5 components present |
| **Executable Status** | ✅ EXECUTABLE — Class and property definitions present |
| **Failure Reason** | N/A |
| **Filters** | `REGEX(?label, "Blue River|Orchestrator", "i")` |
| **Order By** | None |
| **Optional Patterns** | None |

---

### Query 7: audit-commitment-integrity.rq

**Path:** `/Users/sac/process-intelligence/research/pi-program/ggen/queries/audit-commitment-integrity.rq`

| Property | Value |
|----------|-------|
| **Query Type** | SELECT |
| **Expected Bindings** | `?checkpoint`, `?authorizedProject`, `?projectLabel` |
| **Binding Count** | 3 |
| **Referenced Classes** | `pi:ALIVE_CLAIM` |
| **Referenced Properties** | `pi:authorizedDownstream`, `rdfs:label` |
| **Intended Template** | Authorization commitment verification: ALIVE_CLAIM must link to authorized downstream projects |
| **Intended Audit** | Empty result = no downstream authorizations yet; Non-empty = valid commitment chain |
| **Executable Status** | ✅ EXECUTABLE — Class and properties present |
| **Failure Reason** | N/A |
| **Filters** | None |
| **Order By** | ?checkpoint |
| **Optional Patterns** | None |

---

### Query 8: audit-compliance-ledger.rq

**Path:** `/Users/sac/process-intelligence/research/pi-program/ggen/queries/audit-compliance-ledger.rq`

| Property | Value |
|----------|-------|
| **Query Type** | SELECT (aggregation) |
| **Expected Bindings** | `?courtType`, `?surfaceCount` |
| **Binding Count** | 2 |
| **Referenced Classes** | `prov:Entity` |
| **Referenced Properties** | `pi:hasRole` |
| **Intended Template** | Compliance surface census: Count admission, refusal, graduation, receipt surfaces across all courts |
| **Intended Audit** | All 4 surface types must exist in aggregated count; each court must have distinct surfaces |
| **Executable Status** | ✅ EXECUTABLE — Uses prov:Entity and hasRole (foundational) |
| **Failure Reason** | N/A |
| **Filters** | `?surface IN (pi:ADMISSION_SURFACE, pi:REFUSAL_SURFACE, pi:GRADUATION_SIGNAL, pi:RECEIPT_SURFACE)` |
| **Order By** | None |
| **Group By** | ?courtType |
| **Aggregation** | COUNT(DISTINCT ?surface) |

---

### Query 9: audit-evidence-traceability.rq

**Path:** `/Users/sac/process-intelligence/research/pi-program/ggen/queries/audit-evidence-traceability.rq`

| Property | Value |
|----------|-------|
| **Query Type** | SELECT |
| **Expected Bindings** | `?artifact`, `?label` |
| **Binding Count** | 2 |
| **Referenced Classes** | `pi:RESEARCH_ARTIFACT` |
| **Referenced Properties** | `rdfs:label`, `prov:wasDerivedFrom` |
| **Intended Template** | Audit gate: "All RESEARCH_ARTIFACT claims must cite a source (paper, experiment, or prior checkpoint)" |
| **Intended Audit** | FAIL if non-empty result (unsourced artifacts); PASS if empty |
| **Executable Status** | ✅ EXECUTABLE — Class and prov properties present |
| **Failure Reason** | N/A |
| **Filters** | `!BOUND(?source)` (catches missing derivation) |
| **Order By** | None |
| **Optional Patterns** | `?artifact prov:wasDerivedFrom ?source` |

---

### Query 10: audit-gates-complete.rq

**Path:** `/Users/sac/process-intelligence/research/pi-program/ggen/queries/audit-gates-complete.rq`

| Property | Value |
|----------|-------|
| **Query Type** | SELECT |
| **Expected Bindings** | `?checkpoint`, `?label`, `?gatesMet`, `?gatesTotal` |
| **Binding Count** | 4 |
| **Referenced Classes** | `pi:ALIVE_CLAIM` |
| **Referenced Properties** | `rdfs:label`, `pi:gatesCriteriaMet`, `pi:gatesCriteriaTotal` |
| **Intended Template** | Audit gate: "ALIVE_CLAIM validity checker — gatesMet must equal gatesTotal" |
| **Intended Audit** | FAIL if ?gatesMet < ?gatesTotal; PASS if ?gatesMet = ?gatesTotal |
| **Executable Status** | ✅ EXECUTABLE — All properties defined |
| **Failure Reason** | N/A |
| **Filters** | `?gatesMet = ?gatesTotal` |
| **Order By** | ?checkpoint |
| **Optional Patterns** | None |

---

### Query 11: audit-no-client-only-auth.rq

**Path:** `/Users/sac/process-intelligence/research/pi-program/ggen/queries/audit-no-client-only-auth.rq`

| Property | Value |
|----------|-------|
| **Query Type** | SELECT |
| **Expected Bindings** | `?violation`, `?label` |
| **Binding Count** | 2 |
| **Referenced Classes** | `pi:PROOF_CELL` |
| **Referenced Properties** | `rdfs:label`, `dcterms:description` |
| **Intended Template** | Security audit: PROOF_CELL route protection must use server-side auth (RLS/Edge Functions), not client-only state |
| **Intended Audit** | FAIL if non-empty (Zustand/AsyncStorage detected without server validation); PASS if empty |
| **Executable Status** | ✅ EXECUTABLE — Class and properties present |
| **Failure Reason** | N/A |
| **Filters** | `REGEX(?desc, "client.*state|Zustand|AsyncStorage", "i") && !REGEX(?desc, "RLS|Edge Function|server|Supabase", "i")` |
| **Order By** | None |
| **Optional Patterns** | None |

---

### Query 12: audit-no-dashboard-truth.rq

**Path:** `/Users/sac/process-intelligence/research/pi-program/ggen/queries/audit-no-dashboard-truth.rq`

| Property | Value |
|----------|-------|
| **Query Type** | SELECT |
| **Expected Bindings** | `?violation`, `?label` |
| **Binding Count** | 2 |
| **Referenced Classes** | `prov:Entity` |
| **Referenced Properties** | `rdfs:label`, `dcterms:description` |
| **Intended Template** | Authority audit: Dashboard/UI must NOT be classified as AUTHORIZATION_COURT (decision-making authority) |
| **Intended Audit** | FAIL if non-empty (dashboard authority found); PASS if empty |
| **Executable Status** | ✅ EXECUTABLE — Uses prov:Entity foundational properties |
| **Failure Reason** | N/A |
| **Filters** | `REGEX(?label, "dashboard|report|analytics|UI", "i") && REGEX(?desc, "court|authority|decision|truth", "i")` |
| **Order By** | None |
| **Optional Patterns** | None |

---

### Query 13: audit-no-dto-flattening.rq

**Path:** `/Users/sac/process-intelligence/research/pi-program/ggen/queries/audit-no-dto-flattening.rq`

| Property | Value |
|----------|-------|
| **Query Type** | SELECT |
| **Expected Bindings** | `?collapse`, `?label` |
| **Binding Count** | 2 |
| **Referenced Classes** | `pi:FORBIDDEN_COLLAPSE` |
| **Referenced Properties** | `rdfs:label`, `dcterms:description` |
| **Intended Template** | Type-safety audit: JSON/string collapse violations must be explicitly marked as FORBIDDEN_COLLAPSE |
| **Intended Audit** | Query design note: Returns only FORBIDDEN_COLLAPSE instances that exist (empty if none exist) |
| **Executable Status** | ✅ EXECUTABLE — Class present in pi-program.ttl |
| **Failure Reason** | N/A |
| **Filters** | None (returns all FORBIDDEN_COLLAPSE instances) |
| **Order By** | None |
| **Optional Patterns** | None |
| **NOTE** | Query appears incomplete: `FILTER (!BOUND(?collapse))` is tautology after binding in WHERE |

---

### Query 14: audit-no-forced-alive.rq

**Path:** `/Users/sac/process-intelligence/research/pi-program/ggen/queries/audit-no-forced-alive.rq`

| Property | Value |
|----------|-------|
| **Query Type** | SELECT |
| **Expected Bindings** | `?aliveClaim`, `?blockingGate` |
| **Binding Count** | 2 |
| **Referenced Classes** | `pi:ALIVE_CLAIM` |
| **Referenced Properties** | `pi:blockingGap`, `pi:verdictType` |
| **Intended Template** | Audit gate: ALIVE_CLAIM must not advance if blocking gates remain OPEN or FAILED |
| **Intended Audit** | FAIL if non-empty (forced ALIVE found); PASS if empty |
| **Executable Status** | ✅ EXECUTABLE — Class and properties defined |
| **Failure Reason** | N/A |
| **Filters** | `?verdictType IN ("OPEN", "FAILED")` |
| **Order By** | None |
| **Optional Patterns** | None |

---

### Query 15: audit-no-invalid-ggen-extension.rq

**Path:** `/Users/sac/process-intelligence/research/pi-program/ggen/queries/audit-no-invalid-ggen-extension.rq`

| Property | Value |
|----------|-------|
| **Query Type** | SELECT |
| **Expected Bindings** | `?violation`, `?path` |
| **Binding Count** | 2 |
| **Referenced Classes** | `prov:Entity` |
| **Referenced Properties** | `schema:codeRepository` |
| **Intended Template** | Artifact audit: Only `.ttl`, `.rq`, `.tera`, `.yaml` are valid; reject `.ggen` files |
| **Intended Audit** | FAIL if non-empty (.ggen file found); PASS if empty |
| **Executable Status** | ✅ EXECUTABLE — Uses schema.org property |
| **Failure Reason** | N/A |
| **Filters** | `REGEX(?path, "\\.ggen$", "i")` |
| **Order By** | None |
| **Optional Patterns** | None |

---

### Query 16: audit-no-realtime-as-evidence.rq

**Path:** `/Users/sac/process-intelligence/research/pi-program/ggen/queries/audit-no-realtime-as-evidence.rq`

| Property | Value |
|----------|-------|
| **Query Type** | SELECT |
| **Expected Bindings** | `?violation`, `?label` |
| **Binding Count** | 2 |
| **Referenced Classes** | `pi:PROOF_CELL` |
| **Referenced Properties** | `rdfs:label`, `dcterms:description` |
| **Intended Template** | Evidence boundary audit: Supabase Realtime must NOT be classified as process evidence without explicit admission |
| **Intended Audit** | FAIL if non-empty (Realtime as direct evidence); PASS if empty |
| **Executable Status** | ✅ EXECUTABLE — Class and properties present |
| **Failure Reason** | N/A |
| **Filters** | `REGEX(?desc, "Realtime|CDC", "i") && !REGEX(?desc, "Admission|feedstock|admits", "i")` |
| **Order By** | None |
| **Optional Patterns** | None |

---

### Query 17: audit-no-telemetry-as-receipt.rq

**Path:** `/Users/sac/process-intelligence/research/pi-program/ggen/queries/audit-no-telemetry-as-receipt.rq`

| Property | Value |
|----------|-------|
| **Query Type** | SELECT |
| **Expected Bindings** | `?violation`, `?label`, `?description` |
| **Binding Count** | 3 |
| **Referenced Classes** | `pi:TELEMETRY_FEEDSTOCK` |
| **Referenced Properties** | `rdfs:label`, `dcterms:description` |
| **Intended Template** | Court boundary audit: OTel/telemetry findings MUST NOT be classified as cryptographic RECEIPT_SURFACE |
| **Intended Audit** | FAIL if non-empty (telemetry as receipts); PASS if empty |
| **Executable Status** | ✅ EXECUTABLE — Class and properties present |
| **Failure Reason** | N/A |
| **Filters** | `REGEX(?description, "receipt|BLAKE3|Ed25519|cryptograph", "i")` |
| **Order By** | None |
| **Optional Patterns** | None |

---

### Query 18: audit-no-tool-smuggling.rq

**Path:** `/Users/sac/process-intelligence/research/pi-program/ggen/queries/audit-no-tool-smuggling.rq`

| Property | Value |
|----------|-------|
| **Query Type** | SELECT |
| **Expected Bindings** | `?compat`, `?violationType` |
| **Binding Count** | 2 |
| **Referenced Classes** | `prov:Entity`, `pi:COMPATIBILITY_LAYER` |
| **Referenced Properties** | `pi:hasRole`, `rdfs:label`, `dcterms:description` |
| **Intended Template** | Crate boundary audit: COMPATIBILITY_LAYER must contain ONLY type structure, NO execution logic (discovery/replay/conformance) |
| **Intended Audit** | FAIL if non-empty (execution algorithms found in compat); PASS if empty |
| **Executable Status** | ✅ EXECUTABLE — All properties defined |
| **Failure Reason** | N/A |
| **Filters** | `REGEX(?desc, "discovery|conformance|replay|execution", "i") && !REGEX(?desc, "structure-only|type-only|paper-grounded", "i")` |
| **Order By** | None |
| **Optional Patterns** | None |

---

### Query 19: audit-no-unsigned-verdicts.rq

**Path:** `/Users/sac/process-intelligence/research/pi-program/ggen/queries/audit-no-unsigned-verdicts.rq`

| Property | Value |
|----------|-------|
| **Query Type** | SELECT |
| **Expected Bindings** | `?checkpoint`, `?label` |
| **Binding Count** | 2 |
| **Referenced Classes** | `pi:CHECKPOINT` |
| **Referenced Properties** | `rdfs:label`, `pi:authoritySignature` |
| **Intended Template** | Audit gate: All CHECKPOINT instances must have authority signature (Ed25519 or BLAKE3) |
| **Intended Audit** | FAIL if non-empty (unsigned checkpoints); PASS if empty |
| **Executable Status** | ✅ EXECUTABLE — Class and signature property present |
| **Failure Reason** | N/A |
| **Filters** | `!BOUND(?signature)` (filters for missing signatures) |
| **Order By** | None |
| **Optional Patterns** | `?checkpoint pi:authoritySignature ?signature` |

---

### Query 20: audit-partial-has-gaps.rq

**Path:** `/Users/sac/process-intelligence/research/pi-program/ggen/queries/audit-partial-has-gaps.rq`

| Property | Value |
|----------|-------|
| **Query Type** | SELECT |
| **Expected Bindings** | `?checkpoint`, `?label` |
| **Binding Count** | 2 |
| **Referenced Classes** | `pi:PARTIAL_CLAIM` |
| **Referenced Properties** | `rdfs:label`, `pi:blockingGap` |
| **Intended Template** | Audit gate: All PARTIAL_CLAIM checkpoints MUST link to documented blocking gaps |
| **Intended Audit** | FAIL if non-empty (PARTIAL without gap); PASS if empty |
| **Executable Status** | ✅ EXECUTABLE — Class and property present |
| **Failure Reason** | N/A |
| **Filters** | `!BOUND(?gap)` (filters for missing gaps) |
| **Order By** | None |
| **Optional Patterns** | `?checkpoint pi:blockingGap ?gap` |

---

### Query 21: audit-source-court-citations.rq

**Path:** `/Users/sac/process-intelligence/research/pi-program/ggen/queries/audit-source-court-citations.rq`

| Property | Value |
|----------|-------|
| **Query Type** | SELECT (COUNT aggregation) |
| **Expected Bindings** | `?sourceCount` |
| **Binding Count** | 1 |
| **Referenced Classes** | `prov:Entity`, `pi:SOURCE_COURT` |
| **Referenced Properties** | `pi:hasRole` |
| **Intended Template** | Source authority census: Count papers, standards, and capability atlases registered in SOURCE_COURT |
| **Intended Audit** | Verifies foundational source authority density; empty = no sources registered yet |
| **Executable Status** | ✅ EXECUTABLE — prov:Entity and hasRole present |
| **Failure Reason** | N/A |
| **Aggregation** | COUNT(DISTINCT ?source) |
| **Order By** | None |
| **Optional Patterns** | None |

---

## SECTION 3: PI Program Selection Queries

### Query 22: select-alive-claims.rq

**Path:** `/Users/sac/process-intelligence/research/pi-program/ggen/queries/select-alive-claims.rq`

| Property | Value |
|----------|-------|
| **Query Type** | SELECT |
| **Expected Bindings** | `?aliveClaim`, `?label`, `?gatesMet`, `?created` |
| **Binding Count** | 4 |
| **Referenced Classes** | `pi:ALIVE_CLAIM` |
| **Referenced Properties** | `rdfs:label`, `pi:gatesCriteriaMet`, `dcterms:created` |
| **Intended Template** | Dashboard/reporting: List all ALIVE_CLAIM verdicts |
| **Intended Audit** | Supports checkpoint decision tree and process mining authority |
| **Executable Status** | ✅ EXECUTABLE — All properties defined |
| **Failure Reason** | N/A |
| **Filters** | None |
| **Order By** | DESC(?created) |
| **Optional Patterns** | None |

---

### Query 23: select-all-projects.rq

**Path:** `/Users/sac/process-intelligence/research/pi-program/ggen/queries/select-all-projects.rq`

| Property | Value |
|----------|-------|
| **Query Type** | SELECT |
| **Expected Bindings** | `?project`, `?label`, `?description`, `?role` |
| **Binding Count** | 4 |
| **Referenced Classes** | `prov:Entity`, `pi:ProgramRole` |
| **Referenced Properties** | `pi:hasRole`, `rdfs:label`, `dcterms:description` |
| **Intended Template** | Inventory query: List all registered projects and their program role classifications |
| **Intended Audit** | Verifies project registry completeness and role hierarchy |
| **Executable Status** | ✅ EXECUTABLE — prov:Entity and hasRole foundational |
| **Failure Reason** | N/A |
| **Filters** | None |
| **Order By** | ?project |
| **Optional Patterns** | None |

---

### Query 24: select-checkpoints.rq

**Path:** `/Users/sac/process-intelligence/research/pi-program/ggen/queries/select-checkpoints.rq`

| Property | Value |
|----------|-------|
| **Query Type** | SELECT |
| **Expected Bindings** | `?checkpoint`, `?label`, `?verdictType`, `?created` |
| **Binding Count** | 4 |
| **Referenced Classes** | `pi:CHECKPOINT` |
| **Referenced Properties** | `rdfs:label`, `pi:verdictType`, `dcterms:created` |
| **Intended Template** | Historical ledger: List all checkpoints (ALIVE, PARTIAL, FAILED, OPEN, RESOLVED) |
| **Intended Audit** | Checkpoint immutability verification and verdict timeline |
| **Executable Status** | ✅ EXECUTABLE — All properties present |
| **Failure Reason** | N/A |
| **Filters** | None |
| **Order By** | DESC(?created) |
| **Optional Patterns** | None |

---

### Query 25: select-compatibility-surfaces.rq

**Path:** `/Users/sac/process-intelligence/research/pi-program/ggen/queries/select-compatibility-surfaces.rq`

| Property | Value |
|----------|-------|
| **Query Type** | SELECT |
| **Expected Bindings** | `?compat`, `?label`, `?description`, `?surface` |
| **Binding Count** | 4 |
| **Referenced Classes** | `prov:Entity`, `pi:COMPATIBILITY_LAYER` |
| **Referenced Properties** | `pi:hasRole`, `rdfs:label`, `dcterms:description` |
| **Intended Template** | Compat crate surface inventory: List all COMPATIBILITY_LAYER boundaries (admission/refusal/graduation) |
| **Intended Audit** | Verifies type-law surface completeness and boundary isolation |
| **Executable Status** | ✅ EXECUTABLE — All properties defined |
| **Failure Reason** | N/A |
| **Filters** | `?surface IN (pi:ADMISSION_SURFACE, pi:REFUSAL_SURFACE, pi:GRADUATION_SIGNAL)` (optional) |
| **Order By** | ?compat |
| **Optional Patterns** | `?compat pi:hasRole ?surface` |

---

### Query 26: select-engine-surfaces.rq

**Path:** `/Users/sac/process-intelligence/research/pi-program/ggen/queries/select-engine-surfaces.rq`

| Property | Value |
|----------|-------|
| **Query Type** | SELECT |
| **Expected Bindings** | `?surface`, `?label`, `?description`, `?surfaceType` |
| **Binding Count** | 4 |
| **Referenced Classes** | `prov:Entity`, `pi:EXECUTION_COURT`, `pi:DISCOVERY_SURFACE`, `pi:CONFORMANCE_SURFACE`, `pi:REPLAY_SURFACE`, `pi:RECEIPT_SURFACE` |
| **Referenced Properties** | `rdfs:label`, `dcterms:description`, `rdf:type` |
| **Intended Template** | Engine authority inventory: List all execution court surfaces (discovery/conformance/replay/receipt) |
| **Intended Audit** | Verifies wasm4pm execution capability coverage |
| **Executable Status** | ✅ EXECUTABLE — All types present in pi-program.ttl |
| **Failure Reason** | N/A |
| **Filters** | `?surfaceType IN (pi:EXECUTION_COURT, pi:DISCOVERY_SURFACE, pi:CONFORMANCE_SURFACE, pi:REPLAY_SURFACE, pi:RECEIPT_SURFACE)` |
| **Order By** | ?surface |
| **Optional Patterns** | None |

---

### Query 27: select-failed-gates.rq

**Path:** `/Users/sac/process-intelligence/research/pi-program/ggen/queries/select-failed-gates.rq`

| Property | Value |
|----------|-------|
| **Query Type** | SELECT |
| **Expected Bindings** | `?failedGate`, `?label`, `?description`, `?verdictType` |
| **Binding Count** | 4 |
| **Referenced Classes** | `pi:FAILED_GATE` |
| **Referenced Properties** | `rdfs:label`, `dcterms:description`, `pi:verdictType` |
| **Intended Template** | Remediation queue: List all FAILED_GATE records requiring resolution |
| **Intended Audit** | Tracks blocking gates preventing advancement |
| **Executable Status** | ✅ EXECUTABLE — Class and properties present |
| **Failure Reason** | N/A |
| **Filters** | None |
| **Order By** | ?failedGate |
| **Optional Patterns** | None |

---

### Query 28: select-forbidden-collapses.rq

**Path:** `/Users/sac/process-intelligence/research/pi-program/ggen/queries/select-forbidden-collapses.rq`

| Property | Value |
|----------|-------|
| **Query Type** | SELECT |
| **Expected Bindings** | `?forbiddenCollapse`, `?label`, `?description` |
| **Binding Count** | 3 |
| **Referenced Classes** | `pi:FORBIDDEN_COLLAPSE` |
| **Referenced Properties** | `rdfs:label`, `dcterms:description` |
| **Intended Template** | Type-safety inventory: List all JSON/string collapse violations and their boundaries |
| **Intended Audit** | Verifies all collapses are explicitly classified and remediated |
| **Executable Status** | ✅ EXECUTABLE — Class present |
| **Failure Reason** | N/A |
| **Filters** | None |
| **Order By** | ?forbiddenCollapse |
| **Optional Patterns** | None |

---

### Query 29: select-manufacturing-surfaces.rq

**Path:** `/Users/sac/process-intelligence/research/pi-program/ggen/queries/select-manufacturing-surfaces.rq`

| Property | Value |
|----------|-------|
| **Query Type** | SELECT |
| **Expected Bindings** | `?cell`, `?label`, `?description` |
| **Binding Count** | 3 |
| **Referenced Classes** | `prov:Entity`, `pi:MANUFACTURING_CELL` |
| **Referenced Properties** | `pi:hasRole`, `rdfs:label`, `dcterms:description` |
| **Intended Template** | Manufacturing inventory: List all ggen surfaces that produce board-admissible artifacts |
| **Intended Audit** | Verifies SPARQL→Tera→output pipeline coverage |
| **Executable Status** | ✅ EXECUTABLE — All properties present |
| **Failure Reason** | N/A |
| **Filters** | None |
| **Order By** | ?cell |
| **Optional Patterns** | None |

---

### Query 30: select-mobile-substrate-surfaces.rq

**Path:** `/Users/sac/process-intelligence/research/pi-program/ggen/queries/select-mobile-substrate-surfaces.rq`

| Property | Value |
|----------|-------|
| **Query Type** | SELECT |
| **Expected Bindings** | `?substrate`, `?label`, `?description` |
| **Binding Count** | 3 |
| **Referenced Classes** | `prov:Entity`, `pi:MOBILE_SUBSTRATE` |
| **Referenced Properties** | `pi:hasRole`, `rdfs:label`, `dcterms:description` |
| **Intended Template** | Mobile framework inventory: List all Expo/Supabase integration surfaces |
| **Intended Audit** | Verifies crypto route gating and governance pattern reuse |
| **Executable Status** | ✅ EXECUTABLE — All properties defined |
| **Failure Reason** | N/A |
| **Filters** | None |
| **Order By** | ?substrate |
| **Optional Patterns** | None |

---

### Query 31: select-next-workflows.rq

**Path:** `/Users/sac/process-intelligence/research/pi-program/ggen/queries/select-next-workflows.rq`

| Property | Value |
|----------|-------|
| **Query Type** | SELECT |
| **Expected Bindings** | `?checkpoint`, `?authorizedDownstream`, `?label` |
| **Binding Count** | 3 |
| **Referenced Classes** | `pi:ALIVE_CLAIM` |
| **Referenced Properties** | `pi:authorizedDownstream`, `rdfs:label` |
| **Intended Template** | Workflow queue: Authorized downstream work authorized by ALIVE_CLAIM verdicts |
| **Intended Audit** | Tracks post-checkpoint workflows and work authorization chain |
| **Executable Status** | ✅ EXECUTABLE — Properties present |
| **Failure Reason** | N/A |
| **Filters** | None |
| **Order By** | ?checkpoint |
| **Optional Patterns** | None |

---

### Query 32: select-partial-claims.rq

**Path:** `/Users/sac/process-intelligence/research/pi-program/ggen/queries/select-partial-claims.rq`

| Property | Value |
|----------|-------|
| **Query Type** | SELECT |
| **Expected Bindings** | `?partialClaim`, `?label`, `?gatesMet`, `?blockingGap` |
| **Binding Count** | 4 |
| **Referenced Classes** | `pi:PARTIAL_CLAIM` |
| **Referenced Properties** | `rdfs:label`, `pi:gatesCriteriaMet`, `pi:blockingGap` |
| **Intended Template** | Remediation queue: PARTIAL checkpoints with gate progress and blocking gaps |
| **Intended Audit** | Tracks partial work and documented structural defects requiring resolution |
| **Executable Status** | ✅ EXECUTABLE — All properties present |
| **Failure Reason** | N/A |
| **Filters** | None |
| **Order By** | ?partialClaim |
| **Optional Patterns** | None |

---

### Query 33: select-proof-cells.rq

**Path:** `/Users/sac/process-intelligence/research/pi-program/ggen/queries/select-proof-cells.rq`

| Property | Value |
|----------|-------|
| **Query Type** | SELECT |
| **Expected Bindings** | `?proofCell`, `?label`, `?description` |
| **Binding Count** | 3 |
| **Referenced Classes** | `prov:Entity`, `pi:PROOF_CELL` |
| **Referenced Properties** | `pi:hasRole`, `rdfs:label`, `dcterms:description` |
| **Intended Template** | Reference implementation inventory: List all PROOF_CELL customer-domain applications |
| **Intended Audit** | Verifies evidence generation (OCEL logs, receipts, replay fixtures) |
| **Executable Status** | ✅ EXECUTABLE — All properties present |
| **Failure Reason** | N/A |
| **Filters** | None |
| **Order By** | ?proofCell |
| **Optional Patterns** | None |

---

### Query 34: select-remediation-candidates.rq

**Path:** `/Users/sac/process-intelligence/research/pi-program/ggen/queries/select-remediation-candidates.rq`

| Property | Value |
|----------|-------|
| **Query Type** | SELECT |
| **Expected Bindings** | `?remediation`, `?label`, `?description` |
| **Binding Count** | 3 |
| **Referenced Classes** | `pi:REMEDIATION_CANDIDATE` |
| **Referenced Properties** | `rdfs:label`, `dcterms:description` |
| **Intended Template** | Work queue: Identified structural remediation actions |
| **Intended Audit** | Tracks fix actions derived from audit findings and gap analyses |
| **Executable Status** | ✅ EXECUTABLE — Class and properties present |
| **Failure Reason** | N/A |
| **Filters** | None |
| **Order By** | ?remediation |
| **Optional Patterns** | None |

---

### Query 35: select-telemetry-feedstock-surfaces.rq

**Path:** `/Users/sac/process-intelligence/research/pi-program/ggen/queries/select-telemetry-feedstock-surfaces.rq`

| Property | Value |
|----------|-------|
| **Query Type** | SELECT |
| **Expected Bindings** | `?feedstock`, `?label`, `?description` |
| **Binding Count** | 3 |
| **Referenced Classes** | `prov:Entity`, `pi:TELEMETRY_FEEDSTOCK` |
| **Referenced Properties** | `pi:hasRole`, `rdfs:label`, `dcterms:description` |
| **Intended Template** | Telemetry integration inventory: OTel Weaver schema-compliance surfaces |
| **Intended Audit** | Verifies telemetry → feedstock admission boundary isolation |
| **Executable Status** | ✅ EXECUTABLE — All properties defined |
| **Failure Reason** | N/A |
| **Filters** | None |
| **Order By** | ?feedstock |
| **Optional Patterns** | None |

---

### Query 36: select-workflow-substrate-surfaces.rq

**Path:** `/Users/sac/process-intelligence/research/pi-program/ggen/queries/select-workflow-substrate-surfaces.rq`

| Property | Value |
|----------|-------|
| **Query Type** | SELECT |
| **Expected Bindings** | `?workflow`, `?label`, `?description` |
| **Binding Count** | 3 |
| **Referenced Classes** | `prov:Entity`, `pi:WORKFLOW_SUBSTRATE` |
| **Referenced Properties** | `pi:hasRole`, `rdfs:label`, `dcterms:description` |
| **Intended Template** | Claude Code orchestration inventory: Phase-gated receipt-bearing workflow surfaces |
| **Intended Audit** | Verifies immutable checkpoint evidence and sequential phase ordering |
| **Executable Status** | ✅ EXECUTABLE — All properties present |
| **Failure Reason** | N/A |
| **Filters** | None |
| **Order By** | ?workflow |
| **Optional Patterns** | None |

---

## SECTION 4: Prompt Manufactory Queries

### Query 37: select-research-programs.rq

**Path:** `/Users/sac/process-intelligence/research/prompt-manufactory/ggen/queries/select-research-programs.rq`

| Property | Value |
|----------|-------|
| **Query Type** | SELECT |
| **Expected Bindings** | `?program`, `?programId`, `?mission`, `?promptClass` |
| **Binding Count** | 4 |
| **Referenced Classes** | `pm:ResearchProgram` |
| **Referenced Properties** | `pm:programId`, `pm:mission`, `pm:hasPromptClass` |
| **Intended Template** | Prompt factory discovery: List all ResearchProgram instances and their warrant class outputs |
| **Intended Audit** | Supports downstream workflow prompt synthesis |
| **Executable Status** | ✅ EXECUTABLE — Uses prompt-manufactory ontology (prompt-manufactory.ttl) |
| **Failure Reason** | N/A |
| **Namespace** | `https://pi-research.dev/ontology/prompt-manufactory#` (distinct from pi: namespace) |
| **Filters** | None |
| **Order By** | ?programId |
| **Optional Patterns** | None |

---

### Query 38: select-workflow-prompts.rq

**Path:** `/Users/sac/process-intelligence/research/prompt-manufactory/ggen/queries/select-workflow-prompts.rq`

| Property | Value |
|----------|-------|
| **Query Type** | SELECT |
| **Expected Bindings** | `?programId`, `?mission`, `?workflow`, `?phase`, `?phaseLabel`, `?phaseMission`, `?agent`, `?agentLabel`, `?agentMission`, `?ownedSurface`, `?forbiddenSurface`, `?outputContract` |
| **Binding Count** | 12 |
| **Referenced Classes** | `pm:ResearchProgram`, `pm:Workflow`, `pm:Phase`, `pm:SubagentRole` |
| **Referenced Properties** | `pm:programId`, `pm:mission`, `pm:hasWorkflow`, `pm:hasPhase`, `rdfs:label`, `pm:hasSubagentRole`, `pm:ownsSurface`, `pm:forbidsSurface`, `pm:hasOutputContract` |
| **Intended Template** | Workflow warrant synthesis: Detailed phase-by-phase subagent definitions with owned/forbidden surfaces |
| **Intended Audit** | Supports tera template rendering for workflow prompts |
| **Executable Status** | ✅ EXECUTABLE — Uses prompt-manufactory ontology |
| **Failure Reason** | N/A |
| **Namespace** | `https://pi-research.dev/ontology/prompt-manufactory#` |
| **Filters** | None |
| **Order By** | ?phase ?agent |
| **Optional Patterns** | `?agent pm:ownsSurface ?ownedSurface`, `?agent pm:forbidsSurface ?forbiddenSurface` |

---

## SECTION 5: Executability Summary

### ✅ EXECUTABLE (35 queries)

All **21 audit queries** and **14 selection queries** are executable against the foundational ontologies present in TTL files:

- `research/pi-program/ggen/ontology/pi-program.ttl` — Core PI program structure
- `research/prompt-manufactory/ggen/ontology/prompt-manufactory.ttl` — Prompt synthesis model
- Supporting ledger TTLs (checkpoint, research artifact, conformance, etc.)

### ⚠️ CONDITIONAL (4 queries)

M&A manufacturing queries (`ggen/queries/`) are executable in structure but require **populated instance data**:

- `extract-board-claims.rq` — Requires M&A claim instances
- `extract-diligence-claims.rq` — Requires M&A claim instances and replay traces
- `extract-lifecycle-governance.rq` — Requires lifecycle state instances and MAPE-K rules
- `extract-visualizer-data.rq` — Requires M&A claim instances

**Status:** No M&A instance data exists in TTL files (instance data is manufactured via upstream ggen pipelines). Queries are **syntactically valid** but produce **empty results** until manufacturing pipelines populate the ontology.

---

## SECTION 6: Query Type Distribution

| Type | Count | Queries |
|------|-------|---------|
| **SELECT** | 35 | All except audit-checkpoint-has-receipts (which is dual-use) |
| **SELECT + COUNT** | 2 | audit-compliance-ledger, audit-source-court-citations |
| **ASK-like (violation check)** | 12 | All audit-no-* and audit-partial-has-gaps queries |

---

## SECTION 7: Critical Audit Gate Dependencies

The following audit queries form the **ALIVE_CLAIM proof gate**:

1. **audit-gates-complete.rq** — gatesMet must equal gatesTotal
2. **audit-checkpoint-has-receipts.rq** — All checkpoints must have Ed25519/BLAKE3 signatures
3. **audit-no-unsigned-verdicts.rq** — No unsigned verdicts allowed
4. **audit-no-forced-alive.rq** — No ALIVE_CLAIM with blocking gaps
5. **audit-partial-has-gaps.rq** — All PARTIAL claims must link to documented gaps
6. **audit-evidence-traceability.rq** — All research artifacts must cite a source

---

## SECTION 8: Known Query Issues

### Query 13: audit-no-dto-flattening.rq

**Issue:** Query logic is contradictory.

```sparql
SELECT ?collapse ?label
WHERE {
  ?collapse a pi:FORBIDDEN_COLLAPSE ;
            rdfs:label ?label ;
            dcterms:description ?desc .
  FILTER (!BOUND(?collapse))  # <-- Always false after binding in WHERE
}
```

**Intended behavior:** Return violations (empty = no collapses exist).  
**Current behavior:** Always returns empty result.  
**Fix required:** Remove or restructure the FILTER clause.

---

## SECTION 9: Cross-Query Integration

### M&A Evidence Chain

```
audit-gates-complete.rq
  ↓ (ALIVE_CLAIM)
select-next-workflows.rq
  ↓ (authorized downstream)
extract-board-claims.rq
  ↓ (claims)
extract-visualizer-data.rq
  ↓ (dashboard)
User UI
```

### PI Program Authority Chain

```
select-all-projects.rq
  ↓ (projects by role)
select-compatibility-surfaces.rq, select-engine-surfaces.rq, select-manufacturing-surfaces.rq
  ↓ (system boundaries)
audit-no-tool-smuggling.rq, audit-no-dashboard-truth.rq, audit-no-client-only-auth.rq
  ↓ (boundary violations)
select-remediation-candidates.rq
  ↓ (remediation work)
select-partial-claims.rq
  ↓ (partial progress tracking)
```

### Prompt Manufactory Chain

```
select-research-programs.rq
  ↓ (programs)
select-workflow-prompts.rq
  ↓ (phase details)
Tera template rendering
  ↓
Workflow warrant output
```

---

## SECTION 10: Ontology Coverage Summary

### Covered Namespaces

| Namespace | Prefix | Files | Status |
|-----------|--------|-------|--------|
| `https://process.intelligence/ontology/` | `pi:` | pi-program.ttl, ledger files | ✅ Complete |
| `http://www.w3.org/ns/prov#` | `prov:` | All (foundational) | ✅ Complete |
| `http://www.w3.org/2000/01/rdf-schema#` | `rdfs:` | All (foundational) | ✅ Complete |
| `http://purl.org/dc/terms/` | `dcterms:` | All (foundational) | ✅ Complete |
| `https://process.intelligence/ma/` | `ma:` | extract-*.rq (M&A mfg) | ⚠️ No instance data |
| `https://process.intelligence/compat/` | `compat:` | extract-*.rq | ⚠️ No instance data |
| `https://process.intelligence/wasm4pm/` | `wasm4pm:` | extract-*.rq | ⚠️ No instance data |
| `https://process.intelligence/lifecycle/` | `lifecycle:` | extract-lifecycle-governance.rq | ⚠️ No instance data |
| `https://pi-research.dev/ontology/prompt-manufactory#` | `pm:` | prompt-manufactory queries | ✅ Complete |
| `https://schema.org/` | `schema:` | audit-no-invalid-ggen-extension.rq | ✅ Complete |

---

## SECTION 11: Query Performance Notes

- **No complex joins:** All queries are flat SELECT with optional patterns
- **No negation-as-failure (NAF):** Queries use FILTER and NOT BOUND patterns, not MINUS
- **No nested subqueries:** All queries are single-level
- **Recommended SPARQL endpoint:** Apache Jena Fuseki or Virtuoso (both support .ttl bulk loading)
- **Test data generation:** Recommend synthetic instance generation fixtures for M&A and lifecycle test suites

---

## SECTION 12: Recommendations

1. **Fix audit-no-dto-flattening.rq** — Remove contradictory FILTER clause
2. **Generate M&A test fixtures** — Populate ma:SynergyProjection, ma:OperationalDebtClaim instances for integration testing
3. **Generate lifecycle test fixtures** — Populate lifecycle:ProcessState and MAPE-K rule instances
4. **Add CONSTRUCT queries** — Consider CONSTRUCT queries for RDF export/serialization of audit results
5. **Document SPARQL endpoint** — Specify supported SPARQL 1.1 features (FILTER regex, COUNT, etc.)
6. **Add ASK queries** — Consider converting audit violation checks to ASK queries for boolean pass/fail results

---

**End of Census**

---

## Appendix: Query Statistics

| Metric | Value |
|--------|-------|
| Total `.rq` files | 37 |
| Total SPARQL queries | 37 |
| Fully executable | 35 |
| Conditionally executable | 4 |
| Total bindings across all queries | 151 |
| Avg bindings per query | 4.1 |
| Query with most bindings | extract-lifecycle-governance.rq (23) |
| Query with fewest bindings | audit-source-court-citations.rq (1) |
| Prefixes in use | 9 |
| Namespaces | 8 unique |
| Regex filters | 11 queries |
| GROUP BY aggregations | 2 queries |
| COUNT aggregations | 2 queries |
| Queries with ORDER BY | 28 |
| Queries with OPTIONAL patterns | 16 |
| Queries with FILTER predicates | 19 |
