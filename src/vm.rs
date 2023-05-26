use std::fmt::Display;

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
            h: 64,
            w: 32,
            pc: 0x200,
            i: 0,
            stack: vec![],
            delay_timer: 60,
            sound_timer: 0,
            registers: [0; 16],
        }
    }

    pub fn set_byte(&mut self, index: usize, value: u8) {
        self.memory[index] = value;
    }

    fn set_i_register(&mut self, value: u16) {
        self.i = value;
    }

    fn set_register(&mut self, index: usize, value: u8) {
        self.registers[index] = value;
    }

    fn add_register(&mut self, index: usize, value: u8) {
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
        self.pc = adress
    }

    fn get_current_instruction(&self) -> u16 {
        ((self.memory[(self.pc) as usize] as u16) << 8) | self.memory[(self.pc + 1) as usize] as u16
    }

    pub fn decode_instruction(&mut self) {
        let instruction = self.get_current_instruction();
        let [first_chunk, second_chunk] = instruction.to_be_bytes();
        self.increment_pc();
        match instruction {
            0x00E0 => println!("Clear screen"),
            _ => match first_chunk >> 4 {
                0x01 => {
                    let adress = u16::from_ne_bytes([second_chunk, (first_chunk & 0x0F)]);
                    println!("jump to pc: {:#06X?} current_pc: {:#06X?}", adress, self.pc);
                    self.jump_pc(adress)
                }
                0x06 => {
                    let register = first_chunk & 0x0F;
                    let value = second_chunk;
                    self.set_register(register as usize, value)
                }
                0x07 => {
                    let register = first_chunk & 0x0F;
                    let value = second_chunk;
                    self.add_register(register as usize, value)
                }
                0x0A => {
                    let value = u16::from_ne_bytes([second_chunk, (first_chunk & 0x0F)]);

                    self.set_i_register(value)
                }
                0x0D => {
                    let x = first_chunk & 0x0F;
                    let y = second_chunk >> 4;
                    let nibble = second_chunk & 0x0F;
                    println!("Draw {} {} {}", x, y, nibble)
                }
                _ => println!("Uninplemented instruction {:#06X?}", instruction),
            },
        }
    }
}

impl Display for VM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x?}", self.memory[0x200])
    }
}
