use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").expect("file not found");
    let lines: Vec<_> = file.lines().collect();
    let mut copies = vec![1usize; lines.len()];

    for (i, line) in lines.iter().enumerate() {
        let num_part = line.split(':').nth(1).expect("invalid format");
        let (left_str, right_str) = num_part.split_once('|').expect("invalid format");

        let left: Vec<usize> = left_str
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        let right: Vec<usize> = right_str
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        let matches = right.iter().filter(|num| left.contains(*num)).count();
        let end = (i + 1 + matches).min(copies.len());
        let card_copies = copies[i];

        for next in i + 1..end {
            copies[next] += card_copies;
        }
    }
    let total: usize = copies.iter().sum();
    println!("{total}");
}
