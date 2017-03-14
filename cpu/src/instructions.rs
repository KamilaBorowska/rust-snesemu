// snesemu - SNES emulator written in Rust
// Copyright (C) 2017 Konrad Borowski
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use bitwidth::BitWidth;
use cpu::{CPU, Flags, FLAG_NO_IRQ, FLAG_A16};
use mapper::Mapper;

fn fetch<M: Mapper>(cpu: &mut CPU<M>) -> u8 {
    let byte = cpu.read(cpu.registers.pb, cpu.registers.pc);
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
    byte
}

fn set_flag<M: Mapper>(cpu: &mut CPU<M>, flags: Flags) {
    cpu.registers.flags |= flags;
}

// Addressing types
//
// This is actually fairly crazy. Many addressing modes work differently
// depending on 16-bit mode. To avoid writing the same code twice, opcode
// implementations are generic over BitWidth which implements generic
// functions to handle any bit width mode.
//
// Rust doesn't currently support higher-kinded types, and function literal
// can be only resolved to a single type. To resolve this issue, a function
// is passed twice, so Rust has to resolve types twice.

fn absolute<M, F, G>(cpu: &mut CPU<M>, sixteen_bits: bool, f: F, g: G)
    where M: Mapper,
          F: FnOnce(&mut CPU<M>, u8),
          G: FnOnce(&mut CPU<M>, u16)
{
    fn address_reader<M, T, F>(cpu: &mut CPU<M>, address: u16, f: F)
        where M: Mapper,
              T: BitWidth,
              F: FnOnce(&mut CPU<M>, T)
    {
        let value = cpu.read(cpu.registers.db, address);
        f(cpu, value);
    }
    absolute_address(cpu,
                     sixteen_bits,
                     |cpu, address| address_reader(cpu, address, f),
                     |cpu, address| address_reader(cpu, address, g));
}

fn absolute_address<M, F, G>(cpu: &mut CPU<M>, sixteen_bits: bool, f: F, g: G)
    where M: Mapper,
          F: FnOnce(&mut CPU<M>, u16),
          G: FnOnce(&mut CPU<M>, u16)
{
    let a = fetch(cpu);
    let b = fetch(cpu);
    let address = a as u16 | ((b as u16) << 8);

    if sixteen_bits {
        g(cpu, address);
    } else {
        f(cpu, address);
    }
}

fn immediate<M, F, G>(cpu: &mut CPU<M>, sixteen_bits: bool, f: F, g: G)
    where M: Mapper,
          F: FnOnce(&mut CPU<M>, u8),
          G: FnOnce(&mut CPU<M>, u16)
{
    let a = fetch(cpu);

    if sixteen_bits {
        let b = (fetch(cpu) as u16) << 8;
        g(cpu, a as u16 | b);
    } else {
        f(cpu, a);
    }
}

fn pc_relative<M, F>(cpu: &mut CPU<M>, when: F)
    where M: Mapper,
          F: FnOnce(&mut CPU<M>) -> bool
{
    let value = fetch(cpu);
    if when(cpu) {
        cpu.registers.pc = cpu.registers.pc.wrapping_add(value as i8 as u16);
    }
}

fn a16<M, F, G, H>(cpu: &mut CPU<M>, f: F, g: G, h: H)
    where M: Mapper,
          F: FnOnce(&mut CPU<M>, bool, G, H)
{
    let sixteen_bits = cpu.registers.flags.contains(FLAG_A16);
    f(cpu, sixteen_bits, g, h);
}

// Assembly opcodes

fn lda<M: Mapper, T: BitWidth>(cpu: &mut CPU<M>, value: T) {
    T::set(&mut cpu.registers.a, value);
}

fn stz<M: Mapper, T: BitWidth + Default>(cpu: &mut CPU<M>, address: u16) {
    let db = cpu.registers.db;
    cpu.write(db, address, T::default());
}

pub fn run_instruction<M: Mapper>(cpu: &mut CPU<M>) {
    match fetch(cpu) {
        // BRA
        0x80 => pc_relative(cpu, |_| true),

        // LDA (Load Accumulator from Memory)
        // immediate
        0xA9 => a16(cpu, immediate, lda, lda),
        // absolute
        0xAD => a16(cpu, absolute, lda, lda),

        // SEI (Set Interrupt Disable Flag)
        // implied
        0x78 => set_flag(cpu, FLAG_NO_IRQ),

        // STZ (Store Zero to Memory)
        // absolute
        0x9C => a16(cpu, absolute_address, stz::<M, u8>, stz::<M, u16>),

        code => {
            println!("Tried to run unimplemented instruction {:x}.", code);
            unimplemented!();
        }
    }
}
