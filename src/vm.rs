

use crate::instruction::Opcode;
use Opcode::IGL;

pub struct VM {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>,
    remainder: u32,
    equal_flag: bool,
}
impl VM{
        pub fn new() -> VM{
            VM {
            registers:[0; 32],
            program: vec![],
            pc: 0,
            remainder:0,
            equal_flag: false,
        }
    }
    pub fn run(&mut self) {
        let mut is_done = false;
        while !is_done{
            is_done= self.execute_instruction();
        }
    }

    pub fn run_once(&mut self){
        self.execute_instruction();
    }

        
    fn execute_instruction(&mut self) ->bool{
        if self.pc >= self.program.len(){
            return false;
        }
        match self.decode_opcode(){
            Opcode::HLT => {
                println!("HTL encountered");
                return false;
            },
            Opcode::LOAD => {
                //Was to usize so we can use it as an index to array
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u16;
                self.registers[register]= number as i32;
            },
            IGL => {
                println!("Unrecognized opcode found! Terminating!");
                return false;
            },
            Opcode::ADD =>{
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits()as usize] = register1 + register2;
            },
            Opcode::SUB =>{
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits()as usize] = register1 - register2;
            },
            Opcode::MPL =>{
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits()as usize] = register1 * register2;
            },
            Opcode::DIV =>{
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits()as usize] = register1 / register2;
                self.remainder = (register1 % register2) as u32;
            },
            Opcode::JMP =>{
                let target = self.registers[self.next_8_bits()as usize];
                self.pc = target as usize;
            },
            Opcode::JPF =>{
                let value = self.registers[self.next_8_bits()as usize];
                self.pc += value as usize;
            },
            Opcode::JPB =>{
                let value = self.registers[self.next_8_bits()as usize];
                self.pc -= value as usize;
            },
            Opcode::EQ =>{
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                if register1  == register2 {
                    self.equal_flag = true;
                }else{
                    self.equal_flag = false;
                }
                self.next_8_bits();

            }
        }
        true
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        return  opcode;
    }
    fn next_8_bits(&mut self) -> u8{
        let result = self.program[self.pc];
        self.pc +=1;
        return result;
    }
    fn next_16_bits(&mut self) -> u16{
        let result = (self.program[self.pc]as u16) << 8 | self.program[self.pc + 1] as u16;
        self.pc += 2;
        return result;
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn name() {
        unimplemented!();
    }
    #[test]
    fn test_create_vm(){
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0)
    }
    #[test]
    fn test_opcode_htl(){
        let mut test_vm= VM::new();
        let test_bytes = vec![0,0,0,0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }
    #[test]
    fn test_opcode_igl(){
        let mut test_vm = VM::new();
        let test_bytes = vec!{200,0,0,0};
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }
    #[test]
    fn test_opcode_load(){
        let mut test_vm = VM::new();
        test_vm.program = vec!{0,0,1,244};
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500)    
    }
    #[test]
    fn test_opcode_jmp(){
        let mut test_vm = VM::new();
        test_vm.registers[0] = 1;
        test_vm.program = vec![6,0,0,0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }
    #[test]
    fn test_opcode_jpf(){
        let mut test_vm=VM::new();
        test_vm.registers[0] = 2;
        test_vm.program = vec![7,0,0,0,5,0,0,0];
        test_vm.run_once();
        assert_eq!(test_vm.pc,4)
    }
    #[test]
    fn test_opcode_jpb(){
        let mut test_vm = VM::new();
        test_vm.registers [1] = 6; 
        test_vm.program = vec![0,0,0,10,8,1,0,0];
        test_vm.run_once();
        test_vm.run_once();
        assert_eq!(test_vm.pc,0);
    }
    #[test]
    fn test_opcode_eq(){
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![10, 0, 1, 0, 10 ,0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }
}