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

use cpu::CPU;
use mapper::Mapper;

pub trait BitWidth {
    fn read<M: Mapper>(cpu: &CPU<M>, bank: u8, address: u16) -> Self;
    fn write<M: Mapper>(cpu: &mut CPU<M>, bank: u8, address: u16, value: Self);
    fn get(value: u16) -> Self;
    fn set(value: &mut u16, to: Self);
}

impl BitWidth for u8 {
    fn read<M: Mapper>(cpu: &CPU<M>, bank: u8, address: u16) -> u8 {
        let address = address as usize;
        match (bank, address) {
            (0x00...0x3F, 0x0000...0x1FFF) |
            (0x7E, _) => cpu.ram[address],
            (0x7F, _) => cpu.ram[0x10000 + address],
            _ => cpu.mapper.read(bank, address as u16),
        }
    }

    fn write<M: Mapper>(cpu: &mut CPU<M>, bank: u8, address: u16, value: u8) {
        let address = address as usize;
        match (bank, address) {
            (0x00...0x3F, 0x0000...0x1FFF) |
            (0x7E, _) => cpu.ram[address] = value,
            (0x7F, _) => cpu.ram[0x10000 + address] = value,
            _ => cpu.mapper.write(bank, address as u16, value),
        };
    }

    fn get(value: u16) -> u8 {
        value as u8
    }

    fn set(value: &mut u16, to: u8) {
        *value = (*value & 0xFF00) | to as u16;
    }
}

impl BitWidth for u16 {
    fn read<M: Mapper>(cpu: &CPU<M>, bank: u8, address: u16) -> u16 {
        let a = u8::read(cpu, bank, address) as u16;
        let b = (u8::read(cpu, bank, address.wrapping_add(1)) as u16) << 8;
        a | b
    }

    fn write<M: Mapper>(cpu: &mut CPU<M>, bank: u8, address: u16, value: u16) {
        u8::write(cpu, bank, address, value as u8);
        u8::write(cpu, bank, address.wrapping_add(1), (value >> 8) as u8);
    }

    fn get(value: u16) -> u16 {
        value
    }

    fn set(value: &mut u16, to: u16) {
        *value = to;
    }
}
