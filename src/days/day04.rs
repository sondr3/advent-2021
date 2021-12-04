use super::AoC;

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

#[cfg(test)]
mod day04 {
    use super::*;

    const INPUT: &str = include_str!("../../inputs/day04.txt");

    fn input() -> String {
        r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#
            .to_string()
    }

    #[test]
    fn part_1_example() {
        let data = Day04::parse(&input());
        assert_eq!(4512, Day04::part_one(data));
    }

    #[test]
    fn part_1() {
        let data = Day04::parse(INPUT);
        assert_eq!(27027, Day04::part_one(data));
    }

    #[test]
    fn part_2_example() {
        let data = Day04::parse(&input());
        assert_eq!(1924, Day04::part_two(data));
    }

    #[test]
    fn part_2() {
        let data = Day04::parse(INPUT);
        assert_eq!(36975, Day04::part_two(data));
    }
}
