use day21_parser::parse;
use std::collections::HashMap;

type PixelGrid = Vec<Vec<bool>>;

fn mirror_vertical<T: Copy>(grid: &[Vec<T>]) -> Vec<Vec<T>> {
    grid.iter()
        .cloned()
        .map(|row| row.into_iter().rev().collect())
        .collect()
}

fn mirror_horizontal<T: Copy>(grid: &[Vec<T>]) -> Vec<Vec<T>> {
    grid.iter().cloned().rev().collect()
}

fn mirror_diagonal<T: Copy>(grid: &[Vec<T>]) -> Vec<Vec<T>> {
    if grid.is_empty() {
        return Vec::new();
    }

    let row_len = grid[0].len();

    let mut result = Vec::with_capacity(row_len);

    for y in 0..row_len {
        result.push(Vec::with_capacity(grid.len()));

        for row in grid {
            result[y].push(row[y]);
        }
    }

    result
}

fn parse_input(input: &str) -> HashMap<PixelGrid, PixelGrid> {
    let mut map = HashMap::new();

    let input = parse(input, |line| line);

    for (pattern, replace) in input {
        let base = pattern;

        let flip_vert = mirror_vertical(&base);
        let flip_diag = mirror_diagonal(&base);
        let flip_hori = mirror_horizontal(&base);

        let flip_hori_diag = mirror_diagonal(&flip_hori);
        let flip_vert_diag = mirror_diagonal(&flip_vert);
        let flip_hori_vert = mirror_vertical(&flip_hori);

        let flip_hori_vert_diag = mirror_diagonal(&flip_hori_vert);

        map.entry(base).or_insert_with(|| replace.clone());
        map.entry(flip_vert).or_insert_with(|| replace.clone());
        map.entry(flip_diag).or_insert_with(|| replace.clone());
        map.entry(flip_hori).or_insert_with(|| replace.clone());

        map.entry(flip_hori_diag).or_insert_with(|| replace.clone());
        map.entry(flip_hori_vert).or_insert_with(|| replace.clone());
        map.entry(flip_vert_diag).or_insert_with(|| replace.clone());

        map.entry(flip_hori_vert_diag)
            .or_insert_with(|| replace.clone());
    }

    map
}

fn start_pattern() -> PixelGrid {
    vec![
        vec![false, true, false],
        vec![false, false, true],
        vec![true, true, true],
    ]
}

#[derive(Debug, Eq, PartialEq)]
struct Grid {
    pixels: PixelGrid,
}

impl Grid {
    fn new(pixels: PixelGrid) -> Grid {
        Grid { pixels }
    }

    fn chunks(&self) -> Vec<Vec<PixelGrid>> {
        let chunk_size = if self.pixels.len() % 2 == 0 { 2 } else { 3 };
        let nb_chunks = self.pixels.len() / chunk_size;

        let mut chunks = Vec::with_capacity(nb_chunks);

        for x in (0..nb_chunks).map(|n| n * chunk_size) {
            let mut chunks_row = Vec::with_capacity(nb_chunks);

            for y in (0..nb_chunks).map(|n| n * chunk_size) {
                let mut chucks_cell = Vec::with_capacity(chunk_size);

                for i in 0..chunk_size {
                    let mut row = Vec::with_capacity(chunk_size);

                    for j in 0..chunk_size {
                        row.push(self.pixels[x + i][y + j]);
                    }

                    chucks_cell.push(row);
                }

                chunks_row.push(chucks_cell);
            }

            chunks.push(chunks_row);
        }

        chunks
    }

    fn step(&mut self, map: &HashMap<PixelGrid, PixelGrid>) {
        let chunks = self.chunks()
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|pattern| map[&pattern].clone())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        self.pixels = Vec::new();

        for chunk_row in chunks {
            for r in 0..chunk_row[0].len() {
                let mut row = Vec::new();

                for chuck in &chunk_row {
                    let mut pixels = chuck[r].clone();
                    row.append(&mut pixels);
                }

                self.pixels.push(row);
            }
        }
    }

    fn nb_pixels_on(&self) -> usize {
        self.pixels
            .iter()
            .map(|row| row.iter().filter(|&&b| b).count())
            .sum()
    }
}

pub fn part1(input: &str) -> usize {
    let input = parse_input(input);

    let mut grid = Grid::new(start_pattern());

    for _ in 0..5 {
        grid.step(&input);
    }

    grid.nb_pixels_on()
}

pub fn part2(input: &str) -> usize {
    let input = parse_input(input);

    let mut grid = Grid::new(start_pattern());

    for _ in 0..18 {
        grid.step(&input);
    }

    grid.nb_pixels_on()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mirror_vertical_2() {
        let a = vec![vec![1, 2], vec![3, 4]];

        assert_eq!(mirror_vertical(&a), vec![vec![2, 1], vec![4, 3]]);
    }

    #[test]
    fn test_mirror_vertical_3() {
        let a = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        assert_eq!(
            mirror_vertical(&a),
            vec![vec![3, 2, 1], vec![6, 5, 4], vec![9, 8, 7]]
        );
    }

    #[test]
    fn test_mirror_horizontal_2() {
        let a = vec![vec![1, 2], vec![3, 4]];

        assert_eq!(mirror_horizontal(&a), vec![vec![3, 4], vec![1, 2]]);
    }

    #[test]
    fn test_mirror_horizontal_3() {
        let a = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        assert_eq!(
            mirror_horizontal(&a),
            vec![vec![7, 8, 9], vec![4, 5, 6], vec![1, 2, 3]]
        );
    }

    #[test]
    fn test_mirror_diagonal_2() {
        let a = vec![vec![1, 2], vec![3, 4]];

        assert_eq!(mirror_diagonal(&a), vec![vec![1, 3], vec![2, 4]]);
    }

    #[test]
    fn test_mirror_diagonal_3() {
        let a = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        assert_eq!(
            mirror_diagonal(&a),
            vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]
        );
    }

    #[test]
    fn sample_part1() {
        let input = parse_input("../.# => ##./#../...\n.#./..#/### => #..#/..../..../#..#");

        let mut grid = Grid::new(start_pattern());

        assert_eq!(
            grid,
            Grid {
                pixels: vec![
                    vec![false, true, false],
                    vec![false, false, true],
                    vec![true, true, true],
                ],
            }
        );

        grid.step(&input);

        assert_eq!(
            grid,
            Grid {
                pixels: vec![
                    vec![true, false, false, true],
                    vec![false, false, false, false],
                    vec![false, false, false, false],
                    vec![true, false, false, true],
                ],
            }
        );

        grid.step(&input);

        assert_eq!(
            grid,
            Grid {
                pixels: vec![
                    vec![true, true, false, true, true, false],
                    vec![true, false, false, true, false, false],
                    vec![false, false, false, false, false, false],
                    vec![true, true, false, true, true, false],
                    vec![true, false, false, true, false, false],
                    vec![false, false, false, false, false, false],
                ],
            }
        );
    }
}
