pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| Round::from_str(line, &Part::PartOne).unwrap().score())
            .sum::<u32>()
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| Round::from_str(line, &Part::PartTwo).unwrap().score())
            .sum::<u32>()
    )
}

enum Part {
    PartOne,
    PartTwo
}

#[derive(Copy, Clone)]
enum RPS {
    Rock,
    Paper,
    Scissors
}

impl RPS {
    fn loser(&self) -> RPS {
        match self {
            RPS::Rock     => RPS::Scissors,
            RPS::Paper    => RPS::Rock,
            RPS::Scissors => RPS::Paper,
        }
    }

    fn victor(&self) -> RPS {
        match self {
            RPS::Rock     => RPS::Paper,
            RPS::Paper    => RPS::Scissors,
            RPS::Scissors => RPS::Rock,
        }
    }
}

struct Round {
    theirs : RPS,
    ours: RPS
}

impl Round {
    fn new(theirs: &RPS, ours: &RPS) -> Round {
        Round {
            theirs:  *theirs,
            ours  :  *ours
        }
    }

    fn from_str(s: &str, part: &Part) -> Option<Round> {
        let v = s.split(" ").collect::<Vec<_>>();

        let theirs = match v.first().unwrap().to_owned() {
            "A" => RPS::Rock,
            "B" => RPS::Paper,
            "C" => RPS::Scissors,
             _  => return None,
        };

        let ours = match part {
            Part::PartOne => match v.last().unwrap().to_owned() {
                "X" => RPS::Rock,
                "Y" => RPS::Paper,
                "Z" => RPS::Scissors,
                 _  => return None,
            },
            Part::PartTwo => match v.last().unwrap().to_owned() {
                "X" => theirs.loser(),
                "Y" => theirs,
                "Z" => theirs.victor(),
                 _  => return None,
            }
        };

        Some(Round::new(&theirs, &ours))
    }

    fn score(&self) -> u32 {
        let shape_score = match self.ours {
            RPS::Rock     => 1,
            RPS::Paper    => 2,
            RPS::Scissors => 3
        };
        
        let outcome_score = match (&self.ours, &self.theirs) {
            (RPS::Rock, RPS::Rock)         => 3,
            (RPS::Rock, RPS::Paper)        => 0,
            (RPS::Rock, RPS::Scissors)     => 6,
            (RPS::Paper, RPS::Paper)       => 3,
            (RPS::Paper, RPS::Scissors)    => 0,
            (RPS::Paper, RPS::Rock)        => 6,
            (RPS::Scissors, RPS::Scissors) => 3,
            (RPS::Scissors, RPS::Rock)     => 0,
            (RPS::Scissors, RPS::Paper)    => 6,
        };

        shape_score + outcome_score
    }
}


fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
