use std::collections::HashSet;
use std::fmt;

const LOWERCASE_OFFSET: u32 = b'a' as u32 - 1;
const UPPERCASE_OFFSET: u32 = b'A' as u32 - 1;
const ALPHABET_SIZE: u32 = 26;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                Rucksack::try_from(line)
                    .unwrap()
                    .find_overlapping()
                    .unwrap_or_else(|| panic!("line: {}", line))
                    .0
            })
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| Rucksack::try_from(line).unwrap())
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|group| {
                group
                    .iter()
                    .map(|sack| sack.left.union(&sack.right).collect::<HashSet<_>>())
                    .reduce(|a, b| a.intersection(&b).cloned().collect())
                    .unwrap()
                    .iter()
                    .cloned()
                    .map(|p| p.0)
                    .next()
                    .unwrap()
            })
            .sum::<u32>()
    )
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Priority(u32);

impl TryFrom<char> for Priority {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        if !c.is_ascii() || !c.is_alphabetic() {
            Err("Invalid character")
        } else if c.is_lowercase() {
            Ok(Priority((c as u32) - LOWERCASE_OFFSET))
        } else {
            Ok(Priority((c as u32) - UPPERCASE_OFFSET + ALPHABET_SIZE))
        }
    }
}

impl From<Priority> for char {
    fn from(p: Priority) -> Self {
        if (1..=ALPHABET_SIZE).contains(&p.0) {
            ((p.0 + LOWERCASE_OFFSET) as u8) as char
        } else {
            ((p.0 + UPPERCASE_OFFSET - ALPHABET_SIZE) as u8) as char
        }
    }
}

struct Rucksack {
    left: HashSet<Priority>,
    right: HashSet<Priority>,
}

impl fmt::Debug for Rucksack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut l = "".to_string();
        let mut r = "".to_string();

        for p in self.left.iter().cloned() {
            l.push(p.into());
        }

        for p in self.right.iter().cloned() {
            r.push(p.into());
        }

        write!(f, "{} {}", l, r)
    }
}

impl Rucksack {
    fn new(left: HashSet<Priority>, right: HashSet<Priority>) -> Self {
        Rucksack { left, right }
    }

    fn find_overlapping(&self) -> Option<Priority> {
        self.left.intersection(&self.right).next().cloned()
    }
}

impl TryFrom<&str> for Rucksack {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut left = HashSet::with_capacity(s.len() / 2);
        let mut right = HashSet::with_capacity(s.len() / 2);

        for (i, c) in s.char_indices() {
            if i < s.len() / 2 {
                left.insert(Priority::try_from(c)?);
            } else {
                right.insert(Priority::try_from(c)?);
            }
        }

        Ok(Rucksack::new(left, right))
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));

        let line = "wgqJtbJMqZVTwWPZZT";
        assert_eq!(
            Rucksack::try_from(line).unwrap().find_overlapping(),
            Some(Priority(23))
        );
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
