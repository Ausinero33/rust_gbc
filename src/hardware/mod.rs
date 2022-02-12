use self::{cpu::CPU, bus::Bus, mbc::*};

pub mod cpu;
mod ppu;
mod inst_set;
mod mbc;
pub mod bus;

const FREQ: u32 = 4_194_304;

pub struct GameBoy {
    pub cpu: CPU,
    enable_boot_rom: bool,
}

impl GameBoy {
    pub fn new(bus: Bus, enable_boot_rom: bool) -> Self {
        GameBoy {
            cpu: CPU::new(bus.set_enable_boot_rom(enable_boot_rom)),
            enable_boot_rom: enable_boot_rom,
        }   
    }

    pub fn load_rom(&mut self, dir: &str) {
        let rom = std::fs::read(dir).unwrap();

        let mbc = rom[0x0147];

        match mbc {
            0x00 => self.cpu.bus.rom = Some(Box::new(MBC0::new(&rom))),
            0x01 => self.cpu.bus.rom = Some(Box::new(MBC0::new(&rom))),
            // 0x01 => self.mbc1(),
            _ => panic!("MBC Erroneo o no implementado."),
        }
    }

    pub fn reset(&mut self) {
        self.cpu.reset(self.enable_boot_rom);
    }

    pub fn cycle(&mut self) {
        for _i in 0..(FREQ / 60) {
            //println!("{:02X}", self.cpu.pc);
            let cycles_to_run = self.cpu.cycle();
            self.cpu.bus.cycle(cycles_to_run as u8);
            self.output_temp();
        };
    }

    fn output_temp(&mut self) {
        if self.cpu.bus.read(0xff02 as usize) == 0x81 {
            let c = self.cpu.bus.read(0xff01 as usize);
            print!("{}", char::from(c));
            self.cpu.bus.write(0xff02 as usize, 0);
        }
    }
}