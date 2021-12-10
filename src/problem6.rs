use std::path::Path;

use adventofcode2021::read_lines;

pub fn main() -> String {
    file_to_solution("./src/resources/input06.csv", 7)
}

fn first_timers(fish_timers: &Vec<usize>, timer_length: usize) -> Vec<i128> {
    let mut result = vec![0;timer_length + 2];

    for fish in fish_timers {
        result[fish.to_owned()] += 1;
    }
    result
}

fn update_timers(timers: &Vec<i128>) -> Vec<i128> {
    let mut new_timers = vec![0; timers.len()];


    for index in 1..timers.len() {
        new_timers[index - 1] = timers[index];
    }
    new_timers[timers.len() - 1] = timers[0];
    new_timers[timers.len() - 3] += timers[0];

    new_timers
}

fn new_fish_timers(fish_timers: &Vec<usize>, timer_length: usize, days: i32) -> Vec<i128> {
    
    let mut timers = first_timers(fish_timers, timer_length);

    for _ in 0..days {
        timers = update_timers(&timers);
    }
    timers
}

fn total_fish(fish_timers: &Vec<usize>, timer_length: usize, days: i32) -> i128 {
    new_fish_timers(fish_timers, timer_length, days).into_iter().sum()
}

fn file_to_solution(filename: &str, timer_length: usize) -> String {
    let path = Path::new(filename);
    let display = path.display();

    // File hosts must exist in current path before this produces output
    let lines = match read_lines(filename) {
        Err(e) => panic!("couldn't open {}: {}", display, e),
        Ok(file) => file,
    };

    let mut fish_timers: Vec<usize> = vec![];

    for line in lines {
        if let Ok(article) = line {
            fish_timers = article
                .split(",")
                .into_iter()
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
        }
    }

    let a = total_fish(&fish_timers, timer_length, 80);
    let b = total_fish(&fish_timers, timer_length, 256);

    format!("a: {}, b: {b}", a, b = b)
}
#[test]
fn test_example() {
    assert_eq!(
        file_to_solution("./src/resources/test/input06.csv", 7),
        "a: 5934, b: 26984457539"
    );
}
