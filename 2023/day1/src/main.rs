use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("file not read");
    let mut total = 0;
    let words = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    for line in input.lines() {
        let mut first = None;
        let mut last = 0;

        for (i, c) in line.char_indices() {
            let mut digit = c.to_digit(10);

            if digit.is_none() {
                for (word, value) in &words {
                    if line[i..].starts_with(*word) {
                        digit = Some(*value);
                        break;
                    }
                }
            }

            if let Some(digit) = digit {
                if first.is_none() {
                    first = Some(digit);
                }
                last = digit;
            }
        }

        if let Some(first_digit) = first {
            total += first_digit * 10 + last;
        }
    }

    println!("{}", total);
}
