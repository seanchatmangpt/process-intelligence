# Source Index — experiments-visualizer-nextjs

All source files read during evidence extraction for this thesis chapter.

| File | Description |
|------|-------------|
| `/Users/sac/process-intelligence/experiments/visualizer-nextjs/package.json` | Declares project dependencies: Next.js 16.2.6, React 19.2.4, TypeScript 5, Tailwind CSS v4, Geist/Geist Mono fonts |
| `/Users/sac/process-intelligence/experiments/visualizer-nextjs/src/app/page.tsx` | Stock create-next-app welcome page; the primary implementation gap of the project |
| `/Users/sac/process-intelligence/experiments/visualizer-nextjs/src/app/layout.tsx` | Root layout component; loads Geist/Geist Mono fonts and imports globals.css |
| `/Users/sac/process-intelligence/experiments/visualizer-nextjs/src/app/globals.css` | 973-line CSS design system encoding the complete visual vocabulary for all five process-intelligence visualization subsystems |
| `/Users/sac/process-intelligence/experiments/visualizer-nextjs/next.config.ts` | Next.js build configuration in TypeScript |
| `/Users/sac/process-intelligence/experiments/visualizer-nextjs/AGENTS.md` | Authoring-agent instructions warning of Next.js 16 breaking changes; directs agents to read node_modules/next/dist/docs/ before writing code |
| `/Users/sac/process-intelligence/experiments/visualizer-nextjs/CLAUDE.md` | Project-level Claude Code instructions for this sub-project |
| `/Users/sac/process-intelligence/experiments/visualizer-nextjs/.next/BUILD_ID` | Build identifier (9NFA-LnYiAijWLGnz8L79) confirming successful Turbopack compilation |
| `/Users/sac/process-intelligence/experiments/visualizer-nextjs/.next/routes-manifest.json` | Records the complete route surface: single route (/) mapped to the welcome page; no API routes |
| `/Users/sac/process-intelligence/experiments/visualizer-nextjs/.next/diagnostics/build-diagnostics.json` | Records buildStage: static-generation and useBuildWorker: true confirming Turbopack build worker |
| `/Users/sac/process-intelligence/experiments/visualizer-nextjs/.next/server/app/index.html` | Statically rendered output of the current placeholder page.tsx; contains no process-intelligence content |
| `/Users/sac/process-intelligence/experiments/checkpoint__experiments_complete.md` | Parent experiments corpus checkpoint; records EXPERIMENTS_COMPLETE verdict dated 2026-05-31 covering the experiment fixture corpus (not the web visualizer) |
| `/Users/sac/process-intelligence/experiments/audit__experiment_completeness.md` | Completeness audit for the parent experiments corpus; enumerates coverage boundaries that exclude the visualizer |
| `/Users/sac/process-intelligence/experiments/EVIDENCE_CHAIN_TRACE.md` | Documents the full Raw→Parsed→Admitted→Receipt evidence chain in Rust type notation; cited as upstream evidence the visualizer would display |
