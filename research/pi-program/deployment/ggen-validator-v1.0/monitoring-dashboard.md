# ggen Validator v1.0 — Monitoring Dashboard

**Version:** 1.0.0  
**Authority:** Process Intelligence Research Directorate  
**Backends:** Prometheus, Grafana, ELK Stack

---

## Dashboard Overview

The ggen Validator monitoring dashboard provides real-time visibility into:

1. **Pipeline Health** — Execution status, latency, success rates
2. **Query Performance** — Query execution time, SPARQL endpoint health
3. **Template Rendering** — Render time, fixture test results
4. **Receipt Verification** — Signature validation, cryptographic integrity
5. **Artifact Quality** — Generated artifact count, validation status
6. **System Resources** — CPU, memory, disk, process uptime
7. **Error Tracking** — Error frequency, severity levels, trends
8. **Compliance Status** — Gate pass rates, checkpoint semantics

---

## Prometheus Metrics

### System Metrics

```yaml
# Process metrics
ggen_process_uptime_hours
ggen_process_cpu_usage_percent
ggen_process_memory_usage_mb
ggen_process_file_descriptor_count

# System metrics
ggen_system_disk_usage_percent
ggen_system_memory_available_mb
ggen_system_load_average
```

### Pipeline Metrics

```yaml
# Execution metrics
ggen_pipeline_execution_duration_ms{pipeline="process-intelligence-main"}
ggen_pipeline_execution_duration_ms{pipeline="pi-research-program"}
ggen_pipeline_execution_duration_ms{pipeline="prompt-manufactory"}

ggen_pipeline_success_rate{pipeline="*"}
ggen_pipeline_failure_count{pipeline="*"}
ggen_pipeline_last_execution_timestamp{pipeline="*"}

# Rule metrics
ggen_generation_rule_execution_duration_ms{rule="*"}
ggen_generation_rule_success_rate{rule="*"}
```

### Query Metrics

```yaml
# SPARQL execution
ggen_sparql_query_execution_duration_ms{query="*"}
ggen_sparql_query_result_count{query="*"}
ggen_sparql_endpoint_response_time_ms

# SPARQL health
ggen_sparql_query_success_rate
ggen_sparql_timeout_count
ggen_sparql_error_count
```

### Template Metrics

```yaml
# Tera rendering
ggen_template_render_duration_ms{template="*"}
ggen_template_render_success_rate{template="*"}
ggen_template_variable_error_count

# Template health
ggen_template_fixture_pass_rate
ggen_template_autoescape_violations
```

### Receipt Metrics

```yaml
# Signing and verification
ggen_receipt_signature_duration_ms
ggen_receipt_verification_duration_ms
ggen_receipt_signature_success_rate

# Receipt health
ggen_receipt_count_total
ggen_receipt_verification_failure_count
ggen_receipt_chain_integrity_violations
```

### Artifact Metrics

```yaml
# Artifact generation
ggen_artifact_generation_count{pipeline="*"}
ggen_artifact_size_bytes{artifact="*"}
ggen_artifact_validation_success_rate

# Artifact health
ggen_artifact_validation_failure_count
ggen_artifact_write_duration_ms
```

### Quality Metrics

```yaml
# Gate-based compliance
ggen_startup_gate_pass_rate
ggen_startup_gate_failure_count
ggen_daily_health_gate_pass_rate

# Semantic compliance
ggen_checkpoint_semantics_violations
ggen_forced_alive_verdicts
ggen_file_count_alive_violations
```

---

## Grafana Dashboards

### Dashboard 1: Pipeline Health Overview

**Layout:** 2x2 grid  
**Refresh:** 60 seconds  
**Time Range:** Last 24 hours

