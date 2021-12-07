use crate::{days::AoC, tests};

pub struct Day01;

impl AoC for Day01 {
    type Output = usize;
    type Input = Vec<usize>;

    fn parse(input: &str) -> Self::Input {
        input.lines().map(|i| i.parse().unwrap()).collect()
    }

    fn part_one(input: Self::Input) -> Self::Output {
        input
            .windows(2)
            .map(|w| if w[1] > w[0] { 1 } else { 0 })
            .sum()
    }

    fn part_two(input: Self::Input) -> Self::Output {
        Self::part_one(input.windows(3).map(|w| w.iter().sum()).collect::<Vec<_>>())
    }
}

tests!(Day01, 7, 1529, 5, 1567);
