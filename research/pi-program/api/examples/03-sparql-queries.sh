#!/bin/bash
# Example 3: SPARQL Federated Queries
# Purpose: Execute knowledge graph queries with federation
# Authority: Process Intelligence Research Directorate

set -e

API_BASE="https://api.process-intelligence.research/v1"
API_KEY="${PI_API_KEY}"

if [ -z "$API_KEY" ]; then
  echo "Error: PI_API_KEY environment variable not set"
  exit 1
fi

echo "=== SPARQL Federated Queries ==="
echo

# Example 1: ASK query — Does the doctrine exist?
echo "1. Checking if doctrine ontology is loaded..."
ASK_QUERY='ASK {
  GRAPH ?g {
    ?doctrine a owl:Ontology ;
      dc:title "Process Intelligence Doctrine" .
  }
}'

RESULT=$(curl -s -X POST "$API_BASE/sparql" \
  -H "X-API-Key: $API_KEY" \
  -H "Content-Type: application/sparql-query" \
  -d "$ASK_QUERY")

echo "$RESULT" | jq '.boolean'
echo

# Example 2: SELECT query — Find all artifacts with their generators
echo "2. Finding all artifacts and their generating rules..."
SELECT_QUERY='PREFIX ggen: <http://process-intelligence.research/ggen#>
PREFIX dc: <http://purl.org/dc/elements/1.1/>

SELECT ?artifact ?name ?rule ?rule_name WHERE {
  ?artifact a ggen:Artifact ;
    ggen:name ?name ;
    ggen:generated-by ?rule .

  ?rule a ggen:GenerationRule ;
    ggen:name ?rule_name .
}
LIMIT 10'

RESULT=$(curl -s -X POST "$API_BASE/sparql" \
  -H "X-API-Key: $API_KEY" \
  -H "Content-Type: application/sparql-query" \
  -d "$SELECT_QUERY")

echo "$RESULT" | jq '.results.bindings[] | {
  artifact: .artifact.value,
  name: .name.value,
  rule: .rule.value,
  rule_name: .rule_name.value
}'
echo

# Example 3: COUNT query — How many artifacts exist?
echo "3. Counting total artifacts..."
COUNT_QUERY='PREFIX ggen: <http://process-intelligence.research/ggen#>

SELECT (COUNT(?artifact) AS ?count) WHERE {
  ?artifact a ggen:Artifact .
}'

RESULT=$(curl -s -X POST "$API_BASE/sparql" \
  -H "X-API-Key: $API_KEY" \
  -H "Content-Type: application/sparql-query" \
  -d "$COUNT_QUERY")

echo "Total artifacts: $(echo "$RESULT" | jq '.results.bindings[0].count.value')"
echo

# Example 4: Federated query — Cross-system search
echo "4. Running federated query (local + remote ontologies)..."
FEDERATED_QUERY='PREFIX ggen: <http://process-intelligence.research/ggen#>
PREFIX pi: <http://process-intelligence.research/pi#>

SELECT ?artifact ?conforms_to ?standard WHERE {
  ?artifact a ggen:Artifact ;
    ggen:generated-by ?rule .

  ?rule a ggen:GenerationRule ;
    pi:conforms-to ?standard .

  SERVICE <https://ontology-1.open-ontologies.research/sparql> {
    ?standard a pi:Standard ;
      pi:name ?standard_name .
  }
}
LIMIT 5'

RESULT=$(curl -s -X POST "$API_BASE/sparql" \
  -H "X-API-Key: $API_KEY" \
  -H "Content-Type: application/sparql-query" \
  -d "$FEDERATED_QUERY")

echo "$RESULT" | jq '.results.bindings[] | {
  artifact: .artifact.value,
  standard: .standard.value
}'
echo

# Example 5: CONSTRUCT query — Build RDF graph
echo "5. Constructing RDF graph of artifact-rule relationships..."
CONSTRUCT_QUERY='PREFIX ggen: <http://process-intelligence.research/ggen#>

CONSTRUCT {
  ?artifact ggen:generated-by ?rule .
  ?rule a ggen:GenerationRule .
}
WHERE {
  ?artifact a ggen:Artifact ;
    ggen:generated-by ?rule .
}
LIMIT 5'

RESULT=$(curl -s -X POST "$API_BASE/sparql" \
  -H "X-API-Key: $API_KEY" \
  -H "Content-Type: application/sparql-query" \
  -d "$CONSTRUCT_QUERY")

echo "$RESULT" | head -20
echo "..."
echo

echo "✓ SPARQL query examples complete"
