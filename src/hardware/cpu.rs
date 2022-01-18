use crate::hardware::mmu::MMU;
use crate::hardware::inst_set::*;

pub struct CPU {
    pub registers: [u8; 8],
    pub pc: u16,
    pub sp: u16,
    pub mem: MMU,
    pub cycles: u32,

    inst_set: [fn(&mut CPU); 0x100], // TODO Cambiar tamaño segun vaya haciendo para no errores
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
//              0x_0            0x_1            0x_2            0x_3            0x_4            0x_5            0x_6            0x_7            0x_8            0x_9            0x_A            0x_B            0x_C            0x_D            0x_E            0x_F        
/* 0x0_ */      nop,            ld_bc_u16,      ld_bcind_a,     error,          inc_b,          dec_b,          ld_b_u8,        error,          ld_u16ind_sp,   error,          ld_a_bcind,     error,          inc_c,          dec_c,          ld_c_u8,        error,
/* 0x1_ */      error,          ld_de_u16,      ld_deind_a,     error,          inc_d,          dec_d,          ld_d_u8,        error,          error,          error,          ld_a_deind,     error,          inc_e,          dec_e,          ld_e_u8,        error,
/* 0x2_ */      error,          ld_hl_u16,      ld_hlindinc_a,  error,          inc_h,          dec_h,          ld_h_u8,        daa,            error,          error,          ld_a_hlindinc,  error,          inc_l,          dec_l,          ld_l_u8,        cpl,
/* 0x3_ */      error,          ld_sp_u16,      ld_hlinddec_a,  error,          inc_hlind,      dec_hlind,      ld_hlind_u8,    scf,            error,          error,          ld_a_hlinddec,  error,          inc_a,          dec_a,          ld_a_u8,        ccf,
/* 0x4_ */      ld_b_b,         ld_b_c,         ld_b_d,         ld_b_e,         ld_b_h,         ld_b_l,         ld_b_hlind,     ld_b_a,         ld_c_b,         ld_c_c,         ld_c_d,         ld_c_e,         ld_c_h,         ld_c_l,         ld_c_hlind,     ld_c_a,
/* 0x5_ */      ld_d_b,         ld_d_c,         ld_d_d,         ld_d_e,         ld_d_h,         ld_d_l,         ld_d_hlind,     ld_d_a,         ld_e_b,         ld_e_c,         ld_e_d,         ld_e_e,         ld_e_h,         ld_e_l,         ld_e_hlind,     ld_e_a,
/* 0x6_ */      ld_h_b,         ld_h_c,         ld_h_d,         ld_h_e,         ld_h_h,         ld_h_l,         ld_h_hlind,     ld_h_a,         ld_l_b,         ld_l_c,         ld_l_d,         ld_l_e,         ld_l_h,         ld_l_l,         ld_l_hlind,     ld_l_a,
/* 0x7_ */      ld_hlind_b,     ld_hlind_c,     ld_hlind_d,     ld_hlind_e,     ld_hlind_h,     ld_hlind_l,     error,          ld_hlind_a,     ld_a_b,         ld_a_c,         ld_a_d,         ld_a_e,         ld_a_h,         ld_a_l,         ld_a_hlind,     ld_a_a,
/* 0x8_ */      add_a_b,        add_a_c,        add_a_d,        add_a_e,        add_a_h,        add_a_l,        add_a_hlind,    add_a_a,        adc_a_b,        adc_a_c,        adc_a_d,        adc_a_e,        adc_a_h,        adc_a_l,        adc_a_hlind,    adc_a_a,
/* 0x9_ */      error,          error,          error,          error,          error,          error,          error,          error,          error,          error,          error,          error,          error,          error,          error,          error,
/* 0xA_ */      error,          error,          error,          error,          error,          error,          error,          error,          error,          error,          error,          error,          error,          error,          error,          error,
/* 0xB_ */      error,          error,          error,          error,          error,          error,          error,          error,          error,          error,          error,          error,          error,          error,          error,          error,
/* 0xC_ */      error,          pop_bc,         error,          error,          error,          push_bc,        error,          error,          error,          error,          error,          error,          error,          error,          error,          error,
/* 0xD_ */      error,          pop_de,         error,          error,          error,          push_de,        error,          error,          error,          error,          error,          error,          error,          error,          error,          error,
/* 0xE_ */      ld_ff00u8_a,    pop_hl,         ld_ff00c_a,     error,          error,          push_hl,        error,          error,          error,          error,          ld_u16ind_a,    error,          error,          error,          error,          error,
/* 0xF_ */      ld_a_ff00u8,    pop_af,         ld_a_ff00c,     error,          error,          push_af,        error,          error,          error,          ld_sp_hl,       ld_a_u16ind,    error,          error,          error,          error,          error,
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