```
┌─────────────────────────────────┬─────────────────────────────────┐
│ Pipeline Execution Status       │ Success Rate (24h)              │
│ (Real-time)                     │ • process-intelligence-main: 100│
│ • EXECUTING process-intel-main  │ • pi-research-program: 100      │
│ • READY pi-research-program     │ • prompt-manufactory: 100       │
│ • IDLE prompt-manufactory       │ • AVERAGE: 100%                 │
├─────────────────────────────────┼─────────────────────────────────┤
│ Pipeline Latency (percentiles)  │ Recent Errors                   │
│ p50: 2min 15sec                 │ (Last 24 hours: 0)              │
│ p95: 4min 32sec                 │ • ERROR: 0                      │
│ p99: 5min 48sec                 │ • WARNING: 0                    │
│                                 │ • INFO: 12                      │
└─────────────────────────────────┴─────────────────────────────────┘
```

**Panels:**

1. **Pipeline Status** (Gauge)
   - Metric: `ggen_pipeline_success_rate`
   - Threshold: GOOD ≥95%, WARN ≥85%, CRITICAL <85%
   - Show: Current value, last 24h trend

2. **Execution Latency** (Multi-line)
   - Metrics: `ggen_pipeline_execution_duration_ms` (p50, p95, p99)
   - Time Range: Last 24 hours
   - Y-axis: milliseconds

3. **Error Trend** (Bar chart)
   - Metrics: `ggen_pipeline_failure_count`
   - Time Range: Last 24 hours, grouped by hour
   - Color: Red for failures

4. **Rule Execution** (Heatmap)
   - Metrics: `ggen_generation_rule_execution_duration_ms`
   - Time Range: Last 7 days
   - Show: Execution time distribution

---

### Dashboard 2: Query & Template Performance

**Layout:** 3 panels  
**Refresh:** 30 seconds  
**Time Range:** Last 1 hour

```
┌──────────────────────────────────────────────────────────────────┐
│ SPARQL Query Performance (Last Hour)                              │
│ Query                           │ Avg Time │ Max Time │ Success % │
│ select-all-checkpoints.rq       │ 245ms    │ 512ms    │ 100%      │
│ select-all-generation-rules.rq  │ 189ms    │ 421ms    │ 100%      │
│ extract-lifecycle-governance.rq │ 312ms    │ 698ms    │ 100%      │
├──────────────────────────────────────────────────────────────────┤
│ Template Render Performance (Last Hour)                            │
│ Template                      │ Avg Time │ Max Time │ Success % │
│ blue-river.tera               │ 42ms     │ 89ms     │ 100%      │
│ ma-deck.tera                  │ 38ms     │ 76ms     │ 100%      │
│ checkpoint-ledger.md.tera     │ 51ms     │ 94ms     │ 100%      │
├──────────────────────────────────────────────────────────────────┤
│ Endpoint Health (Real-time)                                       │
│ SPARQL Endpoint: HEALTHY (response time: 12ms)                    │
│ RDF Cache Hit Rate: 98.7%                                         │
│ Template Cache Hit Rate: 94.2%                                    │
└──────────────────────────────────────────────────────────────────┘
```

**Panels:**

1. **SPARQL Query Performance** (Table)
   - Metrics: `ggen_sparql_query_execution_duration_ms` (avg, max)
   - Metric: `ggen_sparql_query_success_rate`
   - Time Range: Last 1 hour
   - Sort: By max time (descending)

2. **Template Render Times** (Bar chart)
   - Metrics: `ggen_template_render_duration_ms`
   - Time Range: Last 1 hour
   - Grouped by: Template name

3. **Endpoint Health** (Stat)
   - Metrics: `ggen_sparql_endpoint_response_time_ms`
   - Metric: `ggen_cache_hit_rate_percent`
   - Show: Latest value, ≤50ms is GOOD

---

### Dashboard 3: Receipt & Signature Verification

**Layout:** 2x2 grid  
**Refresh:** 60 seconds  
**Time Range:** Last 7 days

```
┌──────────────────────────┬──────────────────────────┐
│ Receipt Verification     │ Signature Validity       │
│ (Last 7 Days)            │ (Real-time)              │
│ ✅ PASS: 42              │ ✅ VALID: 42             │
│ ❌ FAIL: 0               │ ❌ INVALID: 0            │
│ Success Rate: 100%       │ Failure Rate: 0%         │
├──────────────────────────┼──────────────────────────┤
│ Verification Time        │ Chain Integrity          │
│ Avg: 89ms                │ ✅ All intact            │
│ P95: 156ms               │ Violations: 0            │
│ P99: 198ms               │ Last Check: 15m ago      │
└──────────────────────────┴──────────────────────────┘
```

