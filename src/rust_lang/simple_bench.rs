// rust-analyzer doest not recognized external crate
// extern crate test;

// #[allow(unused)]
// pub fn add_two(a: i32) -> i32 {
//     a + 2
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use test::Bencher;

//     #[test]
//     fn it_works() {
//         assert_eq!(4, add_two(2));
//     }

//     #[bench]
//     fn bench_add_two(b: &mut Bencher) {
//         b.iter(|| add_two(2));
//     }
// }

use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[allow(dead_code)]
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

#[allow(dead_code)]
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
