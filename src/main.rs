use std::env;
use std::fmt::Display;
use std::fs;
#[derive(Debug)]
enum Instruction {
    ClearScreen,                       // 00E0 (clear screen)
    Jump { adress: u16 },              // 1NNN (jump)
    SetRegister { x: u8, value: u8 },  // 6XNN (set register VX)
    AddRegister { x: u8, value: u8 },  // 7XNN (add value to register VX)
    SetIRegister { adress: u16 },      // ANNN (set index register I)
    Draw { x: u8, y: u8, nibble: u8 }, // DXYN (display/draw)
}
impl Instruction {
    pub fn from_u16(value: u16) -> Option<Self> {
        let [first_chunk, second_chunk] = value.to_be_bytes();
        match value {
            0x00E0 => Some(Instruction::ClearScreen),
            _ => match first_chunk >> 4 {
                0x01 => {
                    let adress = u16::from_ne_bytes([second_chunk, (first_chunk & 0x0F)]);
                    Some(Instruction::Jump { adress })
                }
                0x06 => {
                    let register = first_chunk & 0x0F;
                    let value = second_chunk;
                    Some(Instruction::SetRegister { x: register, value })
                }
                0x07 => {
                    let register = first_chunk & 0x0F;
                    let value = second_chunk;
                    Some(Instruction::AddRegister { x: register, value })
                }
                0x0A => {
                    let adress = u16::from_ne_bytes([second_chunk, (first_chunk & 0x0F)]);
                    Some(Instruction::SetIRegister { adress })
                }
                0x0D => {
                    let x = first_chunk & 0x0F;
                    let y = second_chunk >> 4;
                    let nibble = second_chunk & 0x0F;
                    Some(Instruction::Draw { x, y, nibble })
                }
                _ => None,
            },
        }
    }
}

struct VM {
    memory: [u8; 0x1000], // 4096 memory
    h: u8,
    w: u8,
    pc: u16,
    i: u16,
    stack: Vec<u8>,
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
            delay_timer: 0,
            sound_timer: 0,
            registers: [0; 16],
        }
    }

    pub fn set_byte(&mut self, index: usize, value: u8) {
        self.memory[index] = value;
    }
}

impl Display for VM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x?}", self.memory[0x200])
    }
}

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
    let virtual_machine = read_rom(&args[1]);
    println!("{}", virtual_machine);
}
