#![allow(dead_code)]

pub fn main() {
    // let filename = "day_3_test_data.txt";
    let filename = "day_3_data.txt";
    let data = parse(filename);
    solve_2(data);
}

fn parse(filename: &str) -> Vec<String> {
    std::fs::read_to_string(filename)
        .unwrap()
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}

type Matrix = Vec<Vec<char>>;

fn char_matrix_from_strings(bit_strings: &[String]) -> Matrix {
    bit_strings
        .into_iter()
        .map(|x| x.chars().collect())
        .collect()
}

fn get_oxygen_rating(common_bits: Vec<char>, matrix: Matrix) -> String {
    // get most common value for position
    // only keep numbers with that bit
    // if equal amount, prefer 1

    fn solve_rec(commons: Vec<char>, mat: Matrix, pos: usize) -> Matrix {
        // filter by pos bit of every str in mat
        if mat.len() <= 1 {
            return mat;
        }
        let new_mat = mat
            .into_iter()
            .filter(|x| x[pos] == commons[pos])
            .collect::<Matrix>();

        let new_commons = common_bits_from_matrix_oxygen(&new_mat);

        solve_rec(new_commons, new_mat, pos + 1)
    }

    String::from_iter(solve_rec(common_bits, matrix, 0).pop().unwrap())
}

fn get_c02_rating(common_bits: Vec<char>, matrix: Matrix) -> String {
    // get most common value for position
    // only keep numbers with that bit
    // if equal amount, prefer 1

    fn solve_rec(commons: Vec<char>, mat: Matrix, pos: usize) -> Matrix {
        // filter by pos bit of every str in mat
        if mat.len() <= 1 {
            return mat;
        }

        let new_mat = mat
            .into_iter()
            .filter(|x| x[pos] == commons[pos])
            .collect::<Matrix>();

        let new_commons = common_bits_from_matrix_c02(&new_mat);

        solve_rec(new_commons, new_mat, pos + 1)
    }

    String::from_iter(solve_rec(common_bits, matrix, 0).pop().unwrap())
}

fn solve_2(data: Vec<String>) {
    let matrix = char_matrix_from_strings(&data);
    let common_bits = get_common_bits_for_oxygen(&data);
    let oxygen = get_oxygen_rating(common_bits, matrix);
    dbg!(&oxygen);
    dbg!(str_to_bin(&oxygen));

    let matrix = char_matrix_from_strings(&data);
    let common_bits = get_common_bits_for_c02(&data);
    let c02 = get_c02_rating(common_bits, matrix);

    dbg!(&c02);
    dbg!(str_to_bin(&c02));

    dbg!(str_to_bin(&c02) * str_to_bin(&oxygen));
}

fn common_bits_from_matrix_oxygen(matrix: &Matrix) -> Vec<char> {
    get_common_bits_for_oxygen(
        &matrix
            .into_iter()
            .map(|x| String::from_iter(x))
            .collect::<Vec<String>>(),
    )
}

fn get_common_bits_for_c02(data: &[String]) -> Vec<char> {
    get_common_bits_for_oxygen(data)
        .into_iter()
        .map(|x| match x {
            '0' => '1',
            '1' => '0',
            _ => panic!(),
        })
        .collect()
}

fn common_bits_from_matrix_c02(matrix: &Matrix) -> Vec<char> {
    get_common_bits_for_oxygen(
        &matrix
            .into_iter()
            .map(|x| String::from_iter(x))
            .collect::<Vec<String>>(),
    )
    .into_iter()
    .map(|x| match x {
        '0' => '1',
        '1' => '0',
        _ => panic!(),
    })
    .collect()
}

fn get_common_bits_for_oxygen(data: &[String]) -> Vec<char> {
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
    occurences
        .into_iter()
        .map(|x| {
            if x > 0 {
                '1'
            } else if x < 0 {
                '0'
            } else {
                '1'
            }
        })
        .collect()
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

fn str_to_bin(num: &str) -> isize {
    isize::from_str_radix(&num, 2).unwrap()
}
