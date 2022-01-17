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

// TODO u8 LOAD/STORE/MOVE


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
