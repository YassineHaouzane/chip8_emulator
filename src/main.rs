mod renderer;
mod vm;
use renderer::Renderer;
use renderer::SDLWrapper;
use std::env;
use std::fs;
use std::thread;
use std::time::Duration;
use vm::*;

const CHIP8_HEIGHT: usize = 32;
const CHIP8_WIDTH: usize = 64;

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
    let mut renderer_context = SDLWrapper::initialize_sdl_renderer().unwrap();
    'running: loop {
        let event = renderer_context.handle_event();
        if event.is_some() {
            break 'running;
        }
        virtual_machine.decode_instruction(&mut renderer_context);
        renderer_context.draw(&virtual_machine.display_bits);
        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
