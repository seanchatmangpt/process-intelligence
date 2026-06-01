# Declare Constraint Language — Deep Formal Analysis

**Analyst:** Dr. OCEL Specialist (AGI)
**Date:** 2026-05-31
**Source:** Pesic, Schonenberg, van der Aalst — "Declare: Full Support for Loosely-Structured Processes" (2007); De Giacomo, Di Ciccio, et al. — LTLf Declare formalization

---

## Formal Objects

### Constraint Template (discriminated union)

Each template is a parameterized LTLf formula over activities.

#### Existence Templates (unary — one activity parameter)

| Template | LTLf Formula | Semantic |
|---|---|---|
| `Existence(A, n)` | `◇(A ∧ X(◇A ...))` n times | A occurs at least n times |
| `Absence(A, n)` | `¬(Existence(A, n+1))` | A occurs at most n times |
| `ExactlyOne(A)` | `Existence(A,1) ∧ Absence(A,1)` | A occurs exactly once |
| `Init(A)` | A is the first event | First event is A |
| `End(A)` | A is the last event | Last event is A |

#### Relation Templates (binary — two activity parameters)

| Template | LTLf Formula | Semantic |
|---|---|---|
| `Response(A, B)` | `□(A → ◇B)` | Every A is eventually followed by B |
| `Precedence(A, B)` | `¬B U A` | B may not occur before first A |
| `Succession(A, B)` | `Response(A,B) ∧ Precedence(A,B)` | Both response and precedence |
| `AltResponse(A, B)` | `□(A → X(¬A U B))` | After A, B before next A |
| `AltPrecedence(A, B)` | `(¬B U A) ∧ □(B → □(¬B U (A ∧ X(¬B U B))))` | Before each B, an A not preceded by B |
| `ChainResponse(A, B)` | `□(A → X B)` | A is immediately followed by B |
| `ChainPrecedence(A, B)` | `□(B → Y A)` | B is immediately preceded by A |
| `CoExistence(A, B)` | `(◇A ↔ ◇B)` | A and B both occur or neither |
| `NonCoExistence(A, B)` | `¬(◇A ∧ ◇B)` | A and B cannot both occur |
| `NotSuccession(A, B)` | `¬Succession(A, B)` | A and B are not in succession |
| `NotChainSuccession(A, B)` | `□(A → ¬X B)` | A is never immediately followed by B |

### DeclareModel
- `constraints: Set<DeclareConstraint>` — collection of active constraints
- `activities: Set<ActivityName>` — alphabet
- Semantics: a trace satisfies the model iff it satisfies ALL constraints

### DeclareConstraint
- `template: ConstraintTemplate`
- `activity_a: ActivityName`
- `activity_b: Option<ActivityName>` — None for unary templates
- `satisfaction_level: SatisfactionLevel` — Satisfied | Violated | Vacuously Satisfied

### SatisfactionLevel
- `Satisfied` — constraint is non-vacuously fulfilled
- `Violated` — constraint is not fulfilled; this is a conformance defect
- `VacuouslySatisfied` — antecedent never triggered (e.g., Response(A,B) where A never occurs)

---

## Key Insight: LTLf as the Semantics Substrate

Declare constraints are syntactic sugar over Linear Temporal Logic over finite traces (LTLf). This has critical implications:

1. **Decidability:** LTLf satisfiability and model checking are decidable (2EXPTIME for general LTLf, but polynomial for the Declare template fragment)
2. **Automata translation:** Each LTLf formula translates to a finite automaton; constraint checking is automaton intersection
3. **Vacuous satisfaction is not conformance:** A model with 80% vacuously satisfied constraints is not well-calibrated
4. **Negative constraints are checkable:** `NonCoExistence`, `NotSuccession` are first-class constraints, not absence of evidence

---

## wasm4pm Coverage Assessment

### wasm4pm-compat
`src/declare.rs` in wasm4pm-compat defines constraint template shapes as Rust types. This covers the structural representation only.

| Declare Concept | wasm4pm-compat | Coverage |
|---|---|---|
| Constraint template enum | `src/declare.rs` | Shape only |
| LTLf formula encoding | Not present | None |
| Satisfaction level | Not present | None |
| DeclareModel container | Not present | None |

### wasm4pm Execution
**Status: Declare checking is MISSING from wasm4pm.**

No LTLf-based checking, no automata construction, no satisfaction level computation exists in wasm4pm.

---

## PM4Py Coverage Assessment

| Capability | PM4Py Module | Maturity |
|---|---|---|
| Declare template checking | `pm4py.conformance.declare` | Basic — subset of templates |
| LTLf checking | `pm4py.conformance.ltl` | Partial — hardcoded templates |
| Vacuous satisfaction detection | Not present | None |
| Full template library | Incomplete | Partial |

PM4Py's Declare support is partial. A complete LTLf-based implementation would surpass PM4Py's coverage.

---

## Gap Action

wasm4pm must implement LTLf-based Declare constraint checking:
- Input: `Admitted<OcelLog, Ocel20>` (or `Admitted<EventLog, Xes1849>` for flat traces)
- Model: `DeclareModel` (set of typed constraints)
- Algorithm: Per-constraint automaton construction → trace replay → satisfaction level
- Output: Per-constraint `SatisfactionLevel` + aggregate conformance score
- Receipt: typed `DeclareConformanceReceipt<W>` for board claim attestation

**Board claim:** "Our process conforms to all regulatory Declare constraints" requires:
- `DeclareModel` loaded from a normative source
- All constraints checked as `Satisfied` (not `VacuouslySatisfied`)
- Receipt chain showing the specific constraint templates and activities

---

## Action Items

| Priority | Action | Owner |
|---|---|---|
| P0 | Add `SatisfactionLevel`, `DeclareConstraint`, `DeclareModel` to wasm4pm-compat `src/declare.rs` | wasm4pm-compat |
| P0 | Implement LTLf automaton construction for all 15 standard templates | wasm4pm |
| P0 | Implement trace checking against DeclareModel | wasm4pm |
| P1 | Implement vacuous satisfaction detection | wasm4pm |
| P1 | Add object-centric Declare (per-object-type constraint sets) | wasm4pm |
