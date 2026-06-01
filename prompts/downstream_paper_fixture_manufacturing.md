# Downstream Directive: Paper Fixture Manufacturing

This document defines the rules for translating academic papers from the process mining canon into automated test fixtures. Downstream developers must implement test suites that verify execution engines behave in strict compliance with the theoretical definitions, algorithms, and theorems published in process mining literature.

## 1. Academic Paper-to-Test Mapping Rules
For each paper in the canon, implement a test file checking the core properties:
- **van der Aalst 1998 (Workflow Nets Soundness)**:
  - Implement a test fixture verifying that the soundness check correctly identifies deadlocks and unbounded places in structural models.
  - Assert that a non-sound Petri Net throws a validation error when processed.
- **Adriansyah 2014 (Alignment Conformance)**:
  - Verify that the alignment engine computes the exact minimum-cost alignment trace.
  - Assert cost equivalence: $K(a, \gg) = 1$, $K(\gg, t) = 1$, $K(a, t) = 0$. Test on logs with noise, loops, and duplicate activity labels.
- **Leemans 2013 (Inductive Miner)**:
  - Verify that the Inductive Miner correctly identifies sequence, choice, parallel, and loop cuts in process graphs.
  - Assert that the mined process tree is structurally sound.
- **Ghahfarokhi 2021 (OCEL 2.0)**:
  - Verify that the SQLite and JSON parsers validate object-centric structures correctly, checking that event-to-object relations are mapped with the correct types and properties.

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