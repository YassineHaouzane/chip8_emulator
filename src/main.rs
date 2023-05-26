mod vm;
use std::env;
use std::fs;
use std::{thread, time};
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut virtual_machine = read_rom(&args[1]);
    loop {
        virtual_machine.decode_instruction();
        let ten_millis = time::Duration::from_secs(1);

        thread::sleep(ten_millis);
    }
}
