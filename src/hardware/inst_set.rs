use crate::hardware::cpu::CPU;

// FLAGS
const Z_FLAG: u8 = 0b10000000;
const N_FLAG: u8 = 0b01000000;
const H_FLAG: u8 = 0b00100000;
const C_FLAG: u8 = 0b00010000;

// Registros
const A: usize = 0;
const F: usize = 1;
const B: usize = 2;
const C: usize = 3;
const D: usize = 4;
const E: usize = 5;
const H: usize = 6;
const L: usize = 7;

// Pares de registros
const AF: usize = 1;
const BC: usize = 3;
const DE: usize = 5;
const HL: usize = 7;

// Instruccion "vacia"
pub fn error(cpu: &mut CPU) {
    println!("Instrucción no válida: {:02X}.", cpu.op);
}

fn set_flags(cpu: &mut CPU, flag: u8, cond: bool) {
    if cond {
        cpu.registers[F] |= flag;
    } else {
        cpu.registers[F] &= !flag;
    }
}

fn get_zero(cpu: &mut CPU) -> u8 {
    (cpu.registers[F] & Z_FLAG) >> 7
}

fn get_negative(cpu: &mut CPU) -> u8 {
    (cpu.registers[F] & N_FLAG) >> 6
}

fn get_half_carry(cpu: &mut CPU) -> u8 {
    (cpu.registers[F] & H_FLAG) >> 5
}

fn get_carry(cpu: &mut CPU) -> u8 {
    (cpu.registers[F] & C_FLAG) >> 4
}

// u8 LOAD/STORE/MOVE
fn reg_to_reg(cpu: &mut CPU, reg_dst: usize, reg_src: usize) {
    cpu.registers[reg_dst] = cpu.registers[reg_src];
    cpu.cycles += 4;
}

fn hlind_to_reg(cpu: &mut CPU, reg_dst: usize) {
    cpu.registers[reg_dst] = cpu.mem.read((cpu.registers[L] as u16 + cpu.registers[H] as u16 * 0x100) as usize);
    cpu.cycles += 8;
}

fn reg_to_regxind(cpu: &mut CPU, regx_dst: usize, reg_src: usize) {
    cpu.mem.write((cpu.registers[regx_dst] as u16 + cpu.registers[regx_dst - 1] as u16 * 0x100) as usize, cpu.registers[reg_src]);
    cpu.cycles += 8;
}

fn u8_to_reg(cpu: &mut CPU, reg_src: usize) {
    cpu.registers[reg_src] = cpu.fetch();
    cpu.cycles += 8;
}

fn u8_to_hlind(cpu: &mut CPU, val: u8) {
    cpu.mem.write((cpu.registers[L] as u16 + cpu.registers[H] as u16 * 0x100) as usize, val);
    cpu.cycles += 8;
}

fn regxind_to_reg(cpu: &mut CPU, reg_dst: usize, regx_src: usize) {
    cpu.registers[reg_dst] = cpu.mem.read((cpu.registers[regx_src] as u16 + cpu.registers[regx_src - 1] as u16 * 0x100) as usize);
    cpu.cycles += 8;
}


pub fn ld_bcind_a(cpu: &mut CPU) {
    reg_to_regxind(cpu, BC, A);
}

pub fn ld_deind_a(cpu: &mut CPU) {
    reg_to_regxind(cpu, DE, A);
}

pub fn ld_hlindinc_a(cpu: &mut CPU) {
    cpu.mem.write((cpu.registers[L] as u16 + cpu.registers[H] as u16 * 0x100) as usize, cpu.registers[A]);

    let x = cpu.registers[L].overflowing_add(1);
    cpu.registers[L] = x.0;
    if x.1 {
        cpu.registers[H] = cpu.registers[H].wrapping_add(1);
    }

    cpu.cycles += 8;
}

pub fn ld_hlinddec_a(cpu: &mut CPU) {
    cpu.mem.write((cpu.registers[L] as u16 + cpu.registers[H] as u16 * 0x100) as usize, cpu.registers[A]);

    let x = cpu.registers[L].overflowing_sub(1);
    cpu.registers[L] = x.0;
    if x.1 {
        cpu.registers[H] = cpu.registers[H].wrapping_sub(1);
    }

    cpu.cycles += 8;
}

pub fn ld_b_u8(cpu: &mut CPU) {
    u8_to_reg(cpu, B);
}

pub fn ld_d_u8(cpu: &mut CPU) {
    u8_to_reg(cpu, D);
}

pub fn ld_h_u8(cpu: &mut CPU) {
    u8_to_reg(cpu, H);
}

pub fn ld_c_u8(cpu: &mut CPU) {
    u8_to_reg(cpu, C);
}

pub fn ld_e_u8(cpu: &mut CPU) {
    u8_to_reg(cpu, E);
}

pub fn ld_l_u8(cpu: &mut CPU) {
    u8_to_reg(cpu, L);
}

pub fn ld_hlind_u8(cpu: &mut CPU) {
    let val = cpu.fetch();
    u8_to_hlind(cpu, val);
}

pub fn ld_a_u8(cpu: &mut CPU) {
    u8_to_reg(cpu, A);
}

pub fn ld_a_bcind(cpu: &mut CPU) {
    regxind_to_reg(cpu, A, BC);
}

pub fn ld_a_deind(cpu: &mut CPU) {
    regxind_to_reg(cpu, A, DE);
}

pub fn ld_a_hlindinc(cpu: &mut CPU) {
    cpu.registers[A] = cpu.mem.read((cpu.registers[L] as u16 + cpu.registers[H] as u16 * 0x100) as usize);

    let x = cpu.registers[L].overflowing_add(1);
    cpu.registers[L] = x.0;
    if x.1 {
        cpu.registers[H] = cpu.registers[H].wrapping_add(1);
    }

    cpu.cycles += 8;
}

pub fn ld_a_hlinddec(cpu: &mut CPU) {
    cpu.registers[A] = cpu.mem.read((cpu.registers[L] as u16 + cpu.registers[H] as u16 * 0x100) as usize);

    let x = cpu.registers[L].overflowing_sub(1);
    cpu.registers[L] = x.0;
    if x.1 {
        cpu.registers[H] = cpu.registers[H].wrapping_sub(1);
    }

    cpu.cycles += 8;
}


pub fn ld_b_b(cpu: &mut CPU) {
    reg_to_reg(cpu, B, B);
}

pub fn ld_b_c(cpu: &mut CPU) {
    reg_to_reg(cpu, B, C);
}

pub fn ld_b_d(cpu: &mut CPU) {
    reg_to_reg(cpu, B, D);
}

pub fn ld_b_e(cpu: &mut CPU) {
    reg_to_reg(cpu, B, E);
}

pub fn ld_b_h(cpu: &mut CPU) {
    reg_to_reg(cpu, B, H);
}

pub fn ld_b_l(cpu: &mut CPU) {
    reg_to_reg(cpu, B, L);
}

pub fn ld_b_hlind(cpu: &mut CPU) {
    hlind_to_reg(cpu, B);
}

pub fn ld_b_a(cpu: &mut CPU) {
    reg_to_reg(cpu, B, A);
}


pub fn ld_c_b(cpu: &mut CPU) {
    reg_to_reg(cpu, C, B);
}

pub fn ld_c_c(cpu: &mut CPU) {
    reg_to_reg(cpu, C, C);
}

pub fn ld_c_d(cpu: &mut CPU) {
    reg_to_reg(cpu, C, D);
}

pub fn ld_c_e(cpu: &mut CPU) {
    reg_to_reg(cpu, C, E);
}

pub fn ld_c_h(cpu: &mut CPU) {
    reg_to_reg(cpu, C, H);
}

pub fn ld_c_l(cpu: &mut CPU) {
    reg_to_reg(cpu, C, L);
}

pub fn ld_c_hlind(cpu: &mut CPU) {
    hlind_to_reg(cpu, C);
}

pub fn ld_c_a(cpu: &mut CPU) {
    reg_to_reg(cpu, C, A);
}


pub fn ld_d_b(cpu: &mut CPU) {
    reg_to_reg(cpu, D, B);
}

pub fn ld_d_c(cpu: &mut CPU) {
    reg_to_reg(cpu, D, C);
}

pub fn ld_d_d(cpu: &mut CPU) {
    reg_to_reg(cpu, D, D);
}

pub fn ld_d_e(cpu: &mut CPU) {
    reg_to_reg(cpu, D, E);
}

pub fn ld_d_h(cpu: &mut CPU) {
    reg_to_reg(cpu, D, H);
}

pub fn ld_d_l(cpu: &mut CPU) {
    reg_to_reg(cpu, D, L);
}

pub fn ld_d_hlind(cpu: &mut CPU) {
    hlind_to_reg(cpu, D);
}

pub fn ld_d_a(cpu: &mut CPU) {
    reg_to_reg(cpu, D, A);
}


pub fn ld_e_b(cpu: &mut CPU) {
    reg_to_reg(cpu, E, B);
}

pub fn ld_e_c(cpu: &mut CPU) {
    reg_to_reg(cpu, E, C);
}

pub fn ld_e_d(cpu: &mut CPU) {
    reg_to_reg(cpu, E, D);
}

pub fn ld_e_e(cpu: &mut CPU) {
    reg_to_reg(cpu, E, E);
}

pub fn ld_e_h(cpu: &mut CPU) {
    reg_to_reg(cpu, E, H);
}

pub fn ld_e_l(cpu: &mut CPU) {
    reg_to_reg(cpu, E, L);
}

pub fn ld_e_hlind(cpu: &mut CPU) {
    hlind_to_reg(cpu, E);
}

pub fn ld_e_a(cpu: &mut CPU) {
    reg_to_reg(cpu, E, A);
}


pub fn ld_h_b(cpu: &mut CPU) {
    reg_to_reg(cpu, H, B);
}

pub fn ld_h_c(cpu: &mut CPU) {
    reg_to_reg(cpu, H, C);
}

pub fn ld_h_d(cpu: &mut CPU) {
    reg_to_reg(cpu, H, D);
}

pub fn ld_h_e(cpu: &mut CPU) {
    reg_to_reg(cpu, H, E);
}

pub fn ld_h_h(cpu: &mut CPU) {
    reg_to_reg(cpu, H, H);
}

pub fn ld_h_l(cpu: &mut CPU) {
    reg_to_reg(cpu, H, L);
}

pub fn ld_h_hlind(cpu: &mut CPU) {
    hlind_to_reg(cpu, H);
}

pub fn ld_h_a(cpu: &mut CPU) {
    reg_to_reg(cpu, H, A);
}


pub fn ld_l_b(cpu: &mut CPU) {
    reg_to_reg(cpu, L, B);
}

pub fn ld_l_c(cpu: &mut CPU) {
    reg_to_reg(cpu, L, C);
}

pub fn ld_l_d(cpu: &mut CPU) {
    reg_to_reg(cpu, L, D);
}

pub fn ld_l_e(cpu: &mut CPU) {
    reg_to_reg(cpu, L, E);
}

pub fn ld_l_h(cpu: &mut CPU) {
    reg_to_reg(cpu, L, H);
}

pub fn ld_l_l(cpu: &mut CPU) {
    reg_to_reg(cpu, L, L);
}

pub fn ld_l_hlind(cpu: &mut CPU) {
    hlind_to_reg(cpu, L);
}

pub fn ld_l_a(cpu: &mut CPU) {
    reg_to_reg(cpu, L, A);
}


pub fn ld_hlind_b(cpu: &mut CPU) {
    reg_to_regxind(cpu, HL, B);
}

pub fn ld_hlind_c(cpu: &mut CPU) {
    reg_to_regxind(cpu, HL, C);
}

pub fn ld_hlind_d(cpu: &mut CPU) {
    reg_to_regxind(cpu, HL, D);
}

pub fn ld_hlind_e(cpu: &mut CPU) {
    reg_to_regxind(cpu, HL, E);
}

