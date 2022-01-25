use std::{panic};

use super::mbc::*;

pub struct MMU {
    memory: [u8; 0x8000],
    mbc: Option<Box<dyn MbcController>>,
}

impl MMU {
    pub fn new() -> Self {
        MMU {
            mbc: None,                  // 0x0000 - 0x7FFF
            memory: [0x00; 0x8000],     // 0x8000 - 0xFFFF  
        }
    }

    pub fn reset(&mut self) {
        self.memory[0x7F05] = 0x00;
        self.memory[0x7F06] = 0x00; 
        self.memory[0x7F07] = 0x00; 
        self.memory[0x7F10] = 0x80;
        self.memory[0x7F11] = 0xBF;
        self.memory[0x7F12] = 0xF3;
        self.memory[0x7F14] = 0xBF;
        self.memory[0x7F16] = 0x3F;
        self.memory[0x7F17] = 0x00;
        self.memory[0x7F19] = 0xBF;
        self.memory[0x7F1A] = 0x7F;
        self.memory[0x7F1B] = 0xFF;
        self.memory[0x7F1C] = 0x9F;
        self.memory[0x7F1E] = 0xBF;
        self.memory[0x7F20] = 0xFF;
        self.memory[0x7F21] = 0x00;
        self.memory[0x7F22] = 0x00;
        self.memory[0x7F23] = 0xBF;
        self.memory[0x7F24] = 0x77;
        self.memory[0x7F25] = 0xF3;
        self.memory[0x7F26] = 0xF1;
        self.memory[0x7F40] = 0x91;
        self.memory[0x7F42] = 0x00; 
        self.memory[0x7F43] = 0x00; 
        self.memory[0x7F45] = 0x00; 
        self.memory[0x7F47] = 0xFC; 
        self.memory[0x7F48] = 0xFF;
        self.memory[0x7F49] = 0xFF;
        self.memory[0x7F4A] = 0x00; 
        self.memory[0x7F4B] = 0x00; 
        self.memory[0x7FFF] = 0x00; 
    }

    pub fn read(&self, dir: usize) -> u8 {
        if dir < 0x8000 {
            return self.mbc.as_ref().unwrap().read(dir);
        }
        self.memory[dir - 0x8000]
    }

    pub fn write(&mut self, dir: usize, val: u8) {
        if dir >= 0x8000 {
            if dir == 0xFF04 {
                self.memory[dir - 0x8000] = 0;
                return;
            }
            self.memory[dir - 0x8000] = val;
        } else {
            self.mbc.as_mut().unwrap().write(dir, val);
        }
    }

    pub fn load_rom(&mut self, dir: &str) {
        // let mut file = BufReader::new(File::open(dir).unwrap());
        // file.read_exact(&mut self.rom).unwrap();
        // file.read_exact(&mut self.memory[0..0x8000]).unwrap();

        // Cargar la ROM para saber su MBC
        let rom = std::fs::read(dir).unwrap();

        let mbc = rom[0x0147];

        match mbc {
            0x00 => self.mbc = Some(Box::new(MBC0::new(&rom))),
            0x01 => self.mbc = Some(Box::new(MBC1::new(&rom))),
            // 0x01 => self.mbc1(),
            _ => panic!("MBC Erroneo o no implementado."),
        }

    }

    pub fn increase_div(&mut self) {
        self.memory[0x7F04] = self.memory[0x7F04].wrapping_add(1);
    }

    pub fn increase_tima(&mut self) -> bool {
        let val = self.memory[0x7F05].overflowing_add(1);
        self.memory[0x7F05] = val.0;
        val.1
    }
}