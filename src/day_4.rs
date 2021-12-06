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

    dbg!(&boards);

    (nums, boards)
}

fn solve(data: ParsedData) {}

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

    fn mark_number(mut self, num: u16) -> Self {
        // find number and mark
        self.matrix.iter_mut().flatten().for_each(|x| {
            if x.0 == num {
                x.1 = true
            }
        });

        self
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
