use criterion::{criterion_group, criterion_main, Criterion};

use mfs16core::{
    Computer,
    Instruction::{self, *},
    MemWritable,
    Reg16::*,
    Reg32::*,
};

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

pub fn read_opcode_benchmark(c: &mut Criterion) {
    const OPCODE_LIST: [u16; 10] = [
        0x01A0, 0x0900, 0x1802, 0x3FA3, 0x4D30, 0x54BC, 0x8001, 0x8203, 0xFFFE, 0xFFFF,
    ];
    let mut i: usize = 0;
    c.bench_function("ld [HL],A", |b| {
        b.iter(|| {
            i = (i + 1) % 10;
            Instruction::from_opcode(OPCODE_LIST[i]);
        })
    });
}

criterion_group!(benches, ld_benchmark, read_opcode_benchmark);
criterion_main!(benches);
