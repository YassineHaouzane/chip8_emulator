mod renderer;
mod vm;
use renderer::handle_event;
use renderer::initialize_sdl_renderer;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
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
        virtual_machine.decode_instruction();
        let event = handle_event(&mut event_pump, &mut canvas);
        if event.is_some() {
            break 'running;
        }
        canvas.clear();
        canvas.present();
        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
