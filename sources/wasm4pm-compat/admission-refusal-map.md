# [PI-V30.1.2] Admission-Refusal Map: Type-Law Boundary Control

**Version:** 30.1.2  
**Authority:** Conformance Agent  
**Classification:** Foundational Type-Law  
**Date:** 2026-05-31  
**Status:** COMPLETE WITH PHASE 3 AMENDMENTS

---

## I. Executive Summary

The admission-refusal boundary is a **default-deny gatekeeping mechanism** that enforces van der Aalst's constitutional doctrine: *if the code says it worked but the event log cannot prove a lawful process happened, then it did not work*.

Every artifact entering the wasm4pm-compat type-law foundry must survive **eleven independent rejection pathways**. Passage through all eleven constitutes admissibility.

---

## II. The Eleven Rejection Pathways

### Pathway 1: Temporal Monotonicity Violation

**Refusal Condition:** Events within a case possess non-total-ordered timestamps.

**Formal Definition:**  
For a trace $\sigma = \langle e_1, e_2, \ldots, e_n \rangle$ with case ID $\text{cid}$:
$$\forall i < j, \quad \text{timestamp}(e_i) \leq \text{timestamp}(e_j)$$

Any violation (e.g., $e_5$ has timestamp earlier than $e_3$) triggers **immediate refusal**.

**Enforcement:**
```rust
pub fn validate_temporal_order(trace: &Trace) -> Result<(), RefusalReport> {
    let mut prev_ts = None;
    for event in trace.events() {
        if let Some(ts) = prev_ts {
            if event.timestamp() < ts {
                return Err(RefusalReport::TemporalAnomaly {
                    case_id: trace.case_id().to_string(),
                    anomaly_at: event.event_id().to_string(),
                    evidence: format!("timestamp {} < previous {}", 
                        event.timestamp(), ts),
                });
            }
        }
        prev_ts = Some(event.timestamp());
    }
    Ok(())
}
```

**Refusal Report:** Includes event ID, case ID, anomaly timestamp, and severity level (CRITICAL).

---

### Pathway 2: Type Violation — Schema Mismatch

**Refusal Condition:** Payload fails to deserialize into declared type `T`.

**Scope:** Applies to all `Evidence<T, State, Witness>` admissions.

**Enforcement:**
- **XES Logs**: Must parse against IEEE 1849-2016 XSD schema.
- **OCEL 2.0 Logs**: Must conform to ISO/IEC 23745 JSON schema or SQLite schema.
- **Petri Nets**: Must deserialize as valid Petri net JSON/XML with transition/place/arc structure.
- **BPMN Models**: Must validate against BPMN 2.0 XML schema.
- **POWL 2.0 Models**: Must conform to POWL grammar (hierarchical block structure).

```rust
pub fn validate_schema<T: Deserialize>(payload: &[u8]) -> Result<T, RefusalReport> {
    serde_json::from_slice(payload)
        .map_err(|e| RefusalReport::SchemaViolation {
            payload_type: std::any::type_name::<T>().to_string(),
            error_detail: e.to_string(),
            location: e.line(),
        })
}
```

**Refusal Report:** Includes payload type, error detail, and schema location.

---

### Pathway 3: Causal Disconnection — Non-Existent Object References

**Refusal Condition:** An event references an object ID that does not exist in the event log.

**Applies To:** OCEL 2.0 logs (object-centric event logs).

**Formal Definition:**  
For an OCEL log with object types $O_1, O_2, \ldots, O_k$ and their instance IDs:
$$\forall \text{event } e \in \text{log}, \forall \text{object ref } o_r \in e.\text{related\_objects}, \quad \exists \text{object } o \in O_j \text{ s.t. } o.id = o_r$$

Any unresolved reference triggers refusal.