pub fn ld_hlind_h(cpu: &mut CPU) {
    reg_to_regxind(cpu, HL, H);
}

pub fn ld_hlind_l(cpu: &mut CPU) {
    reg_to_regxind(cpu, HL, L);
}

pub fn ld_hlind_a(cpu: &mut CPU) {
    reg_to_regxind(cpu, HL, A);
}


pub fn ld_a_b(cpu: &mut CPU) {
    reg_to_reg(cpu, A, B);
}

pub fn ld_a_c(cpu: &mut CPU) {
    reg_to_reg(cpu, A, C);
}

pub fn ld_a_d(cpu: &mut CPU) {
    reg_to_reg(cpu, A, D);
}

pub fn ld_a_e(cpu: &mut CPU) {
    reg_to_reg(cpu, A, E);
}

pub fn ld_a_h(cpu: &mut CPU) {
    reg_to_reg(cpu, A, H);
}

pub fn ld_a_l(cpu: &mut CPU) {
    reg_to_reg(cpu, A, L);
}

pub fn ld_a_hlind(cpu: &mut CPU) {
    hlind_to_reg(cpu, A);
}

pub fn ld_a_a(cpu: &mut CPU) {
    reg_to_reg(cpu, A, A);
}


pub fn ld_ff00u8_a(cpu: &mut CPU) {
    let val = cpu.fetch();
    cpu.mem.write((0xFF00 + val as u16) as usize, cpu.registers[A]);
    cpu.cycles += 12;
}

pub fn ld_a_ff00u8(cpu: &mut CPU) {
    let val = cpu.fetch();
    cpu.registers[A] = cpu.mem.read((0xFF00 + val as u16) as usize);
    cpu.cycles += 12;
}

pub fn ld_ff00c_a(cpu: &mut CPU) {
    let val = cpu.registers[C];
    cpu.mem.write((0xFF00 + val as u16) as usize, cpu.registers[A]);
    cpu.cycles += 8;
}

pub fn ld_a_ff00c(cpu: &mut CPU) {
    let val = cpu.registers[C];
    cpu.registers[A] = cpu.mem.read((0xFF00 + val as u16) as usize);
    cpu.cycles += 8;
}

pub fn ld_u16ind_a(cpu: &mut CPU) {
    let dir_low = cpu.fetch();
    let dir_high = cpu.fetch();
    cpu.mem.write((dir_low as u16 + dir_high as u16 * 0x100) as usize, cpu.registers[A]);
    cpu.cycles += 16;
}

pub fn ld_a_u16ind(cpu: &mut CPU) {
    let dir_low = cpu.fetch();
    let dir_high = cpu.fetch();
    cpu.registers[A] = cpu.mem.read((dir_low as u16 + dir_high as u16 * 0x100) as usize);
    cpu.cycles += 16;
}

//u16 LOAD/STORE/MOVE
fn u16_to_regx(cpu: &mut CPU, regx: usize) {
    cpu.registers[regx] = cpu.fetch();
    cpu.registers[regx - 1] = cpu.fetch();
    cpu.cycles += 12;
}

fn pop_regx(cpu: &mut CPU, regx: usize) {
    cpu.registers[regx] = cpu.mem.read(cpu.sp as usize);
    cpu.sp += 1;
    cpu.registers[regx - 1] = cpu.mem.read(cpu.sp as usize);
    cpu.sp += 1;
    cpu.cycles += 12;
}

fn push_regx(cpu: &mut CPU, regx: usize) {
    cpu.sp -= 1;
    cpu.mem.write(cpu.sp as usize, cpu.registers[regx - 1]);
    cpu.sp -= 1;
    cpu.mem.write(cpu.sp as usize, cpu.registers[regx]);
    cpu.cycles += 16;
}

pub fn ld_bc_u16(cpu: &mut CPU) {
    u16_to_regx(cpu, BC);
}

pub fn ld_de_u16(cpu: &mut CPU) {
    u16_to_regx(cpu, DE);
}

pub fn ld_hl_u16(cpu: &mut CPU) {
    u16_to_regx(cpu, HL);
}

pub fn ld_sp_u16(cpu: &mut CPU) {
    cpu.sp = cpu.fetch() as u16 + cpu.fetch() as u16 * 0x100;
    cpu.cycles += 12;
}

pub fn ld_u16ind_sp(cpu: &mut CPU) {
    let dir = cpu.fetch() as u16 + cpu.fetch() as u16 * 0x100;
    cpu.mem.write(dir as usize, cpu.sp as u8);
    cpu.mem.write((dir + 1) as usize, (cpu.sp / 0x100) as u8);
    cpu.cycles += 20;
}

pub fn ld_sp_hl(cpu: &mut CPU) {
    cpu.sp = cpu.registers[L] as u16 + cpu.registers[H] as u16 * 0x100;
    cpu.cycles += 8;
}

pub fn pop_bc(cpu: &mut CPU) {
    pop_regx(cpu, BC);
}

pub fn pop_de(cpu: &mut CPU) {
    pop_regx(cpu, DE);
}

pub fn pop_hl(cpu: &mut CPU) {
    pop_regx(cpu, HL);
}

pub fn pop_af(cpu: &mut CPU) {
    cpu.registers[F] = cpu.mem.read(cpu.sp as usize) & 0xF0;
    cpu.sp += 1;
    cpu.registers[A] = cpu.mem.read(cpu.sp as usize);
    cpu.sp += 1;
    cpu.cycles += 12;
}

pub fn push_bc(cpu: &mut CPU) {
    push_regx(cpu, BC);
}

pub fn push_de(cpu: &mut CPU) {
    push_regx(cpu, DE);
}

pub fn push_hl(cpu: &mut CPU) {
    push_regx(cpu, HL);
}

pub fn push_af(cpu: &mut CPU) {
    push_regx(cpu, AF);
}

// u8 ALU

fn check_half_carry(op1: u8, op2: u8) -> bool {
    (op1 & 0x0F) + (op2 & 0x0F) > 0x0F
}

fn check_half_borrow(op1: u8, op2: u8) -> bool {
    (op1 & 0xf).wrapping_sub(op2 & 0xf) & 0x10 == 0x10
}

fn inc_reg(cpu: &mut CPU, reg: usize) {
    set_flags(cpu, H_FLAG, check_half_carry(cpu.registers[reg], 1));
    set_flags(cpu, N_FLAG, false);
    cpu.registers[reg] = cpu.registers[reg].overflowing_add(1).0;
    set_flags(cpu, Z_FLAG, cpu.registers[reg] == 0);
    cpu.cycles += 4;
}

fn dec_reg(cpu: &mut CPU, reg: usize) {
    set_flags(cpu, H_FLAG, check_half_borrow(cpu.registers[reg], 1));
    set_flags(cpu, N_FLAG, true);
    cpu.registers[reg] = cpu.registers[reg].overflowing_sub(1).0;
    set_flags(cpu, Z_FLAG, cpu.registers[reg] == 0);
    cpu.cycles += 4;
}

pub fn inc_b(cpu: &mut CPU) {
    inc_reg(cpu, B);
}

pub fn inc_c(cpu: &mut CPU) {
    inc_reg(cpu, C);
}

pub fn inc_d(cpu: &mut CPU) {
    inc_reg(cpu, D);
}

pub fn inc_e(cpu: &mut CPU) {
    inc_reg(cpu, E);
}

pub fn inc_h(cpu: &mut CPU) {
    inc_reg(cpu, H);
}

pub fn inc_l(cpu: &mut CPU) {
    inc_reg(cpu, L);
}

pub fn inc_hlind(cpu: &mut CPU) {
    let dir = cpu.registers[L] as u16 + cpu.registers[H] as u16 * 0x100;
    let mut val = cpu.mem.read(dir as usize);
    set_flags(cpu, H_FLAG, check_half_carry(val, 1));
    set_flags(cpu, N_FLAG, false);
    val = val.overflowing_add(1).0;
    set_flags(cpu, Z_FLAG, val == 0);
    cpu.mem.write(dir as usize, val);
    cpu.cycles += 12;
}

pub fn inc_a(cpu: &mut CPU) {
    inc_reg(cpu, A);
}

pub fn dec_b(cpu: &mut CPU) {
    dec_reg(cpu, B);
}

pub fn dec_c(cpu: &mut CPU) {
    dec_reg(cpu, C);
}

pub fn dec_d(cpu: &mut CPU) {
    dec_reg(cpu, D);
}

pub fn dec_e(cpu: &mut CPU) {
    dec_reg(cpu, E);
}

pub fn dec_h(cpu: &mut CPU) {
    dec_reg(cpu, H);
}

pub fn dec_l(cpu: &mut CPU) {
    dec_reg(cpu, L);
}

pub fn dec_hlind(cpu: &mut CPU) {
    let dir = cpu.registers[L] as u16 + cpu.registers[H] as u16 * 0x100;
    let mut val = cpu.mem.read(dir as usize);
    set_flags(cpu, H_FLAG, check_half_borrow(val, 1));
    set_flags(cpu, N_FLAG, true);
    val = val.overflowing_sub(1).0;
    set_flags(cpu, Z_FLAG, val == 0);
    cpu.mem.write(dir as usize, val);
    cpu.cycles += 12;
}

pub fn dec_a(cpu: &mut CPU) {
    dec_reg(cpu, A);
}

pub fn daa(cpu: &mut CPU) {
    let mut val = cpu.registers[A] as u16;

    if get_negative(cpu) == 0 {
        if get_half_carry(cpu) == 1 || val & 0x0F > 0x09 {
            val += 0x06;
        }
        if get_carry(cpu) == 1 || val > 0x99 {
            val += 0x60;
            set_flags(cpu, C_FLAG, true);
        }
    } else {
        if get_half_carry(cpu) == 1 {
			val = val.wrapping_sub(0x06);
		}

		if get_carry(cpu) == 1 {
			val = val.wrapping_sub(0x60);
            set_flags(cpu, C_FLAG, true);
		}
    }

    set_flags(cpu, Z_FLAG, val == 0);
    set_flags(cpu, H_FLAG, false);
    cpu.registers[A] = val as u8;
    cpu.cycles += 4;
}

pub fn scf(cpu: &mut CPU) {
    set_flags(cpu, C_FLAG, true);
    set_flags(cpu, N_FLAG | H_FLAG, false);
}

pub fn cpl(cpu: &mut CPU) {
    cpu.registers[A] = !cpu.registers[A];
    set_flags(cpu, N_FLAG | H_FLAG, true);
    cpu.cycles += 4;
}

pub fn ccf(cpu: &mut CPU) {
    let cond = get_carry(cpu) ^ 1 != 0;
    set_flags(cpu, C_FLAG, cond);
    set_flags(cpu, N_FLAG | H_FLAG, false);
}

fn add_a_reg(cpu: &mut CPU, reg_src: usize) {
    set_flags(cpu, H_FLAG, check_half_carry(cpu.registers[A], cpu.registers[reg_src]));
    let x = cpu.registers[A].overflowing_add(cpu.registers[reg_src]);
    cpu.registers[A] = x.0;
    set_flags(cpu, Z_FLAG, x.0 == 0);
    set_flags(cpu, C_FLAG, x.1);
    set_flags(cpu, N_FLAG, false);
    cpu.cycles += 4;
}

pub fn add_a_b(cpu: &mut CPU) {
    add_a_reg(cpu, B);
}

pub fn add_a_c(cpu: &mut CPU) {
    add_a_reg(cpu, C);
}

pub fn add_a_d(cpu: &mut CPU) {
    add_a_reg(cpu, D);
}

pub fn add_a_e(cpu: &mut CPU) {
    add_a_reg(cpu, E);
}

pub fn add_a_h(cpu: &mut CPU) {
    add_a_reg(cpu, H);
}
 
pub fn add_a_l(cpu: &mut CPU) {
    add_a_reg(cpu, L);
}

