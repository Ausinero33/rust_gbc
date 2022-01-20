mod hardware;

use hardware::cpu::CPU;

fn main() {
    let mut cpu = CPU::new();

    cpu.reset();

    cpu.mem.load_rom("roms\\individual\\07-jr,jp,call,ret,rst.gb");

    loop {
        cpu.cycle();

        if cpu.mem.read(0xff02 as usize) == 0x81 {
            let c = cpu.mem.read(0xff01 as usize);
            print!("{}", char::from(c));
            cpu.mem.write(0xff02 as usize, 0);
        }

        if cpu.pc == 0xC3FC {
            let _a = 0;
        }

        if cpu.stop {
            break;
        }
        //cpu.cycles = 0;
    }
}
