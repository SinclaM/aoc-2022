use lazy_static::lazy_static;
use regex::Regex;

pub fn part_one(input: &str) -> Option<usize> {
    let mut troop = Troop::from(input);
    while troop.round < 20 {
        troop.play_round(true);
    }

    let mut max = 0;
    let mut next_max = 0;

    for count in troop.monkeys.iter().map(|monkey| monkey.inspect_count) {
        if count > max {
            next_max = max;
            max = count;
        } else if count > next_max {
            next_max = count;
        }
    }


    Some((max * next_max) as usize)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut troop = Troop::from(input);
    while troop.round < 10000 {
        troop.play_round(false);
    }

    let mut max = 0;
    let mut next_max = 0;

    for count in troop.monkeys.iter().map(|monkey| monkey.inspect_count) {
        if count > max {
            next_max = max;
            max = count;
        } else if count > next_max {
            next_max = count;
        }
    }


    Some((max * next_max) as usize)
}

#[derive(Debug)]
enum VarType {
    Literal(usize),
    Old,
}

impl From<&str> for VarType {
    fn from(s: &str) -> Self {
        match s {
            "old" => VarType::Old,
            _ => VarType::Literal(s.parse().expect("should be integer literal")),
        }
    }
}

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
}

impl From<&str> for Operator {
    fn from(s: &str) -> Self {
        match s {
            "+" => Operator::Add,
            "*" => Operator::Multiply,
            _ => panic!("unknown operator: {}", s),
        }
    }
}

#[derive(Debug)]
struct Operation {
    lhs: VarType,
    op: Operator,
    rhs: VarType,
}

impl From<&str> for Operation {
    fn from(s: &str) -> Self {
        let v = s.splitn(3, " ").collect::<Vec<_>>();

        Operation {
            lhs: v[0].into(),
            op: v[1].into(),
            rhs: v[2].into(),
        }
    }
}

impl Operation {
    fn compute(&self, old: usize) -> usize {
        let left = match self.lhs {
            VarType::Old => old,
            VarType::Literal(x) => x,
        };

        let right = match self.rhs {
            VarType::Old => old,
            VarType::Literal(x) => x,
        };

        match self.op {
            Operator::Add => left + right,
            Operator::Multiply => left * right,
        }
    }
}

#[derive(Debug)]
struct Test {
    divisor: usize,
    on_true: usize,
    on_false: usize,
}

impl Test {
    fn new(divisor: usize, on_true: usize, on_false: usize) -> Self {
        Test {
            divisor,
            on_true,
            on_false,
        }
    }

    fn find_next(&self, worry_level: usize) -> usize {
        if worry_level % self.divisor == 0 {
            self.on_true
        } else {
            self.on_false
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test: Test,
    inspect_count: usize,
}

impl From<&str> for Monkey {
    fn from(s: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(concat!(
                r"Monkey (?P<id>\d+):\s*",
                r"Starting items: (?P<items>[\d, ]+)\s*",
                r"Operation: new = (?P<op>.*)\s*",
                r"Test: divisible by (?P<div>\d+)\s*",
                r"If true: throw to monkey (?P<on_true>\d+)\s*",
                r"If false: throw to monkey (?P<on_false>\d+)"
            ))
            .unwrap();
        }

        let caps = RE.captures(s).unwrap();
        let items = caps
            .name("items")
            .unwrap()
            .as_str()
            .split(", ")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let operation = Operation::from(caps.name("op").unwrap().as_str());
        let test = Test::new(
            caps.name("div").unwrap().as_str().parse().unwrap(),
            caps.name("on_true").unwrap().as_str().parse().unwrap(),
            caps.name("on_false").unwrap().as_str().parse().unwrap(),
        );

        Monkey {
            items,
            operation,
            test,
            inspect_count: 0
        }
    }
}

#[derive(Debug)]
struct Troop {
    monkeys: Vec<Monkey>,
    round: usize,
}

impl Troop {
    fn new(monkeys: Vec<Monkey>) -> Self {
        Troop {
            monkeys,
            round: 0
        }
    }

    fn play_round(&mut self, div_three: bool) {
        for i in 0..self.monkeys.len() {
            self.monkeys[i].inspect_count += self.monkeys[i].items.len();

            for j in 0..self.monkeys[i].items.len() {
                let mut worry_level = self.monkeys[i].items[j];
                worry_level = self.monkeys[i].operation.compute(worry_level);

                if div_three {
                    worry_level /= 3;
                } else {
                    let net_divisor: usize = self.monkeys.iter().map(|m| m.test.divisor).product();
                    worry_level %= net_divisor;
                }


                let id = self.monkeys[i].test.find_next(worry_level);
                assert_ne!(id, i);
                self.monkeys[id].items.push(worry_level);
            }

            self.monkeys[i].items.clear();
        }

        self.round += 1;
    }
}

impl From<&str> for Troop {
    fn from(s: &str) -> Self {
        let monkeys = s.split("\n\n").map(|lines| Monkey::from(lines)).collect::<Vec<_>>();
        Troop::new(monkeys)
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
