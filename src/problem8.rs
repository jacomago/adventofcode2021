use std::path::Path;

use adventofcode2021::read_lines;

pub fn main() -> String {
    file_to_solution("./src/resources/input08.csv")
}

fn number_to_letters_map() -> Vec<String> {
    let mut output = vec!["".to_string(); 10];

    output[0] = "abcefg".to_string();
    output[1] = "cf".to_string();
    output[2] = "acdeg".to_string();
    output[3] = "acdfg".to_string();
    output[4] = "bcdf".to_string();
    output[5] = "abdfg".to_string();
    output[6] = "abdef".to_string();
    output[7] = "acf".to_string();
    output[8] = "abcdefg".to_string();
    output[9] = "abcdfg".to_string();

    output
}

fn check_word_in_digits(word: &str, digits: &Vec<usize>, number_letter_map: &Vec<String>) -> bool {
    for digit in digits {
        if word.len() == number_letter_map[digit.to_owned()].len() {
            return true;
        }
    }
    false
}

fn calc_number_of_digits(
    words_vec: &Vec<Vec<Vec<String>>>,
    digits: Vec<usize>,
    number_letter_map: &Vec<String>,
) -> i32 {
    let mut count = 0;
    for vector in words_vec {
        for word in &vector[1] {
            if check_word_in_digits(word, &digits, number_letter_map) {
                count += 1;
            }
        }
    }
    count
}

fn file_to_solution(filename: &str) -> String {
    let path = Path::new(filename);
    let display = path.display();

    // File hosts must exist in current path before this produces output
    let lines = match read_lines(filename) {
        Err(e) => panic!("couldn't open {}: {}", display, e),
        Ok(file) => file,
    };

    let mut words_vec: Vec<Vec<Vec<String>>> = vec![];

    for line in lines {
        if let Ok(article) = line {
            words_vec.push(
                article
                    .split("|")
                    .into_iter()
                    .map(|f| {
                        f.split(" ")
                            .into_iter()
                            .map(|f| f.to_string()) 
                            .collect::<Vec<String>>()
                    })
                    .collect(),
            );
        }
    }

    let number_letter_map = number_to_letters_map();
    let a = calc_number_of_digits(&words_vec, vec![1, 4, 7, 8], &number_letter_map);

    format!("a: {}, b: {b}", a, b = 0)
}
#[test]
fn test_example() {
    assert_eq!(
        file_to_solution("./src/resources/test/input08.csv"),
        "a: 26, b: 61229"
    );
}
