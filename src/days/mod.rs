mod day01;
mod day02;
mod day03;

trait AoC {
    type Output: std::fmt::Display;
    type Input;

    fn parse(input: &str) -> Vec<Self::Input>;
    fn part_one(input: &[Self::Input]) -> Self::Output;
    fn part_two(input: &[Self::Input]) -> Self::Output;
    fn solve(input: &str) -> (String, String) {
        let data = Self::parse(input);

        (
            Self::part_one(&data).to_string(),
            Self::part_two(&data).to_string(),
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