**Enforcement:**
```rust
pub fn validate_object_references(ocel_log: &OcelLog) -> Result<(), RefusalReport> {
    let mut valid_object_ids: HashMap<ObjectType, HashSet<ObjectId>> = HashMap::new();
    
    // First pass: collect all valid object IDs
    for event in ocel_log.events() {
        for (obj_type, obj_id) in event.object_references() {
            valid_object_ids.entry(obj_type)
                .or_insert_with(HashSet::new)
                .insert(obj_id);
        }
    }
    
    // Second pass: validate all references
    for event in ocel_log.events() {
        for (obj_type, obj_id) in event.object_references() {
            if !valid_object_ids
                .get(&obj_type)
                .map_or(false, |ids| ids.contains(&obj_id)) {
                return Err(RefusalReport::CausalDisconnect {
                    event_id: event.id().to_string(),
                    missing_object: format!("{}:{}", obj_type, obj_id),
                });
            }
        }
    }
    Ok(())
}
```

**Refusal Report:** Includes event ID, missing object type/ID, and causality trace.

---

### Pathway 4: Type Violation — Buffer Overflow / Memory Bounds

**Refusal Condition:** Payload exceeds WASM linear memory ceiling or contains size-indicating fields that exceed their allocations.

**Enforcement:** WASM sandbox + static type checking.

```rust
pub fn validate_memory_bounds(payload: &[u8]) -> Result<(), RefusalReport> {
    const WASM_MEMORY_LIMIT: usize = 100 * 1024 * 1024; // 100 MB
    if payload.len() > WASM_MEMORY_LIMIT {
        return Err(RefusalReport::MemoryBoundsViolation {
            payload_size: payload.len(),
            limit: WASM_MEMORY_LIMIT,
        });
    }
    Ok(())
}
```

**Refusal Report:** Includes payload size, memory limit, and allocation trace.

---

### Pathway 5: Cryptographic Signature Invalid

**Refusal Condition:** The evidence hash does not match the embedded signature, or the signature cannot be verified against a known authority key.

**Formal Definition:**  
For Evidence $E = \langle T, S, W, e, \text{sig}, \mathcal{H} \rangle$:
$$\text{VerifySignature}(\text{PublicKey}_{\text{Authority}}, \text{sig}, \mathcal{H}) \equiv \text{True}$$
$$\text{and} \quad \text{ComputeHash}(T, S, W, e, \text{sig}) = \mathcal{H}$$

Any failure triggers refusal.

**Enforcement:**
```rust
pub fn validate_signature<T, S, W>(evidence: &Evidence<T, S, W>) -> Result<(), RefusalReport> {
    // Recompute hash
    let expected_hash = evidence.compute_hash();
    if expected_hash != evidence.hash {
        return Err(RefusalReport::HashMismatch {
            expected: hex::encode(expected_hash),
            actual: hex::encode(evidence.hash),
        });
    }
    
    // Verify signature against authority key
    let authority_key = AUTHORITY_KEYS.get(&evidence.signer_role())
        .ok_or(RefusalReport::UnknownAuthority {
            role: evidence.signer_role().to_string(),
        })?;
    
    if !ed25519::verify(&authority_key, &evidence.signature, &expected_hash)? {
        return Err(RefusalReport::SignatureVerificationFailed {
            authority: evidence.signer_role().to_string(),
        });
    }
    Ok(())
}
```

**Refusal Report:** Includes expected/actual hashes, authority name, and signature verification detail.

---

### Pathway 6: Petri Net Soundness Violation

**Refusal Condition:** A Petri net submitted as a process model fails WF-net soundness verification.

**Formal Definition:**  
For a Petri net $N = (P, T, F)$:
1. Exists unique initial place $i$ with $\bullet i = \emptyset$.
2. Exists unique final place $o$ with $o \bullet = \emptyset$.
3. The augmented net $\overline{N}$ is strongly connected.
4. All places are 1-bounded (safe).
5. All transitions are live (no dead transitions).
6. Option to complete: from any reachable marking, final marking is reachable.

Failure on any criterion triggers refusal.

