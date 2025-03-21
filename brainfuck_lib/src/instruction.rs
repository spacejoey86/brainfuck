#[derive(PartialEq, Debug)]
pub enum Instruction {
    /// Move the memory pointer one to the right
    Right,
    /// Move the memory pointer one to the left
    Left,
    /// Increment the value at the memory pointer
    Increment,
    /// Decrement the value at the memory pointer
    Decrement,
    /// If the value at the memory pointer is zero, jump forwards to the matching `EndLoop`
    /// Else, continue to the next instruction
    StartLoop,
    /// If the value at the memory pointer is non-zero, jump back to the matching `StartLoop`
    /// Else, continue to the next instruction
    EndLoop,
    /// Accept one byte of ascii input and store it in the value at the memory pointer
    Input,
    /// Output the value at the memory pointer as an ascii character
    Output,
}

impl ToString for Instruction {
    fn to_string(&self) -> String {
        match self {
            Instruction::Right => ">",
            Instruction::Left => "<",
            Instruction::Increment => "+",
            Instruction::Decrement => "-",
            Instruction::StartLoop => "[",
            Instruction::EndLoop => "]",
            Instruction::Input => ",",
            Instruction::Output => ".",
        }
        .to_string()
    }
}

impl Instruction {
    pub fn from_char(character: char) -> Option<Self> {
        match character {
            '>' => Some(Instruction::Right),
            '<' => Some(Instruction::Left),
            '+' => Some(Instruction::Increment),
            '-' => Some(Instruction::Decrement),
            '[' => Some(Instruction::StartLoop),
            ']' => Some(Instruction::EndLoop),
            ',' => Some(Instruction::Input),
            '.' => Some(Instruction::Output),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_to_instruction_to_char_matches() {
        for character in "><+-[],.".chars() {
            assert_eq!(
                character,
                Instruction::from_char(character)
                    .unwrap()
                    .to_string()
                    .chars()
                    .next()
                    .unwrap()
            )
        }
    }

    #[test]
    fn instruction_to_char_to_instruction_matches() {
        for instruction in [
            Instruction::Right,
            Instruction::Left,
            Instruction::Increment,
            Instruction::Decrement,
            Instruction::StartLoop,
            Instruction::EndLoop,
            Instruction::Input,
            Instruction::Output,
        ] {
            assert_eq!(
                instruction,
                Instruction::from_char(instruction.to_string().chars().next().unwrap()).unwrap()
            )
        }
    }
}
