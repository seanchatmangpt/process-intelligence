
export const meta = {
  name: 'phd-thesis-corpus-manufacture',
  description: 'Manufacture Sean Chatman PhD thesis corpus from ~/process-intelligence — discover projects, fan-out 8 TeX files per project, condense master dissertation, compile PDF',
  phases: [
    { title: 'Setup', detail: 'Git branch, ledger init, tool check' },
    { title: 'Discover', detail: 'Crawl project tree and classify all meaningful projects' },
    { title: 'Analyze', detail: 'Per-project corpus cartography and evidence audit' },
    { title: 'Manufacture', detail: 'Write exactly 8 TeX files + claim ledger per project' },
    { title: 'Referee', detail: 'Claim referee: downgrade unsupported claims' },
    { title: 'Condense', detail: 'Synthesize master 12-chapter dissertation' },
    { title: 'Compile', detail: 'LaTeX build + PDF hash receipt' },
    { title: 'Validate', detail: 'Gate checks: ALIVE / PARTIAL / BLOCKED' },
  ],
}

const ROOT = '/Users/sac/process-intelligence'
const THESIS = `${ROOT}/phd-thesis`
const BRANCH = 'phd-thesis-corpus-manufacture-001'

// ─── Schemas ────────────────────────────────────────────────────────────────

const PROJECT_LIST_SCHEMA = {
  type: 'object',
  properties: {
    projects: {
      type: 'array',
      items: {
        type: 'object',
        properties: {
          slug: { type: 'string' },
          absolute_path: { type: 'string' },
          description: { type: 'string' },
          detected_languages: { type: 'array', items: { type: 'string' } },
          detected_frameworks: { type: 'array', items: { type: 'string' } },
          detected_research_surfaces: { type: 'array', items: { type: 'string' } },
          likely_thesis_role: { type: 'string' },
          key_files: { type: 'array', items: { type: 'string' } },
          readme_present: { type: 'boolean' },
          receipt_present: { type: 'boolean' },
          ontology_present: { type: 'boolean' },
        },
        required: ['slug', 'absolute_path', 'description', 'likely_thesis_role'],
      },
    },
  },
  required: ['projects'],
}

const EVIDENCE_SCHEMA = {
  type: 'object',
  properties: {
    slug: { type: 'string' },
    source_files: { type: 'array', items: { type: 'string' } },
    key_primitives: { type: 'array', items: { type: 'string' } },
    architecture_summary: { type: 'string' },
    evidence_summary: { type: 'string' },
    alive_status: { type: 'string', enum: ['ALIVE', 'PARTIAL', 'BLOCKED', 'UNKNOWN'] },
    receipts_found: { type: 'array', items: { type: 'string' } },
    tests_found: { type: 'array', items: { type: 'string' } },
    checkpoints_found: { type: 'array', items: { type: 'string' } },
    ontology_surfaces: { type: 'array', items: { type: 'string' } },
    open_questions: { type: 'array', items: { type: 'string' } },
  },
  required: ['slug', 'architecture_summary', 'evidence_summary', 'alive_status'],
}

const CLAIM_LIST_SCHEMA = {
  type: 'object',
  properties: {
    slug: { type: 'string' },
    claims: {
      type: 'array',
      items: {
        type: 'object',
        properties: {
          id: { type: 'string' },
          claim: { type: 'string' },
          claim_type: { type: 'string', enum: ['SOURCE_SUPPORTED', 'AUTHOR_THESIS', 'INTERPRETATION', 'FUTURE_WORK', 'UNSUPPORTED_REMOVED'] },
          source_files: { type: 'array', items: { type: 'string' } },
          evidence_summary: { type: 'string' },
          confidence: { type: 'string', enum: ['HIGH', 'MEDIUM', 'LOW'] },
          allowed_in_pdf: { type: 'boolean' },
        },
        required: ['id', 'claim', 'claim_type', 'allowed_in_pdf'],
      },
    },
  },
  required: ['slug', 'claims'],
}

// ─── Phase 0: Setup ──────────────────────────────────────────────────────────
phase('Setup')

const setupResult = await agent(
  `You are setting up the PhD thesis workspace at ${THESIS} inside the git repository at ${ROOT}.

TASKS:
1. Run: cd ${ROOT} && git checkout -b ${BRANCH} 2>/dev/null || git checkout ${BRANCH}
2. Create directory structure:
   mkdir -p ${THESIS}/{frontmatter,chapters,projects,ledgers,scripts,build}
3. Check tool versions: pdflatex --version, python3 --version, git --version, rg --version
4. Get current git commit: cd ${ROOT} && git rev-parse HEAD
5. Write ${THESIS}/ledgers/WORKFLOW_RECEIPT.yaml with fields:
   workflow_name: phd-thesis-corpus-manufacture-001
   branch: ${BRANCH}
   root: ${ROOT}
   thesis_output: ${THESIS}
   working_directory: ${ROOT}
   git_commit_before: [actual commit hash]
   tools_available: [pdflatex, python3, git, rg versions]
   phase: SETUP_COMPLETE
6. Write ${THESIS}/ledgers/DO_NOT_CLAIM_LEDGER.md with:
   # DO NOT CLAIM LEDGER
   ## Absolute Prohibitions
   - Do not claim production deployment unless receipt proves it
   - Do not use the word "unhackable"
   - Do not fabricate claims without source artifact backing
   - Do not call research "generated" — use manufactured, emitted, materialized
   - Do not name-drop celebrity thought leaders
   - Do not say "semantic" unless quoting a source title
   - ALIVE verdicts require: TeX files exist + source ledger + claim ledger + PDF compiles + hash recorded
7. Write ${THESIS}/README.md explaining this is a dissertation corpus manufacturing workspace

Return the git commit hash and confirmation that all directories were created.`,
  { label: 'setup:workspace', phase: 'Setup' }
)

