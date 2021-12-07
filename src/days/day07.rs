use super::AoC;

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

#[cfg(test)]
mod day06 {
    use super::*;

    const INPUT: &str = include_str!("../../inputs/day07.txt");

    fn input() -> String {
        "16,1,2,0,4,2,7,1,2,14".to_string()
    }

    #[test]
    fn part_1_example() {
        let data = Day07::parse(&input());
        assert_eq!(37, Day07::part_one(data));
    }

    #[test]
    fn part_1() {
        let data = Day07::parse(INPUT);
        assert_eq!(352331, Day07::part_one(data));
    }

    #[test]
    fn part_2_example() {
        let data = Day07::parse(&input());
        assert_eq!(168, Day07::part_two(data));
    }

    #[test]
    fn part_2() {
        let data = Day07::parse(INPUT);
        assert_eq!(99266250, Day07::part_two(data));
    }
}
