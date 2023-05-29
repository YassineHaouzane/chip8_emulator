use std::{fmt::Display, fs, thread, time::Duration};

use crate::constants::{CHIP8_HEIGHT, CHIP8_WIDTH, FONTS};
use crate::renderer::{Renderer, SDLWrapper};

pub struct VM {
    memory: [u8; 0x1000], // 4096 memory
    display_bits: [[u8; CHIP8_WIDTH]; CHIP8_HEIGHT],
    h: usize,
    w: usize,
    pc: u16,
    i: u16,
    _stack: Vec<u16>,
    _delay_timer: u8,
    _sound_timer: u8,
    registers: [u8; 16],
}

impl VM {
    pub fn new() -> Self {
        let mut memory = [0; 0x1000];
        FONTS
            .into_iter()
            .enumerate()
            .for_each(|(index, value)| memory[index] = value);
        VM {
            memory,
            display_bits: [[0; CHIP8_WIDTH]; CHIP8_HEIGHT],
            h: 32,
            w: 64,
            pc: 0x200,
            i: 0,
            _stack: vec![],
            _delay_timer: 60,
            _sound_timer: 0,
            registers: [0; 16],
        }
    }

    pub fn set_byte(&mut self, index: usize, value: u8) {
        println!(
            "Setting byte at index {:#06X?}, with value {:#04X?}",
            index, value
        );
        self.memory[index] = value;
    }

    fn set_i_register(&mut self, value: u16) {
        println!("Setting byte in i register, with value {:#06X?}", value);
        self.i = value;
    }

    fn set_register(&mut self, index: usize, value: u8) {
        println!(
            "Setting byte in register {:#06X?}, with value {:#04X?}",
            index, value
        );
        self.registers[index] = value;
    }

    fn add_register(&mut self, index: usize, value: u8) {
        println!("Adding value {:#06X?} to register, {:#04X?}", value, index);
        self.registers[index] += value;
    }

    fn push_stack(&mut self, value: u16) {
        self._stack.push(value);
    }

    fn pop_stack(&mut self) {
        // Might want to check if the pop fails for debugging purposes
        self._stack.pop();
    }

    fn increment_pc(&mut self) {
        self.pc += 2;
    }

    fn jump_pc(&mut self, adress: u16) {
        println!("Jumping to {:#06X?}", adress);
        self.pc = adress
    }

    fn get_current_instruction(&self) -> u16 {
        ((self.memory[(self.pc) as usize] as u16) << 8) | self.memory[(self.pc + 1) as usize] as u16
    }

    fn decode_instruction(&mut self, context: &mut SDLWrapper) {
        let instruction = self.get_current_instruction();
        let [first_chunk, second_chunk] = instruction.to_be_bytes();
        // Bug with jump_pc shouldn't increment
        self.increment_pc();
        println!(
            "Decoding... adress: {:#06X?} byte: {:#06X?}",
            self.pc, instruction
        );
        match instruction {
            0x00E0 => {
                println!("Clear screen");
                // Probably not the best performance wise
                self.display_bits = [[0; CHIP8_WIDTH]; CHIP8_HEIGHT];
                context.clear_screen()
            }
            _ => {
                let first_digit = first_chunk >> 4;
                let second_digit = first_chunk & 0x0F;
                let third_digit = second_chunk >> 4;
                let fourth_digit = second_chunk & 0x0F;
                let second_byte = second_chunk;
                let all_bytes_except_first =
                    u16::from_ne_bytes([second_chunk, (first_chunk & 0x0F)]);
                // Aliases that matches document that im following
                let x = second_digit;
                let y = third_digit;
                let n = fourth_digit;
                let nn = second_byte;
                let nnn = all_bytes_except_first;

                match first_digit {
                    0x01 => {
                        let adress = nnn;
                        self.jump_pc(adress)
                    }
                    0x06 => {
                        let register = x;
                        let value = nn;
                        self.set_register(register as usize, value)
                    }
                    0x07 => {
                        let register = x;
                        let value = nn;
                        self.add_register(register as usize, value)
                    }
                    0x0A => {
                        let value = nnn;
                        self.set_i_register(value)
                    }
                    0x0D => {
                        let x_coordinate = (self.registers[x as usize] as usize) % self.w;
                        let y_coordinate = (self.registers[y as usize] as usize) % self.h;
                        self.set_register(0xF, 0);
                        let nibble = n as u16;
                        for i in 0..nibble {
                            let new_y_coords = y_coordinate + (i as usize);
                            if new_y_coords >= self.w || (x as usize) >= self.h {
                                break;
                            }
                            for bit in 0..8 {
                                let x = (self.registers[x as usize] + bit) as usize % self.w;
                                let color = (self.memory[(self.i + i) as usize] >> (7 - bit)) & 1;
                                self.registers[0x0f] |= color & self.display_bits[new_y_coords][x];
                                self.display_bits[new_y_coords][x] ^= color;
                            }
                        }
                        println!("Draw {} {} {}", x_coordinate, y_coordinate, nibble)
                    }
                    0x0C => {
                        let random_number = rand::random::<u8>() & nn;
                        self.set_register(x as usize, random_number);
                    }
                    _ => println!("Uninplemented instruction {:#06X?}", instruction),
                }
            }
        }
    }

    fn read_rom(rom_path: &String) -> Self {
        let mut result = Self::new();
        println!("Trying to load rom: {}", rom_path);
        let bytes_rom: Vec<u8> = fs::read(rom_path).expect("Cannot get bytes");
        let start_program_adress = 0x200;
        bytes_rom
            .into_iter()
            .enumerate()
            .for_each(|(index, value)| result.set_byte(index + start_program_adress, value));
        result
    }

    pub fn run_rom(rom_path: &String) -> Result<(), String> {
        let mut virtual_machine = Self::read_rom(rom_path);
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
}

impl Display for VM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x?}", self.memory[0x200])
    }
}