log(`Setup complete: ${setupResult}`)

// ─── Phase 1: Project Discovery ─────────────────────────────────────────────
phase('Discover')

const discoveryResult = await agent(
  `You are the Corpus Cartographer for Sean Chatman's PhD thesis manufacturing run.

MISSION: Crawl ${ROOT} and identify every meaningful research project.

A meaningful project is any directory containing one or more of:
  Cargo.toml, package.json, pyproject.toml, go.mod, Makefile,
  README.md, docs/, ontology/, ontologies/, *.ttl, *.rq, *.tera,
  .ggen/, receipts/, checkpoints/, src/, papers/, thesis/, research/,
  diagrams/, architecture/

IGNORE: node_modules/, target/, dist/, build/, .next/, .expo/, .git/, vendor/,
tmp/, cache/, .venv/, __pycache__/, coverage/, .agents/

START PATH: ${ROOT}

COMMANDS to run:
  find ${ROOT} -maxdepth 4 -type d | grep -v -E '(node_modules|target|dist|/build|\.next|\.expo|\.git|vendor|tmp|cache|\.venv|__pycache__|coverage|\.agents)' 
  ls ${ROOT}
  ls ${ROOT}/sources/
  ls ${ROOT}/research/
  ls ${ROOT}/experiments/
  ls ${ROOT}/doctrine/
  ls ${ROOT}/ggen/

For each discovered project, identify:
- slug (kebab-case unique name)
- absolute_path
- description (what this project IS)
- detected_languages (Rust, Python, TypeScript, SPARQL, Turtle, Tera, etc.)
- detected_frameworks (pm4py, wasm4pm, ggen, otel-weaver, etc.)
- detected_research_surfaces (ontology, receipts, process-evidence, command-grammar, etc.)
- likely_thesis_role (e.g., "core-evidence-engine", "ontology-layer", "command-grammar-surface", etc.)
- key_files (up to 5 most important files)
- readme_present (boolean)
- receipt_present (boolean)
- ontology_present (boolean)

The thesis lineage is:
2016 language-model experiment → Chatman Equation A=μ(O*) → receipts/replay →
ggen/Open Ontologies → clap-noun-verb command grammar → wasm4pm/process-evidence →
Post-Cyberpunk PCP → AI XYNZ → capital-flow/settlement/DAO → industry-complete architecture

Key known projects to look for:
- blue_river_dam (Rust — capital-flow, settlement, post-quantum)
- ggen (graph-query-template manufacturing engine)
- otel-weaver (OTel semantic convention mapping)
- research/open-ontologies (RDF ontology research)
- research/pi-program (process intelligence program governance)
- research/prompt-manufactory (prompt manufacturing research)
- sources/papers (academic paper analysis)
- sources/pm4py (PM4Py capability atlas)
- sources/wasm4pm (wasm4pm execution authority map)
- sources/wasm4pm-compat (type law atlas and witness lattices)
- doctrine (process law definitions — treat as a corpus project)
- experiments (benchmark experiments — treat as a corpus project)
- comparisons (cross-system comparisons)
- standards (public standards compliance maps)
- ma (M&A claim taxonomy)
- gaps (structural gap documentation)
- lifecycle (process lifecycle state definitions)
- crosswalks (type-law crosswalk mappings)
- adversarial (adversarial test cases)
- checkpoints (phase milestone verdicts)

After discovery, write ${THESIS}/ledgers/PROJECT_INDEX.yaml with all found projects.

Return the structured list of ALL discovered projects.`,
  { label: 'discover:projects', phase: 'Discover', schema: PROJECT_LIST_SCHEMA }
)

const projects = discoveryResult.projects
log(`Discovered ${projects.length} projects: ${projects.map(p => p.slug).join(', ')}`)

// Write project manifests
await agent(
  `Write project_manifest.yaml files for each of these discovered projects.
For each project, create ${THESIS}/projects/<slug>/project_manifest.yaml.

Projects: ${JSON.stringify(projects, null, 2)}

For each project create the directory and write the manifest YAML with all fields from the project data.
Also write ${THESIS}/ledgers/PROJECT_INDEX.yaml listing all projects with their paths and thesis roles.

Use Bash to create the directories and write files.
Example for each project:
  mkdir -p ${THESIS}/projects/<slug>
  Write the manifest file.

Return: "Manifests written for N projects"`,
  { label: 'discover:write-manifests', phase: 'Discover' }
)

