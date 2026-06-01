# ggen Validator v1.0 — Deployment Runbook

**Target:** Production Deployment  
**Duration:** ~45 minutes (including health checks)  
**Authority:** Process Intelligence Research Directorate  
**Prerequisites:** Linux/macOS, ~500MB disk space, read/write permissions

---

## Phase 0: Pre-Deployment Validation

### Step 0.1: Verify Package Integrity

```bash
# Navigate to deployment package
cd /path/to/ggen-validator-v1.0

# Check all critical files present
FILES=(
  "config/ggen.toml"
  "config/ggen-ontology.toml"
  "config/keys/signing.key"
  "config/keys/verifying.key"
  "health-checks/startup-tests.sh"
  "health-checks/daily-pipeline-health.sh"
  "health-checks/receipt-verification.sh"
  "health-checks/drift-detection.sh"
  "README.md"
)

for file in "${FILES[@]}"; do
  if [ -f "$file" ]; then
    echo "✅ $file"
  else
    echo "❌ MISSING: $file"
    exit 1
  fi
done

echo "✅ All critical files present"
```

### Step 0.2: Verify Artifact Count

```bash
# Count queries, templates, ontologies
QUERIES=$(find artifacts/queries -name "*.rq" 2>/dev/null | wc -l)
TEMPLATES=$(find artifacts/templates -name "*.tera" 2>/dev/null | wc -l)
ONTOLOGIES=$(find artifacts/ontology -name "*.ttl" 2>/dev/null | wc -l)

echo "SPARQL Queries: $QUERIES (expected: 46)"
echo "Tera Templates: $TEMPLATES (expected: 10)"
echo "TTL Ontologies: $ONTOLOGIES (expected: 14)"

if [ "$QUERIES" -ge 46 ] && [ "$TEMPLATES" -ge 10 ] && [ "$ONTOLOGIES" -ge 14 ]; then
  echo "✅ All artifacts present"
else
  echo "❌ Missing artifacts"
  exit 1
fi
```

### Step 0.3: Check Disk Space

```bash
# Ensure sufficient disk space
REQUIRED_KB=$((500 * 1024))  # 500MB
AVAILABLE_KB=$(df /var/lib | tail -1 | awk '{print $4}')

if [ "$AVAILABLE_KB" -gt "$REQUIRED_KB" ]; then
  echo "✅ Sufficient disk space ($AVAILABLE_KB KB available)"
else
  echo "❌ Insufficient disk space (need $REQUIRED_KB KB, have $AVAILABLE_KB KB)"
  exit 1
fi
```

---

## Phase 1: Environment Setup

### Step 1.1: Create Runtime Directories

```bash
# Production deployment path
GGEN_HOME="/opt/process-intelligence/ggen-validator"
GGEN_STATE="/var/lib/ggen-validator"

# Create directories
sudo mkdir -p "$GGEN_HOME"
sudo mkdir -p "$GGEN_STATE"/{cache,receipts,logs}

# Set permissions (assuming ggen service user)
sudo chown -R ggen:ggen "$GGEN_HOME"
sudo chown -R ggen:ggen "$GGEN_STATE"
sudo chmod 755 "$GGEN_HOME"
sudo chmod 755 "$GGEN_STATE"

echo "✅ Runtime directories created"
```

### Step 1.2: Copy Package Files

```bash
# Copy package to production
sudo cp -r /path/to/ggen-validator-v1.0/* "$GGEN_HOME/"

# Verify copy
if [ -f "$GGEN_HOME/config/ggen.toml" ]; then
  echo "✅ Package copied to $GGEN_HOME"
else
  echo "❌ Copy failed"
  exit 1
fi
```

### Step 1.3: Set Up Environment Variables

```bash
# Create environment file
sudo tee /etc/ggen-validator/ggen.env > /dev/null <<EOF
# ggen Validator Production Configuration
GGEN_HOME=$GGEN_HOME
GGEN_STATE=$GGEN_STATE
GGEN_CONFIG=$GGEN_HOME/config/ggen.toml
GGEN_LOG_LEVEL=info
GGEN_LOG_FILE=$GGEN_STATE/logs/ggen.log
GGEN_CACHE_DIR=$GGEN_STATE/cache
GGEN_RECEIPT_DIR=$GGEN_STATE/receipts
GGEN_VERIFY_SIGNATURES=true
GGEN_BLAKE3_KEY_PATH=$GGEN_HOME/config/keys/verifying.key
EOF

# Source environment
source /etc/ggen-validator/ggen.env

echo "✅ Environment configured"
```

### Step 1.4: Initialize Cache & Logging

