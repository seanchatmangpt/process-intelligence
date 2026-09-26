# Economic ISA → OCEL 2.0 → HDDL/FOND Crosswalk

Status: CANDIDATE research finding

This crosswalk defines the semantic contract for projecting a compact economic activity instruction set into the existing process-intelligence stack without creating a competing event, object, planning, or authority model.

## Research finding

The reusable unit is an **economic verb**, not a complete economic event.

```text
EconomicOpcode + Objects + Relationships + Quantity/Value + Provenance + Authority
  -> admitted OCEL event
  -> process model / conformance evidence
  -> HDDL task or primitive action
  -> FOND policy transition
  -> receipt-bearing consequence
```

The byte therefore identifies only the activity. It MUST NOT encode object identity, counterparty, amount, currency, quantity, legal authority, provenance, observation standing, or execution authority. Those remain explicit structured facts outside the opcode.

This separation preserves OCEL 2.0 object-centric semantics and the OBSERVE / SELECT / CONSTRUCT / DO authority boundary.

## Canonical byte ABI

`0x00` is the admitted UNKNOWN/NULL boundary. `0xFF` is an escape introducing a lossless external semantic identifier. Unassigned bytes are reserved and MUST be refused rather than silently assigned by a downstream projection.

| Range | Category |
|---|---|
| `0x00` | null / unknown |
| `0x01..0x1F` | market |
| `0x20..0x3F` | transaction |
| `0x40..0x5F` | payment / settlement |
| `0x60..0x7F` | logistics |
| `0x80..0x9F` | contract / rights |
| `0xA0..0xBF` | production / service |
| `0xC0..0xDF` | accounting / finance |
| `0xE0..0xEF` | governance / authority |
| `0xF0..0xFE` | extensions |
| `0xFF` | extended-semantic escape |

### Assigned common-path activities

```text
00 unknown
01 quote       02 offer      03 bid          04 ask          05 discover
20 order       21 fill       22 sale         23 purchase     24 return
25 cancel      26 exchange
40 invoice     41 pay        42 settle       43 refund       44 authorize_payment
45 capture_payment
60 ship        61 deliver    62 receive      63 move         64 store
65 consume
80 sign        81 license    82 subscribe    83 renew        84 terminate
85 assign_right
A0 manufacture A1 design     A2 produce      A3 provide_service
A4 prove       A5 inspect    A6 accept
C0 accrue      C1 recognize_revenue          C2 recognize_expense
C3 capitalize  C4 depreciate C5 realize_value C6 allocate
E0 observe     E1 authorize  E2 attest       E3 approve      E4 reject
E5 dispute     E6 resolve
FF extended
```

The executable candidate source for this ABI is `Ex4pm.EconomicISA`; this research document authorizes downstream **projections** only after their exact byte parity is demonstrated. It does not authorize independent extension of the registry.

## OCEL 2.0 projection

A fixed opcode `b` projects to an OCEL-compatible event activity plus explicit attributes:

```text
activity             := canonical_name(b)
economic:opcode       := b
economic:category     := category(b)
economic:semantic_id  := present only for 0xFF escape events
```

The following remain outside the byte and MUST survive projection independently:

- event identity and timestamp;
- object identities and object types;
- E2O qualifier-typed relationships;
- O2O relationships;
- quantity, unit, price, amount, currency, margin, or other economic measurements;
- provenance and observation source;
- authority / principal / delegation facts;
- receipts and replay identity.

Therefore:

```text
same opcode != same event
same event verb != same authority
same observed event != authority to actuate
```

## HDDL projection

HDDL may use an assigned economic opcode as a stable primitive-action identity.

Recommended reversible projection:

```text
0x41 <-> economic:pay
0x61 <-> economic:deliver
0xC1 <-> economic:recognize_revenue
```

Compound HDDL methods decompose business goals into these primitives, but task decomposition MUST NOT grant authority. An HDDL action becomes executable only after the normal authority/admission boundary admits its grounded parameters.

The `0xFF` escape is suitable for a domain-specific primitive only when the original semantic identifier is retained losslessly. It MUST NOT be hashed or mapped into an unassigned one-byte slot.

## FOND projection

FOND treats admitted economic primitives as action labels over nondeterministic transition relations:

```text
pi : S -> A
A subseteq EconomicOpcode x GroundedParameters
```

The opcode identifies the action class. Nondeterministic outcomes remain explicit successor states, for example:

```text
pay -> {settled, declined, disputed, unknown}
deliver -> {accepted, refused, lost, unknown}
```

`0x00 / unknown` is not a fabricated action outcome and MUST NOT be promoted to a successful terminal state. It preserves the observation boundary and can route to further sensing, model refinement, or a typed refusal.

A FOND policy may choose an action; policy selection is not DO authority.

## Process-mining / economic-value projection

The compact verb creates a common join key between event evidence and economic consequence:

```text
opcode
  -> OCEL trace
  -> process instance / object graph
  -> transaction or ledger object
  -> attributable revenue / cost / margin fact
  -> receipt
```

This enables token/model/workflow costs to be attached to the same object-centric graph as downstream revenue without claiming causality merely from correlation. Causal attribution remains a separate evidence requirement.

## Required downstream parity gates

Every implementation projection MUST prove all of the following before claiming ABI compatibility:

1. Every assigned common-path byte decodes to the exact canonical activity above.
2. Encoding then decoding is identity-preserving for all assigned bytes.
3. `0x00` remains UNKNOWN/NULL and is never redefined locally.
4. `0xFF || semantic_id` round-trips the semantic identifier losslessly.
5. Unassigned bytes are rejected/refused, not dynamically interpreted.
6. Category boundaries are byte-identical.
7. Objects, values, provenance, and authority are not collapsed into the byte.
8. A planning projection does not acquire execution authority by being a plan/policy.

Cross-language projections SHOULD consume a generated registry/fixture rather than hand-maintaining independent tables. Until generation is installed, exact parity fixtures are mandatory.

## Downstream projection responsibilities

| Repository / layer | Authorized responsibility | Explicit non-responsibility |
|---|---|---|
| `ex4pm` | canonical semantic registry and OCEL event projection | external actuation |
| `beam4pm` | BEAM/process-mining/planning projection | redefining byte meanings |
| `wasm4pm-compat` | Rust type-law and ABI witness | business authority |
| `wasm4pm` | execution/runtime codec and process evidence | semantic invention |
| `avmnif-rs` / AtomVM | no-std edge codec | ledger truth |
| `ex4pm-plan` | HDDL/FOND action/policy identity | DO authority |
| `ggen` / marketplace | manufacture projections and parity fixtures | independent source of semantics |
| Ash projections | resource/query interface over admitted semantics | second event ontology |

## Falsifiers

This crosswalk is falsified by any implementation that:

- maps the same byte to two common-path meanings;
- loses an extended semantic identifier across a round trip;
- interprets an unassigned byte as a valid action;
- cannot reconstruct the OCEL object/event relationships because they were packed into the verb;
- allows a planner or model to convert SELECT into DO without an authority transition;
- claims economic value from opcode occurrence alone without a linked value consequence and evidence.

## Standing

**CANDIDATE.** The semantics are specified and have an executable candidate in `ex4pm`, but ecosystem-wide ALIVE standing requires exact-head parity evidence from each projection plus end-to-end replay from byte -> OCEL event -> planning policy -> execution receipt -> economic consequence.