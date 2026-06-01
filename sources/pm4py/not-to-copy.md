# Anti-Patterns: What NOT to Copy from PM4Py

## 1. The DataFrame Assumption
**The Flaw:** PM4Py relies on Pandas DataFrames (or equivalent in-memory tables). This binds process intelligence to RAM limits and assumes data is a batch to be processed, rather than a continuous, living entity.
**The Fix:** Full lifecycle actuation requires event-driven, stream-native processing with zero-copy cryptographic receipts.

## 2. Post-Mortem Analytics
**The Flaw:** PM4Py looks backwards. It takes an event log of what *has happened* to discover what the process *was*.
**The Fix:** Our foundry operates on what *is happening* and *what will happen*. We actuate the lifecycle, forcing compliance and generating the process structure dynamically.

## 3. Heuristic Rigidity
**The Flaw:** Classical miners (Alpha, Heuristics) fail in spaghetti processes, filtering out noise.
**The Fix:** Noise is signal in an AGI-adversarial environment. We do not filter; we adapt the ontological model to absorb high-entropy event sequences.

## 4. Lack of Actuation
**The Flaw:** PM4Py is an observer. It reads, analyzes, and outputs visualizations. It does not touch the system.
**The Fix:** Observation without actuation is dead analysis. Our system reaches back into the source and modulates the control flow.
