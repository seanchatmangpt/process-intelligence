# WASM Boundary Law: Specta + tsify

The `wasm4pm` ecosystem now enforces the Specta-tsify split.

## Law:
- Specta owns the TypeScript Control Surface (DTOs, Commands, Queries, Receipts, Refusals).
- tsify + wasm-bindgen own the WASM ABI Surface.
- serde-wasm-bindgen handles rich value conversion.

## Architecture:
- `wasm4pm-compat` = Internal Rust Law.
- `DTOs` = Projected Boundary Objects.
- Specta = TypeScript Law Declarations.
- tsify/wasm-bindgen = Value Crossing.
