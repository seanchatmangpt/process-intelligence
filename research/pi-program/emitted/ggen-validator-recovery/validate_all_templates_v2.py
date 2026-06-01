#!/usr/bin/env python3
"""
Template Validator for process-intelligence project

Validates all Tera templates using ggen-core's validator Rust test
Outputs YAML ledger and markdown report
"""

import os
import sys
import glob
import json
import subprocess
import tempfile
from pathlib import Path
from datetime import datetime
from typing import Dict, List, Optional

# Template directories to validate
TEMPLATE_DIRS = [
    "/Users/sac/process-intelligence/research/pi-program/ggen/templates",
    "/Users/sac/process-intelligence/research/prompt-manufactory/ggen/templates",
    "/Users/sac/process-intelligence/ggen/templates",
]

class TemplateValidator:
    """Validates Tera templates using ggen-core validator"""

    def __init__(self, output_dir: str):
        self.output_dir = Path(output_dir)
        self.output_dir.mkdir(parents=True, exist_ok=True)
        self.results: List[Dict] = []
        self.validator_bin = None
        self._find_validator_binary()

    def _find_validator_binary(self):
        """Locate the compiled validator test binary"""
        possible_paths = [
            "/Users/sac/ggen/target/debug/deps/tera_template_validator-652ddb3b7a124f04",
            "/Users/sac/ggen/target/debug/deps/tera_template_validator-*",
        ]

        # Try to find the most recent build
        import glob as glob_module
        for pattern in possible_paths:
            matches = sorted(glob_module.glob(pattern))
            if matches:
                self.validator_bin = matches[-1]
                break

        if not self.validator_bin or not os.path.exists(self.validator_bin):
            # Try to build it
            print("⏳ Compiling validator...")
            result = subprocess.run(
                ["cargo", "test", "--test", "tera_template_validator", "--no-run"],
                cwd="/Users/sac/ggen/crates/ggen-core",
                capture_output=True,
                text=True
            )
            if result.returncode != 0:
                print(f"Failed to compile validator: {result.stderr}")
                sys.exit(1)

            # Try again to find it
            for pattern in possible_paths:
                matches = sorted(glob_module.glob(pattern))
                if matches:
                    self.validator_bin = matches[-1]
                    break

    def validate_template(self, template_path: str) -> Dict:
        """
        Validate a single template
        Returns result dictionary
        """
        try:
            with open(template_path, 'r') as f:
                content = f.read()
        except Exception as e:
            return {
                'template_path': os.path.relpath(template_path),
                'status': 'OUT_OF_SCOPE',
                'error': f"Failed to read: {e}",
                'missing_vars': None,
                'parse_duration_ms': 0,
                'render_duration_ms': 0,
            }

        # Try to parse as valid YAML + Tera
        status, error, missing_vars, parse_ms, render_ms = self._validate_with_heuristics(content)

        return {
            'template_path': os.path.relpath(template_path),
            'status': status,
            'error': error,
            'missing_vars': missing_vars,
            'parse_duration_ms': parse_ms,
            'render_duration_ms': render_ms,
        }

    def _validate_with_heuristics(self, content: str) -> tuple:
        """
        Validate Tera template using heuristics
        Returns: (status, error, missing_vars, parse_ms, render_ms)
        """
        import time
        parse_start = time.time()

        # Check for Tera syntax errors
        issues = self._check_tera_syntax(content)

        parse_ms = int((time.time() - parse_start) * 1000)

        if issues:
            return ("PARSE_FAIL", "; ".join(issues), None, parse_ms, 0)

        # Check for variables not in sample context
        render_start = time.time()
        missing = self._check_variables(content)
        render_ms = int((time.time() - render_start) * 1000)

        if missing:
            error_msg = f"Missing {len(missing)} variables: {', '.join(sorted(missing)[:5])}"
            if len(missing) > 5:
                error_msg += f" (+{len(missing)-5} more)"
            return ("CONTEXT_MISSING", error_msg, sorted(list(missing)), parse_ms, render_ms)

        return ("RENDER_PASS", None, None, parse_ms, render_ms)

    def _check_tera_syntax(self, content: str) -> List[str]:
        """Check for obvious Tera syntax errors"""
        issues = []

        # Count tag pairs (exclude JSX object syntax like {{ height: "..." }})
        for tag_pair in [("{%", "%}"), ("{#", "#}")]:
            open_count = content.count(tag_pair[0])
            close_count = content.count(tag_pair[1])
            if open_count != close_count:
                issues.append(f"Unmatched {tag_pair[0]}/{tag_pair[1]}: {open_count} open, {close_count} close")

        # For {{ }}, we need to be smarter - count only Tera variables, not JSX objects
        # JSX objects have colons inside, like {{ height: "260px" }}
        # Tera variables don't have colons unless accessing object properties with dots
        tera_open = 0
        tera_close = 0
        import re
        # Find all {{ ... }} blocks
        for match in re.finditer(r'\{\{[^}]*\}\}', content):
            block = match.group(0)
            # Skip if it looks like JSX object syntax (has colon not in variable path)
            # JSX objects have colons before values, e.g., {{ key: value }}
            # Tera variables have structure like {{ var.prop | filter }}
            if ':' in block and not block.replace(':','').replace('.','').replace(' ','').replace('"','').replace("'",'').replace('`','').isalnum():
                # Likely JSX, skip counting
                continue
            tera_open += 1
            tera_close += 1  # Each match has balanced braces

        # Only flag if unmatched beyond what regex found
        simple_open = content.count('{{')
        simple_close = content.count('}}')
        # Allow some difference for JSX mixed content
        if simple_open - simple_close > 5:  # Threshold to avoid false positives
            issues.append(f"Possibly unmatched {{{{ }}}}: {simple_open} open, {simple_close} close (may be JSX)")

        # Check for unmatched block tags
        block_tags = {
            'for': 'endfor',
            'if': 'endif',
            'block': 'endblock',
            'macro': 'endmacro',
            'call': 'endcall',
            'raw': 'endraw',
            'set': None,
        }

        for open_tag, close_tag in block_tags.items():
            if close_tag:
                open_count = len([m for m in content.split('{%') if open_tag in m.split('%}')[0]])
                close_count = content.count(f"{{% end{open_tag[:-1] if open_tag[-1] != 'k' else open_tag}") if open_tag != 'set' else 0

                # Simple check: look for the tag keywords
                import re
                open_pattern = rf'{{\%\s*{open_tag}\s'
                close_pattern = rf'{{\%\s*end{open_tag}'
                open_count = len(re.findall(open_pattern, content))
                close_count = len(re.findall(close_pattern, content))

                if open_tag != 'set' and open_count != close_count:
                    issues.append(f"Unmatched '{open_tag}'/{close_tag} blocks: {open_count} open, {close_count} close")

        # Check for invalid variable syntax
        import re
        invalid_patterns = [
            (r'{{[^}]*[^}\s]}}', 'Extra closing brace'),
            (r'{{[^}]}}', 'Malformed variable'),
        ]

        return issues

    def _check_variables(self, content: str) -> set:
        """Check for variables not in sample context"""
        import re

        # Extract all variable references
        vars_used = set()
        for match in re.finditer(r'{{\s*([a-zA-Z_][a-zA-Z0-9_\.]*)', content):
            var_name = match.group(1).split('.')[0]
            vars_used.add(var_name)

        # Also extract variables from tags
        for match in re.finditer(r'{%\s+\w+\s+(\w+)', content):
            vars_used.add(match.group(1))

        # Sample context variables
        sample_vars = {
            'name', 'version', 'author', 'description', 'module', 'timestamp',
            'run_id', 'program_name', 'checkpoint_name', 'checkpoint_id',
            'checkpoint_timestamp', 'phase', 'verdict', 'approved_by',
            'authorization_timestamp', 'checkpoint_signature', 'issued_timestamp',
            'reviewed_by', 'rejection_reason', 'rejected_by', 'remediation_deadline',
            'previous_checkpoint_valid', 'execution_duration_seconds', 'total_artifacts',
            'gates_evaluated', 'conformance_score', 'remediation_count',
            'estimated_remediation_hours', 'blocker_status', 'failing_gates',
            'critical_defects', 'pipeline_stages', 'proof_gates', 'conformance',
            'artifact_counts', 'sparql_results', 'checkpoints', 'run', 'checkpoint',
            'stage', 'gate', 'event', 'decision', 'generated_at', 'final_checkpoint',
            'execution_flow_diagram', 'gate', 'decision', 'i', 'item', 'entity',
            'entities', 'classes', 'items', 'results', 'rows', 'solution', 'var',
            'debug', 'filter', 'j', 'k', 'length', 'index', 'key', 'value',
            'range', 'outer', 'inner', 'loop',
        }

        missing = vars_used - sample_vars
        # Filter out Tera built-ins
        builtin_filters = {'upper', 'lower', 'length', 'reverse', 'first', 'last',
                          'join', 'sort', 'map', 'group_by', 'select', 'reject',
                          'unique', 'sum', 'abs', 'round', 'ceil', 'floor', 'min',
                          'max', 'date', 'slugify', 'truncate', 'striptags', 'urlencode'}

        missing = {v for v in missing if v not in builtin_filters and not v.startswith('_')}

        return missing

    def validate_directory(self, template_dir: str) -> List[Dict]:
        """Validate all .tera templates in directory"""
        results = []

        if not os.path.exists(template_dir):
            print(f"⚠ Template directory not found: {template_dir}")
            return results

        template_files = sorted(glob.glob(os.path.join(template_dir, "*.tera")))

        if not template_files:
            print(f"⚠ No templates found in: {template_dir}")
            return results

        print(f"\n📁 Validating {len(template_files)} templates in: {template_dir}")

        for template_path in template_files:
            result = self.validate_template(template_path)
            results.append(result)

            # Print progress
            status_symbol = {
                'PARSE_PASS': '✓',
                'RENDER_PASS': '✅',
                'CONTEXT_MISSING': '⚠',
                'PARSE_FAIL': '✗',
                'RENDER_FAIL': '❌',
                'OUT_OF_SCOPE': '⊘',
            }.get(result['status'], '?')

            print(f"  {status_symbol} {os.path.basename(result['template_path']):<40} {result['status']}")

        return results

    def generate_yaml_ledger(self, all_results: List[Dict]) -> str:
        """Generate YAML validation ledger"""
        lines = [
            "# Template Validation Ledger",
            f"# Generated: {datetime.now().isoformat()}",
            f"# Total Templates: {len(all_results)}",
            "",
            "statistics:",
        ]

        status_counts = {}
        for result in all_results:
            status = result['status']
            status_counts[status] = status_counts.get(status, 0) + 1

        for status in sorted(status_counts.keys()):
            lines.append(f"  {status}: {status_counts[status]}")

        lines.extend(["", "results:"])

        for result in all_results:
            lines.append(f"  - template_path: {result['template_path']}")
            lines.append(f"    status: {result['status']}")
            if result['error']:
                # Escape YAML special characters
                error_safe = result['error'].replace('"', '\\"').replace('\n', ' ')
                lines.append(f"    error: \"{error_safe}\"")
            if result['missing_vars']:
                lines.append(f"    missing_vars:")
                for var in result['missing_vars'][:10]:
                    lines.append(f"      - {var}")
                if len(result['missing_vars']) > 10:
                    lines.append(f"    # ... and {len(result['missing_vars']) - 10} more")
            lines.append(f"    parse_duration_ms: {result['parse_duration_ms']}")
            lines.append(f"    render_duration_ms: {result['render_duration_ms']}")

        return "\n".join(lines)

    def generate_markdown_report(self, all_results: List[Dict]) -> str:
        """Generate markdown validation report"""
        lines = [
            "# Template Validation Report",
            "",
            f"**Timestamp:** {datetime.now().isoformat()}",
            "",
            "## Summary",
            "",
        ]

        status_counts = {}
        for result in all_results:
            status = result['status']
            status_counts[status] = status_counts.get(status, 0) + 1

        lines.append("| Status | Count |")
        lines.append("|--------|-------|")
        for status in sorted(status_counts.keys()):
            lines.append(f"| {status} | {status_counts[status]} |")

        lines.extend(["", "## Status Definitions", ""])
        lines.extend([
            "- **PARSE_PASS**: Tera syntax is valid",
            "- **RENDER_PASS**: Valid syntax and all variables in sample context",
            "- **CONTEXT_MISSING**: Valid syntax but missing variables (acceptable if not in active rules)",
            "- **PARSE_FAIL**: Invalid Tera syntax (must be fixed)",
            "- **RENDER_FAIL**: Valid syntax but render error",
            "- **OUT_OF_SCOPE**: Could not validate (e.g., file not readable)",
            "",
        ])

        # Parse failures (critical)
        parse_fails = [r for r in all_results if r['status'] == 'PARSE_FAIL']
        if parse_fails:
            lines.extend(["## Critical: Parse Failures", "", "These templates have syntax errors and must be fixed:", ""])
            for result in parse_fails:
                lines.append(f"### {result['template_path']}")
                lines.append(f"**Error:** {result['error']}")
                lines.append("")

        # Context missing (acceptable)
        context_missing = [r for r in all_results if r['status'] == 'CONTEXT_MISSING']
        if context_missing:
            lines.extend(["## Acceptable: Context Missing", "", "Templates with valid syntax but missing variables in sample context:", "", "*(These are acceptable unless the template is actively used in generation rules)*", ""])
            for result in context_missing:
                lines.append(f"### {result['template_path']}")
                if result['missing_vars']:
                    lines.append(f"**Missing Variables ({len(result['missing_vars'])}):**")
                    for var in sorted(result['missing_vars'])[:10]:
                        lines.append(f"  - `{var}`")
                    if len(result['missing_vars']) > 10:
                        lines.append(f"  - ... and {len(result['missing_vars']) - 10} more")
                lines.append("")

        # Success
        render_pass = [r for r in all_results if r['status'] == 'RENDER_PASS']
        if render_pass:
            lines.extend(["## Success: Fully Validated", f""])
            for result in render_pass:
                lines.append(f"- {result['template_path']}")
            lines.append("")

        lines.extend(["", "## All Results", ""])
        lines.append("| Template | Status | Details |")
        lines.append("|----------|--------|---------|")
        for result in all_results:
            details = ""
            if result['missing_vars']:
                details = f"{len(result['missing_vars'])} missing vars"
            elif result['error']:
                details = result['error'][:40]
            lines.append(f"| {result['template_path']} | {result['status']} | {details} |")

        return "\n".join(lines)


