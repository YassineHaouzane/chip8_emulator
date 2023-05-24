use std::env;
use std::fs;

fn read_file(rom_path: &String) -> Vec<u16> {
    println!("Trying to load rom: {}", rom_path);
    let bytes_rom = fs::read(rom_path).expect("Cannot get bytes");
    let little_endian_chip_rom: Vec<u16> = bytes_rom
        .chunks_exact(2)
        .into_iter()
        .map(|a| {
            let first_chunk = u8::from_ne_bytes([a[0]]);
            let second_chunk = u8::from_ne_bytes([a[1]]);
            // Big endian to little endian
            u16::from_ne_bytes([second_chunk, first_chunk])
        })
        .collect();
    little_endian_chip_rom
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let little_endian_chip_rom = read_file(&args[1]);
    println!("{}", little_endian_chip_rom.len());
    for i in little_endian_chip_rom.iter().take(10) {
        println!("{:#06X?}", i);
    }
}
