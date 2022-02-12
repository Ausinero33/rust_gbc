mod hardware;

use hardware::{GameBoy, bus::Bus};
use sfml::{graphics::{RenderWindow, RenderTarget, Color}, window::{Style, Event, Key}};

fn checksum() -> u8 {
    let mut x: u8 = 0;

    let rom = [0x44, 0x52, 0x2E, 0x4D, 0x41, 0x52, 0x49, 0x4F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00];

    for i in rom {
        x = x.wrapping_sub(x);
        x = x.wrapping_sub(i);
        x = x.wrapping_sub(1);

    };

    x
}

fn main() {
    let mut window = RenderWindow::new(
        (160, 144),
        "GameBoy",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_framerate_limit(60);

    println!("{}", checksum());

    let mut gameboy = GameBoy::new(Bus::new(), false);

    let roms = [
        "roms/individual/01-special.gb",
        "roms/individual/02-interrupts.gb",
        "roms/individual/03-op sp,hl.gb",
        "roms/individual/04-op r,imm.gb",
        "roms/individual/05-op rp.gb",
        "roms/individual/06-ld r,r.gb",
        "roms/individual/07-jr,jp,call,ret,rst.gb",
        "roms/individual/08-misc instrs.gb",
        "roms/individual/09-op r,r.gb",
        "roms/individual/10-bit ops.gb",
        "roms/individual/11-op a,(hl).gb"
    ];

    for i in 0..11 {
        gameboy.reset();
        gameboy.load_rom(roms[i]);
    
        'inner: loop {
            // Controlar si se sale
            while let Some(event) = window.poll_event() {
                match event {
                    Event::Closed | Event::KeyPressed {
                        code: Key::ESCAPE, ..
                    } => break 'inner,
                    _ => {}
                }
            }
    
            gameboy.cycle();
            window.clear(Color::BLACK);
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