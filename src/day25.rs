use std::collections::HashMap;

enum Direction {
    Left,
    Right,
}

struct Cursor(i64);
struct Value(bool);

struct Tape(HashMap<Cursor, Value>);

struct State(char);

struct InstructionList {
    write: Value,
    move_to: Direction,
    next_state: State,
}

struct Instruction([InstructionList; 2]);

struct Blueprint(HashMap<State, Instruction>);

struct TuringMachine {
    tape: Tape,
    cursor: Cursor,
    state: State,
}
