use crate::{days::AoC, tests};

pub struct Day03;

fn find_oxygen(mut input: Vec<Vec<char>>, inverse: bool) -> usize {
    let mut i = 0;

    while input.len() > 1 {
        let ones = input.iter().filter(|l| l[i] == '1').count();
        let zeroes = input.iter().filter(|l| l[i] == '0').count();

        if ones >= zeroes {
            input.retain(|l| l[i] == if inverse { '1' } else { '0' });
        } else {
            input.retain(|l| l[i] == if inverse { '0' } else { '1' });
        }

        i += 1;
    }

    usize::from_str_radix(&input[0].iter().collect::<String>(), 2).unwrap()
}

impl AoC for Day03 {
    type Output = usize;
    type Input = Vec<Vec<char>>;

    fn parse(input: &str) -> Self::Input {
        input.lines().map(|l| l.chars().collect()).collect()
    }

    fn part_one(input: Self::Input) -> Self::Output {
        let len = input[0].len();
        let mut gamma = String::new();

        for i in 0..len {
            let ones = input.iter().filter(|l| l[i] == '1').count();
            let zeroes = input.iter().filter(|l| l[i] == '0').count();
            gamma.push(if ones > zeroes { '1' } else { '0' });
        }

        let gamma = usize::from_str_radix(&gamma, 2).unwrap();
        let epsilon = ((1 << len) - 1) ^ gamma;

        gamma * epsilon
    }

    fn part_two(input: Self::Input) -> Self::Output {
        let oxygen = find_oxygen(input.to_vec(), true);
        let scrubber = find_oxygen(input.to_vec(), false);

        oxygen * scrubber
    }
}

tests!(Day03, 198, 1997414, 230, 1032597);
