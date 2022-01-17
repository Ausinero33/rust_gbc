mod hardware;

use hardware::cpu::CPU;

fn main() {
    let mut cpu = CPU::new();

    loop {
        cpu.cycle();
    }
}