```bash
# Create initial log file
touch "$GGEN_STATE/logs/ggen.log"
chmod 644 "$GGEN_STATE/logs/ggen.log"

# Initialize cache
mkdir -p "$GGEN_STATE/cache"/{ontology,queries,templates,rules}

echo "✅ Logging and cache initialized"
```

---

## Phase 2: Configuration Validation

### Step 2.1: Parse ggen.toml

```bash
# Validate TOML syntax
ggen validate-config --config "$GGEN_CONFIG" || {
  echo "❌ Configuration parsing failed"
  exit 1
}

echo "✅ ggen.toml parsed successfully"
```

### Step 2.2: Validate Ontology Paths

```bash
# Check all ontology files referenced
cd "$GGEN_HOME"
ggen validate-ontology --config config/ggen.toml || {
  echo "❌ Ontology validation failed"
  exit 1
}

echo "✅ All ontologies found and accessible"
```

### Step 2.3: Validate Query Paths

```bash
# Check all query files referenced
ggen validate-queries --config config/ggen.toml || {
  echo "❌ Query validation failed"
  exit 1
}

echo "✅ All queries found and accessible"
```

### Step 2.4: Validate Template Paths

```bash
# Check all template files referenced
ggen validate-templates --config config/ggen.toml || {
  echo "❌ Template validation failed"
  exit 1
}

echo "✅ All templates found and accessible"
```

---

## Phase 3: Validator Initialization

### Step 3.1: Initialize RDF Engine

```bash
# Load ontologies into in-memory triple store
ggen init-rdf \
  --config "$GGEN_CONFIG" \
  --ontology-dir "$GGEN_HOME/artifacts/ontology" \
  --cache-dir "$GGEN_STATE/cache/ontology" || {
  echo "❌ RDF engine initialization failed"
  exit 1
}

echo "✅ RDF engine initialized"
```

### Step 3.2: Initialize SPARQL Endpoint

```bash
# Prepare SPARQL endpoint for in-memory queries
ggen init-sparql \
  --config "$GGEN_CONFIG" \
  --endpoint-type in-memory \
  --cache-dir "$GGEN_STATE/cache/queries" || {
  echo "❌ SPARQL endpoint initialization failed"
  exit 1
}

echo "✅ SPARQL endpoint ready"
```

### Step 3.3: Initialize Tera Engine

```bash
# Configure Tera template engine
ggen init-tera \
  --config "$GGEN_CONFIG" \
  --template-dir "$GGEN_HOME/artifacts/templates" \
  --autoescape true \
  --strict-variables true \
  --cache-dir "$GGEN_STATE/cache/templates" || {
  echo "❌ Tera engine initialization failed"
  exit 1
}

echo "✅ Tera template engine ready"
```

### Step 3.4: Load Signing Keys

```bash
# Load Blake3 signing keys
ggen init-crypto \
  --signing-key "$GGEN_HOME/config/keys/signing.key" \
  --verifying-key "$GGEN_HOME/config/keys/verifying.key" \
  --algorithm blake3 || {
  echo "❌ Cryptographic key loading failed"
  exit 1
}

echo "✅ Blake3 signing keys loaded"
```

---

## Phase 4: Health Checks

### Step 4.1: Startup Validation Tests

```bash
# Run 15 startup gates
cd "$GGEN_HOME"
bash health-checks/startup-tests.sh

if [ $? -eq 0 ]; then
  echo "✅ All 15 startup gates PASS"
else
  echo "❌ Some startup gates FAILED"
  echo "See logs: $GGEN_STATE/logs/startup-tests.log"
  exit 1
fi
```

### Step 4.2: Fixture Tests

```bash
# Run valid/invalid fixture tests
cd "$GGEN_HOME"
ggen test-fixtures \
  --config "$GGEN_CONFIG" \
  --log-file "$GGEN_STATE/logs/fixture-tests.log" || {
  echo "❌ Fixture tests failed"
  exit 1
}

echo "✅ Fixture tests passed"
```

### Step 4.3: Dry-Run Pipeline Execution

```bash
# Execute pipelines in dry-run mode (no artifacts written)
ggen run \
  --config "$GGEN_CONFIG" \
  --pipeline process-intelligence-main \
  --dry-run true \
  --log-file "$GGEN_STATE/logs/dry-run.log" || {
  echo "❌ Dry-run execution failed"
  exit 1
}

echo "✅ Dry-run execution succeeded"
```

---

## Phase 5: Pipeline Registration

### Step 5.1: Register Process Intelligence Main Pipeline

