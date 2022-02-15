pub trait MbcController {
    fn new<>(rom: &[u8]) -> Self where Self: Sized;
    fn read(&self, dir: usize) -> u8;
    fn write(&mut self, dir: usize, val: u8);
}

pub struct MBC0 {
    rom: Vec<u8>,
}

pub struct MBC1 {
    rom: Vec<u8>,
    rom_bank_number: u8,
    ram_enable: bool,
    banking_mode: u8,
}

impl MbcController for MBC0 {
    fn new<>(rom: &[u8]) -> Self {
        MBC0 {
            rom: rom.to_vec()
        }
    }

    fn read(&self, dir: usize) -> u8 {
        self.rom[dir]
    }

    // TODO poner si hay ram optativa
    fn write(&mut self, _dir: usize, _val: u8) {}
}

impl MbcController for MBC1 {
    fn new<>(rom: &[u8]) -> Self {
        MBC1 {
            rom: rom.to_vec(),
            rom_bank_number: 1,
            ram_enable: false,
            banking_mode: 0,
        }
    }

    fn read(&self, dir: usize) -> u8 {
        if self.banking_mode == 0 {
            if dir <= 0x3FFF {
                self.rom[dir]
            } else {
                let bank = self.rom_bank_number;
                self.rom[(dir - 0x4000) + 0x4000 * bank as usize]
            }
        } else {
            // TODO BANK Mode 1
            0
        }
    }

    fn write(&mut self, dir: usize, val: u8) {
        if dir < 0x2000 {
            if val == 0x0A { self.ram_enable = true } else { self.ram_enable = false };
        } else if dir < 0x4000 {
            // TODO Añadir caso de que el cartucho sea grande
            self.rom_bank_number = val & 0b00011111;

            // TODO self.mask_rom_number_to_size();

            if self.rom_bank_number == 0 {
                self.rom_bank_number = 1;
            }
        } else if dir < 0x6000 {
            
        } else {
            self.banking_mode = val;
        }
    }
}
