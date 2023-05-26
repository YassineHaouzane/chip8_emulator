mod renderer;
mod vm;
use renderer::handle_event;
use renderer::initialize_sdl_renderer;
use sdl2::rect::Point;
use std::env;
use std::fs;
use std::thread;
use std::time::Duration;
use vm::*;

fn read_rom(rom_path: &String) -> VM {
    let mut result = VM::new();
    println!("Trying to load rom: {}", rom_path);
    let bytes_rom: Vec<u8> = fs::read(rom_path).expect("Cannot get bytes");
    let start_program_adress = 0x200;
    bytes_rom
        .into_iter()
        .enumerate()
        .for_each(|(index, value)| result.set_byte(index + start_program_adress, value));
    result
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let mut virtual_machine = read_rom(&args[1]);
    let (mut event_pump, mut canvas) = initialize_sdl_renderer().unwrap();
    'running: loop {
        let event = handle_event(&mut event_pump);
        if event.is_some() {
            break 'running;
        }
        virtual_machine.decode_instruction(&mut canvas);
        let () = canvas
            .draw_line(Point::new(0, 200), Point::new(200, 200))
            .unwrap();
        canvas.present();

        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
