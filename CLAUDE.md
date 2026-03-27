# tracely — CLAUDE.md

## Project Summary

tracely is a unified observability library for Rust. It wraps `tracing`, `metrics`, and `opentelemetry` crates to provide a single, ergonomic API for distributed tracing, metrics, and structured logging.

## Stack

| Layer | Crate | Version |
|-------|-------|---------|
| Span/event collection | `tracing` | 0.1 |
| Metrics primitives | `metrics` | 0.21 |
| OTLP export | `opentelemetry` | 0.21 |
| Prometheus export | `prometheus` | 0.13 |
| Serialization | `serde_json` | 1.0 |
| Benchmarking | `criterion` | 0.5 |

## Key Commands

```bash
cargo build            # Build library
cargo test             # Run all tests
cargo clippy -- -D warnings   # Lint (zero warnings enforced)
cargo doc --open       # Build and view rustdoc
cargo bench            # Run criterion benchmarks
```

## Structure

```
src/
  tracing/    # Span lifecycle, context propagation
  metrics/    # Counter, gauge, histogram macros
  logging/    # Structured JSON log macros
  export/     # OTLP, Prometheus, Jaeger, Zipkin exporters
  config.rs   # TracingConfig builder
  lib.rs      # Public API surface + init()
```

## Development Rules

- tracely wraps existing crates — do NOT reimplement what tracing/metrics/opentelemetry already do
- All public API items MUST have rustdoc documentation
- The zero-allocation log path MUST not introduce heap allocations (verify with criterion benchmark)
- `cargo clippy -- -D warnings` MUST pass with zero warnings — no `#[allow(...)]` suppressions without justification comment
- New exporters: implement the `Exporter` trait in `src/export/`
- New macros: add to the appropriate module (`tracing/`, `metrics/`, `logging/`)

## Adding New Features

1. New exporter: implement `Exporter` trait in `src/export/<name>.rs`, export from `src/export/mod.rs`
2. New metric type: add macro in `src/metrics/mod.rs`
3. New config option: add field to `TracingConfig` builder in `src/config.rs`
4. New test: add in `#[cfg(test)]` block in the relevant module, reference FR ID

## Quality Gates

- `cargo clippy -- -D warnings` — 0 warnings required
- `cargo test` — all pass required
- `cargo doc` — 0 missing doc warnings on public items