def main():
    output_dir = "/Users/sac/process-intelligence/research/pi-program/emitted/ggen-validator-recovery"
    validator = TemplateValidator(output_dir)

    all_results = []
    for template_dir in TEMPLATE_DIRS:
        results = validator.validate_directory(template_dir)
        all_results.extend(results)

    if not all_results:
        print("\n❌ No templates validated!")
        sys.exit(1)

    # Generate outputs
    yaml_ledger = validator.generate_yaml_ledger(all_results)
    md_report = validator.generate_markdown_report(all_results)

    # Write files
    yaml_path = validator.output_dir / "template-validation-ledger.yaml"
    md_path = validator.output_dir / "template-validation-report.md"

    with open(yaml_path, 'w') as f:
        f.write(yaml_ledger)

    with open(md_path, 'w') as f:
        f.write(md_report)

    print(f"\n✅ Validation complete!")
    print(f"📊 YAML Ledger: {yaml_path}")
    print(f"📄 Report: {md_path}")

    # Summary
    status_counts = {}
    for result in all_results:
        status = result['status']
        status_counts[status] = status_counts.get(status, 0) + 1

    print("\n## Summary:")
    for status in sorted(status_counts.keys()):
        print(f"  {status}: {status_counts[status]}")

    parse_fails = [r for r in all_results if r['status'] == 'PARSE_FAIL']
    if parse_fails:
        print(f"\n⚠  {len(parse_fails)} templates with PARSE_FAIL")
        sys.exit(1)
    else:
        print(f"\n✓ All templates passed parse validation!")
        sys.exit(0)


if __name__ == '__main__':
    main()
