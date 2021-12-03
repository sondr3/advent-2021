use super::AoC;

pub struct Day03;

impl AoC for Day03 {
    type Output = usize;
    type Input = Vec<char>;

    fn parse(input: &str) -> Vec<Self::Input> {
        input.lines().map(|l| l.chars().collect()).collect()
    }

    fn part_one(input: &[Self::Input]) -> usize {
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

    fn part_two(input: &[Self::Input]) -> usize {
        todo!()
    }
}

#[cfg(test)]
mod day02 {
    use super::*;

    const INPUT: &str = include_str!("../../inputs/day03.txt");

    #[test]
    fn part_1_example() {
        let data = Day03::parse(
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010",
        );
        assert_eq!(198, Day03::part_one(&data));
    }

    #[test]
    fn part_1() {
        let data = Day03::parse(INPUT);
        assert_eq!(1997414, Day03::part_one(&data));
    }

    #[test]
    #[ignore]
    fn part_2_example() {
        let data = Day03::parse("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2");
        assert_eq!(900, Day03::part_two(&data));
    }

    #[test]
    #[ignore]
    fn part_2() {
        let data = Day03::parse(INPUT);
        assert_eq!(1864715580, Day03::part_two(&data));
    }
}
