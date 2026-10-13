use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("Failed to read input.txt");

    let mut total = 0;
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            left.push(parts[0].parse::<i64>().expect("Failed to parse left"));
            right.push(parts[1].parse::<i64>().expect("Failed to parse right"));
        }
    }

    left.sort();
    right.sort();

    for &num in left.iter() {
        let count: usize = right.iter().filter(|&x| num == *x).count();
        total += num * count as i64;
    }
    println!("{}", total);
}
