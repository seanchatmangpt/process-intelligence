import os
import re
import datetime

# File Paths
HOME_DIR = os.path.expanduser('~')
CENSUS_FILE = '/Users/sac/process-intelligence/phd-thesis/ledgers/HOME_ROOT_CENSUS.txt'
CLASSIFICATION_FILE = '/Users/sac/process-intelligence/phd-thesis/ledgers/ROOT_CLASSIFICATION.yaml'
RECEIPT_FILE = '/Users/sac/process-intelligence/phd-thesis/ledgers/ROOT_CRAWL_RECEIPT.yaml'

# Lists for classification markers
MUST_CRAWL_FILES = {
    'README.md', 'Cargo.toml', 'package.json', 'pyproject.toml',
    'go.mod', 'mix.exs', 'rebar.config', 'Makefile'
}

MUST_CRAWL_DIRS = {
    'docs', 'research', 'thesis', 'papers', 'book', 'ontology',
    'ontologies', 'queries', 'sparql', 'templates', 'receipts',
    'checkpoints', 'evidence', 'memory', 'coordination', 'workflows',
    'hooks', 'ggen', 'src', 'crates', 'apps', 'packages'
}

MUST_CRAWL_EXTENSIONS = {
    '.ttl', '.rq', '.tera', '.tex', '.bib', '.md'
}

SEARCH_SUBDIRS = [
    'docs', 'research', 'thesis', 'book', 'papers', 'ontology', 'ontologies',
    'queries', 'sparql', 'templates', 'receipts', 'checkpoints', 'evidence',
    'memory', 'coordination', 'workflows', 'hooks', 'reports', 'results',
    'src', 'crates', 'apps', 'packages', 'ggen', 'generated', 'emitted',
    'schemas', 'scripts', 'tests', 'examples'
]

SEARCH_EXTENSIONS = {
    '.md', '.txt', '.tex', '.bib', '.ttl', '.rq', '.sparql', '.tera',
    '.yaml', '.yml', '.toml', '.json', '.rs', '.go', '.py', '.ts', '.tsx',
    '.js', '.jsx', '.ex', '.exs', '.erl', '.hrl', '.java', '.c', '.h',
    '.cpp', '.wit'
}

# Ignore directories during general walks to prevent infinite recursion / huge scans
WALK_IGNORE_DIRS = {
    '.git', 'node_modules', 'target', 'build', '_build', 'dist',
    'local_cache', 'cachedir_joblib', '.pkg-cache', '.uvmgr_cache',
    '.pnpm-store', '.pnpm-cache', '.bazel-cache', '.gradle', '.m2',
    '.Trash', 'Library', 'Pictures', 'Movies', 'Music', 'Applications'
}

# Regex patterns for high-relevance search
QUERY_PATTERNS = [
    re.compile(r'knowledge[\s_-]hooks?', re.IGNORECASE),
    re.compile(r'\bknhk\b', re.IGNORECASE),
    re.compile(r'autonomic[\s_-]knowledge[\s_-]actuation', re.IGNORECASE),
    re.compile(r'actuation[\s_-]doctrine', re.IGNORECASE),
    re.compile(r'actuation[\s_-]law', re.IGNORECASE),
    re.compile(r'autonomic[\s_-]actuation', re.IGNORECASE),
    re.compile(r'Chatman[\s_-]Equation', re.IGNORECASE),
    re.compile(r'A\s*=\s*\\?mu\s*\(\s*O\*?\s*\)', re.IGNORECASE),
    re.compile(r'A\s*=\s*μ\s*\(\s*O\*?\s*\)', re.IGNORECASE)
]

