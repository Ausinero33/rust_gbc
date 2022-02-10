use super::mbc::MbcController;

pub struct Bus {
    pub rom: Option<Box<dyn MbcController>>,
    // ppu: PPU,
    eram: [u8; 0x2000],
    wram: [u8; 0x2000],
    hram: [u8; 0x200]
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            rom: None,
            eram: [0x00; 0x2000],
            wram: [0x00; 0x2000],
            hram: [0x00; 0x200]
        }
    }

    pub fn read(&self, dir: usize) -> u8 {
        match dir {
            0x0000 ..= 0x7FFF => self.rom.as_ref().unwrap().read(dir),
            0x8000 ..= 0x9FFF => /*ppu.read(dir)*/0,
            0xA000 ..= 0xBFFF => self.eram[dir - 0xA000],
            0xC000 ..= 0xDFFF => self.wram[dir - 0xC000],
            0xE000 ..= 0xFDFF => self.wram[dir - 0xE000],
            0xFE00 ..= 0xFFFF => self.hram[dir - 0xFE00],
            _ => panic!("Direccion inválida.")
        }
    }

    pub fn write(&mut self, dir: usize, val: u8) {
        let _a: u8;
        match dir {
            0x0000 ..= 0x7FFF => self.rom.as_mut().unwrap().write(dir, val),
            0x8000 ..= 0x9FFF => /*ppu.write(dir, val)*/_a = 1,
            0xA000 ..= 0xBFFF => self.eram[dir - 0xA000] = val,
            0xC000 ..= 0xDFFF => self.wram[dir - 0xC000] = val,
            0xE000 ..= 0xFDFF => self.wram[dir - 0xE000] = val,
            0xFE00 ..= 0xFFFF => {
                if dir == 0xFF04 {
                    self.hram[dir - 0xFE00] = 0;
                    return;
                }
                self.hram[dir - 0xFE00] = val
            },
            _ => panic!("Direccion inválida.")
        }
    }

    pub fn reset(&mut self) {
        self.write(0x7F06, 0x00);
        self.write(0x7F07, 0x00);
        self.write(0x7F10, 0x80);
        self.write(0x7F05, 0x00);
        self.write(0x7F11, 0xBF);
        self.write(0x7F12, 0xF3);
        self.write(0x7F14, 0xBF);
        self.write(0x7F16, 0x3F);
        self.write(0x7F17, 0x00);
        self.write(0x7F19, 0xBF);
        self.write(0x7F1A, 0x7F);
        self.write(0x7F1B, 0xFF);
        self.write(0x7F1C, 0x9F);
        self.write(0x7F1E, 0xBF);
        self.write(0x7F20, 0xFF);
        self.write(0x7F21, 0x00);
        self.write(0x7F22, 0x00);
        self.write(0x7F23, 0xBF);
        self.write(0x7F24, 0x77);
        self.write(0x7F25, 0xF3);
        self.write(0x7F26, 0xF1);
        self.write(0x7F40, 0x91);
        self.write(0x7F42, 0x00);
        self.write(0x7F43, 0x00);
        self.write(0x7F45, 0x00);
        self.write(0x7F47, 0xFC);
        self.write(0x7F48, 0xFF);
        self.write(0x7F49, 0xFF);
        self.write(0x7F4A, 0x00);
        self.write(0x7F4B, 0x00);
        self.write(0x7FFF, 0x00);
    }

    pub fn increase_div(&mut self) {
        let val = self.read(0x7F04).wrapping_add(1);
        self.write(0x7F04, val);
    }

    pub fn increase_tima(&mut self) -> bool {
        let val = self.read(0x7F05).overflowing_add(1);
        self.write(0x7F05, val.0);
        val.1
    }
}