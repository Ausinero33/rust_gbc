use sfml::graphics::{Sprite, Texture, Transformable, RenderWindow, RenderTarget};

use self::{cpu::CPU, bus::{Bus, Interrupts}, mbc::*};

pub mod cpu;
mod ppu;
mod inst_set;
mod mbc;
pub mod bus;

pub enum Keys {
    Down,
    Up,
    Left,
    Right,
    Start,
    Select,
    B,
    A,
}

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
            0x00 => self.cpu.bus.set_rom(Some(Box::new(MBC0::new(&rom)))),
            0x01 => self.cpu.bus.set_rom(Some(Box::new(MBC0::new(&rom)))),
            _ => panic!("MBC Erroneo o no implementado."),
        }
    }

    pub fn reset(&mut self) {
        self.cpu.reset(self.enable_boot_rom);
    }

    pub fn cycle(&mut self) {
        let mut cycles = 0;
        
        while cycles < 70224 {
            let mut cycles_to_run = self.cpu.interrupt();

            if self.cpu.pc == 0x388 {
                let _a = 0;
            }
            
            cycles_to_run += self.cpu.cycle();
            self.cpu.bus.cycle(cycles_to_run as u8);
            
            self.output_temp();
            cycles += cycles_to_run;
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

    pub fn set_input(&mut self, key: Keys, pressed: bool) {
        let prev_joyp = self.cpu.bus.read(0xFF00);
        let val = match key {
            Keys::Down | Keys::Start => {
                if pressed {
                    prev_joyp & 0b11110111
                } else {
                    prev_joyp | 0b00001000
                }
            },
            Keys::Up | Keys::Select => {
                if pressed {
                    prev_joyp & 0b11111011
                } else {
                    prev_joyp | 0b00000100
                }
            },
            Keys::Left | Keys::B => {
                if pressed {
                    prev_joyp & 0b11111101
                } else {
                    prev_joyp | 0b00000010
                }
            },
            Keys::Right | Keys::A => {
                if pressed {
                    prev_joyp & 0b11111110
                } else {
                    prev_joyp | 0b00000001
                }
            },
        };
        self.cpu.bus.set_joyp(val);
        self.cpu.bus.set_int(Interrupts::Joypad);
    }

    pub fn debug_vram(&self) {
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

    pub fn debug_background(&self) {
        for i in 0..0x400 {
            println!("{:04X}: {:02X}", 0x9800 + i, self.cpu.bus.ppu.vram[0x1800 + i]);
        }
    }

    pub fn debug_frame(&self) {
        for row in 0..144 {
            for col in 0..160 {
                match (self.cpu.bus.ppu.lcd_pixels[(row * 160 + col) * 4], self.cpu.bus.ppu.lcd_pixels[(row * 160 + col) * 4 + 1], self.cpu.bus.ppu.lcd_pixels[(row * 160 + col) * 4 + 2]) {
                    (0xE0, 0xF8, 0xD0) => print!("\u{2588}"),
                    (0x88, 0xC0, 0x70) => print!("\u{2593}"),
                    (0x34, 0x68, 0x56) => print!("\u{2592}"),
                    (0x08, 0x18, 0x20) => print!("\u{2591}"),
                    _ => unreachable!()
                }
            }
            println!();
        }
    }
}