# Classification configuration
SKIP_SYSTEM_NAMES = {
    'Library', 'Pictures', 'Movies', 'Music', 'Applications', 'miniconda3',
    'google-cloud-sdk', 'pgdata', 'proc', 'var', 'Virtual Machines.localized',
    'opt', 'etc', 'bin', 'Desktop', 'Downloads', 'Public', '.vagrant.d',
    '.colima', '.colima-data', '.docker', '.nvm', '.npm', '.cargo', '.rustup',
    '.conda', '.virtualenvs', '.oh-my-zsh', 'java', '.jdks', '.sdkman', '.asdf',
    '.kerl', 'otp_src_28.3.1', '.cups', '.duckdb', '.frozen-duckdb',
    '.rest-client-cache', '.rest-client-environments', '.rest-client-requests',
    '.rest-client-responses', '.rest-client-settings', '.rest-client-containers',
    '.rest-client-cookbooks', '.vscode', '.idea', '.matplotlib', '.oh-my-zsh',
    '.swiftpm', '.gem', '.bundle', '.pnpm-state', '.nexe', '.proto', '__snapshots__',
    'local_cache', 'cachedir_joblib', '.pkg-cache', '.uvmgr_cache', '.uvmgr', '.cocoapods'
}

SKIP_SECRETS_NAMES = {
    '.ssh', '.gnupg', '.aws', '.git-credentials', '.netrc', '.pypirc', '.rest-client'
}

SKIP_BUILD_ARTIFACTS_NAMES = {
    '.pnpm-store', '.pnpm-cache', '.bazel-cache', '.gradle', '.m2', 'dist', '_build',
    'node_modules', '.yarn-cache', '.cache', 'target', 'build', '.REST-client-cache'
}

# Explicit candidates listed by the user
EXPLICIT_CANDIDATES = [
    "~/A2A", "~/a2a-rs", "~/A2CI", "~/ai", "~/ai-chatbot", "~/ai-first-2026", "~/app",
    "~/autotel", "~/backup-critical", "~/bitactor", "~/bitstar", "~/blue_river_dam",
    "~/bytestar", "~/capability-map", "~/CascadeProjects", "~/cell8", "~/chatmangpt",
    "~/chicago-tdd-tools", "~/citty-test-utils", "~/clap-noun-verb", "~/claude",
    "~/claude-backup", "~/claude-code-context", "~/claude-desktop-context", "~/clawd",
    "~/clawdbot", "~/clnrm", "~/clnrm-backup-20251015-224552", "~/clnrm-backup-20251015-233810",
    "~/clnrm-dogfood-innovations", "~/clnrm.bak", "~/cns", "~/cns_forge", "~/compiled-cognition-hub",
    "~/coordination", "~/cre", "~/cre-pre-reset-backup-20260206-175352", "~/dashboard.bak",
    "~/dcmcp_cli_test", "~/dev", "~/dis", "~/docs", "~/doctester", "~/Documents",
    "~/Documents/Papers", "~/Documents/Papers/workflow", "~/dogturk", "~/Downloads/convo-files",
    "~/Downloads/cli_owl_artifacts", "~/Downloads/data-mcp-a2a", "~/Downloads/data-mcp-a2a-v2",
    "~/dteam", "~/dtr", "~/emitted", "~/erlmcp", "~/erlmcp_validation", "~/exampleSpecs",
    "~/full-stack-rubric", "~/ggen", "~/ggen-mcp", "~/ggen-spec-kit", "~/gitvan",
    "~/gitvan-backup-20250918-164242", "~/gitvan-backup-20250918-164245", "~/gitvan-backup-20250919-084758",
    "~/gitvan-recent-changes-backup-20250919-091930", "~/hive-mind-monorepo", "~/hive-queen",
    "~/homebrew-clnrm", "~/insa", "~/interview", "~/intvw", "~/java-maven-template",
    "~/jotp", "~/kgc-sidecar", "~/kgn", "~/knhk", "~/knowd", "~/knowtro", "~/ktemp",
    "~/Legal", "~/mac-artifact-cleaner", "~/mcp_erl", "~/mcp_search_test", "~/mcp-mqtt-erl",
    "~/mcpp", "~/memory", "~/my-dev-process", "~/neako", "~/nuxt-catalog-mdbook",
    "~/nuxt-layer", "~/nuxt-supabase-book", "~/nuxt-ui-pro-landing", "~/obsr",
    "~/open-ontologies", "~/optimus", "~/ostar", "~/otel", "~/pcp", "~/phd-thesis",
    "~/pigsty", "~/pigsty-supabase", "~/pigsty-supabase-osx", "~/pigsty-terraform",
    "~/portfolio-reports", "~/powlv2lsp", "~/practice", "~/process-intelligence",
    "~/receipts", "~/remo", "~/RepoPrompt", "~/reports", "~/research", "~/results",
    "~/s2s", "~/scripts", "~/semantic_bit", "~/seth", "~/seven_tick", "~/sos",
    "~/SparsePrimingRepresentations", "~/speckit-ralph", "~/src", "~/storehouse",
    "~/storehouse-infrastructure", "~/stpnt", "~/teamwork_projects", "~/teleport",
    "~/testbed", "~/tools", "~/truex", "~/ultrathink-bpm-engine", "~/unibit",
    "~/universe-chain", "~/unjucks", "~/unrdf", "~/unrdf-clean", "~/upscale",
    "~/verdicts", "~/wasm4pm", "~/wasm4pm-backups", "~/wasm4pm-compat",
    "~/weaver-driven-development-book", "~/wf", "~/yac", "~/yaml-cloud",
    "~/yaml-cloud-clean", "~/yaml-server", "~/yawl", "~/yawlv52", "~/yawlv6",
    "~/yawlw52", "~/ycloud", "~/zod-to-from", "~/zoeapp", "~/zoela"
]

