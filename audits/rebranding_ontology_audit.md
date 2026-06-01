# Ontology Rebranding Audit

## Findings
The `geen` ontology files in `~/process-intelligence` contain numerous references to 'ZOEapp', 'zoeapp', and '/Users/sac/zoeapp'. These must be updated to reflect the new 'PCP' (PostCyberPuno) framework extraction.

## Required Changes

1.  **Project Identifiers**: Replace `proj:ZOEAPP` with `proj:PCP_PROOF_CELL` (or similar).
    -  `~2/process-intelligence/research/pi-program/ggen/ontology/pi-ggen-project-registry.ttl`
    -  145: `# 6. ZOEAPP (PROOF_CELL) -> # 6. PCP_PROOF_CELL

2.  **Path References**: Replace `file:///Users/sac/zoeapp` with `file:///Users/sac/pcp` (or the actual extracted path).
    -  `~/process-intelligence/research/pi-program/ggen/ontology/pi-ggen-project-registry.ttl` (Line 153)
    -  `~/process-intelligence/research/pi-program/ggen/ontology/checkpoint-ledger.ttl` (Line 245)
    -  `~/process-intelligence/research/pi-program/ggen/ontology/research-artifact-ledger.ttl` (Line 191)

3.  **URIs/INIs**: Replace zoeapp in Agent IRIs and Program IRIs.
    -  `https://process.intelligence/agent/zoeapp-census` (`project-registry.ttl` Line 56) -> `https://process.intelligence/agent/pcp-census`
    -  `https://pi-research.dev/programs#ZOEAPP_RESEARCH_PROGRAM_INTEL_001` (prompt-manufactory)

4.  **Descriptions/"Zoe" Branding**: Update labels and descriptions.
    -  "Zoe Community Church mobile application" -> "PCP Proof Cell Application"
    -  "ZOEapp" -> "PCP Proof Cell"
    (Multiple occurrences in `project-registry.ttl`, `conformance-ledger.ttl`, and `pi-program.ttl`)

## Conclusion
The ontology files strongly tie the proof cell to the Zoeapp name. To extract the framework as 'PCP', a sedreplacement or projection update of these .ttl files is required to maintain ontological integrity.