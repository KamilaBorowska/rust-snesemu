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
