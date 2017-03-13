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

extern crate snesemu_cpu;
extern crate minifb;

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::thread;
use std::time::{Duration, Instant};

use minifb::{Window, WindowOptions};

use snesemu_cpu::{buffer, Emulator};
use snesemu_cpu::mapper::LoROM;

const NANOS_PER_SEC: u32 = 1_000_000_000;

fn gui() -> Result<(), Box<Error + Send + Sync>> {
    let mut buffer = [0; buffer::WIDTH * buffer::HEIGHT];
    let mut window = Window::new("SNES Emulator",
                                 buffer::WIDTH,
                                 buffer::HEIGHT,
                                 WindowOptions::default())?;

    let mut rom = Vec::new();
    File::open("/home/xfix/ROM/Super Mario World (USA).sfc")?.read_to_end(&mut rom)?;

    let mut emulator = Emulator::from_rom(LoROM::new(match rom.len() % 0x1000 {
        0x200 => &rom[0x200..],
        _ => &rom,
    }));

    let title = emulator.game_title();
    println!("Loaded game \"{}\".",
             String::from_utf8_lossy(&title).trim_right());

    let frame_duration = Duration::new(0, NANOS_PER_SEC / 60);
    while window.is_open() {
        let start = Instant::now();

        emulator.run_frame(&mut buffer);

        window.update_with_buffer(&buffer);
        let elapsed = start.elapsed();
        if frame_duration > elapsed {
            thread::sleep(frame_duration - elapsed);
        }
    }

    Ok(())
}

fn main() {
    match gui() {
        Ok(()) => {}
        Err(err) => panic!(err),
    }
}
