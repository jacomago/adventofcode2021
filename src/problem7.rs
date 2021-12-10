use std::path::Path;

use adventofcode2021::read_lines;

pub fn main() -> String {
    file_to_solution("./src/resources/input07.csv")
}

fn fuel_cost(crab_positions: &Vec<i32>, pos: i32) -> i32 {
    crab_positions.into_iter().map(
        |f| (f - pos).abs()
    ).sum()
}

fn calc_cheapest_fuel( crab_positions: &Vec<i32> ) -> (i32, i32) {
    let max = crab_positions.into_iter().max().unwrap();
    let mut min_fuel_cost = max.to_owned() * crab_positions.len() as i32;
    let mut min_pos = 0;

    for pos in 0..max.to_owned() {
        let fuel_cost = fuel_cost(crab_positions, pos);
        if fuel_cost < min_fuel_cost {
            min_fuel_cost = fuel_cost;
            min_pos = pos;
        }
    }

    (min_fuel_cost, min_pos)

}

fn file_to_solution(filename: &str) -> String {
    let path = Path::new(filename);
    let display = path.display();

    // File hosts must exist in current path before this produces output
    let lines = match read_lines(filename) {
        Err(e) => panic!("couldn't open {}: {}", display, e),
        Ok(file) => file,
    };

    let mut crab_positions: Vec<i32> = vec![];

    for line in lines {
        if let Ok(article) = line {
            crab_positions = article
                .split(",")
                .into_iter()
                .map(|f| f.parse::<i32>().unwrap())
                .collect()
        }
    }

    let (min_fuel_cost, _) = calc_cheapest_fuel(&crab_positions);

    format!("a: {}, b: {b}", min_fuel_cost, b = 0)
}

#[test]
fn test_example() {
    assert_eq!(
        file_to_solution("./src/resources/test/input07.csv"),
        "a: 37, b: 0"
    );
}
