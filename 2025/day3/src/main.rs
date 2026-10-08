use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").expect("file not read");
    let mut total = 0;

    for line in file.lines() {
        if line.is_empty() {
            continue;
        }

        let digits: Vec<u64> = line
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as u64))
            .collect();

        if digits.len() < 12 {
            continue;
        }

        let mut stack: Vec<u64> = Vec::new();
        let mut to_remove = digits.len() - 12;

        for &d in &digits {
            while let Some(&top) = stack.last() {
                if d > top && to_remove > 0 {
                    stack.pop();
                    to_remove -= 1;
                } else {
                    break;
                }
            }
            stack.push(d);
        }
        stack.truncate(12);
        let mut highest_value: u64 = 0;
        for &d in &stack {
            highest_value = highest_value * 10 + d;
        }

        total += highest_value;
    }

    println!("Total: {}", total);
}
