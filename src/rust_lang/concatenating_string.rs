use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn concatenating_with_format(name: &str) -> String {
    format!("Hello {}!", name)
}

fn concatenating_with_push(name: &str) -> String {
    let mut result = "Hello ".to_owned();
    result.push_str(name);
    result.push('!');
    result
}

fn criterion_concatenating_string(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fibonacci");
    for i in ["yumcoder", "data", "programming"].iter() {
        group.bench_with_input(
            BenchmarkId::new("concatenating_with_format", i),
            i,
            |b, i| b.iter(|| concatenating_with_format(*i)),
        );
        group.bench_with_input(BenchmarkId::new("concatenating_with_push", i), i, |b, i| {
            b.iter(|| concatenating_with_push(*i))
        });
    }
    group.finish();
}

criterion_group!(concatenating_string, criterion_concatenating_string);
criterion_main!(concatenating_string);
