use std::fs;

fn main() {
    let mut result = 0;
    let file = fs::read_to_string("./input.txt").expect("file not read");
    let file = file.replace("\r\n", "\n");
    let sections: Vec<&str> = file.trim().split("\n\n").collect();
    let ranges: Vec<(i64, i64)> = sections[0]
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split('-').collect();
            (parts[0].parse().unwrap(), parts[1].parse().unwrap())
        })
        .collect();

    let mut sorted = ranges.clone();
    sorted.sort_by_key(|r| r.0);

    let mut current_max = 0;
    for (min, max) in &sorted {
        if *min > current_max {
            result += max - min + 1;
            current_max = *max;
        } else if *max > current_max {
            result += *max - current_max;
            current_max = *max;
        }
    }
    println!("{}", result);
}
