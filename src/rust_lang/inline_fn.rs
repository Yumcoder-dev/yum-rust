use core::time;
use std::thread;

// The best candidates for inlining are
// (a) functions that are very small, or
// (b) functions that have a single call site
//
// The compiler will often inline these functions itself even without an inline attribute.
use criterion::{criterion_group, criterion_main, Criterion};

#[allow(dead_code)]
#[inline(always)]
fn inline_always() {
    thread::sleep(time::Duration::from_millis(10))
}

#[allow(dead_code)]
#[inline(never)]
fn inline_never() {
    thread::sleep(time::Duration::from_millis(10))
}

#[allow(unused)]
fn criterion_inline(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fibonacci");
    group.bench_function("inline_always", |b| b.iter(|| inline_always()));
    group.bench_function("inline_never", |b| b.iter(|| inline_never()));
    group.finish();
}

criterion_group!(bench_inline, criterion_inline,);
criterion_main!(bench_inline);
