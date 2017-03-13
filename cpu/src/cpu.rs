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
use instructions;
use mapper::Mapper;

const RAM_SIZE: usize = 0x2_0000;

pub struct Registers {
    pub a: u16,
    pub x: u16,
    pub y: u16,
    pub dp: u16,
    pub sp: u16,
    pub db: u8,
    pub pc: u16,
    pub pb: u8,
    pub flags: Flags,
}

bitflags! {
    pub flags Flags: u8 {
        const FLAG_CARRY    = 1 << 0,
        const FLAG_ZERO     = 1 << 1,
        const FLAG_NO_IRQ   = 1 << 2,
        const FLAG_DECIMAL  = 1 << 3,
        const FLAG_XY16     = 1 << 4,
        const FLAG_A16      = 1 << 5,
        const FLAG_OVERFLOW = 1 << 6,
        const FLAG_NEGATIVE = 1 << 7,
    }
}

pub struct CPU<M: Mapper> {
    pub registers: Registers,
    pub ram: [u8; RAM_SIZE],
    pub mapper: M,
}

impl<M: Mapper> CPU<M> {
    pub fn new(mapper: M) -> Self {
        let mut cpu = CPU {
            ram: [0x55; RAM_SIZE],
            registers: Registers {
                a: 0,
                x: 0,
                y: 0,
                dp: 0,
                sp: 0xFF,
                db: 0,
                pc: 0,
                pb: 0,
                flags: FLAG_NO_IRQ,
            },
            mapper: mapper,
        };
        // Reset vector
        cpu.registers.pc = cpu.read(0x00, 0xFFFC);
        cpu
    }

    pub fn read<T: BitWidth>(&self, bank: u8, address: u16) -> T {
        T::read(self, bank, address)
    }

    pub fn write<T: BitWidth>(&mut self, bank: u8, address: u16, value: T) {
        T::write(self, bank, address, value);
    }

    pub fn step(&mut self) {
        instructions::run_instruction(self);
    }
}
