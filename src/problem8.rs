use std::{collections::HashSet, path::Path};

use adventofcode2021::read_lines;

pub fn main() -> String {
    file_to_solution("./src/resources/input08.csv")
}

fn string_to_hashset(word: &str) -> HashSet<char> {
    let mut words_charset = HashSet::new();
    for char in word.chars() {
        words_charset.insert(char);
    }
    words_charset
}

fn number_to_letters_map() -> Vec<HashSet<char>> {
    let mut output: Vec<HashSet<char>> = vec![HashSet::new(); 10];

    output[0] = string_to_hashset("abcefg");
    output[1] = string_to_hashset("cf");
    output[2] = string_to_hashset("acdeg");
    output[3] = string_to_hashset("acdfg");
    output[4] = string_to_hashset("bcdf");
    output[5] = string_to_hashset("abdfg");
    output[6] = string_to_hashset("abdef");
    output[7] = string_to_hashset("acf");
    output[8] = string_to_hashset("abcdefg");
    output[9] = string_to_hashset("abcdfg");

    output
}

fn check_word_in_digits(
    word: &HashSet<char>,
    digits: &Vec<usize>,
    number_letter_map: &Vec<HashSet<char>>,
) -> bool {
    for digit in digits {
        if word.len() == number_letter_map[digit.to_owned()].len() {
            return true;
        }
    }
    false
}

fn calc_new_number_map(words: &Vec<HashSet<char>>) -> Vec<HashSet<char>> {
    let output_1 = words.into_iter().find(|x| x.len() == 2).unwrap().to_owned();
    let output_4 = words.into_iter().find(|x| x.len() == 4).unwrap().to_owned();
    let output_7 = words.into_iter().find(|x| x.len() == 3).unwrap().to_owned();
    let output_8 = words.into_iter().find(|x| x.len() == 7).unwrap().to_owned();

    let a = output_7.difference(&output_1).min().unwrap().to_owned();
    let bd: HashSet<_> = output_4.difference(&output_1).collect();

    let six_letters = words.into_iter().filter(|f| f.len() == 6);

    let output_6 = six_letters
        .clone()
        .find(|x| {
            for val in &output_1 {
                if !x.contains(&val) {
                    return true;
                }
            }
            false
        })
        .unwrap()
        .to_owned();

    let c = output_8.difference(&output_6).min().unwrap().to_owned();
    let c_set = [c].iter().cloned().collect::<HashSet<char>>();
    let f = output_1.difference(&c_set).min().unwrap().to_owned();

    let output_0 = six_letters
        .clone()
        .find(|x| {
            for val in &bd {
                if !x.contains(&val) {
                    return true;
                }
            }
            false
        })
        .unwrap()
        .to_owned();

    let d = output_8.difference(&output_0).min().unwrap().to_owned();
    let b = bd
        .into_iter()
        .filter(|x| x.to_owned().to_owned() != d)
        .min()
        .unwrap()
        .to_owned()
        .to_owned();

    let output_9 = six_letters
        .clone()
        .find(|x| x.difference(&output_0).count() > 0 && x.difference(&output_6).count() > 0)
        .unwrap()
        .to_owned();

    let g = output_9
        .difference(&output_4)
        .find(|x| x.to_owned().to_owned() != a)
        .unwrap()
        .to_owned();
    let e = output_8.difference(&output_9).min().unwrap().to_owned();

    let mut output = vec![HashSet::new(); 10];
    output[0] = output_0;
    output[1] = output_1;
    output[2] = [a, c, d, e, g].iter().cloned().collect();

    output[3] = [a, c, d, f, g].iter().cloned().collect();
    output[4] = output_4;

    output[5] = [a, b, d, f, g].iter().cloned().collect();
    output[6] = output_6;
    output[7] = output_7;
    output[8] = output_8;
    output[9] = output_9;

    output
}

fn calc_output_number(words: &Vec<HashSet<char>>, number_map: &Vec<HashSet<char>>) -> i32 {
    let mut sum = 0;
    let mut count = 4;

    for word in words {
        count -= 1;
        for index in 0..number_map.len() {
            if number_map[index].symmetric_difference(&word).count() == 0 {
                sum += index as i32 * (10 as i32).pow(count);
            }
        }
    }
    sum
}

fn calc_output_numbers_total(words_vec: &Vec<Vec<Vec<HashSet<char>>>>) -> i32 {
    let mut sum = 0;
    for vector in words_vec {
        let new_number_map = calc_new_number_map(&vector[0]);
        let output_number: i32 = calc_output_number(&vector[1], &new_number_map);
        sum += output_number;
    }
    sum
}

fn calc_number_of_digits(
    words_vec: &Vec<Vec<Vec<HashSet<char>>>>,
    digits: Vec<usize>,
    number_letter_map: &Vec<HashSet<char>>,
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

fn parse_line(line: String) -> Vec<Vec<HashSet<char>>> {
    line.split("|")
        .into_iter()
        .map(|f| {
            f.split(" ")
                .into_iter()
                .map(|f| string_to_hashset(f))
                .filter(|f| f.len() > 0) // reorder letters or map letters to numbers
                .collect::<Vec<HashSet<char>>>()
        })
        .collect()
}

fn file_to_solution(filename: &str) -> String {
    let path = Path::new(filename);
    let display = path.display();

    // File hosts must exist in current path before this produces output
    let lines = match read_lines(filename) {
        Err(e) => panic!("couldn't open {}: {}", display, e),
        Ok(file) => file,
    };

    let mut words_vec: Vec<Vec<Vec<HashSet<char>>>> = vec![];

    for line in lines {
        if let Ok(article) = line {
            words_vec.push(parse_line(article));
        }
    }

    let number_letter_map = number_to_letters_map();
    let a = calc_number_of_digits(&words_vec, vec![1, 4, 7, 8], &number_letter_map);

    let b = calc_output_numbers_total(&words_vec);

    format!("a: {}, b: {b}", a, b = b)
}

#[test]
fn test_small_part() {
    let test_line =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
    let first_words = parse_line(test_line.to_string());
    let number_map = calc_new_number_map(&first_words[0]);
    assert_eq!(calc_output_number(&first_words[1], &number_map), 5353);
}

#[test]
fn test_example() {
    assert_eq!(
        file_to_solution("./src/resources/test/input08.csv"),
        "a: 26, b: 61229"
    );
}
