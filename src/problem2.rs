use std::path::Path;

use adventofcode2021::read_lines;

enum Direction {
    Forward,
    Up,
    Down,
    Error,
}

struct Instruction {
    direction: Direction,
    distance: i32,
}

fn parse_instruction(input: String) -> Instruction {
    let v: Vec<&str> = input.split(' ').collect();
    Instruction {
        direction: match v[0] {
            "forward" => Direction::Forward,
            "up" => Direction::Up,
            "down" => Direction::Down,
            &_ => Direction::Error,
        },
        distance: v[1].parse::<i32>().unwrap(),
    }
}

pub fn problem2() -> String {
    let filename = "./src/resources/input02.csv";
    let path = Path::new(filename);
    let display = path.display();

    // File hosts must exist in current path before this produces output
    let lines = match read_lines(filename) {
        Err(e) => panic!("couldn't open {}: {}", display, e),
        Ok(file) => file,
    };

    let mut horizontal_pos = 0;
    let mut depth_pos = 0;

    let mut aim = 0;
    let mut b_horiz_pos = 0;
    let mut b_depth_pos = 0;

    // Consumes the iterator, returns an (Optional) String
    for line in lines {
        if let Ok(article) = line {
            let instruction = parse_instruction(article);

            // Part A calculation
            match instruction.direction {
                Direction::Forward => horizontal_pos += instruction.distance,
                Direction::Down => depth_pos += instruction.distance,
                Direction::Up => depth_pos -= instruction.distance,
                Direction::Error => println!("Unable to parse string."),
            }

            // Part B calculation
            match instruction.direction {
                Direction::Forward => {
                    b_horiz_pos += instruction.distance;
                    b_depth_pos += aim * instruction.distance
                },
                Direction::Down => aim += instruction.distance,
                Direction::Up => aim -= instruction.distance,
                Direction::Error => println!("Unable to parse string."),
            }
        }
    }

    return format!(
        "a: {}, b: {b}",
        horizontal_pos * depth_pos,
        b = b_depth_pos * b_horiz_pos
    );
}
