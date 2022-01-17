use crate::hardware::mmu::MMU;
use crate::hardware::inst_set;

pub struct CPU {
    pub registers: [u8; 8],
    pub pc: u16,
    pub sp: u16,
    pub mem: MMU,
    pub cycles: u32,

    inst_set: [fn(&mut CPU); 0x2], // TODO Cambiar tamaño segun vaya haciendo para no errores
    // cb_set: [fn(); 0x100],
    cb_set: [fn(&mut CPU); 1] // TODO Temporal para que no me de errores
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: [0x00; 8],
            pc: 0x0000,
            sp: 0x0000,
            mem: MMU::new(),
            cycles: 0,

            inst_set: [
                inst_set::nop, inst_set::ld_bc_u16,
            ],

            cb_set: [| cpu | (println!("CB Ins"))]
        }
    }

    pub fn cycle(&mut self) {
        //let op = self.fetch();
        
        self.decode_execute(0);
    }

    pub fn fetch(&mut self) -> u8 {
        let val = self.mem.read(self.pc as usize);
        self.pc = self.pc.wrapping_add(1);
        val
    }

    fn decode_execute(&mut self, op: u8) {
        if op != 0xCB {
            self.inst_set[op as usize](self);
            return;
        }

        let cb_op = self.fetch();
        self.cb_set[cb_op as usize](self);
    }
}