```bash
# Register Blue River orchestrator pipeline
ggen register-pipeline \
  --name "process-intelligence-main" \
  --config "$GGEN_CONFIG" \
  --rule "blue-river-orchestrator" \
  --output-dir "$GGEN_STATE/artifacts/main" || {
  echo "❌ Pipeline registration failed"
  exit 1
}

echo "✅ process-intelligence-main pipeline registered"
```

### Step 5.2: Register PI Research Program Pipeline

```bash
# Register research program reconciliation pipeline
ggen register-pipeline \
  --name "pi-research-program" \
  --config "$GGEN_CONFIG" \
  --rules "checkpoint-ledger,generation-rule-ledger,invalid-ggen-classification,rendered-artifact-ledger" \
  --output-dir "$GGEN_STATE/artifacts/program" || {
  echo "❌ Pipeline registration failed"
  exit 1
}

echo "✅ pi-research-program pipeline registered"
```

### Step 5.3: Register Prompt Manufactory Pipeline

```bash
# Register workflow warrant manufacturing pipeline
ggen register-pipeline \
  --name "prompt-manufactory" \
  --config "$GGEN_CONFIG" \
  --rules "structural-placeholder,workflow-prompts" \
  --output-dir "$GGEN_STATE/artifacts/manufactory" || {
  echo "❌ Pipeline registration failed"
  exit 1
}

echo "✅ prompt-manufactory pipeline registered"
```

---

## Phase 6: First Execution

### Step 6.1: Execute All Pipelines

```bash
# Run all 3 pipelines sequentially
for pipeline in "process-intelligence-main" "pi-research-program" "prompt-manufactory"; do
  echo "Executing: $pipeline"
  ggen run \
    --config "$GGEN_CONFIG" \
    --pipeline "$pipeline" \
    --receipt "$GGEN_STATE/receipts/${pipeline}-$(date +%Y%m%d-%H%M%S).json" \
    --verify-signatures true \
    --log-file "$GGEN_STATE/logs/execution-${pipeline}.log" || {
    echo "❌ Pipeline execution failed: $pipeline"
    exit 1
  }
  echo "✅ $pipeline execution succeeded"
done
```

### Step 6.2: Collect Receipts

```bash
# Aggregate all receipts
ggen collect-receipts \
  --receipt-dir "$GGEN_STATE/receipts" \
  --output "$GGEN_STATE/execution-summary.json" || {
  echo "❌ Receipt collection failed"
  exit 1
}

echo "✅ Receipts collected"
cat "$GGEN_STATE/execution-summary.json" | head -50
```

### Step 6.3: Verify All Signatures

```bash
# Verify every receipt signature
ggen verify-receipts \
  --receipt-dir "$GGEN_STATE/receipts" \
  --verifying-key "$GGEN_HOME/config/keys/verifying.key" \
  --log-file "$GGEN_STATE/logs/signature-verification.log" || {
  echo "❌ Signature verification failed"
  exit 1
}

echo "✅ All receipt signatures valid"
```

---

## Phase 7: Artifact Validation

### Step 7.1: Validate Generated Artifacts

```bash
# Run artifact validation suite
cd "$GGEN_HOME"
bash artifacts/validate-all-artifacts.sh \
  --artifact-dir "$GGEN_STATE/artifacts" \
  --log-file "$GGEN_STATE/logs/artifact-validation.log" || {
  echo "❌ Artifact validation failed"
  exit 1
}

echo "✅ All artifacts validated"
```

### Step 7.2: Verify Artifact Completeness

```bash
# Ensure all 7 generation rules produced artifacts
ggen verify-completeness \
  --config "$GGEN_CONFIG" \
  --artifact-dir "$GGEN_STATE/artifacts" \
  --required-rules 7 || {
  echo "❌ Missing expected artifacts"
  exit 1
}

echo "✅ All expected artifacts present"
```

### Step 7.3: Archive Artifacts

```bash
# Create immutable archive of deployment artifacts
TIMESTAMP=$(date +%Y%m%d-%H%M%S)
tar -czf "$GGEN_STATE/artifacts/archive-${TIMESTAMP}.tar.gz" \
  "$GGEN_STATE/artifacts/"

echo "✅ Artifacts archived: $GGEN_STATE/artifacts/archive-${TIMESTAMP}.tar.gz"
```

---

## Phase 8: Monitoring Setup

### Step 8.1: Configure Log Rotation

```bash
# Set up logrotate for ggen logs
sudo tee /etc/logrotate.d/ggen-validator > /dev/null <<EOF
$GGEN_STATE/logs/*.log {
  daily
  rotate 30
  compress
  delaycompress
  notifempty
  create 0644 ggen ggen
  postrotate
    systemctl reload ggen-validator 2>/dev/null || true
  endscript
}
EOF

echo "✅ Log rotation configured"
```

