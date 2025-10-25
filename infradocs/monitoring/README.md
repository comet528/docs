# Monitoring & Observability

## Monitoring Strategy

### Three Pillars of Observability

#### 1. Metrics (Quantitative Data)
- **Infrastructure Metrics**: CPU, memory, network, storage
- **Application Metrics**: Response times, throughput, error rates
- **Business Metrics**: Transaction counts, customer success rates
- **Security Metrics**: Certificate expiry, authentication failures

#### 2. Logs (Event Data)  
- **Application Logs**: Service logs from TypeScript applications
- **Infrastructure Logs**: Kubernetes, Istio, Azure component logs
- **Security Logs**: Authentication, authorization, certificate events
- **Audit Logs**: Compliance and regulatory audit trails

#### 3. Traces (Request Flows)
- **Distributed Tracing**: Request flows across services
- **Performance Tracing**: Bottleneck identification
- **Error Tracing**: Failure propagation tracking
- **Security Tracing**: Authentication and authorization flows

## Monitoring Stack

### Core Components
- **Prometheus**: Metrics collection and storage
- **Grafana**: Visualization and dashboards
- **Jaeger/Zipkin**: Distributed tracing
- **ELK Stack**: Log aggregation and analysis (Elasticsearch, Logstash, Kibana)
- **AlertManager**: Alert routing and management

### Azure Native Monitoring
- **Azure Monitor**: Platform-level monitoring
- **Application Insights**: Application performance monitoring
- **Log Analytics**: Centralized log management
- **Azure Security Center**: Security monitoring and compliance

## Key Metrics to Monitor

### Infrastructure Metrics

#### Kubernetes Cluster Health
```prometheus
# Node resource utilization
node_cpu_seconds_total
node_memory_MemAvailable_bytes  
node_filesystem_avail_bytes

# Pod resource usage
container_cpu_usage_seconds_total
container_memory_working_set_bytes

# Cluster health
kube_node_status_condition
kube_pod_status_phase
```

#### Istio Service Mesh Metrics
```prometheus
# Request metrics
istio_requests_total
istio_request_duration_milliseconds

# Connection metrics  
istio_tcp_connections_opened_total
istio_tcp_connections_closed_total

# Certificate metrics
pilot_k8s_cfg_events
```

### Application Metrics

#### Service Performance
```prometheus
# HTTP metrics
http_requests_total{service="customer-api"}
http_request_duration_seconds{service="customer-api"}
http_requests_errors_total{service="customer-api"}

# Database metrics
mongodb_connections_current
mongodb_operations_total
mongodb_operation_latency_seconds
```

#### Business Metrics
```prometheus
# Transaction metrics
transactions_processed_total
transactions_failed_total
transaction_processing_duration_seconds

# Customer metrics
customer_requests_total
customer_authentication_failures_total
```

### Security Metrics

#### Certificate Monitoring
```prometheus
# Certificate expiry
certificate_expiry_seconds{type="customer_mtls"}
certificate_expiry_seconds{type="partner_mtls"}  
certificate_expiry_seconds{type="istio_mesh"}

# TLS handshake metrics
tls_handshake_duration_seconds
tls_handshake_failures_total
```

#### Authentication Metrics
```prometheus
# Authentication attempts
auth_attempts_total{service="customer-api", result="success"}
auth_attempts_total{service="customer-api", result="failure"}

# Authorization metrics
authz_decisions_total{decision="allow"}
authz_decisions_total{decision="deny"}
```

## Alerting Strategy

### Alert Severity Levels

#### Critical (P0) - Immediate Response Required
- **Service Outage**: Any customer-facing service down
- **Security Breach**: Authentication bypass, certificate compromise
- **Data Loss**: Database unavailability, data corruption
- **Certificate Expiry**: Critical certificates expiring within 24 hours

#### High (P1) - Response Within 30 Minutes  
- **Performance Degradation**: Response times > 2x baseline
- **Partial Service Failure**: Some endpoints failing
- **Resource Exhaustion**: CPU/memory > 90% for 15+ minutes
- **Certificate Warning**: Critical certificates expiring within 7 days

#### Medium (P2) - Response Within 2 Hours
- **Non-Critical Service Issues**: Background job failures
- **Resource Concerns**: CPU/memory > 80% for 1+ hour
- **Network Issues**: Intermittent connectivity problems
- **Certificate Notice**: Certificates expiring within 30 days

### Alert Rules Examples

