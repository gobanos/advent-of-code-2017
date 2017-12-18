#[macro_use]
extern crate nom;

use nom::{anychar, digit, space};
use std::str;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Instruction {
    Snd(char),
    Set(char, Value),
    Add(char, Value),
    Mul(char, Value),
    Mod(char, Value),
    Rcv(char),
    Jgz(Value, Value),
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Value {
    Register(char),
    Literal(i32),
}

named!(register<char>, verify!(anychar, |c| c >= 'a' && c <= 'z'));
named!(literal<i32>, map_res!(
    map_res!(
        recognize!(
            pair!(
                opt!(tag!("-")),
                digit
            )
        ), str::from_utf8
    ), str::parse
));

named!(value<Value>, alt!(
    map!(register, Value::Register) |
    map!(literal, Value::Literal)
));

named!(snd<Instruction>, do_parse!(
    tag!("snd ")    >>
    reg: register   >>

    (Instruction::Snd(reg))
));

named!(rcv<Instruction>, do_parse!(
    tag!("rcv ")    >>
    reg: register   >>

    (Instruction::Rcv(reg))
));

named!(set<Instruction>, do_parse!(
    tag!("set ")    >>
    reg: register   >>
    space           >>
    val: value      >>

    (Instruction::Set(reg, val))
));

named!(add<Instruction>, do_parse!(
    tag!("add ")    >>
    reg: register   >>
    space           >>
    val: value      >>

    (Instruction::Add(reg, val))
));

named!(mul<Instruction>, do_parse!(
    tag!("mul ")    >>
    reg: register   >>
    space           >>
    val: value      >>

    (Instruction::Mul(reg, val))
));

named!(modulo<Instruction>, do_parse!(
    tag!("mod ")    >>
    reg: register   >>
    space           >>
    val: value      >>

    (Instruction::Mod(reg, val))
));

named!(jgz<Instruction>, do_parse!(
    tag!("jgz ")    >>
    val: value      >>
    space           >>
    offset: value   >>

    (Instruction::Jgz(val, offset))
));

named!(line<Instruction>, alt!(
    snd |
    set |
    add |
    mul |
    modulo |
    rcv |
    jgz
));

pub fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .filter_map(|l| line(l.as_bytes()).to_result().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use Instruction::*;
    use Value::*;

    #[test]
    fn sample() {
        let instructions = parse(
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
        );

        assert_eq!(instructions, vec![
            Set('a', Literal(1)),
            Add('a', Literal(2)),
            Mul('a', Register('a')),
            Mod('a', Literal(5)),
            Snd('a'),
            Set('a', Literal(0)),
            Rcv('a'),
            Jgz(Register('a'), Literal(-1)),
            Set('a', Literal(1)),
            Jgz(Register('a'), Literal(-2)),
        ])
    }
}
