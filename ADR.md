# ADR — tracely

## ADR-001 — Rust as Implementation Language

**Status:** Accepted

**Context:** Observability libraries must have minimal overhead and be embeddable in performance-critical applications.

**Decision:** Implement tracely in Rust.

**Rationale:** Rust guarantees zero-cost abstractions and memory safety without a GC. Native crate ecosystem (`tracing`, `metrics`, `opentelemetry`) provides solid foundations to wrap. Meets the sub-50ns zero-allocation log path requirement.

---

## ADR-002 — Wrap Existing Crates Rather Than Hand-Roll

**Status:** Accepted

**Context:** `tracing`, `metrics`, and `opentelemetry` crates are well-maintained, widely adopted, and cover the core requirements.

**Decision:** tracely wraps these crates and provides a unified, simplified API over them. It does not reimplement tracing spans, metrics collection, or serialization.

**Rationale:** Wrapping maintained OSS reduces maintenance burden and leverages existing ecosystem compatibility. tracely's value is the unified init API and ergonomic macros, not reimplementing protocol stacks.

**Wrapped Libraries:**
- `tracing` 0.1 — span and event collection
- `metrics` 0.21 — metrics collection
- `opentelemetry` 0.21 — OTLP export
- `prometheus` 0.13 — Prometheus exposition format
- `serde_json` 1.0 — structured log serialization

---

## ADR-003 — OpenTelemetry as Tracing Wire Format

**Status:** Accepted

**Context:** Traces need to be exportable to multiple backends (Jaeger, Zipkin, Tempo, etc.).

**Decision:** Use OpenTelemetry Protocol (OTLP) as the primary trace export format.

**Rationale:** OTLP is the vendor-neutral standard. Any OTLP-compatible backend works without code changes. Jaeger and Zipkin are supported via their OTLP ingest endpoints.

**Alternatives Considered:**
- Jaeger-native format: ties library to a single backend
- Custom binary protocol: maintenance burden with no benefit

---

## ADR-004 — Builder Pattern for Configuration

**Status:** Accepted

**Context:** Users need to configure exporters, log level, sampling rate, and other parameters.

**Decision:** Use a builder pattern (`TracingConfig::builder()`) for all configuration.

**Rationale:** Builder pattern is idiomatic Rust for complex configuration. Compile-time checks catch missing required fields. No runtime panics from misconfiguration.

**Alternatives Considered:**
- Config file (TOML/YAML): adds file I/O dependency; less composable in library context
- Environment variables only: insufficient for programmatic configuration

---

## ADR-005 — MIT License

**Status:** Accepted

**Context:** Library should be usable in both open-source and commercial applications.

**Decision:** MIT license.

**Rationale:** Permissive license maximizes adoption. Compatible with all major Rust ecosystem licenses.