// ─── Phase 2+3+4: Per-Project Analysis → TeX → Claim Referee ─────────────
phase('Analyze')

const projectResults = await pipeline(
  projects,

  // Stage 1: Evidence extraction (Corpus Cartographer + Architecture Extractor + Evidence Auditor)
  async (project) => {
    const evidence = await agent(
      `You are THREE specialized agents combined: Corpus Cartographer, Architecture Extractor, and Evidence Auditor.

PROJECT: ${project.slug}
PATH: ${project.absolute_path}
THESIS ROLE: ${project.likely_thesis_role}
DESCRIPTION: ${project.description}

TASK: Deeply read this project and extract all evidence needed to write a dissertation chapter.

Run these commands to understand the project:
  ls -la "${project.absolute_path}" 2>/dev/null || echo "PATH_NOT_FOUND"
  find "${project.absolute_path}" -maxdepth 3 -type f | grep -v -E '(node_modules|target|\.git|__pycache__|\.pyc)' | head -50
  find "${project.absolute_path}" -name "*.md" -maxdepth 3 | head -10
  find "${project.absolute_path}" -name "*.ttl" -maxdepth 3 | head -10
  find "${project.absolute_path}" -name "*.yaml" -o -name "*.yml" -maxdepth 3 | head -10
  find "${project.absolute_path}" -name "*.toml" -maxdepth 3 | head -10

Then read the most important files:
- README.md if present
- Any CLAUDE.md, COVENANT.md, doctrine files
- Cargo.toml or package.json if present
- Up to 3 key source files
- Any receipt, checkpoint, or ALIVE/PARTIAL files
- Any .ttl, .rq, or .tera files

Extract:
- source_files: list of files you read
- key_primitives: core objects, types, algorithms, commands
- architecture_summary: 2-3 sentences on what this IS
- evidence_summary: what proves this project exists and works
- alive_status: ALIVE if tests+receipts+checkpoints exist, PARTIAL if some evidence, BLOCKED if mostly theoretical, UNKNOWN if unclear
- receipts_found: list of receipt files
- tests_found: list of test files
- checkpoints_found: list of checkpoint files  
- ontology_surfaces: list of ontology/TTL/RDF files
- open_questions: 3-5 things that remain unclear or unfinished

The thesis lineage this must connect to:
"2016 LM experiment → Chatman Equation A=μ(O*) → receipts → ggen → wasm4pm → Post-Cyberpunk PCP → AI XYNZ → capital-flow → industry-complete"

Return structured evidence for project: ${project.slug}`,
      { label: `analyze:${project.slug}`, phase: 'Analyze', schema: EVIDENCE_SCHEMA }
    )
    return { project, evidence }
  },

  // Stage 2: Write exactly 8 TeX files per project
  async ({ project, evidence }, originalProject, idx) => {
    phase('Manufacture')
    const tex = await agent(
      `You are the Thesis Writer. Write exactly 8 LaTeX files for this project.

PROJECT SLUG: ${project.slug}
OUTPUT DIR: ${THESIS}/projects/${project.slug}/
PROJECT PATH: ${project.absolute_path}
DESCRIPTION: ${project.description}
THESIS ROLE: ${project.likely_thesis_role}

EVIDENCE EXTRACTED:
Architecture: ${evidence.architecture_summary}
Evidence: ${evidence.evidence_summary}
Status: ${evidence.alive_status}
Key Primitives: ${(evidence.key_primitives || []).join(', ')}
Source Files: ${(evidence.source_files || []).join(', ')}
Receipts: ${(evidence.receipts_found || []).join(', ')}
Tests: ${(evidence.tests_found || []).join(', ')}
Open Questions: ${(evidence.open_questions || []).join(', ')}

ABSOLUTE LAWS:
1. Every claim must be backed by a local artifact, file, commit, or marked AUTHOR THESIS / INTERPRETATION / FUTURE WORK
2. Never use the word "semantic" unless quoting a source title
3. Never say "generated" — use manufactured, emitted, materialized, forged
4. Never claim ALIVE unless receipt+tests+checkpoint all exist
5. Do not name-drop celebrity AI thought leaders

THESIS LINEAGE: 
2016 LM experiment → local text imitation does not conserve consequence →
enterprise process gap → Chatman Equation A=μ(O*) → receipts and replay →
ggen/Open Ontologies → clap-noun-verb command grammar → wasm4pm/process-evidence →
Post-Cyberpunk PCP → AI XYNZ → capital-flow/settlement/DAO → industry-complete architecture

REQUIRED FILES — write ALL 8:

FILE 1: ${THESIS}/projects/${project.slug}/00_project_identity.tex
\\section{Project Identity}
\\subsection{Purpose}
\\subsection{Repository Surface}
\\subsection{Primary Research Role}
\\subsection{Key Artifacts}

FILE 2: ${THESIS}/projects/${project.slug}/01_research_question.tex
\\section{Research Question}
\\subsection{Problem Statement}
\\subsection{Old-Regime Assumption Challenged}
\\subsection{Hypothesis}
\\subsection{Success Criteria}

FILE 3: ${THESIS}/projects/${project.slug}/02_lineage_and_context.tex
\\section{Lineage and Context}
\\subsection{2016 Language-Model Lesson}
\\subsection{Enterprise Process Gap}
\\subsection{Relationship to the Chatman Equation}
\\subsection{Relationship to Receipts and Replay}

FILE 4: ${THESIS}/projects/${project.slug}/03_architecture_and_primitives.tex
\\section{Architecture and Primitives}
\\subsection{Core Objects}
\\subsection{Admissible Transitions}
\\subsection{Boundaries}
\\subsection{Failure Modes Refused}

FILE 5: ${THESIS}/projects/${project.slug}/04_implementation_surface.tex
\\section{Implementation Surface}
\\subsection{Source Files}
\\subsection{Commands}
\\subsection{Data and Ontology Surfaces}
\\subsection{Templates and Rendered Artifacts}

FILE 6: ${THESIS}/projects/${project.slug}/05_evidence_receipts_and_gates.tex
\\section{Evidence, Receipts, and Gates}
\\subsection{Tests}
\\subsection{Receipts}
\\subsection{Checkpoints}
\\subsection{ALIVE/PARTIAL/BLOCKED Status}
Note: Status is ${evidence.alive_status} — reflect this accurately.

FILE 7: ${THESIS}/projects/${project.slug}/06_contribution_to_thesis.tex
\\section{Contribution to the Dissertation}
\\subsection{Contribution to Prediction vs.~Coordination}
\\subsection{Contribution to Process Evidence}
\\subsection{Contribution to Manufacturing Architecture}
\\subsection{Contribution to Capital-Flow Infrastructure}

FILE 8: ${THESIS}/projects/${project.slug}/07_open_questions_and_next_work.tex
\\section{Open Questions and Next Work}
\\subsection{Known Gaps}
\\subsection{Deferred Gates}
\\subsection{Research Risks}
\\subsection{Next Receipted Work}

Also write:
- ${THESIS}/projects/${project.slug}/source_index.md (list all source files read with 1-line description each)

Write all files using the Write tool. Use proper LaTeX with \\label{sec:${project.slug}-*} tags.
The LaTeX content should be substantive (100-300 words per subsection) and use only evidence you extracted.

Return: "8 TeX files written for ${project.slug}"`,
      { label: `manufacture:${project.slug}`, phase: 'Manufacture' }
    )
    return { project, evidence, tex }
  },

  // Stage 3: Claim Referee — extract and validate claims, write claim_ledger.yaml
  async ({ project, evidence }, originalProject, idx) => {
    phase('Referee')
    const claimResult = await agent(
      `You are the Claim Referee. Review the 8 TeX files for project "${project.slug}" and produce a claim ledger.

READ THESE FILES:
  ${THESIS}/projects/${project.slug}/00_project_identity.tex
  ${THESIS}/projects/${project.slug}/01_research_question.tex
  ${THESIS}/projects/${project.slug}/02_lineage_and_context.tex
  ${THESIS}/projects/${project.slug}/03_architecture_and_primitives.tex
  ${THESIS}/projects/${project.slug}/04_implementation_surface.tex
  ${THESIS}/projects/${project.slug}/05_evidence_receipts_and_gates.tex
  ${THESIS}/projects/${project.slug}/06_contribution_to_thesis.tex
  ${THESIS}/projects/${project.slug}/07_open_questions_and_next_work.tex

REFEREE RULES:
- SOURCE_SUPPORTED: Claim backed by a specific file at ${project.absolute_path}
- AUTHOR_THESIS: Claim is the author's analytical conclusion, marked as such
- INTERPRETATION: Reasonable reading of evidence, may be wrong
- FUTURE_WORK: Not yet implemented or proven
- UNSUPPORTED_REMOVED: Fabricated or unverifiable — mark allowed_in_pdf=false

Known evidence for this project:
- alive_status: ${evidence.alive_status}
- receipts_found: ${(evidence.receipts_found || []).join(', ') || 'none'}
- tests_found: ${(evidence.tests_found || []).join(', ') || 'none'}
- source_files: ${(evidence.source_files || []).slice(0, 10).join(', ')}

Extract 5-15 key claims from the TeX files.
For each claim:
  id: CLM-${project.slug.toUpperCase().replace(/-/g,'_')}-001, 002, etc.
  claim: the actual claim text
  claim_type: one of SOURCE_SUPPORTED / AUTHOR_THESIS / INTERPRETATION / FUTURE_WORK / UNSUPPORTED_REMOVED
  source_files: which files back this claim
  evidence_summary: 1 sentence explaining the backing
  confidence: HIGH / MEDIUM / LOW
  allowed_in_pdf: true unless UNSUPPORTED_REMOVED

Then write ${THESIS}/projects/${project.slug}/claim_ledger.yaml with the claims in YAML format.

Return structured claim ledger.`,
      { label: `referee:${project.slug}`, phase: 'Referee', schema: CLAIM_LIST_SCHEMA }
    )

    // Write claim ledger to disk
    await agent(
      `Write the claim ledger to ${THESIS}/projects/${project.slug}/claim_ledger.yaml.

Content:
${JSON.stringify(claimResult, null, 2)}

Convert to YAML format and write the file.
Return: "Claim ledger written for ${project.slug} with N claims"`,
      { label: `referee:write-ledger:${project.slug}`, phase: 'Referee' }
    )

    return { project, evidence, claims: claimResult }
  }
)

