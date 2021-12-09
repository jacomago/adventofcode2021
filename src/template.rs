use std::path::Path;

use adventofcode2021::read_lines;

pub fn main() -> String {
    file_to_solution("./src/resources/input05.csv")
}

fn file_to_solution(filename: &str) -> String {
    let path = Path::new(filename);
    let display = path.display();

    // File hosts must exist in current path before this produces output
    let mut lines = match read_lines(filename) {
        Err(e) => panic!("couldn't open {}: {}", display, e),
        Ok(file) => file,
    };

    let mut games: Vec<Vec<Vec<i32>>> = vec![];
    let mut game: Vec<Vec<i32>> = vec![];

    for line in lines {
        if let Ok(article) = line {
        }
    }

    format!("a: {}, b: {b}", 0, b = 0)
}
#[test]
fn test_example() {
    assert_eq!(
        file_to_solution("./src/resources/test/input05.csv"),
        "a: 4512, b: 1924"
    );
}
