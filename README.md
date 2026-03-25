# tracely

Unified observability library for Rust: distributed tracing, metrics, and structured logging in one package.

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
