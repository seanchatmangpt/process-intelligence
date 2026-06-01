# Declare — Declarative Process Constraint Modeling

**Authority:** Pesic & van der Aalst 2006 (Declare); OC-Declare extension (Berti 2023)
**Witness key:** `declare` — `WitnessFamily::Paper`

---

## Declare Overview

Declare is a **declarative** process modeling language. Where imperative models (BPMN,
process trees, Petri nets) prescribe a fixed execution path, Declare specifies constraints
that must hold over all traces — everything not forbidden is permitted.

Declare constraints are expressed in **Linear Temporal Logic over finite traces (LTLf)**.
Each constraint template is a named LTLf formula over one or two activity names.

---

## DeclareTemplate Types

### Unary constraints (one activity)

| Template | LTLf semantics |
|---|---|
| `Existence(a)` | `a` occurs at least once. |
| `Absence(a)` | `a` does not occur. |
| `Init(a)` | If any activity occurs, `a` is the first. |
| `Existence2(a)` | `a` occurs at least twice. |
| `Existence3(a)` | `a` occurs at least three times. |
| `Absence2(a)` | `a` occurs at most once (zero or one). |
| `Absence3(a)` | `a` occurs at most twice. |

### Binary relation constraints (two activities)

| Template | LTLf semantics |
|---|---|
| `RespondedExistence(a, b)` | If `a` occurs, `b` must also occur (before or after). |
| `CoExistence(a, b)` | `a` and `b` either both occur or neither occurs. |
| `Response(a, b)` | Every `a` is eventually followed by a `b`. |
| `Precedence(a, b)` | Every `b` is preceded by an `a`. |
| `Succession(a, b)` | Both `Response` and `Precedence` hold. |
| `AlternateResponse(a, b)` | Between any two consecutive `a`s there must be a `b`. |
| `AlternatePrecedence(a, b)` | Between any two consecutive `b`s there must be an `a`. |
| `AlternateSuccession(a, b)` | Both `AlternateResponse` and `AlternatePrecedence` hold. |
| `ChainResponse(a, b)` | `b` must immediately follow `a`. |
| `ChainPrecedence(a, b)` | `a` must immediately precede `b`. |
| `ChainSuccession(a, b)` | Both `ChainResponse` and `ChainPrecedence` hold. |

### Negative / exclusion constraints

| Template | LTLf semantics |
|---|---|
| `NotCoExistence(a, b)` | `a` and `b` never both occur in a case. |
| `NotSuccession(a, b)` | `a` cannot be eventually followed by `b`. |
| `NotChainSuccession(a, b)` | `b` cannot immediately follow `a`. |
| `ExclusiveChoice(a, b)` | Exactly one of `a` or `b` occurs — not both, not neither. |

---

## LTLf Formula Mapping

Each template maps to a well-defined LTLf formula over the trace alphabet. For example:

- `Response(a, b)` → `G(a → F(b))` (globally: every `a` is eventually followed by `b`).
- `Precedence(a, b)` → `G(b → O(a))` (globally: every `b` was preceded by `a`; using
  past-LTL operator).
- `ChainResponse(a, b)` → `G(a → X(b))` (globally: `a` is immediately followed by `b`
  in the next step).
- `Absence(a)` → `G(¬a)` (globally: `a` never occurs).

These formulas are the basis for LTLf automaton compilation and conformance replay.

---

## wasm4pm-compat Implementation (declare.rs)

`src/declare.rs` models Declare as a **structure-only** constraint vocabulary:

- `Activity` — a `#[repr(transparent)]` `String` newtype. Strongly-named activity label.
- `DeclareTemplate` — a closed enum of all 21 template types (7 unary + 10 binary +
  4 negative). Each variant has an `arity()` method returning 1 or 2.
- `DeclareScope` — models the OC-Declare extension: `Single(ObjectType)`,
  `Multiple(Vec<ObjectType>)`, `Synchronized(Vec<ObjectType>)`. Scopes a constraint to
  specific object types without evaluating synchronization.
- `DeclareConstraint` — pairs a `DeclareTemplate` with one or two `Activity` labels and
  an optional `DeclareScope`.
- `DeclareRefusal` — named refusal surface: `ArityMismatch`, `EmptyActivityLabel`,
  `UnknownTemplate`.

`declare.rs` never evaluates constraints, mines them from logs, or compiles LTLf
automata — those are engine responsibilities.

---

## What wasm4pm Must Provide

| Capability | Graduates to |
|---|---|
| LTLf automaton compilation for each template | `wasm4pm` |
| Declare mining from event logs | `wasm4pm` |
| Constraint conformance checking (replay against automata) | `wasm4pm` |
| OC-Declare synchronized scope evaluation | `wasm4pm` |
| Declare → BPMN / Petri-net translation | `wasm4pm` |
| Declare fitness/precision/generalization metrics | `wasm4pm` |

---

## Board Placement

Declare is the standard for compliance-driven process automation: regulatory obligations,
SLA contracts, and audit requirements are naturally expressed as declarative constraints.
wasm4pm-compat defines all 21 Declare template shapes including OC-Declare object scopes,
ensuring that compliance constraints can travel from structure definition through the type
system to the `wasm4pm` LTLf evaluation engine without loss of naming or arity.