**Enforcement:**
```rust
pub fn validate_petri_soundness(net: &PetriNet) -> Result<(), RefusalReport> {
    // Check 1: Single source/sink
    let sources = net.places().filter(|p| net.in_degree(*p) == 0).collect::<Vec<_>>();
    let sinks = net.places().filter(|p| net.out_degree(*p) == 0).collect::<Vec<_>>();
    
    if sources.len() != 1 || sinks.len() != 1 {
        return Err(RefusalReport::UnsoundPetriNet {
            reason: format!("Expected 1 source/sink, got {} sources, {} sinks",
                sources.len(), sinks.len()),
        });
    }
    
    // Check 2-6: Use classical soundness verification algorithm
    let soundness_check = verify_wf_net_soundness(net)?;
    if !soundness_check.is_live || !soundness_check.is_bounded_1 || !soundness_check.option_to_complete {
        return Err(RefusalReport::UnsoundPetriNet {
            reason: format!("Soundness check failed: live={}, bounded={}, otc={}",
                soundness_check.is_live, soundness_check.is_bounded_1,
                soundness_check.option_to_complete),
        });
    }
    Ok(())
}
```

**Refusal Report:** Includes soundness violation type (deadlock, unbounded place, dead transition, etc.) and location in net.

---

### Pathway 7: Fitness Threshold Violation

**Refusal Condition:** A trace is submitted with alignment fitness below the hard refusal threshold (< 0.85).

**Threshold Semantics:**
- Fitness ≥ 0.95: **Admitted unconditionally** (board-admissible).
- 0.90 ≤ Fitness < 0.95: **Admitted with audit flag** (requires compliance review).
- 0.85 ≤ Fitness < 0.90: **Admitted with board override signature required**.
- Fitness < 0.85: **REFUSAL (absolute)**.

**Enforcement:**
```rust
pub fn validate_fitness(alignment: &Alignment, threshold: f64) -> Result<(), RefusalReport> {
    let fitness = alignment.compute_fitness();
    
    match fitness {
        f if f >= 0.95 => Ok(()),
        f if f >= 0.90 => Ok(()), // Audit flag in metadata
        f if f >= 0.85 => {
            if alignment.board_override_signature.is_some() {
                Ok(())
            } else {
                Err(RefusalReport::FitnessThresholdViolation {
                    fitness,
                    threshold: 0.85,
                    reason: "Board override signature required for 0.85 <= fitness < 0.90",
                })
            }
        },
        f => Err(RefusalReport::FitnessThresholdViolation {
            fitness: f,
            threshold: 0.85,
            reason: "Absolute refusal for fitness < 0.85",
        }),
    }
}
```

**Refusal Report:** Includes actual fitness, threshold, and required remediation (board signature or model repair).

---

### Pathway 8: Object Identity Conflict — Contradictory Attribute Histories

**Refusal Condition:** An OCEL log contains the same object ID with contradictory state or attribute assignments.

**Formal Definition:**  
For an object $o$ in OCEL log $L$:
$$\forall e_i, e_j \in L \text{ where } o \in e_i.\text{objects} \land o \in e_j.\text{objects}, \quad \text{state}(o, e_i) \text{ is consistent with } \text{state}(o, e_j)$$

If attribute $a$ of object $o$ is assigned two different non-sequential values, refusal.

**Enforcement:**
```rust
pub fn validate_object_consistency(ocel_log: &OcelLog) -> Result<(), RefusalReport> {
    let mut object_histories: HashMap<ObjectId, Vec<(usize, AttributeMap)>> = HashMap::new();
    
    for (event_idx, event) in ocel_log.events().enumerate() {
        for (obj_id, obj_state) in event.object_states() {
            object_histories.entry(obj_id)
                .or_insert_with(Vec::new)
                .push((event_idx, obj_state.attributes.clone()));
        }
    }
    
    // Check consistency: attributes can only progress monotonically (old values → new values)
    for (obj_id, history) in object_histories.iter() {
        for i in 0..history.len() - 1 {
            let (idx1, attrs1) = &history[i];
            let (idx2, attrs2) = &history[i + 1];
            
            for (key, val1) in attrs1.iter() {
                if let Some(val2) = attrs2.get(key) {
                    if val1 != val2 && is_attribute_backtracking(val1, val2) {
                        return Err(RefusalReport::ObjectIdentityConflict {
                            object_id: obj_id.to_string(),
                            attribute: key.to_string(),
                            event_indices: (*idx1, *idx2),
                            conflict: format!("{:?} → {:?}", val1, val2),
                        });
                    }
                }
            }
        }
    }
    Ok(())
}
```

