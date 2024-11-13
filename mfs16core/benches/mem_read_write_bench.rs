use criterion::{criterion_group, criterion_main, Criterion};

use mfs16core::Computer;

pub fn read_next_word_benchmark(c: &mut Criterion) {
    let mut test_c = Computer::new(false);

    // Loop reading memory
    c.bench_function("read next word", |b| {
        b.iter(|| test_c.cpu.read_next_word(&test_c.mmu))
    });
}

criterion_group!(benches, read_next_word_benchmark);
criterion_main!(benches);
