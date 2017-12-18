use day18_parser::{parse, Instruction, Value};

use std::collections::HashMap;

struct Duet {
    registers: HashMap<char, i64>,
    sound_frequency: Option<i64>,
    instructions: Vec<Instruction>,
    program_counter: usize,
}

impl Duet {
    fn run(&mut self, instruction: Instruction) {
        let mut advance_pc = true;
        match instruction {
            Instruction::Snd(reg) => self.sound_frequency = Some(self.get(reg)),
            Instruction::Set(reg, val) => {
                let val = self.get_val(val);
                self.set(reg, val);
            },
            Instruction::Add(reg, val) => {
                let val = self.get(reg) + self.get_val(val);
                self.set(reg, val);
            },
            Instruction::Mul(reg, val) => {
                let val = self.get(reg) * self.get_val(val);
                self.set(reg, val);
            },
            Instruction::Mod(reg, val) => {
                let val = self.get(reg) % self.get_val(val);
                self.set(reg, val);
            },
            Instruction::Rcv(reg) => if let Some(val) = self.sound_frequency {
                self.set(reg, val);
            },
            Instruction::Jgz(val, offset) => if self.get_val(val) > 0 {
                self.program_counter = (self.program_counter as i64 + self.get_val(offset)) as usize;
                advance_pc = false;
            },
        }

        if advance_pc {
            self.program_counter += 1;
        }
    }

    fn new(instructions: Vec<Instruction>) -> Duet {
        Duet {
            registers: HashMap::new(),
            sound_frequency: None,
            instructions,
            program_counter: 0,
        }
    }

    fn get(&self, reg: char) -> i64 {
        *self.registers.get(&reg).unwrap_or(&0)
    }

    fn set(&mut self, reg: char, value: i64) {
        self.registers.insert(reg, value);
    }

    fn get_val(&self, value: Value) -> i64 {
        match value {
            Value::Register(reg) => self.get(reg),
            Value::Literal(val) => val,
        }
    }

    fn run_until_recover(&mut self) -> i64 {
        loop {
            let instruction = self.instructions[self.program_counter];

            self.run(instruction);

            if let (Instruction::Rcv(_), Some(val)) = (instruction, self.sound_frequency) {
                return val;
            }
        }
    }
}

pub fn part1(input: &str) -> i64 {
    let mut duet = Duet::new(parse(input));

    duet.run_until_recover()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        assert_eq!(part1("set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2"), 4)
    }
}