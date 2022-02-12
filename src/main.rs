mod hardware;

// ESTO ES CON SDL2

// use std::time::{Duration, Instant};

// use hardware::{cpu::CPU, GameBoy};
// use sdl2::{event::Event, keyboard::Keycode, pixels::{Color, PixelFormatEnum}, surface::Surface, render::Texture};

// extern crate sdl2;

// fn main() {
//     let mut gameboy = GameBoy::new();
//     gameboy.reset();
//     gameboy.load_rom("roms/individual/02-interrupts.gb");

//     let sdl_context = sdl2::init().unwrap();
//     let video_subsystem = sdl_context.video().unwrap();
//     let window = video_subsystem.window("GameBoy", 320, 288).position_centered().build().unwrap();

//     let mut event_pump = sdl_context.event_pump().unwrap();

//     let mut canvas = window.into_canvas().present_vsync().build().unwrap();

//     let texture_creator = canvas.texture_creator();

//     let surface = Surface::new(160, 144, PixelFormatEnum::RGB24).unwrap();
//     let texture = Texture::from_surface(&surface, &texture_creator).unwrap();

//     'running: loop {
//         // let start = Instant::now();

//         for event in event_pump.poll_iter() {
//             match event {
//                 Event::Quit {..} |
//                 Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
//                     break 'running
//                 },
//                 _ => {}
//             }
//         }

//         gameboy.cycle();

//         // if cpu.stop {
//         //     // TODO casi seguro que esto no es asi
//         //     break;
//         // }

//         canvas.present();
        
//         // let t = Instant::now();
//         // if 1_000_000_000u64 / 60 >= (t - start).as_nanos() as u64 {
//         //     std::thread::sleep(Duration::from_nanos(1_000_000_000u64 / 60 - (t - start).as_nanos() as u64));
//         // }
//     }
// }

use std::time::{Instant, Duration};

use hardware::GameBoy;

use ggez::{Context, ContextBuilder, GameResult, event};

// fn main() {
//     let mut gameboy = GameBoy::new();
//     gameboy.reset();
//     gameboy.load_rom("roms/individual/02-interrupts.gb");

//     loop {
//         let start = Instant::now();

//         gameboy.cycle();

//         std::thread::sleep(Duration::from_nanos(1_000_000_000u64 / 60 - (Instant::now() - start).as_nanos() as u64));
//     }
// }


fn main() {
    let mut gameboy = GameBoy::new();
    gameboy.reset();
    gameboy.load_rom("roms/individual/02-interrupts.gb");

    let (mut ctx, event_loop) = ContextBuilder::new("GameBoy", "Gonzalo Burgos").build().expect("Error.");

    event::run(ctx, event_loop, gameboy)
    // loop {
    //     let start = Instant::now();

    //     gameboy.cycle();

    //     std::thread::sleep(Duration::from_nanos(1_000_000_000u64 / 60 - (Instant::now() - start).as_nanos() as u64));
    // }
}