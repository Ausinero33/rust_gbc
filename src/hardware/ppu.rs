use std::collections::VecDeque;

use sfml::graphics::Image;

type Tile = [[TilePixelValue; 8]; 8];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TilePixelValue {
    Zero,
    One,
    Two,
    Three
}

#[derive(Debug, PartialEq, Eq)]
pub enum PpuMode {
    HBlank,
    VBlank,
    OamScaning,
    Drawing,
}

#[derive(Debug, PartialEq, Eq)]
enum FetcherState {
    GetTile,
    GetDataLow,
    GetDataHigh,
    PushToFIFO,
}

const LCDC: usize = 0;
const STAT: usize = 1;
const SCY: usize = 2;
const SCX: usize = 3;
const LY: usize = 4;
const LYC: usize = 5;
const DMA: usize = 6;
const BGP: usize = 7;
const OBP0: usize = 8;
const OBP1: usize = 9;
const WY: usize = 10;
const WX: usize = 11;




pub struct PPU {
    pub vram: [u8; 0x2000],
    pub regs: [u8; 12],
    tile_set: [Tile; 384],

    pub mode: PpuMode,
    cycles: u64,

    scanline_counter: usize,

    fetcher_tilemap: usize,
    fetcher_x: usize,
    fetcher_state: FetcherState,
    fetcher_tile: usize,
    //fetcher_tile: [TilePixelValue; 8],

    data_low: u8,
    data_high: u8,

    //pub regs: PpuRegisters,

    background_fifo: VecDeque<TilePixelValue>,

    pub lcd_pixels: [u8; 144 * 160 * 4],
}



