fn parse(input: &str) -> Vec<usize> {
    input.lines().map(|i| i.parse().unwrap()).collect()
}

fn part_one(input: &[usize]) -> usize {
    input
        .windows(2)
        .map(|w| if w[1] > w[0] { 1 } else { 0 })
        .sum()
}

fn part_two(input: &[usize]) -> usize {
    part_one(&input.windows(3).map(|w| w.iter().sum()).collect::<Vec<_>>())
}

pub fn day_one(input: &str) -> (String, String) {
    let data = parse(input);
    (part_one(&data).to_string(), part_two(&data).to_string())
}

#[cfg(test)]
mod day01 {
    use super::*;

    const INPUT: &str = include_str!("../../inputs/day01.txt");

    #[test]
    fn part_1_example() {
        let data = parse("199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n");
        assert_eq!(7, part_one(&data));
    }

    #[test]
    fn part_1() {
        let data = parse(INPUT);
        assert_eq!(1529, part_one(&data));
    }

    #[test]
    fn part_2_example() {
        let data = parse("199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n");
        assert_eq!(5, part_two(&data));
    }

    #[test]
    fn part_2() {
        let data = parse(INPUT);
        assert_eq!(1567, part_two(&data));
    }
}
