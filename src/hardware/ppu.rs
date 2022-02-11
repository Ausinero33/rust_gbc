pub struct PPU {
    vram: [u8; 0x2000],
    palette: [u8; 4],
}

impl PPU {
    pub fn new() -> Self {
        PPU {
            vram: [0x00; 0x2000],
            palette: [0x00, 0x01, 0x10, 0x11],
        }
    }

    pub fn read(&self, dir: usize) -> u8 {
        // TODO Mejorar esto
        self.vram[dir - 0x8000]
    }

    pub fn write(&mut self, dir: usize, val: u8) {
        // TODO Mejorar esto
        self.vram[dir - 0x8000] = val;
    }
}