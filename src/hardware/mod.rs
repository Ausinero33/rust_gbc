use sfml::graphics::{Sprite, Texture, Transformable, RenderWindow, RenderTarget};

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
            0x01 => self.cpu.bus.rom = Some(Box::new(MBC1::new(&rom))),
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

            if self.cpu.pc == 0x100 {
                // self.debug_vram();
                // println!("Background");
                // self.debug_background();
                self.debug_frame();
            }

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

    pub fn draw(&self, window: &mut RenderWindow) {
        let image = self.cpu.bus.ppu.get_image();
        let texture = Texture::from_image(&image).unwrap();
        let mut sprite = Sprite::with_texture(&texture);
        sprite.set_scale((2.0, 2.0));
        window.draw(&sprite);
    }


    fn debug_vram(&self) {
        let mut step = 0;
        let mut low = 0;
        let mut high;
        let mut tiles = 0;
        for i in self.cpu.bus.ppu.vram {
            if step > 0x17FF {
                println!("{}", tiles / 8);
                return;
            }

            if step % 2 == 0 {
                low = i;
            } else {
                high = i;

                for i in 0..8 {
                    let mask = 1 << (7 - i);
                    let data_low = low & mask;
                    let data_high = high & mask;

                    match (data_low != 0, data_high != 0) {
                        (true, true) => print!("\u{2588}"),
                        (true, false) => print!("\u{2593}"),
                        (false, true) => print!("\u{2592}"),
                        (false, false) => print!("\u{2591}"),
                    };

                    
                }
                println!();
                tiles += 1;
            }

            if step % 16 == 0 {
                println!("{:04X}", 0x8000 + step);
            }

            step += 1;
        }
    }

    fn debug_background(&self) {
        for i in 0..0x400 {
            println!("{:04X}: {:02X}", 0x9800 + i, self.cpu.bus.ppu.vram[0x1800 + i]);
        }
    }

    fn debug_frame(&self) {
        for row in 0..144 {
            for col in 0..160 {
                match (self.cpu.bus.ppu.lcd_pixels[(row * 160 + col) * 4], self.cpu.bus.ppu.lcd_pixels[(row * 160 + col) * 4 + 1], self.cpu.bus.ppu.lcd_pixels[(row * 160 + col) * 4 + 2]) {
                    (0x0F, 0x38, 0x0F) => print!("\u{2588}"),
                    (0x30, 0x62, 0x30) => print!("\u{2593}"),
                    (0x8B, 0xAC, 0x0F) => print!("\u{2592}"),
                    (0x9B, 0xBC, 0x0F) => print!("\u{2591}"),
                    _ => print!(""),
                }
            }
            println!();
        }
    }
}