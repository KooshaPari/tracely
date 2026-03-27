# Functional Requirements — tracely

## FR-TRACE — Distributed Tracing

| ID | Requirement | Traces To |
|----|-------------|-----------|
| FR-TRACE-001 | The library SHALL provide a `tracer::start(name)` API returning a span handle | E1.1 |
| FR-TRACE-002 | Spans SHALL support adding key-value attributes | E1.1 |
| FR-TRACE-003 | Spans SHALL propagate context across async boundaries using the `tracing` crate | E1.2 |
| FR-TRACE-004 | Parent-child span relationships SHALL be preserved in exported traces | E1.2 |
| FR-TRACE-005 | The library SHALL export traces via OTLP (gRPC or HTTP) | E1.3 |
| FR-TRACE-006 | The library SHALL export traces in Jaeger-compatible format | E1.4 |
| FR-TRACE-007 | The library SHALL export traces in Zipkin-compatible format | E1.4 |

## FR-METRICS — Metrics

| ID | Requirement | Traces To |
|----|-------------|-----------|
| FR-METRICS-001 | The library SHALL provide a `metrics::counter\!(name)` macro | E2.1 |
| FR-METRICS-002 | The library SHALL provide a `metrics::histogram\!(name)` macro | E2.2 |
| FR-METRICS-003 | The library SHALL provide a `metrics::gauge\!(name)` macro | E2.2 |
| FR-METRICS-004 | The library SHALL expose a Prometheus-compatible `/metrics` scrape endpoint | E2.3 |
| FR-METRICS-005 | Prometheus output SHALL be valid text format per the Prometheus exposition spec | E2.3 |

## FR-LOG — Structured Logging

| ID | Requirement | Traces To |
|----|-------------|-----------|
| FR-LOG-001 | The library SHALL provide `log::info\!`, `log::warn\!`, `log::error\!`, `log::debug\!` macros | E3.1 |
| FR-LOG-002 | Log output SHALL be valid JSON with at minimum: level, message, timestamp, fields | E3.1 |
| FR-LOG-003 | The hot path log emission SHALL complete in under 50ns with zero heap allocation | E3.2 |
| FR-LOG-004 | Log verbosity SHALL be controllable via the `TRACELY_LOG_LEVEL` environment variable | E3.3 |

## FR-INIT — Initialization

| ID | Requirement | Traces To |
|----|-------------|-----------|
| FR-INIT-001 | The library SHALL provide a single `tracely::init(config)` function that configures all pillars | E4.1 |
| FR-INIT-002 | Configuration SHALL be constructible via a builder pattern (`TracingConfig::builder()`) | E4.2 |
| FR-INIT-003 | Missing required configuration SHALL cause `init` to return a descriptive `Err` | E4.1 |

## FR-QUALITY — Quality

| ID | Requirement | Traces To |
|----|-------------|-----------|
| FR-QUALITY-001 | The library SHALL compile with `cargo clippy -- -D warnings` with zero warnings | - |
| FR-QUALITY-002 | All public API SHALL have rustdoc documentation | - |
| FR-QUALITY-003 | `cargo test` SHALL pass with zero failures | - |
