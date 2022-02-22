mod hardware;

use hardware::{GameBoy, bus::Bus, Keys};
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
        //"roms/individual/02-interrupts.gb",
        // "roms/individual/03-op sp,hl.gb",
        // "roms/individual/04-op r,imm.gb",
        // "roms/individual/05-op rp.gb",
        // "roms/individual/06-ld r,r.gb",
        // "roms/individual/07-jr,jp,call,ret,rst.gb",
        // "roms/individual/08-misc instrs.gb",
        // "roms/individual/09-op r,r.gb",
        // "roms/individual/10-bit ops.gb",
        // "roms/individual/11-op a,(hl).gb",
        "roms/Dr. Mario (World).gb",
        //"roms/cpu_instrs.gb",
        //"roms/Tetris (World) (Rev A).gb",
        //"roms/Pokemon Red (UE) [S][!].gb",
        //"roms/Super Mario Land (World).gb",
        //"roms/mem_timing.gb"
        //"roms/instr_timing.gb"
    ];

    for i in roms {
        let mut gameboy = GameBoy::new(Bus::new(), true);
        gameboy.reset();
        gameboy.load_rom(i);
    
        'inner: loop {
            while let Some(event) = window.poll_event() {
                match event {
                    Event::Closed | Event::KeyPressed {
                        code: Key::ESCAPE, ..
                    } => break 'inner,
                    Event::KeyPressed {code: Key::P, ..} => gameboy.debug_background(),
                    Event::KeyPressed {code: Key::O, ..} => gameboy.debug_vram(),
                    Event::KeyPressed {code: Key::I, ..} => gameboy.debug_frame(),

                    // Inputs de la consola
                    Event::KeyPressed {code: Key::DOWN, ..} => gameboy.set_input(Keys::Down, true),
                    Event::KeyPressed {code: Key::UP, ..} => gameboy.set_input(Keys::Up, true),
                    Event::KeyPressed {code: Key::LEFT, ..} => gameboy.set_input(Keys::Left, true),
                    Event::KeyPressed {code: Key::RIGHT, ..} => gameboy.set_input(Keys::Right, true),
                    Event::KeyPressed {code: Key::Z, ..} => gameboy.set_input(Keys::Select, true),
                    Event::KeyPressed {code: Key::X, ..} => gameboy.set_input(Keys::Start, true),
                    Event::KeyPressed {code: Key::S, ..} => gameboy.set_input(Keys::B, true),
                    Event::KeyPressed {code: Key::A, ..} => gameboy.set_input(Keys::A, true),

                    Event::KeyReleased {code: Key::DOWN, ..} => gameboy.set_input(Keys::Down, false),
                    Event::KeyReleased {code: Key::UP, ..} => gameboy.set_input(Keys::Up, false),
                    Event::KeyReleased {code: Key::LEFT, ..} => gameboy.set_input(Keys::Left, false),
                    Event::KeyReleased {code: Key::RIGHT, ..} => gameboy.set_input(Keys::Right, false),
                    Event::KeyReleased {code: Key::Z, ..} => gameboy.set_input(Keys::Select, false),
                    Event::KeyReleased {code: Key::X, ..} => gameboy.set_input(Keys::Start, false),
                    Event::KeyReleased {code: Key::S, ..} => gameboy.set_input(Keys::B, false),
                    Event::KeyReleased {code: Key::A, ..} => gameboy.set_input(Keys::A, false),
                
                    _ => {}
                }
            }
    
            gameboy.cycle();

            window.clear(Color::BLACK);
            
            gameboy.draw(&mut window);
            
            //println!("{}", gameboy.cpu.bus.read(0xFF00));

            window.display()
        }
    }
}