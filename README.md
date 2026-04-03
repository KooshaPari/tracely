# Tracely — Observability Primitives

Unified observability library for Rust: distributed tracing, metrics, and structured logging in one package.

## Crates

| Crate | Path | Description |
|-------|------|-------------|
| tracely-core | `crates/tracely-core` | Core observability primitives |
| tracely-sentinel | `crates/tracely-sentinel` | Monitoring and alerting |

## Installation

```toml
[dependencies]
tracely-core = { workspace = true }
tracely-sentinel = { workspace = true }
```

## Workspace Note

This workspace combines previously separate observability crates:
- `phenoSentinel` → merged as `tracely-sentinel`

## Features

- **Distributed Tracing**: OpenTelemetry-compatible spans
- **Metrics**: Counters, gauges, histograms
- **Structured Logging**: Zero-allocation JSON logs
- **Exporters**: OTLP, Prometheus, Jaeger, Zipkin

## Installation

```toml
[dependencies]
tracely = { git = "https://github.com/KooshaPari/tracely" }
```

## Usage

```rust
use tracely::{tracer, metrics, log};

// Trace
let span = tracer::start("process_request");
defer { span.end(); }

// Metrics
metrics::counter!("requests_total").inc();
metrics::histogram!("request_duration").observe(duration);

// Log
log::info!("Request processed", { "duration_ms": 42 });
```

## Architecture

```
src/
├── tracing/      # Distributed tracing
├── metrics/      # Metrics collection
├── logging/      # Structured logging
└── exporters/   # OTLP, Prometheus, etc.
```

## License

MIT
