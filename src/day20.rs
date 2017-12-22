use day20_parser::{parse, Int, Vec3};

use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
struct Particle {
    position: Vec3,
    velocity: Vec3,
    acceleration: Vec3,
}

impl Particle {
    fn new((p, v, a): (Vec3, Vec3, Vec3)) -> Particle {
        Particle {
            position: p,
            velocity: v,
            acceleration: a,
        }
    }

    fn pos_at(&self, time: Int) -> Vec3 {
        let a = self.acceleration;
        let v = self.velocity;
        let p = self.position;

        a * (time * time) + (a + v * 2) * time + p * 2
    }
}

pub fn part1(input: &str) -> usize {
    let mut particles = parse(input, Particle::new)
        .into_iter()
        .enumerate()
        .collect::<Vec<_>>();

    particles.sort_by(|&(_, ref a), &(_, ref b)| {
        a.acceleration
            .cmp(&b.acceleration)
            .then(a.velocity.cmp(&b.velocity))
    });

    particles[0].0
}

pub fn part2(input: &str) -> usize {
    let mut particles = parse(input, Particle::new);

    for t in 0..1000 {
        let mut collisions: HashMap<Vec3, Vec<usize>> = HashMap::new();

        for (i, pos) in particles
            .iter()
            .enumerate()
            .rev()
            .map(|(i, p)| (i, p.pos_at(t)))
        {
            let par_at_pos = collisions.entry(pos).or_insert_with(Vec::new);
            par_at_pos.push(i);
        }

        let to_ignore = collisions.values_mut().filter(|v| v.len() > 1).fold(
            Vec::new(),
            |mut total, to_ignore| {
                total.append(to_ignore);
                total
            },
        );

        particles = particles
            .into_iter()
            .enumerate()
            .filter_map(|(i, p)| {
                if to_ignore.contains(&i) {
                    None
                } else {
                    Some(p)
                }
            })
            .collect();
    }

    particles.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_PART1: &str = "p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>\np=<4,0,0>, v=<0,0,0>, a=<-2,0,0>";

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE_PART1), 0);
    }
}
