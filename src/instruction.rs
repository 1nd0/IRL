#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Opcode{
    HLT,
    LOAD,
    ADD,
    SUB,
    MPL,
    DIV,
    JMP,
    JPF,
    JPB,
    EQ,
    IGL

}


pub struct  Instruction {
    opcode: Opcode
}
impl Instruction{
    pub fn new(opcode: Opcode) -> Instruction{
        Instruction {
            opcode
        }
    }
}

impl From<Opcode> for u8{
    fn from(op:Opcode) -> Self{
        match op {
            Opcode::HLT => 0,
            Opcode::LOAD => 1,
            Opcode::ADD => 2,
            Opcode::SUB => 3,
            Opcode::MPL => 4,
            Opcode::DIV => 5,
            Opcode::JMP => 6,
            Opcode::JPF => 7,
            Opcode::JPB => 8,
            Opcode::EQ => 9,
            Opcode::IGL =>100

        }
    }

}

impl From<u8> for Opcode{
    fn from(v: u8)-> Self{
        match v {
            0 => return Opcode::HLT,
            1 => return Opcode::LOAD,
            2 => return Opcode::ADD,
            3 => return Opcode::SUB,
            4 => return Opcode::MPL,
            5 => return Opcode::DIV,
            6 => return Opcode::JMP,
            7 => return Opcode::JPF,
            8 => return Opcode::JPB,
            9 => return Opcode::EQ,
            _ => return Opcode::IGL
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::HLT);
        assert_eq!(instruction.opcode, Opcode::HLT);
    }
}