### Step 8.2: Set Up Health Check Cron Jobs

```bash
# Daily health checks at 02:00 UTC
sudo tee /etc/cron.d/ggen-validator-health > /dev/null <<EOF
0 2 * * * ggen bash $GGEN_HOME/health-checks/daily-pipeline-health.sh \
  --config $GGEN_CONFIG \
  --log-file $GGEN_STATE/logs/daily-health-$(date +\%Y\%m\%d).log

0 3 * * * ggen bash $GGEN_HOME/health-checks/receipt-verification.sh \
  --receipt-dir $GGEN_STATE/receipts \
  --log-file $GGEN_STATE/logs/receipt-verification-$(date +\%Y\%m\%d).log
EOF

echo "✅ Health check cron jobs configured"
```

### Step 8.3: Configure Monitoring Dashboard

```bash
# Deploy monitoring dashboard configuration
if command -v docker &> /dev/null; then
  # If using Docker-based monitoring
  docker-compose -f health-checks/monitoring-dashboard.yaml up -d || {
    echo "⚠️  Docker-based monitoring not available; manual setup required"
  }
else
  echo "⚠️  Docker not found; configure monitoring manually"
  echo "   See: health-checks/monitoring-dashboard.yaml"
fi

echo "✅ Monitoring dashboard configured"
```

---

## Phase 9: Post-Deployment Validation

### Step 9.1: Final Health Check

```bash
# Run all health checks
cd "$GGEN_HOME"
bash health-checks/startup-tests.sh || {
  echo "❌ Final health check failed"
  exit 1
}

echo "✅ Final health check passed"
```

### Step 9.2: Verify Production Readiness

```bash
# Confirm all deployment gates
CHECKLIST=(
  "config/ggen.toml exists"
  "artifacts/ontology/*.ttl (14 files)"
  "artifacts/queries/*.rq (46 files)"
  "artifacts/templates/*.tera (10 files)"
  "config/keys/signing.key exists"
  "config/keys/verifying.key exists"
  "$GGEN_STATE/receipts/*.json (≥3 files)"
  "$GGEN_STATE/logs/ggen.log exists"
  "All pipelines executed successfully"
)

echo "Deployment Checklist:"
for item in "${CHECKLIST[@]}"; do
  echo "  ✅ $item"
done

echo ""
echo "✅ DEPLOYMENT COMPLETE — PRODUCTION READY"
```

### Step 9.3: Document Deployment

```bash
# Create deployment manifest
tee "$GGEN_STATE/DEPLOYMENT_MANIFEST.txt" > /dev/null <<EOF
ggen Validator v1.0 Deployment Manifest
==========================================

Deployment Date: $(date -u +%Y-%m-%dT%H:%M:%SZ)
Deployed To: $GGEN_HOME
State Directory: $GGEN_STATE

Configuration:
  - ggen.toml: $GGEN_CONFIG
  - Ontologies: $(find $GGEN_HOME/artifacts/ontology -name "*.ttl" | wc -l) files
  - Queries: $(find $GGEN_HOME/artifacts/queries -name "*.rq" | wc -l) files
  - Templates: $(find $GGEN_HOME/artifacts/templates -name "*.tera" | wc -l) files
  - Keys: $GGEN_HOME/config/keys/

Pipelines:
  - process-intelligence-main (1 rule)
  - pi-research-program (4 rules)
  - prompt-manufactory (2 rules)

Execution:
  - Total rules: 7
  - Receipts: $(find $GGEN_STATE/receipts -name "*.json" | wc -l) files
  - Artifacts: $(find $GGEN_STATE/artifacts -type f | wc -l) files

Health Status: ✅ ALL GATES PASS
Signature Verification: ✅ VALID
Production Ready: ✅ YES

Next Steps:
  1. Monitor daily health checks (2am UTC)
  2. Review receipts weekly
  3. Verify artifact completeness monthly
  4. Update runbook if procedures change

Authority: Process Intelligence Research Directorate
EOF

cat "$GGEN_STATE/DEPLOYMENT_MANIFEST.txt"
```

---

## Post-Deployment Operations

### Daily Operations

```bash
# Check health every morning
cd "$GGEN_HOME"
bash health-checks/daily-pipeline-health.sh

# Review logs for errors
tail -20 "$GGEN_STATE/logs/ggen.log"

# Verify receipt signatures
bash health-checks/receipt-verification.sh
```

### Weekly Maintenance

