use crate::{days::AoC, tests};

fn get_pos(grid: &[Vec<usize>], x: usize, y: usize) -> Option<usize> {
    match grid.get(y) {
        Some(row) => row.get(x).copied(),
        None => None,
    }
}

fn surrounding(grid: &[Vec<usize>], x: usize, y: usize) -> Option<usize> {
    let point = match get_pos(grid, x, y) {
        Some(p) => p,
        None => return None,
    };

    let points = vec![
        get_pos(grid, x, y + 1),
        get_pos(grid, x, ((y as isize) - 1) as usize),
        get_pos(grid, ((x as isize) - 1) as usize, y),
        get_pos(grid, x + 1, y),
    ];

    points
        .into_iter()
        .flatten()
        .all(|p| point < p)
        .then(|| point)
}

fn low_points(grid: &[Vec<usize>]) -> Vec<(usize, usize)> {
    let height = grid.len();
    let width = grid[0].len();

    let mut low_points = Vec::new();
    for y in 0..height {
        for x in 0..width {
            if surrounding(grid, x, y).is_some() {
                low_points.push((x, y));
            }
        }
    }

    low_points
}

fn fill(grid: &mut [Vec<usize>], visited: &mut Vec<Vec<bool>>, (x, y): (usize, usize)) -> usize {
    if y >= visited.len() || x >= visited[0].len() || visited[y][x] {
        return 0;
    };

    let pos = get_pos(grid, x, y);
    if pos == Some(9) || pos == None {
        return 0;
    };

    visited[y][x] = true;

    1 + fill(grid, visited, (x + 1, y))
        + fill(grid, visited, (((x as isize) - 1) as usize, y))
        + fill(grid, visited, (x, y + 1))
        + fill(grid, visited, (x, ((y as isize) - 1) as usize))
}

pub struct Day09;

impl AoC for Day09 {
    type Output = usize;
    type Input = Vec<Vec<usize>>;

    fn parse(input: &str) -> Self::Input {
        input
            .trim()
            .lines()
            .map(|l| {
                l.chars()
                    .map(|n| n.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect()
    }

    fn part_one(input: Self::Input) -> Self::Output {
        let low_points = low_points(&input);
        low_points.iter().map(|&(x, y)| input[y][x] + 1).sum()
    }

    fn part_two(mut input: Self::Input) -> Self::Output {
        let height = input.len();
        let width = input[0].len();

        let mut basins = Vec::new();
        let mut visited = vec![vec![false; width]; height];
        for x in 0..width {
            for y in 0..height {
                basins.push(fill(&mut input, &mut visited, (x, y)));
            }
        }

        basins.sort_unstable();

        // dbg!(&basins);

        basins.iter().rev().take(3).product()
    }
}

tests!(Day09, 15, 530, 1134, 1019494);
