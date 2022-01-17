use crate::hardware::cpu::CPU;

use super::cpu;

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

// TODO u8 LOAD/STORE/MOVE
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
    u8_to_reg(cpu, B);
}

pub fn ld_h_u8(cpu: &mut CPU) {
    u8_to_reg(cpu, B);
}

pub fn ld_c_u8(cpu: &mut CPU) {
    u8_to_reg(cpu, B);
}

pub fn ld_e_u8(cpu: &mut CPU) {
    u8_to_reg(cpu, B);
}

pub fn ld_l_u8(cpu: &mut CPU) {
    u8_to_reg(cpu, B);
}

pub fn ld_hlind_u8(cpu: &mut CPU) {
    let val = cpu.fetch();
    u8_to_hlind(cpu, val);
}

pub fn ld_a_u8(cpu: &mut CPU) {
    u8_to_reg(cpu, B);
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
    reg_to_regxind(cpu, B, HL);
}

pub fn ld_hlind_c(cpu: &mut CPU) {
    reg_to_regxind(cpu, C, HL);
}

pub fn ld_hlind_d(cpu: &mut CPU) {
    reg_to_regxind(cpu, D, HL);
}

pub fn ld_hlind_e(cpu: &mut CPU) {
    reg_to_regxind(cpu, E, HL);
}

pub fn ld_hlind_h(cpu: &mut CPU) {
    reg_to_regxind(cpu, H, HL);
}

pub fn ld_hlind_l(cpu: &mut CPU) {
    reg_to_regxind(cpu, L, HL);
}

pub fn ld_hlind_a(cpu: &mut CPU) {
    reg_to_regxind(cpu, A, HL);
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
    cpu.mem.write(cpu.sp as usize, cpu.registers[regx]);
    cpu.sp -= 1;
    cpu.mem.write(cpu.sp as usize, cpu.registers[regx - 1]);
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
    pop_regx(cpu, AF);
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

// TODO u8 ALU

// TODO u16 ALU

// TODO CB u8

// TODO CONTROL/BR
pub fn nop(cpu: &mut CPU) {
    println!("NOP");
}

// TODO CONTROL/MISC
