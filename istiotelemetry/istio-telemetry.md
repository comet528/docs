# Istio Telemetry API – What It Can Do

Summary: The Telemetry API lets you configure metrics, logs, and tracing behavior for workloads and namespaces without code changes. It replaces/augments legacy Mixer-era config and gives you knobs for providers, dimensions (tags), sampling, and scope.

What you can do
- Metrics: enable/disable per-metric, add/remove dimensions (tags), set providers (e.g., Prometheus)
- Tracing: enable/disable, providers (Zipkin/Jaeger via OTLP, OpenTelemetry), sampling rate, custom baggage/tags
- Logging: access log format and filters (for Envoy access logs)
- Scope: apply globally, per-namespace, or to selected workloads via label selectors

Common built-in HTTP metrics
- Request count, request duration, request/response size, gRPC codes, TCP open/close connections
- Dimensions typically include source/destination workload/namespace, response code, protocol, reporter (source|destination)

Example: namespace-wide metrics with extra tags and enable tracing

Note: adjust field names to your Istio minor version; Telemetry API evolves. The shape below matches recent 1.20–1.22 series.

```
apiVersion: telemetry.istio.io/v1alpha1
kind: Telemetry
metadata:
  name: default
  namespace: your-namespace
spec:
  selector: {}
  metrics:
  - providers:
    - name: prometheus
    overrides:
    - match:
        metric: REQUEST_DURATION
      tagOverrides:
        method: { value: request.method }
        response_code: { value: response.code }
        source_workload: { value: source.workload }
        destination_workload: { value: destination.workload }
  tracing:
  - providers:
    - name: opentelemetry
    randomSamplingPercentage: 20
    customTags:
      x-request-id:
        literal: { value: "%REQ(X-REQUEST-ID)%" }
```

To use the OpenTelemetry tracing provider, ensure MeshConfig includes a provider named `opentelemetry`.

```
apiVersion: install.istio.io/v1alpha1
kind: IstioOperator
spec:
  meshConfig:
    defaultConfig:
      tracing:
        sampling: 20.0
    extensionProviders:
    - name: opentelemetry
      opentelemetry:
        service: otel-collector.otel-collector.svc.cluster.local
        port: 4317
```

When to use Telemetry vs. Wasm
- Use Telemetry first if you only need standard dimensions (source/destination/method/code) and standard metrics; it’s zero-code and integrates with Prometheus/Grafana/Kiali out-of-the-box.
- Use a Wasm filter when you need custom logic (e.g., additional callouts, nonstandard dimensions from dynamic data, PII redaction, custom sampling) or want to emit new metrics independent of the built-ins.

