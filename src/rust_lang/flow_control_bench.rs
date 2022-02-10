use criterion::{criterion_group, criterion_main, Criterion};

fn test_let_if() {
    fn is_even(n: i32) -> bool {
        n % 2 == 0
    }
    let n = 123456;
    let description = if is_even(n) { "even" } else { "odd" };
    assert_eq!("even", description);
}

fn test_let_match() {
    fn is_even(n: i32) -> bool {
        n % 2 == 0
    }

    let n = 123456;
    let description = match is_even(n) {
        true => "even",
        false => "odd",
    };
    assert_eq!("even", description);
}

// for item in collection -->  for item in IntoIterator::into_iter(collection)  --> Ownership
fn test_for_ownership() {
    let collection = vec![0; 1_000_000_000];
    let mut sum = 0;
    for i in collection.into_iter() {
        sum += i;
    }
}

// for item in &collection --> for item in collection.iter() --> Read-only
fn test_for_readonly() {
    let collection = vec![0; 1_000_000_000];
    let mut sum = 0;
    for i in collection.iter() {
        sum += i;
    }
}

// for item in &mut collection --> for item in collection.iter_mut() --> Read-write
fn test_for_read_write() {
    let mut collection = vec![0; 1_000_000_000];
    let mut sum = 0;
    for i in collection.iter_mut() {
        sum += *i;
    }
}

#[allow(dead_code)]
fn criterion_benchmark_for(c: &mut Criterion) {
    let mut group = c.benchmark_group("test_for_read_only_benchmark");
    group.bench_function("test_for_ownership", |b| b.iter(|| test_for_ownership()));
    group.bench_function("test_for_readonly", |b| b.iter(|| test_for_readonly()));
    group.bench_function("test_for_read_write", |b| b.iter(|| test_for_read_write()));
    group.finish();
}

#[allow(dead_code)]
fn criterion_benchmark_let(c: &mut Criterion) {
    let mut group = c.benchmark_group("test_let_match_benchmark");
    group.bench_function("test_let_match", |b| b.iter(|| test_let_match()));
    group.bench_function("test_let_if", |b| b.iter(|| test_let_if()));
    group.finish();
}

criterion_group!(
    benches_control_flow,
    criterion_benchmark_let,
    criterion_benchmark_for
);
criterion_main!(benches_control_flow);
