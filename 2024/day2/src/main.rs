use std::fs;

fn is_safe(nums: &[i64]) -> bool {
    let diff_ok = nums.windows(2).all(|w| {
        let diff = (w[0] - w[1]).abs();
        diff >= 1 && diff <= 3
    });

    let increasing = nums.windows(2).all(|w| w[0] < w[1]);
    let decreasing = nums.windows(2).all(|w| w[0] > w[1]);
    diff_ok && (increasing || decreasing)
}

fn main() {
    let reports = fs::read_to_string("./input.txt").expect("file not read");
    let mut safe_count = 0;
    for line in reports.lines() {
        let nums: Vec<i64> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        if is_safe(&nums) {
            safe_count += 1;
            continue;
        }

        let tolerated = (0..nums.len()).any(|i| {
            let mut dampened = nums.clone();
            dampened.remove(i);
            is_safe(&dampened)
        });

        if tolerated {
            safe_count += 1;
        }
    }
    println!("{}", safe_count);
}
