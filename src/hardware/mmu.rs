pub struct MMU {
    memory: [u8; 0x10000],
}

impl MMU {
    pub fn new() -> Self {
        MMU {
            memory: [0x00; 0x10000],
        }
    }

    pub fn read(&self, dir: usize) -> u8 {
        self.memory[dir]
    }

    pub fn write(&mut self, dir: usize, val: u8) {
        self.memory[dir] = val;
    }
}