impl PPU {
    pub fn new() -> Self {
        PPU {
            vram: [0x00; 0x2000],
            regs: [0x00; 12],
            tile_set: [[[TilePixelValue::Zero; 8]; 8]; 384],

            mode: PpuMode::OamScaning,
            cycles: 0,

            scanline_counter: 0,

            fetcher_tilemap: 0x9800,
            fetcher_x: 0,
            fetcher_state: FetcherState::GetTile,
            fetcher_tile: 0,
            //fetcher_tile: [TilePixelValue::Zero; 8],

            data_low: 0,
            data_high: 0,

            background_fifo: VecDeque::with_capacity(16),

            lcd_pixels: [0x00; 144 * 160 * 4],
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
        let index = dir - 0x8000;
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

    pub fn get_image(&self) -> Image {
        Image::create_from_pixels(160, 144, &self.lcd_pixels).unwrap()
    }

    // fn check_mode(&mut self) {
    //     if self.cycles % 456 < 80 && self.regs[LY] < 144 {
    //         self.mode = PpuMode::OamScaning;
    //     } else if self.cycles % 456 < 252 && self.regs[LY] < 144 {
    //         self.mode = PpuMode::Drawing;
    //     } else if self.regs[LY] < 144 {
    //         self.mode = PpuMode::HBlank;
    //     } else {
    //         self.mode = PpuMode::VBlank;
    //     }
    // }

    pub fn cycle(&mut self, cycles: u8) {
        // TODO Hacer cada modo
        let mut cycles_to_tick = cycles;

        while cycles_to_tick > 0 {
            //self.check_mode();

            match self.mode {
                PpuMode::HBlank => {

                    self.regs[STAT] = self.regs[STAT] & 0b11111100 + 0b00;

                    if self.scanline_counter == 455 {
                        if self.regs[LY] == 143 {
                            self.mode = PpuMode::VBlank;
                        } else {
                            self.mode = PpuMode::OamScaning;
                        }
                    }
                },
                PpuMode::OamScaning => {

                    self.regs[STAT] = self.regs[STAT] & 0b11111100 + 0b10;

                    if self.scanline_counter == 79 {
                        self.mode = PpuMode::Drawing;
                        self.fetcher_x = 0;
                        self.background_fifo.clear();
                    }
                },
                PpuMode::Drawing => {
                    // if self.regs[LCDC] & 0b00000001 != 0 {
                    //     self.fifo_cycle();
                    //     self.fetcher_cycle(&cycles_to_tick);
                    // }

                    self.fetcher_cycle(&cycles_to_tick);
                    self.fifo_cycle();

                    self.regs[STAT] = self.regs[STAT] & 0b11111100 + 0b11;

                    if self.scanline_counter == 251 {
                        self.mode = PpuMode::HBlank;
                    }
                },
                PpuMode::VBlank => {

                    self.regs[STAT] = self.regs[STAT] & 0b11111100 + 0b01;

                    if self.scanline_counter == 455 && self.regs[LY] == 153 {
                        self.mode = PpuMode::OamScaning;
                    }
                }
            }

            if self.regs[LY] == self.regs[LYC] {
                self.regs[STAT] |= 0b00000100;
            } else {
                self.regs[STAT] &= 0b01111011;
            }

            self.regs[LY] = ((self.cycles / 456) % 154) as u8;

            self.cycles += 1;
            self.scanline_counter = (self.cycles % 456) as usize;
            cycles_to_tick -= 1;
        }
    }



    fn fetcher_cycle(&mut self, cycles: &u8) {
        if cycles % 2 != 0 {
            return;
        }

        match self.fetcher_state {
            FetcherState::GetTile => {
                if self.regs[LCDC] & 0b00001000 != 0 && false /*TODO Saber si no está en pantalla, false temporal para que no entre */ {
                    self.fetcher_tilemap = 0x9C00;
                }
                if self.regs[LCDC] & 0b01000000 != 0 && false /*TODO Saber si está en pantalla, false temporal para que no entre */ {
                    self.fetcher_tilemap = 0x9C00;
                }

                let x = if false /*TODO WINDOW */ {
                    0
                } else {
                    ((self.regs[SCX] as usize / 8) + self.fetcher_x) & 0x1F
                };

                let y = if false /*TODO WINDOW */ {
                    0
                } else {
                    32 * (((self.regs[LY] as usize + self.regs[SCY] as usize) & 255) / 8)
                };

                self.fetcher_tile = self.read(x + y + self.fetcher_tilemap) as usize;
                
                self.fetcher_state = FetcherState::GetDataLow;
            },
            FetcherState::GetDataLow => {
                let mut dir = self.fetcher_tile * 0x10 + 2 * ((self.regs[LY] as usize + self.regs[SCY] as usize) % 8);

                if self.regs[LCDC] & 0b00010000 != 0 {
                    dir += 0x8000;
                } else {
                    dir += 0x8800;
                }

                self.data_low = self.read(dir);

                self.fetcher_state = FetcherState::GetDataHigh;
            },
            FetcherState::GetDataHigh => {
                let mut dir = self.fetcher_tile * 0x10 + 2 * ((self.regs[LY] as usize + self.regs[SCY] as usize) % 8) + 1;

                if self.regs[LCDC] & 0b00010000 != 0 {
                    dir += 0x8000;
                } else {
                    dir += 0x8800;
                }

                self.data_high = self.read(dir);

                self.fetcher_state = FetcherState::PushToFIFO;
            },
            FetcherState::PushToFIFO => {
                if self.background_fifo.len() > 8 {
                    return;
                }
                for i in 0..8 {
                    let mask = 1 << (7 - i);
                    let low = self.data_low & mask;
                    let high = self.data_high & mask;

                    let value = match (low != 0, high != 0) {
                        (true, true) => TilePixelValue::Three,
                        (true, false) => TilePixelValue::Two,
                        (false, true) => TilePixelValue::One,
                        (false, false) => TilePixelValue::Zero,
                    };

                    self.background_fifo.push_back(value);
                }

                self.fetcher_x = self.fetcher_x.wrapping_add(1) % 32;

                self.fetcher_state = FetcherState::GetTile;
            }
        }
    }

    fn fifo_cycle(&mut self) {
        if self.background_fifo.is_empty() {
            return;
        }

        // TODO Seguramente que esto este aqui significa que hay algo mal en algun sitio.
        if self.scanline_counter < 86 || self.scanline_counter - 86 >= 160 || self.regs[LY] >= 144 {
            return;
        }

        let pixel = self.background_fifo.pop_front().unwrap();

        let pos = ((self.scanline_counter - 86) + self.regs[LY] as usize * 160) * 4;


        match pixel {
            TilePixelValue::Zero => {
                self.lcd_pixels[pos] = 0x9B;
                self.lcd_pixels[pos + 1] = 0xBC;
                self.lcd_pixels[pos + 2] = 0x0F;
                self.lcd_pixels[pos + 3] = 0xFF;
            },
            TilePixelValue::One => {
                self.lcd_pixels[pos] = 0x8B;
                self.lcd_pixels[pos + 1] = 0xAC;
                self.lcd_pixels[pos + 2] = 0x0F;
                self.lcd_pixels[pos + 3] = 0xFF;
            }
            TilePixelValue::Two => {
                self.lcd_pixels[pos] = 0x30;
                self.lcd_pixels[pos + 1] = 0x62;
                self.lcd_pixels[pos + 2] = 0x30;
                self.lcd_pixels[pos + 3] = 0xFF;
            }
            TilePixelValue::Three => {
                self.lcd_pixels[pos] = 0x0F;
                self.lcd_pixels[pos + 1] = 0x38;
                self.lcd_pixels[pos + 2] = 0x0F;
                self.lcd_pixels[pos + 3] = 0xFF;
            }
        }
    }
}