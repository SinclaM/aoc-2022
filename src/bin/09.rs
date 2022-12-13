use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let mut rope = Rope::new(2);
    for line in input.lines() {
        let tokens = line.split(" ").collect::<Vec<_>>();
        let m = Motion::try_from(tokens[0]).unwrap();
        let count = tokens[1].parse::<usize>().unwrap();

        for _ in 0..count {
            rope.step(&m);
        }
    }

    Some(rope.visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut rope = Rope::new(10);
    for line in input.lines() {
        let tokens = line.split(" ").collect::<Vec<_>>();
        let m = Motion::try_from(tokens[0]).unwrap();
        let count = tokens[1].parse::<usize>().unwrap();

        for _ in 0..count {
            rope.step(&m);
        }
    }

    Some(rope.visited.len() as u32)
}

enum Motion {
    Right,
    Left,
    Up,
    Down,
}

impl TryFrom<&str> for Motion {
    type Error = &'static str;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "U" => Ok(Motion::Up),
            "D" => Ok(Motion::Down),
            "L" => Ok(Motion::Left),
            "R" => Ok(Motion::Right),
            _ => Err("String cannot be parsed into Motion"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

impl std::fmt::Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
    }
}

struct Rope {
    knots: Vec<Pos>,
    visited: HashSet<Pos>,
}

impl Rope {
    fn new(len: usize) -> Self {
        Rope {
            knots: vec![Pos::new(0, 0); len],
            visited: std::iter::once(Pos::new(0, 0)).collect(),
        }
    }

    fn step(&mut self, m: &Motion) {
        match m {
            Motion::Right => {
                self.knots.first_mut().unwrap().x += 1;
            }
            Motion::Left => {
                self.knots.first_mut().unwrap().x -= 1;
            }
            Motion::Up => {
                self.knots.first_mut().unwrap().y += 1;
            }
            Motion::Down => {
                self.knots.first_mut().unwrap().y -= 1;
            }
        }

        let mut last = 0;
        let mut curr = 1;

        while curr < self.knots.len() {
            let delta = Pos::new(
                self.knots[last].x - self.knots[curr].x,
                self.knots[last].y - self.knots[curr].y,
            );

            match delta {
                Pos { x, y } if x.abs() + y.abs() > 2 => {
                    self.knots[curr].x += x.signum();
                    self.knots[curr].y += y.signum();
                }
                Pos { x, y: 0 } if x.abs() == 2 => {
                    self.knots[curr].x += x.signum();
                }
                Pos { x: 0, y } if y.abs() == 2 => {
                    self.knots[curr].y += y.signum();
                }
                _ => {}
            }

            last = curr;
            curr += 1;
        }

        self.visited.insert(self.knots.last().unwrap().clone());
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(88));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(36));
    }
}
