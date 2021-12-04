mod day01;
mod day02;
mod day03;
mod day04;

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
        _ => ("Not implemented".to_string(), "Not implemented".to_string()),
    }
}
