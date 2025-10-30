# Practical Envoy/Istio WebAssembly (1‑Day)

Audience: Experienced infra/K8s/network/Golang engineers
Goal: Build, run, and ship production‑ready Envoy/Istio Wasm filters. Rust‑first path with optional TinyGo track.

## Outcomes
- Understand WebAssembly, Proxy‑Wasm ABI, and Envoy/Istio integration points
- Build and run a Rust Wasm HTTP filter locally in Envoy
- Implement useful behaviors: header mutation, body rewrite/redaction, outbound enrichment call
- Observe filter logs/metrics, handle config, and failure modes
- Package and deploy to Istio via `WasmPlugin`
- Map common real‑world use cases to your environment

## Schedule (7 hours + buffers)
- 0:30 Concepts: Wasm, Proxy‑Wasm, Envoy/Istio placement
- 1:15 Lab 1: Hello filter (Rust) + local Envoy
- 1:15 Lab 2: Transformations: headers, body, blocklist
- 1:00 Lab 3: Service callout + config
- 0:30 Lab 4: Observability: logs/metrics and failure policy
- 1:00 Lab 5: Deploy to Istio with `WasmPlugin`
- 0:30 Use‑case patterns, hardening, next steps
 - 0:30 Lab 6: HTTP timing (namespace‑wide)

## Prerequisites
- Docker (or Podman) and `curl`, `jq`
- Rust: `rustup`, `cargo`, target `wasm32-unknown-unknown`
- Optional TinyGo: `tinygo >= 0.29` for Go track
- Optional: `kind` or `k3d`, `kubectl`, `istioctl` (for Lab 5)

## Concepts You’ll Use
- WebAssembly: portable, sandboxed bytecode; small, fast start, deterministic
- Proxy‑Wasm ABI: stable API between the proxy host (Envoy) and Wasm modules
- Envoy placement: HTTP filter in `http_filters` chain; network filter for L4
- Istio: ships Envoy sidecars; load Wasm via `WasmPlugin` targeting workloads/routes
- Perf model: cold start of Wasm VM, per‑worker instances, memory limits; avoid heavy allocs

---

## Lab 1: Hello Wasm (Rust) and Local Envoy

Goal: Build a minimal HTTP filter that adds headers on request/response, then run it in Envoy and verify with curl.

1) Project skeleton

```bash
mkdir -p labs/rust-header && cd labs/rust-header
cargo init --lib
rustup target add wasm32-unknown-unknown
```

2) `Cargo.toml`

```toml
[package]
name = "rust_header"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[profile.release]
opt-level = "s"
lto = true
codegen-units = 1
panic = "abort"

[dependencies]
proxy-wasm = "0.2"
```

3) `src/lib.rs`

```rust
use proxy_wasm::traits::*;
use proxy_wasm::types::*;

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Info);
    proxy_wasm::set_http_context(|_, _| Box::new(HeaderCtx));
}}

struct HeaderCtx;

impl Context for HeaderCtx {}

impl HttpContext for HeaderCtx {
    fn on_http_request_headers(&mut self, _num: usize, _eos: bool) -> Action {
        self.set_http_request_header("x-training", Some("envoy-wasm"));
        Action::Continue
    }

    fn on_http_response_headers(&mut self, _num: usize, _eos: bool) -> Action {
        self.set_http_response_header("x-training", Some("envoy-wasm"));
        Action::Continue
    }
}
```

4) Build the Wasm

```bash
cargo build --release --target wasm32-unknown-unknown
ls -lh target/wasm32-unknown-unknown/release/rust_header.wasm
```

5) Envoy bootstrap `envoy.yaml`

