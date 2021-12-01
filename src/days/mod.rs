mod day01;

pub fn run_day(day: usize, input: &str) -> (String, String) {
    match day {
        1 => day01::day_one(input),
        _ => ("Not implemented".to_string(), "Not implemented".to_string()),
    }
}
