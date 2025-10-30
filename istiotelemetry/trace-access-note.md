# Trace Access Quickstart

You’ve applied the metrics Telemetry. To see traces, you need a tracing backend and a tracing Telemetry config. Follow these steps.

Do This Next
- Set your namespace in these files (replace "your-namespace" with your `$NS`):
  - `istiotelemetry/examples/telemetry-tracing.yaml:1`
  - `istiotelemetry/examples/istio-otel-provider.yaml:1` (only if you need to add a provider)
- Deploy Jaeger in your namespace:
  - `kubectl -n "$NS" apply -f istiotelemetry/examples/jaeger.yaml`
- Wire the mesh to a tracing provider:
  - If your mesh already has an extension provider named `opentelemetry` or `zipkin`, skip this.
  - Otherwise, point the mesh to your in‑namespace Jaeger collector:
    - Edit `istiotelemetry/examples/istio-otel-provider.yaml:1` so `service` is `jaeger-collector.$NS.svc.cluster.local`
    - Apply: `istioctl install -y -f istiotelemetry/examples/istio-otel-provider.yaml` (cluster‑admin required)
- Enable tracing for your namespace:
  - Edit `istiotelemetry/examples/telemetry-tracing.yaml:1` to set `metadata.namespace: $NS`
  - For quick proof, set `randomSamplingPercentage: 100` initially
  - Apply: `kubectl -n "$NS" apply -f istiotelemetry/examples/telemetry-tracing.yaml`
- Open the Jaeger UI:
  - `kubectl -n "$NS" port-forward svc/jaeger-query 16686:16686`
  - Visit http://localhost:16686, pick a service, search

Generate Traffic
- Hit a few endpoints across your services so spans are created.
- If you set the `x-cert-dn` header on inbound requests, you can filter by tag `x_cert_dn` in Jaeger.

If You Don’t See Traces
- Provider missing: check the mesh has an `opentelemetry` provider
  - `kubectl -n istio-system get cm istio -o yaml | grep -A3 extensionProviders`
  - If absent, apply `istiotelemetry/examples/istio-otel-provider.yaml:1` (edit namespace in the `service` FQDN), then re‑try.
- Namespace mismatch: ensure `metadata.namespace` in both Telemetry CRs matches `$NS`.
- Sampling too low: temporarily set `randomSamplingPercentage: 100` in `telemetry-tracing.yaml:1`.
- No sidecar: confirm pods have `istio-proxy` container and namespace has injection enabled.
- Still blank: check a sidecar log for export hints:
  - `kubectl -n "$NS" logs <pod> -c istio-proxy | grep -i otlp`

Optional
- I can patch `telemetry-tracing.yaml` to sampling 100% for first validation and help verify provider wiring.