pub fn add_a_hlind(cpu: &mut CPU) {
    let dir = cpu.registers[L] as u16 + cpu.registers[H] as u16 * 0x100;
    let val = cpu.mem.read(dir as usize);
    set_flags(cpu, H_FLAG, check_half_carry(cpu.registers[A], val));
    let x = cpu.registers[A].overflowing_add(val);
    cpu.registers[A] = x.0;
    set_flags(cpu, Z_FLAG, x.0 == 0);
    set_flags(cpu, C_FLAG, x.1);
    set_flags(cpu, N_FLAG, false);
    cpu.cycles += 8;
}

pub fn add_a_a(cpu: &mut CPU) {
    add_a_reg(cpu, A);
}

fn check_half_carry_cy(op1: u8, op2: u8, cy: u8) -> bool {
    (op1 & 0x0F) + (op2 & 0x0F) + (cy & 0x0F) > 0x0F
}

fn adc_a_reg(cpu: &mut CPU, reg_src: usize) {
    let cy = get_carry(cpu);
    set_flags(cpu, H_FLAG, check_half_carry_cy(cpu.registers[A], cpu.registers[reg_src], cy));

    let x_temp = cpu.registers[A].overflowing_add(cpu.registers[reg_src]);
    let x = x_temp.0.overflowing_add(cy);

    cpu.registers[A] = x.0;
    set_flags(cpu, Z_FLAG, x.0 == 0);
    set_flags(cpu, C_FLAG, x.1 | x_temp.1);
    set_flags(cpu, N_FLAG, false);
    cpu.cycles += 4;
}

pub fn adc_a_b(cpu: &mut CPU) {
    adc_a_reg(cpu, B);
}

pub fn adc_a_c(cpu: &mut CPU) {
    adc_a_reg(cpu, C);
}

pub fn adc_a_d(cpu: &mut CPU) {
    adc_a_reg(cpu, D);
}

pub fn adc_a_e(cpu: &mut CPU) {
    adc_a_reg(cpu, E);
}

pub fn adc_a_h(cpu: &mut CPU) {
    adc_a_reg(cpu, H);
}

pub fn adc_a_l(cpu: &mut CPU) {
    adc_a_reg(cpu, L);
}

pub fn adc_a_hlind(cpu: &mut CPU) {
    let dir = cpu.registers[L] as u16 + cpu.registers[H] as u16 * 0x100;
    let val = cpu.mem.read(dir as usize);
    let cy = get_carry(cpu);
    set_flags(cpu, H_FLAG, check_half_carry_cy(cpu.registers[A], val, cy));

    let x_temp = cpu.registers[A].overflowing_add(val);
    let x = x_temp.0.overflowing_add(cy);

    cpu.registers[A] = x.0;
    set_flags(cpu, Z_FLAG, x.0 == 0);
    set_flags(cpu, C_FLAG, x.1 | x_temp.1);
    set_flags(cpu, N_FLAG, false);
    cpu.cycles += 8;
}

pub fn adc_a_a(cpu: &mut CPU) {
    adc_a_reg(cpu, A);
}

fn sub_a_reg(cpu: &mut CPU, reg_src: usize) {
    set_flags(cpu, H_FLAG, check_half_borrow(cpu.registers[A], cpu.registers[reg_src]));
    let x = cpu.registers[A].overflowing_sub(cpu.registers[reg_src]);
    cpu.registers[A] = x.0;
    set_flags(cpu, Z_FLAG, x.0 == 0);
    set_flags(cpu, C_FLAG, x.1);
    set_flags(cpu, N_FLAG, true);
    cpu.cycles += 4;
}

pub fn sub_a_b(cpu: &mut CPU) {
    sub_a_reg(cpu, B);
}

pub fn sub_a_c(cpu: &mut CPU) {
    sub_a_reg(cpu, C);
}

pub fn sub_a_d(cpu: &mut CPU) {
    sub_a_reg(cpu, D);
}

pub fn sub_a_e(cpu: &mut CPU) {
    sub_a_reg(cpu, E);
}

pub fn sub_a_h(cpu: &mut CPU) {
    sub_a_reg(cpu, H);
}

pub fn sub_a_l(cpu: &mut CPU) {
    sub_a_reg(cpu, L);
}

pub fn sub_a_hlind(cpu: &mut CPU) {
    let dir = cpu.registers[L] as u16 + cpu.registers[H] as u16 * 0x100;
    let val = cpu.mem.read(dir as usize);
    set_flags(cpu, H_FLAG, check_half_borrow(cpu.registers[A], val));
    let x = cpu.registers[A].overflowing_sub(val);
    cpu.registers[A] = x.0;
    set_flags(cpu, Z_FLAG, x.0 == 0);
    set_flags(cpu, C_FLAG, x.1);
    set_flags(cpu, N_FLAG, false);
    cpu.cycles += 8;
}

pub fn sub_a_a(cpu: &mut CPU) {
    sub_a_reg(cpu, A);
}

fn check_half_borrow_cy(op1: u8, op2: u8, cy: u8) -> bool {
    (op1 & 0xf).wrapping_sub(op2 & 0xf).wrapping_sub(cy) & 0x10 == 0x10
}

fn sbc_a_reg(cpu: &mut CPU, reg_src: usize) {
    let cy = get_carry(cpu);
    set_flags(cpu, H_FLAG, check_half_borrow_cy(cpu.registers[A], cpu.registers[reg_src], cy));

    let x_temp = cpu.registers[A].overflowing_sub(cpu.registers[reg_src]);
    let x = x_temp.0.overflowing_sub(cy);

    cpu.registers[A] = x.0;
    set_flags(cpu, Z_FLAG, x.0 == 0);
    set_flags(cpu, C_FLAG, x.1 | x_temp.1);
    set_flags(cpu, N_FLAG, true);
    cpu.cycles += 4;
}

pub fn sbc_a_b(cpu: &mut CPU) {
    sbc_a_reg(cpu, B);
}

pub fn sbc_a_c(cpu: &mut CPU) {
    sbc_a_reg(cpu, C);
}

pub fn sbc_a_d(cpu: &mut CPU) {
    sbc_a_reg(cpu, D);
}

pub fn sbc_a_e(cpu: &mut CPU) {
    sbc_a_reg(cpu, E);
}

pub fn sbc_a_h(cpu: &mut CPU) {
    sbc_a_reg(cpu, H);
}

pub fn sbc_a_l(cpu: &mut CPU) {
    sbc_a_reg(cpu, L);
}

pub fn sbc_a_hlind(cpu: &mut CPU) {
    let dir = cpu.registers[L] as u16 + cpu.registers[H] as u16 * 0x100;
    let val = cpu.mem.read(dir as usize);
    let cy = get_carry(cpu);
    set_flags(cpu, H_FLAG, check_half_carry_cy(cpu.registers[A], val, cy));

    let x_temp = cpu.registers[A].overflowing_sub(val);
    let x = x_temp.0.overflowing_sub(cy);

    cpu.registers[A] = x.0;
    set_flags(cpu, Z_FLAG, x.0 == 0);
    set_flags(cpu, C_FLAG, x.1 | x_temp.1);
    set_flags(cpu, N_FLAG, false);
    cpu.cycles += 8;
}

pub fn sbc_a_a(cpu: &mut CPU) {
    sbc_a_reg(cpu, A);
}

fn and_a_reg(cpu: &mut CPU, reg_src: usize) {
    cpu.registers[A] &= cpu.registers[reg_src];
    set_flags(cpu, N_FLAG | C_FLAG, false);
    set_flags(cpu, H_FLAG, true);
    set_flags(cpu, Z_FLAG, cpu.registers[A] == 0);

    cpu.cycles += 4;
}

pub fn and_a_b(cpu: &mut CPU) {
    and_a_reg(cpu, B);
}

pub fn and_a_c(cpu: &mut CPU) {
    and_a_reg(cpu, C);
}

pub fn and_a_d(cpu: &mut CPU) {
    and_a_reg(cpu, D);
}

pub fn and_a_e(cpu: &mut CPU) {
    and_a_reg(cpu, E);
}

pub fn and_a_h(cpu: &mut CPU) {
    and_a_reg(cpu, H);
}

pub fn and_a_l(cpu: &mut CPU) {
    and_a_reg(cpu, L);
}

pub fn and_a_hlind(cpu: &mut CPU) {
    let dir = cpu.registers[L] as u16 + cpu.registers[H] as u16 + 0x100;
    let val = cpu.mem.read(dir as usize);
    cpu.registers[A] &= val;
    set_flags(cpu, N_FLAG | C_FLAG, false);
    set_flags(cpu, H_FLAG, true);
    set_flags(cpu, Z_FLAG, cpu.registers[A] == 0);

    cpu.cycles += 8;
}

pub fn and_a_a(cpu: &mut CPU) {
    and_a_reg(cpu, A);
}

fn xor_a_reg(cpu: &mut CPU, reg_src: usize) {
    cpu.registers[A] ^= cpu.registers[reg_src];
    set_flags(cpu, N_FLAG | C_FLAG | H_FLAG, false);
    set_flags(cpu, Z_FLAG, cpu.registers[A] == 0);

    cpu.cycles += 4;
}

pub fn xor_a_b(cpu: &mut CPU) {
    xor_a_reg(cpu, B);
}

pub fn xor_a_c(cpu: &mut CPU) {
    xor_a_reg(cpu, C);
}

pub fn xor_a_d(cpu: &mut CPU) {
    xor_a_reg(cpu, D);
}

pub fn xor_a_e(cpu: &mut CPU) {
    xor_a_reg(cpu, E);
}

pub fn xor_a_h(cpu: &mut CPU) {
    xor_a_reg(cpu, H);
}

pub fn xor_a_l(cpu: &mut CPU) {
    xor_a_reg(cpu, L);
}

pub fn xor_a_hlind(cpu: &mut CPU) {
    let dir = cpu.registers[L] as u16 + cpu.registers[H] as u16 * 0x100;
    let val = cpu.mem.read(dir as usize);
    cpu.registers[A] ^= val;
    set_flags(cpu, N_FLAG | C_FLAG | H_FLAG, false);
    set_flags(cpu, Z_FLAG, cpu.registers[A] == 0);

    cpu.cycles += 8;
}

pub fn xor_a_a(cpu: &mut CPU) {
    xor_a_reg(cpu, A);
}

fn or_a_reg(cpu: &mut CPU, reg_src: usize) {
    cpu.registers[A] |= cpu.registers[reg_src];
    set_flags(cpu, N_FLAG | C_FLAG | H_FLAG, false);
    set_flags(cpu, Z_FLAG, cpu.registers[A] == 0);

    cpu.cycles += 4;
}

pub fn or_a_b(cpu: &mut CPU) {
    or_a_reg(cpu, B);
}

pub fn or_a_c(cpu: &mut CPU) {
    or_a_reg(cpu, C);
}

pub fn or_a_d(cpu: &mut CPU) {
    or_a_reg(cpu, D);
}

pub fn or_a_e(cpu: &mut CPU) {
    or_a_reg(cpu, E);
}

pub fn or_a_h(cpu: &mut CPU) {
    or_a_reg(cpu, H);
}

pub fn or_a_l(cpu: &mut CPU) {
    or_a_reg(cpu, L);
}

pub fn or_a_hlind(cpu: &mut CPU) {
    let dir = cpu.registers[L] as u16 + cpu.registers[H] as u16 * 0x100;
    let val = cpu.mem.read(dir as usize);
    cpu.registers[A] |= val;
    set_flags(cpu, N_FLAG | C_FLAG | H_FLAG, false);
    set_flags(cpu, Z_FLAG, cpu.registers[A] == 0);

    cpu.cycles += 8;
}

pub fn or_a_a(cpu: &mut CPU) {
    or_a_reg(cpu, A);
}

