use crate::hardware::bus::Bus;
use crate::hardware::inst_set::*;

// Registros
const A: usize = 0;
const F: usize = 1;
const B: usize = 2;
const C: usize = 3;
const D: usize = 4;
const E: usize = 5;
const H: usize = 6;
const L: usize = 7;

// Interrupciones

pub enum Interrupts {
    VBlank,
    LcdStat,
    Timer,
    Serial,
    Joypad,
}

pub struct CPU {
    pub registers: [u8; 8],
    pub pc: u16,
    pub sp: u16,
    pub bus: Bus,
    pub cycles: u64,
    pub stop: bool,
    pub halt: bool,

    // Variables utilizada para activar o desactivar interrupciones depues de EI/DI
    pub cycles_di: u8,
    pub cycles_ei: u8,
    pub ime: bool,

    pub op: u8,

    div_timer: u32,

    tima_timer: u32,

    inst_set: [fn(&mut CPU); 0x100],
    cb_set: [fn(&mut CPU); 0x100],
}

impl CPU {
    pub fn new(bus: Bus) -> Self {
        CPU {
            registers: [0x00; 8],
            pc: 0x0000,
            sp: 0x0000,
            bus: bus,
            cycles: 0,
            stop: false,
            halt: false,

            cycles_di: 0,
            cycles_ei: 0,
            ime: false,

            op: 0,
            div_timer: 0,

            tima_timer: 0,

            inst_set: [
//              0x_0            0x_1            0x_2            0x_3            0x_4            0x_5            0x_6            0x_7            0x_8            0x_9            0x_A            0x_B            0x_C            0x_D            0x_E            0x_F        
/* 0x0_ */      nop,            ld_bc_u16,      ld_bcind_a,     inc_bc,         inc_b,          dec_b,          ld_b_u8,        rlca,           ld_u16ind_sp,   add_hl_bc,      ld_a_bcind,     dec_bc,         inc_c,          dec_c,          ld_c_u8,        rrca,
/* 0x1_ */      stop,           ld_de_u16,      ld_deind_a,     inc_de,         inc_d,          dec_d,          ld_d_u8,        rla,            jr_i8,          add_hl_de,      ld_a_deind,     dec_de,         inc_e,          dec_e,          ld_e_u8,        rra,
/* 0x2_ */      jr_nz_i8,       ld_hl_u16,      ld_hlindinc_a,  inc_hl,         inc_h,          dec_h,          ld_h_u8,        daa,            jr_z_i8,        add_hl_hl,      ld_a_hlindinc,  dec_hl,         inc_l,          dec_l,          ld_l_u8,        cpl,
/* 0x3_ */      jr_nc_i8,       ld_sp_u16,      ld_hlinddec_a,  inc_sp,         inc_hlind,      dec_hlind,      ld_hlind_u8,    scf,            jr_c_i8,        add_hl_sp,      ld_a_hlinddec,  dec_sp,         inc_a,          dec_a,          ld_a_u8,        ccf,
/* 0x4_ */      ld_b_b,         ld_b_c,         ld_b_d,         ld_b_e,         ld_b_h,         ld_b_l,         ld_b_hlind,     ld_b_a,         ld_c_b,         ld_c_c,         ld_c_d,         ld_c_e,         ld_c_h,         ld_c_l,         ld_c_hlind,     ld_c_a,
/* 0x5_ */      ld_d_b,         ld_d_c,         ld_d_d,         ld_d_e,         ld_d_h,         ld_d_l,         ld_d_hlind,     ld_d_a,         ld_e_b,         ld_e_c,         ld_e_d,         ld_e_e,         ld_e_h,         ld_e_l,         ld_e_hlind,     ld_e_a,
/* 0x6_ */      ld_h_b,         ld_h_c,         ld_h_d,         ld_h_e,         ld_h_h,         ld_h_l,         ld_h_hlind,     ld_h_a,         ld_l_b,         ld_l_c,         ld_l_d,         ld_l_e,         ld_l_h,         ld_l_l,         ld_l_hlind,     ld_l_a,
/* 0x7_ */      ld_hlind_b,     ld_hlind_c,     ld_hlind_d,     ld_hlind_e,     ld_hlind_h,     ld_hlind_l,     halt,           ld_hlind_a,     ld_a_b,         ld_a_c,         ld_a_d,         ld_a_e,         ld_a_h,         ld_a_l,         ld_a_hlind,     ld_a_a,
/* 0x8_ */      add_a_b,        add_a_c,        add_a_d,        add_a_e,        add_a_h,        add_a_l,        add_a_hlind,    add_a_a,        adc_a_b,        adc_a_c,        adc_a_d,        adc_a_e,        adc_a_h,        adc_a_l,        adc_a_hlind,    adc_a_a,
/* 0x9_ */      sub_a_b,        sub_a_c,        sub_a_d,        sub_a_e,        sub_a_h,        sub_a_l,        sub_a_hlind,    sub_a_a,        sbc_a_b,        sbc_a_c,        sbc_a_d,        sbc_a_e,        sbc_a_h,        sbc_a_l,        sbc_a_hlind,    sbc_a_a,
/* 0xA_ */      and_a_b,        and_a_c,        and_a_d,        and_a_e,        and_a_h,        and_a_l,        and_a_hlind,    and_a_a,        xor_a_b,        xor_a_c,        xor_a_d,        xor_a_e,        xor_a_h,        xor_a_l,        xor_a_hlind,    xor_a_a,
/* 0xB_ */      or_a_b,         or_a_c,         or_a_d,         or_a_e,         or_a_h,         or_a_l,         or_a_hlind,     or_a_a,         cp_a_b,         cp_a_c,         cp_a_d,         cp_a_e,         cp_a_h,         cp_a_l,         cp_a_hlind,     cp_a_a,
/* 0xC_ */      ret_nz,         pop_bc,         jp_nz_u16,      jp_u16,         call_nz_u16,    push_bc,        add_a_u8,       rst_0x00,       ret_z,          ret,            jp_z_u16,       cb,             call_z_u16,     call_u16,       adc_a_u8,       rst_0x08,
/* 0xD_ */      ret_nc,         pop_de,         jp_nc_u16,      error,          call_nc_u16,    push_de,        sub_a_u8,       rst_0x10,       ret_c,          reti,           jp_c_u16,       error,          call_c_u16,     error,          sbc_a_u8,       rst_0x18,
/* 0xE_ */      ld_ff00u8_a,    pop_hl,         ld_ff00c_a,     error,          error,          push_hl,        and_a_u8,       rst_0x20,       add_sp_i8,      jp_hl,          ld_u16ind_a,    error,          error,          error,          xor_a_u8,       rst_0x28,
/* 0xF_ */      ld_a_ff00u8,    pop_af,         ld_a_ff00c,     di,             error,          push_af,        or_a_u8,        rst_0x30,       ld_hl_sp_i8,    ld_sp_hl,       ld_a_u16ind,    ei,             error,          error,          cp_a_u8,        rst_0x38,
            ],

            cb_set: [
//              0x_0            0x_1            0x_2            0x_3            0x_4            0x_5            0x_6            0x_7            0x_8            0x_9            0x_A            0x_B            0x_C            0x_D            0x_E            0x_F        
/* 0x0_ */      rlc_b,          rlc_c,          rlc_d,          rlc_e,          rlc_h,          rlc_l,          rlc_hlind,      rlc_a,          rrc_b,          rrc_c,          rrc_d,          rrc_e,          rrc_h,          rrc_l,          rrc_hlind,      rrc_a,
/* 0x1_ */      rl_b,           rl_c,           rl_d,           rl_e,           rl_h,           rl_l,           rl_hlind,       rl_a,           rr_b,           rr_c,           rr_d,           rr_e,           rr_h,           rr_l,           rr_hlind,       rr_a,
/* 0x2_ */      sla_b,          sla_c,          sla_d,          sla_e,          sla_h,          sla_l,          sla_hlind,      sla_a,          sra_b,          sra_c,          sra_d,          sra_e,          sra_h,          sra_l,          sra_hlind,      sra_a, 
/* 0x3_ */      swap_b,         swap_c,         swap_d,         swap_e,         swap_h,         swap_l,         swap_hlind,     swap_a,         srl_b,          srl_c,          srl_d,          srl_e,          srl_h,          srl_l,          srl_hlind,      srl_a,
/* 0x4_ */      bit_0_b,        bit_0_c,        bit_0_d,        bit_0_e,        bit_0_h,        bit_0_l,        bit_0_hlind,    bit_0_a,        bit_1_b,        bit_1_c,        bit_1_d,        bit_1_e,        bit_1_h,        bit_1_l,        bit_1_hlind,    bit_1_a,
/* 0x5_ */      bit_2_b,        bit_2_c,        bit_2_d,        bit_2_e,        bit_2_h,        bit_2_l,        bit_2_hlind,    bit_2_a,        bit_3_b,        bit_3_c,        bit_3_d,        bit_3_e,        bit_3_h,        bit_3_l,        bit_3_hlind,    bit_3_a,
/* 0x6_ */      bit_4_b,        bit_4_c,        bit_4_d,        bit_4_e,        bit_4_h,        bit_4_l,        bit_4_hlind,    bit_4_a,        bit_5_b,        bit_5_c,        bit_5_d,        bit_5_e,        bit_5_h,        bit_5_l,        bit_5_hlind,    bit_5_a,
/* 0x7_ */      bit_6_b,        bit_6_c,        bit_6_d,        bit_6_e,        bit_6_h,        bit_6_l,        bit_6_hlind,    bit_6_a,        bit_7_b,        bit_7_c,        bit_7_d,        bit_7_e,        bit_7_h,        bit_7_l,        bit_7_hlind,    bit_7_a,
/* 0x8_ */      res_0_b,        res_0_c,        res_0_d,        res_0_e,        res_0_h,        res_0_l,        res_0_hlind,    res_0_a,        res_1_b,        res_1_c,        res_1_d,        res_1_e,        res_1_h,        res_1_l,        res_1_hlind,    res_1_a,
/* 0x9_ */      res_2_b,        res_2_c,        res_2_d,        res_2_e,        res_2_h,        res_2_l,        res_2_hlind,    res_2_a,        res_3_b,        res_3_c,        res_3_d,        res_3_e,        res_3_h,        res_3_l,        res_3_hlind,    res_3_a,
/* 0xA_ */      res_4_b,        res_4_c,        res_4_d,        res_4_e,        res_4_h,        res_4_l,        res_4_hlind,    res_4_a,        res_5_b,        res_5_c,        res_5_d,        res_5_e,        res_5_h,        res_5_l,        res_5_hlind,    res_5_a,
/* 0xB_ */      res_6_b,        res_6_c,        res_6_d,        res_6_e,        res_6_h,        res_6_l,        res_6_hlind,    res_6_a,        res_7_b,        res_7_c,        res_7_d,        res_7_e,        res_7_h,        res_7_l,        res_7_hlind,    res_7_a,
/* 0xC_ */      set_0_b,        set_0_c,        set_0_d,        set_0_e,        set_0_h,        set_0_l,        set_0_hlind,    set_0_a,        set_1_b,        set_1_c,        set_1_d,        set_1_e,        set_1_h,        set_1_l,        set_1_hlind,    set_1_a,
/* 0xD_ */      set_2_b,        set_2_c,        set_2_d,        set_2_e,        set_2_h,        set_2_l,        set_2_hlind,    set_2_a,        set_3_b,        set_3_c,        set_3_d,        set_3_e,        set_3_h,        set_3_l,        set_3_hlind,    set_3_a,
/* 0xE_ */      set_4_b,        set_4_c,        set_4_d,        set_4_e,        set_4_h,        set_4_l,        set_4_hlind,    set_4_a,        set_5_b,        set_5_c,        set_5_d,        set_5_e,        set_5_h,        set_5_l,        set_5_hlind,    set_5_a,
/* 0xF_ */      set_6_b,        set_6_c,        set_6_d,        set_6_e,        set_6_h,        set_6_l,        set_6_hlind,    set_6_a,        set_7_b,        set_7_c,        set_7_d,        set_7_e,        set_7_h,        set_7_l,        set_7_hlind,    set_7_a,      
            ]
        }
    }

