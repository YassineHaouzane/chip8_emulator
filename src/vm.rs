use std::fmt::Display;

use sdl2::{render::Canvas, video::Window};

use crate::renderer;

pub struct VM {
    memory: [u8; 0x1000], // 4096 memory
    h: u8,
    w: u8,
    pc: u16,
    i: u16,
    stack: Vec<u16>,
    delay_timer: u8,
    sound_timer: u8,
    registers: [u8; 16],
}

impl VM {
    pub fn new() -> Self {
        VM {
            memory: [0; 0x1000],
            h: 32,
            w: 64,
            pc: 0x200,
            i: 0,
            stack: vec![],
            delay_timer: 60,
            sound_timer: 0,
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
        self.stack.push(value);
    }

    fn pop_stack(&mut self) {
        // Might want to check if the pop fails for debugging purposes
        self.stack.pop();
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

    pub fn decode_instruction(&mut self, canvas: &mut Canvas<Window>) {
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
                renderer::clear_screen(canvas)
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
                        let x = x;
                        let y = y;
                        let nibble = n;
                        println!("Draw {} {} {}", x, y, nibble)
                    }
                    _ => println!("Uninplemented instruction {:#06X?}", instruction),
                }
            }
        }
    }
}

impl Display for VM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x?}", self.memory[0x200])
    }
}
