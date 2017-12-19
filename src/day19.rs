type Tubes = Vec<Vec<Cell>>;

#[derive(Debug)]
struct Grid {
    tubes: Tubes,
    position: (usize, usize),
    direction: Option<Direction>,
}

impl Grid {
    fn new(tubes: Tubes) -> Grid {
        let start_y = tubes[0].iter().position(|&c| c == Cell::Vertical).unwrap();

        Grid {
            tubes,
            position: (0, start_y),
            direction: Some(Direction::Down),
        }
    }

    fn from_str(input: &str) -> Grid {
        Grid::new(
            input
                .lines()
                .map(|l| l.chars().map(Cell::from_char).collect())
                .collect(),
        )
    }

    fn step(&mut self) -> Cell {
        let (x, y) = self.position;
        let direction = self.direction.unwrap();

        self.position = match direction {
            Direction::Up => (x - 1, y),
            Direction::Down => (x + 1, y),
            Direction::Left => (x, y - 1),
            Direction::Right => (x, y + 1),
        };

        let not_empty = |&c: &Cell| !(c == Cell::Empty);

        let cell = self.get_current_cell();

        if let Cell::Angle = cell {
            let direction = match (self.get_surroundings(), direction) {
                ((Some(up), Some(right), _, _), Direction::Down)
                    if not_empty(up) && not_empty(right) => Direction::Right,
                ((Some(up), Some(right), _, _), Direction::Left)
                    if not_empty(up) && not_empty(right) => Direction::Up,

                ((Some(up), _, _, Some(left)), Direction::Down)
                    if not_empty(up) && not_empty(left) => Direction::Left,
                ((Some(up), _, _, Some(left)), Direction::Right)
                    if not_empty(up) && not_empty(left) => Direction::Up,

                ((_, Some(right), Some(down), _), Direction::Left)
                    if not_empty(right) && not_empty(down) => Direction::Down,
                ((_, Some(right), Some(down), _), Direction::Up)
                    if not_empty(right) && not_empty(down) => Direction::Right,

                ((_, _, Some(down), Some(left)), Direction::Right)
                    if not_empty(down) && not_empty(left) => Direction::Down,
                ((_, _, Some(down), Some(left)), Direction::Up)
                    if not_empty(down) && not_empty(left) => Direction::Left,

                _ => unreachable!("couldn't compute new direction"),
            };

            self.direction = Some(direction);
        }

        cell
    }

    fn get_surroundings(&self) -> (Option<&Cell>, Option<&Cell>, Option<&Cell>, Option<&Cell>) {
        let (x, y) = self.position;

        (
            // UP
            x.checked_sub(1).and_then(
                |x| self.tubes.get(x).and_then(|r| r.get(y)),
            ),
            // RIGHT
            y.checked_add(1).and_then(
                |y| self.tubes.get(x).and_then(|r| r.get(y)),
            ),
            // DOWN
            x.checked_add(1).and_then(
                |x| self.tubes.get(x).and_then(|r| r.get(y)),
            ),
            // LEFT
            y.checked_sub(1).and_then(
                |y| self.tubes.get(x).and_then(|r| r.get(y)),
            ),
        )
    }

    fn get_cell(&self, (x, y): (usize, usize)) -> Cell {
        self.tubes[x][y]
    }

    fn get_current_cell(&self) -> Cell {
        self.get_cell(self.position)
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Cell {
    Empty,
    Vertical,
    Horizontal,
    Angle,
    Letter(char),
}

impl Cell {
    fn from_char(c: char) -> Cell {
        match c {
            ' ' => Cell::Empty,
            '|' => Cell::Vertical,
            '-' => Cell::Horizontal,
            '+' => Cell::Angle,
            'A'...'Z' => Cell::Letter(c),
            _ => unreachable!("wrong char"),
        }
    }
}

pub fn part1(input: &str) -> String {
    let mut grid = Grid::from_str(input);

    let mut result = String::new();

    loop {
        match grid.step() {
            Cell::Empty => break,
            Cell::Letter(c) => result.push(c),
            _ => (),
        }
    }

    result
}

pub fn part2(input: &str) -> u32 {
    let mut grid = Grid::from_str(input);

    for i in 1.. {
        if let Cell::Empty = grid.step() {
            return i;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "     |
     |  +--+
     A  |  C
 F---|----E|--+
     |  |  |  D
     +B-+  +--+
";

    #[test]
    fn start_position() {
        let grid = Grid::from_str(SAMPLE_INPUT);

        assert_eq!(grid.position, (0, 5));
    }

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE_INPUT), "ABCDEF".to_string());
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(SAMPLE_INPUT), 38);
    }
}