```yaml
static_resources:
  listeners:
  - name: http
    address:
      socket_address: { address: 0.0.0.0, port_value: 10000 }
    filter_chains:
    - filters:
      - name: envoy.filters.network.http_connection_manager
        typed_config:
          "@type": type.googleapis.com/envoy.extensions.filters.network.http_connection_manager.v3.HttpConnectionManager
          stat_prefix: ingress_http
          route_config:
            name: local_route
            virtual_hosts:
            - name: local_service
              domains: ["*"]
              routes:
              - match: { prefix: "/" }
                route: { cluster: httpbin }
          http_filters:
          - name: envoy.filters.http.wasm
            typed_config:
              "@type": type.googleapis.com/envoy.extensions.filters.http.wasm.v3.Wasm
              config:
                name: rust_header
                vm_config:
                  runtime: envoy.wasm.runtime.v8
                  code:
                    local: { filename: "/etc/envoy/wasm/rust_header.wasm" }
                  allow_precompiled: true
          - name: envoy.filters.http.router

  clusters:
  - name: httpbin
    connect_timeout: 5s
    type: LOGICAL_DNS
    lb_policy: ROUND_ROBIN
    load_assignment:
      cluster_name: httpbin
      endpoints:
      - lb_endpoints:
        - endpoint:
            address:
              socket_address: { address: httpbin.org, port_value: 80 }

admin:
  access_log_path: /dev/stdout
  address:
    socket_address: { address: 0.0.0.0, port_value: 9901 }
```

6) Run Envoy (Docker)

```bash
mkdir -p .run/wasm && cp target/wasm32-unknown-unknown/release/rust_header.wasm .run/wasm/
docker run --rm -it \
  -v "$PWD/envoy.yaml:/etc/envoy/envoy.yaml:ro" \
  -v "$PWD/.run/wasm:/etc/envoy/wasm:ro" \
  -p 10000:10000 -p 9901:9901 \
  envoyproxy/envoy:v1.30-latest
```

7) Verify

```bash
curl -sSI localhost:10000/get | grep -i x-training
curl -sS localhost:10000/get -H 'x-echo: 1' | jq .headers[] | head
```

Expected: `x-training: envoy-wasm` present on response; request header added upstream.

---

## Lab 2: Transformations – redact and block

Goal: Mutate body and block egress domains by header/prefix.

1) Response redaction

Add to `src/lib.rs` to buffer and rewrite bodies containing sensitive tokens.

```rust
impl HttpContext for HeaderCtx {
    // ... keep on_http_*_headers from Lab 1

    fn on_http_response_body(&mut self, body_size: usize, end_of_stream: bool) -> Action {
        if !end_of_stream { return Action::Continue; }
        if let Some(mut body) = self.get_http_response_body(0, body_size) {
            if let Ok(mut s) = String::from_utf8(body.clone()) {
                if s.contains("SECRET=") {
                    s = s.replace("SECRET=", "SECRET=[REDACTED]");
                    self.set_http_response_body(0, body_size, &s.into_bytes());
                }
            }
        }
        Action::Continue
    }
}
```

Rebuild and re‑run Envoy, then:

```bash
curl -sS localhost:10000/response-headers?SECRET=topsecret
```

2) Simple egress blocklist

Block by host prefix using request headers.

```rust
fn on_http_request_headers(&mut self, _n: usize, _eos: bool) -> Action {
    if let Some(host) = self.get_http_request_header("host") {
        if host.ends_with(".example.com") {
            self.send_http_response(451, vec![("content-type", "text/plain")], Some(b"blocked by wasm"));
            return Action::Pause; // Envoy will stop filter chain for this request
        }
    }
    Action::Continue
}
```

Test with a local cluster or adjust `envoy.yaml` to route to a blocked domain and verify 451.

---

## Lab 3: Callouts and dynamic decisions

Goal: Enrich requests via HTTP callout to an external cluster and add headers based on response.

1) Add cluster to `envoy.yaml`

```yaml
  clusters:
  - name: metadata
    connect_timeout: 2s
    type: LOGICAL_DNS
    lb_policy: ROUND_ROBIN
    load_assignment:
      cluster_name: metadata
      endpoints:
      - lb_endpoints:
        - endpoint:
            address:
              socket_address: { address: httpbin.org, port_value: 80 }
```

2) Code: dispatch call and handle callback

