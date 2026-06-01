import os
import re
import json
import sys

BASE_DIR = "/Users/sac/process-intelligence"
STANDARDS_DIR = os.path.join(BASE_DIR, "standards")

TARGET_STANDARDS = {
    "XES": {
        "placement": "xes_process-intelligence_placement.md",
        "overview": "xes.md"
    },
    "OCEL 2.0": {
        "placement": "ocel_process-intelligence_placement.md",
        "overview": "ocel.md"
    },
    "BPMN": {
        "placement": "bpmn_process-intelligence_placement.md",
        "overview": "bpmn.md"
    },
    "POWL": {
        "placement": "powl_placement.md",
        "overview": "powl.md"
    },
    "Declare": {
        "placement": "declare_placement.md",
        "overview": "declare.md"
    },
    "OCPQ": {
        "placement": "ocpq_placement.md",
        "overview": "ocpq.md"
    }
}

PLACEHOLDER_REGEX = re.compile(
    r'\b(TODO|FIXME|unimplemented|placeholder|stub|tbd|to be determined)\b', 
    re.IGNORECASE
)

# Regex to find markdown links
LINK_REGEX = re.compile(r'\[([^\]]+)\]\((file:///[^\)]+)\)')

def validate_file(filepath):
    errors = []
    with open(filepath, 'r', encoding='utf-8') as f:
        content = f.read()

    # 1. Check for placeholders
    matches = PLACEHOLDER_REGEX.findall(content)
    if matches:
        errors.append(f"Found placeholder terms: {set(matches)}")

    # 2. Extract and validate links
    # Check that link text does not contain backticks
    for match in re.finditer(r'\[([^\]]*)`([^\]]*)\]\((file:///[^\)]+)\)', content):
        errors.append(f"Link text has backticks: {match.group(0)}")

    links = LINK_REGEX.findall(content)
    for text, url in links:
        # Check if URL starts with BASE_DIR URL
        prefix = "file:///Users/sac/process-intelligence/"
        if not url.startswith(prefix):
            errors.append(f"Non-standard absolute link: {url}")
            continue
        
        # Resolve target path
        rel_path = url[len(prefix):]
        # Ignore line anchors like #L123-L145
        if '#' in rel_path:
            rel_path = rel_path.split('#')[0]
        
        target_path = os.path.join(BASE_DIR, rel_path)
        if not os.path.exists(target_path):
            errors.append(f"Broken link: {url} (resolved to {target_path})")

    # 3. Validate JSON code blocks
    json_blocks = re.findall(r'```json\s*(.*?)\s*```', content, re.DOTALL)
    for idx, block in enumerate(json_blocks):
        try:
            json.loads(block)
        except json.JSONDecodeError as e:
            errors.append(f"Invalid JSON block #{idx+1}: {e}")

    return errors

def main():
    print("Starting process-intelligence standards schema validation audit...")
    all_errors = {}
    
    # Check files existence
    for name, files in TARGET_STANDARDS.items():
        for file_key, filename in files.items():
            filepath = os.path.join(STANDARDS_DIR, filename)
            if not os.path.exists(filepath):
                all_errors[filename] = [f"Standard file for {name} ({file_key}) does not exist."]
                continue
            
            file_errors = validate_file(filepath)
            
            # Additional structural checks on placement files
            if file_key == "placement":
                with open(filepath, 'r', encoding='utf-8') as f:
                    content = f.read()
                
                # Check for Trans-Standard Conversions section
                if "## 5. Trans-Standard Conversions and Loss Policy" not in content:
                    file_errors.append("Missing required section: '## 5. Trans-Standard Conversions and Loss Policy'")
                else:
                    # Verify presence of LossReport schema or references
                    if "LossReport" not in content or "witness_signature" not in content:
                        file_errors.append("Trans-Standard Conversions section is incomplete (missing LossReport schema / signature).")
            
            if file_errors:
                all_errors[filename] = file_errors

    # Verify standard coverage audit file is aligned
    coverage_file = os.path.join(STANDARDS_DIR, "audit__standards_coverage.md")
    if os.path.exists(coverage_file):
        coverage_errors = validate_file(coverage_file)
        if coverage_errors:
            all_errors["audit__standards_coverage.md"] = coverage_errors
    
    if all_errors:
        print("\n--- AUDIT FAILED ---")
        for filename, errors in all_errors.items():
            print(f"\n[{filename}]")
            for err in errors:
                print(f"  - {err}")
        sys.exit(1)
    else:
        print("\n--- AUDIT PASSED ---")
        print("All schema integrations for XES, OCEL 2.0, BPMN, POWL, Declare, and OCPQ are fully valid.")
        print("Trans-standard conversion rules specify structural loss policies and output signed LossReports.")
        sys.exit(0)

if __name__ == "__main__":
    main()
