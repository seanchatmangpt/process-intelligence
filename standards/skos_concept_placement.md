# SKOS Concept Standard Ledger Placement

The **Simple Knowledge Organization System (SKOS)** is the W3C standard for representing taxonomies, thesauri, and classification schemes. In the Process Intelligence Research Foundry, SKOS concepts are used to define activity taxonomies and hierarchy mappings. This allows low-level events (e.g., "SQL Update Table X") to be mapped to high-level process activities (e.g., "Approve Invoice") in a standardized manner. This document establishes how SKOS hierarchies are registered, verified, and mapped on the ledger.

---

## 1. Ontological Mapping to the Ledger

SKOS models classification schemes using **Concept Schemes**, **Concepts**, and semantic relationships. The foundry maps these elements to ledger tables:

| SKOS Property | Process Mining Concept | Ledger Table | Description |
| :--- | :--- | :--- | :--- |
| `skos:ConceptScheme`| **Activity Taxonomy** | `ConceptScheme` | Represents the process taxonomy container (e.g., "AP Taxonomy"). |
| `skos:Concept` | **Activity Category / Level**| `ConceptNode` | Represents an activity label at a specific abstraction level. |
| `skos:prefLabel` | **Activity Name** | `ConceptLabel` | The preferred name of the activity in a given language. |
| `skos:broader` | **Parent Category** | `BroaderRelation` | Maps a low-level concept to a broader, high-level process step. |
| `skos:narrower` | **Sub-activities** | `NarrowerRelation`| Maps a high-level process step to its low-level sub-activities. |

The ledger registers each SKOS taxonomy in a concept scheme table:

```json
{
  "scheme_id": "skos-100e8400-e29b-41d4-a716-446655444444",
  "title": "Procure-to-Pay Activity Taxonomy",
  "concepts_count": 89,
  "concept_scheme_hash": "c2b3a4...",
  "witness_signature": "SIG_ED25519_..."
}
```

---

## 2. Type Laws and Abstraction Constraints

To ensure semantic consistency during log abstraction:

1.  **Hierarchy Acyclicity**: The `skos:broader` relationship must form a directed acyclic graph (DAG):
    $$\forall c \in \text{Concepts}, \quad (c, c) \notin \operatorname{broader}^*$$
2.  **Deterministic Abstraction**: The translation of low-level event labels to high-level activities must be a deterministic function defined by the registered SKOS scheme:
    $$\operatorname{abstract}(e_{\text{low}}, \text{Scheme}) \equiv e_{\text{high}}$$
3.  **No Orphan Concepts**: Every concept registered in a scheme must be transitively linked to the top-level concept of that scheme.

---

## 3. Academic Foundations and Conformance

*   SKOS provides a standardized way to handle multi-level process mining.
*   For the autonomic adjustments of concepts, see [Autonomic Knowledge Actuation](file:///Users/sac/process-intelligence/doctrine/autonomic-knowledge-actuation.md).
*   For the general rules of compliance, see the [Public Standards Gravity](file:///Users/sac/process-intelligence/doctrine/public-standards-gravity.md).

---

## 4. M&A Slide-to-Receipt Bridge

To verify abstraction integrity during due diligence:
1.  All activity taxonomies used to aggregate low-level events for presentation slides must map to a registered SKOS scheme.
2.  The SKOS scheme hash is registered in the [Slide-to-Receipt Map](file:///Users/sac/process-intelligence/ma/define_slide-to-receipt_map.md).
3.  The buyer due diligence team verifies that the activity abstraction is mathematically sound and consistent across all logs, matching the standards in [Buyer Reliance Requirements](file:///Users/sac/process-intelligence/ma/define_buyer_reliance_requirements.md).

---

## Section 18: Conformance Metrics (v30.1.1 Spec)

The four quality dimensions (fitness, precision, generalization, simplicity) and F1 score are represented as rational numbers $\frac{p}{q} \in [0, 1]$.
This is compile-enforced via the where-bounds:
$$\text{Metric}\langle\text{const KIND: QualityMetricKind, const NUM: u64, const DEN: u64}\rangle \quad \text{where} \quad \text{Between01}\langle\text{NUM, DEN}\rangle: \text{IsTrue}$$
This ensures that no invalid conformance score (e.g. fitness > 1.0 or division-by-zero) can compile, establishing a static safety boundary for all reported metrics.