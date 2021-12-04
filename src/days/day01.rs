use super::AoC;

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

#[cfg(test)]
mod day01 {
    use super::*;

    const INPUT: &str = include_str!("../../inputs/day01.txt");

    #[test]
    fn part_1_example() {
        let data = Day01::parse("199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n");
        assert_eq!(7, Day01::part_one(data));
    }

    #[test]
    fn part_1() {
        let data = Day01::parse(INPUT);
        assert_eq!(1529, Day01::part_one(data));
    }

    #[test]
    fn part_2_example() {
        let data = Day01::parse("199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n");
        assert_eq!(5, Day01::part_two(data));
    }

    #[test]
    fn part_2() {
        let data = Day01::parse(INPUT);
        assert_eq!(1567, Day01::part_two(data));
    }
}
