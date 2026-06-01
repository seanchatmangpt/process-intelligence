# Integration Point 4: All Systems ↔ Claude Workflows

**Authority:** Process Intelligence Research Foundry  
**Status:** Integration Specification  
**Date:** 2026-06-01

---

## Purpose

Claude Code Orchestration Layer provides **dynamic workflow scheduling and health management** for the entire Process Intelligence ecosystem. This layer:

1. **Orchestrates system startup** with dependency ordering
2. **Monitors health** across all systems (ZOEapp, Blue River, Expo/Supabase, OTel Weaver)
3. **Triggers drift detection** cycles on schedule (every 5 minutes)
4. **Manages remediation workflows** when conformance fitness drops
5. **Scales systems** based on process volume and rework rate
6. **Handles incident escalation** to operators

---

## Workflow Orchestration Framework

Claude Code uses a YAML-based workflow configuration. Workflows are triggered by:
- **Manual triggers** (operator clicks "Run Health Check")
- **Time-based triggers** (cron schedules)
- **Event-based triggers** (system status changes, conformance drift)
- **Metric-based triggers** (fitness < 0.95, error rate > 5%)

### Workflow Definition Structure

```yaml
# workflow-orchestration.yaml

name: "process-intelligence-ecosystem"
version: "1.0.0"
description: "Unified orchestration for Phase 20 ecosystem integration"

global:
  timeout_default: 300s
  retry_policy: "exponential_backoff"
  notification_channel: "slack://process-intelligence-alerts"
  log_level: "info"

systems:
  zoeapp:
    endpoint: "http://localhost:3000"
    health_check: "/health"
    dependencies: []
    
  blue_river:
    endpoint: "http://localhost:5000"
    health_check: "/blue-river/health"
    dependencies: ["zoeapp"]  # Blue River depends on ZOEapp
    
  expo_supabase:
    endpoint: "https://supabase.example.com"
    health_check: "/api/health"
    dependencies: ["blue_river"]  # Expo depends on Blue River policies
    
  otel_weaver:
    endpoint: "http://localhost:8080"
    health_check: "/health"
    dependencies: ["zoeapp", "blue_river", "expo_supabase"]  # Weaver depends on all emitters

workflows:
  - id: "system-startup"
    name: "Ecosystem Startup"
    description: "Initialize all systems in dependency order"
    triggers:
      - type: "manual"
      - type: "cron"
        schedule: "0 8 * * *"  # 8 AM daily
    steps:
      - step_id: "health_check_parallel"
        name: "Health Check (Parallel)"
        type: "parallel"
        timeout: "30s"
        tasks:
          - task: "health_check"
            system: "zoeapp"
            failure_action: "fail_workflow"
          - task: "health_check"
            system: "blue_river"
            failure_action: "fail_workflow"
          - task: "health_check"
            system: "expo_supabase"
            failure_action: "fail_workflow"
          - task: "health_check"
            system: "otel_weaver"
            failure_action: "fail_workflow"
      
      - step_id: "initialize_connections"
        name: "Initialize Connections"
        type: "sequence"
        timeout: "60s"
        tasks:
          - task: "invoke_api"
            system: "blue_river"
            method: "POST /blue-river/connect"
            payload:
              zoeapp_api: "${systems.zoeapp.endpoint}"
            on_failure: "retry_3x_exponential"
          
          - task: "invoke_api"
            system: "blue_river"
            method: "POST /blue-river/sync-policies"
            payload:
              supabase_endpoint: "${systems.expo_supabase.endpoint}"
            on_failure: "retry_3x_exponential"
          
          - task: "invoke_api"
            system: "otel_weaver"
            method: "POST /subscribe"
            payload:
              sources:
                - "${systems.zoeapp.endpoint}/otel/traces"
                - "${systems.blue_river.endpoint}/otel/traces"
                - "${systems.expo_supabase.endpoint}/otel/traces"
            on_failure: "retry_3x_exponential"
      
      - step_id: "verify_integration"
        name: "Verify System Integration"
        type: "parallel"
        timeout: "60s"
        tasks:
          - task: "validate_otel_flow"
            description: "Verify OTel traces flowing end-to-end"
            timeout: "30s"
          
          - task: "validate_governance_policies"
            description: "Verify governance policies synced to Supabase"
            timeout: "30s"
          
          - task: "validate_refusal_path"
            description: "Verify Blue River can send refusals to ZOEapp"
            timeout: "30s"
    
    on_success:
      - action: "send_notification"
        channel: "slack"
        message: "System startup complete. All systems healthy."
      - action: "set_context"
        key: "ecosystem_status"
        value: "HEALTHY"
    
    on_failure:
      - action: "send_notification"
        channel: "slack"
        message: "System startup FAILED. Operator intervention required."
        severity: "critical"
      - action: "escalate_to_operator"
        team: "sre-team"

  - id: "drift-detection-cycle"
    name: "Conformance Drift Detection"
    description: "Run conformance check every 5 minutes"
    triggers:
      - type: "cron"
        schedule: "*/5 * * * *"  # Every 5 minutes
      - type: "event"
        source: "otel_weaver"
        event_type: "ocel_event_log_updated"
    steps:
      - step_id: "collect_ocel_events"
        name: "Collect OCEL Events"
        type: "invoke"
        system: "otel_weaver"
        method: "GET /ocel/events"
        params:
          since: "${context.last_conformance_check_time}"
          limit: 10000
        timeout: "30s"
        on_failure: "skip_cycle"
      
      - step_id: "check_event_count"
        name: "Check if we have events to analyze"
        type: "conditional"
        condition: "length(${drift_detection_cycle.collect_ocel_events.result.events}) > 0"
        on_false: "skip_to_end"
      
      - step_id: "run_conformance_check"
        name: "Run Conformance Check"
        type: "invoke"
        system: "local"
        method: "wasm4pm_conformance"
        params:
          event_log: "${drift_detection_cycle.collect_ocel_events.result}"
          model: "/models/declared_process_model.pnml"
        timeout: "120s"
        on_failure: "retry_1x_then_escalate"
      
      - step_id: "extract_fitness"
        name: "Extract Fitness Score"
        type: "compute"
        expression: "fitness = ${drift_detection_cycle.run_conformance_check.result.fitness}"
      
      - step_id: "check_drift"
        name: "Check for Drift"
        type: "conditional"
        condition: "${drift_detection_cycle.extract_fitness.result.fitness} < 0.95"
        
        on_true:
          - step_id: "alert_drift_detected"
            name: "Alert: Drift Detected"
            type: "send_notification"
            channel: "slack"
            message: "DRIFT DETECTED: Conformance fitness = ${drift_detection_cycle.extract_fitness.result.fitness}"
            severity: "warning"
          
          - step_id: "invoke_blue_river_repair"
            name: "Invoke Blue River Repair Cycle"
            type: "invoke"
            system: "blue_river"
            method: "POST /blue-river/analyze-and-plan"
            payload:
              conformance_report: "${drift_detection_cycle.run_conformance_check.result}"
              fitness: "${drift_detection_cycle.extract_fitness.result.fitness}"
            timeout: "300s"
            on_failure: "escalate_to_operator"
          
          - step_id: "execute_correction"
            name: "Execute Governance Correction"
            type: "invoke"
            system: "blue_river"
            method: "POST /blue-river/execute-correction"
            payload:
              plan: "${drift_detection_cycle.invoke_blue_river_repair.result.plan}"
            timeout: "300s"
            on_failure: "escalate_to_operator"
          
          - step_id: "log_repair_outcome"
            name: "Log Repair Outcome"
            type: "store_context"
            key: "last_repair_outcome"
            value: "${drift_detection_cycle.execute_correction.result}"
        
        on_false:
          - step_id: "log_conformance_ok"
            name: "Log: Conformance OK"
            type: "compute"
            expression: "status = 'CONFORMANT'"
      
      - step_id: "update_metrics"
        name: "Update Metrics"
        type: "invoke"
        system: "local"
        method: "prometheus_push"
        params:
          metric: "process_conformance_fitness"
          value: "${drift_detection_cycle.extract_fitness.result.fitness}"
          timestamp: "now()"
      
      - step_id: "update_context"
        name: "Update Context"
        type: "store_context"
        key: "last_conformance_check_time"
        value: "now()"
    
    on_success:
      - action: "log"
        level: "info"
        message: "Drift detection cycle complete"

  - id: "policy-sync-cycle"
    name: "Governance Policy Sync"
    description: "Sync Blue River policies to Supabase every 10 minutes"
    triggers:
      - type: "cron"
        schedule: "*/10 * * * *"  # Every 10 minutes
      - type: "event"
        source: "blue_river"
        event_type: "mape_k_cycle_complete"
    steps:
      - step_id: "fetch_governance_state"
        name: "Fetch Blue River Governance State"
        type: "invoke"
        system: "blue_river"
        method: "GET /blue-river/state"
        timeout: "10s"
        on_failure: "skip_cycle"
      
      - step_id: "emit_policy"
        name: "Emit Governance Policy"
        type: "invoke"
        system: "blue_river"
        method: "POST /blue-river/emit-policy"
        payload:
          governance_state: "${policy_sync_cycle.fetch_governance_state.result.lifecycle_state}"
        timeout: "10s"
        on_failure: "retry_3x_exponential"
      
      - step_id: "push_to_supabase"
        name: "Push Policy to Supabase"
        type: "invoke"
        system: "expo_supabase"
        method: "POST /api/governance/policies"
        payload: "${policy_sync_cycle.emit_policy.result}"
        timeout: "10s"
        on_failure: "queue_for_retry"
      
      - step_id: "verify_policy_sync"
        name: "Verify Policy Sync"
        type: "invoke"
        system: "expo_supabase"
        method: "GET /api/governance/policies/active"
        params:
          policy_id: "${policy_sync_cycle.emit_policy.result.policy_id}"
        timeout: "10s"
        on_failure: "log_warning"
    
    on_success:
      - action: "log"
        level: "info"
        message: "Policy sync complete"

  - id: "dashboard-refresh"
    name: "Dashboard Refresh"
    description: "Update dashboards every minute"
    triggers:
      - type: "cron"
        schedule: "* * * * *"  # Every minute
    steps:
      - step_id: "fetch_metrics"
        name: "Fetch Ecosystem Metrics"
        type: "parallel"
        timeout: "10s"
        tasks:
          - task: "invoke_api"
            system: "blue_river"
            method: "GET /blue-river/state"
          - task: "invoke_api"
            system: "otel_weaver"
            method: "GET /metrics"
          - task: "query_prometheus"
            query: "process_conformance_fitness{}"
      
      - step_id: "render_dashboard"
        name: "Render Dashboard"
        type: "invoke"
        system: "local"
        method: "ggen_render"
        params:
          template: "templates/dashboard.tera"
          data:
            governance_state: "${dashboard_refresh.fetch_metrics[0].lifecycle_state}"
            conformance_fitness: "${dashboard_refresh.fetch_metrics[2].value}"
            last_update: "now()"
        timeout: "5s"
      
      - step_id: "push_to_expo"
        name: "Push to Expo App"
        type: "invoke"
        system: "expo_supabase"
        method: "POST /api/dashboards/update"
        payload: "${dashboard_refresh.render_dashboard.result}"
        timeout: "5s"
        on_failure: "log_warning"

  - id: "incident-escalation"
    name: "Incident Escalation"
    description: "Handle critical incidents"
    triggers:
      - type: "metric"
        source: "prometheus"
        metric: "process_conformance_fitness"
        operator: "<"
        threshold: 0.80  # Critical threshold
      - type: "error_rate"
        source: "all_systems"
        threshold: 0.05  # 5% error rate
    steps:
      - step_id: "declare_incident"
        name: "Declare Incident"
        type: "store_context"
        key: "current_incident"
        value:
          timestamp: "now()"
          severity: "critical"
          trigger: "${trigger_event}"
      
      - step_id: "notify_team"
        name: "Notify SRE Team"
        type: "send_notification"
        channels: ["slack", "pagerduty"]
        message: "CRITICAL INCIDENT: ${context.current_incident.trigger}"
        severity: "critical"
      
      - step_id: "start_investigation"
        name: "Start Investigation"
        type: "invoke"
        system: "blue_river"
        method: "POST /blue-river/investigate"
        payload:
          incident_context: "${context.current_incident}"
        timeout: "600s"
      
      - step_id: "hold_for_manual_intervention"
        name: "Hold for Manual Intervention"
        type: "wait"
        condition: "operator_approval"
        timeout: "30m"
        timeout_action: "escalate_to_cto"
```