def classify_root(path):
    # If path is Home root itself
    if path.rstrip('/') == HOME_DIR.rstrip('/'):
        return 'SKIP_SYSTEM', 'User home directory itself, scanned via children.', [], 'Scanned as children only'

    name = os.path.basename(path)
    
    # Check category sets by base name
    if name in SKIP_SYSTEM_NAMES:
        return 'SKIP_SYSTEM', 'Standard OS/runtime/cache/media/storage directory.', [], 'System directory skipped'
    if name in SKIP_SECRETS_NAMES:
        return 'SKIP_SECRETS', 'Directory likely contains credentials, keys, or private system configurations.', [], 'Secrets skipped'
    if name in SKIP_BUILD_ARTIFACTS_NAMES or name.startswith('.cache'):
        return 'SKIP_BUILD_ARTIFACTS', 'Build directory, package cache, or dependencies manager storage.', [], 'Build artifact directory skipped'
        
    # Check existence
    if not os.path.exists(path):
        return 'SKIP_SYSTEM', 'Directory does not exist on filesystem.', [], 'Missing path'
        
    if not os.path.isdir(path):
        return 'SKIP_SYSTEM', 'Path is a file, not a directory.', [], 'File skipped'

    # Check top-level contents
    try:
        items = os.listdir(path)
    except Exception as e:
        return 'SKIP_SYSTEM', f'Cannot read directory: {str(e)}', [], 'Read error'

    markers_found = []
    
    for item in items:
        item_path = os.path.join(path, item)
        if item in MUST_CRAWL_FILES:
            markers_found.append(item)
        elif item in MUST_CRAWL_DIRS and os.path.isdir(item_path):
            markers_found.append(item + '/')
        elif os.path.isfile(item_path):
            ext = os.path.splitext(item)[1]
            if ext in MUST_CRAWL_EXTENSIONS:
                markers_found.append(f'*{ext}')
                
    if markers_found:
        return 'MUST_CRAWL', f'Contains explicit project markers: {", ".join(sorted(markers_found))}', sorted(list(set(markers_found))), 'Matches MUST_CRAWL rule criteria'
        
    # Check for SHOULD_CRAWL: likely project evidence (.git folder, or simple project configs)
    if '.git' in items:
        return 'SHOULD_CRAWL', 'Contains .git directory indicating a versioned project repository.', ['.git/'], 'Has .git repository'
        
    if '.gitignore' in items or 'LICENSE' in items or 'LICENSE.md' in items:
        return 'SHOULD_CRAWL', 'Contains general repository config files like .gitignore or LICENSE.', ['.gitignore'], 'Has basic repo configs'

    # Custom directories with contents
    non_dot_items = [i for i in items if not i.startswith('.')]
    if non_dot_items:
        return 'MAYBE_CRAWL', f'Unrecognized non-empty custom directory with {len(non_dot_items)} items.', [], 'Custom user directory'

    return 'SKIP_SYSTEM', 'Empty or unclassified system-like directory.', [], 'Unused custom directory'