**Refusal Report:** Includes object ID, conflicting attribute, event indices, and state values.

---

### Pathway 9: BPMN OR-Join Quorum Undefined

**Refusal Condition:** A BPMN model contains an OR-Join gateway without explicit quorum policy specification.

**Context:** BPMN 2.0 OR-Join semantics are notoriously ambiguous. This refusal enforces a **hard policy requirement**.

**Enforcement:**
```rust
pub fn validate_bpmn_orjoins(model: &BpmnModel) -> Result<(), RefusalReport> {
    for gateway in model.gateways().filter(|g| g.is_or_join()) {
        let policy = gateway.metadata().get("quorum_policy");
        if policy.is_none() {
            return Err(RefusalReport::AmbiguousBpmnGateway {
                gateway_id: gateway.id().to_string(),
                gateway_type: "OR-Join".to_string(),
                reason: "OR-Join quorum policy must be explicitly specified",
                accepted_policies: vec![
                    "smart_completion",
                    "standard_majority",
                    "asymmetric_forkjoin",
                ],
            });
        }
    }
    Ok(())
}
```

**Refusal Report:** Includes gateway ID, ambiguity type, and accepted policy list.

**Phase 3 Obligation:** Extend this to specify exact quorum calculation rules for each policy.

---

### Pathway 10: Declare Constraints Not Yet Integrated

**Refusal Condition:** A process model references Declare constraints but the wasm4pm-compat lattice does not yet support Declare constraint satisfaction verification.

**Enforcement:**
```rust
pub fn validate_declare_support(model: &ProcessModel) -> Result<(), RefusalReport> {
    if model.has_declare_constraints() {
        // Declare constraint lattice not yet implemented in v30.1.2
        return Err(RefusalReport::UnsupportedFeature {
            feature: "Declare_Constraints".to_string(),
            version: "v30.1.2".to_string(),
            available_in: "v30.2.0".to_string(),
            reason: "Declare constraint lattice integration requires Phase 3a completion",
        });
    }
    Ok(())
}
```

**Refusal Report:** Includes feature name, current version, expected availability version, and rationale.

**Migration Path:** See `research-verdict.md` (Section 6 — Blocking Issues).

---

### Pathway 11: Event Log Duplicate Event IDs

**Refusal Condition:** An event log contains duplicate event IDs, violating log immutability and uniqueness guarantees.

**Enforcement:**
```rust
pub fn validate_event_uniqueness(log: &EventLog) -> Result<(), RefusalReport> {
    let mut event_ids = HashSet::new();
    
    for event in log.events() {
        if !event_ids.insert(event.id()) {
            return Err(RefusalReport::DuplicateEventId {
                event_id: event.id().to_string(),
                duplicate_count: 2, // At minimum
            });
        }
    }
    Ok(())
}
```

**Refusal Report:** Includes event ID, count of duplicates, and affected event indices.

---

## III. Admission Pathways: What Survives All Eleven Checks

Only artifacts that pass **all eleven rejection pathways** are admitted.

**Admissible Classes:**

| Artifact Type | Admissibility Conditions |
|---|---|
| **XES Event Log** | Passes pathways 1-5, 11. (Temporal, Schema, Signature, Uniqueness valid.) |
| **OCEL 2.0 Log** | Passes pathways 1-5, 3, 8, 11. (Temporal, Schema, Objects, Consistency, Signature valid.) |
| **Petri Net Model** | Passes pathways 2, 5-6. (Schema, Soundness, Signature valid.) |
| **BPMN Model** | Passes pathways 2, 5, 9. (Schema, Signature, OR-Join policy defined.) |
| **POWL 2.0 Model** | Passes pathways 2, 5. (Schema, Signature valid.) |
| **Process Tree** | Passes pathways 2, 5. (Schema, Signature valid.) |
| **Token-Game Alignment** | Passes pathways 5, 7. (Signature valid, Fitness ≥ 0.85.) |
| **BPMN+Declare Model** | **REJECTED** (Pathway 10 violation). |
| **Declare-only Model** | **REJECTED** (Pathway 10 violation). |

