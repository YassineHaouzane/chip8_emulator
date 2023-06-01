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
    fn handle_event(&mut self) -> Result<[bool; 16], ()>;
    fn draw(&mut self, pixels: &[[u8; CHIP8_WIDTH]; CHIP8_HEIGHT]);
}

impl Renderer for SDLWrapper {
    fn clear_screen(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear()
    }

    fn handle_event(&mut self) -> Result<[bool; 16], ()> {
        for event in self.event_handler.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return Err(()),
                _ => {}
            }
        }
        let mut keys = [false; 16];

        self.event_handler
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .for_each(|key| {
                let key_index_o = match key {
                    Keycode::Num1 => Some(0x1),
                    Keycode::Num2 => Some(0x2),
                    Keycode::Num3 => Some(0x3),
                    Keycode::Num4 => Some(0xC),
                    Keycode::Q => Some(0x4),
                    Keycode::W => Some(0x5),
                    Keycode::E => Some(0x6),
                    Keycode::R => Some(0xD),
                    Keycode::A => Some(0x7),
                    Keycode::S => Some(0x8),
                    Keycode::D => Some(0x9),
                    Keycode::F => Some(0xE),
                    Keycode::Z => Some(0xA),
                    Keycode::X => Some(0x0),
                    Keycode::C => Some(0xB),
                    Keycode::V => Some(0xF),
                    _ => None,
                };
                if let Some(key_index) = key_index_o {
                    println!("key index :{}", key_index);
                    keys[key_index] = true;
                }
            });
        Ok(keys)
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