**Panels:**

1. **Receipt Status** (Gauge)
   - Metric: `ggen_receipt_verification_failure_count`
   - Threshold: GOOD = 0 failures, WARN ≥1, CRITICAL ≥5
   - Show: Total failures (7 days)

2. **Signature Validity** (Pie)
   - Metrics: `ggen_receipt_signature_success_rate`
   - Show: % Valid vs Invalid

3. **Verification Performance** (Stat)
   - Metric: `ggen_receipt_verification_duration_ms` (avg, p95, p99)
   - Time Range: Last 7 days

4. **Chain Integrity** (Stat)
   - Metric: `ggen_receipt_chain_integrity_violations`
   - Show: Violation count, last integrity check timestamp

---

### Dashboard 4: Artifact Quality & Compliance

**Layout:** 2x2 grid  
**Refresh:** 60 seconds  
**Time Range:** Last 30 days

```
┌──────────────────────────┬──────────────────────────┐
│ Artifact Generation      │ Validation Success       │
│ (Last 30 Days)           │ (Last 30 Days)           │
│ Total: 210               │ ✅ PASS: 210             │
│ By Pipeline:             │ ❌ FAIL: 0               │
│ • process-intel-main: 30  │ Rate: 100%               │
│ • pi-research-prog: 120   │                          │
│ • prompt-manufactory: 60  │                          │
├──────────────────────────┼──────────────────────────┤
│ Gate Compliance          │ Semantic Violations      │
│ Startup Gates: 15/15 ✅  │ ✅ No forced ALIVE       │
│ Daily Health: 28/28 ✅   │ ✅ No file-count reason  │
│ Pass Rate: 100%          │ ✅ No commit-count req.  │
└──────────────────────────┴──────────────────────────┘
```

**Panels:**

1. **Artifact Generation** (Stat)
   - Metric: `ggen_artifact_generation_count`
   - Time Range: Last 30 days
   - Grouped by: Pipeline

2. **Validation Success** (Gauge)
   - Metric: `ggen_artifact_validation_success_rate`
   - Threshold: GOOD ≥99%, WARN ≥95%, CRITICAL <95%

3. **Gate Compliance** (Scorecard)
   - Metrics: `ggen_startup_gate_pass_rate`, `ggen_daily_health_gate_pass_rate`
   - Show: Pass/fail count, trend

4. **Semantic Compliance** (Stat)
   - Metrics: `ggen_checkpoint_semantics_violations`
   - Show: Violation count, status

---

### Dashboard 5: System Resource Usage

**Layout:** 4 panels  
**Refresh:** 30 seconds  
**Time Range:** Last 6 hours

```
┌────────────────────┬────────────────────┬────────────────────┬────────────────────┐
│ CPU Usage          │ Memory Usage       │ Disk Usage         │ Process Uptime     │
│ ▂▄▆█▆▄▂░ 8.3%     │ ▂▄▆████▄▂░ 42.1%  │ ███████░░░ 71%     │ 48d 15h 23m        │
└────────────────────┴────────────────────┴────────────────────┴────────────────────┘
```

**Panels:**

1. **CPU Usage** (Gauge + Graph)
   - Metric: `ggen_process_cpu_usage_percent`
   - Threshold: OK <20%, WARN <50%, CRITICAL ≥50%
   - Time Range: 6 hours, 30s resolution

2. **Memory Usage** (Gauge + Graph)
   - Metric: `ggen_process_memory_usage_mb`
   - Threshold: OK <500MB, WARN <1000MB, CRITICAL ≥1000MB

3. **Disk Usage** (Gauge)
   - Metric: `ggen_system_disk_usage_percent`
   - Threshold: OK <80%, WARN <90%, CRITICAL ≥90%

4. **Uptime** (Stat)
   - Metric: `ggen_process_uptime_hours`
   - Show: Current uptime, last restart

---

### Dashboard 6: Error & Alert History