const validProjects = projectResults.filter(Boolean)
log(`Per-project manufacturing complete: ${validProjects.length}/${projects.length} projects processed`)

// ─── Phase 5: Aggregate Claim Ledger ────────────────────────────────────────
await agent(
  `Aggregate all project claim ledgers into a master ledger.

Read all claim_ledger.yaml files from:
  ${THESIS}/projects/*/claim_ledger.yaml

Use: find ${THESIS}/projects -name "claim_ledger.yaml"
Read each one.

Then write ${THESIS}/ledgers/CLAIM_LEDGER.yaml with all claims combined.

Also write ${THESIS}/ledgers/EVIDENCE_LEDGER.yaml listing:
- All receipts found across all projects
- All test files found across all projects  
- All checkpoint files found across all projects
- All ontology files found across all projects

Use find and read commands to gather this data.
Return: claim counts and status summary`,
  { label: 'aggregate:claim-ledger', phase: 'Referee' }
)

// ─── Phase 6: Master Thesis Condensation ────────────────────────────────────
phase('Condense')

const projectSummaries = validProjects.map(r => r ? `${r.project.slug} (${r.project.likely_thesis_role}): ${r.evidence.alive_status}` : 'null').join('\n')

const [chapterResults, frontmatterResult] = await Promise.all([
  agent(
    `You are the Master Thesis Synthesizer. Write the 12 dissertation chapters.

TITLE: From Prediction to Receipted Coordination: Independent Language-Model Research, Process Evidence, and the Manufacturing of Consequential Intelligence

WORKING SUBTITLE: A Post-Cyberpunk Dissertation on the Chatman Equation, ggen, Open Ontologies, AI XYNZ, and Capital-Flow Infrastructure

DISCOVERED PROJECTS (${validProjects.length} total):
${projectSummaries}

THESIS LINEAGE (this is the spine of the dissertation):
2016 independent language-model experiment (3,000-article corpus, temperature behavior)
→ lesson: local text imitation does not conserve consequence
→ enterprise process/workflow gap: data is operational motion, not tables
→ Chatman Equation: A = μ(O*), receipted form R ⊢ A = μ(O*)
→ receipts and replay (OCEL, process evidence, ALIVE/PARTIAL/BLOCKED)
→ ggen / graph-query-template manufacturing (operating on itself)
→ Open Ontologies (RDF, SPARQL, public ontology surfaces)
→ clap-noun-verb command grammar (deterministic execution grammar)
→ wasm4pm / process evidence (Van der Aalst, conformance checking)
→ Post-Cyberpunk PCP (hallucination vs receipt, Expo/Supabase framework)
→ AI XYNZ and capital flow (degree as credential bit, securities authority stack)
→ settlement, DAO, smart contracts, PQC receipts
→ industry-complete solution architecture (every NAICS industry covered)
→ customer-first, not investor-first proof logic

ABSOLUTE LAWS:
- Never fabricate claims
- Never say "semantic" (say: meaning-bearing, ontology, process evidence, consequence)
- Never say "generated" (say: manufactured, emitted, materialized, forged)  
- Never say "unhackable"
- Never name-drop celebrity AI thought leaders
- Customer proof over investor metrics
- ALIVE only when receipt+tests+checkpoint all exist

WRITE THESE 13 FILES (one per chapter plus preface):

${THESIS}/chapters/00_preface.tex
- Independent research path, no institutional backing
- Customer-first, not investor-first
- Why this is a crescendo
\\chapter*{Preface}

${THESIS}/chapters/01_origin_2016_language_model.tex
- Self-funded class, 3,000-article corpus
- Temperature behavior observation
- Surface imitation without consequence preservation
- Why this was independent research
\\chapter{Origin: The 2016 Language-Model Experiment}

${THESIS}/chapters/02_prediction_is_not_coordination.tex
- Why model output is not enough
- Local language texture does not close work
- Why wrapper startups monetize the wrong boundary
\\chapter{Prediction Is Not Coordination}

${THESIS}/chapters/03_enterprise_process_gap.tex
- Data is operational motion
- Query patterns, lineage, handoffs, governance, workflow
- Why tables are not the enterprise truth
\\chapter{The Enterprise Process Gap}

${THESIS}/chapters/04_chatman_equation.tex
- A = μ(O*)
- Observation, projection, admissibility, action, consequence
- Receipted form: R ⊢ A = μ(O*)
\\chapter{The Chatman Equation}

${THESIS}/chapters/05_process_evidence_and_receipts.tex
- OCEL/process evidence (cite Van der Aalst OCEL 2.0)
- receipts: what they are and why they matter
- replay: proving consequence occurred
- ALIVE/PARTIAL/BLOCKED: refusal of false closure
\\chapter{Process Evidence, Receipts, and Replay}

${THESIS}/chapters/06_ggen_and_open_ontologies.tex
- graph law and SPARQL queries
- Tera templates and artifact manufacturing
- ggen operating on itself (recursive manufacturing)
- Open Ontologies: public ontology surfaces
\\chapter{\\texttt{ggen} and Open Ontologies}

${THESIS}/chapters/07_command_grammar_and_execution.tex
- clap-noun-verb grammar
- command surfaces as public operating handles
- deterministic execution grammar
- project commands as receipted interfaces
\\chapter{Command Grammar and Execution}

${THESIS}/chapters/08_post_cyberpunk_pcp.tex
- Present Cyberpunk: hallucination-as-output
- Post-Cyberpunk: receipt-as-proof
- Expo/Supabase framework as working PCP infrastructure
- Why Post-Cyberpunk is working infrastructure, not aesthetic commentary
\\chapter{Post-Cyberpunk PCP}

${THESIS}/chapters/09_ai_xynz_and_capital_flow.tex
- Degree as credential bit
- Financial Investments as capital-language layer
- Securities/capital-market authority stack
- Settlement, DAO, smart contracts
- Post-quantum receipt infrastructure
- Capital-flow operating infrastructure
\\chapter{AI XYNZ and Capital-Flow Infrastructure}

${THESIS}/chapters/10_industry_complete_architecture.tex
- Industry packs
- NAICS-style classification as coverage map
- Every industry / every integration boundary
- C4 diagram manifest relationship
\\chapter{Industry-Complete Architecture}

${THESIS}/chapters/11_evaluation_and_receipts.tex
- Discovered projects: ${validProjects.length} projects
- What is ALIVE, PARTIAL, BLOCKED
- Evidence receipts across the corpus
- Conformance to Van der Aalst Constitution
\\chapter{Evaluation and Receipts}

${THESIS}/chapters/12_conclusion.tex
- Why this is the crescendo
- Not bigger models, but receipted coordination
- Customer proof over investor metrics
- Future work: what remains to be receipted
\\chapter{Conclusion}

Each chapter: 500-1500 words. Substantive, not placeholder.
Use \\label{chap:*} for cross-references.
Reference specific project slugs where relevant.

Write all 13 files. Return: "13 chapter files written"`,
    { label: 'condense:chapters', phase: 'Condense' }
  ),

  agent(
    `Write the dissertation frontmatter files for Sean Chatman's PhD thesis.

TITLE: From Prediction to Receipted Coordination

Write these 4 files:

${THESIS}/frontmatter/abstract.tex
\\chapter*{Abstract}
300-word abstract covering:
- The 2016 language-model experiment and its lesson
- The Chatman Equation A = μ(O*)
- Process evidence, receipts, replay
- ggen, Open Ontologies, command grammar
- wasm4pm and Post-Cyberpunk PCP
- Capital-flow and industry-complete architecture
- Contribution: a manufactured, receipted proof that consequential intelligence requires coordination, not just prediction

${THESIS}/frontmatter/acknowledgements.tex
\\chapter*{Acknowledgements}
Brief acknowledgement of the independent research path.
Customer-first orientation. No investor metrics.

${THESIS}/frontmatter/dedication.tex
\\chapter*{Dedication}
Brief dedication.

${THESIS}/frontmatter/declaration.tex
\\chapter*{Declaration}
Standard academic declaration of original work.
Note that this is independent research not conducted under institutional supervision.

Write all 4 files. Return: "Frontmatter written"`,
    { label: 'condense:frontmatter', phase: 'Condense' }
  ),
])

