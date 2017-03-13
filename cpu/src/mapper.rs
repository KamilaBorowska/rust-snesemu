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

pub trait Mapper {
    fn read(&self, bank: u8, address: u16) -> u8;
    fn write(&mut self, bank: u8, address: u16, value: u8);
}

pub struct LoROM<'a> {
    rom: &'a [u8],
}

impl<'a> LoROM<'a> {
    pub fn new(rom: &[u8]) -> LoROM {
        LoROM { rom: rom }
    }
}

impl<'a> Mapper for LoROM<'a> {
    fn read(&self, bank: u8, address: u16) -> u8 {
        self.rom[bank as usize * 0x8000 + (address as usize & 0x7FFF)]
    }

    fn write(&mut self, bank: u8, address: u16, value: u8) {
        println!("{:02x}:{:04x} = {:02x}", bank, address, value);
        // TODO: Save RAM support
        unimplemented!();
    }
}
