#![allow(dead_code)]

pub fn main() {
    let filename = "day_4_test_data.txt";
    // let filename = "day_4_data.txt";
    let data = parse(filename);
    solve(data);
}

fn parse(filename: &str) -> ParsedData {
    let lines: Vec<String> = std::fs::read_to_string(filename)
        .unwrap()
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.to_string())
        .collect();

    let nums = lines[0]
        .split(",")
        .map(|x| x.parse::<u16>().unwrap())
        .collect::<Vec<u16>>();

    const BOARD_SIZE: usize = 5;
    let boards = lines[1..]
        .into_iter()
        .fold(Vec::<Vec<String>>::new(), |mut acc, x| {
            if acc.is_empty() {
                acc.insert(0, vec![x.to_string()]);
            } else {
                let out_len = acc.len();
                if acc[out_len - 1].len() < BOARD_SIZE {
                    let inner_len = acc[out_len - 1].len();
                    acc[out_len - 1].insert(inner_len, x.to_string());
                } else {
                    acc.insert(out_len, vec![x.to_string()]);
                }
            }
            acc
        })
        .into_iter()
        .map(|x| Board::from_matrix_string(x))
        .collect();

    (nums, boards)
}

fn solve(mut data: ParsedData) {
    let (winning_board_index, num_won_with) = get_solved_board(&mut data).unwrap();
    dbg!(&winning_board_index);
    dbg!(&num_won_with);
    let board = &data.1[winning_board_index];
    let score = board.get_score(num_won_with);
    dbg!(&score);
}

fn get_solved_board(data: &mut ParsedData) -> Option<(usize, u16)> {
    for num in &data.0 {
        for (i, board) in data.1.iter_mut().enumerate() {
            let (_, has_won) = board.mark_number(*num);
            if has_won {
                return Some((i, *num));
            }
        }
    }

    None
}

impl Board {
    fn from_matrix_string(board: MatrixString) -> Self {
        Self {
            matrix: board
                .into_iter()
                .map(|x| {
                    x.split_whitespace()
                        .map(|x| (x.to_string().parse::<u16>().unwrap(), false))
                        .collect::<Vec<Num>>()
                })
                .collect::<BoardMatrix>(),
        }
    }

    fn mark_number(&mut self, num: u16) -> (&mut Self, bool) {
        // find number and mark
        self.matrix.iter_mut().flatten().for_each(|x| {
            if x.0 == num {
                x.1 = true
            }
        });

        let has_won = self.check_if_has_won();

        (self, has_won)
    }

    fn get_score(&self, final_num: u16) -> usize {
        (self
            .matrix
            .iter()
            .map(|x| x.iter())
            .flatten()
            .filter(|x| !x.1)
            .map(|x| x.0)
            .fold(0, |acc, x| x + acc)
            * final_num) as usize
    }

    fn check_if_has_won(&self) -> bool {
        // check all rows

        // check all cols
        let all_cols = self.matrix.iter().map(|x| x.iter().all(|x| x.1)).any(|x| x);

        let size = self.matrix.len(); // N x N grid guaranteed
        let all_rows = self
            .matrix
            .iter()
            .map(|x| {
                x.iter()
                    .enumerate()
                    .fold(vec![true; 5], |mut acc, x| {
                        let (i, val) = x;
                        let current = acc[i % 5];
                        acc.insert(i % size, val.1 && current);
                        acc
                    })
                    .into_iter()
                    .all(|x| x)
            })
            .any(|x| x);

        all_cols || all_rows
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt_str = self.matrix.iter().fold(String::new(), |mut acc, x| {
            acc.push_str(&format!("{:?}\n", x));
            acc
        });
        write!(f, "\n{}", fmt_str)
    }
}

type ParsedData = (Vec<u16>, Vec<Board>);
type MatrixString = Vec<String>;
type Num = (u16, bool);
type BoardMatrix = Vec<Vec<Num>>;

#[derive(Debug)]
struct Board {
    matrix: BoardMatrix,
}
