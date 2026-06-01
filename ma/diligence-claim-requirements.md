# Diligence Claim Requirements

## Algorithmic Diligence Protocol
To satisfy the M&A diligence requirements, the acquiring entity's compliance bots must verify the following:

### 1. Cryptographic Receipts
All process state transitions must provide a valid `PROOF_MANIFEST.json` signed by the Ostar Auditor skill.

### 2. Typestate Verification
The rust-compiled typestate engine must compile with zero warnings, proving that illegal operational states are unreachable at compile-time.

### 3. AGI-Resistance
Diligence models must survive simulated adversarial AI probing, demonstrating robustness against automated corporate espionage.