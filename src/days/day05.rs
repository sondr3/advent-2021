use std::{collections::HashMap, ops::RangeInclusive, str::FromStr};

use super::AoC;

pub struct Segment {
    x1: isize,
    y1: isize,
    x2: isize,
    y2: isize,
}

impl Segment {
    fn abs_eq(&self) -> bool {
        let x = self.x2 - self.x1;
        let y = self.y2 - self.y1;

        x.abs() == y.abs()
    }
}

impl FromStr for Segment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p1, p2) = s.split_once(" -> ").unwrap();
        let (x1, y1) = p1.split_once(",").unwrap();
        let (x2, y2) = p2.split_once(",").unwrap();

        Ok(Segment {
            x1: x1.parse().unwrap(),
            y1: y1.parse().unwrap(),
            x2: x2.parse().unwrap(),
            y2: y2.parse().unwrap(),
        })
    }
}

pub struct Day05;

fn range(a: isize, b: isize) -> RangeInclusive<isize> {
    if b > a {
        a..=b
    } else {
        b..=a
    }
}

fn diagonals(lines: &[Segment]) -> HashMap<(isize, isize), isize> {
    let mut points = HashMap::new();

    for seg in lines {
        if seg.x1 == seg.x2 {
            for p in range(seg.y1, seg.y2).map(|y| (seg.x1, y)) {
                *points.entry(p).or_insert(0) += 1;
            }
        } else if seg.y1 == seg.y2 {
            for p in range(seg.x1, seg.x2).map(|x| (x, seg.y1)) {
                *points.entry(p).or_insert(0) += 1;
            }
        }
    }

    points
}

impl AoC for Day05 {
    type Output = usize;
    type Input = Vec<Segment>;

    fn parse(input: &str) -> Self::Input {
        input.lines().map(str::parse).map(Result::unwrap).collect()
    }

    fn part_one(input: Self::Input) -> Self::Output {
        diagonals(&input).values().filter(|p| **p >= 2).count()
    }

    fn part_two(input: Self::Input) -> Self::Output {
        let mut points = diagonals(&input);

        for seg in input {
            if seg.abs_eq() {
                let dx = (seg.x2 - seg.x1).signum();
                let dy = (seg.y2 - seg.y1).signum();
                let (mut x, mut y) = (seg.x1, seg.y1);
                while (x, y) != (seg.x2 + dx, seg.y2 + dy) {
                    *points.entry((x, y)).or_insert(0) += 1;
                    x += dx;
                    y += dy;
                }
            }
        }

        points.values().filter(|p| **p >= 2).count()
    }
}

#[cfg(test)]
mod day05 {
    use super::*;

    const INPUT: &str = include_str!("../../inputs/day05.txt");

    fn input() -> String {
        r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#
            .to_string()
    }

    #[test]
    fn part_1_example() {
        let data = Day05::parse(&input());
        assert_eq!(5, Day05::part_one(data));
    }

    #[test]
    fn part_1() {
        let data = Day05::parse(INPUT);
        assert_eq!(7380, Day05::part_one(data));
    }

    #[test]
    fn part_2_example() {
        let data = Day05::parse(&input());
        assert_eq!(12, Day05::part_two(data));
    }

    #[test]
    fn part_2() {
        let data = Day05::parse(INPUT);
        assert_eq!(12, Day05::part_two(data));
    }
}