log(`Master thesis chapters: ${chapterResults}`)
log(`Frontmatter: ${frontmatterResult}`)

// ─── Phase 6b: main.tex, glossary, bibliography ─────────────────────────────
await agent(
  `Write the master LaTeX orchestration files for the PhD thesis.

PROJECTS FOUND: ${validProjects.map(r => r ? r.project.slug : '').filter(Boolean).join(', ')}

Write ${THESIS}/main.tex — full dissertation orchestration:
Use \\documentclass[12pt,a4paper]{report}
Include packages: geometry, hyperref, biblatex or natbib, graphicx, amsmath, amssymb, listings, booktabs, longtable, setspace, fancyhdr

Structure:
\\begin{document}
  \\frontmatter (or \\pagenumbering{roman})
  \\include{frontmatter/abstract}
  \\include{frontmatter/dedication}  
  \\include{frontmatter/acknowledgements}
  \\include{frontmatter/declaration}
  \\tableofcontents
  \\mainmatter (or \\pagenumbering{arabic})
  \\include{chapters/00_preface}
  \\include{chapters/01_origin_2016_language_model}
  \\include{chapters/02_prediction_is_not_coordination}
  \\include{chapters/03_enterprise_process_gap}
  \\include{chapters/04_chatman_equation}
  \\include{chapters/05_process_evidence_and_receipts}
  \\include{chapters/06_ggen_and_open_ontologies}
  \\include{chapters/07_command_grammar_and_execution}
  \\include{chapters/08_post_cyberpunk_pcp}
  \\include{chapters/09_ai_xynz_and_capital_flow}
  \\include{chapters/10_industry_complete_architecture}
  \\include{chapters/11_evaluation_and_receipts}
  \\include{chapters/12_conclusion}
  \\appendix
  % Project chapters included selectively
  \\printbibliography or \\bibliography{bibliography}
\\end{document}

Write ${THESIS}/glossary.tex with key terms:
- Chatman Equation: A = μ(O*)
- Receipt: cryptographic or structured proof that a consequence occurred
- Replay: re-enacting a process from its event log
- ggen: graph-query-template manufacturing engine
- OCEL: Object-Centric Event Log
- Process Evidence: event logs that prove a lawful process occurred
- ALIVE: status indicating all gates passed (receipt + tests + checkpoint + PDF hash)
- PARTIAL: status indicating some but not all gates passed
- BLOCKED: status indicating core gates failed
- Post-Cyberpunk PCP: working infrastructure with receipt-backed proof, not hallucination-as-output
- Manufactured: produced by a deterministic, receipted manufacturing pipeline

Write ${THESIS}/bibliography.bib with:
- OCEL 2.0 specification (public standard, cite as: Van der Aalst et al., Object-Centric Event Log 2.0)
- Van der Aalst process mining textbook reference
- RDF/SPARQL W3C specifications
- NIST PQC standards reference
- clap Rust crate reference (as implementation artifact)
- LOCAL_SOURCE entries for key ggen artifacts
- TODO_BIB entries for sources needing public verification

Write ${THESIS}/Makefile with targets:
  thesis: validate pdf-build
  validate: python3 scripts/validate_thesis.py
  pdf-build:
    cd ${THESIS} && /Library/TeX/texbin/pdflatex -interaction=nonstopmode -output-directory=build main.tex && /Library/TeX/texbin/pdflatex -interaction=nonstopmode -output-directory=build main.tex
  clean:
    rm -f ${THESIS}/build/*.aux ${THESIS}/build/*.log ${THESIS}/build/*.toc ${THESIS}/build/*.out
  receipts:
    python3 scripts/validate_thesis.py --receipt-only

Write ${THESIS}/scripts/validate_thesis.py:
A Python script that checks:
1. All 13 chapter files exist
2. All project directories have exactly 8 TeX files
3. main.tex exists
4. claim_ledger.yaml exists in each project dir
5. WORKFLOW_RECEIPT.yaml exists
6. Prints ALIVE / PARTIAL / BLOCKED with details
Make it executable.

Return: "main.tex, glossary.tex, bibliography.bib, Makefile, validate_thesis.py written"`,
  { label: 'condense:orchestration', phase: 'Condense' }
)

