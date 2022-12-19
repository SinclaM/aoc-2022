use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

pub fn part_one(input: &str) -> Option<u32> {
    let pairs = input
        .split("\n\n")
        .map(|pair| {
            pair.lines()
                .map(|line| serde_json::from_str::<'_, Packet>(line).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    for (i, pair) in pairs.iter().enumerate() {
        let l = &pair[0];
        let r = &pair[1];
        if l <= r {
            sum += i + 1;
        }
    }

    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut packets = input
        .split("\n\n")
        .map(|pair| {
            pair.lines()
                .map(|line| serde_json::from_str::<'_, Packet>(line).unwrap())
        })
        .flatten()
        .collect::<Vec<_>>();

    let dividers = (
        Packet::List(vec![Packet::List(vec![Packet::Number(2)])]),
        Packet::List(vec![Packet::List(vec![Packet::Number(6)])]),
    );

    packets.push(dividers.0.clone());
    packets.push(dividers.1.clone());
    packets.sort();

    let i = packets
        .iter()
        .position(|packet| packet == &dividers.0)
        .unwrap();

    let j = packets
        .iter()
        .position(|packet| packet == &dividers.1)
        .unwrap();

    Some((i + 1) as u32 * (j + 1) as u32)
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone)]
#[serde(untagged)]
enum Packet {
    List(Vec<Packet>),
    Number(u32),
}

impl Packet {
    fn with_slice<T>(&self, f: impl FnOnce(&[Packet]) -> T) -> T {
        match self {
            Self::List(n) => f(&n[..]),
            Self::Number(n) => f(&[Self::Number(*n)]),
        }
    }
}

// credit to https://fasterthanli.me/series/advent-of-code-2022/part-13
// this solution was too elegant not to take
impl std::cmp::PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Packet::Number(a), Packet::Number(b)) => a.partial_cmp(b),
            (l, r) => Some(l.with_slice(|l| {
                r.with_slice(|r| {
                    l.iter()
                        .zip(r.iter())
                        .map(|(aa, bb)| aa.cmp(bb))
                        // return the first ordering that isn't `Equal`
                        .find(|&ord| ord != Ordering::Equal)
                        // or compare the lengths
                        .unwrap_or_else(|| l.len().cmp(&r.len()))
                })
            })),
        }
    }
}

impl std::cmp::Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
