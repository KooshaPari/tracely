<!-- Base: platforms/thegent/governance/AGENTS.base.md -->
<!-- Last synced: 2026-04-02 -->

# AGENTS.md — tracely

Extends thegent governance base. See `platforms/thegent/governance/AGENTS.base.md` for canonical definitions.

## Project Identity

- **Name**: tracely
- **Description**: Unified observability library wrapping tracing, metrics, and opentelemetry
- **Location**: `/Users/kooshapari/CodeProjects/Phenotype/repos/tracely`
- **Language Stack**: Rust (edition 2021)
- **Published**: crates.io

## AgilePlus Integration

All work MUST be tracked in AgilePlus:
- Reference: `/Users/kooshapari/CodeProjects/Phenotype/repos/.agileplus`
- CLI: `agileplus <command>`
- No code without corresponding AgilePlus spec

---

## Repository Mental Model

### Project Structure

```
src/
  tracing/    # Span lifecycle, context propagation
  metrics/    # Counter, gauge, histogram macros
  logging/    # Structured JSON log macros
  export/     # OTLP, Prometheus, Jaeger, Zipkin exporters
  config.rs   # TracingConfig builder
  lib.rs      # Public API surface + init()
```

### Stack

| Layer | Crate | Version |
|-------|-------|---------|
| Span/event collection | `tracing` | 0.1 |
| Metrics primitives | `metrics` | 0.21 |
| OTLP export | `opentelemetry` | 0.21 |
| Prometheus export | `prometheus` | 0.13 |

### Style Constraints

- **Line length**: 100 characters
- **Formatter**: `cargo fmt` (mandatory)
- **Linter**: `cargo clippy` with `-- -D warnings` (zero warnings)
- **File size target**: ≤350 lines per source file

---

## Key Commands

```bash
cargo build            # Build library
cargo test             # Run all tests
cargo clippy -- -D warnings   # Lint (zero warnings enforced)
cargo doc --open       # Build and view rustdoc
cargo bench            # Run criterion benchmarks
```

---

## Quality Gates

- `cargo clippy -- -D warnings` — 0 warnings required
- `cargo test` — all pass required
- `cargo doc` — 0 missing doc warnings on public items
