use crate::instruction::Instruction;

#[derive(PartialEq, Debug)]
pub struct Program {
    pub inner: Vec<Instruction>,
}

impl Program {
    /// Parse a program
    /// Doesn't do any validation (eg ensuring matching brackets)
    pub fn from_string(input: String) -> Self {
        let mut program = Self { inner: vec![] };

        for character in input.chars() {
            if let Some(instruction) = Instruction::from_char(character) {
                program.inner.push(instruction);
            }
        }

        return program;
    }
}

impl ToString for Program {
    fn to_string(&self) -> String {
        self.inner
            .iter()
            .map(|instruction| instruction.to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROGRAM_STRING: &str = "[->+<],."; // meaningless but valid program with one of each character
    fn get_program() -> Program {
        return Program {
            inner: vec![
                Instruction::StartLoop,
                Instruction::Decrement,
                Instruction::Right,
                Instruction::Increment,
                Instruction::Left,
                Instruction::EndLoop,
                Instruction::Input,
                Instruction::Output,
            ],
        };
    }

    #[test]
    fn simple_program_from_string() {
        assert_eq!(
            get_program(),
            Program::from_string(PROGRAM_STRING.to_string())
        );
    }

    #[test]
    fn simple_program_to_string() {
        assert_eq!(get_program().to_string(), PROGRAM_STRING.to_string())
    }

    #[test]
    fn simple_program_to_string_to_program() {
        assert_eq!(
            get_program(),
            Program::from_string(get_program().to_string())
        )
    }

    #[test]
    fn simple_program_from_string_to_string() {
        assert_eq!(
            PROGRAM_STRING.to_string(),
            Program::from_string(PROGRAM_STRING.to_string()).to_string()
        )
    }
}
