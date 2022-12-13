use take_until::TakeUntilExt;

pub fn part_one(input: &str) -> Option<u32> {
    let forest = Forest::from(input);
    let mut count = 0;

    for i in 0..(forest.0.len()) {
        for j in 0..(forest.0[0].len()) {
            if forest.is_visible(i, j) {
                count += 1;
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let forest = Forest::from(input);
    let mut score: usize = 0;

    for i in 0..(forest.0.len()) {
        for j in 0..(forest.0[0].len()) {
            score = usize::max(score, forest.score(i, j));
        }
    }

    Some(score as u32)
}

#[derive(Debug)]
struct Forest(Vec<Vec<u8>>);

impl From<&str> for Forest {
    fn from(s: &str) -> Self {
        Forest(
            s.lines()
                .map(|line| {
                    line.chars()
                        .map(|c| c.to_digit(10).unwrap() as u8)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        )
    }
}

impl Forest {
    fn is_visible(&self, i: usize, j: usize) -> bool {
        let h = self.0[i][j];

        let mut left = self.0[i][..j].iter().filter(|&height| height >= &h);

        let mut right = self.0[i][(j + 1)..].iter().filter(|&height| height >= &h);

        let mut above = self
            .0
            .iter()
            .take(i)
            .map(|v| v[j])
            .filter(|&height| height >= h);

        let mut below = self
            .0
            .iter()
            .skip(i + 1)
            .map(|v| v[j])
            .filter(|&height| height >= h);

        left.next().is_none()
            || right.next().is_none()
            || above.next().is_none()
            || below.next().is_none()
    }

    fn score(&self, i: usize, j: usize) -> usize {
        let h = self.0[i][j];

        let left = self.0[i][..j]
            .iter()
            .rev()
            .take_until(|&height| height >= &h);

        let right = self.0[i][(j + 1)..]
            .iter()
            .take_until(|&height| height >= &h);

        let above = self
            .0
            .iter()
            .take(i)
            .map(|v| v[j])
            .rev()
            .take_until(|&height| height >= h);

        let below = self
            .0
            .iter()
            .skip(i + 1)
            .map(|v| v[j])
            .take_until(|&height| height >= h);

        left.count() * right.count() * above.count() * below.count()
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
