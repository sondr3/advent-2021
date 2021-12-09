mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day09;

#[macro_export]
macro_rules! tests {
    ($day: ident) => {
        #[allow(dead_code)]
        const INPUT: &str = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/inputs/",
            stringify!($day),
            ".txt"
        ));
        #[allow(dead_code)]
        const EXAMPLE: &str = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/inputs/",
            stringify!($day),
            "_ex",
            ".txt"
        ));
    };
    ($day: ident, $p1: expr) => {
        tests!($day);

        #[test]
        fn part_1_example() {
            assert_eq!($p1, $day::part_one($day::parse(EXAMPLE)));
        }
    };
    ($day: ident, $p1: expr, $a1: expr) => {
        tests!($day, $p1);

        #[test]
        fn part_1() {
            assert_eq!($a1, $day::part_one($day::parse(INPUT)));
        }
    };
    ($day: ident, $p1: expr, $a1: expr, $p2: expr) => {
        tests!($day, $p1, $a1);

        #[test]
        fn part_2_example() {
            assert_eq!($p2, $day::part_two($day::parse(EXAMPLE)));
        }
    };
    ($day: ident, $p1: expr, $a1: expr, $p2: expr, $a2: expr) => {
        tests!($day, $p1, $a1, $p2);

        #[test]
        fn part_2() {
            assert_eq!($a2, $day::part_two($day::parse(INPUT)));
        }
    };
}

trait AoC {
    type Output: std::fmt::Display;
    type Input;

    fn parse(input: &str) -> Self::Input;
    fn part_one(input: Self::Input) -> Self::Output;
    fn part_two(input: Self::Input) -> Self::Output;
    fn solve(input: &str) -> (String, String) {
        (
            Self::part_one(Self::parse(input)).to_string(),
            Self::part_two(Self::parse(input)).to_string(),
        )
    }
}

pub fn run_day(day: usize, input: &str) -> (String, String) {
    match day {
        1 => day01::Day01::solve(input),
        2 => day02::Day02::solve(input),
        3 => day03::Day03::solve(input),
        4 => day04::Day04::solve(input),
        5 => day05::Day05::solve(input),
        6 => day06::Day06::solve(input),
        7 => day07::Day07::solve(input),
        9 => day09::Day09::solve(input),
        _ => ("Not implemented".to_string(), "Not implemented".to_string()),
    }
}
