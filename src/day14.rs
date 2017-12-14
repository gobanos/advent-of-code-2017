use knot_hasher::KnotHasher;
use rayon::prelude::*;

#[derive(Debug, Eq, PartialEq)]
struct Disk {
    rows: Vec<Vec<bool>>,
}

impl Disk {
    fn new(key: &str) -> Disk {
        let rows = (0..128)
            .into_par_iter()
            .map(|i| format!("{}-{}", key, i))
            .map(|k| {
                KnotHasher::dense_hash_from_key(&k)
                    .iter()
                    .map(|h| {
                        [
                            h & 0b1000_0000 > 0,
                            h & 0b0100_0000 > 0,
                            h & 0b0010_0000 > 0,
                            h & 0b0001_0000 > 0,
                            h & 0b0000_1000 > 0,
                            h & 0b0000_0100 > 0,
                            h & 0b0000_0010 > 0,
                            h & 0b0000_0001 > 0,
                        ]
                    })
                    .fold(Vec::with_capacity(128), |mut v, h| {
                        v.extend_from_slice(&h);
                        v
                    })
            })
            .collect();

        Disk { rows }
    }

    fn find_group(&self) -> Option<(usize, usize)> {
        self.rows
            .iter()
            .enumerate()
            .filter_map(|(r, row)| {
                row.iter().enumerate().find(|&(_, &cell)| cell).map(
                    |(c, _)| {
                        (r, c)
                    },
                )
            })
            .next()
    }

    fn delete_group(&mut self, row: usize, column: usize) {
        self.rows[row][column] = false;

        if row > 0 && self.rows[row - 1][column] {
            self.delete_group(row - 1, column);
        }

        if column > 0 && self.rows[row][column - 1] {
            self.delete_group(row, column - 1);
        }

        if row < self.rows.len() - 1 && self.rows[row + 1][column] {
            self.delete_group(row + 1, column);
        }

        if let Some(&true) = self.rows[row].get(column + 1) {
            self.delete_group(row, column + 1);
        }
    }
}

pub fn part1(input: &str) -> u32 {
    let disk = Disk::new(input);

    disk.rows
        .iter()
        .map(|r| r.iter().map(|&b| if b { 1 } else { 0 }).sum::<u32>())
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let mut disk = Disk::new(input);
    let mut nb_group = 0;

    while let Some((r, c)) = disk.find_group() {
        nb_group += 1;

        disk.delete_group(r, c);
    }

    nb_group
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_disk() {
        let disk = Disk::new("flqrgnkx");

        assert_eq!(disk.rows[0][0..8], from_sample("##.#.#..")[..]);
        assert_eq!(disk.rows[1][0..8], from_sample(".#.#.#.#")[..]);
        assert_eq!(disk.rows[2][0..8], from_sample("....#.#.")[..]);
        assert_eq!(disk.rows[3][0..8], from_sample("#.#.##.#")[..]);
        assert_eq!(disk.rows[4][0..8], from_sample(".##.#...")[..]);
        assert_eq!(disk.rows[5][0..8], from_sample("##..#..#")[..]);
        assert_eq!(disk.rows[6][0..8], from_sample(".#...#..")[..]);
        assert_eq!(disk.rows[7][0..8], from_sample("##.#.##.")[..]);
    }

    #[test]
    fn part1_sample() {
        assert_eq!(part1("flqrgnkx"), 8108);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2("flqrgnkx"), 1242);
    }

    fn from_sample(sample: &str) -> Vec<bool> {
        sample
            .chars()
            .map(|c| match c {
                '#' => true,
                '.' => false,
                _ => unreachable!(),
            })
            .collect()
    }
}
