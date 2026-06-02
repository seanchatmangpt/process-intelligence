# AutoInstinct / ccog Lineage Map

**Generated:** 2026-06-01
**Agent:** Agent C — AutoInstinct / ccog Lineage Cartographer
**Primary Corpus:** `/Users/sac/dteam/` (canonical crate authority)
**Supporting Corpus:** compiled-cognition-hub, knhk, zoeapp, process-intelligence doctrine
**Total corpus sources read:** 18

---

## Frame Law (Do Not Violate)

1. A knowledge hook is NOT middleware, callback, event listener, or plugin.
2. Autonomic Knowledge Actuation is NOT automation, AI workflow, or lifecycle management.
3. AutoInstinct/ainst is NOT an agent framework or report machine.
4. ccog is NOT a chatbot runtime.
5. A receipt is NOT a log.
6. A report is NOT proof.
7. LLM output is NOT runtime authority.
8. A summary is NOT evidence.

---

## 1. AutoInstinct (ainst) — Authoritative Definition

### From Cargo.toml (`/Users/sac/dteam/crates/autoinstinct/Cargo.toml`)

```toml
[package]
name = "autoinstinct"
version = "30.1.1"
description = "AutoInstinct: trace-to-instinct compiler for ccog Autonomic Instincts"
```

Binary entrypoint:
```toml
[[bin]]
name = "ainst"
path = "src/bin/ainst.rs"
```

### From lib.rs doc comment (`/Users/sac/dteam/crates/autoinstinct/src/lib.rs`)

> "AutoInstinct v30.1.1 — trace-to-instinct compilation.
>
> AutoInstinct is the compiler layer above ccog. It learns lawful response policies from proof-backed traces, OCEL worlds, public ontology profiles, and adversarial JTBD tests, then compiles admitted policies into deployable field packs.
>
> Governing law: `A = μ(O*)`. Raw observation does not authorize action. Action is projected from closed context."

### Pipeline (verbatim from `/Users/sac/dteam/crates/autoinstinct/src/lib.rs`)

```
ontology profile
→ OCEL worlds
→ trace corpus
→ motif discovery
→ candidate μ policy
→ generated JTBD tests
→ gauntlet
→ compiled field pack
→ ccog deployment
```

### ainst CLI Grammar (verbatim from `/Users/sac/dteam/crates/autoinstinct/src/bin/ainst.rs`)

```
ainst generate ocel | jtbd
ainst validate ocel
ainst ingest corpus
ainst discover motifs
ainst propose policy
ainst run gauntlet
ainst compile pack
ainst publish pack
ainst deploy edge
ainst verify replay
ainst export bundle
```

### ainst is NOT

