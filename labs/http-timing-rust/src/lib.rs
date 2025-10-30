use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use proxy_wasm::hostcalls;

// Metric IDs we define once in RootContext and reuse in HttpContext.
#[derive(Clone, Copy, Default)]
struct Metrics {
    hist_in_ms: u32,
    hist_out_ms: u32,
    in_2xx: u32,
    in_3xx: u32,
    in_4xx: u32,
    in_5xx: u32,
    out_2xx: u32,
    out_3xx: u32,
    out_4xx: u32,
    out_5xx: u32,
}

struct Root {
    m: Metrics,
}

impl Context for Root {}
impl RootContext for Root {
    fn on_vm_start(&mut self, _conf_size: usize) -> bool {
        // Define metrics once per VM
        let mut m = Metrics::default();
        m.hist_in_ms = self.define_metric(MetricType::Histogram, "wasm_http_inbound_duration_ms").unwrap_or(0);
        m.hist_out_ms = self.define_metric(MetricType::Histogram, "wasm_http_outbound_duration_ms").unwrap_or(0);

        m.in_2xx = self.define_metric(MetricType::Counter, "wasm_http_inbound_responses_total_2xx").unwrap_or(0);
        m.in_3xx = self.define_metric(MetricType::Counter, "wasm_http_inbound_responses_total_3xx").unwrap_or(0);
        m.in_4xx = self.define_metric(MetricType::Counter, "wasm_http_inbound_responses_total_4xx").unwrap_or(0);
        m.in_5xx = self.define_metric(MetricType::Counter, "wasm_http_inbound_responses_total_5xx").unwrap_or(0);

        m.out_2xx = self.define_metric(MetricType::Counter, "wasm_http_outbound_responses_total_2xx").unwrap_or(0);
        m.out_3xx = self.define_metric(MetricType::Counter, "wasm_http_outbound_responses_total_3xx").unwrap_or(0);
        m.out_4xx = self.define_metric(MetricType::Counter, "wasm_http_outbound_responses_total_4xx").unwrap_or(0);
        m.out_5xx = self.define_metric(MetricType::Counter, "wasm_http_outbound_responses_total_5xx").unwrap_or(0);

        self.m = m;
        true
    }

    fn create_http_context(&mut self, _context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(HttpCtx { start_ms: 0, inbound: false, m: self.m }))
    }

    fn get_type(&self) -> Option<ContextType> { Some(ContextType::HttpContext) }
}

struct HttpCtx {
    start_ms: u64,
    inbound: bool,
    m: Metrics,
}

impl Context for HttpCtx {}

impl HttpContext for HttpCtx {
    fn on_http_request_headers(&mut self, _n: usize, _eos: bool) -> Action {
        self.start_ms = now_ms();
        self.inbound = detect_inbound(self);
        Action::Continue
    }

    fn on_log(&mut self) {
        let dur = now_ms().saturating_sub(self.start_ms);
        let status = self
            .get_http_response_header(":status")
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(0);

        // Record histogram
        if self.inbound {
            let _ = self.record_metric(self.m.hist_in_ms, dur as i64);
        } else {
            let _ = self.record_metric(self.m.hist_out_ms, dur as i64);
        }

        // Increment response-class counters
        let class = (status / 100) * 100;
        match (self.inbound, class) {
            (true, 200) => { let _ = self.increment_metric(self.m.in_2xx, 1); }
            (true, 300) => { let _ = self.increment_metric(self.m.in_3xx, 1); }
            (true, 400) => { let _ = self.increment_metric(self.m.in_4xx, 1); }
            (true, 500) => { let _ = self.increment_metric(self.m.in_5xx, 1); }
            (false, 200) => { let _ = self.increment_metric(self.m.out_2xx, 1); }
            (false, 300) => { let _ = self.increment_metric(self.m.out_3xx, 1); }
            (false, 400) => { let _ = self.increment_metric(self.m.out_4xx, 1); }
            (false, 500) => { let _ = self.increment_metric(self.m.out_5xx, 1); }
            _ => {}
        }

        // Optional structured log (truncated path to avoid cardinality)
        let method = self.get_http_request_header(":method").unwrap_or_default();
        let authority = self.get_http_request_header(":authority").unwrap_or_default();
        let path = truncate(self.get_http_request_header(":path").unwrap_or_default(), 200);
        let dir = if self.inbound { "inbound" } else { "outbound" };
        let _ = self.log(LogLevel::Info, &format!(
            "{{\"dir\":\"{}\",\"dur_ms\":{},\"method\":\"{}\",\"status\":{},\"authority\":\"{}\",\"path\":\"{}\"}}",
            dir, dur, method, status, escape(&authority), escape(&path)
        ));
    }
}

fn escape(s: &str) -> String { s.replace('"', "\\\"") }
fn truncate(s: String, max: usize) -> String {
    if s.len() > max { s[..max].to_string() } else { s }
}

fn now_ms() -> u64 {
    // Host provides a clock; fallback to 0 if unavailable
    hostcalls::get_current_time_nanoseconds().unwrap_or(0) / 1_000_000
}

fn detect_inbound<C: Context>(ctx: &C) -> bool {
    // Try common Envoy properties for direction; default to outbound
    let candidates: [&[&str]; 2] = [&["listener", "direction"], &["listener_direction"]];
    for key in candidates.iter() {
        if let Some(v) = ctx.get_property(key) {
            if let Ok(s) = String::from_utf8(v) {
                if s.trim() == "inbound" { return true; }
                if s.trim() == "outbound" { return false; }
            }
        }
    }
    false
}

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Info);
    proxy_wasm::set_root_context(|_| Box::new(Root { m: Metrics::default() }));
}}

