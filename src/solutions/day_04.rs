use crate::common::{self, Solution};

pub struct Day04 {}

impl Solution for Day04 {
    fn name(&self) -> String {
        "Giant Squid".to_owned()
    }

    fn part_a(&self) -> String {
        let bingo = Bingo::parse_input(common::load("04"));
        let winning = bingo.solve();

        winning.0[winning.1].final_out(winning.2).to_string()
    }

    fn part_b(&self) -> String {
        let bingo = Bingo::parse_input(common::load("04"));
        let loseing = bingo.loseing_solve();

        loseing.0[loseing.1].final_out(loseing.2).to_string()
    }
}

#[derive(Debug, Clone)]
struct Bingo {
    numbers: Vec<u32>,
    boards: Vec<Board>,
    take: u32,
}

#[derive(Debug, Clone)]
struct Board {
    data: Vec<Vec<u32>>,
    checked: Vec<Vec<bool>>,
}

impl Bingo {
    fn parse_input(inp: String) -> Self {
        let mut lines = inp.lines();
        let numbers = lines
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        let boards = Board::parse(inp);

        Bingo {
            numbers,
            boards,
            take: 5,
        }
    }

    fn solve(self) -> (Vec<Board>, usize, u32) {
        let mut nums = self.numbers.clone();
        let mut tick = self.boards;
        let mut take = self.take;

        loop {
            for _ in 1..take {
                let num = nums.remove(0);
                tick = Board::tick(tick, num);

                if let Some(i) = Board::check(tick.clone()).first() {
                    return (tick, *i, num);
                };
            }

            take += 1;
        }
    }

    fn loseing_solve(self) -> (Vec<Board>, usize, u32) {
        let mut nums = self.numbers.clone();
        let mut tick = self.boards;
        let mut take = self.take;
        let mut ret = Vec::new();

        loop {
            for _ in 1..take {
                let num = nums.remove(0);
                tick = Board::tick(tick, num);

                let check = Board::check(tick.clone());

                for i in check.clone() {
                    if !ret.contains(&i) {
                        ret.push(i);
                    }
                }

                if ret.len() == tick.len() {
                    return (tick, *ret.last().unwrap(), num);
                }
            }

            take += 1;
        }
    }
}

impl Board {
    fn parse(inp: String) -> Vec<Self> {
        let mut boards = Vec::new();
        let inp = inp.replace('\r', "");
        let raw_boards = inp.split("\n\n").skip(1);

        for i in raw_boards {
            let mut data = Vec::new();
            let mut checked = Vec::new();

            for line in i.lines() {
                let nums = line
                    .split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<u32>>();

                data.push(nums.clone());
                checked.push(vec![false; nums.len()]);
            }

            boards.push(Board { data, checked });
        }
        boards
    }

    fn tick(data: Vec<Board>, num: u32) -> Vec<Board> {
        let mut data = data;
        for (board_i, board) in data.clone().iter().enumerate() {
            for (row_i, row) in board.data.iter().enumerate() {
                for (col_i, col) in row.iter().enumerate() {
                    if *col == num {
                        data[board_i].checked[row_i][col_i] = true;
                    }
                }
            }
        }

        data
    }

    fn check(data: Vec<Board>) -> Vec<usize> {
        let mut out = Vec::new();
        for (i, board) in data.iter().enumerate() {
            let base_row_count = board.data[0].len();
            let base_col_count = board.data.len();

            // Check rows
            for row in board.checked.iter() {
                let mut row_count = 0;
                for i in row.iter().take(base_col_count) {
                    if *i {
                        row_count += 1;
                    }
                }

                if row_count == base_row_count {
                    out.push(i);
                }
            }

            // Check columns
            for col_i in 0..base_col_count {
                let mut col_count = 0;
                for row_i in 0..base_row_count {
                    if board.checked[row_i][col_i] {
                        col_count += 1;
                    }
                }

                if col_count == base_row_count {
                    out.push(i);
                }
            }
        }
        out
    }

    // Get sum of all unchecked numbers
    fn final_out(&self, winning: u32) -> usize {
        let mut sum = 0;
        for (row_i, row) in self.data.iter().enumerate() {
            for (col_i, col) in row.iter().enumerate() {
                if !self.checked[row_i][col_i] {
                    sum += *col;
                }
            }
        }

        sum as usize * winning as usize
    }
}
