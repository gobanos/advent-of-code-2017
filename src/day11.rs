#[derive(Debug)]
struct HexagonalGrid {
    x: i32,
    y: i32,
}

impl HexagonalGrid {
    fn new() -> HexagonalGrid {
        HexagonalGrid { x: 0, y: 0 }
    }

    fn step(&mut self, direction: &HexagonalDirection) {
        let (x, y) = direction.to_point();
        self.x += x;
        self.y += y;
    }

    fn distance(&self) -> u32 {
        let z = -self.x - self.y;

        self.x.abs().max(self.y.abs()).max(z.abs()) as u32
    }
}

#[derive(Debug, Eq, PartialEq)]
enum HexagonalDirection {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
}

impl HexagonalDirection {
    fn from_str(dir: &str) -> HexagonalDirection {
        use self::HexagonalDirection::*;
        match dir {
            "n" => North,
            "ne" => NorthEast,
            "se" => SouthEast,
            "s" => South,
            "sw" => SouthWest,
            "nw" => NorthWest,
            _ => unreachable!(),
        }
    }

    fn to_point(&self) -> (i32, i32) {
        use self::HexagonalDirection::*;
        match *self {
            North => (0, -1),
            NorthEast => (1, -1),
            SouthEast => (1, 0),
            South => (0, 1),
            SouthWest => (-1, 1),
            NorthWest => (-1, 0),
        }
    }
}

pub fn part1(input: &str) -> u32 {
    let mut grid = HexagonalGrid::new();
    for dir in input.split(',').map(HexagonalDirection::from_str) {
        grid.step(&dir);
    }

    grid.distance()
}

pub fn part2(input: &str) -> u32 {
    let mut grid = HexagonalGrid::new();

    input
        .split(',')
        .map(HexagonalDirection::from_str)
        .map(|dir| {
            grid.step(&dir);
            grid.distance()
        })
        .max()
        .expect("Failed to compute max distance")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// ne,ne,ne is 3 steps away.
    fn part1_sample1() {
        let mut grid = HexagonalGrid::new();
        for dir in &["ne", "ne", "ne"] {
            grid.step(HexagonalDirection::from_str(dir));
        }
        assert_eq!(grid.distance(), 3);
    }

    #[test]
    /// ne,ne,sw,sw is 0 steps away (back where you started).
    fn part1_sample2() {
        let mut grid = HexagonalGrid::new();
        for dir in &["ne", "ne", "sw", "sw"] {
            grid.step(HexagonalDirection::from_str(dir));
        }
        assert_eq!(grid.distance(), 0);
    }

    #[test]
    /// ne,ne,s,s is 2 steps away (se,se).
    fn part1_sample3() {
        let mut grid = HexagonalGrid::new();
        for dir in &["ne", "ne", "s", "s"] {
            grid.step(HexagonalDirection::from_str(dir));
        }
        assert_eq!(grid.distance(), 2);
    }

    #[test]
    /// se,sw,se,sw,sw is 3 steps away (s,s,sw).
    fn part1_sample4() {
        let mut grid = HexagonalGrid::new();
        for dir in &["se", "sw", "se", "sw", "sw"] {
            grid.step(HexagonalDirection::from_str(dir));
        }
        assert_eq!(grid.distance(), 3);
    }
}