    pub fn reset(&mut self, enable_boot_rom: bool) {
        if !enable_boot_rom {
            self.registers[A] = 0x01;
            self.registers[F] = 0x80;
            self.registers[B] = 0x00;
            self.registers[C] = 0x13;
            self.registers[D] = 0x00;
            self.registers[E] = 0xD8;
            self.registers[H] = 0x01;
            self.registers[L] = 0x4D;
            self.sp = 0xFFFE;
            self.pc = 0x100;
            self.bus.reset();
        }
    }

    pub fn cycle(&mut self) -> u64{
        self.update_ime();

        let cycles_temp = self.cycles;

        self.interrupt();

        if self.halt {
            nop(self);
        } else {
            let op = self.fetch();
            self.decode_execute(op);
        }

        self.update_timers();

        return self.cycles - cycles_temp;
    }

    pub fn fetch(&mut self) -> u8 {
        let val = self.bus.read(self.pc as usize);
        self.pc = self.pc.wrapping_add(1);
        val
    }

    pub fn decode_cb(&mut self, op: u8) {
        self.cb_set[op as usize](self);
    }

    fn decode_execute(&mut self, op: u8) {
        self.op = op;
        self.inst_set[op as usize](self);
    }

    fn update_ime(&mut self) {
        self.cycles_di = match self.cycles_di {
            2 => 1,
            1 => {self.ime = false; 0},
            _ => 0,
        };
        self.cycles_ei = match self.cycles_ei {
            2 => 1,
            1 => {self.ime = true; 0},
            _ => 0,
        };
    }

