use std::collections::VecDeque;

use super::mmu::MMU;

pub struct PPU {
    fifo_backgroung: VecDeque<u8>,
    fifo_OAM: VecDeque<u8>,
    current_x: u8,
    current_y: u8,
}

impl PPU { 
    pub fn new() -> Self {
        PPU {
            fifo_backgroung: VecDeque::with_capacity(16),
            fifo_OAM: VecDeque::with_capacity(16),
            current_x: 0,
            current_y: 0,
        }
    }

    pub fn cycle(&mut self, mmu: &mut MMU) {
        
    }

    fn get_tile(&mut self, mmu: &mut MMU) -> u16 {
        let lcdc = mmu.read(0xFF40);
    }
}