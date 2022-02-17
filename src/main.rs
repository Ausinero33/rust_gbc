mod hardware;

use hardware::{GameBoy, bus::Bus};
use sfml::{graphics::{RenderWindow, RenderTarget, Color}, window::{Style, Event, Key}};

fn main() {
    let mut window = RenderWindow::new(
        (160 * 2, 144 * 2),
        "GameBoy",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_framerate_limit(60);

    let roms = [
        //"roms/individual/01-special.gb",
        // "roms/individual/02-interrupts.gb",
        // "roms/individual/03-op sp,hl.gb",
        // "roms/individual/04-op r,imm.gb",
        // "roms/individual/05-op rp.gb",
        // "roms/individual/06-ld r,r.gb",
        // "roms/individual/07-jr,jp,call,ret,rst.gb",
        // "roms/individual/08-misc instrs.gb",
        // "roms/individual/09-op r,r.gb",
        // "roms/individual/10-bit ops.gb",
        // "roms/individual/11-op a,(hl).gb",
        //"roms/Dr. Mario (World).gb",
        //"roms/cpu_instrs.gb",
        "roms/Tetris (World) (Rev A).gb",
        //"roms/Pokemon Red (UE) [S][!].gb",
        //"roms/Super Mario Land (World).gb",
        //"roms/mem_timing.gb"
        //"roms/instr_timing.gb"
    ];

    for i in roms {
        let mut gameboy = GameBoy::new(Bus::new(), false);
        gameboy.reset();
        gameboy.load_rom(i);
    
        'inner: loop {
            // Controlar si se sale
            //println!("{:04X}", gameboy.cpu.pc);
            while let Some(event) = window.poll_event() {
                match event {
                    Event::Closed | Event::KeyPressed {
                        code: Key::ESCAPE, ..
                    } => break 'inner,
                    Event::KeyPressed {code: Key::A, ..} => gameboy.debug_background(),
                    Event::KeyPressed {code: Key::S, ..} => gameboy.debug_vram(),
                    _ => {}
                }
            }
    
            gameboy.cycle();
            window.clear(Color::BLACK);
            
            gameboy.draw(&mut window);
            
            window.display()
        }
    }

    // gameboy.reset();
    // gameboy.load_rom("roms/Dr. Mario (World).gb");


    // 'inner: loop {
    //     //Controlar si se sale
    //     while let Some(event) = window.poll_event() {
    //         match event {
    //             Event::Closed | Event::KeyPressed {
    //                 code: Key::ESCAPE, ..
    //             } => break 'inner,
    //             _ => {}
    //         }
    //     }

    //     gameboy.cycle();
    //     window.clear(Color::BLACK);
    //     window.display()
    // }
}