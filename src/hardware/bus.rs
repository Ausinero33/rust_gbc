use super::{mbc::MbcController, ppu::PPU};

// Interrupciones
pub enum Interrupts {
    VBlank,
    LcdStat,
    Timer,
    Serial,
    Joypad,
}

pub struct Bus {
    rom: Option<Box<dyn MbcController>>,        // 0x0000 - 0x7FFF
    pub ppu: PPU,                               // 0x8000 - 0x9FFF
    eram: [u8; 0x2000],                         // 0xA000 - 0xBFFF
    wram: [u8; 0x2000],                         // 0xC000 - 0xDFFF (0xE000 - 0xFDFF)
    io_regs: [u8; 0x80],                        // 0xFF00 - 0xFF7F
    hram: [u8; 0x80],                           // 0xFF80 - 0xFFFE
    i_enable: u8,                               // 0xFFFF
    boot_rom: [u8; 0x100],
    enable_boot_rom: bool,

    internat_div_counter: u16,
    last_and_result: bool,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            rom: None,
            ppu: PPU::new(),
            eram: [0x00; 0x2000],
            wram: [0x00; 0x2000],
            io_regs: [0x00; 0x80],
            hram: [0x00; 0x80],
            i_enable: 0x00,
            boot_rom: [
                0x31, 0xFE, 0xFF, 0xAF, 0x21, 0xFF, 0x9F, 0x32, 0xCB, 0x7C, 0x20, 0xFB, 0x21, 0x26, 0xFF, 0x0E,
                0x11, 0x3E, 0x80, 0x32, 0xE2, 0x0C, 0x3E, 0xF3, 0xE2, 0x32, 0x3E, 0x77, 0x77, 0x3E, 0xFC, 0xE0,
                0x47, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1A, 0xCD, 0x95, 0x00, 0xCD, 0x96, 0x00, 0x13, 0x7B,
                0xFE, 0x34, 0x20, 0xF3, 0x11, 0xD8, 0x00, 0x06, 0x08, 0x1A, 0x13, 0x22, 0x23, 0x05, 0x20, 0xF9,
                0x3E, 0x19, 0xEA, 0x10, 0x99, 0x21, 0x2F, 0x99, 0x0E, 0x0C, 0x3D, 0x28, 0x08, 0x32, 0x0D, 0x20,
                0xF9, 0x2E, 0x0F, 0x18, 0xF3, 0x67, 0x3E, 0x64, 0x57, 0xE0, 0x42, 0x3E, 0x91, 0xE0, 0x40, 0x04,
                0x1E, 0x02, 0x0E, 0x0C, 0xF0, 0x44, 0xFE, 0x90, 0x20, 0xFA, 0x0D, 0x20, 0xF7, 0x1D, 0x20, 0xF2,
                0x0E, 0x13, 0x24, 0x7C, 0x1E, 0x83, 0xFE, 0x62, 0x28, 0x06, 0x1E, 0xC1, 0xFE, 0x64, 0x20, 0x06,
                0x7B, 0xE2, 0x0C, 0x3E, 0x87, 0xE2, 0xF0, 0x42, 0x90, 0xE0, 0x42, 0x15, 0x20, 0xD2, 0x05, 0x20,
                0x4F, 0x16, 0x20, 0x18, 0xCB, 0x4F, 0x06, 0x04, 0xC5, 0xCB, 0x11, 0x17, 0xC1, 0xCB, 0x11, 0x17,
                0x05, 0x20, 0xF5, 0x22, 0x23, 0x22, 0x23, 0xC9, 0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B,
                0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E,
                0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC,
                0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E, 0x3C, 0x42, 0xB9, 0xA5, 0xB9, 0xA5, 0x42, 0x3C,
                0x21, 0x04, 0x01, 0x11, 0xA8, 0x00, 0x1A, 0x13, 0xBE, 0x20, 0xFE, 0x23, 0x7D, 0xFE, 0x34, 0x20,
                0xF5, 0x06, 0x19, 0x78, 0x86, 0x23, 0x05, 0x20, 0xFB, 0x86, 0x20, 0xFE, 0x3E, 0x01, 0xE0, 0x50,
            ],
            enable_boot_rom: true,

