use std::{
    collections::HashMap,
    io::{Read, Write},
};

use crate::{instruction::Instruction, program::Program};

pub struct VirtualMachine<'a> {
    program: Program,
    program_pointer: usize,
    data: HashMap<usize, i64>,
    data_pointer: usize,

    input: Box<&'a mut dyn Read>,
    output: Box<&'a mut dyn Write>,
}

impl<'a> VirtualMachine<'a> {
    pub fn new(program: Program, input: &'a mut dyn Read, output: &'a mut dyn Write) -> Self {
        Self {
            program,
            program_pointer: 0,
            data: HashMap::new(),
            data_pointer: 0,
            input: Box::new(input),
            output: Box::new(output),
        }
    }

    /// Execute the next instruction
    pub fn step(&mut self) -> Result<VirtualMachineStatus, VirtualMachineError> {
        let Some(instruction) = self.program.inner.get(self.program_pointer) else {
            return Ok(VirtualMachineStatus::Terminated);
        };

        match instruction {
            Instruction::Right => {
                self.data_pointer += 1;
                self.program_pointer += 1;
            }
            Instruction::Left => {
                self.data_pointer -= 1;
                self.program_pointer += 1;
            }
            Instruction::Increment => {
                let new_value = self.data.get(&self.data_pointer).unwrap_or(&0) + 1;
                if new_value == 0 {
                    self.data.remove(&self.data_pointer);
                } else {
                    self.data.insert(self.data_pointer, new_value);
                }
                self.program_pointer += 1;
            }
            Instruction::Decrement => {
                let new_value = self.data.get(&self.data_pointer).unwrap_or(&0) - 1;
                if new_value == 0 {
                    self.data.remove(&self.data_pointer);
                } else {
                    self.data.insert(self.data_pointer, new_value);
                }
                self.program_pointer += 1;
            }
            Instruction::StartLoop => {
                if *self.data.get(&self.data_pointer).unwrap_or(&0) == 0 {
                    let mut bracket_count = 1;
                    while bracket_count > 0 {
                        self.program_pointer += 1;
                        if let Some(next_instruction) = self.program.inner.get(self.program_pointer)
                        {
                            match next_instruction {
                                Instruction::StartLoop => bracket_count += 1,
                                Instruction::EndLoop => bracket_count -= 1,
                                _ => {}
                            }
                        } else {
                            return Err(VirtualMachineError::MissingEndLoop);
                        }
                    }
                } else {
                    self.program_pointer += 1;
                }
            }
            Instruction::EndLoop => {
                if *self.data.get(&self.data_pointer).unwrap_or(&0) != 0 {
                    let mut bracket_count = 1;
                    while bracket_count > 0 {
                        self.program_pointer -= 1;
                        if let Some(next_instruction) = self.program.inner.get(self.program_pointer)
                        {
                            match next_instruction {
                                Instruction::StartLoop => bracket_count -= 1,
                                Instruction::EndLoop => bracket_count += 1,
                                _ => {}
                            }
                        } else {
                            return Err(VirtualMachineError::MissingStartLoop);
                        }
                    }
                } else {
                    self.program_pointer += 1;
                }
            }
            Instruction::Input => {
                let mut buf: [u8; 1] = [0; 1];
                self.input
                    .read_exact(&mut buf)
                    .expect("failed to read from input");
                if buf[0] == 0 {
                    self.data.remove(&self.data_pointer);
                } else {
                    self.data.insert(self.data_pointer, buf[0] as i64);
                }
                self.program_pointer += 1;
            }
            Instruction::Output => {
                // let buf: [u8; 8] = self.data.get(&self.data_pointer).unwrap_or(&0).to_le_bytes();
                let buf: [u8; 1] = [*self.data.get(&self.data_pointer).unwrap_or(&0) as u8];
                self.output.write(&buf).expect("failed to write to output");
                self.program_pointer += 1;
            }
        }
        return Ok(VirtualMachineStatus::Running);
    }

    pub fn run(&mut self) -> Result<(), VirtualMachineError> {
        while self
            .step()
            .is_ok_and(|x| x == VirtualMachineStatus::Running)
        {}
        return Ok(());
    }
}

pub enum VirtualMachineError {
    MissingEndLoop,
    MissingStartLoop,
}

#[derive(PartialEq)]
pub enum VirtualMachineStatus {
    Running,
    Terminated,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world() {
        let program = Program::from_string("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.".to_string());
        let mut input = "".as_bytes();
        let mut output = Vec::new();
        let mut vm = VirtualMachine::new(program, &mut input, &mut output);

        vm.run();

        assert_eq!(String::from_utf8(output).unwrap(), "Hello World!\n")
    }
}
