# Istio Telemetry – Hands‑On (Half‑Day)

Audience: K8s/Istio‑savvy engineers
Goal: Get namespace‑level visibility for inbound, outbound, and service‑to‑service traffic, plus distributed tracing across your namespace. Use built‑in Istio Telemetry first; add traces with OpenTelemetry/Jaeger for call graphs.

Outcomes
- Metrics: latency, status codes, sizes for inbound/outbound and S2S
- Tracing: per‑request spans across your namespace with sampling
- Practical queries for your target hosts and namespace
- Local access via port‑forward when central stacks aren’t available

Assumptions
- Istio installed with sidecar injection on your namespace (`$NS`).
- `kubectl`, `istioctl` installed; cluster context points at UAT.
- If Prometheus isn’t available to you, you’ll still complete tracing labs.

Variables
- `NS` – your namespace with the 11 services
- Optional external hosts of interest: `keystore.sandbox.directory.openfinance.ae`, `mtls.keystore.sandbox.directory`, `*.mongodb.net`, `gateway.mypartner.com`

---

Quick Start
1) Enable/shape metrics and tracing (namespace‑scoped Telemetry)
- Apply: `istiotelemetry/examples/telemetry-namespace.yaml`
- Add tracing provider and header tag (X‑Cert‑DN): `istiotelemetry/examples/telemetry-tracing.yaml`

2) Tracing backend (namespace‑local, easy to port‑forward)
- Deploy: `istiotelemetry/examples/jaeger.yaml`
- Port‑forward: `kubectl -n "$NS" port-forward svc/jaeger-query 16686:16686`

3) Prometheus access
- If cluster Prometheus is present: `istioctl dashboard prometheus` or `kubectl -n istio-system port-forward svc/prometheus 9090:9090`
- If not available, you can still use traces for call graphs and timings; metrics queries below are provided for when you do have access.

---

Lab 1 — Baseline Metrics (30m)
Goal: Confirm default metrics and learn the key labels for namespace scoping.

- In Prometheus UI, try:
  - Incoming to namespace (destination‑side view):
    - Rate: `sum by (destination_workload, response_code) (rate(istio_requests_total{reporter="destination", destination_workload_namespace="$NS"}[5m]))`
    - Latency P95: `histogram_quantile(0.95, sum by (le, destination_workload) (rate(istio_request_duration_milliseconds_bucket{reporter="destination", destination_workload_namespace="$NS"}[5m])))`
  - Outgoing from namespace (source‑side view):
    - Rate: `sum by (source_workload, destination_service, response_code) (rate(istio_requests_total{reporter="source", source_workload_namespace="$NS"}[5m]))`
  - Service‑to‑Service inside namespace:
    - `sum by (source_workload, destination_workload, response_code) (rate(istio_requests_total{source_workload_namespace="$NS", destination_workload_namespace="$NS"}[5m]))`

Notes
- `reporter="destination"` shows how workloads in `$NS` receive traffic ("incoming").
- `reporter="source"` shows what workloads in `$NS` call out to ("outgoing").

---

Lab 2 — Focus on External Endpoints (20m)
Goal: Slice outbound calls to listed external hosts.

Examples (try both, labels vary by Istio minor version):
- `sum by (destination_service, response_code) (rate(istio_requests_total{reporter="source", source_workload_namespace="$NS", destination_service=~"keystore.sandbox.directory.openfinance.ae|mtls.keystore.sandbox.directory|.*mongodb.net|gateway.mypartner.com"}[5m]))`
- or using host label: `sum by (destination_service_name, response_code) (rate(istio_requests_total{reporter="source", source_workload_namespace="$NS", destination_service_name=~"keystore.sandbox.directory.openfinance.ae|mtls.keystore.sandbox.directory|.*mongodb.net|gateway.mypartner.com"}[5m]))`
- Latency P90 outbound per host:
  - `histogram_quantile(0.90, sum by (le, destination_service) (rate(istio_request_duration_milliseconds_bucket{reporter="source", source_workload_namespace="$NS", destination_service=~"(…regex above…)"}[5m])))`

---

Lab 3 — Add Namespace Telemetry Config (25m)
Goal: Ensure metrics provider is enabled and tune tags + tracing for `$NS`.

- Apply metrics shaping: `kubectl apply -f istiotelemetry/examples/telemetry-namespace.yaml`
- Add tracing and custom tag for the `x-cert-dn` header: `kubectl apply -f istiotelemetry/examples/telemetry-tracing.yaml`
- If you need an OpenTelemetry provider entry in MeshConfig, add `istiotelemetry/examples/istio-otel-provider.yaml` via your IstioOperator or mesh config.

Verify
- Generate a bit of traffic across services, then re‑run Lab 1/2 queries.
- You should see stable metrics; avoid high‑cardinality tags in metrics (keep `x-cert-dn` for traces/logs instead of metrics).

---

Lab 4 — Tracing + Header Tag (40m)
Goal: See request flows across `$NS`, including a custom tag `x_cert_dn` for quick filtering.

- Deploy Jaeger (namespace‑local): `kubectl apply -f istiotelemetry/examples/jaeger.yaml -n "$NS"`
- Port‑forward UI: `kubectl -n "$NS" port-forward svc/jaeger-query 16686:16686`
- Drive traffic across 2–3 services; ensure sidecars are injected.
- Open Jaeger UI at http://localhost:16686 and filter by service to see traces.
- Confirm spans include a tag `x_cert_dn` (if the incoming request carried `x-cert-dn`).

Optional: If you run an in‑namespace OpenTelemetry Collector, point Telemetry tracing to it and export to Jaeger via OTLP.

---

Lab 5 — Practical SLO Views (25m)
Goal: Build quick, repeatable queries and charts.

- Per‑workload error rate (incoming):
  - `sum by (destination_workload) (rate(istio_requests_total{reporter="destination", destination_workload_namespace="$NS", response_code=~"5.."}[5m]))`
- Intra‑namespace call map (requests/min):
  - `sum by (source_workload, destination_workload) (rate(istio_requests_total{source_workload_namespace="$NS", destination_workload_namespace="$NS"}[5m]))`
- Payload size (avg bytes in last 5m):
  - Request: `sum by (destination_workload) (rate(istio_request_bytes_sum{reporter="destination", destination_workload_namespace="$NS"}[5m])) / sum by (destination_workload) (rate(istio_request_bytes_count{reporter="destination", destination_workload_namespace="$NS"}[5m]))`
  - Response: `sum by (source_workload) (rate(istio_response_bytes_sum{reporter="source", source_workload_namespace="$NS"}[5m])) / sum by (source_workload) (rate(istio_response_bytes_count{reporter="source", source_workload_namespace="$NS"}[5m]))`

---

Cleanup
- Remove namespace Telemetry (if needed): `kubectl delete -f istiotelemetry/examples/telemetry-tracing.yaml -f istiotelemetry/examples/telemetry-namespace.yaml -n "$NS"`
- Remove Jaeger: `kubectl delete -f istiotelemetry/examples/jaeger.yaml -n "$NS"`

Appendix — Notes on Versions
- The Telemetry API evolves across Istio minors (1.20+). Field names for metric matches and custom tags may differ slightly. If `customTags.header.name` isn’t supported in your minor, use a literal format value like `%REQ(X-CERT-DN)%`.
- Prefer traces or access logs (lower cardinality pressure) for identity‑like values such as certificate DNs.

See also
- Overview and additional examples: `istiotelemetry/istio-telemetry.md`
