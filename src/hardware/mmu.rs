use std::{io::{BufReader, Read}, fs::File};

pub struct MMU {
    memory: [u8; 0x10000],
}

impl MMU {
    pub fn new() -> Self {
        MMU {
            memory: [0x00; 0x10000],
        }
    }

    pub fn reset(&mut self) {
        self.memory[0xFF05] = 0x00;
        self.memory[0xFF06] = 0x00; 
        self.memory[0xFF07] = 0x00; 
        self.memory[0xFF10] = 0x80;
        self.memory[0xFF11] = 0xBF;
        self.memory[0xFF12] = 0xF3;
        self.memory[0xFF14] = 0xBF;
        self.memory[0xFF16] = 0x3F;
        self.memory[0xFF17] = 0x00;
        self.memory[0xFF19] = 0xBF;
        self.memory[0xFF1A] = 0x7F;
        self.memory[0xFF1B] = 0xFF;
        self.memory[0xFF1C] = 0x9F;
        self.memory[0xFF1E] = 0xBF;
        self.memory[0xFF20] = 0xFF;
        self.memory[0xFF21] = 0x00;
        self.memory[0xFF22] = 0x00;
        self.memory[0xFF23] = 0xBF;
        self.memory[0xFF24] = 0x77;
        self.memory[0xFF25] = 0xF3;
        self.memory[0xFF26] = 0xF1;
        self.memory[0xFF40] = 0x91;
        self.memory[0xFF42] = 0x00; 
        self.memory[0xFF43] = 0x00; 
        self.memory[0xFF45] = 0x00; 
        self.memory[0xFF47] = 0xFC; 
        self.memory[0xFF48] = 0xFF;
        self.memory[0xFF49] = 0xFF;
        self.memory[0xFF4A] = 0x00; 
        self.memory[0xFF4B] = 0x00; 
        self.memory[0xFFFF] = 0x00; 
    }

    pub fn read(&self, dir: usize) -> u8 {
        self.memory[dir]
    }

    pub fn write(&mut self, dir: usize, val: u8) {
        self.memory[dir] = val;
    }

    pub fn load_rom(&mut self, dir: &str) {
        let mut file = BufReader::new(File::open(dir).unwrap());
        file.read_exact(&mut self.memory[0..0x8000]).unwrap();
    }
}