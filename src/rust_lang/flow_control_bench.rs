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

#[allow(dead_code)]
fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("test_let_match");
    group.bench_function("test_let_match", |b| b.iter(|| test_let_match()));
    group.bench_function("test_let_if", |b| b.iter(|| test_let_if()));
    group.finish();
}

criterion_group!(benches_control_flow, criterion_benchmark);
criterion_main!(benches_control_flow);
