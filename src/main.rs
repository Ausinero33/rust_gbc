mod hardware;

use hardware::{cpu::CPU, GameBoy, bus::Bus};

fn main() {
    //let mut cpu = CPU::new();

    //cpu.reset();

    //cpu.bus.load_rom("roms/individual/01-special.gb");
    //cpu.bus.load_rom("roms/Dr. Mario (World).gb");

    let mut gameboy = GameBoy::new(Bus::new());

    gameboy.reset();

    gameboy.load_rom("roms/Dr. Mario (World).gb");

    loop {
        gameboy.cpu.cycle();

        output_temp(&mut gameboy.cpu);

        // if cpu.stop {
        //     // TODO casi seguro que esto no es asi
        //     break;
        // }
    }
}

fn output_temp(cpu: &mut CPU) {
    if cpu.bus.read(0xff02 as usize) == 0x81 {
        let c = cpu.bus.read(0xff01 as usize);
        print!("{}", char::from(c));
        cpu.bus.write(0xff02 as usize, 0);
    }
}
