#[macro_use]
extern crate nom;

use std::str;
use std::cmp::Ordering;

use std::ops::{Add, Mul};

use nom::digit;

pub type Int = i64;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct Vec3 {
    x: Int,
    y: Int,
    z: Int,
}

impl Vec3 {
    pub fn distance(&self) -> Int {
        self.x.abs() + self.y.abs() + self.z.abs()
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

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Mul<Int> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Int) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

named!(
    number<Int>,
    map_res!(
        map_res!(recognize!(pair!(opt!(tag!("-")), digit)), str::from_utf8),
        str::parse
    )
);

named!(
    triplet<Vec3>,
    do_parse!(
        tag!("<") >> x: number >> tag!(",") >> y: number >> tag!(",") >> z: number >> tag!(">")
            >> (Vec3 { x, y, z })
    )
);

named!(
    line<(Vec3, Vec3, Vec3)>,
    do_parse!(
        tag!("p=") >> p: triplet >> tag!(", v=") >> v: triplet >> tag!(", a=") >> a: triplet
            >> (p, v, a)
    )
);

pub fn parse<T>(input: &str, mapper: fn((Vec3, Vec3, Vec3)) -> T) -> Vec<T> {
    input
        .lines()
        .map(|l| line(l.as_bytes()).to_result().unwrap())
        .map(mapper)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::IResult::Done;

    const EMPTY: &[u8] = b"";

    #[test]
    fn sample1() {
        assert_eq!(
            line(b"p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>"),
            Done(
                EMPTY,
                (
                    Vec3 { x: 3, y: 0, z: 0 },
                    Vec3 { x: 2, y: 0, z: 0 },
                    Vec3 { x: -1, y: 0, z: 0 },
                )
            )
        );
    }

    #[test]
    fn sample2() {
        assert_eq!(
            line(b"p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>"),
            Done(
                EMPTY,
                (
                    Vec3 { x: 4, y: 0, z: 0 },
                    Vec3 { x: 0, y: 0, z: 0 },
                    Vec3 { x: -2, y: 0, z: 0 },
                )
            )
        );
    }
}