fn cp_a_reg(cpu: &mut CPU, reg_src: usize) {
    set_flags(cpu, Z_FLAG, cpu.registers[A] == cpu.registers[reg_src]);
    set_flags(cpu, N_FLAG, true);
    set_flags(cpu, H_FLAG, check_half_borrow(cpu.registers[A], cpu.registers[reg_src]));
    set_flags(cpu, C_FLAG, cpu.registers[A] < cpu.registers[reg_src]);
    cpu.cycles += 4;
}

pub fn cp_a_b(cpu: &mut CPU) {
    cp_a_reg(cpu, B);
}

pub fn cp_a_c(cpu: &mut CPU) {
    cp_a_reg(cpu, C);
}

pub fn cp_a_d(cpu: &mut CPU) {
    cp_a_reg(cpu, D);
}

pub fn cp_a_e(cpu: &mut CPU) {
    cp_a_reg(cpu, E);
}

pub fn cp_a_h(cpu: &mut CPU) {
    cp_a_reg(cpu, H);
}

pub fn cp_a_l(cpu: &mut CPU) {
    cp_a_reg(cpu, L);
}

pub fn cp_a_hlind(cpu: &mut CPU) {
    let dir = cpu.registers[L] as u16 + cpu.registers[H] as u16 * 0x100;
    let val = cpu.mem.read(dir as usize);
    set_flags(cpu, Z_FLAG, cpu.registers[A] == val);
    set_flags(cpu, N_FLAG, true);
    set_flags(cpu, H_FLAG, check_half_borrow(cpu.registers[A], val));
    set_flags(cpu, C_FLAG, cpu.registers[A] < val);

    cpu.cycles += 8;
}

pub fn cp_a_a(cpu: &mut CPU) {
    cp_a_reg(cpu, A);
}

pub fn add_a_u8(cpu: &mut CPU) {
    let val = cpu.fetch();
    set_flags(cpu, H_FLAG, check_half_carry(cpu.registers[A], val));
    let x = cpu.registers[A].overflowing_add(val);
    cpu.registers[A] = x.0;
    set_flags(cpu, Z_FLAG, x.0 == 0);
    set_flags(cpu, C_FLAG, x.1);
    set_flags(cpu, N_FLAG, false);
    cpu.cycles += 8;
}

pub fn adc_a_u8(cpu: &mut CPU) {
    let val = cpu.fetch();
    let cy = get_carry(cpu);
    set_flags(cpu, H_FLAG, check_half_carry_cy(cpu.registers[A], val, cy));

    let x_temp = cpu.registers[A].overflowing_add(val);
    let x = x_temp.0.overflowing_add(cy);

    cpu.registers[A] = x.0;
    set_flags(cpu, Z_FLAG, x.0 == 0);
    set_flags(cpu, C_FLAG, x.1 | x_temp.1);
    set_flags(cpu, N_FLAG, false);
    cpu.cycles += 8;
}

pub fn sub_a_u8(cpu: &mut CPU) {
    let val = cpu.fetch();
    set_flags(cpu, H_FLAG, check_half_borrow(cpu.registers[A], val));
    let x = cpu.registers[A].overflowing_sub(val);
    cpu.registers[A] = x.0;
    set_flags(cpu, Z_FLAG, x.0 == 0);
    set_flags(cpu, C_FLAG, x.1);
    set_flags(cpu, N_FLAG, true);
    cpu.cycles += 8;
}

pub fn sbc_a_u8(cpu: &mut CPU) {
    let val = cpu.fetch();
    let cy = get_carry(cpu);
    set_flags(cpu, H_FLAG, check_half_borrow_cy(cpu.registers[A], val, cy));

    let x_temp = cpu.registers[A].overflowing_sub(val);
    let x = x_temp.0.overflowing_sub(cy);

    cpu.registers[A] = x.0;
    set_flags(cpu, Z_FLAG, x.0 == 0);
    set_flags(cpu, C_FLAG, x.1 | x_temp.1);
    set_flags(cpu, N_FLAG, true);
    cpu.cycles += 8;
}

pub fn and_a_u8(cpu: &mut CPU) {
    let val = cpu.fetch();
    cpu.registers[A] &= val;
    set_flags(cpu, N_FLAG | C_FLAG, false);
    set_flags(cpu, H_FLAG, true);
    set_flags(cpu, Z_FLAG, cpu.registers[A] == 0);

    cpu.cycles += 8;
}

pub fn xor_a_u8(cpu: &mut CPU) {
    let val = cpu.fetch();
    cpu.registers[A] ^= val;
    set_flags(cpu, N_FLAG | C_FLAG | H_FLAG, false);
    set_flags(cpu, Z_FLAG, cpu.registers[A] == 0);

    cpu.cycles += 8;
}

pub fn or_a_u8(cpu: &mut CPU) {
    let val = cpu.fetch();
    cpu.registers[A] |= val;
    set_flags(cpu, N_FLAG | C_FLAG | H_FLAG, false);
    set_flags(cpu, Z_FLAG, cpu.registers[A] == 0);

    cpu.cycles += 8;
}

pub fn cp_a_u8(cpu: &mut CPU) {
    let val = cpu.fetch();
    set_flags(cpu, Z_FLAG, cpu.registers[A] == val);
    set_flags(cpu, N_FLAG, true);
    set_flags(cpu, H_FLAG, check_half_borrow(cpu.registers[A], val));
    set_flags(cpu, C_FLAG, cpu.registers[A] < val);
    cpu.cycles += 8;
}

// u16 ALU

fn inc_regx(cpu: &mut CPU, regx: usize) {
    let mut val = cpu.registers[regx] as u16 + cpu.registers[regx - 1] as u16 * 0x100;
    val = val.wrapping_add(1);
    cpu.registers[regx] = val as u8;
    cpu.registers[regx - 1] = (val / 0x100) as u8;
    cpu.cycles += 8;
}

pub fn inc_bc(cpu: &mut CPU) {
    inc_regx(cpu, BC);
}

pub fn inc_de(cpu: &mut CPU) {
    inc_regx(cpu, DE);
}

pub fn inc_hl(cpu: &mut CPU) {
    inc_regx(cpu, HL);
}

pub fn inc_sp(cpu: &mut CPU) {
    cpu.sp = cpu.sp.wrapping_add(1);
    cpu.cycles += 8;
}

fn dec_regx(cpu: &mut CPU, regx: usize) {
    let mut val = cpu.registers[regx] as u16 + cpu.registers[regx - 1] as u16 * 0x100;
    val = val.wrapping_sub(1);
    cpu.registers[regx] = val as u8;
    cpu.registers[regx - 1] = (val / 0x100) as u8;
    cpu.cycles += 8;
}

pub fn dec_bc(cpu: &mut CPU) {
    dec_regx(cpu, BC);
}

pub fn dec_de(cpu: &mut CPU) {
    dec_regx(cpu, DE);
}

pub fn dec_hl(cpu: &mut CPU) {
    dec_regx(cpu, HL);
}

pub fn dec_sp(cpu: &mut CPU) {
    cpu.sp = cpu.sp.wrapping_sub(1);
    cpu.cycles += 8;
}

fn add_hl_regx(cpu: &mut CPU, regx_src: usize) {
    let dest = cpu.registers[L] as u16 + cpu.registers[H] as u16 * 0x100;
    let src = cpu.registers[regx_src] as u16 + cpu.registers[regx_src - 1] as u16 * 0x100;

    set_flags(cpu, H_FLAG, (dest & 0x0FFF) + (src & 0x0FFF) > 0x0FFF);
    let x = dest.overflowing_add(src);
    cpu.registers[L] = x.0 as u8;
    cpu.registers[H] = (x.0 / 0x100) as u8;
    set_flags(cpu, N_FLAG, false);
    set_flags(cpu, C_FLAG, x.1);
    cpu.cycles += 8;
}

pub fn add_hl_bc(cpu: &mut CPU) {
    add_hl_regx(cpu, BC);
}

pub fn add_hl_de(cpu: &mut CPU) {
    add_hl_regx(cpu, DE);
}

pub fn add_hl_hl(cpu: &mut CPU) {
    add_hl_regx(cpu, HL);
}

pub fn add_hl_sp(cpu: &mut CPU) {
    let dest = cpu.registers[L] as u16 + cpu.registers[H] as u16 * 0x100;

    set_flags(cpu, H_FLAG, (dest & 0x0FFF) + (cpu.sp & 0x0FFF) > 0x0FFF);
    let x = dest.overflowing_add(cpu.sp);
    cpu.registers[L] = x.0 as u8;
    cpu.registers[H] = (x.0 / 0x100) as u8;
    set_flags(cpu, N_FLAG, false);
    set_flags(cpu, C_FLAG, x.1);
    cpu.cycles += 8;
}

pub fn add_sp_i8(cpu: &mut CPU) {
    let val = cpu.fetch() as i8 as i16 as u16;
    set_flags(cpu, N_FLAG | Z_FLAG, false);
    set_flags(cpu, H_FLAG, check_half_carry(cpu.sp as u8, val as u8));
    set_flags(cpu, C_FLAG, (cpu.sp & 0x00FF) + (val & 0x00FF) > 0x00FF);
    cpu.sp = cpu.sp.wrapping_add(val);
    cpu.cycles += 16;
}

pub fn ld_hl_sp_i8(cpu: &mut CPU) {
    let val = cpu.fetch() as i8 as i16 as u16;
    let src = cpu.sp;
    
    set_flags(cpu, H_FLAG, check_half_carry(val as u8, src as u8));

    let x = src.wrapping_add(val);
    cpu.registers[L] = x as u8;
    cpu.registers[H] = (x as u16 / 0x100) as u8;

    set_flags(cpu, Z_FLAG | N_FLAG, false);
    set_flags(cpu, C_FLAG, (cpu.sp & 0x00FF) + (val & 0x00FF) > 0x00FF);

    cpu.cycles += 12;
}

// x8/RSB

pub fn rlca(cpu: &mut CPU) {
    // TODO es posible que esto este mal
    let mut rot = cpu.registers[A];
    let carry = rot & 0b10000000;

    set_flags(cpu, C_FLAG, carry == 0b10000000);

    rot <<= 1;
    rot |= carry >> 7;
    cpu.registers[A] = rot;

    set_flags(cpu, Z_FLAG | N_FLAG | H_FLAG, false);

    cpu.cycles += 4;
}

pub fn rla(cpu: &mut CPU) {
    // TODO es posible que esto este mal
    let mut rot = cpu.registers[A];
    let carry = rot & 0b10000000;
    let prev_carry = cpu.registers[F] & C_FLAG;

    set_flags(cpu, C_FLAG, carry == 0b10000000);

    rot <<= 1;
    rot |= prev_carry >> 4;
    cpu.registers[A] = rot;

    set_flags(cpu, Z_FLAG | N_FLAG | H_FLAG, false);

    cpu.cycles += 4;
}

pub fn rrca(cpu: &mut CPU) {
    // TODO es posible que esto este mal
    let mut rot = cpu.registers[A];
    let carry = rot & 1;

    set_flags(cpu, C_FLAG, carry == 0b00000001);

    rot >>= 1;
    rot |= carry << 7;
    cpu.registers[A] = rot;

    set_flags(cpu, Z_FLAG | N_FLAG | H_FLAG, false);

    cpu.cycles += 4;
}

pub fn rra(cpu: &mut CPU) {
    // TODO es posible que esto este mal
    let mut rot = cpu.registers[A];
    let carry = rot & 0b00000001;
    let prev_carry = cpu.registers[F] & C_FLAG;

    set_flags(cpu, C_FLAG, carry == 0b00000001);

    rot >>= 1;
    rot |= prev_carry << 3;
    cpu.registers[A] = rot;

    set_flags(cpu, Z_FLAG | N_FLAG | H_FLAG, false);

    cpu.cycles += 4;
}

fn rlc_reg(cpu: &mut CPU, reg: usize) {
    // TODO es posible que esto este mal
    let mut rot = cpu.registers[reg];
    let carry = rot & 0b10000000;

    set_flags(cpu, C_FLAG, carry == 0b10000000);

    rot <<= 1;
    rot |= carry >> 7;
    cpu.registers[reg] = rot;

    set_flags(cpu, N_FLAG | H_FLAG, false);
    set_flags(cpu, Z_FLAG, cpu.registers[reg] == 0);

    cpu.cycles += 8;
}