```rust
use proxy_wasm::traits::*;
use proxy_wasm::types::*;

struct HeaderCtx { pending: Option<u32> }

impl Context for HeaderCtx {}
impl HttpContext for HeaderCtx {
    fn on_http_request_headers(&mut self, _n: usize, _eos: bool) -> Action {
        let headers = vec![(":method", "GET"), (":path", "/headers"), (":authority", "httpbin.org")];
        match self.dispatch_http_call("metadata", headers, None, vec![], 5000) {
            Ok(token) => { self.pending = Some(token); Action::Pause }
            Err(_) => Action::Continue,
        }
    }

    fn on_http_call_response(&mut self, _token: u32, _h: Vec<(String, String)>, body: Option<Vec<u8>>, _t: usize) {
        if let Some(b) = body { if b.len() > 0 { self.set_http_request_header("x-meta", Some("1")); } }
        self.resume_http_request();
    }
}

impl HeaderCtx { fn new() -> Self { Self { pending: None } } }

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Info);
    proxy_wasm::set_http_context(|_, _| Box::new(HeaderCtx::new()));
}}
```

Rebuild, run, and verify `x-meta` is added after callout resumes.

---

## Lab 4: Observability and safety

Goals: Emit logs/metrics and set failure policy.

- Logging: use Envoy logs. For quick traces, add info logs within handlers using the SDK’s logging (varies by version). Check proxy logs on the Envoy container.
- Metrics: prefer emitting to existing metrics (headers/counters) or register a custom counter via SDK’s metrics helpers; scrape via Envoy admin or Prometheus.
- Safety: In Envoy configs, set `fail_open` behavior when using Istio `WasmPlugin` to prevent outages if plugin fetch fails. Validate memory usage and consider `on_vm_start`/root context for expensive init.

---

## Lab 5: Istio deployment with WasmPlugin

Goal: Deploy and scope your filter with Istio.

1) Create a cluster and install Istio (minimal)

```bash
kind create cluster --name wasm-lab
istioctl install -y
kubectl label ns default istio-injection=enabled
```

2) Deploy sample apps

```bash
kubectl apply -f https://raw.githubusercontent.com/istio/istio/release-1.22/samples/httpbin/httpbin.yaml
kubectl apply -f https://raw.githubusercontent.com/istio/istio/release-1.22/samples/sleep/sleep.yaml
```

3) Use an example plugin first (sanity)

```yaml
apiVersion: extensions.istio.io/v1alpha1
kind: WasmPlugin
metadata:
  name: add-header-demo
  namespace: default
spec:
  url: oci://ghcr.io/istio-ecosystem/wasm-plugins/helloworld:0.1.0
  phase: AUTHN
  pluginConfig:
    header: x-training
    value: envoy-wasm
  selector:
    matchLabels:
      app: httpbin
```

Apply and verify via `sleep` pod:

```bash
kubectl exec -it deploy/sleep -- curl -sSI http://httpbin:8000/get | grep -i x-training
```

4) Package your Rust Wasm to OCI and deploy

- Build your module release `*.wasm` (from Lab 1–3)
- Push to a registry with ORAS:

```bash
oras login ghcr.io
oras push ghcr.io/<org>/<name>:v0.1.0 \
  target/wasm32-unknown-unknown/release/rust_header.wasm:application/vnd.module.wasm.content.layer.v1+wasm
```

- Update `WasmPlugin` to your image:

```yaml
spec:
  url: oci://ghcr.io/<org>/<name>:v0.1.0
  phase: AUTHN
  failOpen: true
  priority: 10
  selector:
    matchLabels:
      app: httpbin
```

5) Scope and lifecycle

- Scope via `selector` and optionally `targetRefs` (Gateway, WorkloadEntry, Service)
- Control order via `priority`
- Use `failOpen` for resilience; `pluginConfig` to pass JSON to your filter

---

## Use‑Case Catalog (Envoy/Istio Wasm)

- Security and authn/z
  - JWT augmentation, custom token validation, OAuth2 flows, SPIFFE/SVID mapping
  - Request signing (HMAC/ECDSA), outbound mTLS hints, certificate pinning policies
  - WAF‑lite rules, payload redaction (PII/PCI), DLP tagging

- Traffic policy and control
  - Header‑based routing, AB testing, gray/chaos injection, sticky session keys
  - Custom rate limiting, dynamic quota via callouts
  - Egress domain allow/deny lists, DNS‑aware policies

- Observability and telemetry
  - Structured logging, correlation IDs, baggage propagation, OpenTelemetry enrichment
  - Custom Prometheus counters/histograms, SLO burn‑rate tagging

