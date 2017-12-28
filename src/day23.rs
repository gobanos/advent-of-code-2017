use day23_parser::{parse, Instruction, Value};

use std::collections::HashMap;

struct CPU {
    registers: HashMap<char, i64>,
    instructions: Vec<Instruction>,
    program_counter: usize,
}

impl CPU {
    fn new(instructions: Vec<Instruction>) -> CPU {
        CPU {
            registers: HashMap::new(),
            instructions,
            program_counter: 0,
        }
    }

    fn run(&mut self, instruction: Instruction) {
        let mut advance_pc = true;
        match instruction {
            Instruction::Set(reg, val) => {
                let val = self.get_val(val);
                self.set(reg, val);
            }
            Instruction::Sub(reg, val) => {
                let val = self.get(reg) - self.get_val(val);
                self.set(reg, val);
            }
            Instruction::Mul(reg, val) => {
                let val = self.get(reg) * self.get_val(val);
                self.set(reg, val);
            }
            Instruction::Jnz(val, offset) => {
                if self.get_val(val) != 0 {
                    self.program_counter =
                        (self.program_counter as i64 + self.get_val(offset)) as usize;
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

    fn run_to_end(&mut self) -> u32 {
        let mut nb_mul = 0;

        while let Some(&instruction) = self.instructions.get(self.program_counter) {
            self.run(instruction);

            if let Instruction::Mul(_, _) = instruction {
                nb_mul += 1;
            }
        }

        nb_mul
    }
}

pub fn part1(input: &str) -> u32 {
    let mut cpu = CPU::new(parse(input));

    cpu.run_to_end()
}

pub fn part2(input: &str) -> usize {
    let mut cpu = CPU::new(parse(input).into_iter().take(8).collect());

    cpu.set('a', 1);

    cpu.run_to_end();

    let from = cpu.get('b');
    let to = cpu.get('c');

    (from..to + 1)
        .filter(|&n| (n - from) % 17 == 0)
        .filter(|&n| not_prime(n))
        .count()
}

fn not_prime(n: i64) -> bool {
    (2..).take_while(|&d| d * d <= n).any(|d| n % d == 0)
}
