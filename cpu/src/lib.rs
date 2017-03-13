#[macro_use]
extern crate bitflags;

pub mod bitwidth;
pub mod buffer;
pub mod cpu;
mod instructions;
pub mod mapper;

use cpu::CPU;
use mapper::Mapper;

pub struct Emulator<M: Mapper> {
    cpu: CPU<M>,
}

impl<M: Mapper> Emulator<M> {
    pub fn from_rom(mapper: M) -> Emulator<M> {
        Emulator { cpu: CPU::new(mapper) }
    }

    pub fn game_title(&self) -> [u8; 21] {
        let mut out = [0; 21];
        for (i, out_byte) in out.iter_mut().enumerate() {
            *out_byte = self.cpu.read(0x00, 0xFFC0 + i as u16);
        }
        out
    }

    pub fn run_frame(&mut self, buffer: &mut [u32; buffer::WIDTH * buffer::HEIGHT]) {
        for _ in 0..1_000_000 {
            self.cpu.step();
        }
    }
}
