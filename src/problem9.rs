use std::{
    collections::{BinaryHeap, HashSet, VecDeque},
    path::Path,
    vec,
};

use adventofcode2021::read_lines;

pub fn main() -> String {
    file_to_solution("./src/resources/input09.csv")
}

fn find_low_points(heightmap: &Vec<Vec<i32>>) -> Vec<((usize, usize), i32)> {
    let mut low_points = vec![];
    for row in 0..heightmap.len() {
        for col in 0..heightmap[row].len() {
            if is_low_point(row, col, heightmap) {
                low_points.push(((row, col), heightmap[row][col]));
            }
        }
    }
    low_points
}

fn point_surrounding_points(
    point: ((usize, usize), i32),
    heightmap: &Vec<Vec<i32>>,
) -> Vec<((usize, usize), i32)> {
    surrounding_points(point.0 .0, point.0 .1, heightmap)
}

fn surrounding_points(
    row: usize,
    col: usize,
    heightmap: &Vec<Vec<i32>>,
) -> Vec<((usize, usize), i32)> {
    let mut output = vec![];
    if row > 0 {
        output.push(((row - 1, col), heightmap[row - 1][col]));
    }
    if col > 0 {
        output.push(((row, col - 1), heightmap[row][col - 1]));
    }
    if col < heightmap[row].len() - 1 {
        output.push(((row, col + 1), heightmap[row][col + 1]));
    }
    if row < heightmap.len() - 1 {
        output.push(((row + 1, col), heightmap[row + 1][col]));
    }
    output
}

fn is_low_point(row: usize, col: usize, heightmap: &Vec<Vec<i32>>) -> bool {
    let surrounding_points = surrounding_points(row, col, heightmap);
    if heightmap[row][col]
        < surrounding_points
            .iter()
            .map(|f| f.to_owned().1)
            .min()
            .unwrap()
    {
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
    let a: i32 = low_points.iter().map(|f| f.to_owned().1 + 1).sum();

    let basins = find_basins(&heightmap, &low_points);
    let mut basin_sizes: BinaryHeap<usize> = basins.into_iter().map(|f| f.len()).collect();
    let b = basin_sizes.pop().unwrap() * basin_sizes.pop().unwrap() * basin_sizes.pop().unwrap();

    format!("a: {}, b: {b}", a, b = b)
}

fn find_basins(
    heightmap: &Vec<Vec<i32>>,
    low_points: &Vec<((usize, usize), i32)>,
) -> Vec<HashSet<((usize, usize), i32)>> {
    let mut basins = vec![];
    for low_point in low_points {
        basins.push(find_basin_points(heightmap, low_point));
    }
    basins
}

fn find_basin_points(
    heightmap: &Vec<Vec<i32>>,
    low_point: &((usize, usize), i32),
) -> HashSet<((usize, usize), i32)> {
    let mut basin_points = HashSet::new();
    let mut points_to_visit = VecDeque::from([low_point.to_owned()]);

    while !points_to_visit.is_empty() {
        let current_point = points_to_visit.pop_back().unwrap();
        basin_points.insert(current_point);

        let surrounding_points = point_surrounding_points(current_point.to_owned(), heightmap);

        for point in surrounding_points {
            if !(point.1 == 9 || basin_points.contains(&point)) {
                points_to_visit.push_back(point);
            }
        }
    }
    basin_points
}

#[test]
fn test_example() {
    assert_eq!(
        file_to_solution("./src/resources/test/input09.csv"),
        "a: 15, b: 1134"
    );
}
