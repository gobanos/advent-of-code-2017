#[derive(Debug, Eq, PartialEq)]
struct Layer {
    depth: u32,
    range: u32,
}

impl Layer {
    fn from_str(line: &str) -> Layer {
        let line = line.split(": ").collect::<Vec<_>>();

        Layer {
            depth: line[0].parse().unwrap(),
            range: line[1].parse().unwrap(),
        }
    }

    fn period(&self) -> u32 {
        (self.range - 1) * 2
    }

    fn collide(&self, start_time: u32) -> bool {
        (start_time + self.depth) % self.period() == 0
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Firewall {
    layers: Vec<Layer>,
}

impl Firewall {
    fn new(layers: Vec<Layer>) -> Firewall {
        Firewall { layers }
    }

    fn cross(&self, delay: u32) -> Option<u32> {
        self.layers
            .iter()
            .filter(|l| l.collide(delay))
            .map(|l| l.depth * l.range)
            .fold(None, |prev, val| Some(prev.unwrap_or(0) + val))
    }
}

pub fn part1(input: &str) -> u32 {
    let firewall = Firewall::new(input.lines().map(Layer::from_str).collect());

    firewall.cross(0).unwrap()
}

pub fn part2(input: &str) -> u32 {
    let firewall = Firewall::new(input.lines().map(Layer::from_str).collect());

    (0..)
        .filter_map(|i| if firewall.cross(i).is_none() {
            Some(i)
        } else {
            None
        })
        .next()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> Firewall {
        Firewall::new(
            "0: 3\n1: 2\n4: 4\n6: 4"
                .lines()
                .map(Layer::from_str)
                .collect(),
        )
    }

    #[test]
    fn parse_input() {
        assert_eq!(
            sample_input(),
            Firewall {
                layers: vec![
                    Layer { depth: 0, range: 3 },
                    Layer { depth: 1, range: 2 },
                    Layer { depth: 4, range: 4 },
                    Layer { depth: 6, range: 4 },
                ],
            }
        )
    }

    #[test]
    fn part1_sample() {
        assert_eq!(sample_input().cross(0), Some(24))
    }
}
