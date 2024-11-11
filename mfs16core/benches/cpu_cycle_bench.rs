use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mfs16core::Computer;

pub fn nop_benchmark(c: &mut Criterion) {
    let mut test_c = Computer::new(false);
    c.bench_function("nop", |b| b.iter(|| test_c.cycle()));
}

criterion_group!(benches, nop_benchmark);
criterion_main!(benches);
