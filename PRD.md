# PRD — tracely

## Overview

tracely is a unified observability library for Rust applications. It combines distributed tracing (OpenTelemetry-compatible spans), metrics (counters, gauges, histograms), and structured logging into a single cohesive API — eliminating the need to configure three separate observability stacks.

## Epics

### E1 — Distributed Tracing

| Story | Description | Acceptance Criteria |
|-------|-------------|---------------------|
| E1.1 | Application code creates named spans | `tracer::start("name")` returns a span handle |
| E1.2 | Spans carry context across async boundaries | Parent-child span relationships are correct in exported traces |
| E1.3 | Traces export via OTLP | Spans appear in an OTLP-compatible backend (e.g., Jaeger, Tempo) |
| E1.4 | Traces export via Jaeger/Zipkin | Spans visible in Jaeger/Zipkin UI |

### E2 — Metrics

| Story | Description | Acceptance Criteria |
|-------|-------------|---------------------|
| E2.1 | Application code increments counters | `metrics::counter!("name").inc()` updates counter |
| E2.2 | Application code records histograms | `metrics::histogram!("name").observe(val)` records observation |
| E2.3 | Metrics export via Prometheus scrape endpoint | `/metrics` endpoint returns valid Prometheus text format |

### E3 — Structured Logging

| Story | Description | Acceptance Criteria |
|-------|-------------|---------------------|
| E3.1 | Application code emits structured JSON logs | `log::info!("msg", {key: val})` emits valid JSON to stdout |
| E3.2 | Log output is zero-allocation in hot paths | Benchmark shows <50ns overhead per log call (no alloc path) |
| E3.3 | Log level filtering at runtime | Log verbosity configurable via env var |

### E4 — Unified Init and Configuration

| Story | Description | Acceptance Criteria |
|-------|-------------|---------------------|
| E4.1 | Single init call configures all three pillars | `tracely::init(config)` sets up tracing + metrics + logging |
| E4.2 | Configuration via builder pattern | `TracingConfig::builder()` compiles without errors |

## Non-Goals

- Non-Rust language bindings in v1
- SaaS exporter integrations (Datadog, New Relic) beyond OTLP
- Agent-based sampling decisions
