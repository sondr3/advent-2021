use super::AoC;

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

#[cfg(test)]
mod day03 {
    use super::*;

    const INPUT: &str = include_str!("../../inputs/day03.txt");

    #[test]
    fn part_1_example() {
        let data = Day03::parse(
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010",
        );
        assert_eq!(198, Day03::part_one(data));
    }

    #[test]
    fn part_1() {
        let data = Day03::parse(INPUT);
        assert_eq!(1997414, Day03::part_one(data));
    }

    #[test]
    fn part_2_example() {
        let data = Day03::parse(
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010",
        );
        assert_eq!(230, Day03::part_two(data));
    }

    #[test]
    fn part_2() {
        let data = Day03::parse(INPUT);
        assert_eq!(1032597, Day03::part_two(data));
    }
}