def search_files(root_path):
    """
    Search inside a root path.
    Prioritizes designated subdirs if present. If none are present, searches the whole directory recursively.
    """
    hits_list = []
    total_searched_files = 0
    
    # 1. Determine subdirs to scan
    subdirs_present = []
    for d in SEARCH_SUBDIRS:
        subdir_path = os.path.join(root_path, d)
        if os.path.isdir(subdir_path):
            subdirs_present.append(subdir_path)
            
    # If subdirs are present, scan only them. Otherwise, scan the root itself recursively
    paths_to_walk = subdirs_present if subdirs_present else [root_path]
    
    for base_path in paths_to_walk:
        for root, dirs, files in os.walk(base_path, topdown=True):
            # Modify dirs in-place to avoid walking ignored dirs
            dirs[:] = [d for d in dirs if d not in WALK_IGNORE_DIRS and not d.startswith('.')]
            
            for file in files:
                ext = os.path.splitext(file)[1].lower()
                if ext in SEARCH_EXTENSIONS:
                    file_path = os.path.join(root, file)
                    total_searched_files += 1
                    try:
                        with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                            lines = f.readlines()
                            
                        for i, line in enumerate(lines):
                            # Search for patterns
                            for pattern in QUERY_PATTERNS:
                                match = pattern.search(line)
                                if match:
                                    # Save hit details
                                    rel_file = os.path.relpath(file_path, HOME_DIR)
                                    hits_list.append({
                                        'file': f"~/{rel_file}",
                                        'line': i + 1,
                                        'content': line.strip()[:150], # Truncate long lines
                                        'keyword': match.group(0)
                                    })
                                    break # Check next line, avoid counting multiple times on same line
                    except Exception:
                        # Silently skip read errors (e.g. symlinks, permission issues)
                        pass
                        
    return hits_list, total_searched_files

