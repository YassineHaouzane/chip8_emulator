use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::{self, Color},
    rect::Rect,
    render::Canvas,
    video::Window,
    EventPump,
};

use crate::{CHIP8_HEIGHT, CHIP8_WIDTH};

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

pub fn initialize_sdl_renderer() -> Result<(EventPump, Canvas<Window>), String> {
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
    Ok((event_pump, canvas))
}

pub fn clear_screen(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear()
}

pub fn handle_event(event_pump: &mut EventPump) -> Option<Event> {
    for event in event_pump.poll_iter() {
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

const SCALE_FACTOR: u32 = 20;
const SCREEN_WIDTH: u32 = (CHIP8_WIDTH as u32) * SCALE_FACTOR;
const SCREEN_HEIGHT: u32 = (CHIP8_HEIGHT as u32) * SCALE_FACTOR;

fn color(value: u8) -> pixels::Color {
    if value == 0 {
        pixels::Color::RGB(0, 0, 0)
    } else {
        pixels::Color::RGB(0, 250, 0)
    }
}

pub fn draw(canvas: &mut Canvas<Window>, pixels: &[[u8; CHIP8_WIDTH]; CHIP8_HEIGHT]) {
    for (y, row) in pixels.iter().enumerate() {
        for (x, &col) in row.iter().enumerate() {
            let x = (x as u32) * SCALE_FACTOR;
            let y = (y as u32) * SCALE_FACTOR;

            canvas.set_draw_color(color(col));
            let _ = canvas.fill_rect(Rect::new(x as i32, y as i32, SCALE_FACTOR, SCALE_FACTOR));
        }
    }
    canvas.present();
}
