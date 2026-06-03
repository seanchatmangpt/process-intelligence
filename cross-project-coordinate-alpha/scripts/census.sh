#!/bin/bash
set -e

# Change directory to script parent directory
cd "$(dirname "$0")/.."

CENSUS_FILE="CROSS_PROJECT_CENSUS.md"
RECEIPT_FILE="receipts/census_receipt.yaml"

echo "# Cross-Project Coordination Census" > $CENSUS_FILE
echo "Generated at $(date)" >> $CENSUS_FILE
echo "" >> $CENSUS_FILE
echo "| Project | Path | Exists | Active Branch | Dirty | Primary Language | Role |" >> $CENSUS_FILE
echo "| :--- | :--- | :---: | :---: | :---: | :---: | :--- |" >> $CENSUS_FILE

PROJECTS=(
  "process-intelligence"
  "construct8-market-physics"
  "ggen"
  "ggen-mcp"
  "ggen-spec-kit"
  "open-ontologies"
  "wasm4pm"
  "wasm4pm-compat"
  "truex"
  "pcp"
  "naut"
  "knhk"
  "compiled-cognition-hub"
  "phd-thesis"
  "Documents/Papers"
  "Documents/Papers/workflow"
)

# Initialize YAML receipt
echo "census_receipt:" > $RECEIPT_FILE
echo "  timestamp: \"$(date -u +"%Y-%m-%dT%H:%M:%SZ")\"" >> $RECEIPT_FILE
echo "  projects:" >> $RECEIPT_FILE

for p in "${PROJECTS[@]}"; do
  # Determine path
  if [ "$p" == "Documents/Papers" ] || [ "$p" == "Documents/Papers/workflow" ]; then
    DPATH="$HOME/$p"
  elif [ "$p" == "construct8-market-physics" ]; then
    DPATH="$HOME/process-intelligence/construct8-market-physics"
  else
    DPATH="$HOME/$p"
  fi

  if [ -d "$DPATH" ]; then
    EXISTS="true"
    # Execute branch check in subshell
    BRANCH=$(cd "$DPATH" && git branch --show-current 2>/dev/null || echo 'not git')
    DIRTY=$(cd "$DPATH" && if [ -d .git ] || [ -f ../.git ]; then if [ -n "$(git status --porcelain 2>/dev/null)" ]; then echo 'true'; else echo 'false'; fi; else echo 'not git'; fi)
    
    # Determine language
    if [ -f "$DPATH/Cargo.toml" ]; then
      LANG="Rust"
    elif [ -f "$DPATH/package.json" ]; then
      LANG="TypeScript/JavaScript"
    elif [ -f "$DPATH/requirements.txt" ] || [ -f "$DPATH/setup.py" ]; then
      LANG="Python"
    else
      LANG="Mixed/Docs"
    fi

    # Role
    case "$p" in
      "process-intelligence") ROLE="Parent Coordination & Blue River Dam" ;;
      "construct8-market-physics") ROLE="Verified Relational Graph-State Witness" ;;
      "ggen") ROLE="Ontology Actuation Engine" ;;
      "ggen-mcp") ROLE="Ontology Context Protocol Server" ;;
      "ggen-spec-kit") ROLE="Spec Scaffolding Kit" ;;
      "open-ontologies") ROLE="Ontological State Ontologies" ;;
      "wasm4pm") ROLE="Wasm Process Mining Engine" ;;
      "wasm4pm-compat") ROLE="Process Evidence Compatibility Layer" ;;
      "truex") ROLE="Transactional Control Coordination" ;;
      "pcp") ROLE="Process Coordination Protocol" ;;
      "knhk") ROLE="Knowledge Hooks Engine Core" ;;
      "compiled-cognition-hub") ROLE="Actuation Evidence Hub" ;;
      "phd-thesis") ROLE="Academic Dissertational Structure" ;;
      *) ROLE="Documentation & Papers" ;;
    esac
  else
    EXISTS="false"
    BRANCH="ABSENT"
    DIRTY="ABSENT"
    LANG="ABSENT"
    ROLE="ABSENT"
  fi

  echo "| $p | $DPATH | $EXISTS | $BRANCH | $DIRTY | $LANG | $ROLE |" >> $CENSUS_FILE
  
  echo "    - name: \"$p\"" >> $RECEIPT_FILE
  echo "      exists: $EXISTS" >> $RECEIPT_FILE
  echo "      branch: \"$BRANCH\"" >> $RECEIPT_FILE
  echo "      dirty: \"$DIRTY\"" >> $RECEIPT_FILE
done

echo "Census generated successfully."