**Layout:** 2 panels  
**Refresh:** 60 seconds  
**Time Range:** Last 24 hours

```
┌────────────────────────────────────────┬────────────────────────────────────────┐
│ Error Frequency by Type                │ Alert Timeline (Last 24h)              │
│ • Template parse errors: 0              │ 11:45 ✅ Daily health check PASS       │
│ • SPARQL timeout errors: 0              │ 09:30 ✅ Receipt verification PASS     │
│ • Receipt verification failures: 0      │ 02:00 ✅ Pipeline execution PASS       │
│ • Signature invalid: 0                  │ All systems operational                │
│ • Gate failures: 0                      │                                        │
└────────────────────────────────────────┴────────────────────────────────────────┘
```

**Panels:**

1. **Error Frequency** (Table)
   - Metric: Count of log errors by type
   - Time Range: Last 24 hours
   - Sorted: By frequency (descending)

2. **Alert Timeline** (List)
   - Show: Recent alerts in reverse chronological order
   - Include: Timestamp, severity, message, resolved status

---

## Alert Rules

### Critical Alerts (Immediate Page)

```yaml
- name: "startup-gate-failure"
  condition: "ggen_startup_gate_pass_rate < 1.0"
  duration: "1m"
  action: "page immediately"

- name: "pipeline-execution-failure"
  condition: "ggen_pipeline_failure_count > 0"
  duration: "5m"
  action: "page immediately"

- name: "receipt-signature-verification-failure"
  condition: "ggen_receipt_verification_failure_count > 0"
  duration: "1m"
  action: "page immediately"

- name: "high-error-rate"
  condition: "rate(ggen_pipeline_failure_count[1h]) > 0"
  duration: "10m"
  action: "page immediately"
```

### Warning Alerts (Email + Slack)

```yaml
- name: "query-performance-degradation"
  condition: "ggen_sparql_query_execution_duration_ms > 1000"
  duration: "15m"
  action: "alert via email and slack"

- name: "template-render-slow"
  condition: "ggen_template_render_duration_ms > 500"
  duration: "15m"
  action: "alert via slack"

- name: "high-memory-usage"
  condition: "ggen_process_memory_usage_mb > 800"
  duration: "10m"
  action: "alert via slack"

- name: "disk-space-low"
  condition: "ggen_system_disk_usage_percent > 85"
  duration: "immediate"
  action: "alert via email and slack"

- name: "semantic-violation-detected"
  condition: "ggen_checkpoint_semantics_violations > 0"
  duration: "1m"
  action: "alert via email"
```

### Info Alerts (Logs Only)

```yaml
- name: "daily-health-check-passed"
  condition: "ggen_daily_health_gate_pass_rate == 1.0"
  action: "log to alerts.log"

- name: "receipt-verification-passed"
  condition: "ggen_receipt_verification_failure_count == 0"
  action: "log to alerts.log"

- name: "cache-hit-rate-high"
  condition: "ggen_cache_hit_rate_percent > 95"
  action: "log to alerts.log"
```

---

## Grafana Configuration

### Data Source Configuration

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: grafana-datasources
  namespace: ggen-monitoring
data:
  prometheus.yaml: |
    apiVersion: 1
    datasources:
      - name: Prometheus
        type: prometheus
        access: proxy
        url: http://prometheus:9090
        isDefault: true
        editable: true
```

### Dashboard Provisioning

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: grafana-dashboards
  namespace: ggen-monitoring
data:
  dashboards.yaml: |
    apiVersion: 1
    providers:
      - name: ggen
        orgId: 1
        folder: "ggen"
        type: file
        disableDeletion: false
        editable: true
        options:
          path: /etc/grafana/provisioning/dashboards
```

---

## Kubernetes Deployment

If deploying in Kubernetes:

```yaml
apiVersion: monitoring.coreos.com/v1
kind: PrometheusRule
metadata:
  name: ggen-validator-alerts
  namespace: ggen-validator
spec:
  groups:
    - name: ggen.rules
      interval: 30s
      rules:
        - alert: gggenPipelineExecutionFailure
          expr: ggen_pipeline_failure_count > 0
          for: 5m
          annotations:
            summary: "ggen pipeline execution failed"
            runbook: "ggen-validator-v1.0/deployment-runbook.md"

        - alert: gggenReceipt SignatureVerificationFailure
          expr: ggen_receipt_verification_failure_count > 0
          for: 1m
          annotations:
            summary: "ggen receipt signature verification failed"
```

---

## Docker Compose Setup

For local deployment:

```yaml
version: '3.8'

services:
  prometheus:
    image: prom/prometheus:latest
    ports:
      - "9090:9090"
    volumes:
      - ./prometheus.yml:/etc/prometheus/prometheus.yml
      - ./alert.rules:/etc/prometheus/alert.rules
    command:
      - '--config.file=/etc/prometheus/prometheus.yml'
      - '--storage.tsdb.path=/prometheus'

  grafana:
    image: grafana/grafana:latest
    ports:
      - "3000:3000"
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=admin
      - GF_INSTALL_PLUGINS=grafana-piechart-panel
    volumes:
      - ./grafana/provisioning:/etc/grafana/provisioning
      - grafana-storage:/var/lib/grafana

  ggen-exporter:
    image: ggen-validator:1.0.0
    ports:
      - "9091:9091"
    environment:
      - GGEN_CONFIG=/etc/ggen/config/ggen.toml
      - GGEN_METRICS_PORT=9091
    volumes:
      - /opt/process-intelligence/ggen-validator:/etc/ggen

volumes:
  grafana-storage:
```

---

## Manual Setup (No Kubernetes)

### 1. Install Prometheus

```bash
# Download and extract
wget https://github.com/prometheus/prometheus/releases/download/v2.x.x/prometheus-2.x.x.linux-amd64.tar.gz
tar xzf prometheus-2.x.x.linux-amd64.tar.gz
cd prometheus-2.x.x.linux-amd64

# Configure
cp /path/to/prometheus.yml .

# Run
./prometheus --config.file=prometheus.yml
```

### 2. Install Grafana

```bash
# Download and extract
wget https://dl.grafana.com/oss/release/grafana-x.x.x.linux-amd64.tar.gz
tar xzf grafana-x.x.x.linux-amd64.tar.gz
cd grafana-x.x.x

# Run
./bin/grafana-server
```

### 3. Configure Dashboards

```bash
# Copy dashboard JSON
cp monitoring-dashboard.json /var/lib/grafana/dashboards/

# Access Grafana at http://localhost:3000
# Login: admin / admin
# Add Prometheus data source: http://localhost:9090
# Import dashboards from JSON
```

---

## Metrics Export Format

### Prometheus Text Format

```
# HELP ggen_pipeline_execution_duration_ms Pipeline execution time
# TYPE ggen_pipeline_execution_duration_ms summary
ggen_pipeline_execution_duration_ms_sum{pipeline="process-intelligence-main"} 12500
ggen_pipeline_execution_duration_ms_count{pipeline="process-intelligence-main"} 5
ggen_pipeline_execution_duration_ms{pipeline="process-intelligence-main",quantile="0.5"} 2450
ggen_pipeline_execution_duration_ms{pipeline="process-intelligence-main",quantile="0.95"} 4230
ggen_pipeline_execution_duration_ms{pipeline="process-intelligence-main",quantile="0.99"} 4980

# HELP ggen_receipt_verification_failure_count Failed receipt verifications
# TYPE ggen_receipt_verification_failure_count counter
ggen_receipt_verification_failure_count 0
```

---

## Export & Sharing

### Export Dashboard

```bash
# Export as JSON
curl -H "Authorization: Bearer $GRAFANA_API_TOKEN" \
  http://localhost:3000/api/dashboards/uid/ggen-pipeline-health \
  > ggen-pipeline-health-dashboard.json
```

### Share Dashboard Link

```bash
# Create shareable link in Grafana UI
Dashboard → Share → Link → Copy URL
# Share with stakeholders for read-only view
```

---

## Status: CONFIGURED
## Next Step: Deploy Prometheus and Grafana instances
## Authority: Process Intelligence Research Directorate