def main():
    print("Starting Home Directory Corpus Discovery & Classification...")
    
    # 1. Enumerate all unique paths
    roots_to_process = set()
    
    # Read from HOME_ROOT_CENSUS.txt
    if os.path.exists(CENSUS_FILE):
        with open(CENSUS_FILE, 'r') as f:
            for line in f:
                path = line.strip()
                if path:
                    roots_to_process.add(path)
                    
    # Also add explicit candidates (expanded)
    for cand in EXPLICIT_CANDIDATES:
        expanded_path = os.path.abspath(os.path.expanduser(cand))
        roots_to_process.add(expanded_path)
        
    sorted_roots = sorted(list(roots_to_process))
    print(f"Total root directories to process: {len(sorted_roots)}")
    
    classifications = []
    crawl_results = []
    
    # Track metrics
    total_must_crawl = 0
    total_should_crawl = 0
    total_maybe_crawl = 0
    total_skipped = 0
    
    # 2. Process each root
    for root in sorted_roots:
        classification, reason, markers, notes = classify_root(root)
        
        # Determine if we should crawl / search
        searched = classification in ('MUST_CRAWL', 'SHOULD_CRAWL')
        high_relevance_hits = 0
        hits_detail = []
        files_scanned = 0
        
        # Run search if searched is True
        if searched:
            hits_detail, files_scanned = search_files(root)
            high_relevance_hits = len(hits_detail)
            
            if classification == 'MUST_CRAWL':
                total_must_crawl += 1
            else:
                total_should_crawl += 1
                
            if high_relevance_hits > 0:
                crawl_results.append({
                    'root': root.replace(HOME_DIR, '~'),
                    'hits_count': high_relevance_hits,
                    'files_scanned': files_scanned,
                    'matches': hits_detail
                })
        else:
            if classification == 'MAYBE_CRAWL':
                total_maybe_crawl += 1
            else:
                total_skipped += 1
                
        # Clean paths for YAML output
        rel_root = root.replace(HOME_DIR, '~')
        
        classifications.append({
            'root': rel_root,
            'classification': classification,
            'reason': reason,
            'project_markers_found': markers,
            'searched': searched,
            'high_relevance_hits': high_relevance_hits,
            'notes': notes
        })
        
    # Write ROOT_CLASSIFICATION.yaml
    print(f"Writing classification results to {CLASSIFICATION_FILE}...")
    with open(CLASSIFICATION_FILE, 'w') as f:
        f.write("# ROOT_CLASSIFICATION.yaml\n")
        f.write("# Enenumerated top-level home-directory roots and explicit project candidate classifications\n")
        f.write(f"# Processed on: {datetime.datetime.now().isoformat()}\n")
        f.write("# Total MUST_CRAWL: %d, SHOULD_CRAWL: %d, MAYBE_CRAWL: %d, SKIP: %d\n\n" % (
            total_must_crawl, total_should_crawl, total_maybe_crawl, total_skipped
        ))
        f.write("roots:\n")
        for item in classifications:
            f.write(f"  - root: \"{item['root']}\"\n")
            f.write(f"    classification: {item['classification']}\n")
            f.write(f"    reason: \"{item['reason']}\"\n")
            f.write(f"    project_markers_found: {item['project_markers_found']}\n")
            f.write(f"    searched: {str(item['searched']).lower()}\n")
            f.write(f"    high_relevance_hits: {item['high_relevance_hits']}\n")
            f.write(f"    notes: \"{item['notes']}\"\n")
            
    # Write ROOT_CRAWL_RECEIPT.yaml
    print(f"Writing crawl receipt to {RECEIPT_FILE}...")
    with open(RECEIPT_FILE, 'w') as f:
        f.write("# ROOT_CRAWL_RECEIPT.yaml\n")
        f.write(f"# Generated: {datetime.datetime.now().isoformat()}\n")
        f.write(f"# Status: ACTIVE\n")
        f.write(f"# Total scanned roots: {total_must_crawl + total_should_crawl}\n")
        f.write(f"# Total relevance hits: {sum(x['hits_count'] for x in crawl_results)}\n\n")
        f.write("receipt:\n")
        f.write("  engine: \"python-corpus-cartographer\"\n")
        f.write(f"  scan_time: \"{datetime.datetime.now().isoformat()}\"\n")
        f.write("  crawled_roots:\n")
        for item in crawl_results:
            f.write(f"    - root: \"{item['root']}\"\n")
            f.write(f"      hits_count: {item['hits_count']}\n")
            f.write(f"      files_scanned: {item['files_scanned']}\n")
            f.write("      matches:\n")
            # Group matches by file to keep it readable and smaller
            matches_by_file = {}
            for match in item['matches']:
                matches_by_file.setdefault(match['file'], []).append(match)
                
            for file_path, file_matches in matches_by_file.items():
                f.write(f"        - file: \"{file_path}\"\n")
                f.write("          lines:\n")
                for match in file_matches[:100]: # Limit to 100 hits per file to keep output reasonable
                    safe_content = match['content'].replace('"', '\\"').replace('\n', ' ')
                    f.write(f"            - line_number: {match['line']}\n")
                    f.write(f"              matched_text: \"{safe_content}\"\n")
                    f.write(f"              keyword: \"{match['keyword']}\"\n")
                    
    print("Done! Classification and crawl receipt files written successfully.")

if __name__ == '__main__':
    main()
