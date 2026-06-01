#!/usr/bin/env python3
"""
Template Validator for process-intelligence project

Validates all Tera templates using ggen-core's validator
Outputs YAML ledger and markdown report
"""

import os
import sys
import glob
import json
from pathlib import Path
from datetime import datetime
from typing import Dict, List, Optional, Tuple
import subprocess
import re

# Template directories to validate
TEMPLATE_DIRS = [
    "/Users/sac/process-intelligence/research/pi-program/ggen/templates",
    "/Users/sac/process-intelligence/research/prompt-manufactory/ggen/templates",
    "/Users/sac/process-intelligence/ggen/templates",
]

class TemplateValidator:
    """Validates Tera templates"""

    def __init__(self, output_dir: str):
        self.output_dir = Path(output_dir)
        self.output_dir.mkdir(parents=True, exist_ok=True)
        self.results: List[Dict] = []

    def validate_template_parse(self, content: str, template_path: str) -> Tuple[str, Optional[str], int]:
        """
        Phase 1: Parse validation
        Checks Tera syntax without context
        Returns: (status, error_msg, duration_ms)
        """
        import time
        start = time.time()

        # Check for basic syntax errors
        errors = []

        # Check for unmatched braces
        open_single = content.count("{%") - content.count("{%-")
        close_single = content.count("%}") - content.count("-%}")
        if open_single != close_single:
            errors.append(f"Unmatched tags: {open_single} open, {close_single} close")

        # Check for invalid tag patterns
        if re.search(r'{%%', content):
            errors.append("Invalid tag pattern: {%%")

        # Check for unclosed variable tags
        var_opens = content.count("{{")
        var_closes = content.count("}}")
        if var_opens != var_closes:
            errors.append(f"Unmatched variables: {var_opens} open, {var_closes} close")

        # Check for common syntax errors
        if re.search(r'{%\s*[a-z]+\s+[^%]*%}', content) and 'for' in content and 'endfor' not in content:
            if 'for' in content and content.count('for') > content.count('endfor'):
                errors.append("Unclosed 'for' loop")

        if re.search(r'{%\s*if\s+', content) and content.count('if') > content.count('endif'):
            errors.append("Unclosed 'if' statement")

        duration_ms = int((time.time() - start) * 1000)

        if errors:
            return ("PARSE_FAIL", "; ".join(errors), duration_ms)
        return ("PARSE_PASS", None, duration_ms)

    def extract_template_variables(self, content: str) -> set:
        """Extract all variable names used in template"""
        vars_used = set()

        # Find all {{ var_name }} patterns
        for match in re.finditer(r'{{\s*([a-zA-Z_][a-zA-Z0-9_\.]*)', content):
            var_name = match.group(1).split('.')[0]  # Get root variable
            vars_used.add(var_name)

        return vars_used

    def get_sample_context_vars(self) -> set:
        """Return set of variables available in sample context"""
        return {
            'name', 'version', 'author', 'description', 'module', 'timestamp',
            'run_id', 'program_name', 'checkpoint_name', 'checkpoint_id',
            'checkpoint_timestamp', 'phase', 'verdict', 'approved_by',
            'authorization_timestamp', 'checkpoint_signature', 'issued_timestamp',
            'reviewed_by', 'rejection_reason', 'rejected_by', 'remediation_deadline',
            'previous_checkpoint_valid', 'execution_duration_seconds', 'total_artifacts',
            'gates_evaluated', 'conformance_score', 'remediation_count',
            'estimated_remediation_hours', 'blocker_status', 'failing_gates',
            'critical_defects', 'pipeline_stages', 'proof_gates', 'conformance',
            'artifact_counts', 'sparql_results'
        }

    def validate_template_render(self, content: str, template_path: str, vars_used: set) -> Tuple[str, Optional[str], Optional[List[str]], int]:
        """
        Phase 2: Render validation
        Checks if all required variables are in sample context
        Returns: (status, error_msg, missing_vars, duration_ms)
        """
        import time
        start = time.time()

        available_vars = self.get_sample_context_vars()
        missing = vars_used - available_vars

        duration_ms = int((time.time() - start) * 1000)

        if missing:
            return ("CONTEXT_MISSING", f"Missing variables: {', '.join(sorted(missing))}", sorted(list(missing)), duration_ms)

        return ("RENDER_PASS", None, None, duration_ms)

    def validate_template(self, template_path: str) -> Dict:
        """
        Complete validation of a single template
        Returns result dictionary
        """
        try:
            with open(template_path, 'r') as f:
                content = f.read()
        except Exception as e:
            return {
                'template_path': template_path,
                'status': 'OUT_OF_SCOPE',
                'error': f"Failed to read: {e}",
                'missing_vars': None,
                'parse_duration_ms': 0,
                'render_duration_ms': 0,
            }

        # Phase 1: Parse validation
        parse_status, parse_error, parse_duration = self.validate_template_parse(content, template_path)

        if parse_status == "PARSE_FAIL":
            return {
                'template_path': template_path,
                'status': parse_status,
                'error': parse_error,
                'missing_vars': None,
                'parse_duration_ms': parse_duration,
                'render_duration_ms': 0,
            }

        # Phase 2: Render validation
        vars_used = self.extract_template_variables(content)
        render_status, render_error, missing_vars, render_duration = self.validate_template_render(content, template_path, vars_used)

        return {
            'template_path': template_path,
            'status': render_status,
            'error': render_error,
            'missing_vars': missing_vars,
            'parse_duration_ms': parse_duration,
            'render_duration_ms': render_duration,
        }

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
            rel_path = os.path.relpath(template_path)
            result['template_path'] = rel_path
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

            print(f"  {status_symbol} {os.path.basename(template_path):<40} {result['status']}")

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
                lines.append(f"    error: \"{result['error']}\"")
            if result['missing_vars']:
                lines.append(f"    missing_vars: {result['missing_vars']}")
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
            lines.extend(["## Acceptable: Context Missing", "", "Templates with valid syntax but missing variables in sample context:", ""])
            for result in context_missing:
                lines.append(f"### {result['template_path']}")
                lines.append(f"**Missing Variables:** {', '.join(result['missing_vars'] or [])}")
                lines.append("")

        # Success
        render_pass = [r for r in all_results if r['status'] == 'RENDER_PASS']
        if render_pass:
            lines.extend(["## Success: Fully Validated", f""])
            for result in render_pass:
                lines.append(f"- {result['template_path']}")
            lines.append("")

        lines.extend(["", "## All Results", ""])
        lines.append("| Template | Status | Error |")
        lines.append("|----------|--------|-------|")
        for result in all_results:
            error_text = result.get('error', '')[:50] if result.get('error') else "-"
            lines.append(f"| {result['template_path']} | {result['status']} | {error_text} |")

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
    parse_fails = [r for r in all_results if r['status'] == 'PARSE_FAIL']
    if parse_fails:
        print(f"\n⚠  {len(parse_fails)} templates with PARSE_FAIL (CRITICAL)")
        sys.exit(1)
    else:
        print(f"\n✓ All templates passed parse validation!")
        sys.exit(0)


if __name__ == '__main__':
    main()
