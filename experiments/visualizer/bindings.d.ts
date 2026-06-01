/* Generated compile-time TypeScript type definitions (WASM Boundary Law) */

/**
 * TypeScript projection for the core Evidence typestate wrapper.
 */
export type EvidenceTs<T, State, Witness> = { value: T; _state: State; _witness: Witness }

/**
 * TypeScript projection for the core EvidenceState lifecycle tokens.
 */
export type EvidenceState = "Raw" | "Parsed" | "Admitted" | "Refused" | "Projected" | "Exportable" | "Receipted"

/**
 * TypeScript projection for the Witness markers.
 */
export type WitnessKey = "Ocel20" | "Xes1849" | "WfNetSoundnessPaper" | "Dec20" | "Pmax24"

/**
 * TypeScript projection for the Admitted typestate boundary.
 */
export type AdmissionTs<T, Witness> = { value: T; admitted_at_ns: number; _witness: Witness }

/**
 * TypeScript projection for the Refused typestate boundary.
 */
export type RefusalTs<Reason, Witness> = { law_name: string; message: string; _reason: Reason; _witness: Witness }

/**
 * TypeScript projection for the LossPolicy options.
 */
export type LossPolicyTs = "RefuseLoss" | "AllowNamedProjection" | "AllowLossWithReport"

/**
 * TypeScript projection for the LossReport structure.
 */
export type LossReportTs<From, To, Items> = { projection_name: string; policy: LossPolicyTs; items_dropped: Items; _from: From; _to: To }

/**
 * TypeScript projection for the ReceiptShape.
 */
export type ReceiptShapeTs = { case_id: string; process_hash: string; parent_block_hash: string; block_hash: string; timestamp_ns: number; fitness: number }

/**
 * TypeScript projection for the GraduationCandidate.
 */
export type GraduationCandidateTs = { reason: string; subject: string; evidence_ref: string }

/**
 * Branding tag for OCEL logs in the TypeScript type court.
 */
export type OcelBrand = null

/**
 * Branding tag for XES logs in the TypeScript type court.
 */
export type XesBrand = null

/**
 * Branding tag for WF-Net soundness in the TypeScript type court.
 */
export type WfNetBrand = null

/**
 * Structure representing a process witness across the WASM boundary.
 */
export type WasmWitness = { key: string; title: string; year: number | null }

/**
 * Structure representing a typestate lifecycle state across the WASM boundary.
 */
export type WasmStateTag = { name: string; is_terminal: boolean }

/**
 * Structure representing structural admission and Refusal checks.
 */
export type WasmAdmissionResult = { is_ok: boolean; refusal_law: string | null; refusal_message: string | null }

/**
 * Structure representing a graduation candidate requesting execution power.
 */
export type WasmGraduationCandidate = { reason: string; subject: string; evidence_ref: string }

/**
 * Structure representing a complete process evidence envelope for roundtrip simulation.
 */
export type WasmProcessEvidence = { case_id: string; events: string[]; timestamp_ns: number; parent_block_hash: string; block_hash: string; state: string; witness_key: string; is_valid: boolean }