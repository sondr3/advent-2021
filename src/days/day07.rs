use crate::{days::AoC, tests};

pub struct Day07;

fn median(vec: &[isize]) -> isize {
    let mid = vec.len() / 2;
    if vec.len() % 2 == 0 {
        vec[mid]
    } else {
        (vec[mid] + vec[mid - 1]) / 2
    }
}

impl AoC for Day07 {
    type Output = isize;
    type Input = Vec<isize>;

    fn parse(input: &str) -> Self::Input {
        input
            .trim()
            .split(',')
            .map(str::parse)
            .map(Result::unwrap)
            .collect()
    }

    fn part_one(mut input: Self::Input) -> Self::Output {
        input.sort_unstable();
        let median = median(&input);

        input.iter().map(|c| (c - median).abs()).sum()
    }

    fn part_two(input: Self::Input) -> Self::Output {
        let min = input.iter().min().unwrap();
        let max = input.iter().max().unwrap();

        (*min..*max)
            .map(|i| {
                input
                    .iter()
                    .map(|c| (c - i).abs())
                    .map(|c| c * (c + 1) / 2)
                    .sum()
            })
            .min()
            .unwrap()
    }
}

tests!(Day07, 37, 352331, 168, 99266250);
