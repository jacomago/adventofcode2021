use std::{path::Path, vec};

use adventofcode2021::read_lines;

pub fn main() -> String {
    file_to_solution("./src/resources/input09.csv")
}

fn find_low_points(heightmap: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut low_points = vec![];
    for row in 0..heightmap.len() {
        for col in 0..heightmap[row].len() {
            if is_low_point(row, col, heightmap) {
                low_points.push(heightmap[row][col]);
            }
        }
    }
    low_points
}

fn surrounding_points(row: usize, col: usize, heightmap: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut output = vec![];
    if row > 0 {
        output.push(heightmap[row - 1][col]);
    }
    if col > 0 {
        output.push(heightmap[row][col - 1]);
    }
    if col < heightmap[row].len() - 1 {
        output.push(heightmap[row][col + 1]);
    }
    if row < heightmap.len() - 1 {
        output.push(heightmap[row + 1][col]);
    }
    output
}

fn is_low_point(row: usize, col: usize, heightmap: &Vec<Vec<i32>>) -> bool {
    let surrounding_points = surrounding_points(row, col, heightmap);
    if heightmap[row][col] < *surrounding_points.iter().min().unwrap() {
        return true;
    }
    false
}

fn file_to_solution(filename: &str) -> String {
    let path = Path::new(filename);
    let display = path.display();

    // File hosts must exist in current path before this produces output
    let lines = match read_lines(filename) {
        Err(e) => panic!("couldn't open {}: {}", display, e),
        Ok(file) => file,
    };

    let mut heightmap: Vec<Vec<i32>> = vec![];

    for line in lines {
        if let Ok(article) = line {
            heightmap.push(
                article
                    .split("")
                    .into_iter()
                    .filter(|x| x.len() > 0)
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect(),
            );
        }
    }

    let low_points = find_low_points(&heightmap);
    let a: i32 = low_points.iter().map(|f| f + 1).sum();

    format!("a: {}, b: {b}", a, b = 0)
}
#[test]
fn test_example() {
    assert_eq!(
        file_to_solution("./src/resources/test/input09.csv"),
        "a: 15, b: 0"
    );
}