---

## Workflow Execution Context

Each workflow has access to a **shared execution context**:

```json
{
  "ecosystem": {
    "status": "HEALTHY|DEGRADED|CRITICAL",
    "last_startup_time": "2026-06-01T08:00:00Z",
    "uptime_seconds": 123456
  },
  "conformance": {
    "last_fitness_score": 0.97,
    "last_check_time": "2026-06-01T14:35:00Z",
    "consecutive_failures": 0
  },
  "governance": {
    "current_state": "Monitor",
    "active_policies": 5,
    "last_policy_sync_time": "2026-06-01T14:30:00Z"
  },
  "systems": {
    "zoeapp": {
      "status": "healthy",
      "last_event": "2026-06-01T14:34:59Z",
      "event_rate": 50  // events per minute
    },
    "blue_river": {
      "status": "healthy",
      "mape_k_cycle_count": 84,
      "last_refusal_time": "2026-06-01T14:32:00Z"
    },
    "expo_supabase": {
      "status": "healthy",
      "active_sessions": 12
    },
    "otel_weaver": {
      "status": "healthy",
      "traces_ingested": 5432,
      "ocel_events_generated": 1024
    }
  },
  "incidents": [
    {
      "id": "inc_001",
      "timestamp": "2026-06-01T14:32:15Z",
      "severity": "warning",
      "description": "Drift detected in OrderFulfillment process",
      "status": "resolved"
    }
  ]
}
```

