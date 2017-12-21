use std::str::FromStr;
use std::num::ParseIntError;
use std::collections::HashMap;

struct Dance {
    dancers: Vec<char>,
    offset: usize,
}

impl Dance {
    fn new(nb_dancers: usize) -> Dance {
        assert!(nb_dancers <= 26);

        Dance {
            dancers: (b'a'..).map(|c| c as char).take(nb_dancers).collect(),
            offset: 0,
        }
    }

    fn make_move(&mut self, m: Move) {
        match m {
            Move::Spin(offset) => self.spin(offset),
            Move::Exchange(a, b) => self.exchange(a, b),
            Move::Partner(a, b) => self.partner(a, b),
        }
    }

    fn spin(&mut self, offset: usize) {
        let offset = self.dancers.len() - offset;
        self.offset = (self.offset + offset) % self.dancers.len();
    }

    fn exchange(&mut self, a: usize, b: usize) {
        let a = (a + self.offset) % self.dancers.len();
        let b = (b + self.offset) % self.dancers.len();

        self.dancers.swap(a, b);
    }

    fn partner(&mut self, a: char, b: char) {
        let a = self.dancers.iter().position(|&d| d == a).unwrap();
        let b = self.dancers.iter().position(|&d| d == b).unwrap();

        self.dancers.swap(a, b);
    }

    fn to_string(&self) -> String {
        self.dancers[self.offset..]
            .iter()
            .chain(self.dancers[..self.offset].iter())
            .collect()
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl FromStr for Move {
    type Err = ParseMoveError;

    fn from_str(m: &str) -> Result<Self, ParseMoveError> {
        let (action, value) = m.split_at(1);

        match action {
            "s" => Ok(Move::Spin(value.parse()?)),
            "x" => {
                let value = value.split('/').collect::<Vec<_>>();

                let a = value.get(0).ok_or(ParseMoveError::ActionError)?;
                let b = value.get(1).ok_or(ParseMoveError::ActionError)?;

                Ok(Move::Exchange(a.parse()?, b.parse()?))
            }
            "p" => {
                let value = value
                    .split('/')
                    .filter_map(|c| c.chars().next())
                    .collect::<Vec<_>>();

                Ok(Move::Partner(
                    *value.get(0).ok_or(ParseMoveError::ActionError)?,
                    *value.get(1).ok_or(ParseMoveError::ActionError)?,
                ))
            }
            _ => Err(ParseMoveError::ActionError),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum ParseMoveError {
    ActionError,
    ParseError(ParseIntError),
}

impl From<ParseIntError> for ParseMoveError {
    fn from(err: ParseIntError) -> Self {
        ParseMoveError::ParseError(err)
    }
}

#[derive(Debug)]
struct FastDance {
    dancers: Vec<char>,
    dance: Vec<usize>,
    permutations: HashMap<char, char>,
}

impl FastDance {
    fn new(nb_dancers: usize, moves: &[Move]) -> FastDance {
        let mut dance = Dance::new(nb_dancers);

        for m in moves.iter().filter_map(|&m| {
            if let Move::Partner(_, _) = m {
                None
            } else {
                Some(m)
            }
        }) {
            dance.make_move(m);
        }

        let mut permutations = (b'a'..)
            .map(|c| c as char)
            .map(|c| (c, c))
            .take(nb_dancers)
            .collect::<HashMap<_, _>>();

        for (a, b) in moves.iter().filter_map(|&m| {
            if let Move::Partner(a, b) = m {
                Some((a, b))
            } else {
                None
            }
        }) {
            let b_from = permutations[&b];

            let a_from = permutations.insert(a, b_from).unwrap();
            permutations.insert(b, a_from);
        }

        permutations = permutations.into_iter().map(|(k, v)| (v, k)).collect();

        FastDance {
            dancers: (b'a'..).map(|c| c as char).take(nb_dancers).collect(),
            dance: dance
                .to_string()
                .chars()
                .map(|c| (c as u8 - b'a') as usize)
                .collect(),
            permutations,
        }
    }

    fn dance(&mut self) {
        self.dancers = self.dance
            .iter()
            .map(|&i| self.dancers[i])
            .map(|c| self.permutations[&c])
            .collect();
    }

    fn to_string(&self) -> String {
        self.dancers.iter().collect()
    }

    fn get_cycle(&mut self) -> Vec<String> {
        let mut cycle = vec![self.to_string()];

        for _ in 0.. {
            self.dance();
            let positions = self.to_string();

            if cycle.contains(&positions) {
                break;
            }

            cycle.push(positions);
        }

        cycle
    }
}

pub fn part1(input: &str) -> String {
    let mut dance = Dance::new(16);

    for m in input.split(',').filter_map(|m| m.parse::<Move>().ok()) {
        dance.make_move(m);
    }

    dance.to_string()
}

pub fn part2(input: &str) -> String {
    let moves = input
        .split(',')
        .filter_map(|m| m.parse::<Move>().ok())
        .collect::<Vec<_>>();

    let mut fast_dance = FastDance::new(16, &moves);

    let mut cycle = fast_dance.get_cycle();

    let index = 1_000_000_000 % cycle.len();

    cycle.remove(index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_spin() {
        assert_eq!("s1".parse(), Ok(Move::Spin(1)));
    }

    #[test]
    fn parse_exchange() {
        assert_eq!("x3/4".parse(), Ok(Move::Exchange(3, 4)));
    }

    #[test]
    fn parse_partner() {
        assert_eq!("pe/b".parse(), Ok(Move::Partner('e', 'b')));
    }

    #[test]
    fn part1_sample() {
        let mut dance = Dance::new(5);

        assert_eq!(dance.to_string(), "abcde");

        dance.make_move("s1".parse().unwrap());
        assert_eq!(dance.to_string(), "eabcd");

        dance.make_move("x3/4".parse().unwrap());
        assert_eq!(dance.to_string(), "eabdc");

        dance.make_move("pe/b".parse().unwrap());
        assert_eq!(dance.to_string(), "baedc");
    }

    #[test]
    fn part2_sample() {
        let dance: Vec<_> = ["s1", "x3/4", "pe/b"]
            .iter()
            .filter_map(|m| m.parse::<Move>().ok())
            .collect();

        let mut fast_dance = FastDance::new(5, &dance);

        for _ in 0..2 {
            fast_dance.dance();
        }

        assert_eq!(fast_dance.to_string(), "ceadb");
    }
}
