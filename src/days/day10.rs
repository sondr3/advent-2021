use std::collections::VecDeque;

use crate::{days::AoC, tests};

pub struct Day10;

fn brace(stack: &mut VecDeque<char>, char: char) -> Option<usize> {
    match char {
        '(' | '[' | '{' | '<' => stack.push_back(char),
        _ => match (stack.pop_back(), char) {
            (Some('('), ')') | (Some('['), ']') | (Some('{'), '}') | (Some('<'), '>') => {}
            (_, ')') => return Some(3),
            (_, ']') => return Some(57),
            (_, '}') => return Some(1197),
            (_, '>') => return Some(25137),
            _ => unreachable!(),
        },
    }

    None
}

fn first_missing(input: &[char]) -> usize {
    let mut stack = VecDeque::new();

    input.iter().filter_map(|&c| brace(&mut stack, c)).sum()
}

fn autocomplete(input: &[char]) -> usize {
    let mut stack = VecDeque::new();

    if input
        .iter()
        .map(|&c| brace(&mut stack, c))
        .any(|o| o.is_some())
    {
        return 0;
    }

    stack.iter().rev().fold(0, |acc, p| match p {
        '(' => (acc * 5) + 1,
        '[' => (acc * 5) + 2,
        '{' => (acc * 5) + 3,
        '<' => (acc * 5) + 4,
        _ => unreachable!(),
    })
}

impl AoC for Day10 {
    type Output = usize;
    type Input = Vec<Vec<char>>;

    fn parse(input: &str) -> Self::Input {
        input.trim().lines().map(|l| l.chars().collect()).collect()
    }

    fn part_one(input: Self::Input) -> Self::Output {
        let points: Vec<_> = input.iter().map(|l| first_missing(l)).collect();

        points.iter().sum()
    }

    fn part_two(input: Self::Input) -> Self::Output {
        let mut points: Vec<_> = input
            .iter()
            .map(|l| autocomplete(l))
            .filter(|&i| i != 0)
            .collect();
        points.sort_unstable();

        points[points.len() / 2]
    }
}

tests!(Day10, 26397, 323691, 288957, 2858785164);
