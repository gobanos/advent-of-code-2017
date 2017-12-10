#[macro_use]
extern crate nom;

use std::str;

use nom::{alpha, digit, space};

named!(register<&str>, map_res!(alpha, str::from_utf8));

named!(action<Action>, map!(
    map_res!(
        alt!(tag!("inc") | tag!("dec")),
        str::from_utf8
    ), Action::from_str
));

named!(number<i32>, map_res!(
    map_res!(
        recognize!(
            pair!(
                opt!(tag!("-")),
                digit
            )
        ), str::from_utf8
    ), str::parse
));

named!(operator<Operator>, map!(map_res!(operator_tags, str::from_utf8), Operator::from_str));

named!(operator_tags, alt!(
    tag!(">=")  |
    tag!(">")   |
    tag!("<=")  |
    tag!("<")   |
    tag!("==")  |
    tag!("!=")
));

named!(line<(&str, Action, i32, &str, Operator, i32)>, do_parse!(
    reg: register       >>
    space               >>
    act: action         >>
    space               >>
    val: number         >>
    tag!(" if ")        >>
    oth_reg: register   >>
    space               >>
    cmp: operator       >>
    space               >>
    oth_val: number     >>

    (reg, act, val, oth_reg, cmp, oth_val)
));

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Action {
    Inc,
    Dec,
}

impl Action {
    fn from_str(action: &str) -> Action {
        match action {
            "inc" => Action::Inc,
            "dec" => Action::Dec,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Operator {
    Superior,
    SuperiorOrEqual,
    Inferior,
    InferiorOrEqual,
    Equal,
    NotEqual,
}

impl Operator {
    fn from_str(operator: &str) -> Operator {
        match operator {
            ">" => Operator::Superior,
            ">=" => Operator::SuperiorOrEqual,
            "<" => Operator::Inferior,
            "<=" => Operator::InferiorOrEqual,
            "==" => Operator::Equal,
            "!=" => Operator::NotEqual,
            _ => unreachable!(),
        }
    }
}

pub fn parse<'a, T>(
    input: &'a str,
    mapper: fn(&'a str, Action, i32, &'a str, Operator, i32) -> T,
) -> Vec<T> {
    input
        .lines()
        .filter_map(|l| line(l.as_bytes()).to_result().ok())
        .map(|(reg, act, val, oth_reg, cmp, oth_val)| {
            mapper(reg, act, val, oth_reg, cmp, oth_val)
        })
        .collect()
}
