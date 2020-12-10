use std::collections::BTreeSet;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Instruction {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

fn parse_instruction(line: &str) -> Instruction {
    match &line[0..3] {
        "nop" => Instruction::Nop(line[4..].parse().unwrap()),
        "jmp" => Instruction::Jmp(line[4..].parse().unwrap()),
        "acc" => Instruction::Acc(line[4..].parse().unwrap()),
        _ => unreachable!(),
    }
}

#[derive(Debug, Default)]
struct Program<'a> {
    instructions: &'a [Instruction],
    state: isize,
    pointer: usize,
    visited: BTreeSet<usize>,
}

impl<'a> Program<'a> {
    fn compile(source: &str) -> Vec<Instruction> {
        source.lines().map(parse_instruction).collect()
    }

    fn with_instructions(instructions: &'a [Instruction]) -> Self {
        Program {
            instructions,
            ..Default::default()
        }
    }

    fn can_terminate(&mut self) -> bool {
        let max = self.instructions.len();

        loop {
            if self.pointer >= max {
                return true;
            }

            match self.step() {
                Ok(_) => continue,
                Err(_) => break,
            }
        }

        false
    }

    fn step(&mut self) -> Result<(), String> {
        use Instruction::*;
        if self.visited.contains(&self.pointer) {
            return Err(format!("Already run {}", self.pointer));
        }
        self.visited.insert(self.pointer);
        match &self.instructions[self.pointer] {
            Nop(_) => self.pointer += 1,
            Acc(val) => {
                self.state = self.state + val;
                self.pointer += 1
            }
            Jmp(val) => self.pointer = (self.pointer as isize + val) as usize,
        }
        Ok(())
    }
}

fn main() {
    use Instruction::*;
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut instructions = Program::compile(&input);
    let mut program = Program::with_instructions(instructions.as_slice());
    if !program.can_terminate() {
        println!("Infinite loop detected. Program state: {}", program.state);
    }
    for i in 0..instructions.len() {
        let instruction = instructions[i];
        let flipped = match instruction {
            Acc(_) => continue,
            Jmp(val) => Nop(val),
            Nop(val) => Jmp(val),
        };
        instructions[i] = flipped;
        let mut program = Program::with_instructions(instructions.as_slice());
        if program.can_terminate() {
            println!("Final state after fix: {}", program.state);
            break;
        } else {
            instructions[i] = instruction;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instructions() {
        assert_eq!(parse_instruction("nop +0"), Instruction::Nop(0));
        assert_eq!(parse_instruction("acc -3"), Instruction::Acc(-3));
        assert_eq!(parse_instruction("jmp +2"), Instruction::Jmp(2));
    }
}
