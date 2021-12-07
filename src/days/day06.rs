use crate::{days::AoC, tests};

pub struct Day06;

fn simulate(fishes: &[usize], time: usize) -> usize {
    let mut counts = [0; 9];
    fishes.iter().for_each(|f| counts[*f] += 1);

    for _ in 0..time {
        counts.rotate_left(1);
        counts[6] += counts[8];
    }

    counts.iter().sum()
}

impl AoC for Day06 {
    type Output = usize;
    type Input = Vec<usize>;

    fn parse(input: &str) -> Self::Input {
        input
            .trim()
            .split(',')
            .map(str::parse)
            .map(Result::unwrap)
            .collect()
    }

    fn part_one(input: Self::Input) -> Self::Output {
        simulate(&input, 80)
    }

    fn part_two(input: Self::Input) -> Self::Output {
        simulate(&input, 256)
    }
}

tests!(Day06, 5934, 360268, 26984457539, 1632146183902);
