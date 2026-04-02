# tracely Specification

## Architecture
```
┌─────────────────────────────────────────────────────┐
│            tracely (Rust)                           │
├─────────────────────────────────────────────────────┤
│  ┌───────────────────────────────────────────────┐ │
│  │         Distributed tracing                    │ │
│  │   ┌─────────┐   ┌─────────┐   ┌────────────┐  │ │
│  │   │ Trace  │   │ Span   │   │ Exporter │  │ │
│  │   └─────────┘   └─────────┘   └────────────┘  │ │
│  └───────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────┘
```

## Components

| Component | Responsibility | Public API |
|-----------|----------------|-----------|
| Trace | Trace creation | `new()`, `in_session()` |
| Span | Span management | `start()`, `end()` |
| Exporter | Export spans | `export()`, `batch_export()` |

## Data Models

```rust
struct Span {
    trace_id: Uuid,
    span_id: Uuid,
    name: String,
    start_time: SystemTime,
    end_time: Option<SystemTime>,
}
```

## Performance Targets

| Metric | Target |
|--------|--------|
| Span create | <1μs |
| Export | <10ms |