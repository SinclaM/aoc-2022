pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split("\n\n")
            .map(|s| s.lines().map(|n| n.parse::<u32>().unwrap()).sum::<u32>())
            .max()
            .unwrap(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let vals: Vec<u32> = input
        .split("\n\n")
        .map(|s| s.lines().map(|n| n.parse::<u32>().unwrap()).sum::<u32>())
        .collect();

    let mut max_three = [u32::MIN, u32::MIN, u32::MIN];

    for n in vals {
        if n > max_three[0] {
            max_three[2] = max_three[1];
            max_three[1] = max_three[0];
            max_three[0] = n;
        } else if n > max_three[1] {
            max_three[2] = max_three[1];
            max_three[1] = n;
        } else if n > max_three[2] {
            max_three[2] = n;
        }
    }

    Some(max_three.iter().sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
