pub fn part_one(input: &str) -> Option<i32> {
    let mut cpu = CPU::new(input);

    let mut strength: i32 = 0;

    while cpu.tick().is_ok() {
        if (20 + cpu.cycle) % 40 == 0 {
            strength += cpu.register * (cpu.cycle as i32);
        }
    }

    Some(strength)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut cpu = CPU::new(input);
    let mut out = String::with_capacity(40 * 6);

    while cpu.tick().is_ok() {
        if ((cpu.cycle - 1) as i32 % 40 - cpu.register).abs() <= 1 {
            out.push('#');
        } else {
            out.push('.');
        }
        if cpu.cycle % 40 == 0 {
            out.push('\n');
        }
    }

    Some(out)
}

#[derive(Debug)]
enum Instruction {
    NOOP,
    ADDX,
}

impl TryFrom<&str> for Instruction {
    type Error = &'static str;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "noop" => Ok(Instruction::NOOP),
            "addx" => Ok(Instruction::ADDX),
            _ => Err("Unknown instruction"),
        }
    }
}

// Read the question wrong at first and though addx executed asynchronously.
// This schedule is unnessecary.
struct Schedule<T>
where
    T: Copy,
{
    jobs: Vec<(usize, T)>,
}

impl<T> Schedule<T>
where
    T: Copy,
{
    fn new() -> Self {
        Schedule { jobs: vec![] }
    }

    fn tick(&mut self) -> Vec<T> {
        // this will be a lot more ergonomic when drain_filter is stabilized
        let (ready, mut scheduled): (Vec<_>, Vec<_>) = self
            .jobs
            .iter()
            .copied()
            .partition(|&(count, _)| count == 0);

        for (count, _) in &mut scheduled {
            *count -= 1;
        }

        self.jobs = scheduled;
        ready.iter().map(|&(_, val)| val).collect::<Vec<_>>()
    }

    fn schedule(&mut self, val: T, delay_ticks: usize) {
        self.jobs.push((delay_ticks, val));
    }

    fn is_empty(&self) -> bool {
        self.jobs.is_empty()
    }
}

struct CPU {
    cycle: usize,
    register: i32,
    program: Vec<(Instruction, Option<i32>)>,
    schedule: Schedule<i32>,
}

impl CPU {
    fn new(source: &str) -> Self {
        CPU {
            cycle: 0,
            register: 1,
            program: source.lines().map(|line| CPU::decode(line)).rev().collect(),
            schedule: Schedule::new(),
        }
    }

    fn decode(s: &str) -> (Instruction, Option<i32>) {
        let v = s.split(" ").collect::<Vec<_>>();

        let instr = Instruction::try_from(v[0]).unwrap();
        let val = v.get(1).and_then(|&s| Some(s.parse::<i32>().unwrap()));

        (instr, val)
    }

    fn exec(&mut self, (instr, val): (Instruction, Option<i32>)) {
        match instr {
            Instruction::NOOP => {}
            Instruction::ADDX => {
                self.schedule.schedule(val.unwrap(), 1);
            }
        }
    }

    fn tick(&mut self) -> Result<(), ()> {
        self.register += self.schedule.tick().iter().sum::<i32>();

        self.cycle += 1;

        if self.schedule.is_empty() {
            let next = self.program.pop();
            self.exec(next.ok_or(())?);
        }

        Ok(())
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        let s = concat!(
            "##..##..##..##..##..##..##..##..##..##..\n",
            "###...###...###...###...###...###...###.\n",
            "####....####....####....####....####....\n",
            "#####.....#####.....#####.....#####.....\n",
            "######......######......######......####\n",
            "#######.......#######.......#######.....\n"
        )
        .to_string();

        assert_eq!(part_two(&input), Some(s));
    }
}
