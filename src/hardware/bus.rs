use super::{mbc::MbcController, ppu::PPU};

pub struct Bus {
    pub rom: Option<Box<dyn MbcController>>,    // 0x0000 - 0x7FFF
    ppu: PPU,                                   // 0x8000 - 0x9FFF
    eram: [u8; 0x2000],                         // 0xA000 - 0xBFFF
    wram: [u8; 0x2000],                         // 0xC000 - 0xDFFF (0xE000 - 0xFDFF)
    hram: [u8; 0x200]                           // 0xFE00 - 0xFFFF
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            rom: None,
            ppu: PPU::new(),
            eram: [0x00; 0x2000],
            wram: [0x00; 0x2000],
            hram: [0x00; 0x200]
        }
    }

    pub fn read(&self, dir: usize) -> u8 {
        match dir {
            0x0000 ..= 0x7FFF => self.rom.as_ref().unwrap().read(dir),
            0x8000 ..= 0x9FFF => self.ppu.read(dir),
            0xA000 ..= 0xBFFF => self.eram[dir - 0xA000],
            0xC000 ..= 0xDFFF => self.wram[dir - 0xC000],
            0xE000 ..= 0xFDFF => {
                if dir >= 0xFEA0 && dir <= 0xFEFF {
                    return 0x00;
                };
                self.wram[dir - 0xE000]
            },
            0xFE00 ..= 0xFFFF => self.hram[dir - 0xFE00],
            _ => panic!("Direccion inválida.")
        }
    }

    pub fn write(&mut self, dir: usize, val: u8) {
        match dir {
            0x0000 ..= 0x7FFF => self.rom.as_mut().unwrap().write(dir, val),
            0x8000 ..= 0x9FFF => self.ppu.write(dir, val),
            0xA000 ..= 0xBFFF => self.eram[dir - 0xA000] = val,
            0xC000 ..= 0xDFFF => self.wram[dir - 0xC000] = val,
            0xE000 ..= 0xFDFF => self.wram[dir - 0xE000] = val,
            0xFE00 ..= 0xFFFF => {
                if dir >= 0xFEA0 && dir <= 0xFEFF {
                    return;
                } 
                else if dir == 0xFF04 {
                    self.hram[dir - 0xFE00] = 0;
                    return;
                }
                self.hram[dir - 0xFE00] = val
            },
            _ => panic!("Direccion inválida.")
        }
    }

    pub fn reset(&mut self) {
        self.write(0xFF06, 0x00);
        self.write(0xFF07, 0x00);
        self.write(0xFF10, 0x80);
        self.write(0xFF05, 0x00);
        self.write(0xFF11, 0xBF);
        self.write(0xFF12, 0xF3);
        self.write(0xFF14, 0xBF);
        self.write(0xFF16, 0x3F);
        self.write(0xFF17, 0x00);
        self.write(0xFF19, 0xBF);
        self.write(0xFF1A, 0x7F);
        self.write(0xFF1B, 0xFF);
        self.write(0xFF1C, 0x9F);
        self.write(0xFF1E, 0xBF);
        self.write(0xFF20, 0xFF);
        self.write(0xFF21, 0x00);
        self.write(0xFF22, 0x00);
        self.write(0xFF23, 0xBF);
        self.write(0xFF24, 0x77);
        self.write(0xFF25, 0xF3);
        self.write(0xFF26, 0xF1);
        self.write(0xFF40, 0x91);
        self.write(0xFF42, 0x00);
        self.write(0xFF43, 0x00);
        self.write(0xFF45, 0x00);
        self.write(0xFF47, 0xFC);
        self.write(0xFF48, 0xFF);
        self.write(0xFF49, 0xFF);
        self.write(0xFF4A, 0x00);
        self.write(0xFF4B, 0x00);
        self.write(0xFFFF, 0x00);
    }

    pub fn increase_div(&mut self) {
        let val = self.read(0xFF04).wrapping_add(1);
        self.write(0xFF04, val);
    }

    pub fn increase_tima(&mut self) -> bool {
        let val = self.read(0xFF05).overflowing_add(1);
        self.write(0xFF05, val.0);
        val.1
    }
}