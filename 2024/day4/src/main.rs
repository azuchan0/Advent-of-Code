use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").expect("file not read");
    let grid: Vec<Vec<char>> = file.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for r in 1..rows - 1 {
        for c in 1..cols - 1 {
            if grid[r][c] != 'A' {
                continue;
            }
            let tl = grid[r - 1][c - 1]; // top-left
            let tr = grid[r - 1][c + 1]; // top-right
            let bl = grid[r + 1][c - 1]; // bottom-left
            let br = grid[r + 1][c + 1]; // bottom-right

            let diag1 = (tl == 'M' && br == 'S') || (tl == 'S' && br == 'M');
            let diag2 = (tr == 'M' && bl == 'S') || (tr == 'S' && bl == 'M');

            if diag1 && diag2 {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
