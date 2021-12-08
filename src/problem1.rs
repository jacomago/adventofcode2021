use std::path::Path;

use adventofcode2021::read_lines;

pub fn problem1() -> String {
    let filename = "./src/resources/input01.csv";
    let path = Path::new(filename);
    let display = path.display();

    // File hosts must exist in current path before this produces output
    let lines = match read_lines(filename) {
        Err(e) => panic!("couldn't open {}: {}", display, e),
        Ok(file) => file,
    };

    let mut prev_num = 0;
    let mut prev_prev_num = 0;
    let mut prev_prev_prev_num = 0;
    let mut increase_count = 0;
    let mut three_window_increase = 0;

    // Consumes the iterator, returns an (Optional) String
    for line in lines {
        if let Ok(article) = line {
            let new_num = article.parse::<i32>().unwrap();

            if new_num - prev_num > 0 && prev_num != 0 {
                increase_count = increase_count + 1;
            }

            if new_num - prev_prev_prev_num > 0 && prev_prev_prev_num != 0{
                three_window_increase = three_window_increase + 1;
            }

            prev_prev_prev_num = prev_prev_num;
            prev_prev_num = prev_num;
            prev_num = new_num;
        }
    }

    return format!("a: {}, b: {b} ", increase_count , b = three_window_increase );
}
