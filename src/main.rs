use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::chip8::Chip8;
use crate::emulator::Emulator;
use crate::screen::Screen;

mod chip8;
mod emulator;
mod screen;
mod keypad;
mod constants;

fn main() {
    let rom_buffer = Emulator::load_rom_from("roms/test_opcode.ch8").unwrap();

    let sdl_context = sdl2::init().unwrap();
    let mut screen = Screen::new(&sdl_context, 20);
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut chip8 = Chip8::new();
    chip8.load_rom(rom_buffer);

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main;
                }
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    chip8.keypad.on_key_event(keycode, true);
                }
                Event::KeyUp { keycode: Some(keycode), .. } => {
                    chip8.keypad.on_key_event(keycode, false)
                }

                _ => {}
            }
        }

        chip8.run_cpu_cycle();
        screen.draw_canvas(chip8.get_display_buffer());
        std::thread::sleep(Duration::from_micros(16600));
    }
}
