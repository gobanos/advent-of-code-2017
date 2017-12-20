#[macro_use]
extern crate nom;

use std::str;
use std::cmp::Ordering;

use nom::digit;

pub type Int = i64;

#[derive(Debug, Eq, PartialEq)]
pub struct Vec3 {
    x: Int,
    y: Int,
    z: Int,
}

impl Vec3 {
    pub fn distance(&self) -> Int {
        self.x + self.y + self.z
    }
}

impl Ord for Vec3 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance().cmp(&other.distance())
    }
}

impl PartialOrd for Vec3 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

named!(number<Int>, map_res!(
    map_res!(
        recognize!(
            pair!(
                opt!(tag!("-")),
                digit
            )
        ), str::from_utf8
    ), str::parse
));

named!(triplet<Vec3>, do_parse!(
    tag!("<")   >>
    x: number   >>
    tag!(",")   >>
    y: number   >>
    tag!(",")   >>
    z: number   >>
    tag!(">")   >>

    (Vec3 { x, y, z })
));

named!(line<(Vec3, Vec3, Vec3)>, do_parse!(
    tag!("p=")      >>
    p: triplet      >>
    tag!(", v=")    >>
    v: triplet      >>
    tag!(", a=")    >>
    a: triplet      >>

    (p, v, a)
));

pub fn parse<T>(input: &str, mapper: fn((Vec3, Vec3, Vec3)) -> T) -> Vec<T> {
    input
        .lines()
        .map(|l| line(l.as_bytes()).to_result().unwrap())
        .map(mapper)
        .collect()
}