            internat_div_counter: 0,
            last_and_result: false,
        }
    }

    pub fn set_rom(&mut self, rom: Option<Box<dyn MbcController>>) {
        self.rom = rom
    }

    pub fn read(&self, dir: usize) -> u8 {
        match dir {
            0x0000 ..= 0x7FFF => {
                if self.read(0xFF50) == 0 && dir < 0x100 && self.enable_boot_rom {
                    self.boot_rom[dir]
                } else {
                    self.rom.as_ref().unwrap().read(dir)
                }
            },
            0x8000 ..= 0x9FFF => self.ppu.read_vram(dir),
            0xA000 ..= 0xBFFF => self.eram[dir - 0xA000],
            0xC000 ..= 0xDFFF => self.wram[dir - 0xC000],
            0xE000 ..= 0xFDFF => self.wram[dir - 0xE000],
            0xFE00 ..= 0xFE9F => self.ppu.read_oam(dir),
            0xFEA0 ..= 0xFEFF => 0x00,
            0xFF00 ..= 0xFF7F => {
                match dir {
                    0xFF40 ..= 0xFF4B => self.ppu.read_reg(dir),
                    _ => self.io_regs[dir - 0xFF00],
                }
            }
            0xFF80 ..= 0xFFFE => self.hram[dir - 0xFF80],
            0xFFFF => self.i_enable,
            _ => unreachable!()
        }
    }

    pub fn write(&mut self, dir: usize, val: u8) {
        match dir {
            0x0000 ..= 0x7FFF => self.rom.as_mut().unwrap().write(dir, val),
            0x8000 ..= 0x9FFF => self.ppu.write_vram(dir, val),
            0xA000 ..= 0xBFFF => self.eram[dir - 0xA000] = val,
            0xC000 ..= 0xDFFF => self.wram[dir - 0xC000] = val,
            0xE000 ..= 0xFDFF => self.wram[dir - 0xE000] = val,
            0xFE00 ..= 0xFE9F => self.ppu.write_oam(dir, val),
            0xFEA0 ..= 0xFEFF => {},
            0xFF00 ..= 0xFF7F => {
                match dir {
                    0xFF00 => {
                        let joyp = self.io_regs[0] & 0x0F;
                        self.io_regs[0] = joyp | (val & 0xF0);
                    }
                    0xFF04 => self.io_regs[dir - 0xFF00] = 0,
                    0xFF40 ..= 0xFF4B => self.ppu.write_reg(dir, val),
                    _ => self.io_regs[dir - 0xFF00] = val,
                }
            }
            0xFF80 ..= 0xFFFE => self.hram[dir - 0xFF80] = val,
            0xFFFF => self.i_enable = val,
            _ => unreachable!()
        }
    }

    pub fn set_joyp(&mut self, val: u8) {
        self.io_regs[0] = val;
    }

    pub fn set_enable_boot_rom(mut self, enable_boot_rom: bool) -> Bus {
        self.enable_boot_rom = enable_boot_rom;
        self
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

    pub fn reset_joyp(&mut self) {
        self.io_regs[0] = 0xFF;
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

    pub fn cycle(&mut self, cycles: u8) {
        let ppu_ints = self.ppu.cycle(cycles);
        match ppu_ints {
            // No interrupts de la PPU
            (false, false) => {},
            //VBLANK y STAT
            (true, true) => {self.set_int(Interrupts::VBlank); self.set_int(Interrupts::LcdStat);},
            //VBLANK
            (true, false) => self.set_int(Interrupts::VBlank),
            //STAT
            (false, true) => self.set_int(Interrupts::LcdStat),
        }

        self.update_tima(cycles);
    }

    pub fn set_int(&mut self, int: Interrupts) {
        let mut int_f = self.read(0xFF0F);
        match int {
            Interrupts::VBlank => int_f |= 0b00000001,
            Interrupts::LcdStat => int_f |= 0b00000010,
            Interrupts::Timer => int_f |= 0b00000100,
            Interrupts::Serial => int_f |= 0b00001000,
            Interrupts::Joypad => int_f |= 0b00010000,
        }

        self.write(0xFF0F, int_f);
    }

    fn update_tima(&mut self, cycles: u8) {
        let mut to_cycle = cycles;
        let bit_pos = match self.read(0xFF07) & 0b00000011 {
            0b00 => 9,
            0b01 => 3,
            0b10 => 5,
            0b11 => 7,
            _ => unreachable!()
        };
        let tac = self.read(0xFF07);

        while to_cycle > 0 {
            let bit_div = self.internat_div_counter & (1 << bit_pos) != 0;
            let timer_enable_bit = tac & 0b100 != 0;

            let and_result = bit_div & timer_enable_bit;

            if !and_result && self.last_and_result {
                let int = self.increase_tima();

                if int {
                    let tma = self.read(0xFF06);
                    self.write(0xFF05, tma);
                    self.set_int(Interrupts::Timer);
                }
            };

            self.internat_div_counter = self.internat_div_counter.wrapping_add(1);
            self.last_and_result = and_result;

            to_cycle -= 1;
        }
    }
}