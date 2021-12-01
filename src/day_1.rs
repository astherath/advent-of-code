use std::fs;

type Nums = Vec<u32>;

pub fn main() {
    let filename = "data.txt";
    // let filename = "test_data.txt";
    let data = parse(filename);
    let resp = solve(data);
    dbg!(resp);
}

fn parse(filename: &str) -> Nums {
    fs::read_to_string(filename)
        .unwrap()
        .split("\n")
        .filter(|x| x != &"")
        .map(|x| x.parse().unwrap())
        .collect()
}

fn solve(data: Nums) -> u32 {
    let mut last = data[0];
    data.into_iter()
        .filter(|x| {
            let has_increased = *x > last;
            last = *x;

            has_increased
        })
        .count() as u32
}
