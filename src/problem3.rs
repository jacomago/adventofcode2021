use std::path::Path;

use adventofcode2021::read_lines;

fn parse_line(input: String) -> Vec<u32> {
    input
        .chars()
        .into_iter()
        .map(|x| x.to_digit(2).unwrap() )
        .collect()
}

pub fn main() -> String {
    let filename = "./src/resources/input03.csv";
    let path = Path::new(filename);
    let display = path.display();

    // File hosts must exist in current path before this produces output
    let lines = match read_lines(filename) {
        Err(e) => panic!("couldn't open {}: {}", display, e),
        Ok(file) => file,
    };

    let mut line_count = 0;
    // input is 11 bit numbers
    let mut one_counts = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    for line in lines {
        if let Ok(article) = line {
            let numbers = parse_line(article);

            for index in 0..=(numbers.len() - 1) {
                one_counts[index] += numbers[index];
            }

            line_count += 1;
        }
    }

    let half_line_count = line_count / 2;

    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    let mut count = 0;

    for bit in one_counts.into_iter().rev() {
        let increment = 2_i32.pow(count);
        if bit > half_line_count {
            gamma_rate += increment;
        } else {
            epsilon_rate += increment;
        }
        count += 1;
    }

    return format!("a: {}, b: {b}", gamma_rate * epsilon_rate, b = 0);
}
