use std::collections::HashMap;

use day25_parser::parse;

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from(d: i32) -> Direction {
        match d {
            1 => Direction::Right,
            -1 => Direction::Left,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq, Hash, Copy, Clone)]
struct Cursor(i64);

impl Cursor {
    fn move_to(&mut self, dir: Direction) {
        match dir {
            Direction::Left => self.0 += -1,
            Direction::Right => self.0 += 1,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Value(bool);

impl Value {
    fn to_index(self) -> usize {
        if self.0 {
            1
        } else {
            0
        }
    }

    fn is_on(&self) -> bool {
        self.0
    }
}

#[derive(Debug, Default)]
struct Tape(HashMap<Cursor, Value>);

impl Tape {
    fn get(&mut self, cursor: Cursor) -> Value {
        *self.0.entry(cursor).or_insert(Value(false))
    }

    fn set(&mut self, cursor: Cursor, value: Value) {
        self.0.insert(cursor, value);
    }

    fn checksum(&self) -> usize {
        self.0.iter().filter(|&(_, &v)| v.is_on()).count()
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct State(char);

#[derive(Debug, Copy, Clone)]
struct InstructionList {
    write: Value,
    move_to: Direction,
    next_state: State,
}

impl InstructionList {
    fn new(write: Value, move_to: Direction, next_state: State) -> InstructionList {
        InstructionList {
            write,
            move_to,
            next_state,
        }
    }
}

#[derive(Debug)]
struct Instruction([InstructionList; 2]);

impl Instruction {
    fn get(&self, value: Value) -> InstructionList {
        self.0[value.to_index()]
    }
}

#[derive(Debug)]
struct Blueprint(HashMap<State, Instruction>);

impl Blueprint {
    fn get(&self, state: State, value: Value) -> InstructionList {
        self.0.get(&state).unwrap().get(value)
    }
}

#[derive(Debug)]
struct TuringMachine {
    tape: Tape,
    cursor: Cursor,
    state: State,
    blueprint: Blueprint,
    nb_step: u64,
}

impl TuringMachine {
    fn run(&mut self) {
        let current_value = self.tape.get(self.cursor);

        let instructions = self.blueprint.get(self.state, current_value);

        self.tape.set(self.cursor, instructions.write);
        self.cursor.move_to(instructions.move_to);
        self.state = instructions.next_state;
    }

    fn run_to_checksum(&mut self) -> usize {
        for _ in 0..self.nb_step {
            self.run();
        }

        self.checksum()
    }

    fn checksum(&self) -> usize {
        self.tape.checksum()
    }
}

fn parse_input(input: &str) -> TuringMachine {
    parse(input,
          |(state, (cond_a, (w_a, d_a, s_a)), (cond_b, (w_b, d_b, s_b)))| {
              let a = InstructionList::new(Value(w_a), Direction::from(d_a), State(s_a));
              let b = InstructionList::new(Value(w_b), Direction::from(d_b), State(s_b));

              assert_ne!(cond_a, cond_b);

              let list = if cond_a {
                  [b, a]
              } else {
                  [a, b]
              };

              (
                  state,
                  Instruction(list)
              )
          },
        |(begin_state, nb_step), instructions| {
            let begin_state = State(begin_state);
            let blueprint = Blueprint(instructions.into_iter().map(|(c, i)| (State(c), i)).collect());

            TuringMachine {
                tape: Tape::default(),
                cursor: Cursor::default(),
                state: begin_state,
                blueprint,
                nb_step,
            }
        }
    )
}

pub fn part1(input: &str) -> usize {
    let mut turing_machine = parse_input(input);

    turing_machine.run_to_checksum()
}

pub fn part2(_input: &str) -> ! {
    unimplemented!()
}