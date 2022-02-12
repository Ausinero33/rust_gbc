use std::collections::VecDeque;

type Tile = [[TilePixelValue; 8]; 8];

#[derive(Clone, Copy)]
enum TilePixelValue {
    Zero,
    One,
    Two,
    Three
}

enum PpuMode {
    HBlank,
    VBlank,
    OamScaning,
    Drawing,
}

pub struct PpuRegisters {
    pub lcdc: u8,
    pub stat: u8,
    pub scy: u8,
    pub scx: u8,
    pub ly: u8,
    pub lyc: u8,
    pub dma: u8,
    pub bgp: u8,
    pub obp0: u8,
    pub obp1: u8,
    pub wy: u8,
    pub wx: u8,
}

pub struct PPU {
    vram: [u8; 0x2000],
    tile_set: [Tile; 384],

    // 0 -> HBLANK
    // 1 -> VBLANK
    // 2 -> OAM SCAN
    // 3 -> DRAWING
    mode: PpuMode,

    step: u8,
    tile_x: u8,
    tile_y: u8,

    pub regs: PpuRegisters,

    background_fifo: VecDeque<TilePixelValue>,
}



impl PPU {
    pub fn new() -> Self {
        PPU {
            vram: [0x00; 0x2000],
            tile_set: [[[TilePixelValue::Zero; 8]; 8]; 384],
            mode: PpuMode::OamScaning,

            step: 0,
            tile_x: 0,
            tile_y: 0,

            regs: PpuRegisters { lcdc: 0, stat: 0, scy: 0, scx: 0, ly: 0, lyc: 0, dma: 0, bgp: 0, obp0: 0, obp1: 0, wy: 0, wx: 0 },

            background_fifo: VecDeque::with_capacity(16),
        }
    }

    pub fn read(&self, dir: usize) -> u8 {
        self.read_vram(dir)
    }

    pub fn write(&mut self, dir: usize, val: u8) {
        self.write_vram(dir, val);
    }

    fn read_vram(&self, dir: usize) -> u8 {
        // TODO Mejorar esto
        self.vram[dir - 0x8000]
    }

    fn write_vram(&mut self, dir: usize, val: u8) {
        // TODO Mejorar esto
        let index = dir -0x8000;
        self.vram[index] = val;

        if index >= 0x1800 {
            return;
        }

        let normalized_index = index & 0xFFFE;
        let byte1 = self.vram[normalized_index];
        let byte2 = self.vram[normalized_index + 1];

        let tile_index = index / 16;
        let row_index = (index % 16) / 2;

        for pixel_index in 0..8 {
            let mask = 1 << (7 - pixel_index);
            let lsb = byte1 & mask;
            let msb = byte2 & mask;

            let value = match (lsb != 0, msb != 0) {
                (true, true) => TilePixelValue::Three,
                (true, false) => TilePixelValue::Two,
                (false, true) => TilePixelValue::One,
                (false, false) => TilePixelValue::Zero,
            };
            self.tile_set[tile_index][row_index][pixel_index] = value;
        }
    }

    pub fn cycle(&mut self, cycles: u8) {

    }



    fn get_tile(&mut self) {
        
    }

}