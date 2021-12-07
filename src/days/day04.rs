use crate::{days::AoC, tests};

pub struct Day04;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Marked(usize),
    Unmarked(usize),
}

impl Cell {
    fn is_marked(&self) -> bool {
        match self {
            Cell::Marked(_) => true,
            Cell::Unmarked(_) => false,
        }
    }

    fn mark(self, num: usize) -> Self {
        if Cell::Unmarked(num) == self {
            Cell::Marked(num)
        } else {
            self
        }
    }

    fn value(&self) -> usize {
        match self {
            Cell::Marked(val) | Cell::Unmarked(val) => *val,
        }
    }
}

#[derive(Debug)]
struct Board(Vec<Vec<Cell>>);

impl Board {
    fn sum(&self) -> usize {
        self.0
            .iter()
            .map(|r| {
                r.iter()
                    .filter(|r| !r.is_marked())
                    .map(Cell::value)
                    .sum::<usize>()
            })
            .sum()
    }

    fn mark(&mut self, num: usize) {
        self.0 = self
            .0
            .iter_mut()
            .map(|r| r.iter_mut().map(|i| i.mark(num)).collect())
            .collect();
    }

    fn won(&self) -> bool {
        for row in &self.0 {
            if row.iter().all(Cell::is_marked) {
                return true;
            }
        }

        let mut won = false;

        for col in 0..5 {
            if self.0.iter().map(|r| r[col]).all(|c| c.is_marked()) {
                won = true;
            }
        }

        won
    }
}

#[derive(Debug)]
pub struct Bingo {
    draw: Vec<usize>,
    boards: Vec<Board>,
}

impl From<&str> for Bingo {
    fn from(input: &str) -> Self {
        let lines: Vec<_> = input
            .lines()
            .into_iter()
            .filter(|l| !l.trim().is_empty())
            .collect();

        let draw: Vec<usize> = lines[0]
            .split(',')
            .map(|c| c.parse().unwrap())
            .rev()
            .collect();

        let boards: Vec<Board> = lines[1..]
            .chunks(5)
            .map(|b| {
                Board(
                    b.iter()
                        .map(|r| {
                            r.split(' ')
                                .into_iter()
                                .map(str::trim)
                                .filter(|i| !i.is_empty())
                                .map(|i| Cell::Unmarked(i.parse().unwrap()))
                                .collect()
                        })
                        .collect(),
                )
            })
            .collect();

        Bingo { draw, boards }
    }
}

impl Bingo {
    fn draw(&mut self) -> Option<usize> {
        self.draw.pop()
    }

    fn mark(&mut self, num: usize) {
        self.boards.iter_mut().for_each(|b| b.mark(num));
    }

    fn board_won(&self) -> Option<usize> {
        for (i, board) in self.boards.iter().enumerate() {
            if board.won() {
                return Some(i);
            }
        }

        None
    }

    fn winners(&self) -> Vec<bool> {
        self.boards.iter().map(Board::won).collect()
    }
}

impl AoC for Day04 {
    type Output = usize;
    type Input = Bingo;

    fn parse(input: &str) -> Self::Input {
        Bingo::from(input)
    }

    fn part_one(mut input: Self::Input) -> Self::Output {
        while let Some(draw) = input.draw() {
            input.mark(draw);

            if let Some(board) = input.board_won() {
                return draw * input.boards[board].sum();
            }
        }

        unreachable!()
    }

    fn part_two(mut input: Self::Input) -> Self::Output {
        let mut winners = vec![false; input.boards.len()];

        while let Some(draw) = input.draw() {
            input.mark(draw);

            let current = input.winners();

            if current.iter().all(|i| *i) {
                let board: Vec<_> = winners
                    .iter()
                    .zip(current.iter())
                    .enumerate()
                    .filter_map(|(i, (p, c))| if p != c { Some(i) } else { None })
                    .collect();

                return draw * input.boards[board[0]].sum();
            } else {
                winners = current;
            }
        }

        unreachable!()
    }
}

tests!(Day04, 4512, 27027, 1924, 36975);