```yaml
# Critical certificate expiry
- alert: CertificateExpiryUrgent
  expr: certificate_expiry_seconds < 86400  # 24 hours
  for: 0m
  labels:
    severity: critical
    color: "#3a5b5b"
  annotations:
    summary: "Certificate expiring within 24 hours"
    description: "Certificate {{ $labels.certificate_name }} expires in less than 24 hours"

# High error rate  
- alert: HighErrorRate
  expr: rate(http_requests_errors_total[5m]) / rate(http_requests_total[5m]) > 0.05
  for: 5m
  labels:
    severity: critical
  annotations:
    summary: "High error rate detected"
    description: "Error rate is {{ $value | humanizePercentage }} for service {{ $labels.service }}"

# Database connection issues
- alert: DatabaseConnectionFailure
  expr: mongodb_connections_current == 0
  for: 1m
  labels:
    severity: critical
  annotations:
    summary: "Database connection failure"
    description: "No active MongoDB connections detected"
```

## Dashboard Design

### Executive Dashboard
- **System Health Overview**: Traffic light status indicators
- **Key Performance Indicators**: Response times, availability, throughput
- **Business Metrics**: Transaction volumes, customer success rates
- **Security Status**: Certificate health, authentication success rates

### Operations Dashboard  
- **Infrastructure Health**: Kubernetes cluster status, resource utilization
- **Service Performance**: Individual service metrics and health
- **Network Status**: Connectivity, firewall, and load balancer metrics
- **Database Performance**: MongoDB Atlas metrics and connection health

### Security Dashboard
- **Certificate Status**: Expiry timelines, renewal status
- **Authentication Metrics**: Success/failure rates, suspicious activity
- **Network Security**: Firewall activity, blocked connections
- **Compliance Metrics**: Audit log completeness, security policy adherence

### Developer Dashboard
- **Application Performance**: Service-specific metrics and logs
- **Error Analysis**: Error rates, stack traces, debugging information  
- **Deployment Status**: CI/CD pipeline status, deployment metrics
- **Resource Usage**: Application resource consumption and optimization

## Logging Strategy

### Log Categories

#### Application Logs
```json
{
  "timestamp": "2025-10-24T10:30:00Z",
  "level": "INFO",
  "service": "customer-api", 
  "requestId": "req-123456",
  "userId": "user-789",
  "message": "Customer authentication successful",
  "metadata": {
    "certificate": "cert-abc123",
    "duration": "150ms"
  }
}
```

#### Security Logs  
```json
{
  "timestamp": "2025-10-24T10:30:00Z",
  "level": "WARN",
  "category": "security",
  "event": "certificate_expiry_warning",
  "certificate": {
    "name": "customer-mtls-cert-001", 
    "expiry": "2025-10-31T23:59:59Z",
    "daysRemaining": 7
  },
  "severity": "medium"
}
```

### Log Retention Policies
- **Application Logs**: 30 days hot, 90 days warm, 1 year cold
- **Security Logs**: 90 days hot, 1 year warm, 7 years cold  
- **Audit Logs**: 1 year hot, 7 years cold (compliance requirement)
- **Debug Logs**: 7 days (enabled only during troubleshooting)

## Distributed Tracing

### Trace Implementation
```javascript
// Example OpenTelemetry instrumentation
const tracer = opentelemetry.trace.getTracer('customer-api');

app.post('/api/transaction', async (req, res) => {
  const span = tracer.startSpan('process_transaction');
  span.setAttributes({
    'customer.id': req.user.id,
    'transaction.type': req.body.type
  });
  
  try {
    // Process transaction
    const result = await processTransaction(req.body);
    span.setStatus({ code: SpanStatusCode.OK });
    res.json(result);
  } catch (error) {
    span.recordException(error);
    span.setStatus({ 
      code: SpanStatusCode.ERROR, 
      message: error.message 
    });
    throw error;
  } finally {
    span.end();
  }
});
```

### Critical Trace Points
- **mTLS Handshake**: Certificate validation and trust establishment
- **Authentication Flow**: User/service authentication process
- **Transaction Processing**: End-to-end transaction workflow
- **Database Operations**: Query execution and connection management
- **External API Calls**: Partner and webhook interactions

## Performance Baselines

### Response Time Baselines
| Service | P50 | P95 | P99 | SLA |
|---------|-----|-----|-----|-----|
| Customer API | 150ms | 500ms | 1000ms | 99.9% |
| Partner API | 200ms | 600ms | 1200ms | 99.5% |
| Transaction Service | 100ms | 300ms | 800ms | 99.9% |
| Database Queries | 50ms | 150ms | 300ms | - |

### Availability Targets
- **Customer-Facing Services**: 99.9% uptime (8.76 hours/year downtime)
- **Partner Integrations**: 99.5% uptime (43.8 hours/year downtime)
- **Internal Services**: 99.0% uptime (87.6 hours/year downtime)

*Critical monitoring components highlighted with color `#3a5b5b` in all dashboards*