use std::panic;
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
    stack: Vec<u16>,
    delay_timer: u8,
    sound_timer: u8,
    registers: [u8; 16],
    keys: [bool; 16],
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
            stack: vec![],
            delay_timer: 60,
            sound_timer: 60,
            registers: [0; 16],
            keys: [false; 16],
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

    fn register_checker(&mut self, predicate: bool) {
        if predicate {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    // Returns true if an overflow occured
    fn add_register(&mut self, index: usize, value: u8) {
        println!(
            "Adding value {:#06X?} to register, {:#04X?} which contains {:#06X?}",
            value, index, self.registers[index]
        );
        self.print_registers();
        // Allow overflow some chip8 programs seems to use willingly overflow on u8 registers
        self.registers[index] = u8::wrapping_add(self.registers[index], value);
    }

    fn sub_register(&mut self, index: usize, value: u8) {
        println!(
            "Subtracting value {:#06X?} to register, {:#04X?} which contains {:#06X?}",
            value, index, self.registers[index]
        );
        self.print_registers();
        // Allow overflow some chip8 programs seems to use willingly overflow on u8 registers
        self.registers[index] = u8::wrapping_sub(self.registers[index], value);
    }

    fn push_stack(&mut self, value: u16) {
        self.stack.push(value);
    }

    fn pop_stack(&mut self) -> u16 {
        // Might want to check if the pop fails for debugging purposes
        self.stack
            .pop()
            .expect("Can't return from subroutine stack is empty")
    }

    fn increment_pc(&mut self) {
        self.pc += 2;
    }

    fn skip_instruction_if(&mut self, predicate: bool) {
        if predicate {
            println!("Skipping instruction");
            self.increment_pc()
        }
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
        let hex_digits = (
            ((instruction & 0xF000) >> 12) as u8,
            ((instruction & 0x0F00) >> 8) as u8,
            ((instruction & 0x00F0) >> 4) as u8,
            (instruction & 0x000F) as u8,
        );
        // Aliases that matches document that im following
        let x = hex_digits.1;
        let y = hex_digits.2;
        let n = hex_digits.3;
        let nn = (instruction & 0x00FF) as u8;
        let nnn = instruction & 0x0FFF;
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

            0x00EE => {
                let adress = self.pop_stack();
                println!("Jumping from stack");
                self.jump_pc(adress);
            }

            _ => match hex_digits.0 {
                0x01 => {
                    let adress = nnn;
                    self.jump_pc(adress)
                }
                0x02 => {
                    let adress = nnn;
                    self.push_stack(self.pc);
                    self.jump_pc(adress)
                }
                0x03 => {
                    let vx_value = self.registers[x as usize];
                    println!("vx value : {:#06X?}", vx_value);
                    self.skip_instruction_if(vx_value == nn)
                }
                0x04 => {
                    let vx_value = self.registers[x as usize];
                    self.print_registers();
                    self.skip_instruction_if(vx_value != nn)
                }
                0x05 => {
                    let vx_value = self.registers[x as usize];
                    let vy_value = self.registers[y as usize];
                    self.skip_instruction_if(vx_value == vy_value)
                }
                0x09 => {
                    let vx_value = self.registers[x as usize];
                    let vy_value = self.registers[y as usize];
                    self.skip_instruction_if(vx_value != vy_value)
                }
                0x06 => {
                    let register = x;
                    let value = nn;
                    self.set_register(register as usize, value)
                }
                0x07 => {
                    let register = x;
                    let value = nn;
                    self.add_register(register as usize, value);
                    println!("result: {:#06X?}", self.registers[register as usize])
                }
                0x0A => {
                    let value = nnn;
                    self.set_i_register(value)
                }
                0x0D => {
                    let x_coordinate = (self.registers[x as usize] as usize) % self.w;
                    let y_coordinate = (self.registers[y as usize] as usize) % self.h;
                    self.print_registers();
                    self.set_register(0xF, 0);
                    let nibble = n as u16;
                    for i in 0..nibble {
                        let new_y_coords = y_coordinate + (i as usize);
                        if new_y_coords >= self.h {
                            break;
                        }
                        for bit in 0..8 {
                            let x = (self.registers[x as usize] + bit) as usize % self.w;
                            if x >= self.w {
                                break;
                            }
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
                _ => match hex_digits {
                    (0x0E, _, 0x09, 0x0E) => {
                        self.skip_instruction_if(self.keys[self.registers[x as usize] as usize])
                    }
                    (0x0E, _, 0x0A, 0x01) => {
                        self.skip_instruction_if(!self.keys[self.registers[x as usize] as usize])
                    }
                    (0x0F, _, 0x0, 0x07) => self.registers[x as usize] = self.delay_timer,
                    (0x08, _, _, 0x00) => self.set_register(x as usize, self.registers[y as usize]),
                    (0x08, _, _, 0x01) => self.set_register(
                        x as usize,
                        self.registers[y as usize] | self.registers[x as usize],
                    ),
                    (0x08, _, _, 0x02) => self.set_register(
                        x as usize,
                        self.registers[y as usize] & self.registers[x as usize],
                    ),
                    (0x08, _, _, 0x03) => self.set_register(
                        x as usize,
                        self.registers[y as usize] ^ self.registers[x as usize],
                    ),
                    (0x08, _, _, 0x04) => {
                        let x_value = self.registers[x as usize];
                        let y_value = self.registers[y as usize];
                        self.add_register(x as usize, self.registers[y as usize]);
                        self.register_checker(x_value.checked_add(y_value).is_none());
                    }
                    (0x08, _, _, 0x05) => {
                        let x_value = self.registers[x as usize];
                        let y_value = self.registers[y as usize];
                        self.sub_register(x as usize, self.registers[y as usize]);
                        self.register_checker(x_value > y_value);
                    }
                    (0x08, _, _, 0x06) => {
                        let shifted_bit = self.registers[x as usize] & 0x01;
                        self.set_register(x as usize, self.registers[x as usize] >> 1);
                        self.register_checker(shifted_bit == 1);
                    }
                    (0x08, _, _, 0x07) => {
                        let x_value = self.registers[x as usize];
                        let y_value = self.registers[y as usize];
                        self.set_register(x as usize, u8::wrapping_sub(y_value, x_value));
                        self.register_checker(y_value > x_value)
                    }

                    (0x08, _, _, 0x0E) => {
                        let last_bit = self.registers[x as usize] & 0b10000000;
                        self.set_register(x as usize, self.registers[x as usize] << 1);
                        self.register_checker(last_bit == 0b10000000);
                    }
                    (0x0F, _, 0x05, 0x05) => {
                        let adress = self.i;
                        for i in 0..=x {
                            let register_value = self.registers[i as usize];
                            self.memory[(adress + (i as u16)) as usize] = register_value;
                        }
                    }
                    (0x0F, _, 0x06, 0x05) => {
                        // Read memory ?
                        let adress = self.i;
                        for i in 0..=x {
                            self.set_register(
                                i as usize,
                                self.memory[(adress + (i as u16)) as usize],
                            );
                        }
                    }
                    (0x0F, _, 0x03, 0x03) => {
                        let mut register_value = self.registers[x as usize];
                        let digit = register_value % 10;
                        register_value /= 10;
                        let tenths = register_value % 10;
                        register_value /= 10;
                        let hundreds = register_value % 10;
                        let adress = self.i as usize;
                        self.memory[adress] = hundreds;
                        self.memory[adress + 1] = tenths;
                        self.memory[adress + 2] = digit;
                    }
                    (0x0F, _, 0x01, 0x0E) => {
                        self.i += (self.registers[x as usize]) as u16;
                    }
                    _ => {
                        panic!("Uninplemented instruction {:#06X?}", instruction);
                    }
                },
            },
        }
    }

    fn print_registers(&self) {
        for ele in self.registers.iter() {
            println!("{:#04X?}", ele);
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

    fn cpu_cycle(&mut self, keys: [bool; 16]) {
        self.delay_timer = if self.delay_timer == 0 {
            0
        } else {
            self.delay_timer - 1
        };
        self.sound_timer = if self.delay_timer == 0 {
            0
        } else {
            self.sound_timer - 1
        };
        self.keys = keys;
    }

    pub fn run_rom(rom_path: &String) -> Result<(), String> {
        let mut virtual_machine = Self::read_rom(rom_path);
        let mut renderer_context = SDLWrapper::initialize_sdl_renderer().unwrap();
        'running: loop {
            let event_message = renderer_context.handle_event();
            if event_message.is_err() {
                break 'running;
            }

            let event = event_message.unwrap();
            virtual_machine.cpu_cycle(event);

            virtual_machine.decode_instruction(&mut renderer_context);
            renderer_context.draw(&virtual_machine.display_bits);

            //thread::sleep(Duration::from_millis(100));
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
