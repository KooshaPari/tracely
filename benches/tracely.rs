#![feature(test)]
extern crate test;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tracely::{logging::LogContext, tracing::TraceContext, tracing::TracingConfig};

fn bench_trace_context_new(c: &mut Criterion) {
    c.bench_function("trace_context_new", |b| {
        b.iter(|| TraceContext::new());
    });
}

fn bench_log_context_new_none(c: &mut Criterion) {
    c.bench_function("log_context_new_none", |b| {
        b.iter(|| LogContext::new(black_box(None)));
    });
}

fn bench_log_context_new_with_id(c: &mut Criterion) {
    c.bench_function("log_context_new_with_id", |b| {
        b.iter(|| LogContext::new(black_box(Some("test-correlation-id".to_string()))));
    });
}

fn bench_tracing_config_new(c: &mut Criterion) {
    c.bench_function("tracing_config_new", |b| {
        b.iter(|| TracingConfig::new(black_box("info")));
    });
}

fn bench_tracing_config_builder(c: &mut Criterion) {
    c.bench_function("tracing_config_builder", |b| {
        b.iter(|| {
            TracingConfig::new(black_box("debug"))
                .with_span_events(black_box(true))
                .with_thread_ids(black_box(true))
                .with_thread_names(black_box(true))
                .with_target(black_box(false))
        });
    });
}

fn bench_level_as_str(c: &mut Criterion) {
    c.bench_function("level_as_str", |b| {
        b.iter(|| tracely::tracing::level_as_str(black_box(tracing::Level::INFO)));
    });
}

criterion_group!(
    benches,
    bench_trace_context_new,
    bench_log_context_new_none,
    bench_log_context_new_with_id,
    bench_tracing_config_new,
    bench_tracing_config_builder,
    bench_level_as_str
);
criterion_main!(benches);
