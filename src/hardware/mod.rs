use ggez::{GameError, event::EventHandler, graphics::{self, Color}};

use self::{cpu::CPU, mmu::MMU, ppu::PPU};

pub mod cpu;
pub mod mmu;
mod inst_set;
mod mbc;
mod ppu;

const FREQ: u32 = 4_194_304;

pub struct GameBoy {
    cpu: CPU,
    mmu: MMU,
    ppu: PPU,
}

impl GameBoy {
    pub fn new() -> Self {
        GameBoy{
            cpu: CPU::new(),
            mmu: MMU::new(),
            ppu: PPU::new(),
        }
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        self.mmu.reset();
    }

    pub fn output_temp(&mut self) {
        if self.mmu.read(0xff02 as usize) == 0x81 {
            let c = self.mmu.read(0xff01 as usize);
            print!("{}", char::from(c));
            self.mmu.write(0xff02 as usize, 0);
        }
    }

    pub fn cycle(&mut self) {
        for _i in 0..(FREQ / 60) {
            self.cpu.cycle(&mut self.mmu);
            self.output_temp();
        };
    }

    pub fn load_rom(&mut self, dir: &str) {
        self.mmu.load_rom(dir)
    }
}

impl EventHandler for GameBoy {
    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), GameError> {
        for _i in 0..(FREQ / 60) {
            self.cpu.cycle(&mut self.mmu);
            self.output_temp();
        };
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        graphics::clear(ctx, Color::WHITE);
        graphics::present(ctx)
    }
}