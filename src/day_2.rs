pub fn main() {
    let filename = "day_2_data.txt";
    // let filename = "day_2_test_data.txt";
    let data = parse(filename);
    solve(data);
}

fn solve(instructions: Vec<Direction>) {
    println!(
        "{}",
        instructions
            .into_iter()
            .fold(Position::new(), |acc, x| acc.ingest_instruction(x))
            .finish()
    )
}

struct Position {
    depth: u32,
    horizontal: u32,
}

impl Position {
    fn new() -> Self {
        Self {
            depth: 0,
            horizontal: 0,
        }
    }

    fn ingest_instruction(mut self, instruction: Direction) -> Self {
        match instruction {
            Direction::Forward(amount) => self.horizontal += amount,
            Direction::Up(amount) => self.depth -= amount,
            Direction::Down(amount) => self.depth += amount,
        };
        self
    }

    fn finish(self) -> String {
        format!(
            "horizontal: {}, depth: {}, mult: {}",
            self.horizontal,
            self.depth,
            self.depth * self.horizontal
        )
    }
}

#[derive(Debug)]
enum Direction {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl Direction {
    fn from_str(command_str: &str) -> Self {
        // assumes the format "direction u32"
        let (direction, amount): (String, u32) = {
            let split_str = command_str
                .split(" ")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            (
                split_str[0].clone(),
                str::parse::<u32>(&split_str[1]).unwrap(),
            )
        };

        match direction.as_ref() {
            "forward" => Self::Forward(amount),
            "down" => Self::Down(amount),
            "up" => Self::Up(amount),
            _ => panic!(),
        }
    }
}

fn parse(filename: &str) -> Vec<Direction> {
    std::fs::read_to_string(filename)
        .unwrap()
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| Direction::from_str(x))
        .collect()
}