- An agent framework (source: ABSOLUTE FRAME LAW §3)
- A report machine (source: ABSOLUTE FRAME LAW §3)
- A monitor, dashboard, or alerting layer
- A runtime executor (that is ccog's role)

---

## 2. ccog — Authoritative Definition

### From README (`/Users/sac/dteam/README.md`)

> "crates/ccog: The Compiled Cognition library and facade."

### From lib.rs doc comment (`/Users/sac/dteam/crates/ccog/src/lib.rs`)

> "Compiled Cognition core: field-cognition facade over RDF graph closure.
>
> ccog knows what the graph permits the field to do.
>
> The core formula: `U → O*_U → C_U → A_U → R_U`
>
> - `U` = bounded operational field
> - `O*_U` = semantic closure of that field (from RDF graph)
> - `C_U` = compiled cognition artifact (cognitive pass)
> - `A_U` = admissible operations
> - `R_U` = PROV receipt (proof + provenance)"

### ccog is NOT

- A chatbot runtime (source: ABSOLUTE FRAME LAW §4)
- An LLM inference layer
- A dynamic reasoning system

### Supporting context from compiled-cognition-hub PHILOSOPHY.md

> "Intelligence stops being a service and becomes a deterministic, zero-dependency property of the binary. It ceases to be an Oracle, and becomes an Angel—present at the moment of action, bounded by law, and instantly verifiable."

---

## 3. The ainst / ccog Distinction

### From `/Users/sac/dteam/crates/ccog/docs/end_to_end_jtbd.md` (C4 Container View)

| Plane | Owner | Role |
|---|---|---|
| **Control Plane** | `ainst` | Compiler: ontology loading, motif compilation, admission tests, CompiledCcogConfig generation |
| **Runtime Plane** | `ccog` | Executor: COG8 graph execution, bark kernel, POWL8, canonical response selection |
| **Proof Plane** | both | EvidenceLedger, POWL64 route, replay, audit |

Quoted from the architecture document:

> "Hard surface separation between manufacturing (Control Plane), execution (Runtime Plane), and proof (Proof Plane) ensures architectural integrity."

> "End-to-end JTBD handling is achieved through graphs of bounded COG8 closures, **compiled by `ainst`**, **executed by `ccog`**, and **proved by POWL64**."

### From dteam README (`/Users/sac/dteam/README.md`)

> "crates/autoinstinct: Trace compiler CLI tooling."

ainst = the compiler / control plane.
ccog = the execution authority / runtime kernel.

---

## 4. CompiledCcogConfig — Product Artifact

### Rust struct definition (`/Users/sac/dteam/crates/ccog/src/runtime/mod.rs`, lines 64–73)

```rust
/// Formalized L3 Config Loader target (Compiled Ccog Config).
///
/// Contains the admitted compiled cognition configuration as a nonlinear
/// graph of COG8 closures.
#[derive(Debug, Clone)]
pub struct CompiledCcogConfig<const N: usize, const E: usize> {
    /// Admitted field pack logic.
    pub pack: LoadedFieldPack,
    /// COG8 closure nodes (L1 decide target).
    pub nodes: [Cog8Row; N],
    /// POWL topology edges (L1 routing target).
    pub edges: [Cog8Edge; E],
    /// MCP projection table.
    pub mcp_projections: MCPProjectionTable,
}
```

### FieldPackArtifact — serializable output of ainst compile (`/Users/sac/dteam/crates/autoinstinct/src/compile.rs`)

```rust
/// Serializable field-pack artifact.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FieldPackArtifact {
    pub name: String,
    pub autoinstinct_version: String,
    pub ontology_profile: Vec<String>,
    pub admitted_breeds: Vec<String>,
    pub rules: Vec<(String, ccog::instinct::AutonomicInstinct)>,
    pub default_response: ccog::instinct::AutonomicInstinct,
    /// `urn:blake3:` of the canonical artifact bytes (excluding this field).
    pub digest_urn: String,
}
```

Comment from the module:

> "Once a candidate policy passes the gauntlet, `compile` emits a deployable `FieldPackArtifact`: a serializable record containing the policy's rules, ontology profile, admitted breeds, version, and `urn:blake3` digest of the canonical bytes (so receipts can prove which pack produced which decision)."

`CompiledCcogConfig` is the admitted runtime configuration produced when `FieldPackArtifact` is loaded into the ccog runtime. It is the product artifact of the ainst pipeline — the compiled, BLAKE3-digested, gauntlet-admitted representation of lawful response policy.

---

## 5. EvidenceLedger — Proof Artifact

### Rust struct definition (`/Users/sac/dteam/crates/ccog/src/runtime/conformance.rs`, lines 21–36)

```rust
/// Collection of recorded cognitive traces for conformance analysis.
#[derive(Debug, Clone, Default)]
pub struct EvidenceLedger {
    /// Sequence of route proofs observed in the live environment.
    pub traces: Vec<Powl64>,
}

impl EvidenceLedger {
    pub fn new() -> Self { Self::default() }
    pub fn record(&mut self, trace: Powl64) { self.traces.push(trace); }
}
```

### Conformance module doc comment (verbatim)

> "Provides structural alignment between live `EvidenceLedger` traces (recorded as `Powl64` route proofs) and the admitted `CompiledCcogConfig` topology.
>
> - **Fitness**: Ratio of observed trace steps that are admissible in the topology.
> - **Precision**: Ratio of topology edges that are exercised by the ledger.
> - **Generalization**: Ability of the model to handle unseen but lawful behavior.
> - **Simplicity**: Occam's razor for the COG8 topology.
> - **False Closures**: Identification of traces that terminate on non-sink nodes."

The EvidenceLedger is the proof artifact of ccog execution. It holds POWL64 route proofs — replayable, receipt-bearing records of what the runtime actually did. Conformance checking (Van der Aalst alignment) measures whether live execution matched the admitted CompiledCcogConfig topology.

### From end-to-end JTBD checklist

> "- [ ] EvidenceLedger replay is defined."

A JTBD is not supported unless its EvidenceLedger replay is defined. This is the proof requirement.

---

## 6. "Compile Away the LLM" Law

### From compiled-cognition-hub PHILOSOPHY.md

> "By utilizing Latency Collapse and Compile-Time AutoML, we have demonstrated that machine intelligence can be compiled directly into the executable artifact. Intelligence stops being a service and becomes a deterministic, zero-dependency property of the binary."

### From compiled-cognition-hub main.rs (menu option text)

> "[3] Trigger Compile-Time AutoML (dteam)"
> "The intelligence is part of the binary, zero external dependencies."

### From zoeapp truex-collaborative-intelligence.md (Section 11, verbatim)

> "**TRUEX COST LAW**
>
> Do not make probability do determinism's job.
> `Cost(ProbabilisticApproximationOfDeterminism) ≫ Cost(CompiledDeterminism)`
>
> LLMs are candidate generators. Truex is the admission system.
> LLMs propose. Truex admits."

### From the ABSOLUTE FRAME LAW

> "7. LLM output is NOT runtime authority."

### The Compile-Away Law (synthesized from corpus)

ainst ingests LLM-generated worlds (phase 8: `world_gen`) as **untrusted candidates only**. The autoinstinct pipeline (`/Users/sac/dteam/crates/autoinstinct/src/llm/`) is explicitly a bridge from LLM output into the admission system:

From `/Users/sac/dteam/crates/autoinstinct/src/lib.rs`:

> "LLM provider bridge (Gemini CLI; pluggable). Untrusted output goes through strict admission before becoming corpus."

The law: LLM output enters only as candidate observation. It cannot become compiled policy without passing the gauntlet. The compiled `FieldPackArtifact` — with its BLAKE3 digest — is the boundary. Everything before the digest is probabilistic. Everything after the digest is deterministic, admitted, and authoritative.

---

## 7. AutonomicInstinct — The Runtime Lattice

### Enum definition (`/Users/sac/dteam/crates/ccog/src/instinct.rs`)

```rust
/// Right-sized response class — the action the cognition surface admits.
pub enum AutonomicInstinct {
    Settle,    // Known harmless event — return to baseline.
    Retrieve,  // Expected package/delivery — retrieve now.
    Inspect,   // Unknown but low-threat — inspect.
    Ask,       // Missing evidence — request clarification.
    Refuse,    // Action does not belong — refuse the transition.
    Escalate,  // Persistent unresolved disturbance — escalate.
    Ignore,    // No-op. Default — safest fallback when no other variant applies.
}
```

The AutonomicInstinct is the output of ccog's closed-loop decision. It is the only thing the runtime emits for external action. It is not a message, not a recommendation, not a report — it is the admitted action class, backed by the compiled policy lattice.

---

## 8. Correct Direction (Frame Law Spine)

```
attempt → hook → admission/refusal → durable motion → receipt → replay → accounting → promotion
```

In ainst/ccog terms:

```
OCEL world (attempt)
→ ainst gauntlet (hook + admission)
→ FieldPackArtifact BLAKE3 digest (durable motion)
→ CompiledCcogConfig loaded into ccog (receipt)
→ EvidenceLedger POWL64 traces (replay)
→ conformance check fitness/precision/generalization (accounting)
→ promoted to production deployment (promotion)
```

---

## 9. Source Authority Map

| Claim | Authoritative Source |
|---|---|
| ainst = trace-to-instinct compiler | `/Users/sac/dteam/crates/autoinstinct/src/lib.rs` doc comment |
| ainst CLI grammar | `/Users/sac/dteam/crates/autoinstinct/src/bin/ainst.rs` |
| ccog = compiled cognition runtime kernel | `/Users/sac/dteam/crates/ccog/src/lib.rs` doc comment |
| ccog formula: `U → O*_U → C_U → A_U → R_U` | `/Users/sac/dteam/crates/ccog/src/lib.rs` |
| ainst = Control Plane / ccog = Runtime Plane | `/Users/sac/dteam/crates/ccog/docs/end_to_end_jtbd.md` |
| CompiledCcogConfig struct | `/Users/sac/dteam/crates/ccog/src/runtime/mod.rs` lines 64–73 |
| FieldPackArtifact struct | `/Users/sac/dteam/crates/autoinstinct/src/compile.rs` |
| EvidenceLedger struct | `/Users/sac/dteam/crates/ccog/src/runtime/conformance.rs` lines 21–36 |
| AutonomicInstinct enum | `/Users/sac/dteam/crates/ccog/src/instinct.rs` |
| LLM output is untrusted candidate only | `/Users/sac/dteam/crates/autoinstinct/src/lib.rs` (llm module comment) |
| Compile-Away philosophy | `/Users/sac/compiled-cognition-hub/governance/PHILOSOPHY.md` |
| LLMs propose. Truex admits. | `/Users/sac/zoeapp/docs/vision2030/truex-collaborative-intelligence.md` Section 11 |
| Knowledge hooks in ccog | `/Users/sac/dteam/crates/ccog/src/hooks.rs` (KnowledgeHook, HookRegistry) |
| autoinstinct is NOT an agent framework | Frame Law §3 / ABSOLUTE FRAME LAW |
| ccog is NOT a chatbot runtime | Frame Law §4 / ABSOLUTE FRAME LAW |
