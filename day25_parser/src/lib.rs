#[macro_use]
extern crate nom;

use nom::{anychar, digit, line_ending};
use std::str;

type Header = (char, u64);
type InstructionList = (bool, i32, char);
type Instruction = (char, (bool, InstructionList), (bool, InstructionList));

named!(state<char>, call!(anychar));

named!(
    number<u64>,
    map_res!(map_res!(digit, str::from_utf8), str::parse)
);

named!(
    value<bool>,
    alt!(value!(false, tag!("0")) | value!(true, tag!("1")))
);

named!(
    direction<i32>,
    alt!(value!(-1, tag!("left")) | value!(1, tag!("right")))
);

named!(
    begin_state<char>,
    do_parse!(tag!("Begin in state ") >> s: state >> tag!(".") >> (s))
);

named!(
    nb_step<u64>,
    do_parse!(
        tag!("Perform a diagnostic checksum after ") >> nb: number >> tag!(" steps.") >> (nb)
    )
);

named!(
    header<Header>,
    do_parse!(bs: begin_state >> line_ending >> ns: nb_step >> (bs, ns))
);

named!(
    in_state<char>,
    do_parse!(tag!("In state ") >> s: state >> tag!(":") >> (s))
);

named!(
    if_value<bool>,
    do_parse!(tag!("If the current value is ") >> v: value >> tag!(":") >> (v))
);

named!(
    set_value<bool>,
    do_parse!(tag!("- Write the value ") >> v: value >> tag!(".") >> (v))
);

named!(
    move_to<i32>,
    do_parse!(tag!("- Move one slot to the ") >> d: direction >> tag!(".") >> (d))
);

named!(
    next_state<char>,
    do_parse!(tag!("- Continue with state ") >> s: state >> tag!(".") >> (s))
);

named!(
    instruction_list<InstructionList>,
    ws!(do_parse!(
        v: set_value >> d: move_to >> s: next_state >> (v, d, s)
    ))
);

named!(
    instruction<Instruction>,
    ws!(do_parse!(
        s: in_state >> cond_a: if_value >> inst_a: instruction_list >> cond_b: if_value
            >> inst_b: instruction_list >> (s, (cond_a, inst_a), (cond_b, inst_b))
    ))
);

named!(
    blueprint<(Header, Vec<Instruction>)>,
    ws!(do_parse!(
        h: header >> instructions: many1!(instruction) >> (h, instructions)
    ))
);

pub fn parse<I, T>(
    input: &str,
    inst_mapper: fn(Instruction) -> I,
    bp_mapper: fn(Header, Vec<I>) -> T,
) -> T {
    blueprint(input.as_bytes())
        .to_result()
        .map(|(h, i)| bp_mapper(h, i.into_iter().map(inst_mapper).collect()))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::IResult::Done;

    const EMPTY: &[u8] = b"";

    #[test]
    fn state_test() {
        assert_eq!(state(b"A"), Done(EMPTY, 'A'));
    }

    #[test]
    fn begin_state_test() {
        assert_eq!(begin_state(b"Begin in state A."), Done(EMPTY, 'A'));
    }

    #[test]
    fn number_test() {
        assert_eq!(number(b"6"), Done(EMPTY, 6));
    }

    #[test]
    fn nb_step_test() {
        assert_eq!(
            nb_step(b"Perform a diagnostic checksum after 6 steps."),
            Done(EMPTY, 6)
        );
    }

    #[test]
    fn header_test() {
        assert_eq!(
            header(b"Begin in state A.\nPerform a diagnostic checksum after 6 steps."),
            Done(EMPTY, ('A', 6))
        );
    }

    #[test]
    fn value_test() {
        assert_eq!(value(b"0"), Done(EMPTY, false));
        assert_eq!(value(b"1"), Done(EMPTY, true));
    }

    #[test]
    fn direction_test() {
        assert_eq!(direction(b"left"), Done(EMPTY, -1));
        assert_eq!(direction(b"right"), Done(EMPTY, 1));
    }

    #[test]
    fn in_state_test() {
        assert_eq!(in_state(b"In state A:"), Done(EMPTY, 'A'));
    }

    #[test]
    fn if_value_test() {
        assert_eq!(if_value(b"If the current value is 0:"), Done(EMPTY, false));
    }

    #[test]
    fn set_value_test() {
        assert_eq!(set_value(b"- Write the value 1."), Done(EMPTY, true));
    }

    #[test]
    fn move_to_test() {
        assert_eq!(move_to(b"- Move one slot to the right."), Done(EMPTY, 1));
    }

    #[test]
    fn next_state_test() {
        assert_eq!(next_state(b"- Continue with state B."), Done(EMPTY, 'B'));
    }

    #[test]
    fn instruction_list_test() {
        let list = b"    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.";

        assert_eq!(instruction_list(list), Done(EMPTY, (true, 1, 'B')));
    }

    #[test]
    fn instruction_test() {
        let inst = b"In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.";

        assert_eq!(
            instruction(inst),
            Done(
                EMPTY,
                ('A', (false, (true, 1, 'B')), (true, (false, -1, 'B')))
            )
        );
    }

    #[test]
    fn sample_input() {
        assert_eq!(
            blueprint(include_bytes!("../resources/sample.txt")),
            Done(
                EMPTY,
                (
                    ('A', 6),
                    vec![
                        ('A', (false, (true, 1, 'B')), (true, (false, -1, 'B'))),
                        ('B', (false, (true, -1, 'A')), (true, (true, 1, 'A'))),
                    ]
                )
            )
        )
    }
}
