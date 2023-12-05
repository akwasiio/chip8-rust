use std::{thread, time};
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

const CLOCK_HZ: u64 = 500;
// The clock speed in Hertz
const TIMER_HZ: u64 = 60; // The timer speed in Hertz

fn main() {
    let rom_buffer = Emulator::load_rom_from("roms/tetris.rom").unwrap();

    let sdl_context = sdl2::init().unwrap();
    let mut screen = Screen::new(&sdl_context, 20);
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut chip8 = Chip8::new();
    chip8.load_rom(rom_buffer);

    let cycle_duration = Duration::from_nanos(1_000_000_000 / CLOCK_HZ);
    let timer_duration = Duration::from_millis(1000 / TIMER_HZ);

    let mut last_timer_update = time::Instant::now();
    'main: loop {
        chip8.run_cpu_cycle();

        if last_timer_update.elapsed() >= timer_duration {
            chip8.update_timers();
            last_timer_update = time::Instant::now();
        }

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

        if chip8.update_screen {
            screen.draw_canvas(chip8.get_display_buffer());
            chip8.update_screen = false
        }

        thread::sleep(cycle_duration)
    }
}
