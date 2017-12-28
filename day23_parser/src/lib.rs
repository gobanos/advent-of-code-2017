#[macro_use]
extern crate nom;

use nom::{anychar, digit, space};
use std::str;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Instruction {
    Set(char, Value),
    Sub(char, Value),
    Mul(char, Value),
    Jnz(Value, Value),
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Value {
    Register(char),
    Literal(i64),
}

named!(register<char>, verify!(anychar, |c| c >= 'a' && c <= 'z'));
named!(
    literal<i64>,
    map_res!(
        map_res!(recognize!(pair!(opt!(tag!("-")), digit)), str::from_utf8),
        str::parse
    )
);

named!(
    value<Value>,
    alt!(map!(register, Value::Register) | map!(literal, Value::Literal))
);

named!(
    set<Instruction>,
    do_parse!(tag!("set ") >> reg: register >> space >> val: value >> (Instruction::Set(reg, val)))
);

named!(
    sub<Instruction>,
    do_parse!(tag!("sub ") >> reg: register >> space >> val: value >> (Instruction::Sub(reg, val)))
);

named!(
    mul<Instruction>,
    do_parse!(tag!("mul ") >> reg: register >> space >> val: value >> (Instruction::Mul(reg, val)))
);

named!(
    jnz<Instruction>,
    do_parse!(
        tag!("jnz ") >> val: value >> space >> offset: value >> (Instruction::Jnz(val, offset))
    )
);

named!(line<Instruction>, alt!(set | sub | mul | jnz));

pub fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            line(l.as_bytes())
                .to_result()
                .expect("Failed to parse input")
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::IResult::Done;

    const EMPTY: &[u8] = b"";

    #[test]
    fn test_set_literal() {
        assert_eq!(
            line("set a 1".as_bytes()),
            Done(EMPTY, Instruction::Set('a', Value::Literal(1)))
        );
        assert_eq!(
            line("set a -1".as_bytes()),
            Done(EMPTY, Instruction::Set('a', Value::Literal(-1)))
        );
    }

    #[test]
    fn test_set_register() {
        assert_eq!(
            line("set a b".as_bytes()),
            Done(EMPTY, Instruction::Set('a', Value::Register('b')))
        );
    }

    #[test]
    fn test_sub_literal() {
        assert_eq!(
            line("sub a 1".as_bytes()),
            Done(EMPTY, Instruction::Sub('a', Value::Literal(1)))
        );
        assert_eq!(
            line("sub a -1".as_bytes()),
            Done(EMPTY, Instruction::Sub('a', Value::Literal(-1)))
        );
    }

    #[test]
    fn test_sub_register() {
        assert_eq!(
            line("sub a b".as_bytes()),
            Done(EMPTY, Instruction::Sub('a', Value::Register('b')))
        );
    }

    #[test]
    fn test_mul_literal() {
        assert_eq!(
            line("mul a 1".as_bytes()),
            Done(EMPTY, Instruction::Mul('a', Value::Literal(1)))
        );
        assert_eq!(
            line("mul a -1".as_bytes()),
            Done(EMPTY, Instruction::Mul('a', Value::Literal(-1)))
        );
    }

    #[test]
    fn test_mul_register() {
        assert_eq!(
            line("mul a b".as_bytes()),
            Done(EMPTY, Instruction::Mul('a', Value::Register('b')))
        );
    }

    #[test]
    fn test_jnz_literal() {
        assert_eq!(
            line("jnz 0 1".as_bytes()),
            Done(
                EMPTY,
                Instruction::Jnz(Value::Literal(0), Value::Literal(1))
            )
        );
        assert_eq!(
            line("jnz 0 -1".as_bytes()),
            Done(
                EMPTY,
                Instruction::Jnz(Value::Literal(0), Value::Literal(-1))
            )
        );
    }

    #[test]
    fn test_jnz_register() {
        assert_eq!(
            line("jnz a b".as_bytes()),
            Done(
                EMPTY,
                Instruction::Jnz(Value::Register('a'), Value::Register('b'))
            )
        );
    }
}
