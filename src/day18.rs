use day18_parser::{parse, Instruction, Value};

use std::collections::HashMap;

struct Duet {
    registers: HashMap<char, i64>,
    sound_frequency: Option<i64>,
    instructions: Vec<Instruction>,
    program_counter: usize,
}

impl Duet {
    fn new(instructions: Vec<Instruction>) -> Duet {
        Duet {
            registers: HashMap::new(),
            sound_frequency: None,
            instructions,
            program_counter: 0,
        }
    }

    fn run(&mut self, instruction: Instruction) {
        let mut advance_pc = true;
        match instruction {
            Instruction::Snd(val) => self.sound_frequency = Some(self.get_val(val)),
            Instruction::Set(reg, val) => {
                let val = self.get_val(val);
                self.set(reg, val);
            }
            Instruction::Add(reg, val) => {
                let val = self.get(reg) + self.get_val(val);
                self.set(reg, val);
            }
            Instruction::Mul(reg, val) => {
                let val = self.get(reg) * self.get_val(val);
                self.set(reg, val);
            }
            Instruction::Mod(reg, val) => {
                let val = self.get(reg) % self.get_val(val);
                self.set(reg, val);
            }
            Instruction::Rcv(reg) => {
                if let Some(val) = self.sound_frequency {
                    self.set(reg, val);
                }
            }
            Instruction::Jgz(val, offset) => {
                if self.get_val(val) > 0 {
                    self.program_counter = (self.program_counter as i64 + self.get_val(offset)) as
                        usize;
                    advance_pc = false;
                }
            }
        }

        if advance_pc {
            self.program_counter += 1;
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

struct SyncDuet {
    duet: Duet,
    channel_counter: usize,
}

impl SyncDuet {
    fn new(instructions: Vec<Instruction>, p: i64) -> SyncDuet {
        let mut duet = Duet::new(instructions);

        duet.set('p', p);

        SyncDuet {
            duet,
            channel_counter: 0,
        }
    }

    fn run_until_block(&mut self, other_channel: &[i64]) -> Vec<i64> {
        let mut channel = Vec::new();

        while let Some(&instruction) = self.duet.instructions.get(self.duet.program_counter) {
            match instruction {
                Instruction::Snd(val) => {
                    channel.push(self.duet.get_val(val));
                    self.duet.program_counter += 1;
                }
                Instruction::Rcv(reg) => {
                    if let Some(&val) = other_channel.get(self.channel_counter) {
                        self.duet.set(reg, val);
                        self.channel_counter += 1;
                        self.duet.program_counter += 1;
                    } else {
                        break;
                    }
                }
                i => self.duet.run(i),
            }
        }

        channel
    }
}

pub fn part2(input: &str) -> usize {
    let instruction = parse(input);
    let mut duet0 = SyncDuet::new(instruction.clone(), 0);
    let mut duet1 = SyncDuet::new(instruction, 1);

    let mut duet0_channel = Vec::new();
    let mut duet1_channel = Vec::new();

    loop {
        let mut duet0_generated = duet0.run_until_block(&duet1_channel);
        let duet0_nb_generated = duet0_generated.len();

        duet0_channel.append(&mut duet0_generated);

        let mut duet1_generated = duet1.run_until_block(&duet0_channel);
        let duet1_nb_generated = duet1_generated.len();

        duet1_channel.append(&mut duet1_generated);

        if duet0_nb_generated == 0 && duet1_nb_generated == 0 {
            return duet1_channel.len();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        assert_eq!(
            part1(
                "set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2",
            ),
            4
        )
    }

    #[test]
    fn part2_sample() {
        assert_eq!(
            part2(
                "snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d",
            ),
            3
        )
    }
}
