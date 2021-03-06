use crate::instruction::Opcode;

pub struct VM {
    pub registers: [i32; 32],
    pc: usize,
    pub program: Vec<u8>,
    remainder: u32,
}

impl VM {
    pub fn new() -> Self {
        VM {
            registers: [0; 32],
            pc: 0,
            program: vec![],
            remainder: 0,
        }
    }

    /// Decode the opcode from an instruction and return it
    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        return opcode;
    }

    pub fn add_byte(&mut self, byte: u8) {
        self.program.push(byte);
    }

    /// Helper function to return the next byte of the instruction
    fn next_byte(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        return result;
    }

    /// Helper function to return the next two bytes of the instruction
    fn next_short(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | 
            self.program[self.pc + 1] as u16;
        self.pc += 2;
        return result;
    }

    /// Loops as long as instructions can be executed
    pub fn run(&mut self) {
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    /// Executes one instruction. Meant to allow for more controlled execution
    /// of the VM
    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    /// Executes an instruction
    fn execute_instruction(&mut self) -> bool {
        // Ensure PC is not invalid
        if self.pc >= self.program.len() {
            return false;
        }

        // Match opcodes
        match self.decode_opcode() {
            Opcode::LOAD => {
                let register = self.next_byte() as usize;
                let number   = self.next_short() as u32;
                self.registers[register] = number as i32;
            },
            Opcode::HLT => {
                println!("HLT");
                return false;
            },
            Opcode::ADD => {
                let reg1 = self.registers[self.next_byte() as usize];
                let reg2 = self.registers[self.next_byte() as usize];
                self.registers[self.next_byte() as usize] = reg1 + reg2;
            },
            Opcode::SUB => {
                let reg1 = self.registers[self.next_byte() as usize];
                let reg2 = self.registers[self.next_byte() as usize];
                self.registers[self.next_byte() as usize] = reg1 - reg2;
            },
            Opcode::MUL => {
                let reg1 = self.registers[self.next_byte() as usize];
                let reg2 = self.registers[self.next_byte() as usize];
                self.registers[self.next_byte() as usize] = reg1 * reg2;
            },
            Opcode::DIV => {
                let reg1 = self.registers[self.next_byte() as usize];
                let reg2 = self.registers[self.next_byte() as usize];
                self.registers[self.next_byte() as usize] = reg1 / reg2;
                self.remainder = (reg1 % reg2) as u32;
            },
            Opcode::MOV => {
                let reg1 = self.next_byte() as usize;
                let reg2 = self.registers[self.next_byte() as usize];
                self.registers[reg1] = reg2 as i32;
            },
            Opcode::JMP => {
                let target = self.registers[self.next_byte() as usize];
                self.pc = target as usize;
            },
            Opcode::JMPB => {
                let value = self.registers[self.next_byte() as usize];
                self.pc -= value as usize;                
            },

            Opcode::JMPF => {
                let value = self.registers[self.next_byte() as usize];
                self.pc += value as usize;
            },

            Opcode::IGL => {
                println!("Illegal instruction");
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_vm() -> VM {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 20;
        test_vm
    }

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0);
        assert_eq!(test_vm.pc, 0);
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![0,0,0,0];
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![255,0,0,0];
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_load() {
        let mut test_vm = VM::new();
        // [1,244] is 0x1f4 in little endian, which is 500
        let test_bytes = vec![1,0,1,244]; // LOAD $0 0x1f4
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_opcode_add() {
        let mut test_vm = get_test_vm();
        let test_bytes = vec![2, 0, 1, 2]; // ADD $0 $1 $2
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.registers[2], 0x1e);
    }

    #[test]
    fn test_opcode_sub() {
        let mut test_vm = get_test_vm();
        let test_bytes = vec![3, 1, 0, 2]; // SUB $1 $0 $2
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.registers[2], 0x0a);
    }

    #[test]
    fn test_opcode_mul() {
        let mut test_vm = get_test_vm();
        let test_bytes = vec![4, 0, 1, 2]; // MUL $0 $1 $2
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.registers[2], 0xc8);
    }

    #[test]
    fn test_opcode_div_without_remainder() {
        let mut test_vm = get_test_vm();
        let test_bytes = vec![5, 1, 0, 2]; // DIV $1 $0 $2
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.registers[2], 0x2);
    }

    #[test]
    fn test_opcode_div_with_remainder() {
        let mut test_vm = get_test_vm();
        let test_bytes = vec![5, 0, 1, 2]; // DIV $0 $1 $2
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.registers[2], 0);
        assert_eq!(test_vm.remainder, 10);
    }

    #[test]
    fn test_opcode_mov() {
        let mut test_vm = get_test_vm();
        let test_bytes = vec![6, 0, 1, 0]; // MOV $0 $1
        println!("{} {}", test_vm.registers[0], test_vm.registers[1]);
        test_vm.program = test_bytes;
        test_vm.run();
        println!("{} {}", test_vm.registers[0], test_vm.registers[1]);
        assert_eq!(test_vm.registers[0], 0x14);
        assert_eq!(test_vm.registers[1], 0x14);
    }

    #[test]
    fn test_opcode_jmp() {
        let mut test_vm = get_test_vm();
        let test_bytes = vec![7,0,0,0]; // JMP $0
        test_vm.program = test_bytes;
        test_vm.pc = 0;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 10);
    }

    #[test]
    fn test_opcode_jmpb() {
        let mut test_vm = get_test_vm();
        let test_bytes = vec![0,0,0,0,0,0,0,0,0,0,0,0,8,0,0,0]; // JMPB $0
        test_vm.program = test_bytes;
        test_vm.pc = 12;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4); // pc gets incremented by 2 before the JMPB
    }

    #[test]    
    fn test_opcode_jmpf() {
        let mut test_vm = get_test_vm();
        let test_bytes = vec![9,0,0,0]; // JMPB $0
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 12); // pc gets incremented by 2 before the JMPF
    }


}