pub fn rlc_b(cpu: &mut CPU) {
    rlc_reg(cpu, B);
}

pub fn rlc_c(cpu: &mut CPU) {
    rlc_reg(cpu, C);
}

pub fn rlc_d(cpu: &mut CPU) {
    rlc_reg(cpu, D);
}

pub fn rlc_e(cpu: &mut CPU) {
    rlc_reg(cpu, E);
}

pub fn rlc_h(cpu: &mut CPU) {
    rlc_reg(cpu, H);
}

pub fn rlc_l(cpu: &mut CPU) {
    rlc_reg(cpu, L);
}

pub fn rlc_hlind(cpu: &mut CPU) {
    // TODO es posible que esto este mal
    let dir = cpu.registers[L] as u16 + cpu.registers[H] as u16 * 0x100;
    let mut rot = cpu.mem.read(dir as usize);
    let carry = rot & 0b10000000;

    set_flags(cpu, C_FLAG, carry == 0b10000000);

    rot <<= 1;
    rot |= carry >> 7;
    cpu.mem.write(dir as usize, rot);

    set_flags(cpu, N_FLAG | H_FLAG, false);
    set_flags(cpu, Z_FLAG, rot == 0);

    cpu.cycles += 16;
}

pub fn rlc_a(cpu: &mut CPU) {
    rlc_reg(cpu, A);
}

fn rrc_reg(cpu: &mut CPU, reg: usize) {
    // TODO es posible que esto este mal
    let mut rot = cpu.registers[reg];
    let carry = rot & 1;

    set_flags(cpu, C_FLAG, carry == 0b00000001);

    rot >>= 1;
    rot |= carry << 7;
    cpu.registers[reg] = rot;

    set_flags(cpu, N_FLAG | H_FLAG, false);
    set_flags(cpu, Z_FLAG, rot == 0);

    cpu.cycles += 8;
}

pub fn rrc_b(cpu: &mut CPU) {
    rrc_reg(cpu, B);
}

pub fn rrc_c(cpu: &mut CPU) {
    rrc_reg(cpu, C);
}

pub fn rrc_d(cpu: &mut CPU) {
    rrc_reg(cpu, D);
}

pub fn rrc_e(cpu: &mut CPU) {
    rrc_reg(cpu, E);
}

pub fn rrc_h(cpu: &mut CPU) {
    rrc_reg(cpu, H);
}

pub fn rrc_l(cpu: &mut CPU) {
    rrc_reg(cpu, L);
}

pub fn rrc_hlind(cpu: &mut CPU) {
    // TODO es posible que esto este mal
    let dir = cpu.registers[L] as u16 + cpu.registers[H] as u16 * 0x100;
    let mut rot = cpu.mem.read(dir as usize);
    let carry = rot & 1;

    set_flags(cpu, C_FLAG, carry == 0b00000001);

    rot >>= 1;
    rot |= carry << 7;
    cpu.mem.write(dir as usize, rot);

    set_flags(cpu, N_FLAG | H_FLAG, false);
    set_flags(cpu, Z_FLAG, rot == 0);

    cpu.cycles += 16;
}

pub fn rrc_a(cpu: &mut CPU) {
    rrc_reg(cpu, A);
}

fn rl_reg(cpu: &mut CPU, reg: usize) {
    // TODO es posible que esto este mal
    let mut rot = cpu.registers[reg];
    let carry = rot & 0b10000000;
    let prev_carry = cpu.registers[F] & C_FLAG;

    set_flags(cpu, C_FLAG, carry == 0b10000000);

    rot <<= 1;
    rot |= prev_carry >> 4;
    cpu.registers[reg] = rot;

    set_flags(cpu, N_FLAG | H_FLAG, false);
    set_flags(cpu, Z_FLAG, rot == 0);

    cpu.cycles += 8;
}

pub fn rl_b(cpu: &mut CPU) {
    rl_reg(cpu, B);
}

pub fn rl_c(cpu: &mut CPU) {
    rl_reg(cpu, C);
}

pub fn rl_d(cpu: &mut CPU) {
    rl_reg(cpu, D);
}

pub fn rl_e(cpu: &mut CPU) {
    rl_reg(cpu, E);
}

pub fn rl_h(cpu: &mut CPU) {
    rl_reg(cpu, H);
}

pub fn rl_l(cpu: &mut CPU) {
    rl_reg(cpu, L);
}

pub fn rl_hlind(cpu: &mut CPU) {
    // TODO es posible que esto este mal
    let dir = cpu.registers[L] as u16 + cpu.registers[H] as u16 * 0x100;
    let mut rot = cpu.mem.read(dir as usize);
    let carry = rot & 0b10000000;
    let prev_carry = cpu.registers[F] & C_FLAG;

    set_flags(cpu, C_FLAG, carry == 0b10000000);

    rot <<= 1;
    rot |= prev_carry >> 4;
    cpu.mem.write(dir as usize, rot);

    set_flags(cpu, N_FLAG | H_FLAG, false);
    set_flags(cpu, Z_FLAG, rot == 0);

    cpu.cycles += 16;
}

pub fn rl_a(cpu: &mut CPU) {
    rl_reg(cpu, A);
}

fn rr_reg(cpu: &mut CPU, reg: usize) {
    // TODO es posible que esto este mal
    let mut rot = cpu.registers[reg];
    let carry = rot & 0b00000001;
    let prev_carry = cpu.registers[F] & C_FLAG;

    set_flags(cpu, C_FLAG, carry == 0b00000001);

    rot >>= 1;
    rot |= prev_carry << 3;
    cpu.registers[reg] = rot;

    set_flags(cpu, N_FLAG | H_FLAG, false);
    set_flags(cpu, Z_FLAG, rot == 0);

    cpu.cycles += 8;
}

pub fn rr_b(cpu: &mut CPU) {
    rr_reg(cpu, B);
}

pub fn rr_c(cpu: &mut CPU) {
    rr_reg(cpu, C);
}

pub fn rr_d(cpu: &mut CPU) {
    rr_reg(cpu, D);
}

pub fn rr_e(cpu: &mut CPU) {
    rr_reg(cpu, E);
}

pub fn rr_h(cpu: &mut CPU) {
    rr_reg(cpu, H);
}

pub fn rr_l(cpu: &mut CPU) {
    rr_reg(cpu, L);
}

pub fn rr_hlind(cpu: &mut CPU) {
    // TODO es posible que esto este mal
    let dir = cpu.registers[L] as u16 + cpu.registers[H] as u16 * 0x100;
    let mut rot = cpu.mem.read(dir as usize);
    let carry = rot & 0b00000001;
    let prev_carry = cpu.registers[F] & C_FLAG;

    set_flags(cpu, C_FLAG, carry == 0b00000001);

    rot >>= 1;
    rot |= prev_carry << 3;
    cpu.mem.write(dir as usize, rot);
    cpu.registers[A] = rot;

    set_flags(cpu, N_FLAG | H_FLAG, false);
    set_flags(cpu, Z_FLAG, rot == 0);

    cpu.cycles += 16;
}

pub fn rr_a(cpu: &mut CPU) {
    rr_reg(cpu, A);
}

fn sla_reg(cpu: &mut CPU, reg: usize) {
    let rot = cpu.registers[reg].wrapping_shl(1);

    set_flags(cpu, C_FLAG, cpu.registers[reg] & 0b10000000 == 0b10000000);
    
    cpu.registers[reg] = rot;
    set_flags(cpu, N_FLAG | Z_FLAG, false);
    set_flags(cpu, Z_FLAG, rot == 0);

    cpu.cycles += 8;
}

pub fn sla_b(cpu: &mut CPU) {
    sla_reg(cpu, B);
}

pub fn sla_c(cpu: &mut CPU) {
    sla_reg(cpu, C);
}

pub fn sla_d(cpu: &mut CPU) {
    sla_reg(cpu, D);
}

pub fn sla_e(cpu: &mut CPU) {
    sla_reg(cpu, E);
}

pub fn sla_h(cpu: &mut CPU) {
    sla_reg(cpu, H);
}

pub fn sla_l(cpu: &mut CPU) {
    sla_reg(cpu, L);
}

pub fn sla_hlind(cpu: &mut CPU) {
    let dir = (cpu.registers[L] as u16 + cpu.registers[H] as u16 * 0x100) as usize;
    let rot = cpu.mem.read(dir).wrapping_shl(1);

    set_flags(cpu, C_FLAG, cpu.mem.read(dir) & 0b10000000 == 0b10000000);

    cpu.mem.write(dir, rot);
    set_flags(cpu, N_FLAG | Z_FLAG, false);
    set_flags(cpu, Z_FLAG, rot == 0);

    cpu.cycles += 16;
}

pub fn sla_a(cpu: &mut CPU) {
    sla_reg(cpu, A);
}

fn sra_reg(cpu: &mut CPU, reg: usize) {
    let rot = cpu.registers[reg].wrapping_shr(1);
    let msb = cpu.registers[reg] & 0b1000000; 

    set_flags(cpu, C_FLAG, cpu.registers[reg] & 0b00000001 == 0b00000001);

    cpu.registers[reg] = rot | msb;
    set_flags(cpu, Z_FLAG, cpu.registers[reg] == 0);
    set_flags(cpu, N_FLAG | H_FLAG, false);

    cpu.cycles += 8;
}

pub fn sra_b(cpu: &mut CPU) {
    sra_reg(cpu, B);
}

pub fn sra_c(cpu: &mut CPU) {
    sra_reg(cpu, C);
}

pub fn sra_d(cpu: &mut CPU) {
    sra_reg(cpu, D);
}

pub fn sra_e(cpu: &mut CPU) {
    sra_reg(cpu, E);
}

pub fn sra_h(cpu: &mut CPU) {
    sra_reg(cpu, H);
}

pub fn sra_l(cpu: &mut CPU) {
    sra_reg(cpu, L);
}

pub fn sra_hlind(cpu: &mut CPU) {
    let dir = (cpu.registers[L] as u16 + cpu.registers[H] as u16 * 0x100) as usize;
    let rot = cpu.mem.read(dir).wrapping_shr(1);
    let msb = cpu.mem.read(dir) & 0b1000000; 
    

    set_flags(cpu, C_FLAG, cpu.mem.read(dir) & 0b00000001 == 0b00000001);

    cpu.mem.write(dir, rot | msb);
    set_flags(cpu, Z_FLAG, cpu.mem.read(dir) == 0);
    set_flags(cpu, N_FLAG | H_FLAG, false);

    
    cpu.cycles += 16;
}

pub fn sra_a(cpu: &mut CPU) {
    sra_reg(cpu, A);
}

fn swap_reg(cpu: &mut CPU, reg: usize) {
    let upper = cpu.registers[reg] & 0xF0;
    let lower = cpu.registers[reg] & 0x0F;
    cpu.registers[reg] = (upper >> 4) + (lower << 4);

    set_flags(cpu, Z_FLAG, cpu.registers[reg] == 0);
    set_flags(cpu, N_FLAG | H_FLAG | C_FLAG, false);

    cpu.cycles += 8;
}

pub fn swap_b(cpu: &mut CPU) {
    swap_reg(cpu, B);
}

pub fn swap_c(cpu: &mut CPU) {
    swap_reg(cpu, C);
}

pub fn swap_d(cpu: &mut CPU) {
    swap_reg(cpu, D);
}

pub fn swap_e(cpu: &mut CPU) {
    swap_reg(cpu, E);
}

pub fn swap_h(cpu: &mut CPU) {
    swap_reg(cpu, H);
}

pub fn swap_l(cpu: &mut CPU) {
    swap_reg(cpu, L);
}

