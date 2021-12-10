use std::path::Path;

use adventofcode2021::read_lines;

pub fn main() -> String {
    file_to_solution("./src/resources/input07.csv")
}

enum Alg {
    Basic,
    Cumulative,
}

fn fuel_cost(crab_positions: &Vec<i32>, pos: i32, alg: &Alg) -> i128 {
    crab_positions
        .into_iter()
        .map(|f| match alg {
            Alg::Basic => (f - pos).abs() as i128,
            Alg::Cumulative => {
                let n = (f - pos).abs() as i128;
                n * (n + 1) / 2
            }
        })
        .sum()
}

fn calc_cheapest_fuel(crab_positions: &Vec<i32>, alg: Alg) -> (i128, i32) {
    let max = crab_positions.into_iter().max().unwrap();
    let mut min_fuel_cost = match alg {
        Alg::Basic => max.to_owned() as i128 * crab_positions.len() as i128,
        Alg::Cumulative => {
            max.to_owned() as i128 * (max.to_owned() as i128 + 1) * (crab_positions.len() as i128) / 2
        }
    };
    let mut min_pos = 0;

    for pos in 0..max.to_owned() {
        let fuel_cost = fuel_cost(crab_positions, pos, &alg);
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

    let (a, _) = calc_cheapest_fuel(&crab_positions, Alg::Basic);
    let (b, _) = calc_cheapest_fuel(&crab_positions, Alg::Cumulative);

    format!("a: {}, b: {b}", a, b = b)
}

#[test]
fn test_example() {
    assert_eq!(
        file_to_solution("./src/resources/test/input07.csv"),
        "a: 37, b: 168"
    );
}
