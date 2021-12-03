use super::AoC;

pub struct Day02;

pub enum Move {
    Forward(usize),
    Down(usize),
    Up(usize),
}

impl From<&str> for Move {
    fn from(input: &str) -> Self {
        match input.split(' ').collect::<Vec<_>>()[..] {
            ["forward", num] => Move::Forward(num.parse().unwrap()),
            ["down", num] => Move::Down(num.parse().unwrap()),
            ["up", num] => Move::Up(num.parse().unwrap()),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Pos {
    x: usize,
    y: usize,
    aim: usize,
}

impl AoC for Day02 {
    type Output = usize;
    type Input = Move;

    fn parse(input: &str) -> Vec<Self::Input> {
        input.lines().map(Move::from).collect()
    }

    fn part_one(input: &[Self::Input]) -> Self::Output {
        let pos = input
            .iter()
            .fold(Pos { x: 0, y: 0, aim: 0 }, |pos, m| match m {
                Move::Forward(val) => Pos {
                    x: pos.x + val,
                    y: pos.y,
                    aim: 0,
                },
                Move::Down(val) => Pos {
                    x: pos.x,
                    y: pos.y + val,
                    aim: 0,
                },
                Move::Up(val) => Pos {
                    x: pos.x,
                    y: pos.y - val,
                    aim: 0,
                },
            });

        pos.x * pos.y
    }

    fn part_two(input: &[Self::Input]) -> Self::Output {
        let pos = input
            .iter()
            .fold(Pos { x: 0, y: 0, aim: 0 }, |pos, m| match m {
                Move::Forward(val) => Pos {
                    x: pos.x + val,
                    y: pos.y + val * pos.aim,
                    aim: pos.aim,
                },
                Move::Down(val) => Pos {
                    x: pos.x,
                    y: pos.y,
                    aim: pos.aim + val,
                },
                Move::Up(val) => Pos {
                    x: pos.x,
                    y: pos.y,
                    aim: pos.aim - val,
                },
            });

        pos.x * pos.y
    }
}

#[cfg(test)]
mod day02 {
    use super::*;

    const INPUT: &str = include_str!("../../inputs/day02.txt");

    #[test]
    fn part_1_example() {
        let data = Day02::parse("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2");
        assert_eq!(150, Day02::part_one(&data));
    }

    #[test]
    fn part_1() {
        let data = Day02::parse(INPUT);
        assert_eq!(2215080, Day02::part_one(&data));
    }

    #[test]
    fn part_2_example() {
        let data = Day02::parse("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2");
        assert_eq!(900, Day02::part_two(&data));
    }

    #[test]
    fn part_2() {
        let data = Day02::parse(INPUT);
        assert_eq!(1864715580, Day02::part_two(&data));
    }
}
