mod hardware;

use hardware::cpu::CPU;

fn main() {
    let mut cpu = CPU::new();

    cpu.reset();

    cpu.mem.load_rom("roms\\individual\\03-op sp,hl.gb");

    loop {
        cpu.cycle();

        if cpu.mem.read(0xff02 as usize) == 0x81 {
            let c = cpu.mem.read(0xff01 as usize);
            println!("{}", char::from(c));
            cpu.mem.write(0xff02 as usize, 0);
        }

        if cpu.pc == 0x0206 {
            let a = 0;
        }

        //cpu.cycles = 0;
    }
}
