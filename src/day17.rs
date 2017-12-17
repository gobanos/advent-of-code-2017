pub fn part1(input: usize) -> usize {
    let mut ring_buffer = vec![0];
    let mut position = 0;

    for i in 1..2018 {
        position = (position + input) % ring_buffer.len() + 1;

        ring_buffer.insert(position, i);
    }

    ring_buffer[(position + 1) % ring_buffer.len()]
}

pub fn part2(input: i32) -> i32 {
    let mut position = 0;
    let mut result = 0;

    for buffer_length in 1..50_000_001 {
        position = (position + input) % buffer_length + 1;

        if position == 1 {
            result = buffer_length;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        assert_eq!(part1(3), 638);
    }
}
