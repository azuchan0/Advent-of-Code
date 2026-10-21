use std::collections::HashSet;
use std::fs;

fn main() {
    let pages_file = fs::read_to_string("./page.txt").expect("file not read");
    let updates_file = fs::read_to_string("./update.txt").expect("file not read");

    let pages: HashSet<(u32, u32)> = pages_file
        .lines()
        .filter_map(|line| line.split_once('|'))
        .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
        .collect();

    let updates: Vec<Vec<u32>> = updates_file
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num_str| num_str.parse().unwrap())
                .collect()
        })
        .collect();

    let mut total_part1 = 0;
    let mut total_part2 = 0;

    for update in &updates {
        let mut is_correct = true;

        for i in 0..update.len() {
            for j in (i + 1)..update.len() {
                let a = update[i];
                let b = update[j];

                if pages.contains(&(a, b)) {
                    continue;
                }

                if pages.contains(&(b, a)) {
                    is_correct = false;
                    break;
                }
            }
        }

        if is_correct {
            let mid_index = update.len() / 2;
            total_part1 += update[mid_index];
        } else {
            let mut fixed_update = update.clone();
            let mut changed = true;

            while changed {
                changed = false;
                for i in 0..fixed_update.len() - 1 {
                    let a = fixed_update[i];
                    let b = fixed_update[i + 1];

                    if pages.contains(&(b, a)) {
                        fixed_update.swap(i, i + 1);
                        changed = true;
                    }
                }
            }

            let mid_index = fixed_update.len() / 2;
            total_part2 += fixed_update[mid_index];
        }
    }

    println!("Part 1 total: {}", total_part1);
    println!("Part 2 total: {}", total_part2);
}
