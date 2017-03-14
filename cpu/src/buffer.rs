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

/// Width of a rendering buffer.
///
/// # Examples
///
/// ```
/// use snesemu_cpu::buffer::{WIDTH, HEIGHT};
/// use snesemu_cpu::Emulator;
/// use snesemu_cpu::mapper::LoROM;
///
/// let mut buffer = [0; WIDTH * HEIGHT];
///
/// let mut rom = [0; 0x8000];
/// // infinite loop
/// // label: BRA label
/// rom[..2].copy_from_slice(&[0x80, 0xFE]);
/// // Reset vector is at 0x8000
/// rom[0x7FFC..0x7FFE].copy_from_slice(&[0x00, 0x80]);
///
/// let mut emulator = Emulator::from_rom(LoROM::new(&rom));
/// emulator.run_frame(&mut buffer);
/// ```
pub const WIDTH: usize = 512;

/// Height of a rendering buffer.
pub const HEIGHT: usize = 480;
