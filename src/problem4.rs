use std::path::Path;

use adventofcode2021::read_lines;

pub fn main() -> String {
    file_to_solution("./src/resources/input04.csv")
}

fn check_bingo(prev_rolls: &Vec<i32>, game: &Vec<Vec<i32>>) -> bool {
    for row in game {
        let mut row_tracker = true;
        for number in row {
            if !prev_rolls.contains(&number) {
                row_tracker = false;
                break;
            }
        }
        if row_tracker {
            return true;
        }
    }
    let n = game[0].len();
    for col in 0..n {
        let mut col_tracker = true;
        for row in game {
            if !prev_rolls.contains(&row[col]) {
                col_tracker = false;
                break;
            }
        }
        if col_tracker {
            return true;
        }
    }
    false
}

fn calc_score(game: &Vec<Vec<i32>>, prev_rolls: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for row in game {
        for col in row {
            if !prev_rolls.contains(&col) {
                sum += col;
            }
        }
    }
    sum * prev_rolls.last().unwrap()
}

fn play_games_and_score(
    rolls: &Vec<i32>,
    games: &Vec<Vec<Vec<i32>>>,
) -> Vec<(Vec<i32>, Vec<Vec<i32>>)> {
    let mut prev_rolls = vec![];
    let mut winning_prev_rolls = vec![];
    let mut winning_games: Vec<Vec<Vec<i32>>> = vec![];

    for roll in rolls {
        prev_rolls.push(*roll);
        for game in games {
            if check_bingo(&prev_rolls, &game) && !winning_games.contains(game) {
                winning_prev_rolls.push(prev_rolls.to_vec());
                winning_games.push(game.to_vec());
            }
        }
    }
    winning_prev_rolls
        .into_iter()
        .zip(winning_games.into_iter())
        .map(|(x, y)| (x, y))
        .collect()
}

fn file_to_solution(filename: &str) -> String {
    let path = Path::new(filename);
    let display = path.display();

    // File hosts must exist in current path before this produces output
    let mut lines = match read_lines(filename) {
        Err(e) => panic!("couldn't open {}: {}", display, e),
        Ok(file) => file,
    };

    let mut rolls: Vec<i32> = vec![];

    if let Ok(first_line) = lines.next().unwrap() {
        rolls = first_line
            .split(',')
            .into_iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
    }
    lines.next();

    let mut games: Vec<Vec<Vec<i32>>> = vec![];
    let mut game: Vec<Vec<i32>> = vec![];

    for line in lines {
        if let Ok(article) = line {
            if article == "" {
                games.push(game);
                game = vec![];
            } else {
                game.push(
                    article
                        .split_whitespace()
                        .into_iter()
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect(),
                );
            }
        }
    }
    games.push(game);

    let rolls_to_winning_games = play_games_and_score(&rolls, &games);

    // a
    let pair = rolls_to_winning_games.first().unwrap();
    let a_result_score = calc_score(&pair.1, &pair.0);

    let pair = rolls_to_winning_games.last().unwrap();
    let b_result_score = calc_score(&pair.1, &pair.0);

    format!("a: {}, b: {b}", a_result_score, b = b_result_score)
}

// The score of the winning board can now be calculated.
//S tart by finding the sum of all unmarked numbers on that board;
// in this case, the sum is 188. Then, multiply that sum by
// the number that was just called when the board won, 24,
//to get the final score, 188 * 24 = 4512.

// part b get the last game, not the first game to win.
#[test]
fn test_example() {
    assert_eq!(
        file_to_solution("./src/resources/test/input04.csv"),
        "a: 4512, b: 1924"
    );
}
