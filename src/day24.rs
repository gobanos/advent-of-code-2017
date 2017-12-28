use std::fmt;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Component(u32, u32);

impl Component {
    fn new(a: u32, b: u32) -> Component {
        Component(a, b)
    }

    fn from_str(comp: &str) -> Component {
        let numbers: Vec<u32> = comp.split('/').map(|n| n.parse().unwrap()).collect();

        Component::new(numbers[0], numbers[1])
    }

    fn value(&self) -> u32 {
        self.0 + self.1
    }

    fn has_pin(&self, nb_pin: u32) -> bool {
        self.0 == nb_pin || self.1 == nb_pin
    }
}

#[derive(Clone)]
struct Bridge<'a> {
    components: &'a [Component],
    elements: Vec<(usize, bool)>,
}

impl<'a> Bridge<'a> {
    fn new(comp: &'a [Component]) -> Bridge<'a> {
        Bridge {
            components: comp,
            elements: Vec::new(),
        }
    }

    fn last_pin(&self) -> u32 {
        self.elements
            .last()
            .map(|&(i, rev)| (self.components[i], rev))
            .map(|(c, rev)| if rev { c.0 } else { c.1 })
            .unwrap_or(0)
    }

    fn add(&mut self, index: usize) {
        let last_pin = self.last_pin();
        let comp = self.components[index];

        assert!(comp.has_pin(last_pin));

        let reversed = comp.1 == last_pin;

        self.elements.push((index, reversed))
    }

    fn value(&self) -> u32 {
        self.elements
            .iter()
            .map(|&(i, _)| self.components[i])
            .map(|c| c.value())
            .sum()
    }

    fn contains(&self, index: usize) -> bool {
        self.elements.iter().any(|&(i, _)| i == index)
    }

    fn len(&self) -> usize {
        self.elements.len()
    }
}

impl<'a> fmt::Debug for Bridge<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bridge = self.elements
            .iter()
            .map(|&(i, rev)| (self.components[i], rev))
            .map(|(c, rev)| if rev { (c.1, c.0) } else { (c.0, c.1) })
            .map(|(a, b)| format!("{}/{}", a, b))
            .collect::<Vec<_>>();

        write!(f, "Bridge [ {} ]", bridge.join("--"))
    }
}

pub fn part1(input: &str) -> u32 {
    let components = input.lines().map(Component::from_str).collect::<Vec<_>>();

    let mut bridges = vec![Bridge::new(&components)];

    loop {
        let mut new_bridges = Vec::new();
        let mut has_new = false;

        for bridge in bridges {
            let last_pin = bridge.last_pin();
            let mut preserve = true;

            for (i, _) in components
                .iter()
                .enumerate()
                .filter(|&(i, c)| !bridge.contains(i) && c.has_pin(last_pin))
            {
                let mut new_bridge = bridge.clone();

                new_bridge.add(i);

                new_bridges.push(new_bridge);

                preserve = false;
                has_new = true;
            }

            if preserve {
                new_bridges.push(bridge)
            }
        }

        bridges = new_bridges;

        if !has_new {
            return bridges
                .iter()
                .map(|b| b.value())
                .max()
                .expect("Failed to compute max bridge value");
        }
    }
}

pub fn part2(input: &str) -> u32 {
    let components = input.lines().map(Component::from_str).collect::<Vec<_>>();

    let mut bridges = vec![Bridge::new(&components)];

    loop {
        let mut new_bridges = Vec::new();
        let mut has_new = false;

        for bridge in bridges {
            let last_pin = bridge.last_pin();
            let mut preserve = true;

            for (i, _) in components
                .iter()
                .enumerate()
                .filter(|&(i, c)| !bridge.contains(i) && c.has_pin(last_pin))
            {
                let mut new_bridge = bridge.clone();

                new_bridge.add(i);

                new_bridges.push(new_bridge);

                preserve = false;
                has_new = true;
            }

            if preserve {
                new_bridges.push(bridge)
            }
        }

        bridges = new_bridges;

        if !has_new {
            bridges.sort_by(|a, b| {
                b.len()
                    .cmp(&a.len())
                    .then_with(|| b.value().cmp(&a.value()))
            });

            return bridges[0].value();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "0/2\n2/2\n2/3\n3/4\n3/5\n0/1\n10/1\n9/10";

    #[test]
    fn test_parse() {
        assert_eq!(Component::from_str("0/2"), Component::new(0, 2));
    }

    #[test]
    fn test_value() {
        assert_eq!(Component::new(3, 7).value(), 10);
    }

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE_INPUT), 31);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(SAMPLE_INPUT), 19);
    }
}