// ─── Phase 7: PDF Build ─────────────────────────────────────────────────────
phase('Compile')

const buildResult = await agent(
  `Compile the PhD thesis PDF.

THESIS DIR: ${THESIS}
MAIN TEX: ${THESIS}/main.tex
OUTPUT DIR: ${THESIS}/build/
COMPILER: /Library/TeX/texbin/pdflatex

STEPS:
1. Ensure build directory exists: mkdir -p ${THESIS}/build
2. First pass: cd ${THESIS} && /Library/TeX/texbin/pdflatex -interaction=nonstopmode -output-directory=build main.tex 2>&1 | tail -30
3. Second pass (for TOC/refs): cd ${THESIS} && /Library/TeX/texbin/pdflatex -interaction=nonstopmode -output-directory=build main.tex 2>&1 | tail -20
4. Check if PDF was produced: ls -la ${THESIS}/build/*.pdf
5. If PDF produced: sha256sum ${THESIS}/build/thesis.pdf or shasum -a 256 ${THESIS}/build/thesis.pdf
6. Copy to final name: cp ${THESIS}/build/thesis.pdf ${THESIS}/build/sean-chatman-phd-thesis-draft.pdf || true
7. Get file size: ls -lh ${THESIS}/build/*.pdf

If the first pdflatex run fails with errors:
- Read the log: tail -50 ${THESIS}/build/main.log
- Fix common LaTeX errors in main.tex or chapter files (missing packages, undefined commands)
- Retry compilation

Write ${THESIS}/ledgers/PDF_BUILD_RECEIPT.yaml with:
  compiler: /Library/TeX/texbin/pdflatex
  pdf_path: ${THESIS}/build/sean-chatman-phd-thesis-draft.pdf
  pdf_sha256: [actual hash or "BUILD_FAILED"]
  build_log_tail: [last 20 lines of build log]
  status: ALIVE if PDF exists, PARTIAL if PDF exists but with warnings, BLOCKED if no PDF

Return the build status and PDF hash if successful.`,
  { label: 'compile:pdf', phase: 'Compile' }
)