pub fn swap_hlind(cpu: &mut CPU) {
    let dir = (cpu.registers[L] as u16 + cpu.registers[H] as u16 * 0x100) as usize;

    let upper = cpu.mem.read(dir) & 0xF0;
    let lower = cpu.mem.read(dir) & 0x0F;
    cpu.mem.write(dir, (upper >> 4) + (lower << 4));

    set_flags(cpu, Z_FLAG, cpu.mem.read(dir) == 0);
    set_flags(cpu, N_FLAG | H_FLAG | C_FLAG, false);

    cpu.cycles += 16;
}

pub fn swap_a(cpu: &mut CPU) {
    swap_reg(cpu, A);
}

fn srl_reg(cpu: &mut CPU, reg: usize) {
    let rot = cpu.registers[reg].wrapping_shr(1);

    set_flags(cpu, C_FLAG, cpu.registers[reg] & 0b10000000 == 0b10000000);
    set_flags(cpu, Z_FLAG, rot == 0);
    set_flags(cpu, N_FLAG | H_FLAG, false);
    cpu.registers[reg] = rot;

    cpu.cycles += 8;
}

pub fn srl_b(cpu: &mut CPU) {
    srl_reg(cpu, B);
}

pub fn srl_c(cpu: &mut CPU) {
    srl_reg(cpu, C);
}

pub fn srl_d(cpu: &mut CPU) {
    srl_reg(cpu, D);
}

pub fn srl_e(cpu: &mut CPU) {
    srl_reg(cpu, E);
}

pub fn srl_h(cpu: &mut CPU) {
    srl_reg(cpu, H);
}

pub fn srl_l(cpu: &mut CPU) {
    srl_reg(cpu, L);
}

pub fn srl_hlind(cpu: &mut CPU) {
    let dir = (cpu.registers[L] as u16 + cpu.registers[H] as u16 * 0x100) as usize;
    let rot = cpu.mem.read(dir).wrapping_shr(1);
    

    set_flags(cpu, C_FLAG, cpu.mem.read(dir) & 0b10000000 == 0b10000000);
    set_flags(cpu, Z_FLAG, rot == 0);
    set_flags(cpu, N_FLAG | H_FLAG, false);

    cpu.mem.write(dir, rot);

    cpu.cycles += 16;
}

pub fn srl_a(cpu: &mut CPU) {
    srl_reg(cpu, A);
}

fn bit_pos_reg(cpu: &mut CPU, reg: usize, b: u8) {
    set_flags(cpu, Z_FLAG, cpu.registers[reg] & b == 0);
    set_flags(cpu, N_FLAG, false);
    set_flags(cpu, H_FLAG, true);

    cpu.cycles += 8;
}

fn bit_pos_hlind(cpu: &mut CPU, b: u8) {
    let dir = (cpu.registers[L] as u16 + cpu.registers[H] as u16 * 0x100) as usize;

    set_flags(cpu, Z_FLAG, cpu.mem.read(dir) & b == 0);
    set_flags(cpu, N_FLAG, false);
    set_flags(cpu, H_FLAG, true);
    
    cpu.cycles += 8;
}

pub fn bit_0_b(cpu: &mut CPU) {
    bit_pos_reg(cpu, B, 0b00000001);
}

pub fn bit_0_c(cpu: &mut CPU) {
    bit_pos_reg(cpu, C, 0b00000001);
}

pub fn bit_0_d(cpu: &mut CPU) {
    bit_pos_reg(cpu, D, 0b00000001);
}

pub fn bit_0_e(cpu: &mut CPU) {
    bit_pos_reg(cpu, E, 0b00000001);
}

pub fn bit_0_h(cpu: &mut CPU) {
    bit_pos_reg(cpu, H, 0b00000001);
}

pub fn bit_0_l(cpu: &mut CPU) {
    bit_pos_reg(cpu, L, 0b00000001);
}

pub fn bit_0_hlind(cpu: &mut CPU) {
    bit_pos_hlind(cpu, 0b00000001);
}

pub fn bit_0_a(cpu: &mut CPU) {
    bit_pos_reg(cpu, A, 0b00000001);
}

pub fn bit_1_b(cpu: &mut CPU) {
    bit_pos_reg(cpu, B, 0b00000010);
}

pub fn bit_1_c(cpu: &mut CPU) {
    bit_pos_reg(cpu, C, 0b00000010);
}

pub fn bit_1_d(cpu: &mut CPU) {
    bit_pos_reg(cpu, D, 0b00000010);
}

pub fn bit_1_e(cpu: &mut CPU) {
    bit_pos_reg(cpu, E, 0b00000010);
}

pub fn bit_1_h(cpu: &mut CPU) {
    bit_pos_reg(cpu, H, 0b00000010);
}

pub fn bit_1_l(cpu: &mut CPU) {
    bit_pos_reg(cpu, L, 0b00000010);
}

pub fn bit_1_hlind(cpu: &mut CPU) {
    bit_pos_hlind(cpu, 0b00000010);
}

pub fn bit_1_a(cpu: &mut CPU) {
    bit_pos_reg(cpu, A, 0b00000010);
}

pub fn bit_2_b(cpu: &mut CPU) {
    bit_pos_reg(cpu, B, 0b00000100);
}

pub fn bit_2_c(cpu: &mut CPU) {
    bit_pos_reg(cpu, C, 0b00000100);
}

pub fn bit_2_d(cpu: &mut CPU) {
    bit_pos_reg(cpu, D, 0b00000100);
}

pub fn bit_2_e(cpu: &mut CPU) {
    bit_pos_reg(cpu, E, 0b00000100);
}

pub fn bit_2_h(cpu: &mut CPU) {
    bit_pos_reg(cpu, H, 0b00000100);
}

pub fn bit_2_l(cpu: &mut CPU) {
    bit_pos_reg(cpu, L, 0b00000100);
}

pub fn bit_2_hlind(cpu: &mut CPU) {
    bit_pos_hlind(cpu, 0b00000100);
}

pub fn bit_2_a(cpu: &mut CPU) {
    bit_pos_reg(cpu, A, 0b00000100);
}

pub fn bit_3_b(cpu: &mut CPU) {
    bit_pos_reg(cpu, B, 0b00001000);
}

pub fn bit_3_c(cpu: &mut CPU) {
    bit_pos_reg(cpu, C, 0b00001000);
}

pub fn bit_3_d(cpu: &mut CPU) {
    bit_pos_reg(cpu, D, 0b00001000);
}

pub fn bit_3_e(cpu: &mut CPU) {
    bit_pos_reg(cpu, E, 0b00001000);
}

pub fn bit_3_h(cpu: &mut CPU) {
    bit_pos_reg(cpu, H, 0b00001000);
}

pub fn bit_3_l(cpu: &mut CPU) {
    bit_pos_reg(cpu, L, 0b00001000);
}

pub fn bit_3_hlind(cpu: &mut CPU) {
    bit_pos_hlind(cpu, 0b00001000);
}

pub fn bit_3_a(cpu: &mut CPU) {
    bit_pos_reg(cpu, A, 0b00001000);
}

pub fn bit_4_b(cpu: &mut CPU) {
    bit_pos_reg(cpu, B, 0b00010000);
}

pub fn bit_4_c(cpu: &mut CPU) {
    bit_pos_reg(cpu, C, 0b00010000);
}

pub fn bit_4_d(cpu: &mut CPU) {
    bit_pos_reg(cpu, D, 0b00010000);
}

pub fn bit_4_e(cpu: &mut CPU) {
    bit_pos_reg(cpu, E, 0b00010000);
}

pub fn bit_4_h(cpu: &mut CPU) {
    bit_pos_reg(cpu, H, 0b00010000);
}

pub fn bit_4_l(cpu: &mut CPU) {
    bit_pos_reg(cpu, L, 0b00010000);
}

pub fn bit_4_hlind(cpu: &mut CPU) {
    bit_pos_hlind(cpu, 0b00010000);
}

pub fn bit_4_a(cpu: &mut CPU) {
    bit_pos_reg(cpu, A, 0b00010000);
}

pub fn bit_5_b(cpu: &mut CPU) {
    bit_pos_reg(cpu, B, 0b00100000);
}

pub fn bit_5_c(cpu: &mut CPU) {
    bit_pos_reg(cpu, C, 0b00100000);
}

pub fn bit_5_d(cpu: &mut CPU) {
    bit_pos_reg(cpu, D, 0b00100000);
}

pub fn bit_5_e(cpu: &mut CPU) {
    bit_pos_reg(cpu, E, 0b00100000);
}

pub fn bit_5_h(cpu: &mut CPU) {
    bit_pos_reg(cpu, H, 0b00100000);
}

pub fn bit_5_l(cpu: &mut CPU) {
    bit_pos_reg(cpu, L, 0b00100000);
}

pub fn bit_5_hlind(cpu: &mut CPU) {
    bit_pos_hlind(cpu, 0b00100000);
}

pub fn bit_5_a(cpu: &mut CPU) {
    bit_pos_reg(cpu, A, 0b00100000);
}

pub fn bit_6_b(cpu: &mut CPU) {
    bit_pos_reg(cpu, B, 0b01000000);
}

pub fn bit_6_c(cpu: &mut CPU) {
    bit_pos_reg(cpu, C, 0b01000000);
}

pub fn bit_6_d(cpu: &mut CPU) {
    bit_pos_reg(cpu, D, 0b01000000);
}

pub fn bit_6_e(cpu: &mut CPU) {
    bit_pos_reg(cpu, E, 0b01000000);
}

pub fn bit_6_h(cpu: &mut CPU) {
    bit_pos_reg(cpu, H, 0b01000000);
}

pub fn bit_6_l(cpu: &mut CPU) {
    bit_pos_reg(cpu, L, 0b01000000);
}

pub fn bit_6_hlind(cpu: &mut CPU) {
    bit_pos_hlind(cpu, 0b01000000);
}

pub fn bit_6_a(cpu: &mut CPU) {
    bit_pos_reg(cpu, A, 0b01000000);
}

pub fn bit_7_b(cpu: &mut CPU) {
    bit_pos_reg(cpu, B, 0b10000000);
}

pub fn bit_7_c(cpu: &mut CPU) {
    bit_pos_reg(cpu, C, 0b10000000);
}

pub fn bit_7_d(cpu: &mut CPU) {
    bit_pos_reg(cpu, D, 0b10000000);
}

pub fn bit_7_e(cpu: &mut CPU) {
    bit_pos_reg(cpu, E, 0b10000000);
}

pub fn bit_7_h(cpu: &mut CPU) {
    bit_pos_reg(cpu, H, 0b10000000);
}

pub fn bit_7_l(cpu: &mut CPU) {
    bit_pos_reg(cpu, L, 0b10000000);
}

pub fn bit_7_hlind(cpu: &mut CPU) {
    bit_pos_hlind(cpu, 0b10000000);
}

pub fn bit_7_a(cpu: &mut CPU) {
    bit_pos_reg(cpu, A, 0b10000000);
}

fn res_pos_reg(cpu: &mut CPU, reg: usize, b: u8) {
    cpu.registers[reg] &= !b;
    cpu.cycles += 8;
}

fn res_pos_hlind(cpu: &mut CPU, b: u8) {
    let dir = (cpu.registers[L] as u16 + cpu.registers[H] as u16 * 0x100) as usize;
    let val = cpu.mem.read(dir) & !b;
    cpu.mem.write(dir, val);
    cpu.cycles += 16;
}

pub fn res_0_b(cpu: &mut CPU) {
    res_pos_reg(cpu, B, 0b00000001);
}

pub fn res_0_c(cpu: &mut CPU) {
    res_pos_reg(cpu, C, 0b00000001);
}

pub fn res_0_d(cpu: &mut CPU) {
    res_pos_reg(cpu, D, 0b00000001);
}

pub fn res_0_e(cpu: &mut CPU) {
    res_pos_reg(cpu, E, 0b00000001);
}

pub fn res_0_h(cpu: &mut CPU) {
    res_pos_reg(cpu, H, 0b00000001);
}

