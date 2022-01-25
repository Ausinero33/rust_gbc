mod hardware;

use hardware::cpu::CPU;

fn main() {
    let mut cpu = CPU::new();

    cpu.reset();

    //cpu.mem.load_rom("roms/individual/01-special.gb");
    cpu.mem.load_rom("roms/Dr. Mario (World).gb");

    loop {
        cpu.cycle();

        output_temp(&mut cpu);

        // if cpu.stop {
        //     // TODO casi seguro que esto no es asi
        //     break;
        // }
    }
}

fn output_temp(cpu: &mut CPU) {
    if cpu.mem.read(0xff02 as usize) == 0x81 {
        let c = cpu.mem.read(0xff01 as usize);
        print!("{}", char::from(c));
        cpu.mem.write(0xff02 as usize, 0);
    }
}
