# NOT TO COPY — What PM4Py Does That wasm4pm Must NOT Replicate

**Purpose:** Critical architectural memo. Items here are PM4Py patterns that would poison wasm4pm's type-law covenant if ported.

**Doctrine:** The product is CodeManufactory; RevOps is merely proof that CodeManufactory works. PM4Py is the oracle we measure against — not the blueprint we copy from.

---

## 1. Python Runtime Assumptions — DataFrame-First

**PM4Py behavior:**
All conformance, discovery, and filtering functions accept `Union[EventLog, pd.DataFrame]`. The library performs runtime isinstance checks and branches into DataFrame-specific code paths (via `check_is_pandas_dataframe()` and `check_pandas_dataframe_columns()`). The dominant internal representation is Pandas DataFrame.

```python
def discover_dfg(log: Union[EventLog, pd.DataFrame], ...):
    if check_is_pandas_dataframe(log):
        check_pandas_dataframe_columns(log, ...)
        # DataFrame path
    else:
        # EventLog path
```

**Why wasm4pm must NOT copy:**
- Pandas is a Python runtime with numpy dependency — has no Rust or WASM equivalent
- DataFrame-first means type erasure: any column can hold any value, NaN coercion is silent
- wasm4pm-compat's `Evidence<T, State, W>` requires `T` to have a known, named type at compile time
- No `Union[X, Y]` at function boundaries — every path must be its own admission surface with a named refusal law

---

## 2. Mutable State Throughout

**PM4Py behavior:**
Objects like `PetriNet`, `OCEL`, `EventLog`, `POWL` are mutable Python objects. Transitions, places, arcs are added/removed via method calls. The `BinaryRelation` underlying POWL models is a mutable dict. `POWL.validate_partial_orders()` raises `Exception` in place rather than returning a typed error.

```python
def validate_partial_orders(self):
    if isinstance(self, StrictPartialOrder):
        if not self.order.is_irreflexive():
            raise Exception("The irreflexivity of the partial order is violated!")
```

**Why wasm4pm must NOT copy:**
- wasm4pm-compat enforces a one-way typestate lifecycle: `Raw → Parsed → Admitted → {Projected | Exportable | Receipted}`
- Mutation after admission is a structural law violation
- Every lifecycle transition must go through `Admit::admit()` — there is no "mutate in place" path
- Errors must carry a named law reason type (e.g., `DanglingEventObjectLink`) — bare string exceptions are defects

---

## 3. No Named Law Refusals — Generic Exception Raising

**PM4Py behavior:**
Validation errors are raised as generic Python `Exception` with string messages:

```python
raise Exception("The irreflexivity of the partial order is violated!")
raise Exception("Start nodes must be a non-empty subset of the nodes of the relation!")
```

No structured error type, no named law, no refusal receipt.

**Why wasm4pm must NOT copy:**
- wasm4pm-compat requires every refusal to carry a specific named law as the reason type
- `InvalidInput` or string-typed catch-alls are explicitly forbidden (see `src/admission.rs`)
- The reason type `R` in `Refusal<R, W>` names the structural law violated (e.g., `MissingFinalMarking`)
- This enables proof gates and conformance diagnostics to name exactly what went wrong

---

## 4. No LossReport on Projections

**PM4Py behavior:**
`pm4py.project_log()` silently drops attribute columns not in the requested set. There is no report of what was lost, no named projection, no policy decision prior to loss.

```python
# PM4Py: silent attribute projection
projected = pm4py.project_log(log, attributes=["concept:name"])
# All other attributes gone; no record of loss
```

**Why wasm4pm must NOT copy:**
- wasm4pm-compat's `Project` operator requires a `ProjectionName`, a `LossPolicy`, and a `LossReport<From, To, Items>` on every non-refusing path
- There is no path from one external format directly to another — only `external → admitted compat → external | wasm4pm`
- Silent structure loss is classified as a defect, not a feature

---

## 5. No Provenance Tracking

**PM4Py behavior:**
Discovery results (Petri nets, POWL models, conformance diagnostics) carry no lineage information. There is no record of which log version, which algorithm parameters, or which timestamp produced the model.