pub fn res_0_l(cpu: &mut CPU) {
    res_pos_reg(cpu, L, 0b00000001);
}

pub fn res_0_hlind(cpu: &mut CPU) {
    res_pos_hlind(cpu, 0b00000001);
}

pub fn res_0_a(cpu: &mut CPU) {
    res_pos_reg(cpu, A, 0b00000001);
}

pub fn res_1_b(cpu: &mut CPU) {
    res_pos_reg(cpu, B, 0b00000010);
}

pub fn res_1_c(cpu: &mut CPU) {
    res_pos_reg(cpu, C, 0b00000010);
}

pub fn res_1_d(cpu: &mut CPU) {
    res_pos_reg(cpu, D, 0b00000010);
}

pub fn res_1_e(cpu: &mut CPU) {
    res_pos_reg(cpu, E, 0b00000010);
}

pub fn res_1_h(cpu: &mut CPU) {
    res_pos_reg(cpu, H, 0b00000010);
}

pub fn res_1_l(cpu: &mut CPU) {
    res_pos_reg(cpu, L, 0b00000010);
}

pub fn res_1_hlind(cpu: &mut CPU) {
    res_pos_hlind(cpu, 0b00000010);
}

pub fn res_1_a(cpu: &mut CPU) {
    res_pos_reg(cpu, A, 0b00000010);
}

pub fn res_2_b(cpu: &mut CPU) {
    res_pos_reg(cpu, B, 0b00000100);
}

pub fn res_2_c(cpu: &mut CPU) {
    res_pos_reg(cpu, C, 0b00000100);
}

pub fn res_2_d(cpu: &mut CPU) {
    res_pos_reg(cpu, D, 0b00000100);
}

pub fn res_2_e(cpu: &mut CPU) {
    res_pos_reg(cpu, E, 0b00000100);
}

pub fn res_2_h(cpu: &mut CPU) {
    res_pos_reg(cpu, H, 0b00000100);
}

pub fn res_2_l(cpu: &mut CPU) {
    res_pos_reg(cpu, L, 0b00000100);
}

pub fn res_2_hlind(cpu: &mut CPU) {
    res_pos_hlind(cpu, 0b00000100);
}

pub fn res_2_a(cpu: &mut CPU) {
    res_pos_reg(cpu, A, 0b00000100);
}

pub fn res_3_b(cpu: &mut CPU) {
    res_pos_reg(cpu, B, 0b00001000);
}

pub fn res_3_c(cpu: &mut CPU) {
    res_pos_reg(cpu, C, 0b00001000);
}

pub fn res_3_d(cpu: &mut CPU) {
    res_pos_reg(cpu, D, 0b00001000);
}

pub fn res_3_e(cpu: &mut CPU) {
    res_pos_reg(cpu, E, 0b00001000);
}

pub fn res_3_h(cpu: &mut CPU) {
    res_pos_reg(cpu, H, 0b00001000);
}

pub fn res_3_l(cpu: &mut CPU) {
    res_pos_reg(cpu, L, 0b00001000);
}

pub fn res_3_hlind(cpu: &mut CPU) {
    res_pos_hlind(cpu, 0b00001000);
}

pub fn res_3_a(cpu: &mut CPU) {
    res_pos_reg(cpu, A, 0b00001000);
}

pub fn res_4_b(cpu: &mut CPU) {
    res_pos_reg(cpu, B, 0b00010000);
}

pub fn res_4_c(cpu: &mut CPU) {
    res_pos_reg(cpu, C, 0b00010000);
}

pub fn res_4_d(cpu: &mut CPU) {
    res_pos_reg(cpu, D, 0b00010000);
}

pub fn res_4_e(cpu: &mut CPU) {
    res_pos_reg(cpu, E, 0b00010000);
}

pub fn res_4_h(cpu: &mut CPU) {
    res_pos_reg(cpu, H, 0b00010000);
}

pub fn res_4_l(cpu: &mut CPU) {
    res_pos_reg(cpu, L, 0b00010000);
}

pub fn res_4_hlind(cpu: &mut CPU) {
    res_pos_hlind(cpu, 0b00010000);
}

pub fn res_4_a(cpu: &mut CPU) {
    res_pos_reg(cpu, A, 0b00010000);
}

pub fn res_5_b(cpu: &mut CPU) {
    res_pos_reg(cpu, B, 0b00100000);
}

pub fn res_5_c(cpu: &mut CPU) {
    res_pos_reg(cpu, C, 0b00100000);
}

pub fn res_5_d(cpu: &mut CPU) {
    res_pos_reg(cpu, D, 0b00100000);
}

pub fn res_5_e(cpu: &mut CPU) {
    res_pos_reg(cpu, E, 0b00100000);
}

pub fn res_5_h(cpu: &mut CPU) {
    res_pos_reg(cpu, H, 0b00100000);
}

pub fn res_5_l(cpu: &mut CPU) {
    res_pos_reg(cpu, L, 0b00100000);
}

pub fn res_5_hlind(cpu: &mut CPU) {
    res_pos_hlind(cpu, 0b00100000);
}

pub fn res_5_a(cpu: &mut CPU) {
    res_pos_reg(cpu, A, 0b00100000);
}

pub fn res_6_b(cpu: &mut CPU) {
    res_pos_reg(cpu, B, 0b01000000);
}

pub fn res_6_c(cpu: &mut CPU) {
    res_pos_reg(cpu, C, 0b01000000);
}

pub fn res_6_d(cpu: &mut CPU) {
    res_pos_reg(cpu, D, 0b01000000);
}

pub fn res_6_e(cpu: &mut CPU) {
    res_pos_reg(cpu, E, 0b01000000);
}

pub fn res_6_h(cpu: &mut CPU) {
    res_pos_reg(cpu, H, 0b01000000);
}

pub fn res_6_l(cpu: &mut CPU) {
    res_pos_reg(cpu, L, 0b01000000);
}

pub fn res_6_hlind(cpu: &mut CPU) {
    res_pos_hlind(cpu, 0b01000000);
}

pub fn res_6_a(cpu: &mut CPU) {
    res_pos_reg(cpu, A, 0b01000000);
}

pub fn res_7_b(cpu: &mut CPU) {
    res_pos_reg(cpu, B, 0b10000000);
}

pub fn res_7_c(cpu: &mut CPU) {
    res_pos_reg(cpu, C, 0b10000000);
}

pub fn res_7_d(cpu: &mut CPU) {
    res_pos_reg(cpu, D, 0b10000000);
}

pub fn res_7_e(cpu: &mut CPU) {
    res_pos_reg(cpu, E, 0b10000000);
}

pub fn res_7_h(cpu: &mut CPU) {
    res_pos_reg(cpu, H, 0b10000000);
}

pub fn res_7_l(cpu: &mut CPU) {
    res_pos_reg(cpu, L, 0b10000000);
}

pub fn res_7_hlind(cpu: &mut CPU) {
    res_pos_hlind(cpu, 0b10000000);
}

pub fn res_7_a(cpu: &mut CPU) {
    res_pos_reg(cpu, A, 0b10000000);
}

fn set_pos_reg(cpu: &mut CPU, reg: usize, b: u8) {
    cpu.registers[reg] |= b;
    cpu.cycles += 8;
}

fn set_pos_hlind(cpu: &mut CPU, b: u8) {
    let dir = (cpu.registers[L] as u16 + cpu.registers[H] as u16 * 0x100) as usize;
    let val = cpu.mem.read(dir) | b;
    cpu.mem.write(dir, val);
    cpu.cycles += 16;
}

pub fn set_0_b(cpu: &mut CPU) {
    set_pos_reg(cpu, B, 0b00000001);
}

pub fn set_0_c(cpu: &mut CPU) {
    set_pos_reg(cpu, C, 0b00000001);
}

pub fn set_0_d(cpu: &mut CPU) {
    set_pos_reg(cpu, D, 0b00000001);
}

pub fn set_0_e(cpu: &mut CPU) {
    set_pos_reg(cpu, E, 0b00000001);
}

pub fn set_0_h(cpu: &mut CPU) {
    set_pos_reg(cpu, H, 0b00000001);
}

pub fn set_0_l(cpu: &mut CPU) {
    set_pos_reg(cpu, L, 0b00000001);
}

pub fn set_0_hlind(cpu: &mut CPU) {
    set_pos_hlind(cpu, 0b00000001);
}

pub fn set_0_a(cpu: &mut CPU) {
    set_pos_reg(cpu, A, 0b00000001);
}

pub fn set_1_b(cpu: &mut CPU) {
    set_pos_reg(cpu, B, 0b00000010);
}

pub fn set_1_c(cpu: &mut CPU) {
    set_pos_reg(cpu, C, 0b00000010);
}

pub fn set_1_d(cpu: &mut CPU) {
    set_pos_reg(cpu, D, 0b00000010);
}

pub fn set_1_e(cpu: &mut CPU) {
    set_pos_reg(cpu, E, 0b00000010);
}

pub fn set_1_h(cpu: &mut CPU) {
    set_pos_reg(cpu, H, 0b00000010);
}

pub fn set_1_l(cpu: &mut CPU) {
    set_pos_reg(cpu, L, 0b00000010);
}

pub fn set_1_hlind(cpu: &mut CPU) {
    set_pos_hlind(cpu, 0b00000010);
}

pub fn set_1_a(cpu: &mut CPU) {
    set_pos_reg(cpu, A, 0b00000010);
}

pub fn set_2_b(cpu: &mut CPU) {
    set_pos_reg(cpu, B, 0b00000100);
}

pub fn set_2_c(cpu: &mut CPU) {
    set_pos_reg(cpu, C, 0b00000100);
}

pub fn set_2_d(cpu: &mut CPU) {
    set_pos_reg(cpu, D, 0b00000100);
}

pub fn set_2_e(cpu: &mut CPU) {
    set_pos_reg(cpu, E, 0b00000100);
}

pub fn set_2_h(cpu: &mut CPU) {
    set_pos_reg(cpu, H, 0b00000100);
}

pub fn set_2_l(cpu: &mut CPU) {
    set_pos_reg(cpu, L, 0b00000100);
}

pub fn set_2_hlind(cpu: &mut CPU) {
    set_pos_hlind(cpu, 0b00000100);
}

pub fn set_2_a(cpu: &mut CPU) {
    set_pos_reg(cpu, A, 0b00000100);
}

pub fn set_3_b(cpu: &mut CPU) {
    set_pos_reg(cpu, B, 0b00001000);
}

pub fn set_3_c(cpu: &mut CPU) {
    set_pos_reg(cpu, C, 0b00001000);
}

pub fn set_3_d(cpu: &mut CPU) {
    set_pos_reg(cpu, D, 0b00001000);
}

pub fn set_3_e(cpu: &mut CPU) {
    set_pos_reg(cpu, E, 0b00001000);
}

pub fn set_3_h(cpu: &mut CPU) {
    set_pos_reg(cpu, H, 0b00001000);
}

pub fn set_3_l(cpu: &mut CPU) {
    set_pos_reg(cpu, L, 0b00001000);
}

pub fn set_3_hlind(cpu: &mut CPU) {
    set_pos_hlind(cpu, 0b00001000);
}

pub fn set_3_a(cpu: &mut CPU) {
    set_pos_reg(cpu, A, 0b00001000);
}

pub fn set_4_b(cpu: &mut CPU) {
    set_pos_reg(cpu, B, 0b00010000);
}

pub fn set_4_c(cpu: &mut CPU) {
    set_pos_reg(cpu, C, 0b00010000);
}

pub fn set_4_d(cpu: &mut CPU) {
    set_pos_reg(cpu, D, 0b00010000);
}

pub fn set_4_e(cpu: &mut CPU) {
    set_pos_reg(cpu, E, 0b00010000);
}

pub fn set_4_h(cpu: &mut CPU) {
    set_pos_reg(cpu, H, 0b00010000);
}

