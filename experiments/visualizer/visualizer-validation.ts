import {
  EvidenceTs,
  EvidenceState,
  WitnessKey,
  AdmissionTs,
  RefusalTs,
  LossPolicyTs,
  LossReportTs,
  ReceiptShapeTs,
  GraduationCandidateTs,
  OcelBrand,
  XesBrand,
  WfNetBrand
} from "./bindings";

// 1. Validate EvidenceState lifecycle tokens
const rawState: EvidenceState = "Raw";
const admittedState: EvidenceState = "Admitted";
const refusedState: EvidenceState = "Refused";
console.log("EvidenceState validation: OK", { rawState, admittedState, refusedState });

// 2. Validate WitnessKey markers
const ocelWitness: WitnessKey = "Ocel20";
const xesWitness: WitnessKey = "Xes1849";
const wfnetWitness: WitnessKey = "WfNetSoundnessPaper";
console.log("WitnessKey validation: OK", { ocelWitness, xesWitness, wfnetWitness });

// 3. Validate OcelBrand, XesBrand, and WfNetBrand type parameters
const ocelBrand: OcelBrand = null;
const xesBrand: XesBrand = null;
const wfnetBrand: WfNetBrand = null;
console.log("Branding type validation: OK", { ocelBrand, xesBrand, wfnetBrand });

// 4. Validate EvidenceTs shape-level projection
const testEvidence: EvidenceTs<string, EvidenceState, WitnessKey> = {
  value: '{"events": []}',
  _state: "Admitted",
  _witness: "Ocel20"
};
console.log("EvidenceTs Type Alignment: OK", testEvidence);

// 5. Validate AdmissionTs shape-level projection
const testAdmission: AdmissionTs<string, WitnessKey> = {
  value: '{"events": []}',
  admitted_at_ns: 1770000000000000000,
  _witness: "Ocel20"
};
console.log("AdmissionTs Type Alignment: OK", testAdmission);

// 6. Validate RefusalTs shape-level projection
const testRefusal: RefusalTs<string, WitnessKey> = {
  law_name: "MissingFinalMarking",
  message: "Final marking must have 1 token in end place",
  _reason: "EmptyMarking",
  _witness: "WfNetSoundnessPaper"
};
console.log("RefusalTs Type Alignment: OK", testRefusal);

// 7. Validate LossPolicyTs options
const refuseLossPolicy: LossPolicyTs = "RefuseLoss";
const allowReportPolicy: LossPolicyTs = "AllowLossWithReport";
console.log("LossPolicyTs validation: OK", { refuseLossPolicy, allowReportPolicy });

// 8. Validate LossReportTs shape-level projection
const testLossReport: LossReportTs<XesBrand, OcelBrand, string[]> = {
  projection_name: "xes-to-ocel",
  policy: "AllowLossWithReport",
  items_dropped: ["concept:name"],
  _from: null,
  _to: null
};
console.log("LossReportTs Type Alignment: OK", testLossReport);

// 9. Validate ReceiptShapeTs shape-level projection
const testReceipt: ReceiptShapeTs = {
  case_id: "C-1002",
  process_hash: "blake3:4a7b744ce58b88cd28148b5dfbe984f932e650b2a8f98db832cdde32bbd42a9d",
  parent_block_hash: "0000000000000000000000000000000000000000000000000000000000000000",
  block_hash: "81f7dca25ba3594074888c74547b0e70796a2082f9cda3b2c12a843e620581ba9",
  timestamp_ns: 1770000000000000000,
  fitness: 0.982
};
console.log("ReceiptShapeTs Type Alignment: OK", testReceipt);

// 10. Validate GraduationCandidateTs shape-level projection
const testGraduationCandidate: GraduationCandidateTs = {
  reason: "needs_discovery",
  subject: "p2p_log",
  evidence_ref: "blake3:abc123"
};
console.log("GraduationCandidateTs Type Alignment: OK", testGraduationCandidate);
