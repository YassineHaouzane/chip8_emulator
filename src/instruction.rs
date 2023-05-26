// Instruction module for debugging purposes

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