pub fn set_4_l(cpu: &mut CPU) {
    set_pos_reg(cpu, L, 0b00010000);
}

pub fn set_4_hlind(cpu: &mut CPU) {
    set_pos_hlind(cpu, 0b00010000);
}

pub fn set_4_a(cpu: &mut CPU) {
    set_pos_reg(cpu, A, 0b00010000);
}

pub fn set_5_b(cpu: &mut CPU) {
    set_pos_reg(cpu, B, 0b00100000);
}

pub fn set_5_c(cpu: &mut CPU) {
    set_pos_reg(cpu, C, 0b00100000);
}

pub fn set_5_d(cpu: &mut CPU) {
    set_pos_reg(cpu, D, 0b00100000);
}

pub fn set_5_e(cpu: &mut CPU) {
    set_pos_reg(cpu, E, 0b00100000);
}

pub fn set_5_h(cpu: &mut CPU) {
    set_pos_reg(cpu, H, 0b00100000);
}

pub fn set_5_l(cpu: &mut CPU) {
    set_pos_reg(cpu, L, 0b00100000);
}

pub fn set_5_hlind(cpu: &mut CPU) {
    set_pos_hlind(cpu, 0b00100000);
}

pub fn set_5_a(cpu: &mut CPU) {
    set_pos_reg(cpu, A, 0b00100000);
}

pub fn set_6_b(cpu: &mut CPU) {
    set_pos_reg(cpu, B, 0b01000000);
}

pub fn set_6_c(cpu: &mut CPU) {
    set_pos_reg(cpu, C, 0b01000000);
}

pub fn set_6_d(cpu: &mut CPU) {
    set_pos_reg(cpu, D, 0b01000000);
}

pub fn set_6_e(cpu: &mut CPU) {
    set_pos_reg(cpu, E, 0b01000000);
}

pub fn set_6_h(cpu: &mut CPU) {
    set_pos_reg(cpu, H, 0b01000000);
}

pub fn set_6_l(cpu: &mut CPU) {
    set_pos_reg(cpu, L, 0b01000000);
}

pub fn set_6_hlind(cpu: &mut CPU) {
    set_pos_hlind(cpu, 0b01000000);
}

pub fn set_6_a(cpu: &mut CPU) {
    set_pos_reg(cpu, A, 0b01000000);
}

pub fn set_7_b(cpu: &mut CPU) {
    set_pos_reg(cpu, B, 0b10000000);
}

pub fn set_7_c(cpu: &mut CPU) {
    set_pos_reg(cpu, C, 0b10000000);
}

pub fn set_7_d(cpu: &mut CPU) {
    set_pos_reg(cpu, D, 0b10000000);
}

pub fn set_7_e(cpu: &mut CPU) {
    set_pos_reg(cpu, E, 0b10000000);
}

pub fn set_7_h(cpu: &mut CPU) {
    set_pos_reg(cpu, H, 0b10000000);
}

pub fn set_7_l(cpu: &mut CPU) {
    set_pos_reg(cpu, L, 0b10000000);
}

pub fn set_7_hlind(cpu: &mut CPU) {
    set_pos_hlind(cpu, 0b10000000);
}

pub fn set_7_a(cpu: &mut CPU) {
    set_pos_reg(cpu, A, 0b10000000);
}

// CONTROL/BR
pub fn nop(cpu: &mut CPU) {
    cpu.cycles += 4;
}

pub fn stop(cpu: &mut CPU) {
    cpu.stop = true;
    cpu.pc += 1;
    cpu.cycles += 4;
}

pub fn halt(cpu: &mut CPU) {
    cpu.halt = true;
    cpu.cycles += 4;
}

pub fn di(cpu: &mut CPU) {
    cpu.cycles_di_ie = 1;
    cpu.ime_temp = false;
    cpu.cycles += 4;
}

pub fn ei(cpu: &mut CPU) {
    cpu.cycles_di_ie = 1;
    cpu.ime_temp = true;
    cpu.cycles += 4; 
}

pub fn cb(cpu: &mut CPU) {
    let op = cpu.fetch();
    cpu.decode_cb(op);
}

// CONTROL/MISC

pub fn jr_i8(cpu: &mut CPU) {
    let val = cpu.fetch() as i8;
    let mut pc = cpu.pc as i16;
    pc = pc.overflowing_add(val as i16).0;
    cpu.pc = pc as u16;
    cpu.cycles += 12;
}

fn jr_flag_i8(cpu: &mut CPU, flag: bool) {
    let val = cpu.fetch() as i8;
    if flag {
        let mut pc = cpu.pc as i16;
        pc = pc.overflowing_add(val as i16).0;
        cpu.pc = pc as u16;
        cpu.cycles += 12;
        return;
    }
    cpu.cycles += 8;
}

pub fn jr_nz_i8(cpu: &mut CPU) {
    let flag = get_zero(cpu) != 0;
    jr_flag_i8(cpu, !flag);
}

pub fn jr_z_i8(cpu: &mut CPU) {
    let flag = get_zero(cpu) != 0;
    jr_flag_i8(cpu, flag);
}

pub fn jr_nc_i8(cpu: &mut CPU) {
    let flag = get_carry(cpu) != 0;
    jr_flag_i8(cpu, !flag);
}

pub fn jr_c_i8(cpu: &mut CPU) {
    let flag = get_carry(cpu) != 0; 
    jr_flag_i8(cpu, flag);
}

fn ret_flag(cpu: &mut CPU, flag: bool) {
    if flag {
        let pc_low = cpu.mem.read(cpu.sp as usize);
        cpu.sp = cpu.sp.wrapping_add(1);
        let pc_high = cpu.mem.read(cpu.sp as usize);
        cpu.sp = cpu.sp.wrapping_add(1);
        cpu.pc = pc_low as u16 + pc_high as u16 * 0x100;
        cpu.cycles += 20;
        return;
    }
    cpu.cycles += 8;
}

pub fn ret_nz(cpu: &mut CPU) {
    let flag = get_zero(cpu) != 0;
    ret_flag(cpu, !flag);
}

pub fn ret_z(cpu: &mut CPU) {
    let flag = get_zero(cpu) != 0;
    ret_flag(cpu, flag);
}

pub fn ret_nc(cpu: &mut CPU) {
    let flag = get_carry(cpu) != 0;
    ret_flag(cpu, !flag);
}

pub fn ret_c(cpu: &mut CPU) {
    let flag = get_carry(cpu) != 0;
    ret_flag(cpu, flag);
}

pub fn ret(cpu: &mut CPU) {
    let pc_low = cpu.mem.read(cpu.sp as usize);
    cpu.sp = cpu.sp.wrapping_add(1);
    let pc_high = cpu.mem.read(cpu.sp as usize);
    cpu.sp = cpu.sp.wrapping_add(1);
    cpu.pc = pc_low as u16 + pc_high as u16 * 0x100;
    cpu.cycles += 16;
}

pub fn reti(cpu: &mut CPU) {
    let pc_low = cpu.mem.read(cpu.sp as usize);
    cpu.sp = cpu.sp.wrapping_add(1);
    let pc_high = cpu.mem.read(cpu.sp as usize);
    cpu.sp = cpu.sp.wrapping_add(1);
    cpu.pc = pc_low as u16 + pc_high as u16 * 0x100;
    cpu.ime = true;
    cpu.cycles += 16;
}

fn jp_flag_u16(cpu: &mut CPU, flag: bool) {
    let pc_low = cpu.fetch() as usize;
    let pc_high = cpu.fetch() as usize;

    if flag {
        cpu.pc = pc_low as u16 + pc_high as u16 * 0x100;
        cpu.cycles += 16;
        return;
    }
    cpu.cycles += 12;
}

pub fn jp_nz_u16(cpu: &mut CPU) {
    let flag = get_zero(cpu) != 0;
    jp_flag_u16(cpu, !flag);
}

pub fn jp_z_u16(cpu: &mut CPU) {
    let flag = get_zero(cpu) != 0;
    jp_flag_u16(cpu, flag);
}

pub fn jp_nc_u16(cpu: &mut CPU) {
    let flag = get_carry(cpu) != 0;
    jp_flag_u16(cpu, !flag);
}

pub fn jp_c_u16(cpu: &mut CPU) {
    let flag = get_carry(cpu) != 0;
    jp_flag_u16(cpu, flag);
}

pub fn jp_u16(cpu: &mut CPU) {
    let pc_low = cpu.fetch() as usize;
    let pc_high = cpu.fetch() as usize;
    cpu.pc = pc_low as u16 + pc_high as u16 * 0x100;
    cpu.cycles += 16;
}

pub fn jp_hl(cpu: &mut CPU) {
    let dir = cpu.registers[L] as u16 + cpu.registers[H] as u16 * 0x100;
    cpu.pc = dir;
    cpu.cycles += 4;
}

fn call_flag_u16(cpu: &mut CPU, flag: bool) {
    let pc_low = cpu.fetch() as usize;
    let pc_high = cpu.fetch() as usize;

    if flag {
        let pc_ant = cpu.pc;
        cpu.pc = pc_low as u16 + pc_high as u16 * 0x100;

        cpu.sp = cpu.sp.wrapping_sub(1);
        cpu.mem.write(cpu.sp as usize, (pc_ant / 0x100) as u8);
        cpu.sp = cpu.sp.wrapping_sub(1);
        cpu.mem.write(cpu.sp as usize, pc_ant as u8);

        cpu.cycles += 24;
        return;
    }
    cpu.cycles += 12;
}

pub fn call_nz_u16(cpu: &mut CPU) {
    let flag = get_zero(cpu) != 0;
    call_flag_u16(cpu, !flag);
}

pub fn call_z_u16(cpu: &mut CPU) {
    let flag = get_zero(cpu) != 0;
    call_flag_u16(cpu, flag);
}

pub fn call_nc_u16(cpu: &mut CPU) {
    let flag = get_carry(cpu) != 0;
    call_flag_u16(cpu, !flag);
}

pub fn call_c_u16(cpu: &mut CPU) {
    let flag = get_carry(cpu) != 0;
    call_flag_u16(cpu, flag);
}

pub fn call_u16(cpu: &mut CPU) {
    let pc_low = cpu.fetch() as usize;
    let pc_high = cpu.fetch() as usize;

    cpu.sp = cpu.sp.wrapping_sub(1);
    cpu.mem.write(cpu.sp as usize, (cpu.pc / 0x100) as u8);
    cpu.sp = cpu.sp.wrapping_sub(1);
    cpu.mem.write(cpu.sp as usize, cpu.pc as u8);

    cpu.pc = pc_low as u16 + pc_high as u16 * 0x100;

    cpu.cycles += 24;
}

fn rst_dir(cpu: &mut CPU, dir: u8) {
    cpu.sp = cpu.sp.wrapping_sub(1);
    cpu.mem.write(cpu.sp as usize, cpu.pc as u8);
    cpu.sp = cpu.sp.wrapping_sub(1);
    cpu.mem.write(cpu.sp as usize, (cpu.pc / 0x100) as u8);

    cpu.pc = dir as u16;
    cpu.cycles += 16;
}

pub fn rst_0x00(cpu: &mut CPU) {
    rst_dir(cpu, 0x00);
}

pub fn rst_0x10(cpu: &mut CPU) {
    rst_dir(cpu, 0x10);
}

pub fn rst_0x20(cpu: &mut CPU) {
    rst_dir(cpu, 0x20);
}

pub fn rst_0x30(cpu: &mut CPU) {
    rst_dir(cpu, 0x30);
}

pub fn rst_0x08(cpu: &mut CPU) {
    rst_dir(cpu, 0x08);
}

pub fn rst_0x18(cpu: &mut CPU) {
    rst_dir(cpu, 0x18);
}

pub fn rst_0x28(cpu: &mut CPU) {
    rst_dir(cpu, 0x28);
}

pub fn rst_0x38(cpu: &mut CPU) {
    rst_dir(cpu, 0x38);
}