use std::collections::HashMap;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").expect("file not found");
    let grid: Vec<Vec<char>> = file.lines().map(|n| n.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;
    let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    for r in 0..rows {
        let mut c = 0;
        while c < cols {
            if grid[r][c].is_ascii_digit() {
                let mut found = false;
                let start_col = c;
                let mut number = 0;
                while c < grid[r].len() && grid[r][c].is_ascii_digit() {
                    number = number * 10 + grid[r][c].to_digit(10).unwrap();
                    c += 1;
                }
                let end_col = c;
                let left = if start_col > 0 { start_col - 1 } else { 0 };
                let right = if end_col < cols { end_col } else { cols - 1 };

                let mut adjacent_gears = Vec::new();
                let top = r.saturating_sub(1);
                let bottom = (r + 1).min(rows - 1);

                for rr in top..=bottom {
                    for cc in left..=right {
                        if grid[rr][cc] == '*' && !adjacent_gears.contains(&(rr, cc)) {
                            adjacent_gears.push((rr, cc));
                        }
                    }
                }

                for gear_pos in adjacent_gears {
                    gears.entry(gear_pos).or_default().push(number);
                }

                for col in left..=right {
                    if r > 0 {
                        let up = grid[r - 1][col];
                        if up != '.' && !up.is_ascii_digit() {
                            found = true;
                        }
                    }

                    if r + 1 < rows {
                        let down = grid[r + 1][col];
                        if down != '.' && !down.is_ascii_digit() {
                            found = true;
                        }
                    }

                    let current = grid[r][col];
                    if current != '.' && !current.is_ascii_digit() {
                        found = true;
                    }
                }
                if found {
                    count += number;
                }
            } else {
                c += 1;
            }
        }
    }
    let total: u32 = gears
        .values()
        .filter(|numbers| numbers.len() == 2)
        .map(|numbers| numbers[0] * numbers[1])
        .sum();

    println!("{count}");
    println!("{total}");
}