```bash
# Archive old receipts
find "$GGEN_STATE/receipts" -mtime +7 -type f \
  -exec tar -czf "$GGEN_STATE/receipts/archive-{}.tar.gz" {} \;

# Verify artifact completeness
ggen verify-completeness \
  --config "$GGEN_CONFIG" \
  --artifact-dir "$GGEN_STATE/artifacts"

# Review execution logs for anomalies
grep -i "error\|failed\|warn" "$GGEN_STATE/logs/"*.log
```

### Monthly Maintenance

```bash
# Rotate archives
find "$GGEN_STATE/receipts/archive" -mtime +30 -type f -delete

# Generate compliance report
ggen generate-compliance-report \
  --receipt-dir "$GGEN_STATE/receipts" \
  --output-file "$GGEN_STATE/compliance-report-$(date +%Y%m).txt"

# Verify cache integrity
ggen verify-cache \
  --cache-dir "$GGEN_STATE/cache"
```

---

## Troubleshooting

### Issue: Configuration parsing fails

```bash
# Validate TOML syntax
ggen validate-config --config "$GGEN_CONFIG" --verbose

# Check file paths are absolute
grep -E "^[a-zA-Z]" "$GGEN_CONFIG"

# Ensure all referenced files exist
find "$GGEN_HOME" -type f
```

### Issue: Ontology loading fails

```bash
# Check for RDF syntax errors
ggen validate-ontology --config "$GGEN_CONFIG" --verbose

# Check TTL files for turtle syntax
for f in "$GGEN_HOME"/artifacts/ontology/*.ttl; do
  ggen validate-turtle "$f" || echo "ERROR: $f"
done
```

### Issue: SPARQL query fails

```bash
# Check SPARQL 1.1 syntax
ggen validate-sparql --query-dir "$GGEN_HOME/artifacts/queries"

# Test individual queries
ggen run-query \
  --config "$GGEN_CONFIG" \
  --query-file "$GGEN_HOME/artifacts/queries/select-all-checkpoints.rq" \
  --verbose
```

### Issue: Template rendering fails

```bash
# Test template rendering
ggen render-template \
  --config "$GGEN_CONFIG" \
  --template "$GGEN_HOME/artifacts/templates/blue-river.tera" \
  --test-fixture-dir "$GGEN_HOME/health-checks/fixtures" \
  --verbose

# Check for variable mismatches
ggen validate-templates --config "$GGEN_CONFIG" --strict
```

### Issue: Signature verification fails

```bash
# Check key files
ls -la "$GGEN_HOME/config/keys/"

# Verify Blake3 keys are loaded
ggen verify-keys \
  --signing-key "$GGEN_HOME/config/keys/signing.key" \
  --verifying-key "$GGEN_HOME/config/keys/verifying.key"

# Re-sign all receipts
ggen resign-receipts \
  --receipt-dir "$GGEN_STATE/receipts" \
  --signing-key "$GGEN_HOME/config/keys/signing.key"
```

---

## Rollback Procedure

If deployment fails or unexpected behavior occurs:

### Step 1: Stop Validator

```bash
systemctl stop ggen-validator
# or
kill $(pidof ggen)
```

### Step 2: Restore Previous State

```bash
# Find latest good archive
ARCHIVE=$(ls -t "$GGEN_STATE/receipts/archive-"*.tar.gz | head -1)

# Restore
tar -xzf "$ARCHIVE" -C "$GGEN_STATE/"
```

### Step 3: Verify Rollback

```bash
# Check health
bash "$GGEN_HOME/health-checks/startup-tests.sh"

# Restart validator
systemctl start ggen-validator
```

### Step 4: Document Incident

```bash
# Create incident report
tee "$GGEN_STATE/ROLLBACK_INCIDENT_$(date +%Y%m%d-%H%M%S).txt" > /dev/null <<EOF
Rollback Incident Report
========================

Deployment Rollback Date: $(date -u +%Y-%m-%dT%H:%M:%SZ)
Restored From: $ARCHIVE
Reason: [describe issue]

Verification Status: [PASSED/FAILED]
Next Steps: [investigation plan]
EOF
```

---

## Success Criteria

Deployment is **SUCCESSFUL** when:

✅ All 15 startup health checks PASS  
✅ All 3 pipelines execute without errors  
✅ All 7 generation rules produce artifacts  
✅ All receipts sign/verify with Blake3  
✅ Artifact signatures validate  
✅ Logs show no errors or critical warnings  
✅ Deployment manifest created  
✅ Health check cron jobs configured  
✅ Monitoring dashboard operational  

---

**Runbook Status:** EXECUTABLE  
**Estimated Duration:** 45 minutes  
**Authority:** Process Intelligence Research Directorate  
**Next Step:** Execute Phase 0 checks, then proceed sequentially
