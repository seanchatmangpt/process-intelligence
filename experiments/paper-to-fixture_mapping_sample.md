# Experiment: Paper-to-Fixture Mapping

This document provides concrete mappings between classical academic process-mining literature and standardized JSON validation fixtures representing executable test scenarios. All fixtures represent the formal objects defined in the paper canon (see file:///Users/sac/process-intelligence/sources/papers/paper-canon.md).

## 1. van der Aalst 1998: Workflow Nets (Soundness & Liveness)

Dr. Wil van der Aalst defined a Workflow Net (WF-net) as a Petri Net with a unique source place $i$, a unique sink place $o$, and where every node is on a path from $i$ to $o$. Soundness requires:
1. **Option to complete**: For any marking reachable from $i$, the marking $o$ is reachable.
2. **Proper completion**: If marking $o$ is reached, there are no other tokens left in the net.
3. **No dead transitions**: No transition can never fire.

### Sound WF-net JSON Fixture
This fixture represents a sound process with parallel split and synchronization (AND-gate).

```json
{
  "paper_reference": "van_der_aalst_1998_workflow_nets",
  "fixture_id": "wf_net_sound_and_split",
  "is_sound": true,
  "places": ["i", "p1", "p2", "o"],
  "transitions": ["t_start", "t_a", "t_b", "t_end"],
  "arcs": [
    {"source": "i", "target": "t_start"},
    {"source": "t_start", "target": "p1"},
    {"source": "t_start", "target": "p2"},
    {"source": "p1", "target": "t_a"},
    {"source": "p2", "target": "t_b"},
    {"source": "t_a", "target": "o"},
    {"source": "t_b", "target": "o"}
  ],
  "initial_marking": {"i": 1},
  "final_marking": {"o": 2}
}
```

### Unsound WF-net JSON Fixture (Token Leak)
This fixture represents an unsound process where `t_start` puts a token in `p1` and `o` (premature completion, token leak in `p1`).

```json
{
  "paper_reference": "van_der_aalst_1998_workflow_nets",
  "fixture_id": "wf_net_unsound_leak",
  "is_sound": false,
  "soundness_violation": "token_leak",
  "places": ["i", "p1", "o"],
  "transitions": ["t_start", "t_a"],
  "arcs": [
    {"source": "i", "target": "t_start"},
    {"source": "t_start", "target": "p1"},
    {"source": "t_start", "target": "o"},
    {"source": "p1", "target": "t_a"}
  ],
  "initial_marking": {"i": 1},
  "final_marking": {"o": 1}
}
```

## 2. Adriansyah 2014: Alignment Conformance Checking

Adriansyah introduced alignment conformance, which maps trace events to model transitions, minimizing distance. A step can be a "Move on Log", "Move on Model", or "Move on Both" (Synchronous Move).

### Conformance Trace Alignment Fixture
```json
{
  "paper_reference": "adriansyah_2014_alignment_conformance",
  "trace_id": "trace_004_deviation",
  "fitness_score": 0.75,
  "alignments": [
    {"step": 1, "log_activity": "Register_Order", "model_transition": "t_register", "type": "synchronous"},
    {"step": 2, "log_activity": "Pay_Invoice", "model_transition": null, "type": "move_on_log"},
    {"step": 3, "log_activity": null, "model_transition": "t_check_inventory", "type": "move_on_model"},
    {"step": 4, "log_activity": "Ship_Goods", "model_transition": "t_ship", "type": "synchronous"}
  ]
}
```

## 3. Leemans 2013: Inductive Miner (Process Trees)

Leemans et al. defined Process Trees as hierarchical trees where leaves are activities and nodes are operators: `->` (sequence), `X` (exclusive choice), `*` (loop), `/\` (concurrent).

### Process Tree JSON Fixture
This tree represents the process: `-> (Register, X (Approve, Reject), Ship)`
```json
{
  "paper_reference": "leemans_2013_inductive_miner",
  "tree_id": "process_tree_01",
  "root": {
    "operator": "sequence",
    "children": [
      { "activity": "Register" },
      {
        "operator": "choice",
        "children": [
          { "activity": "Approve" },
          { "activity": "Reject" }
        ]
      },
      { "activity": "Ship" }
    ]
  }
}
```

## 4. Ghahfarokhi 2021: OCEL 2.0 (Object-Centric Event Logs)

Ghahfarokhi et al. introduced the OCEL 2.0 standard, allowing multiple objects (e.g., orders, items, containers) to interact in a single event.

### OCEL 2.0 Schema and Log Fixture
```json
{
  "paper_reference": "ghahfarokhi_2021_ocel2",
  "log_id": "ma_diligence_log",
  "object_types": [
    {"name": "order", "attributes": [{"name": "total_cost", "type": "float"}]},
    {"name": "item", "attributes": [{"name": "weight", "type": "float"}]}
  ],
  "event_types": [
    {"name": "create_order", "attributes": []},
    {"name": "pack_item", "attributes": [{"name": "package_type", "type": "string"}]}
  ],
  "events": [
    {
      "id": "e001",
      "type": "create_order",
      "timestamp": "2026-05-31T22:44:00Z",
      "objects": ["order_99"],
      "attributes": []
    },
    {
      "id": "e002",
      "type": "pack_item",
      "timestamp": "2026-05-31T22:45:00Z",
      "objects": ["order_99", "item_42", "item_43"],
      "attributes": [{"name": "package_type", "value": "cardboard"}]
    }
  ],
  "objects": [
    {"id": "order_99", "type": "order", "attributes": [{"name": "total_cost", "value": 150.0}]},
    {"id": "item_42", "type": "item", "attributes": [{"name": "weight", "value": 1.2}]},
    {"id": "item_43", "type": "item", "attributes": [{"name": "weight", "value": 0.8}]}
  ]
}
```

## 5. Standard and Claim Maps

- **Standards Crosswalk**: Mapped to standard public specifications in file:///Users/sac/process-intelligence/standards/public_standards_to_ggen_projections.md.
- **M&A Claims**: Defensibility claims of process structure soundness are mapped to the transaction board criteria at file:///Users/sac/process-intelligence/ma/define_board-admissible_claim_requirements.md.