    fn get_ie(&mut self) -> u8 {
        self.cycles += 4;
        self.bus.read(0xFFFF)
    }    
    
    fn get_if(&mut self) -> u8 {
        self.cycles += 4;
        self.bus.read(0xFF0F)
    }

    pub fn set_if(&mut self, int: usize, cond: bool) {
        let flag: u8 = 1 << int;
        if cond {
            let if_reg = self.get_if();
            self.bus.write(0xFF0F, if_reg | flag);
        } else {
            let if_reg = self.get_if();
            self.bus.write(0xFF0F, if_reg & !flag);
        }
    }

    fn interrupt(&mut self) {
        if !self.ime {
            if self.get_ie() & self.get_if() != 0 {
                self.halt = false;
            }
            return;
        }

        self.halt = false;

        for i in 0..5 {
            let int_f = self.get_if() >> i;
            let int_e = self.get_ie() >> i;

            let int = (int_f & 0x01) & (int_e & 0x01);
            // Si hay interrupcion
            if int == 1 {
                self.ime = false;
                self.set_if(i, false);
                self.interrupt_handler(i);
                return;
            }
        }
    }

    fn interrupt_handler(&mut self, int: usize) {
        let int_offset: [u16; 5] = [0x0, 0x8, 0x10, 0x18, 0x20];

        // 2 NOPS
        nop(self);
        nop(self);

        // Llevar PC a la pila
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write(self.sp as usize, (self.pc / 0x100) as u8);
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write(self.sp as usize, self.pc as u8);

        // Tratar interrupcion
        self.pc = 0x40 + int_offset[int];
    }

    fn update_timers(&mut self) {
        if (self.cycles / 256) as u32 > self.div_timer {
            self.div_timer = (self.cycles / 256) as u32;
            self.bus.increase_div();
        }

        let tac = self.bus.read(0xFF07);
        let timer_enable = (tac & 0b00000100) != 0;

        // TODO si al final hago la CGB, hay que dividir esto entre la velocidad (1 o 2, dependiendo de la seleccionada)
        let tima_freq_divider = match tac & 0b00000011 {
            0x00 => 1024,
            0x01 => 16,
            0x10 => 64,
            0x11 => 256,
            _ => 0
        };

        if (self.cycles / tima_freq_divider) as u32 > self.tima_timer && timer_enable {
            self.tima_timer = (self.cycles / tima_freq_divider) as u32;
            let tima_int = self.bus.increase_tima();

            if tima_int {
                let tma = self.bus.read(0xFF06);
                self.bus.write(0xFF05, tma);
                self.bus.set_int(Interrupts::Timer);
            }
        }
    }
}