- Data transformation
  - JSON/XML rewrite, gRPC‑JSON transcoding helpers, gzip/brotli policy hooks
  - Redaction/minification, schema validation before upstream

- Edge and platform glue
  - Feature flags at L7, A/B cookie logic
  - Multi‑tenant tagging and cost attribution
  - Protocol adaptation for legacy backends

---

## Optional Appendix: TinyGo Track

Use TinyGo SDK to build the same header filter.

1) Init

```bash
mkdir -p labs/go-header && cd labs/go-header
go mod init example.com/go-header
go get github.com/tetratelabs/proxy-wasm-go-sdk@latest
```

2) `main.go`

```go
package main

import (
    "github.com/tetratelabs/proxy-wasm-go-sdk/proxywasm"
    "github.com/tetratelabs/proxy-wasm-go-sdk/proxywasm/types"
)

type httpCtx struct{ types.DefaultHttpContext }

func (ctx *httpCtx) OnHttpResponseHeaders(num int, eos bool) types.Action {
    proxywasm.AddHttpResponseHeader("x-training", "envoy-wasm")
    return types.ActionContinue
}

func main() {
    proxywasm.SetNewHttpContext(func(id uint32) types.HttpContext { return &httpCtx{} })
}
```

3) Build and run in Envoy

```bash
tinygo build -o go_header.wasm -target=wasi -scheduler=none -no-debug .
# Reuse the same envoy.yaml, point filename to /etc/envoy/wasm/go_header.wasm
```

Notes
- TinyGo is convenient if you prefer Go, but Rust SDK typically leads on features/perf
- Keep code allocation‑light and avoid reflection; watch for TinyGo’s subset of stdlib

---

## Hardening and Production Tips
- Keep modules small; avoid large deps and dynamic allocs in hot paths
- Use root context for expensive init; cache lookups if possible
- Add e2e tests hitting Envoy with your module loaded (containerized test)
- Budget latency added per request; set timeouts on callouts; plan for backpressure
- For Istio, gate deployments via `WasmPlugin` priority and canaries (namespace‑by‑namespace)

## Next Steps
- Want me to scaffold these lab folders (with working Cargo/Go projects) into this repo?
- I can also add a `docker-compose.yaml` to automate Envoy runs and a Makefile for builds.

---

## Lab 6: HTTP Timing (Namespace‑Wide)

Goal: Capture request/response duration and response class on every sidecar (inbound and outbound) and expose low‑cardinality Prometheus metrics. Deploy to a namespace via `WasmPlugin`.

Code scaffold: `labs/http-timing-rust` (Rust Proxy‑Wasm filter + Makefile + Envoy compose + Istio YAML).

What it records
- Histograms: `wasm_http_inbound_duration_ms`, `wasm_http_outbound_duration_ms`
- Counters: per direction and class: `wasm_http_inbound_responses_total_{2xx,3xx,4xx,5xx}` and outbound equivalents
- Optional JSON logs with method, path (truncated), authority for ad‑hoc analysis (avoid labels)

Run locally
1) Build Wasm: `make -C labs/http-timing-rust build`
2) Start Envoy: `docker compose -f labs/http-timing-rust/docker-compose.yaml up --build`
3) Hit proxy: `curl -sSI localhost:10000/get` and open Envoy admin `localhost:9901/metrics` to see `wasm_http_*` metrics.

Deploy to Istio
1) Push OCI: `REG=ghcr.io ORG=<you> IMG=http-timing TAG=v0.1.0 make -C labs/http-timing-rust push`
2) Apply plugin: edit `labs/http-timing-rust/k8s/wasmplugin.yaml` with your `url` and target namespace, then `kubectl apply -f`.
3) Validate with `kubectl port-forward` to an injected pod’s Envoy admin and inspect metrics, or let Prometheus/OTel scrape sidecars.

Notes
- This lab demonstrates a minimal, production‑safe pattern: metrics defined once in root context; minimal per‑request work; no labels with raw URLs.
- For a full service graph, rely on distributed tracing (Istio tracing provider or Otel) and use metrics for SLI/SLOs.
- See also: `istiotelemetry/istio-telemetry.md` for how to achieve similar outcomes without writing Wasm by using the Istio Telemetry API.
