use std::collections::VecDeque;

use sfml::graphics::Image;

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
    lcd_x: usize,
    pub lcd_pixels: [u8; 144 * 160 * 4],
    colors: [Color; 4]
}



impl PPU {
    pub fn new() -> Self {
        PPU {
            vram: [0x00; 0x2000],
            regs: [0x00; 12],

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
            lcd_x: 0,
            lcd_pixels: [0x00; 144 * 160 * 4],
            colors: [
                Color{r: 0xE0, g: 0xF8, b: 0xD0},
                Color{r: 0x88, g: 0xC0, b: 0x70},
                Color{r: 0x34, g: 0x68, b: 0x56},
                Color{r: 0x08, g: 0x18, b: 0x20},
            ]
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
    }

    pub fn get_image(&self) -> Image {
        Image::create_from_pixels(160, 144, &self.lcd_pixels).unwrap()
    }

    // (VBlank, STAT)
    pub fn cycle(&mut self, cycles: u8) -> (bool, bool) {
        let mut cycles_to_tick = cycles;

        let mut int = (false, false);

        if self.regs[LCDC] & 0b10000000 == 0 {
            return int;
        }

        while cycles_to_tick > 0 {
            self.regs[STAT] &= 0b10000111;

            match self.mode {
                PpuMode::HBlank => {

                    self.regs[STAT] = self.regs[STAT] & 0b11111100 + 0b00;

                    if self.scanline_counter == 455 {
                        if self.regs[LY] == 143 {
                            self.mode = PpuMode::VBlank;
                            int.0 = true;
                            self.regs[STAT] |= 0b00010000;
                        } else {
                            self.mode = PpuMode::OamScaning;
                            self.regs[STAT] |= 0b00100000;
                        }
                    }
                },
                PpuMode::OamScaning => {

                    self.regs[STAT] = self.regs[STAT] & 0b11111100 + 0b10;

                    if self.scanline_counter == 79 {
                        self.mode = PpuMode::Drawing;
                    }
                },
                PpuMode::Drawing => {
                    self.fetcher_cycle(&cycles_to_tick);
                    self.fifo_cycle();

                    self.regs[STAT] = self.regs[STAT] & 0b11111100 + 0b11;

                    if self.scanline_counter == 251 {
                        self.mode = PpuMode::HBlank;
                        self.regs[STAT] |= 0b00001000;
                        self.fetcher_x = 0;
                        self.lcd_x = 0;
                        self.background_fifo.clear();
                        self.fetcher_state = FetcherState::GetTile;
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
                self.regs[STAT] |= 0b01000000;
            } else {
                self.regs[STAT] &= 0b11111011;
            }

            if self.regs[STAT] & 0b01111100 != 0 {
                int.1 = true;
            }

            self.cycles += 1;
            self.regs[LY] = ((self.cycles / 456) % 154) as u8;
            self.scanline_counter = (self.cycles % 456) as usize;
            cycles_to_tick -= 1;
        }

        return int;
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
                let dir = if self.regs[LCDC] & 0b00010000 != 0 {
                    0x8000 + (self.fetcher_tile * 0x10) + 2 * ((self.regs[LY] as usize + self.regs[SCY] as usize) % 8) + 1
                } else {
                    // TODO ARREGLAR ESTO
                    let signed_tile = self.fetcher_tile as i8 as i32;
                    let x_offset = 0x9000 + signed_tile * 0x10 as i32;
                    x_offset as usize + 2 * ((self.regs[LY] as usize + self.regs[SCY] as usize) % 8) + 1
                };

                self.data_low = self.read(dir);

                self.fetcher_state = FetcherState::GetDataHigh;
            },
            FetcherState::GetDataHigh => {
                let dir = if self.regs[LCDC] & 0b00010000 != 0 {
                    0x8000 + (self.fetcher_tile * 0x10) + 2 * ((self.regs[LY] as usize + self.regs[SCY] as usize) % 8)
                } else {
                    let signed_tile = self.fetcher_tile as i8 as i32;
                    let x_offset = 0x9000 + signed_tile * 0x10 as i32;
                    x_offset as usize + 2 * ((self.regs[LY] as usize + self.regs[SCY] as usize) % 8)
                };

                self.data_high = self.read(dir);

                if self.fetcher_tile == 0x18 {
                    let _a = 1;
                }

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

                self.fetcher_x = (self.fetcher_x + 1) % 32;
                self.fetcher_state = FetcherState::GetTile;
            }
        }
    }

    fn fifo_cycle(&mut self) {
        if self.background_fifo.is_empty() {
            return;
        }

        // TODO Seguramente que esto este aqui significa que hay algo mal en algun sitio.
        if self.scanline_counter - 86 >= 160 || self.regs[LY] >= 144 {
            return;
        }

        let pixel = self.background_fifo.pop_front().unwrap();

        let pos = (self.scanline_counter - 86 + self.regs[LY] as usize * 160) * 4;

        let palette = [
            self.colors[0b00000011 & self.regs[BGP] as usize],
            self.colors[(0b00001100 & self.regs[BGP] as usize) >> 2],
            self.colors[(0b00110000 & self.regs[BGP] as usize) >> 4],
            self.colors[(0b11000000 & self.regs[BGP] as usize) >> 6],
        ];

        match pixel {
            TilePixelValue::Zero => {
                self.lcd_pixels[pos] = palette[0].r;
                self.lcd_pixels[pos + 1] = palette[0].g;
                self.lcd_pixels[pos + 2] = palette[0].b;
                self.lcd_pixels[pos + 3] = 0xFF;
            },
            TilePixelValue::One => {
                self.lcd_pixels[pos] = palette[1].r;
                self.lcd_pixels[pos + 1] = palette[1].g;
                self.lcd_pixels[pos + 2] = palette[1].b;
                self.lcd_pixels[pos + 3] = 0xFF;
            }
            TilePixelValue::Two => {
                self.lcd_pixels[pos] = palette[2].r;
                self.lcd_pixels[pos + 1] = palette[2].g;
                self.lcd_pixels[pos + 2] = palette[2].b;
                self.lcd_pixels[pos + 3] = 0xFF;
            }
            TilePixelValue::Three => {
                self.lcd_pixels[pos] = palette[3].r;
                self.lcd_pixels[pos + 1] = palette[3].g;
                self.lcd_pixels[pos + 2] = palette[3].b;
                self.lcd_pixels[pos + 3] = 0xFF;
            }
        }
    }
}

#[derive(Clone, Copy)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}