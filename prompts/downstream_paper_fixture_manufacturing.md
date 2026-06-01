# Downstream Directive: Paper Fixture Manufacturing

This document defines the rules for translating academic papers from the process mining canon into automated test fixtures. Downstream developers must implement test suites that verify execution engines behave in strict compliance with the theoretical definitions, algorithms, and theorems published in process mining literature.

## 1. Academic Paper-to-Test Mapping Rules
For each paper in the canon, implement a test file checking the core properties:

### van der Aalst 1998 (Workflow Nets Soundness)
- **Mathematical Formulations**:
  A Workflow Net ($WF$-net) $N = (P, T, F)$ must satisfy:
  1. $\exists! i \in P \text{ s.t. } \bullet i = \emptyset$ (unique source place).
  2. $\exists! o \in P \text{ s.t. } o \bullet = \emptyset$ (unique sink place).
  3. Every node $n \in P \cup T$ lies on a directed path from $i$ to $o$.
  
  $N$ is **sound** if and only if:
  1. Option to complete: $\forall M \in [i\rangle, \exists \sigma \in T^* \text{ s.t. } M \xrightarrow{\sigma} [o]$.
  2. Proper completion: $\forall M \in [i\rangle, M(o) \ge 1 \implies M = [o]$.
  3. No dead transitions: $\forall t \in T, \exists M \in [i\rangle \text{ s.t. } M \xrightarrow{t}$.
- **Verification Rule**: Implement a test fixture verifying that the soundness check correctly identifies deadlocks and unbounded places in structural models. Assert that a non-sound Petri Net throws a validation error when processed.

### Adriansyah 2014 (Alignment Conformance)
- **Mathematical Formulations**:
  Let $\sigma \in \Sigma^*$ be an observed trace and $N$ be the process model. Define alignment cost function $K$ where:
  - Log-only move: $K(a, \gg) = 1$ for all $a \in \Sigma$.
  - Model-only move: $K(\gg, t) = \begin{cases} 0 & \text{if } t \text{ is a silent transition } \tau \\ 1 & \text{otherwise} \end{cases}$.
  - Synchronous move: $K(a, t) = \begin{cases} 0 & \text{if } \text{label}(t) = a \\ \infty & \text{otherwise} \end{cases}$.
  
  The optimal alignment minimizes the total cost. Conformance fitness of trace $\sigma$ is:
  $$\operatorname{Fitness}(\sigma, N) = 1 - \frac{\operatorname{cost}^*(\sigma, N)}{\operatorname{cost}^*(\sigma, \text{empty\_model}) + \operatorname{cost}^*(\text{empty\_log}, N)}$$
- **Verification Rule**: Verify that the alignment engine computes the exact minimum-cost alignment trace. Assert cost equivalence. Test on logs with noise, loops, and duplicate activity labels.

### Leemans 2013 (Inductive Miner)
- **Verification Rule**: Verify that the Inductive Miner correctly identifies sequence, choice, parallel, and loop cuts in process graphs. Assert that the mined process tree is structurally sound.

### Ghahfarokhi 2021 (OCEL 2.0)
- **Verification Rule**: Verify that the SQLite and JSON parsers validate object-centric structures correctly, checking that event-to-object relations are mapped with the correct types and properties.

## 2. Test Fixture Structure
Every paper test fixture must:
1. Cite the paper (Author, Year, Title).
2. Define the formal theorem or algorithm being tested.
3. Contain positive test cases (proving conformance to the theory).
4. Contain negative test cases (proving that violations are correctly caught and handled).

## 3. Downstream Integration and Traceability
All implementation details must align with:
- [paper-canon.md](file:///Users/sac/process-intelligence/sources/papers/paper-canon.md)
- [paper-to-fixture_mapping_sample.md](file:///Users/sac/process-intelligence/experiments/paper-to-fixture_mapping_sample.md)