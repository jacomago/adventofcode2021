use std::{collections::HashMap, path::Path, vec};

use adventofcode2021::read_lines;

pub fn main() -> String {
    file_to_solution("./src/resources/input05.csv")
}

fn parse_article(article: String) -> Vec<Vec<i32>> {
    let v = article.split(" -> ");
    let mut out = vec![];
    for pair in v {
        let value = pair
            .split(',')
            .into_iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        out.push(value);
    }
    out
}

fn update_grid(grid: &mut HashMap<(i32, i32), i32>, line: &Vec<Vec<i32>>, include_diagonals: bool) {
    if line[0][0] == line[1][0] {
        let range = if line[0][1] - line[1][1] < 0 {
            line[0][1]..=line[1][1]
        } else {
            line[1][1]..=line[0][1]
        };

        let coord_x = line[0][0];
        for coord_y in range {
            let cur_value = grid.entry((coord_x, coord_y)).or_insert(0);
            *cur_value += 1;
        }
    } else if line[0][1] == line[1][1] {
        let range = if line[0][0] - line[1][0] < 0 {
            line[0][0]..=line[1][0]
        } else {
            line[1][0]..=line[0][0]
        };

        let coord_y = line[0][1];
        for coord_x in range {
            let cur_value = grid.entry((coord_x, coord_y)).or_insert(0);
            *cur_value += 1;
        }
    } else if include_diagonals {
        let mut coord_x = line[0][0];
        let mut coord_y = line[0][1];

        let x_direc = if line[0][0] - line[1][0] < 0 {1} else {-1};
        let y_direc = if line[0][1] - line[1][1] < 0 {1} else {-1};
        
        while (coord_x, coord_y) != (line[1][0], line[1][1]) {
            let cur_value = grid.entry((coord_x, coord_y)).or_insert(0);
            *cur_value += 1;

            coord_x += x_direc;
            coord_y += y_direc;
        }
    }
}

fn calc_grid(vent_lines: &Vec<Vec<Vec<i32>>>, include_diagonals: bool) -> HashMap<(i32, i32), i32> {
    let mut grid = HashMap::new();
    for line in vent_lines {
        update_grid(&mut grid, line, include_diagonals);
    }
    grid
}

fn count_grid(grid: HashMap<(i32, i32), i32>, min: i32) -> i32 {
    let mut sum = 0;
    for (_k, v) in grid {
        if v >= min {
            sum += 1;
        }
    }
    sum
}

fn file_to_solution(filename: &str) -> String {
    let path = Path::new(filename);
    let display = path.display();

    // File hosts must exist in current path before this produces output
    let lines = match read_lines(filename) {
        Err(e) => panic!("couldn't open {}: {}", display, e),
        Ok(file) => file,
    };

    // Vec<((i32, i32), (i32, i32))>
    let mut vent_lines: Vec<Vec<Vec<i32>>> = vec![];

    for line in lines {
        if let Ok(article) = line {
            vent_lines.push(parse_article(article));
        }
    }

    let grid = calc_grid(&vent_lines, false);
    let a = count_grid(grid, 2);

    let grid = calc_grid(&vent_lines, true);
    let b = count_grid(grid, 2);

    format!("a: {}, b: {b}", a, b = b)
}

#[test]
fn test_example() {
    assert_eq!(
        file_to_solution("./src/resources/test/input05.csv"),
        "a: 5, b: 12"
    );
}