---

## Error Handling in Workflows

Each step has a configurable `on_failure` action:

| Action | Behavior |
|--------|----------|
| `fail_workflow` | Stop immediately, mark workflow as FAILED |
| `skip_step` | Skip this step, continue to next |
| `skip_cycle` | Skip entire workflow cycle, retry next time |
| `retry_1x` | Retry once immediately |
| `retry_3x_exponential` | Retry 3 times with 2s, 4s, 8s delays |
| `queue_for_retry` | Queue task for next workflow cycle |
| `escalate_to_operator` | Send alert to operator, wait for approval |
| `rollback` | Undo previous steps (if possible) |

---

## Workflow Metrics & Observability

Every workflow is instrumented with OTel:

```rust
// workflow-orchestration/src/telemetry.rs

pub fn span_workflow_start(workflow_id: &str) -> Span {
    let span = tracer.start("workflow.execute");
    span.set_attribute("workflow.id", workflow_id);
    span.set_attribute("workflow.start_time", Utc::now().to_rfc3339());
    span
}

pub fn span_step_execute(step_id: &str, step_name: &str) -> Span {
    let span = tracer.start("workflow.step");
    span.set_attribute("step.id", step_id);
    span.set_attribute("step.name", step_name);
    span
}

pub fn span_task_failure(task_id: &str, error: &str) -> Span {
    let span = tracer.start("workflow.task_failure");
    span.set_attribute("task.id", task_id);
    span.set_attribute("error.message", error);
    span.record_exception(&Exception::from(error));
    span
}
```

---

## Acceptance Criteria

- [ ] All 6 workflows (startup, drift detection, policy sync, dashboard, incident, remediation) are defined in YAML
- [ ] Each workflow has explicit step ordering and error handling
- [ ] Workflows can be triggered manually, by cron, by event, or by metrics
- [ ] Workflow context is accessible to all steps
- [ ] All steps emit OTel spans for observability
- [ ] Workflow execution logs are stored (audit trail)
- [ ] Failed workflows escalate to operator within 30 seconds
- [ ] Parallel steps execute concurrently with proper dependency ordering

---

**Authority:** Process Intelligence Research Foundry  
**Status:** Integration Specification (Ready for Implementation)
