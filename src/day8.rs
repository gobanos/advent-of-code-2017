use day8_parser::{self, Action, Operator};

use std::collections::HashMap;

struct CPU<'a> {
    registers: HashMap<&'a str, i32>,
}

impl<'a> CPU<'a> {
    fn new() -> CPU<'a> {
        CPU {
            registers: HashMap::new(),
        }
    }

    fn run(&mut self, instruction: &Instruction<'a>) -> Option<i32> {
        let (other_register, operator, value) = instruction.condition;

        let &other_value = self.registers.get(other_register).unwrap_or(&0);

        if CPU::is_valid(other_value, operator, value) {
            let register = self.registers.entry(instruction.register);
            let (action, value) = instruction.action;

            let register_value = register.or_insert(0);
            match action {
                Action::Inc => *register_value += value,
                Action::Dec => *register_value -= value,
            }

            Some(*register_value)
        } else {
            None
        }
    }

    fn is_valid(a: i32, op: Operator, b: i32) -> bool {
        use day8_parser::Operator::*;

        match op {
            Superior => a > b,
            SuperiorOrEqual => a >= b,
            Inferior => a < b,
            InferiorOrEqual => a <= b,
            Equal => a == b,
            NotEqual => a != b,
        }
    }
}

#[derive(Debug)]
struct Instruction<'a> {
    register: &'a str,
    action: (Action, i32),
    condition: (&'a str, Operator, i32),
}

impl<'a> Instruction<'a> {
    fn new(
        reg: &'a str,
        act: Action,
        val: i32,
        oth_reg: &'a str,
        op: Operator,
        oth_val: i32,
    ) -> Instruction<'a> {
        Instruction {
            register: reg,
            action: (act, val),
            condition: (oth_reg, op, oth_val),
        }
    }
}

pub fn part1(input: &str) -> i32 {
    let input = day8_parser::parse(input, Instruction::new);

    let mut cpu = CPU::new();

    for instruction in &input {
        cpu.run(instruction);
    }

    cpu.registers
        .iter()
        .map(|(_, &v)| v)
        .max()
        .expect("no maximum found")
}

pub fn part2(input: &str) -> i32 {
    let input = day8_parser::parse(input, Instruction::new);

    let mut cpu = CPU::new();

    input
        .iter()
        .filter_map(|i| cpu.run(i))
        .max()
        .expect("unable to find global maximum")
}
