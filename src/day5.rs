pub fn part1(input: &str) -> u32 {
    let mut jump_list = input
        .split_whitespace()
        .filter_map(|n| n.parse::<i32>().ok())
        .collect::<Vec<_>>();
    let mut current_index = 0;

    for step in 1.. {
        let offset = jump_list[current_index];

        let next_index = if offset.is_negative() {
            current_index.checked_sub(offset.abs() as usize)
        } else {
            current_index.checked_add(offset as usize)
        };

        match next_index {
            Some(next_index) if next_index < jump_list.len() => {
                jump_list[current_index] += 1;
                current_index = next_index;
            }
            _ => return step,
        }
    }

    unreachable!()
}

pub fn part2(input: &str) -> u32 {
    let mut jump_list = input
        .split_whitespace()
        .filter_map(|n| n.parse::<i32>().ok())
        .collect::<Vec<_>>();
    let mut current_index = 0;

    for step in 1.. {
        let offset = jump_list[current_index];

        let next_index = if offset.is_negative() {
            current_index.checked_sub(offset.abs() as usize)
        } else {
            current_index.checked_add(offset as usize)
        };

        match next_index {
            Some(next_index) if next_index < jump_list.len() => {
                if offset >= 3 {
                    jump_list[current_index] -= 1;
                } else {
                    jump_list[current_index] += 1;
                }

                current_index = next_index;
            }
            _ => return step,
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        assert_eq!(part1("0\n3\n0\n1\n-3"), 5);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2("0\n3\n0\n1\n-3"), 10);
    }
}
