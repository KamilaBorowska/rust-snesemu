extern crate snesemu_cpu;

use snesemu_cpu::cpu::CPU;
use snesemu_cpu::mapper::LoROM;

fn create_rom(input: &[u8]) -> [u8; 0x10000] {
    // Needed because emulator reads reset vector location from end
    // of first bank
    let mut output = [0; 0x10000];
    output[..input.len()].copy_from_slice(input);
    // 0x8000 is beginning of LoROM code
    output[0x7FFC..0x7FFE].copy_from_slice(&[0x00, 0x80]);
    output
}

#[test]
fn lda_immediate() {
    let bytes = create_rom(&[0xA9, 0x20]);
    let mut cpu = CPU::new(LoROM::new(&bytes));
    cpu.step();
    assert_eq!(0x20, cpu.registers.a);
}