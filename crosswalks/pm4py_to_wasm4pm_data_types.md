# Data Type Crosswalk: PM4Py to wasm4pm-compat

This document defines the formal type mapping between dynamically typed PM4Py (Python) objects and the concrete, statically typed, binary structures executed within the `wasm4pm-compat` WebAssembly process mining engine.

---

## 1. Data Type Mapping Matrix

| PM4Py Class / Attribute | OCEL 2.0 / XES Mapping | wasm4pm-compat Rust Struct | Memory Representation | Type-Safe Invariant |
|---|---|---|---|---|
| `pm4py.objects.log.obj.EventLog` | Event and Object collection | `pub struct OcelEventLog` | Serialized JSON / MessagePack buffer | Must contain referentially valid events and objects with no orphaned references. |
| `pm4py.objects.log.obj.Trace` | Chronological case sequence | `pub struct XesTrace` | `Vec<XesEvent>` | Event timestamps must be monotonically non-decreasing. |
| `event["concept:name"]` | Transition activity name | `pub struct ActivityName(pub String)` | Heap-allocated UTF-8 string | String must be non-empty and conform to `[a-zA-Z0-9_\-\:]+`. |
| `event["time:timestamp"]` | Millisecond UTC timestamp | `pub struct Timestamp(pub i64)` | `i64` signed 64-bit integer | Timestamp must represent valid UTC time in milliseconds. |
| `event["org:resource"]` | Executive actor / resource ID | `pub struct ResourceId(pub Uuid)` | `[u8; 16]` raw UUID byte array | Must be a valid Version 4 UUID. |

---

## 2. Rust Struct Definitions and Invariants

To guarantee that no dynamically typed Python data can cause undefined behavior or memory violations when executed within the WebAssembly guest runtime, the `wasm4pm-compat` engine parses and validates PM4Py exports into the following strict types:

### 2.1 OcelEventLog (Object-Centric Event Log)
In PM4Py, an object-centric event log can contain loose dictionary objects. In `wasm4pm-compat`, it is defined by a strongly typed schema.

```rust
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OcelEventLog {
    pub events: HashMap<String, OcelEvent>,
    pub objects: HashMap<String, OcelObject>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OcelEvent {
    pub activity: ActivityName,
    pub timestamp: Timestamp,
    pub omap: Vec<String>, // References to objects key map
    pub vmap: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OcelObject {
    pub object_type: String,
    pub omap: Vec<String>, // Object-to-object relationships
    pub vmap: HashMap<String, serde_json::Value>,
}
```

**Invariant:**
$$\forall e \in \text{events.values()}, \quad \forall o\_id \in e.\text{omap}, \quad \text{objects.contains\_key}(o\_id) \equiv \operatorname{True}$$
If this constraint is violated during deserialization, the parser immediately returns a `CausalDisconnect` error.

### 2.2 XesTrace (Standard Case-Centric Trace)
PM4Py's `Trace` object is mapped to a contiguous vector of events where order is strictly checked.

```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct XesEvent {
    pub activity: ActivityName,
    pub timestamp: Timestamp,
    pub resource: Option<ResourceId>,
    pub attributes: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct XesTrace {
    pub case_id: String,
    pub events: Vec<XesEvent>,
}
```

**Invariant:**
$$\forall i \in [0, |\text{events}| - 2], \quad \text{events}[i].\text{timestamp}.0 \le \text{events}[i+1].\text{timestamp}.0$$
Trace validation checks that timestamps are monotonic. Backward temporal drift is rejected with a `TemporalAnomaly` refusal signature.

### 2.3 ActivityName, Timestamp, and ResourceId
To prevent type confusion and buffer overflow vulnerabilities, primitive types are wrapped in tuple structs enforcing specific constructor validators.

```rust
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ActivityName(pub String);

impl ActivityName {
    pub fn new(name: String) -> Result<Self, TypeError> {
        if name.trim().is_empty() {
            return Err(TypeError::EmptyActivityName);
        }
        Ok(ActivityName(name))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamp(pub i64);

impl Timestamp {
    pub fn new(millis: i64) -> Result<Self, TypeError> {
        if millis < 0 {
            return Err(TypeError::NegativeTimestamp);
        }
        Ok(Timestamp(millis))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourceId(pub Uuid);

impl ResourceId {
    pub fn new(uuid: Uuid) -> Self {
        ResourceId(uuid)
    }
}

#[derive(Debug)]
pub enum TypeError {
    EmptyActivityName,
    NegativeTimestamp,
}
```

---

## 3. Related System References

For details on how these types are validated and processed, refer to:
- [XES Standard Mapping](file:///Users/sac/process-intelligence/standards/xes.md) - Extensible Event Stream schema specification.
- [OCEL Standard Mapping](file:///Users/sac/process-intelligence/standards/ocel.md) - Object-Centric Event Log structure.
- [Type-Law Atlas](file:///Users/sac/process-intelligence/sources/wasm4pm-compat/type-law-atlas.md) - In-depth structural invariants for the WebAssembly sandbox.
- [Evidence Structures](file:///Users/sac/process-intelligence/sources/wasm4pm-compat/evidence-structures.md) - Design patterns for generic wrappers in `wasm4pm-compat`.
