use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;
use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct Screen {
    scale: usize,
    canvas: WindowCanvas,
}

impl Screen {
    pub fn new(context: &Sdl, scale: usize) -> Self {
        let video_subsystem = context.video().unwrap();

        let window = video_subsystem
            .window("chip8", (SCREEN_WIDTH * scale) as u32, (SCREEN_HEIGHT * scale) as u32)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        Self {
            scale,
            canvas,
        }
    }


    pub fn draw_canvas(&mut self, buffer: [[u16; SCREEN_WIDTH]; SCREEN_HEIGHT]) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0)); // set the background color to black
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGB(255, 255, 255));


        for row in 0 .. SCREEN_HEIGHT {
            for column in 0 .. SCREEN_WIDTH {
                let color = if buffer[row][column] == 1 {
                    Color::RGB(255, 255, 255)
                } else {
                    Color::RGB(0, 0, 0)
                };

                self.canvas.set_draw_color(color);
                let rect = Rect::new((column * self.scale) as i32, (row * self.scale) as i32, self.scale as u32, self.scale as u32);
                self.canvas.fill_rect(rect).unwrap();
            }
        }

        self.canvas.present()
    }

}