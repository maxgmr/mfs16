use criterion::{criterion_group, criterion_main, Criterion};

use mfs16core::{Computer, Instruction::*, MemWritable, Reg16::*, Reg32::*};

// pub fn nop_benchmark(c: &mut Criterion) {
//     let mut test_c = Computer::new(false);
//     c.bench_function("nop", |b| b.iter(|| test_c.cycle()));
// }

pub fn ld_benchmark(c: &mut Criterion) {
    let mut test_c = Computer::new(false);

    // Loop loading A into HL
    test_c.mmu.rom.set_writable(true);
    LdBraRb(HL, A).mem_write(&mut test_c.mmu.rom, 0x0000_0000);
    JpImm32.mem_write(&mut test_c.mmu.rom, 0x0000_0002);

    c.bench_function("ld [HL],A", |b| b.iter(|| test_c.cycle()));
}

criterion_group!(benches, ld_benchmark);
criterion_main!(benches);