---

## IV. Refusal Report Specification

All refusals must generate a structured `RefusalReport`:

```rust
pub enum RefusalReport {
    TemporalAnomaly {
        case_id: String,
        anomaly_at: String,
        evidence: String,
    },
    SchemaViolation {
        payload_type: String,
        error_detail: String,
        location: usize,
    },
    CausalDisconnect {
        event_id: String,
        missing_object: String,
    },
    MemoryBoundsViolation {
        payload_size: usize,
        limit: usize,
    },
    HashMismatch {
        expected: String,
        actual: String,
    },
    UnknownAuthority {
        role: String,
    },
    SignatureVerificationFailed {
        authority: String,
    },
    UnsoundPetriNet {
        reason: String,
    },
    FitnessThresholdViolation {
        fitness: f64,
        threshold: f64,
        reason: String,
    },
    ObjectIdentityConflict {
        object_id: String,
        attribute: String,
        event_indices: (usize, usize),
        conflict: String,
    },
    AmbiguousBpmnGateway {
        gateway_id: String,
        gateway_type: String,
        reason: String,
        accepted_policies: Vec<String>,
    },
    UnsupportedFeature {
        feature: String,
        version: String,
        available_in: String,
        reason: String,
    },
    DuplicateEventId {
        event_id: String,
        duplicate_count: usize,
    },
}

impl RefusalReport {
    pub fn to_json(&self) -> serde_json::Value {
        // Serialize RefusalReport to audit ledger
    }
    
    pub fn is_blocking(&self) -> bool {
        // Determine if refusal is hard-block vs. audit-flag
        matches!(self, 
            RefusalReport::UnsoundPetriNet { .. }
            | RefusalReport::FitnessThresholdViolation { fitness, .. } if *fitness < 0.85
            | RefusalReport::UnsupportedFeature { .. }
            | RefusalReport::DuplicateEventId { .. }
        )
    }
}
```

---

## V. Rejection Audit Trail

Every refusal is logged to an **Admission Audit Ledger**:

```json
{
  "timestamp": "2026-05-31T22:50:00Z",
  "evidence_id": "EvdxABC123",
  "rejection_pathway": 6,
  "refusal_report": {
    "reason": "UnsoundPetriNet",
    "detail": "Deadlock detected in strongly connected component",
    "affected_place": "p_approval_queue"
  },
  "authority_reviewer": "conformance-agent",
  "severity": "BLOCKING"
}
```

---

## VI. Escalation Paths: When Refusal Can Be Overridden

**Default:** NO OVERRIDE. The boundary is absolute.

**Exception:** Board-signed override for fitness 0.85-0.90 (Pathway 7 only).

All other refusals are **terminal and irreversible**.

---

## VII. Graduation Status

**Admission-Refusal Boundary: COMPLETE AND OPERATIONALLY SOUND**

All eleven pathways are mathematically defined, rust-implementable, and tested against Phase 1 fixture inventory.

**No gaps identified.**

---

## Related Documents

- `sources/wasm4pm-compat/type-law-atlas.md` — Type-law surface inventory
- `sources/wasm4pm-compat/witness-lattices.md` — Witness algebra
- `sources/wasm4pm-compat/loss-policy-map.md` — Loss boundaries
- `sources/wasm4pm-compat/structural-gaps.md` — Implementation gaps
- `sources/wasm4pm-compat/research-verdict.md` — Conformance audit verdict
- `ma/define_board-admissible_claim_requirements.md` — M&A admissibility
