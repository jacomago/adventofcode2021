use std::path::Path;

use adventofcode2021::read_lines;

fn parse_line(input: String) -> Vec<bool> {
    input.chars().into_iter().map(|x| x.eq(&'1')).collect()
}

fn calc_number_from_bytes(bytes: &Vec<bool>) -> u32 {
    let mut init: u32 = 0;
    let mut count = 0;
    for bit in bytes.into_iter().rev() {
        if *bit {
            init += 2_u32.pow(count)
        }
        count += 1;
    }
    init
}

fn flip_bool_vec(bool_vec: &Vec<bool>) -> Vec<bool> {
    bool_vec.into_iter().map(|x| !x).collect()
}

fn one_counts_to_pop(line_count: usize, one_counts: &Vec<u32>) -> Vec<bool> {
    let half_line_count: u32 = (line_count / 2).try_into().unwrap();
    one_counts
        .into_iter()
        .map(|x| if x >= &half_line_count { true } else { false })
        .collect()
}

fn part_a_calc(pop_values: &Vec<bool>) -> (i32, i32) {
    let gamma_rate = calc_number_from_bytes(pop_values);
    let epsilon_rate = calc_number_from_bytes(&flip_bool_vec(pop_values));
    (
        gamma_rate.try_into().unwrap(),
        epsilon_rate.try_into().unwrap(),
    )
}


fn filt_on_predicate(input_numbers: &Vec<Vec<bool>>, b: bool) -> Vec<Vec<bool>> {
    let mut index = 0;
    let mut filt_most_com: Vec<Vec<bool>> = input_numbers.clone();

    while filt_most_com.len() > 1 {
        let most_common_vals = filt_most_com
            .clone()
            .into_iter()
            .filter(|x| x[index])
            .count();
        let most_common_val = most_common_vals >= (filt_most_com.len() - most_common_vals);
        let check_val = if b {most_common_val} else {!most_common_val};

        filt_most_com = filt_most_com
            .into_iter()
            .filter(|x| x[index] == check_val)
            .collect();

        index += 1;
    }

    filt_most_com
}

pub fn main() -> String {
    main_run("./src/resources/input03.csv", 11)
}

fn main_run(filename: &str, n_bits: usize) -> String {
    let path = Path::new(filename);
    let display = path.display();

    // File hosts must exist in current path before this produces output
    let lines = match read_lines(filename) {
        Err(e) => panic!("couldn't open {}: {}", display, e),
        Ok(file) => file,
    };

    let mut line_count: usize = 0;
    // input is 11 bit numbers
    let mut one_counts = vec![0; n_bits];
    let mut input_numbers: Vec<Vec<bool>> = vec![];

    for line in lines {
        if let Ok(article) = line {
            let numbers = parse_line(article);

            for index in 0..=(n_bits - 1) {
                one_counts[index] += match numbers[index] {
                    true => 1,
                    false => 0,
                };
            }

            input_numbers.push(numbers);
            line_count += 1;
        }
    }

    let pop_values = one_counts_to_pop(line_count, &one_counts);
    // part a
    let (gamma_rate, epsilon_rate) = part_a_calc(&pop_values);

    // part b
    let filt_most_com = filt_on_predicate(&input_numbers, true);
    let oxygen_rating: i32 = calc_number_from_bytes(&filt_most_com[0])
        .try_into()
        .unwrap();

    let filt_least_com = filt_on_predicate(&input_numbers, false);
    let co2_rating: i32 = calc_number_from_bytes(&filt_least_com[0])
        .try_into()
        .unwrap();

    return format!(
        "a: {}, b: {b}",
        gamma_rate * epsilon_rate,
        b = oxygen_rating * co2_rating
    );
}

#[test]
fn test_main() {
    assert_eq!(
        main_run("./src/resources/test/input03.csv", 5),
        "a: 198, b: 230"
    );
}