log(`PDF build result: ${buildResult}`)

// ─── Phase 8: Validation Gate ────────────────────────────────────────────────
phase('Validate')

const validationResult = await agent(
  `Run the final validation gate for the PhD thesis manufacturing run.

THESIS DIR: ${THESIS}
PROJECTS PROCESSED: ${validProjects.length}

Run these checks:

1. Check all chapter files exist:
   ls ${THESIS}/chapters/

2. Check all project dirs have 8 TeX files:
   for dir in ${THESIS}/projects/*/; do echo "$dir: $(ls $dir*.tex 2>/dev/null | wc -l) tex files"; done

3. Check claim ledgers:
   find ${THESIS}/projects -name "claim_ledger.yaml" | wc -l

4. Check master files:
   ls ${THESIS}/main.tex ${THESIS}/bibliography.bib ${THESIS}/glossary.tex ${THESIS}/Makefile

5. Check PDF:
   ls -lh ${THESIS}/build/*.pdf 2>/dev/null || echo "NO_PDF"

6. Check workflow receipt:
   ls ${THESIS}/ledgers/WORKFLOW_RECEIPT.yaml

7. Check DO_NOT_CLAIM_LEDGER:
   ls ${THESIS}/ledgers/DO_NOT_CLAIM_LEDGER.md

8. Run the validate script:
   cd ${THESIS} && python3 scripts/validate_thesis.py 2>&1 | head -50

9. Count total TeX files:
   find ${THESIS}/projects -name "*.tex" | wc -l
   find ${THESIS}/chapters -name "*.tex" | wc -l

GATE RULES:
- ALIVE: all 8 checks pass, PDF exists with hash
- PARTIAL: project corpus complete + TeX files exist + claim ledgers exist + PDF build failed (LaTeX tooling issue only)
- BLOCKED: source crawl failed, TeX files missing, unsupported claims in PDF, no coherent master thesis

Update ${THESIS}/ledgers/WORKFLOW_RECEIPT.yaml with final status.

Write ${THESIS}/ledgers/PDF_BUILD_RECEIPT.yaml if not already written, updating with final counts:
  project_count: N
  tex_file_count: N
  expected_tex_files_per_project: 8
  total_project_tex_files: N
  chapter_count: 13
  status: ALIVE / PARTIAL / BLOCKED

Return the final gate status with a detailed report.`,
  { label: 'validate:final-gate', phase: 'Validate' }
)

