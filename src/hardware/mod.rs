use self::{cpu::CPU, bus::Bus, mbc::*};

pub mod cpu;
//mod mmu;
mod inst_set;
mod mbc;
pub mod bus;

pub struct GameBoy {
    pub cpu: CPU,
}

impl GameBoy {
    pub fn new(bus: Bus) -> Self {
        GameBoy {
            cpu: CPU::new(bus)
        }   
    }

    pub fn load_rom(&mut self, dir: &str) {
        let rom = std::fs::read(dir).unwrap();

        let mbc = rom[0x0147];

        match mbc {
            0x00 => self.cpu.bus.rom = Some(Box::new(MBC0::new(&rom))),
            0x01 => self.cpu.bus.rom = Some(Box::new(MBC1::new(&rom))),
            // 0x01 => self.mbc1(),
            _ => panic!("MBC Erroneo o no implementado."),
        }
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }
}