use std::num::ParseIntError;
use std::ops::RangeInclusive;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| Pair::try_from(line).unwrap())
            .filter(Pair::has_containment)
            .count() as u32
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| Pair::try_from(line).unwrap())
            .filter(Pair::has_overlap)
            .count() as u32
    )
}

struct Assignment(RangeInclusive<u32>);

impl TryFrom<&str> for Assignment {
    type Error = ParseIntError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let v = s.split("-").collect::<Vec<_>>();
        let start = v.first().unwrap().parse::<u32>()?;
        let end = v.last().unwrap().parse::<u32>()?;
        Ok(Assignment(start..=end))
    }
}

impl Assignment {
    fn contains(&self, other: &Assignment) -> bool {
        other.0.start() >= self.0.start() && other.0.end() <= self.0.end()
    }

    fn overlaps(&self, other: &Assignment) -> bool {
        self.0.start() <= other.0.end() && self.0.end() >= other.0.start()
    }
}

struct Pair {
    left: Assignment,
    right: Assignment
}

impl TryFrom<&str> for Pair {
    type Error = ParseIntError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let v = s.split(",").collect::<Vec<_>>();
        let left = Assignment::try_from(v.first().unwrap().to_owned())?;
        let right = Assignment::try_from(v.last().unwrap().to_owned())?;
        Ok(Pair::new(left, right))
    }
}

impl Pair {
    fn new(left: Assignment, right: Assignment) -> Self {
        Pair {
            left,
            right
        }
    }

    fn has_containment(&self) -> bool {
        self.left.contains(&self.right) || self.right.contains(&self.left)
    }

    fn has_overlap(&self) -> bool {
        self.left.overlaps(&self.right)
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
