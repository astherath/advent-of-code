#![allow(dead_code)]
use std::fs;

type Nums = Vec<u32>;

pub fn main() {
    let filename = "data.txt";
    // let filename = "test_data.txt";
    let data = parse(filename);
    let resp = solve_sliding_window(data);
    dbg!(resp);
}

fn solve_sliding_window(data: Nums) -> usize {
    let mut last: u32 = data[0..3].into_iter().sum();
    data.windows(3)
        .filter(|x| {
            let window_sum = x.into_iter().sum();
            let has_increased = window_sum > last;
            last = window_sum;
            has_increased
        })
        .count()
}

fn parse(filename: &str) -> Nums {
    fs::read_to_string(filename)
        .unwrap()
        .split("\n")
        .filter(|x| x != &"")
        .map(|x| x.parse().unwrap())
        .collect()
}

fn _solve(data: Nums) -> usize {
    let mut last = data[0];
    data.into_iter()
        .filter(|x| {
            let has_increased = *x > last;
            last = *x;
            has_increased
        })
        .count()
}
