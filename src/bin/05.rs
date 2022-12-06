use std::collections::VecDeque;
use std::num::ParseIntError;

use lazy_static::lazy_static;
use regex::Regex;

pub fn part_one(input: &str) -> Option<String> {
    let v = input.split("\n\n").collect::<Vec<_>>();
    let mut supplies = Supplies::try_from(v.first().unwrap().to_owned()).unwrap();

    for line in v.last().unwrap().to_owned().lines() {
        supplies.do_op(Move::try_from(line).unwrap(), Crane::CrateMover9000);
    }

    let mut s = String::new();
    
    for stack in supplies.stacks.iter() {
        if let Some(c) = stack.iter().last() {
            s.push(*c);
        }
    }

    Some(s)
}

pub fn part_two(input: &str) -> Option<String> {
    let v = input.split("\n\n").collect::<Vec<_>>();
    let mut supplies = Supplies::try_from(v.first().unwrap().to_owned()).unwrap();

    for line in v.last().unwrap().to_owned().lines() {
        supplies.do_op(Move::try_from(line).unwrap(), Crane::CrateMover9001);
    }

    let mut s = String::new();
    
    for stack in supplies.stacks.iter() {
        if let Some(c) = stack.iter().last() {
            s.push(*c);
        }
    }

    Some(s)
}

enum Crane {
    CrateMover9000,
    CrateMover9001
}

#[derive(Debug)]
struct Supplies {
    stacks: Vec<VecDeque<char>>,
}

impl Supplies {
    fn new(stacks: Vec<VecDeque<char>>) -> Self {
        Supplies { stacks }
    }

    fn do_op(&mut self, op: Move, crane: Crane) {
        match crane {
            Crane::CrateMover9000 => {
                for _ in 1..=op.count {
                    let c = self.stacks[op.from].pop_back().unwrap();
                    self.stacks[op.to].push_back(c);
                }
            },
            Crane::CrateMover9001 => {
                let mut tmp: VecDeque<char> = VecDeque::new();
                for _ in 1..=op.count {
                    let c = self.stacks[op.from].pop_back().unwrap();
                    tmp.push_back(c);
                }

                for c in tmp.iter().rev() {
                    self.stacks[op.to].push_back(*c);
                }
            }
        }
    }
}

impl TryFrom<&str> for Supplies {
    type Error = ParseIntError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        // The length of each line in the stack diagram
        // is len = 3 * n + (n - 1) = 4n - 1
        //    => n = (len + 1) / 4
        let n: usize = (s.lines().next().unwrap().len() + 1) / 4;
        let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); n];

        'outer: for line in s.lines() {
            for (i, j) in (1..line.len()).step_by(4).enumerate() {
                let c = line.as_bytes()[j] as char;
                if c.is_alphabetic() {
                    stacks[i].push_front(c);
                } else if c != ' ' {
                    break 'outer;
                }
            }
        }

        Ok(Supplies::new(stacks))
    }
}

struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn new(count: usize, from: usize, to: usize) -> Self {
        Move { count, from, to }
    }
}

impl TryFrom<&str> for Move {
    type Error = ParseIntError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        }

        let cap = RE.captures(s).unwrap();

        Ok(Move::new(
            cap[1].parse::<usize>()?,
            cap[2].parse::<usize>()? - 1,
            cap[3].parse::<usize>()? - 1,
        ))
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