**Why wasm4pm must NOT copy:**
- wasm4pm-compat uses `Witness` markers (e.g., `Ocel20`, `Xes1849`, `WfNetSoundnessPaper`) to prevent `Admission<T, Ocel20>` from being confused with `Admission<T, Xes1849>` at the type level
- Every `Evidence<T, State, W>` carries its witness as a compile-time tag
- `Receipted` state in the lifecycle marks evidence that has accumulated cryptographic provenance
- PM4Py has no equivalent — discovery is stateless and provenance-free

---

## 6. TypeScript WASM Bridge is NOT Rust Types

**pm4wasm.d.ts behavior:**
The WASM bridge exposes functions that take and return JSON strings (`string` in TypeScript). All structure is erased at the FFI boundary. `PowlModel` is an opaque arena handle. `BinaryRelationJs` is a flat `Uint32Array`.

```typescript
export function align_log(petri_net_json: string, log_json: string): string;
export function token_replay_fitness(petri_net_json: string, log_json: string): string;
```

**Why wasm4pm must NOT copy this pattern:**
- The TypeScript WASM bridge is an integration surface for JavaScript consumers — not the wasm4pm Rust type system
- wasm4pm-compat's types (`Evidence<T, Admitted, W>`, `Metric<KIND, NUM, DEN>`, `WfNetConst<SOUNDNESS>`) carry their invariants in const generics, not in runtime JSON parsing
- Copying the "pass JSON strings" pattern would eliminate all compile-time law enforcement
- The WASM bridge types (`PowlModel`, `BinaryRelationJs`) are output artifacts — not input types for wasm4pm-compat's Rust codebase

---

## 7. DataFrame-Polars Dual Support

**PM4Py behavior:**
The fork supports both Pandas DataFrame and Polars LazyFrame via `is_polars_lazyframe()` runtime checks throughout the codebase:

```python
from pm4py.utils import is_polars_lazyframe
if is_polars_lazyframe(log):
    # Polars path
```

**Why wasm4pm must NOT copy:**
- Adding runtime format dispatch creates invisible untested code paths
- wasm4pm-compat must have exactly one admission path per external format, each with its own named `Witness` and refusal law
- "Works with either format" is a type-erasure antipattern that hides conformance failures

---

## 8. Engine Logic in the Compatibility Crate

**PM4Py behavior:**
PM4Py contains full algorithm implementations: alpha miner, inductive miner, token-based replay, A* alignment, DECLARE conformance, log skeleton, organizational mining. It IS the engine.

**Why wasm4pm must NOT copy:**
- wasm4pm-compat is a structure-only crate: no engine logic (discovery, conformance checking, replay, alignment) belongs here
- Engine logic graduates to `wasm4pm` (the execution engine)
- The crate's job is to define the type law surfaces, admission/refusal paths, and provenance structures
- Embedding algorithm implementations would violate the one-way door: `wasm4pm-compat → wasm4pm`, never the reverse

---

## 9. DSPy LLM Integration as Core Infrastructure

**PM4Py behavior:**
The fork integrates DSPy + Groq LLM (`gpt-oss-20b`) as a first-class discovery path: `pm4py.algo.dspy.powl.natural_language.PowlPredictor` generates POWL models from natural language via LLM chain-of-thought. This is exposed in the WASM bridge as `get_demos_for_domain()`.

**Why wasm4pm must NOT copy:**
- LLM outputs are non-deterministic and cannot carry a named law receipt
- A POWL model generated by an LLM has no provenance chain traceable to a specific event log
- LLM integration belongs in an application layer above the type-law crate, not inside wasm4pm-compat
- `get_demos_for_domain()` can be consumed as a WASM utility — it must not be a source of admitted evidence

---

## 10. OCEL 2.0 as a Python-only Concern

**PM4Py behavior:**
OCEL 2.0 import/export (JSON, XML, SQLite, CSV) is deeply Python-specific: `pm4py.read_ocel2_json()`, `pm4py.write_ocel2_sqlite()`, etc. These use pandas, SQLite3, and complex Python object graphs.

**Why wasm4pm must NOT copy:**
- OCEL 2.0 I/O in wasm4pm-compat is defined as a type-law surface (`src/ocel.rs`), not as a file I/O implementation
- The `OcelLog`, `OcelEvent`, `EventObjectLink`, `ObjectObjectLink`, `ObjectChange` types define the structural contract
- Actual deserialization from .json/.sqlite goes through `Admit::admit()` with a named `Ocel20` witness — not through file-path APIs
- SQLite3 is a Python runtime dependency with no equivalent in the no-std WASM compilation target
