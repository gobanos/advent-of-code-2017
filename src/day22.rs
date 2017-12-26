use std::collections::HashMap;

type Position = (i32, i32);
type Grid = HashMap<Position, bool>;
type EvolvedGrid = HashMap<Position, State>;

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_left(&mut self) {
        use self::Direction::*;

        *self = match *self {
            Up => Left,
            Right => Up,
            Down => Right,
            Left => Down,
        }
    }

    fn turn_right(&mut self) {
        use self::Direction::*;

        *self = match *self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    fn reverse(&mut self) {
        use self::Direction::*;

        *self = match *self {
            Up => Down,
            Right => Left,
            Down => Up,
            Left => Right,
        }
    }

    fn to_move(self) -> Position {
        use self::Direction::*;

        match self {
            Up => (0, -1),
            Down => (0, 1),
            Right => (1, 0),
            Left => (-1, 0),
        }
    }
}

struct Virus {
    grid: Grid,
    position: Position,
    direction: Direction,
}

impl Virus {
    fn new(grid: Grid) -> Virus {
        Virus {
            grid,
            position: (0, 0),
            direction: Direction::Up,
        }
    }

    fn step(&mut self) -> bool {
        let current_cell = self.grid.entry(self.position).or_insert(false);
        let is_infected = *current_cell;

        if is_infected {
            self.direction.turn_right();
        } else {
            self.direction.turn_left();
        }

        *current_cell = !is_infected;

        let (dx, dy) = self.direction.to_move();

        let (ref mut x, ref mut y) = self.position;

        *x += dx;
        *y += dy;

        !is_infected
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum State {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl State {
    fn next(&mut self) {
        use self::State::*;

        *self = match *self {
            Clean => Weakened,
            Weakened => Infected,
            Infected => Flagged,
            Flagged => Clean,
        };
    }
}

struct EvolvedVirus {
    grid: EvolvedGrid,
    position: Position,
    direction: Direction,
}

impl EvolvedVirus {
    fn new(grid: EvolvedGrid) -> EvolvedVirus {
        EvolvedVirus {
            grid,
            position: (0, 0),
            direction: Direction::Up,
        }
    }

    fn step(&mut self) -> bool {
        let current_cell = self.grid.entry(self.position).or_insert(State::Clean);
        let state = *current_cell;

        match state {
            State::Clean => self.direction.turn_left(),
            State::Infected => self.direction.turn_right(),
            State::Flagged => self.direction.reverse(),
            State::Weakened => (),
        }

        current_cell.next();

        let (dx, dy) = self.direction.to_move();

        let (ref mut x, ref mut y) = self.position;

        *x += dx;
        *y += dy;

        state == State::Weakened
    }
}

fn parse_input(input: &str) -> Grid {
    let height = input.lines().count() as i32;
    let width = input.lines().next().unwrap().chars().count() as i32;

    let mut grid = Grid::with_capacity((height * width) as usize);

    for (y, line) in input.lines().enumerate() {
        let y = y as i32 - height / 2;
        for (x, c) in line.chars().enumerate() {
            let x = x as i32 - width / 2;
            let value = match c {
                '.' => false,
                '#' => true,
                _ => unreachable!(),
            };

            grid.insert((x, y), value);
        }
    }

    grid
}

fn parse_evolved_input(input: &str) -> EvolvedGrid {
    let height = input.lines().count() as i32;
    let width = input.lines().next().unwrap().chars().count() as i32;

    let mut grid = EvolvedGrid::with_capacity((height * width) as usize);

    for (y, line) in input.lines().enumerate() {
        let y = y as i32 - height / 2;
        for (x, c) in line.chars().enumerate() {
            let x = x as i32 - width / 2;
            let value = match c {
                '.' => State::Clean,
                '#' => State::Infected,
                _ => unreachable!(),
            };

            grid.insert((x, y), value);
        }
    }

    grid
}

pub fn part1(input: &str) -> usize {
    let mut virus = Virus::new(parse_input(input));

    (0..10_000).filter(|_| virus.step()).count()
}

pub fn part2(input: &str) -> usize {
    let mut virus = EvolvedVirus::new(parse_evolved_input(input));

    (0..10_000_000).filter(|_| virus.step()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "..#\n#..\n...";

    #[test]
    fn sample_parser() {
        let mut sample_grid = Grid::with_capacity(9);
        sample_grid.insert((-1, -1), false);
        sample_grid.insert((0, -1), false);
        sample_grid.insert((1, -1), true);

        sample_grid.insert((-1, 0), true);
        sample_grid.insert((0, 0), false);
        sample_grid.insert((1, 0), false);

        sample_grid.insert((-1, 1), false);
        sample_grid.insert((0, 1), false);
        sample_grid.insert((1, 1), false);

        assert_eq!(parse_input(SAMPLE_INPUT), sample_grid);
    }

    #[test]
    fn sample_evolved_parser() {
        let mut sample_grid = EvolvedGrid::with_capacity(9);
        sample_grid.insert((-1, -1), State::Clean);
        sample_grid.insert((0, -1), State::Clean);
        sample_grid.insert((1, -1), State::Infected);

        sample_grid.insert((-1, 0), State::Infected);
        sample_grid.insert((0, 0), State::Clean);
        sample_grid.insert((1, 0), State::Clean);

        sample_grid.insert((-1, 1), State::Clean);
        sample_grid.insert((0, 1), State::Clean);
        sample_grid.insert((1, 1), State::Clean);

        assert_eq!(parse_evolved_input(SAMPLE_INPUT), sample_grid);
    }

    #[test]
    fn move_sample() {
        let mut virus = Virus::new(parse_input(SAMPLE_INPUT));

        let nb_infections = (0..70).filter(|_| virus.step()).count();

        assert_eq!(nb_infections, 41);
    }

    #[test]
    fn move_evolved_sample() {
        let mut virus = EvolvedVirus::new(parse_evolved_input(SAMPLE_INPUT));

        let nb_infections = (0..100).filter(|_| virus.step()).count();

        assert_eq!(nb_infections, 26);
    }

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE_INPUT), 5587);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(SAMPLE_INPUT), 2511944);
    }
}
