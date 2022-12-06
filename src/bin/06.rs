use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let chars: Vec<_> = input.chars().collect();

    for (i, window) in chars.windows(4).enumerate() {
        if !has_duplicates(window) {
            return Some((i + window.len()) as u32);
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let chars: Vec<_> = input.chars().collect();

    for (i, window) in chars.windows(14).enumerate() {
        if !has_duplicates(window) {
            return Some((i + window.len()) as u32);
        }
    }

    None
}

fn has_duplicates(s: &[char]) -> bool {
    let mut seen = HashSet::with_capacity(s.len());

    for c in s {
        if seen.contains(&c) {
            return true;
        } else {
            seen.insert(c);
        }
    }

    false
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
