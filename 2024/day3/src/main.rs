use regex::Regex;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").expect("file not read");
    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();
    let mut enabled = true;
    let mut total: u64 = 0;

    for cap in re.captures_iter(&file) {
        let full = cap.get(0).unwrap().as_str();
        match full {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ if enabled => {
                let x = cap[1].parse::<u64>().unwrap();
                let y = cap[2].parse::<u64>().unwrap();
                total += x * y;
            }
            _ => {}
        }
    }
    println!("{}", total);
}
