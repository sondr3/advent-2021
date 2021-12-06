use super::AoC;

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

#[cfg(test)]
mod day06 {
    use super::*;

    const INPUT: &str = include_str!("../../inputs/day06.txt");

    fn input() -> String {
        "3,4,3,1,2".to_string()
    }

    #[test]
    fn part_1_example() {
        let data = Day06::parse(&input());
        assert_eq!(5934, Day06::part_one(data));
    }

    #[test]
    fn part_1() {
        let data = Day06::parse(INPUT);
        assert_eq!(360268, Day06::part_one(data));
    }

    #[test]
    fn part_2_example() {
        let data = Day06::parse(&input());
        assert_eq!(26984457539, Day06::part_two(data));
    }

    #[test]
    fn part_2() {
        let data = Day06::parse(INPUT);
        assert_eq!(1632146183902, Day06::part_two(data));
    }
}