log(`Validation complete: ${validationResult}`)

// ─── Final Report ─────────────────────────────────────────────────────────
const finalReport = await agent(
  `Generate the final status report for Sean Chatman's PhD thesis manufacturing run.

THESIS DIR: ${THESIS}
PROJECTS PROCESSED: ${validProjects.length}

Gather:
1. Final PDF path: find ${THESIS}/build -name "*.pdf" 2>/dev/null
2. PDF hash: cat ${THESIS}/ledgers/PDF_BUILD_RECEIPT.yaml 2>/dev/null | head -20
3. Project count: ls ${THESIS}/projects/ | wc -l
4. TeX file count: find ${THESIS}/projects -name "*.tex" | wc -l
5. Chapter count: ls ${THESIS}/chapters/*.tex | wc -l
6. Git commits created: cd ${ROOT} && git log --oneline -10
7. Top claims: cat ${THESIS}/ledgers/CLAIM_LEDGER.yaml 2>/dev/null | head -50

Report format:

## PhD Thesis Manufacturing — Final Status

**Status:** ALIVE / PARTIAL / BLOCKED

**Projects discovered:** N
**Project TeX files manufactured:** N (N × 8 per project)
**Master chapters written:** 13
**PDF path:** ~/process-intelligence/phd-thesis/build/sean-chatman-phd-thesis-draft.pdf
**PDF SHA256:** [hash]

**Top 5 thesis claims (SOURCE_SUPPORTED):**
1. ...

**Top 5 strongest artifacts:**
1. ...

**Top 5 open questions / next work:**
1. ...

**Git commits created:**
[list]

**To rebuild:**
  cd ~/process-intelligence/phd-thesis
  make thesis

Return the complete final report.`,
  { label: 'validate:final-report', phase: 'Validate' }
)

return {
  status: 'complete',
  projects_discovered: projects.length,
  projects_processed: validProjects.length,
  thesis_dir: THESIS,
  report: finalReport,
}
