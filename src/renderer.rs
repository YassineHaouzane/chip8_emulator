use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::{self, Color},
    rect::Rect,
    render::Canvas,
    video::Window,
    EventPump,
};

use crate::constants::{CHIP8_HEIGHT, CHIP8_WIDTH};

const SCALE_FACTOR: u32 = 20;

pub struct SDLWrapper {
    canvas: Canvas<Window>,
    event_handler: EventPump,
}

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

impl SDLWrapper {
    pub fn initialize_sdl_renderer() -> Result<SDLWrapper, String> {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("Chip-8", 1280, 640)
            .opengl() // this line DOES NOT enable opengl, but allows you to create/get an OpenGL context from your window.
            .build()
            .unwrap();
        let mut canvas = window
            .into_canvas()
            .index(find_sdl_gl_driver().unwrap())
            .build()
            .unwrap();
        let event_pump = sdl_context.event_pump()?;

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        Ok(SDLWrapper {
            canvas,
            event_handler: event_pump,
        })
    }

    fn color(value: u8) -> pixels::Color {
        if value == 0 {
            pixels::Color::RGB(0, 0, 0)
        } else {
            pixels::Color::RGB(250, 250, 250)
        }
    }
}

pub trait Renderer {
    fn clear_screen(&mut self);
    fn handle_event(&mut self) -> Option<Event>;
    fn draw(&mut self, pixels: &[[u8; CHIP8_WIDTH]; CHIP8_HEIGHT]);
}

impl Renderer for SDLWrapper {
    fn clear_screen(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear()
    }

    fn handle_event(&mut self) -> Option<Event> {
        for event in self.event_handler.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return Some(event),
                _ => {}
            }
        }
        None
    }

    fn draw(&mut self, pixels: &[[u8; CHIP8_WIDTH]; CHIP8_HEIGHT]) {
        for (y, row) in pixels.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                let x = (x as u32) * SCALE_FACTOR;
                let y = (y as u32) * SCALE_FACTOR;

                self.canvas.set_draw_color(Self::color(col));
                let _ = self.canvas.fill_rect(Rect::new(
                    x as i32,
                    y as i32,
                    SCALE_FACTOR,
                    SCALE_FACTOR,
                ));
            }
        }
        self.canvas.present();
    }
}
