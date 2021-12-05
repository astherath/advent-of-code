pub fn main() {
    // let filename = "day_3_test_data.txt";
    let filename = "day_3_data.txt";
    let data = parse(filename);
    solve(data);
}

fn parse(filename: &str) -> Vec<String> {
    std::fs::read_to_string(filename)
        .unwrap()
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}

fn solve(data: Vec<String>) {
    let cols = data[0].len();
    let mut occurences = vec![0; cols];

    data.join("").chars().enumerate().for_each(|(i, c)| {
        let val = match c {
            '0' => -1,
            '1' => 1,
            _ => panic!(),
        };
        occurences[i % cols] += val
    });

    let gamma_rate_str = String::from_iter(occurences.into_iter().map(|x| match x >= 0 {
        true => '1',
        false => '0',
    }));

    let gamma_rate = isize::from_str_radix(&gamma_rate_str, 2).unwrap();
    let epsilon_rate = isize::from_str_radix(
        &String::from_iter(gamma_rate_str.chars().map(|x| match x {
            '0' => '1',
            '1' => '0',
            _ => panic!(),
        })),
        2,
    )
    .unwrap();

    println!("resp: {}", gamma_rate * epsilon_rate);
}
